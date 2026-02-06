//! Configuration types loaded from `railgun.toml`.

use serde::{Deserialize, Serialize};

/// Root configuration structure.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    /// Policy settings.
    #[serde(default)]
    pub policy: PolicyConfig,
    /// Tool-level permissions.
    #[serde(default)]
    pub tools: ToolsConfig,
}

/// Tool-level permission configuration.
///
/// These patterns are checked BEFORE parameter inspection.
/// Patterns use glob syntax (e.g., "mcp__*", "Read", "Bash").
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ToolsConfig {
    /// Tools that always proceed without inspection.
    #[serde(default)]
    pub allow: Vec<String>,
    /// Tools that are completely blocked.
    #[serde(default)]
    pub deny: Vec<String>,
    /// Tools that require user confirmation.
    #[serde(default)]
    pub ask: Vec<String>,
    /// MCP tool configuration.
    #[serde(default)]
    pub mcp: McpConfig,
}

/// MCP tool permission configuration.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct McpConfig {
    /// MCP servers to allow (glob patterns on server name).
    /// Example: `["context7", "devtools"]` allows `mcp__context7__*` and `mcp__devtools__*`
    #[serde(default)]
    pub allow_servers: Vec<String>,
    /// MCP servers to deny.
    #[serde(default)]
    pub deny_servers: Vec<String>,
    /// MCP servers requiring user confirmation.
    #[serde(default)]
    pub ask_servers: Vec<String>,
}

/// Policy configuration for LLM protection.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PolicyConfig {
    /// Operation mode (strict or monitor).
    #[serde(default)]
    pub mode: PolicyMode,
    /// Fail closed on errors (default: true).
    #[serde(default = "default_fail_closed")]
    pub fail_closed: bool,
    /// Secret scanning configuration.
    #[serde(default)]
    pub secrets: SecretsConfig,
    /// Dangerous command detection.
    #[serde(default)]
    pub commands: CommandsConfig,
    /// Protected path configuration.
    #[serde(default)]
    pub protected_paths: ProtectedPathsConfig,
    /// Network exfiltration detection.
    #[serde(default)]
    pub network: NetworkConfig,
}

fn default_fail_closed() -> bool {
    true
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            mode: PolicyMode::default(),
            fail_closed: default_fail_closed(),
            secrets: SecretsConfig::default(),
            commands: CommandsConfig::default(),
            protected_paths: ProtectedPathsConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

/// Policy operation mode.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyMode {
    /// Block actions that violate policy.
    #[default]
    Strict,
    /// Log violations but allow all actions.
    Monitor,
}

/// Secret scanning configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(clippy::struct_excessive_bools)] // Config structs intentionally use many bools
pub struct SecretsConfig {
    /// Enable secret scanning (default: true).
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Entropy threshold for generic secret detection (default: 4.5).
    #[serde(default = "default_entropy_threshold")]
    pub entropy_threshold: f64,
    /// Detect AWS access keys.
    #[serde(default = "default_true")]
    pub detect_aws_keys: bool,
    /// Detect GitHub tokens.
    #[serde(default = "default_true")]
    pub detect_github_tokens: bool,
    /// Detect `OpenAI` API keys.
    #[serde(default = "default_true")]
    pub detect_openai_keys: bool,
    /// Detect private keys (PEM format).
    #[serde(default = "default_true")]
    pub detect_private_keys: bool,
}

fn default_true() -> bool {
    true
}

fn default_entropy_threshold() -> f64 {
    4.5
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            entropy_threshold: default_entropy_threshold(),
            detect_aws_keys: true,
            detect_github_tokens: true,
            detect_openai_keys: true,
            detect_private_keys: true,
        }
    }
}

/// Dangerous command detection configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommandsConfig {
    /// Enable command scanning (default: true).
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Patterns to block (regex).
    #[serde(default = "default_block_patterns")]
    pub block_patterns: Vec<String>,
    /// Patterns to allow (override blocks).
    #[serde(default)]
    pub allow_patterns: Vec<String>,
}

fn default_block_patterns() -> Vec<String> {
    vec![
        r"rm\s+-rf\s+[/~]".to_string(),
        r">\s*/dev/sd[a-z]".to_string(),
        r"mkfs\.".to_string(),
        r"dd\s+if=.+of=/dev/".to_string(),
        r"chmod\s+-R\s+777\s+/".to_string(),
        r":\(\)\s*\{\s*:\|:&\s*\}\s*;".to_string(), // Fork bomb
    ]
}

impl Default for CommandsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            block_patterns: default_block_patterns(),
            allow_patterns: Vec::new(),
        }
    }
}

/// Protected paths configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProtectedPathsConfig {
    /// Enable path protection (default: true).
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Glob patterns for blocked paths.
    #[serde(default = "default_blocked_paths")]
    pub blocked: Vec<String>,
}

fn default_blocked_paths() -> Vec<String> {
    vec![
        "**/.env".to_string(),
        "**/.env.*".to_string(),
        "**/*.pem".to_string(),
        "**/*.key".to_string(),
        "**/id_rsa".to_string(),
        "**/id_ed25519".to_string(),
        "**/.ssh/**".to_string(),
        "**/.aws/credentials".to_string(),
        "**/.git/config".to_string(),
    ]
}

impl Default for ProtectedPathsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            blocked: default_blocked_paths(),
        }
    }
}

/// Network exfiltration detection configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetworkConfig {
    /// Enable network checking (default: true).
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Domains to block.
    #[serde(default = "default_blocked_domains")]
    pub block_domains: Vec<String>,
}

fn default_blocked_domains() -> Vec<String> {
    vec![
        "pastebin.com".to_string(),
        "hastebin.com".to_string(),
        "paste.ee".to_string(),
        "ghostbin.com".to_string(),
        "ngrok.io".to_string(),
        "ngrok.app".to_string(),
        "requestbin.com".to_string(),
        "hookbin.com".to_string(),
        "webhook.site".to_string(),
    ]
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            block_domains: default_blocked_domains(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.policy.mode, PolicyMode::Strict);
        assert!(config.policy.fail_closed);
        assert!(config.policy.secrets.enabled);
        assert!(config.policy.commands.enabled);
        assert!(config.policy.protected_paths.enabled);
        assert!(config.policy.network.enabled);
    }

    #[test]
    fn test_config_deserialize() {
        let toml_content = r#"
[policy]
mode = "monitor"
fail_closed = false

[policy.secrets]
enabled = true
entropy_threshold = 4.0

[policy.commands]
enabled = true
block_patterns = ["rm -rf"]
allow_patterns = ["rm -rf node_modules"]

[policy.protected_paths]
enabled = true
blocked = ["**/.env"]

[policy.network]
enabled = true
block_domains = ["evil.com"]
"#;

        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.policy.mode, PolicyMode::Monitor);
        assert!(!config.policy.fail_closed);
        assert!((config.policy.secrets.entropy_threshold - 4.0).abs() < f64::EPSILON);
        assert_eq!(config.policy.commands.block_patterns, vec!["rm -rf"]);
        assert_eq!(config.policy.network.block_domains, vec!["evil.com"]);
    }

    #[test]
    fn test_policy_mode() {
        assert_eq!(PolicyMode::default(), PolicyMode::Strict);
    }
}
