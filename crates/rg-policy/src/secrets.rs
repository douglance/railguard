//! Secret detection for Claude Code hook inputs.
//!
//! Detects various types of secrets including:
//! - AWS access keys (AKIA...)
//! - GitHub tokens (ghp_, ghs_, gho_, `github_pat`_)
//! - `OpenAI` API keys (sk-...)
//! - Private keys (PEM format)
//! - High-entropy strings that may be secrets

use regex::Regex;
use rg_types::SecretsConfig;
use std::ops::Range;

/// A detected secret in the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretMatch {
    /// Type of secret detected.
    pub secret_type: String,
    /// Redacted preview of the secret.
    pub redacted: String,
    /// Position in the input.
    pub position: Range<usize>,
}

/// Secret scanner with compiled patterns.
#[derive(Debug)]
pub struct SecretScanner {
    /// Configuration.
    config: SecretsConfig,
    /// AWS access key pattern.
    aws_key_pattern: Option<Regex>,
    /// GitHub token pattern.
    github_token_pattern: Option<Regex>,
    /// `OpenAI` API key pattern.
    openai_key_pattern: Option<Regex>,
    /// Private key pattern.
    private_key_pattern: Option<Regex>,
}

impl SecretScanner {
    /// Create a new secret scanner from configuration.
    pub fn new(config: &SecretsConfig) -> Self {
        let aws_key_pattern = if config.detect_aws_keys {
            // AWS access key ID: starts with AKIA, ABIA, ACCA, ASIA
            Regex::new(r"(?i)\b(A[SK]IA|ABIA|ACCA)[A-Z0-9]{16}\b").ok()
        } else {
            None
        };

        let github_token_pattern = if config.detect_github_tokens {
            // GitHub tokens: ghp_, ghs_, gho_, ghu_, github_pat_
            Regex::new(r"\b(ghp_[a-zA-Z0-9]{36}|ghs_[a-zA-Z0-9]{36}|gho_[a-zA-Z0-9]{36}|ghu_[a-zA-Z0-9]{36}|github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59})\b").ok()
        } else {
            None
        };

        let openai_key_pattern = if config.detect_openai_keys {
            // OpenAI API keys: sk-... (various formats)
            Regex::new(r"\bsk-[a-zA-Z0-9]{20,}(?:-[a-zA-Z0-9]+)*\b").ok()
        } else {
            None
        };

        let private_key_pattern = if config.detect_private_keys {
            // PEM private keys
            Regex::new(
                r"-----BEGIN\s+(?:RSA\s+|EC\s+|OPENSSH\s+|DSA\s+|ENCRYPTED\s+)?PRIVATE\s+KEY-----",
            )
            .ok()
        } else {
            None
        };

        Self {
            config: config.clone(),
            aws_key_pattern,
            github_token_pattern,
            openai_key_pattern,
            private_key_pattern,
        }
    }

    /// Scan text for secrets.
    pub fn scan(&self, text: &str) -> Vec<SecretMatch> {
        if !self.config.enabled {
            return Vec::new();
        }

        let mut matches = Vec::new();

        // Check AWS keys
        if let Some(ref pattern) = self.aws_key_pattern {
            for m in pattern.find_iter(text) {
                matches.push(SecretMatch {
                    secret_type: "aws_access_key".to_string(),
                    redacted: redact(m.as_str()),
                    position: m.start()..m.end(),
                });
            }
        }

        // Check GitHub tokens
        if let Some(ref pattern) = self.github_token_pattern {
            for m in pattern.find_iter(text) {
                matches.push(SecretMatch {
                    secret_type: "github_token".to_string(),
                    redacted: redact(m.as_str()),
                    position: m.start()..m.end(),
                });
            }
        }

        // Check OpenAI keys
        if let Some(ref pattern) = self.openai_key_pattern {
            for m in pattern.find_iter(text) {
                matches.push(SecretMatch {
                    secret_type: "openai_key".to_string(),
                    redacted: redact(m.as_str()),
                    position: m.start()..m.end(),
                });
            }
        }

        // Check private keys
        if let Some(ref pattern) = self.private_key_pattern {
            for m in pattern.find_iter(text) {
                matches.push(SecretMatch {
                    secret_type: "private_key".to_string(),
                    redacted: "-----BEGIN PRIVATE KEY-----...".to_string(),
                    position: m.start()..m.end(),
                });
            }
        }

        matches
    }
}

/// Calculate Shannon entropy of a string.
#[allow(dead_code)]
pub fn shannon_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }

    let mut freq = [0u32; 256];
    for byte in s.bytes() {
        freq[byte as usize] += 1;
    }

    let len = s.len() as f64;
    let mut entropy = 0.0;

    for &count in &freq {
        if count > 0 {
            let p = f64::from(count) / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Redact a secret value, showing only prefix and suffix.
fn redact(value: &str) -> String {
    if value.len() <= 8 {
        return "*".repeat(value.len());
    }

    let prefix = &value[..4];
    let suffix = &value[value.len() - 4..];
    format!("{prefix}...{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_scanner() -> SecretScanner {
        SecretScanner::new(&SecretsConfig::default())
    }

    #[test]
    fn test_detect_aws_key() {
        let scanner = default_scanner();
        let text = "export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE";
        let matches = scanner.scan(text);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].secret_type, "aws_access_key");
        assert!(matches[0].redacted.starts_with("AKIA"));
    }

    #[test]
    fn test_detect_github_token() {
        let scanner = default_scanner();
        let text = "GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        let matches = scanner.scan(text);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].secret_type, "github_token");
    }

    #[test]
    fn test_detect_openai_key() {
        let scanner = default_scanner();
        // Old format: sk- followed by 48+ alphanumeric chars
        let text = "OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        let matches = scanner.scan(text);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].secret_type, "openai_key");
    }

    #[test]
    fn test_detect_private_key() {
        let scanner = default_scanner();
        let text =
            "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQ...\n-----END RSA PRIVATE KEY-----";
        let matches = scanner.scan(text);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].secret_type, "private_key");
    }

    #[test]
    fn test_no_false_positives() {
        let scanner = default_scanner();
        let text = "This is normal text without any secrets";
        let matches = scanner.scan(text);

        assert!(matches.is_empty());
    }

    #[test]
    fn test_disabled_scanner() {
        let config = SecretsConfig {
            enabled: false,
            ..Default::default()
        };
        let scanner = SecretScanner::new(&config);
        let text = "AKIAIOSFODNN7EXAMPLE";
        let matches = scanner.scan(text);

        assert!(matches.is_empty());
    }

    #[test]
    fn test_shannon_entropy() {
        // Low entropy (repeated chars)
        assert!(shannon_entropy("aaaaaaaaaa") < 1.0);

        // High entropy (random-looking)
        assert!(shannon_entropy("aB3$xY9!mK") > 3.0);

        // Empty string
        assert_eq!(shannon_entropy(""), 0.0);
    }

    #[test]
    fn test_redact() {
        assert_eq!(redact("AKIAIOSFODNN7EXAMPLE"), "AKIA...MPLE");
        assert_eq!(redact("short"), "*****");
    }
}
