//! Hook mode for Claude Code integration.
//!
//! Reads JSON from stdin, inspects against policy, and outputs Claude Code-native
//! hookSpecificOutput JSON to stdout.
//!
//! # Output Format
//!
//! All responses use Claude Code's native `hookSpecificOutput` format:
//!
//! ```json
//! {
//!   "hookSpecificOutput": {
//!     "hookEventName": "PreToolUse",
//!     "permissionDecision": "allow" | "deny" | "ask",
//!     "permissionDecisionReason": "...",  // for deny/ask
//!     "additionalContext": "..."          // for deny
//!   }
//! }
//! ```

use std::io::{self, BufRead};
use std::process::ExitCode;

use rg_policy::{inspect, RuntimePolicy};
use rg_types::{HookInput, Verdict};

/// Run as a Claude Code hook.
///
/// - Reads JSON from stdin
/// - Parses as `HookInput`
/// - Inspects against policy
/// - Outputs hookSpecificOutput JSON to stdout
/// - Exit codes: 0 = allow/ask, 2 = deny
pub fn run_hook(policy: &RuntimePolicy) -> ExitCode {
    // Read from stdin
    let stdin = io::stdin();
    let mut input_str = String::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                input_str.push_str(&l);
                input_str.push('\n');
            }
            Err(e) => {
                output_error(&format!("Failed to read stdin: {e}"));
                return ExitCode::from(2); // Fail closed on errors
            }
        }
    }

    // Parse JSON
    let input: HookInput = match serde_json::from_str(&input_str) {
        Ok(i) => i,
        Err(e) => {
            output_error(&format!("Failed to parse JSON: {e}"));
            return ExitCode::from(2); // Fail closed on parse errors
        }
    };

    // Inspect
    let (verdict, _latency) = inspect(&input, policy);

    // Output Claude Code-native format
    output_verdict(&verdict);

    // Exit code: 0 = allow/ask, 2 = deny
    match verdict {
        Verdict::Allow | Verdict::Ask { .. } => ExitCode::SUCCESS,
        Verdict::Deny { .. } => ExitCode::from(2),
    }
}

/// Output a verdict as Claude Code-native hookSpecificOutput JSON.
fn output_verdict(verdict: &Verdict) {
    let output = match verdict {
        Verdict::Allow => serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow"
            }
        }),
        Verdict::Deny { reason, context } => {
            let mut hook_output = serde_json::json!({
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": reason
            });
            if let Some(ctx) = context {
                hook_output["additionalContext"] = serde_json::Value::String(ctx.clone());
            }
            serde_json::json!({ "hookSpecificOutput": hook_output })
        }
        Verdict::Ask { reason } => serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "ask",
                "permissionDecisionReason": reason
            }
        }),
    };

    // JSON serialization of simple JSON values cannot fail
    #[allow(clippy::expect_used)]
    let json = serde_json::to_string(&output).expect("JSON serialization failed");
    println!("{json}");
}

/// Output an error as a deny verdict.
fn output_error(message: &str) {
    let output = serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": message,
            "additionalContext": "Railguard encountered an error and is operating in fail-closed mode."
        }
    });
    // JSON serialization of simple JSON values cannot fail
    #[allow(clippy::expect_used)]
    let json = serde_json::to_string(&output).expect("JSON serialization failed");
    println!("{json}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rg_types::PolicyConfig;

    #[test]
    fn test_hook_allowed() {
        let config = PolicyConfig::default();
        let policy = RuntimePolicy::from_config(&config);

        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "ls -la" }),
        };

        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_allow());
    }

    #[test]
    fn test_hook_denied() {
        let config = PolicyConfig::default();
        let policy = RuntimePolicy::from_config(&config);

        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "rm -rf /" }),
        };

        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_deny());
    }

    #[test]
    fn test_verdict_output_allow() {
        let verdict = Verdict::allow();
        // Just verify it doesn't panic
        let output = match &verdict {
            Verdict::Allow => serde_json::json!({
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "allow"
                }
            }),
            _ => panic!("Expected Allow"),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"permissionDecision\":\"allow\""));
    }

    #[test]
    fn test_verdict_output_deny() {
        let verdict = Verdict::deny_with_context("Blocked", "Context");
        let output = match &verdict {
            Verdict::Deny { reason, context } => {
                let mut hook_output = serde_json::json!({
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": reason
                });
                if let Some(ctx) = context {
                    hook_output["additionalContext"] = serde_json::Value::String(ctx.clone());
                }
                serde_json::json!({ "hookSpecificOutput": hook_output })
            }
            _ => panic!("Expected Deny"),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"permissionDecision\":\"deny\""));
        assert!(json.contains("\"additionalContext\":\"Context\""));
    }

    #[test]
    fn test_verdict_output_ask() {
        let verdict = Verdict::ask("Confirm?");
        let output = match &verdict {
            Verdict::Ask { reason } => serde_json::json!({
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "ask",
                    "permissionDecisionReason": reason
                }
            }),
            _ => panic!("Expected Ask"),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"permissionDecision\":\"ask\""));
    }
}
