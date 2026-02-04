//! Policy engine for Railguard.
//!
//! This crate provides tool inspection and policy enforcement for Claude Code hooks:
//!
//! - Secret detection (AWS keys, GitHub tokens, etc.)
//! - Dangerous command blocking
//! - Protected path enforcement
//! - Network exfiltration prevention
//!
//! The core function [`inspect()`] evaluates a tool input against the configured
//! policy rules and returns a [`Verdict`](rg_types::Verdict) (Allowed or Blocked).
//!
//! # Architecture
//!
//! The policy engine is designed for minimal latency:
//!
//! 1. **Startup**: Parse config, compile regex patterns
//! 2. **Runtime**: Fast pattern matching against tool inputs
//!
//! # Panic Safety
//!
//! The [`inspect()`] function wraps all inspection logic in `panic::catch_unwind`.
//! Any panic is converted to a Blocked verdict with "Internal error - fail closed".
//! This ensures the system fails securely even in the presence of bugs.
//!
//! # Example
//!
//! ```rust
//! use rg_policy::{RuntimePolicy, inspect};
//! use rg_types::{HookInput, PolicyConfig, Verdict};
//!
//! // Build policy from config at startup
//! let config = PolicyConfig::default();
//! let policy = RuntimePolicy::from_config(&config);
//!
//! // Inspect tool inputs at runtime
//! let input = HookInput {
//!     tool_name: "Bash".to_string(),
//!     tool_input: serde_json::json!({ "command": "ls -la" }),
//! };
//!
//! let (verdict, latency_us) = inspect(&input, &policy);
//!
//! match verdict {
//!     Verdict::Allow => println!("Tool use allowed in {}us", latency_us),
//!     Verdict::Deny { reason, .. } => println!("Denied: {}", reason),
//!     Verdict::Ask { reason } => println!("Ask user: {}", reason),
//! }
//! ```

pub mod commands;
mod engine;
mod error;
pub mod network;
pub mod paths;
pub mod secrets;
pub mod tools;

// Re-export primary API
pub use engine::{inspect, RuntimePolicy};
pub use error::PolicyError;

// Re-export scanner types for advanced use cases
pub use commands::{CommandMatch, CommandScanner};
pub use network::{NetworkChecker, NetworkMatch};
pub use paths::{PathMatch, PathProtector};
pub use secrets::{SecretMatch, SecretScanner};
pub use tools::ToolChecker;

#[cfg(test)]
mod tests {
    use super::*;
    use rg_types::{HookInput, PolicyConfig};

    #[test]
    fn test_end_to_end_allowed() {
        let config = PolicyConfig::default();
        let policy = RuntimePolicy::from_config(&config);

        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "cargo build" }),
        };

        let (verdict, latency) = inspect(&input, &policy);

        assert!(verdict.is_allow(), "Expected allowed, got: {verdict:?}");
        assert!(latency < 10_000, "Latency too high: {latency}us");
    }

    #[test]
    fn test_end_to_end_blocked() {
        let config = PolicyConfig::default();
        let policy = RuntimePolicy::from_config(&config);

        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "rm -rf /" }),
        };

        let (verdict, _) = inspect(&input, &policy);

        assert!(verdict.is_deny(), "Expected blocked, got: {verdict:?}");
        assert!(verdict.reason().unwrap().contains("Dangerous"));
    }

    #[test]
    fn test_secret_in_write_blocked() {
        let config = PolicyConfig::default();
        let policy = RuntimePolicy::from_config(&config);

        let input = HookInput {
            tool_name: "Write".to_string(),
            tool_input: serde_json::json!({
                "file_path": "config.txt",
                "content": "API_KEY=AKIAIOSFODNN7EXAMPLE"
            }),
        };

        let (verdict, _) = inspect(&input, &policy);

        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("Secret"));
    }

    #[test]
    fn test_protected_path_blocked() {
        let config = PolicyConfig::default();
        let policy = RuntimePolicy::from_config(&config);

        let input = HookInput {
            tool_name: "Read".to_string(),
            tool_input: serde_json::json!({ "file_path": ".env" }),
        };

        let (verdict, _) = inspect(&input, &policy);

        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("Protected"));
    }
}
