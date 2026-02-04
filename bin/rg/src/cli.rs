//! CLI argument parsing with clap.

use clap::{Parser, Subcommand};

/// Railguard - Claude Code LLM Protection Hook
///
/// Protects against secrets leakage and dangerous LLM actions.
#[derive(Parser, Debug)]
#[command(name = "railguard")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Path to configuration file
    #[arg(short, long, default_value = "railguard.toml", global = true)]
    pub config: String,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run as a Claude Code hook (reads JSON from stdin)
    ///
    /// Exit codes:
    ///   0 - Tool use allowed
    ///   2 - Tool use blocked (reason written to stderr as JSON)
    Hook,

    /// Install hook into ~/.claude/settings.json
    Install,

    /// Uninstall hook from ~/.claude/settings.json
    Uninstall,

    /// Validate configuration file
    Lint,

    /// Test policy with a specific tool input
    ///
    /// Example:
    ///   railguard test Bash '{"command":"rm -rf /"}'
    Test {
        /// Tool name (e.g., "Bash", "Write", "Edit")
        tool_name: String,
        /// Tool input as JSON
        tool_input: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_hook_command() {
        let cli = Cli::parse_from(["railguard", "hook"]);
        assert!(matches!(cli.command, Commands::Hook));
    }

    #[test]
    fn test_cli_install_command() {
        let cli = Cli::parse_from(["railguard", "install"]);
        assert!(matches!(cli.command, Commands::Install));
    }

    #[test]
    fn test_cli_uninstall_command() {
        let cli = Cli::parse_from(["railguard", "uninstall"]);
        assert!(matches!(cli.command, Commands::Uninstall));
    }

    #[test]
    fn test_cli_lint_command() {
        let cli = Cli::parse_from(["railguard", "lint"]);
        assert!(matches!(cli.command, Commands::Lint));
    }

    #[test]
    fn test_cli_test_command() {
        let cli = Cli::parse_from(["railguard", "test", "Bash", r#"{"command":"ls"}"#]);
        match cli.command {
            Commands::Test {
                tool_name,
                tool_input,
            } => {
                assert_eq!(tool_name, "Bash");
                assert!(tool_input.contains("command"));
            }
            _ => panic!("Expected Test command"),
        }
    }

    #[test]
    fn test_cli_custom_config() {
        let cli = Cli::parse_from(["railguard", "-c", "custom.toml", "hook"]);
        assert_eq!(cli.config, "custom.toml");
    }
}
