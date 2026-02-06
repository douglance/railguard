//! Shared types for Railgun.
//!
//! This crate contains the core types used throughout the Railgun system:
//!
//! - [`Config`] - Configuration structures loaded from TOML
//! - [`Verdict`] - Policy evaluation results (Allow/Deny/Ask)
//! - [`BlockReason`] - Structured block reasons for policy violations
//! - [`HookInput`] - Claude Code hook input types

mod block_reason;
mod config;
mod tool_input;
mod verdict;

// Re-export all public types
pub use block_reason::BlockReason;
pub use config::{
    CommandsConfig, Config, McpConfig, NetworkConfig, PolicyConfig, PolicyMode,
    ProtectedPathsConfig, SecretsConfig, ToolsConfig,
};
pub use tool_input::{HookInput, ToolInput};
pub use verdict::Verdict;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialize() {
        let toml_content = r#"
[policy]
mode = "strict"
fail_closed = true

[policy.secrets]
enabled = true

[policy.commands]
enabled = true
block_patterns = ["rm -rf /"]

[policy.protected_paths]
enabled = true
blocked = ["**/.env"]

[policy.network]
enabled = true
block_domains = ["pastebin.com"]
"#;

        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.policy.mode, PolicyMode::Strict);
        assert!(config.policy.fail_closed);
        assert!(config.policy.secrets.enabled);
    }

    #[test]
    fn test_verdict_serialization() {
        let allow = Verdict::allow();
        let deny = Verdict::deny_from_block_reason(&BlockReason::DangerousCommand {
            pattern: "test".to_string(),
            matched: "test".to_string(),
        });
        let ask = Verdict::ask("Confirm?");

        let allow_json = serde_json::to_string(&allow).unwrap();
        let deny_json = serde_json::to_string(&deny).unwrap();
        let ask_json = serde_json::to_string(&ask).unwrap();

        assert_eq!(allow_json, r#""allow""#);
        assert!(deny_json.contains("deny"));
        assert!(ask_json.contains("ask"));
    }

    #[test]
    fn test_hook_input_parsing() {
        let json = r#"{"tool_name":"Bash","tool_input":{"command":"ls -la"}}"#;
        let input: HookInput = serde_json::from_str(json).unwrap();

        match input.parse() {
            ToolInput::Bash { command } => assert_eq!(command, "ls -la"),
            _ => panic!("Expected Bash variant"),
        }
    }
}
