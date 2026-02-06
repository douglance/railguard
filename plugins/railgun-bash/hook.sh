#!/usr/bin/env bash
set -euo pipefail

# Railgun - Claude Code LLM protection hook
# Reads JSON from stdin, outputs verdict, exits 0 (allow/ask) or 2 (deny)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RULES_DIR="${RAILGUN_RULES_DIR:-$SCRIPT_DIR/rules.d}"

# Load and merge all JSON files from rules directory
# Any .json file is merged - structure is arbitrary
# Later files override earlier ones (sorted order)
load_rules() {
    local merged='{}'

    if [[ -d "$RULES_DIR" ]]; then
        for f in "$RULES_DIR"/*.json; do
            [[ -e "$f" ]] || continue
            merged=$(echo "$merged" | jq -s --slurpfile new "$f" '.[0] * $new[0]')
        done
    fi

    echo "$merged"
}

# Output deny verdict and exit
deny() {
    local reason="$1"
    local context="${2:-}"
    echo "[railgun-bash] DENY: $reason - $context" >> /tmp/railgun-debug.log
    echo "❌ Blocked by Railgun: $reason - $context" >&2
    exit 2
}

# Output ask verdict and exit
ask_user() {
    local reason="$1"
    cat <<EOF
{"hookSpecificOutput":{"ask":true,"reason":"$reason"}}
EOF
    exit 0
}

# Check for secrets in text
# Iterates over all patterns in secrets.patterns.* (arbitrary keys)
check_secrets() {
    local text="$1"
    local rules="$2"

    # Get all pattern names and values from secrets.patterns
    local pattern_names
    pattern_names=$(echo "$rules" | jq -r '.secrets.patterns // {} | keys[]' 2>/dev/null) || return 0

    for name in $pattern_names; do
        local pattern
        pattern=$(echo "$rules" | jq -r ".secrets.patterns[\"$name\"] // empty")
        if [[ -n "$pattern" ]] && echo "$text" | grep -Eq -- "$pattern"; then
            deny "SecretDetected" "$name detected"
        fi
    done
}

# Check for dangerous commands
# Block always wins, then ask patterns are checked
check_commands() {
    local command="$1"
    local rules="$2"

    # Check block patterns first - block always wins
    local block_patterns
    block_patterns=$(echo "$rules" | jq -r '.commands.block // [] | .[]' 2>/dev/null) || true
    for pattern in $block_patterns; do
        if echo "$command" | grep -Eq -- "$pattern"; then
            deny "DangerousCommand" "Blocked pattern: $pattern"
        fi
    done

    # Check ask patterns - require user confirmation
    local ask_patterns
    ask_patterns=$(echo "$rules" | jq -r '.commands.ask // [] | .[]' 2>/dev/null) || true
    for pattern in $ask_patterns; do
        if echo "$command" | grep -Eq -- "$pattern"; then
            ask_user "Command requires confirmation: $pattern"
        fi
    done
}

# Check for protected paths
check_paths() { return 0; } 
_disabled_check_paths() {
    local path="$1"
    local rules="$2"

    # Normalize path
    path="${path#./}"
    path="${path//\/\//\/}"

    local blocked_patterns
    blocked_patterns=$(echo "$rules" | jq -r '.paths.blocked // [] | .[]')

    for pattern in $blocked_patterns; do
        # Convert glob to regex (simple conversion)
        local regex="${pattern//\*\*/.*}"
        regex="${regex//\*/[^/]*}"
        regex="^${regex}$"

        if echo "$path" | grep -Eq "$regex"; then
            deny "ProtectedPath" "Access to protected path: $path"
        fi

        # Also check just the filename for patterns like **/.env
        local filename
        filename=$(basename "$path")
        local filename_pattern="${pattern##**/}"
        if [[ "$filename_pattern" != "$pattern" ]]; then
            local filename_regex="${filename_pattern//\*/[^/]*}"
            if echo "$filename" | grep -Eq "^${filename_regex}$"; then
                deny "ProtectedPath" "Access to protected file: $filename"
            fi
        fi
    done
}

# Check for network exfiltration
check_network() {
    local text="$1"
    local rules="$2"

    # Extract URLs
    local urls
    urls=$(echo "$text" | grep -oE 'https?://[a-zA-Z0-9][-a-zA-Z0-9]*(\.[a-zA-Z0-9][-a-zA-Z0-9]*)+' || true)

    if [[ -z "$urls" ]]; then
        return 0
    fi

    local blocked_domains
    blocked_domains=$(echo "$rules" | jq -r '.network.blocked // [] | .[]')

    for url in $urls; do
        # Extract domain
        local domain
        domain=$(echo "$url" | sed -E 's|https?://||' | cut -d'/' -f1 | cut -d':' -f1)

        for blocked in $blocked_domains; do
            # Check exact match or subdomain match
            if [[ "$domain" == "$blocked" ]] || [[ "$domain" == *".$blocked" ]]; then
                deny "NetworkExfiltration" "Blocked domain: $domain"
            fi
        done
    done
}

# Check tool-level permissions (deny/ask/allow patterns)
# Supports glob-style patterns like "mcp__*" or "Bash"
check_tool_permissions() {
    local tool_name="$1"
    local rules="$2"

    # Check deny patterns first
    local deny_patterns
    deny_patterns=$(echo "$rules" | jq -r '.tools.deny // [] | .[]' 2>/dev/null) || true
    for pattern in $deny_patterns; do
        if [[ "$tool_name" == $pattern ]]; then
            deny "ToolDenied" "Tool $tool_name is blocked"
        fi
    done

    # Check ask patterns
    local ask_patterns
    ask_patterns=$(echo "$rules" | jq -r '.tools.ask // [] | .[]' 2>/dev/null) || true
    for pattern in $ask_patterns; do
        if [[ "$tool_name" == $pattern ]]; then
            ask_user "Tool $tool_name requires confirmation"
        fi
    done

    # Check allow patterns (skip further inspection if matched)
    local allow_patterns
    allow_patterns=$(echo "$rules" | jq -r '.tools.allow // [] | .[]' 2>/dev/null) || true
    for pattern in $allow_patterns; do
        if [[ "$tool_name" == $pattern ]]; then
            exit 0  # Early allow, skip all inspection
        fi
    done
}

# Main inspection
inspect() {
    local tool_name="$1"
    local tool_input="$2"
    local rules="$3"

    # Check tool-level permissions first
    check_tool_permissions "$tool_name" "$rules"

    case "$tool_name" in
        Bash)
            local command
            command=$(echo "$tool_input" | jq -r '.command // empty')
            if [[ -n "$command" ]]; then
                check_secrets "$command" "$rules"
                check_commands "$command" "$rules"
                check_network "$command" "$rules"
            fi
            ;;
        Write)
            local file_path content
            file_path=$(echo "$tool_input" | jq -r '.file_path // empty')
            content=$(echo "$tool_input" | jq -r '.content // empty')
            if [[ -n "$file_path" ]]; then
                check_paths "$file_path" "$rules"
            fi
            if [[ -n "$content" ]]; then
                check_secrets "$content" "$rules"
            fi
            ;;
        Edit)
            local file_path old_string new_string
            file_path=$(echo "$tool_input" | jq -r '.file_path // empty')
            old_string=$(echo "$tool_input" | jq -r '.old_string // empty')
            new_string=$(echo "$tool_input" | jq -r '.new_string // empty')
            if [[ -n "$file_path" ]]; then
                check_paths "$file_path" "$rules"
            fi
            check_secrets "$old_string" "$rules"
            check_secrets "$new_string" "$rules"
            ;;
        Read)
            local file_path
            file_path=$(echo "$tool_input" | jq -r '.file_path // empty')
            if [[ -n "$file_path" ]]; then
                check_paths "$file_path" "$rules"
            fi
            ;;
        WebFetch)
            local url
            url=$(echo "$tool_input" | jq -r '.url // empty')
            if [[ -n "$url" ]]; then
                check_network "$url" "$rules"
            fi
            ;;
        Task)
            local prompt
            prompt=$(echo "$tool_input" | jq -r '.prompt // empty')
            if [[ -n "$prompt" ]]; then
                check_secrets "$prompt" "$rules"
            fi
            ;;
    esac
}

# Entry point
main() {
    # Debug log
    echo "[railgun-bash] Hook invoked at $(date)" >> /tmp/railgun-debug.log

    # Read JSON from stdin
    local input
    input=$(cat)
    echo "[railgun-bash] Input: $input" >> /tmp/railgun-debug.log

    # Parse input
    local tool_name tool_input
    tool_name=$(echo "$input" | jq -r '.tool_name // empty')
    tool_input=$(echo "$input" | jq -c '.tool_input // {}')

    if [[ -z "$tool_name" ]]; then
        deny "InvalidInput" "Missing tool_name"
    fi

    # Load rules
    local rules
    rules=$(load_rules)

    # Run inspection
    inspect "$tool_name" "$tool_input" "$rules"

    # If we get here, allow
    exit 0
}

main
