---
description: Manage Railgun security policies - lint config, test tool inputs, view policy
triggers:
  - railgun
  - rg
---

# Railgun Security Manager

Railgun protects your Claude Code sessions by intercepting tool calls and blocking:
- **Secrets**: AWS keys, GitHub tokens, OpenAI keys, private keys
- **Dangerous commands**: `rm -rf /`, fork bombs, disk writes
- **Protected paths**: `.env`, `.ssh/`, `*.pem`, credentials files
- **Data exfiltration**: pastebin, ngrok, webhook capture sites

## Commands

### Lint Configuration

Validate your `railgun.toml` configuration file:

```bash
# Find and run the appropriate binary
BINARY="${CLAUDE_PLUGIN_ROOT}/bin/rg-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m | sed 's/x86_64/x64/' | sed 's/aarch64/arm64/')"
"$BINARY" lint --config ./railgun.toml
```

### Test a Tool Input

Test how a specific tool input would be evaluated by the policy:

```bash
# Test a Bash command
echo '{"tool_name":"Bash","tool_input":{"command":"ls -la"}}' | "$BINARY" hook --config ./railgun.toml

# Test a file read
echo '{"tool_name":"Read","tool_input":{"file_path":"./src/main.rs"}}' | "$BINARY" hook --config ./railgun.toml
```

Exit code 0 = allowed, exit code 2 = blocked.

### Create Configuration

Copy the example configuration to customize:

```bash
cp "${CLAUDE_PLUGIN_ROOT}/railgun.example.toml" ./railgun.toml
```

## Configuration

Railgun looks for configuration in this order:
1. `./railgun.toml` (project-level)
2. `~/.config/railgun/railgun.toml` (user-level)
3. Plugin default (read-only)

## Policy Modes

- **strict** (default): Block dangerous actions
- **monitor**: Log only, don't block (for testing)

## Getting Help

If Railgun blocks something you need:
1. Check which pattern matched in the block reason
2. Add an allow pattern to override the block
3. Or disable the specific scanner in config
