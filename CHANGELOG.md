# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-02-04

### Added

#### Core Features
- **Secret Detection** - Blocks tool invocations containing secrets
  - AWS access keys and secret keys
  - GitHub tokens (classic and fine-grained)
  - OpenAI API keys
  - Private keys (RSA, EC, OpenSSH)
  - High-entropy strings detection
  - Configurable custom patterns via regex

- **Dangerous Command Blocking** - Prevents destructive shell commands
  - `rm -rf /` and recursive root deletion
  - `rm -rf ~` and home directory deletion
  - Fork bombs (`:(){ :|:& };:`)
  - Disk operations (`dd if=`, `mkfs`, `fdisk`)
  - Configurable allow patterns for exceptions

- **Protected Path Detection** - Blocks access to sensitive files
  - SSH keys and config (`~/.ssh/`)
  - AWS credentials (`~/.aws/credentials`)
  - Environment files (`.env`, `.env.local`)
  - Cloud provider configs (GCP, Azure)
  - Configurable custom path patterns

- **Network Exfiltration Prevention** - Blocks data exfiltration attempts
  - Ngrok and tunnel services
  - Pastebin and paste sites
  - Webhook testing sites (webhook.site, requestbin)
  - Configurable blocked domains list

- **Tool-Level Permissions** - Fine-grained control over Claude tools
  - Allow/deny/ask rules per tool
  - Glob pattern matching for tool names
  - MCP server-specific permissions
  - Input pattern matching for conditional rules

- **MCP Server Controls** - Security for Model Context Protocol
  - Per-server allow/deny/ask policies
  - Tool-specific overrides within servers
  - Configurable via `[mcp.servers]` section

#### CLI Commands
- `railguard install` - Configures Claude Code to use Railguard hook
- `railguard uninstall` - Removes Railguard from Claude Code settings
- `railguard lint` - Validates configuration file syntax and semantics
- `railguard test <tool> <json>` - Test policy against specific inputs
- `railguard hook` - Run as Claude Code hook (internal use)

#### Configuration
- TOML-based configuration (`railguard.toml`)
- Policy modes: `strict` (block) and `monitor` (log only)
- Sensible defaults with full customization
- Per-scanner enable/disable controls

### Technical Details

- Written in Rust for performance and safety
- Sub-millisecond latency (<1ms p99)
- Zero external runtime dependencies
- Cross-platform: macOS (Intel/ARM), Linux (x64/ARM)
- 99 tests covering all security features

[0.1.0]: https://github.com/douglance/railguard/releases/tag/v0.1.0
