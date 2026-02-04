//! Structured block reasons for policy violations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Structured reason for why a tool use was blocked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum BlockReason {
    /// A secret was detected in the input.
    SecretDetected {
        /// Type of secret detected (e.g., "`aws_key`", "`github_token`")
        secret_type: String,
        /// Redacted preview of the secret
        redacted: String,
    },

    /// A dangerous command pattern was detected.
    DangerousCommand {
        /// The pattern that matched
        pattern: String,
        /// The matched portion of the command
        matched: String,
    },

    /// Access to a protected path was attempted.
    ProtectedPath {
        /// The path that was accessed
        path: String,
        /// The pattern that matched
        pattern: String,
    },

    /// Potential network exfiltration detected.
    NetworkExfiltration {
        /// The blocked domain
        domain: String,
    },

    /// Internal error (fail-closed behavior).
    InternalError {
        /// Error message
        message: String,
    },
}

impl BlockReason {
    /// Get the reason code as a string.
    pub fn code(&self) -> &'static str {
        match self {
            Self::SecretDetected { .. } => "secret_detected",
            Self::DangerousCommand { .. } => "dangerous_command",
            Self::ProtectedPath { .. } => "protected_path",
            Self::NetworkExfiltration { .. } => "network_exfiltration",
            Self::InternalError { .. } => "internal_error",
        }
    }
}

impl fmt::Display for BlockReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SecretDetected {
                secret_type,
                redacted,
            } => {
                write!(f, "Secret detected ({secret_type}): {redacted}")
            }
            Self::DangerousCommand { pattern, matched } => {
                write!(
                    f,
                    "Dangerous command blocked: '{matched}' matches pattern '{pattern}'"
                )
            }
            Self::ProtectedPath { path, pattern } => {
                write!(
                    f,
                    "Protected path blocked: '{path}' matches pattern '{pattern}'"
                )
            }
            Self::NetworkExfiltration { domain } => {
                write!(
                    f,
                    "Network exfiltration blocked: domain '{domain}' is not allowed"
                )
            }
            Self::InternalError { message } => {
                write!(f, "Internal error: {message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_reason_codes() {
        let reason = BlockReason::SecretDetected {
            secret_type: "aws_key".to_string(),
            redacted: "AKIA...XXXX".to_string(),
        };
        assert_eq!(reason.code(), "secret_detected");

        let reason = BlockReason::DangerousCommand {
            pattern: "rm -rf".to_string(),
            matched: "rm -rf /".to_string(),
        };
        assert_eq!(reason.code(), "dangerous_command");
    }

    #[test]
    fn test_block_reason_display() {
        let reason = BlockReason::SecretDetected {
            secret_type: "github_token".to_string(),
            redacted: "ghp_...".to_string(),
        };
        let display = reason.to_string();
        assert!(display.contains("Secret detected"));
        assert!(display.contains("github_token"));
    }

    #[test]
    fn test_block_reason_serialization() {
        let reason = BlockReason::DangerousCommand {
            pattern: "rm -rf".to_string(),
            matched: "rm -rf /".to_string(),
        };

        let json = serde_json::to_string(&reason).unwrap();
        assert!(json.contains("\"code\":\"dangerous_command\""));

        let parsed: BlockReason = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, reason);
    }
}
