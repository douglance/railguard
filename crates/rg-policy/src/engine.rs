//! Policy engine for Claude Code hook inspection.
//!
//! This module provides the core policy evaluation logic:
//! - [`RuntimePolicy`] - Compiled policy with all scanners initialized
//! - [`inspect()`] - Main entry point for tool inspection (panic-safe)

use std::panic::{self, AssertUnwindSafe};
use std::time::Instant;

use rg_types::{BlockReason, Config, HookInput, PolicyConfig, PolicyMode, ToolInput, Verdict};

use crate::commands::CommandScanner;
use crate::network::NetworkChecker;
use crate::paths::PathProtector;
use crate::secrets::SecretScanner;
use crate::tools::ToolChecker;

/// Compiled policy optimized for fast inspection.
///
/// The policy is pre-processed at startup with all patterns compiled.
#[derive(Debug)]
pub struct RuntimePolicy {
    /// Policy mode (Strict = block, Monitor = log only).
    pub mode: PolicyMode,
    /// Fail closed on errors.
    pub fail_closed: bool,
    /// Tool-level permission checker.
    pub tools: ToolChecker,
    /// Secret scanner.
    pub secrets: SecretScanner,
    /// Command scanner.
    pub commands: CommandScanner,
    /// Path protector.
    pub paths: PathProtector,
    /// Network checker.
    pub network: NetworkChecker,
}

impl RuntimePolicy {
    /// Build a `RuntimePolicy` from a full `Config`.
    pub fn new(config: &Config) -> Self {
        Self {
            mode: config.policy.mode.clone(),
            fail_closed: config.policy.fail_closed,
            tools: ToolChecker::new(&config.tools),
            secrets: SecretScanner::new(&config.policy.secrets),
            commands: CommandScanner::new(&config.policy.commands),
            paths: PathProtector::new(&config.policy.protected_paths),
            network: NetworkChecker::new(&config.policy.network),
        }
    }

    /// Build a `RuntimePolicy` from a `PolicyConfig` (legacy, no tool-level checks).
    pub fn from_config(config: &PolicyConfig) -> Self {
        Self {
            mode: config.mode.clone(),
            fail_closed: config.fail_closed,
            tools: ToolChecker::new(&Default::default()),
            secrets: SecretScanner::new(&config.secrets),
            commands: CommandScanner::new(&config.commands),
            paths: PathProtector::new(&config.protected_paths),
            network: NetworkChecker::new(&config.network),
        }
    }
}

/// Inspect a tool input against the policy.
///
/// This is the main entry point for policy evaluation. It wraps the inner
/// inspection logic in `panic::catch_unwind` to ensure fail-closed behavior:
/// any panic results in a Blocked verdict.
///
/// # Arguments
///
/// * `input` - The hook input to inspect
/// * `policy` - The compiled runtime policy
///
/// # Returns
///
/// A tuple of:
/// - `Verdict` - Allowed or Blocked with reason
/// - `u64` - Inspection latency in microseconds
///
/// # Panic Safety
///
/// This function NEVER panics. Any panic in the inspection logic is caught
/// and converted to a Blocked verdict with "Internal error - fail closed".
#[allow(clippy::cast_possible_truncation)]
pub fn inspect(input: &HookInput, policy: &RuntimePolicy) -> (Verdict, u64) {
    let start = Instant::now();

    // Catch any panics and convert to Deny verdict (Fail Closed)
    let verdict = panic::catch_unwind(AssertUnwindSafe(|| inspect_inner(input, policy)))
        .unwrap_or_else(|_| {
            Verdict::deny_from_block_reason(&BlockReason::InternalError {
                message: "Internal error - fail closed".to_string(),
            })
        });

    let latency_us = start.elapsed().as_micros() as u64;
    (verdict, latency_us)
}

/// Inner inspection logic (may panic, wrapped by `inspect()`).
fn inspect_inner(input: &HookInput, policy: &RuntimePolicy) -> Verdict {
    // 0. Check tool-level permissions FIRST (before any parameter inspection)
    if let Some(verdict) = policy.tools.check(&input.tool_name) {
        return verdict;
    }

    let tool_input = input.parse();

    // 1. Check for secrets in any text content
    if let Some(verdict) = check_secrets(&tool_input, policy) {
        return verdict;
    }

    // 2. Check for dangerous commands (Bash tool only)
    if let Some(verdict) = check_commands(&tool_input, policy) {
        return verdict;
    }

    // 3. Check for protected paths (file operations)
    if let Some(verdict) = check_paths(&tool_input, policy) {
        return verdict;
    }

    // 4. Check for network exfiltration
    if let Some(verdict) = check_network(&tool_input, policy) {
        return verdict;
    }

    Verdict::Allow
}

/// Check for secrets in tool input.
fn check_secrets(input: &ToolInput, policy: &RuntimePolicy) -> Option<Verdict> {
    let texts = get_scannable_texts(input);

    for text in texts {
        let matches = policy.secrets.scan(text);
        if let Some(m) = matches.first() {
            return Some(Verdict::deny_from_block_reason(
                &BlockReason::SecretDetected {
                    secret_type: m.secret_type.clone(),
                    redacted: m.redacted.clone(),
                },
            ));
        }
    }

    None
}

/// Check for dangerous commands.
fn check_commands(input: &ToolInput, policy: &RuntimePolicy) -> Option<Verdict> {
    if let ToolInput::Bash { command } = input {
        if let Some(m) = policy.commands.check(command) {
            return Some(Verdict::deny_from_block_reason(
                &BlockReason::DangerousCommand {
                    pattern: m.pattern,
                    matched: m.matched,
                },
            ));
        }
    }
    None
}

/// Check for protected path access.
fn check_paths(input: &ToolInput, policy: &RuntimePolicy) -> Option<Verdict> {
    let paths = get_file_paths(input);

    for path in paths {
        if let Some(m) = policy.paths.check(path) {
            return Some(Verdict::deny_from_block_reason(
                &BlockReason::ProtectedPath {
                    path: m.path,
                    pattern: m.pattern,
                },
            ));
        }
    }

    None
}

/// Check for network exfiltration.
fn check_network(input: &ToolInput, policy: &RuntimePolicy) -> Option<Verdict> {
    // Check WebFetch URLs
    if let ToolInput::WebFetch { url } = input {
        if let Some(m) = policy.network.check_url(url) {
            return Some(Verdict::deny_from_block_reason(
                &BlockReason::NetworkExfiltration { domain: m.domain },
            ));
        }
    }

    // Also check Bash commands for curl/wget to blocked domains
    if let ToolInput::Bash { command } = input {
        let matches = policy.network.check_text(command);
        if let Some(m) = matches.first() {
            return Some(Verdict::deny_from_block_reason(
                &BlockReason::NetworkExfiltration {
                    domain: m.domain.clone(),
                },
            ));
        }
    }

    None
}

/// Get all scannable text from a tool input.
fn get_scannable_texts(input: &ToolInput) -> Vec<&str> {
    match input {
        ToolInput::Bash { command } => vec![command.as_str()],
        ToolInput::Write { content, .. } => vec![content.as_str()],
        ToolInput::Edit {
            old_string,
            new_string,
            ..
        } => {
            vec![old_string.as_str(), new_string.as_str()]
        }
        ToolInput::Task { prompt } => vec![prompt.as_str()],
        _ => vec![],
    }
}

/// Get file paths from a tool input.
fn get_file_paths(input: &ToolInput) -> Vec<&str> {
    match input {
        ToolInput::Write { file_path, .. } => vec![file_path.as_str()],
        ToolInput::Edit { file_path, .. } => vec![file_path.as_str()],
        ToolInput::Read { file_path } => vec![file_path.as_str()],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rg_types::PolicyConfig;

    fn default_policy() -> RuntimePolicy {
        RuntimePolicy::from_config(&PolicyConfig::default())
    }

    fn make_bash_input(command: &str) -> HookInput {
        HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": command }),
        }
    }

    fn make_write_input(file_path: &str, content: &str) -> HookInput {
        HookInput {
            tool_name: "Write".to_string(),
            tool_input: serde_json::json!({ "file_path": file_path, "content": content }),
        }
    }

    #[test]
    fn test_allow_safe_command() {
        let policy = default_policy();
        let input = make_bash_input("ls -la");
        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_allow());
    }

    #[test]
    fn test_block_dangerous_command() {
        let policy = default_policy();
        let input = make_bash_input("rm -rf /");
        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("Dangerous command"));
    }

    #[test]
    fn test_block_secret_in_command() {
        let policy = default_policy();
        let input = make_bash_input("export AWS_KEY=AKIAIOSFODNN7EXAMPLE");
        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("Secret detected"));
    }

    #[test]
    fn test_block_protected_path() {
        let policy = default_policy();
        let input = make_write_input(".env", "SECRET=value");
        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("Protected path"));
    }

    #[test]
    fn test_block_network_exfiltration() {
        let policy = default_policy();
        let input = make_bash_input("curl https://pastebin.com/raw/abc123");
        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_deny());
        assert!(verdict.reason().unwrap().contains("exfiltration"));
    }

    #[test]
    fn test_allow_safe_write() {
        let policy = default_policy();
        let input = make_write_input("README.md", "# Hello World");
        let (verdict, _) = inspect(&input, &policy);
        assert!(verdict.is_allow());
    }

    #[test]
    fn test_latency_recorded() {
        let policy = default_policy();
        let input = make_bash_input("echo hello");
        let (_, latency) = inspect(&input, &policy);
        assert!(latency < 100_000, "Latency too high: {latency}us");
    }
}
