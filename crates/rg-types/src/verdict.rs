//! Policy evaluation result types for Claude Code native integration.

use serde::Serialize;

use crate::BlockReason;

/// Result of a policy check - maps to Claude Code's permission decisions.
///
/// Claude Code hooks support three decisions:
/// - `allow`: Tool proceeds silently
/// - `deny`: Tool is blocked, reason shown to Claude
/// - `ask`: User is prompted for confirmation
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum Verdict {
    /// Action is allowed to proceed silently.
    #[default]
    Allow,

    /// Action is blocked with a reason.
    Deny {
        /// Human-readable reason for the denial.
        reason: String,
        /// Additional context for Claude (optional).
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<String>,
    },

    /// Action requires user confirmation.
    Ask {
        /// Human-readable reason for asking.
        reason: String,
    },
}

impl Verdict {
    /// Create an allow verdict.
    pub fn allow() -> Self {
        Verdict::Allow
    }

    /// Create a deny verdict with a reason.
    pub fn deny(reason: impl Into<String>) -> Self {
        Verdict::Deny {
            reason: reason.into(),
            context: None,
        }
    }

    /// Create a deny verdict with reason and context.
    pub fn deny_with_context(reason: impl Into<String>, context: impl Into<String>) -> Self {
        Verdict::Deny {
            reason: reason.into(),
            context: Some(context.into()),
        }
    }

    /// Create a deny verdict from a `BlockReason`.
    pub fn deny_from_block_reason(block_reason: &BlockReason) -> Self {
        Verdict::Deny {
            reason: block_reason.to_string(),
            context: Some(Self::context_for_block_reason(block_reason)),
        }
    }

    /// Create an ask verdict with a reason.
    pub fn ask(reason: impl Into<String>) -> Self {
        Verdict::Ask {
            reason: reason.into(),
        }
    }

    /// Check if this verdict allows the action.
    pub fn is_allow(&self) -> bool {
        matches!(self, Verdict::Allow)
    }

    /// Check if this verdict denies the action.
    pub fn is_deny(&self) -> bool {
        matches!(self, Verdict::Deny { .. })
    }

    /// Check if this verdict asks the user.
    pub fn is_ask(&self) -> bool {
        matches!(self, Verdict::Ask { .. })
    }

    /// Get the reason string (for deny or ask).
    pub fn reason(&self) -> Option<&str> {
        match self {
            Verdict::Allow => None,
            Verdict::Deny { reason, .. } | Verdict::Ask { reason } => Some(reason),
        }
    }

    /// Get the context string (for deny only).
    pub fn context(&self) -> Option<&str> {
        match self {
            Verdict::Deny { context, .. } => context.as_deref(),
            _ => None,
        }
    }

    /// Get the permission decision string for Claude Code.
    pub fn permission_decision(&self) -> &'static str {
        match self {
            Verdict::Allow => "allow",
            Verdict::Deny { .. } => "deny",
            Verdict::Ask { .. } => "ask",
        }
    }

    /// Generate context hints based on block reason type.
    fn context_for_block_reason(reason: &BlockReason) -> String {
        match reason {
            BlockReason::SecretDetected { .. } => {
                "This content contains secrets. Use environment variables or a secrets manager instead.".to_string()
            }
            BlockReason::DangerousCommand { .. } => {
                "This command matches a dangerous pattern. Use more targeted commands or adjust your policy.".to_string()
            }
            BlockReason::ProtectedPath { .. } => {
                "This file is protected by policy. Check railgun.toml for allowed paths.".to_string()
            }
            BlockReason::NetworkExfiltration { .. } => {
                "This domain is blocked to prevent data exfiltration. Add to allow list if needed.".to_string()
            }
            BlockReason::InternalError { .. } => {
                "An internal error occurred. Railgun is operating in fail-closed mode.".to_string()
            }
        }
    }
}

// ============================================================================
// Legacy compatibility - these methods maintain backward compatibility
// with code that used the old Allowed/Blocked variants
// ============================================================================

impl Verdict {
    /// Legacy: Create a blocked verdict from a `BlockReason`.
    #[deprecated(note = "Use deny_from_block_reason instead")]
    pub fn blocked(reason: &BlockReason) -> Self {
        Self::deny_from_block_reason(reason)
    }

    /// Legacy: Create a blocked verdict from a string.
    #[deprecated(note = "Use deny instead")]
    pub fn blocked_str(reason: impl Into<String>) -> Self {
        Verdict::deny(reason)
    }

    /// Legacy: Check if blocked.
    #[deprecated(note = "Use is_deny instead")]
    pub fn is_blocked(&self) -> bool {
        self.is_deny()
    }

    /// Legacy: Check if allowed.
    #[deprecated(note = "Use is_allow instead")]
    pub fn is_allowed(&self) -> bool {
        self.is_allow()
    }

    /// Legacy: Get block reason - not applicable to new model.
    #[deprecated(note = "Use reason() instead")]
    pub fn block_reason(&self) -> Option<BlockReason> {
        // Cannot reconstruct BlockReason from string, return None
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verdict_allow() {
        let verdict = Verdict::allow();
        assert!(verdict.is_allow());
        assert!(!verdict.is_deny());
        assert!(!verdict.is_ask());
        assert!(verdict.reason().is_none());
        assert_eq!(verdict.permission_decision(), "allow");
    }

    #[test]
    fn test_verdict_deny() {
        let verdict = Verdict::deny("Test denial");
        assert!(!verdict.is_allow());
        assert!(verdict.is_deny());
        assert!(!verdict.is_ask());
        assert_eq!(verdict.reason(), Some("Test denial"));
        assert_eq!(verdict.permission_decision(), "deny");
    }

    #[test]
    fn test_verdict_deny_with_context() {
        let verdict = Verdict::deny_with_context("Blocked", "Try another approach");
        assert!(verdict.is_deny());
        assert_eq!(verdict.reason(), Some("Blocked"));
        assert_eq!(verdict.context(), Some("Try another approach"));
    }

    #[test]
    fn test_verdict_ask() {
        let verdict = Verdict::ask("Needs confirmation");
        assert!(!verdict.is_allow());
        assert!(!verdict.is_deny());
        assert!(verdict.is_ask());
        assert_eq!(verdict.reason(), Some("Needs confirmation"));
        assert_eq!(verdict.permission_decision(), "ask");
    }

    #[test]
    fn test_verdict_from_block_reason() {
        let reason = BlockReason::SecretDetected {
            secret_type: "aws_key".to_string(),
            redacted: "AKIA...".to_string(),
        };
        let verdict = Verdict::deny_from_block_reason(&reason);
        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("Secret detected"));
        assert!(verdict.context().is_some());
    }

    #[test]
    fn test_verdict_default() {
        let verdict = Verdict::default();
        assert!(verdict.is_allow());
    }

    #[test]
    fn test_verdict_serialization() {
        let allow = Verdict::allow();
        let deny = Verdict::deny_with_context("blocked", "context");
        let ask = Verdict::ask("confirm?");

        let allow_json = serde_json::to_string(&allow).unwrap();
        let deny_json = serde_json::to_string(&deny).unwrap();
        let ask_json = serde_json::to_string(&ask).unwrap();

        assert_eq!(allow_json, r#""allow""#);
        assert!(deny_json.contains("deny"));
        assert!(deny_json.contains("blocked"));
        assert!(ask_json.contains("ask"));
    }
}
