# Railguard

**The Firewall for Claude Code** — A local-first security hook that protects against secrets leakage, dangerous commands, and unauthorized tool use.

[![CI](https://github.com/douglance/railguard/actions/workflows/ci.yml/badge.svg)](https://github.com/douglance/railguard/actions)
[![Crates.io](https://img.shields.io/crates/v/railguard.svg)](https://crates.io/crates/railguard)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/douglance/railguard)](https://github.com/douglance/railguard/releases)

---

## What is Railguard?

Railguard sits between Claude Code and your system, inspecting every tool invocation before it executes. It blocks secrets from leaking, prevents dangerous commands, and gives you fine-grained control over what Claude can do.

```
┌──────────────┐     ┌─────────────┐     ┌──────────────────┐
│  Claude Code │ ──► │  Railguard  │ ──► │  Tool Execution  │
│  (LLM)       │     │  (Inspect)  │     │  (Bash, Write..) │
└──────────────┘     └─────────────┘     └──────────────────┘
                           │
                           ▼
                     Block or Allow
```

### Key Features

- **Secret Detection** — Blocks AWS keys, GitHub tokens, private keys, and high-entropy strings
- **Dangerous Command Blocking** — Prevents `rm -rf /`, fork bombs, and disk operations
- **Protected Path Detection** — Blocks access to `~/.ssh/`, `~/.aws/credentials`, `.env` files
- **Network Exfiltration Prevention** — Blocks ngrok, pastebin, webhook.site
- **Tool-Level Permissions** — Allow/deny/ask rules for any Claude tool or MCP server
- **Sub-Millisecond Latency** — < 1ms p99 overhead, won't slow down your workflow

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/douglance/railguard/main/install.sh | bash
```

This will:
1. Download the correct binary for your platform
2. Install it to `~/.local/bin/`
3. Configure Claude Code to use Railguard

### Homebrew (macOS/Linux)

```bash
brew tap douglance/tap
brew install railguard
railguard install
```

### Cargo (Rust)

```bash
cargo install railguard
railguard install
```

### From GitHub Releases

Download the latest release for your platform:

- [darwin-arm64](https://github.com/douglance/railguard/releases/latest/download/railguard-darwin-arm64.tar.gz) (macOS Apple Silicon)
- [darwin-x64](https://github.com/douglance/railguard/releases/latest/download/railguard-darwin-x64.tar.gz) (macOS Intel)
- [linux-arm64](https://github.com/douglance/railguard/releases/latest/download/railguard-linux-arm64.tar.gz)
- [linux-x64](https://github.com/douglance/railguard/releases/latest/download/railguard-linux-x64.tar.gz)

```bash
# Example: macOS Apple Silicon
tar -xzf railguard-darwin-arm64.tar.gz
mv railguard ~/.local/bin/
railguard install
```

### From Source

```bash
git clone https://github.com/douglance/railguard.git
cd railguard
cargo build --release
cp target/release/railguard ~/.local/bin/
railguard install
```

## Quick Start

### 1. Install and Configure

```bash
# Install the hook
railguard install

# Verify installation
railguard --version
```

### 2. Create Policy (Optional)

Create `railguard.toml` in your project or home directory:

```toml
[policy]
mode = "strict"  # "strict" blocks, "monitor" logs only

[policy.secrets]
enabled = true
# Add custom patterns
patterns = [
    { name = "Slack Token", pattern = "xox[baprs]-[0-9a-zA-Z-]+" }
]

[policy.commands]
enabled = true
# Allow specific dangerous patterns if needed
allow_patterns = ["rm -rf ./node_modules"]

[policy.paths]
enabled = true
# Add custom protected paths
patterns = ["*.pem", "*.key"]

[policy.network]
enabled = true
blocked_domains = ["evil.com"]

# Tool-level permissions
[tools]
allow = ["Bash", "Read", "Write", "Edit", "Glob", "Grep"]
deny = ["mcp__*"]  # Block all MCP tools by default
ask = []           # Prompt for confirmation

# MCP server permissions
[mcp.servers.github]
tools.allow = ["get_issue", "list_issues"]
tools.deny = ["delete_*"]
```

### 3. Test Your Policy

```bash
# Test a safe command
railguard test Bash '{"command":"ls -la"}'
# Result: ALLOWED

# Test a dangerous command
railguard test Bash '{"command":"rm -rf /"}'
# Result: DENIED
# Reason: Dangerous command blocked

# Test secret detection
railguard test Write '{"content":"aws_secret_access_key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"}'
# Result: DENIED
# Reason: Secret detected in content
```

### 4. Validate Configuration

```bash
railguard lint
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `railguard install` | Configure Claude Code to use Railguard |
| `railguard uninstall` | Remove Railguard from Claude Code |
| `railguard lint` | Validate configuration file |
| `railguard test <tool> <json>` | Test policy against specific input |
| `railguard hook` | Run as hook (used internally by Claude Code) |

## How It Works

Railguard integrates with Claude Code's hook system. When Claude attempts to use a tool:

1. Claude Code calls `railguard hook` with JSON input
2. Railguard inspects the tool name and input
3. Policy engine checks: secrets, commands, paths, network, permissions
4. Returns verdict: `allow`, `deny` (with reason), or `ask` (prompt user)
5. Claude Code proceeds or blocks based on verdict

## Architecture

```
railguard/
├── bin/rg/           # CLI binary
│   └── src/
│       ├── cli.rs        # Argument parsing
│       ├── hook.rs       # Hook implementation
│       ├── install.rs    # Install/uninstall
│       └── lint.rs       # Config validation
├── crates/
│   ├── rg-types/     # Config, Verdict, HookInput types
│   └── rg-policy/    # Policy engine
│       ├── secrets.rs    # Secret detection
│       ├── commands.rs   # Dangerous command blocking
│       ├── paths.rs      # Protected path detection
│       ├── network.rs    # Network exfiltration prevention
│       └── tools.rs      # Tool permission matching
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Security

See [SECURITY.md](SECURITY.md) for:
- How to report vulnerabilities
- Security best practices
- Supported versions

## License

MIT - see [LICENSE](LICENSE)
