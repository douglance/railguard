//! Dangerous command detection for Claude Code hook inputs.
//!
//! Detects dangerous shell commands using regex patterns.
//! Allow patterns can override block patterns.

use regex::Regex;
use rg_types::CommandsConfig;

/// A matched dangerous command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandMatch {
    /// The pattern that matched.
    pub pattern: String,
    /// The matched portion of the command.
    pub matched: String,
}

/// Command scanner with compiled patterns.
#[derive(Debug)]
pub struct CommandScanner {
    /// Configuration.
    config: CommandsConfig,
    /// Compiled block patterns.
    block_patterns: Vec<(String, Regex)>,
    /// Compiled allow patterns (override blocks).
    allow_patterns: Vec<Regex>,
}

impl CommandScanner {
    /// Create a new command scanner from configuration.
    pub fn new(config: &CommandsConfig) -> Self {
        let block_patterns: Vec<(String, Regex)> = config
            .block_patterns
            .iter()
            .filter_map(|p| Regex::new(p).ok().map(|r| (p.clone(), r)))
            .collect();

        let allow_patterns: Vec<Regex> = config
            .allow_patterns
            .iter()
            .filter_map(|p| Regex::new(p).ok())
            .collect();

        Self {
            config: config.clone(),
            block_patterns,
            allow_patterns,
        }
    }

    /// Check if a command should be blocked.
    ///
    /// Returns `Some(CommandMatch)` if the command matches a block pattern
    /// and does NOT match any allow patterns.
    pub fn check(&self, command: &str) -> Option<CommandMatch> {
        if !self.config.enabled {
            return None;
        }

        // Check allow patterns first - if any match, command is allowed
        for allow_pattern in &self.allow_patterns {
            if allow_pattern.is_match(command) {
                return None;
            }
        }

        // Check block patterns
        for (pattern_str, block_pattern) in &self.block_patterns {
            if let Some(m) = block_pattern.find(command) {
                return Some(CommandMatch {
                    pattern: pattern_str.clone(),
                    matched: m.as_str().to_string(),
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_scanner() -> CommandScanner {
        CommandScanner::new(&CommandsConfig::default())
    }

    #[test]
    fn test_block_rm_rf_root() {
        let scanner = default_scanner();

        let result = scanner.check("rm -rf /");
        assert!(result.is_some());

        let result = scanner.check("rm -rf /home");
        assert!(result.is_some());

        let result = scanner.check("sudo rm -rf /etc");
        assert!(result.is_some());
    }

    #[test]
    fn test_block_rm_rf_home() {
        let scanner = default_scanner();

        let result = scanner.check("rm -rf ~");
        assert!(result.is_some());

        let result = scanner.check("rm -rf ~/");
        assert!(result.is_some());
    }

    #[test]
    fn test_allow_safe_rm() {
        let scanner = default_scanner();

        // Safe rm commands without -rf on root
        let result = scanner.check("rm file.txt");
        assert!(result.is_none());

        let result = scanner.check("rm -r ./temp");
        assert!(result.is_none());
    }

    #[test]
    fn test_allow_pattern_override() {
        let config = CommandsConfig {
            enabled: true,
            block_patterns: vec![r"rm\s+-rf".to_string()],
            allow_patterns: vec![r"rm\s+-rf\s+node_modules".to_string()],
        };
        let scanner = CommandScanner::new(&config);

        // This matches allow pattern, so it's allowed
        let result = scanner.check("rm -rf node_modules");
        assert!(result.is_none());

        // This doesn't match allow pattern, so it's blocked
        let result = scanner.check("rm -rf /tmp");
        assert!(result.is_some());
    }

    #[test]
    fn test_disabled_scanner() {
        let config = CommandsConfig {
            enabled: false,
            ..Default::default()
        };
        let scanner = CommandScanner::new(&config);

        let result = scanner.check("rm -rf /");
        assert!(result.is_none());
    }

    #[test]
    fn test_block_fork_bomb() {
        let scanner = default_scanner();

        let result = scanner.check(":() { :|:& } ;");
        assert!(result.is_some());
    }

    #[test]
    fn test_block_disk_operations() {
        let scanner = default_scanner();

        let result = scanner.check("dd if=/dev/zero of=/dev/sda");
        assert!(result.is_some());

        let result = scanner.check("> /dev/sda");
        assert!(result.is_some());

        let result = scanner.check("mkfs.ext4 /dev/sda1");
        assert!(result.is_some());
    }

    #[test]
    fn test_safe_commands() {
        let scanner = default_scanner();

        assert!(scanner.check("ls -la").is_none());
        assert!(scanner.check("cat file.txt").is_none());
        assert!(scanner.check("grep pattern file").is_none());
        assert!(scanner.check("echo hello").is_none());
        assert!(scanner.check("npm install").is_none());
        assert!(scanner.check("cargo build").is_none());
    }
}
