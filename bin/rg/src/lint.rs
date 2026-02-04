//! Policy configuration linter.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Severity of a lint issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Errors prevent the config from being used.
    Error,
    /// Warnings indicate potential issues but don't prevent usage.
    Warning,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

/// A lint issue found in the configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintIssue {
    /// Severity of the issue.
    pub severity: Severity,
    /// Issue code (e.g., `invalid_regex`).
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Location in the config file (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl LintIssue {
    /// Create a new error.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            location: None,
        }
    }

    /// Create a new warning.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: code.into(),
            message: message.into(),
            location: None,
        }
    }
}

/// Result of running the linter.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LintResult {
    /// All issues found.
    pub issues: Vec<LintIssue>,
    /// Number of errors.
    pub error_count: usize,
    /// Number of warnings.
    pub warning_count: usize,
}

impl LintResult {
    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    /// Add an issue to the result.
    pub fn add(&mut self, issue: LintIssue) {
        match issue.severity {
            Severity::Error => self.error_count += 1,
            Severity::Warning => self.warning_count += 1,
        }
        self.issues.push(issue);
    }
}

/// Run the linter on a configuration file.
pub fn lint_config(path: &Path) -> LintResult {
    let mut result = LintResult::default();

    // Read the file
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            result.add(LintIssue::error(
                "file_read_error",
                format!("Failed to read config file: {e}"),
            ));
            return result;
        }
    };

    // Parse TOML
    let config: toml::Value = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            result.add(LintIssue::error(
                "toml_parse_error",
                format!("Invalid TOML syntax: {e}"),
            ));
            return result;
        }
    };

    // Validate policy section exists
    if config.get("policy").is_none() {
        result.add(LintIssue::warning(
            "missing_policy",
            "No [policy] section found, using defaults",
        ));
    }

    // Validate patterns if commands section exists
    if let Some(policy) = config.get("policy") {
        if let Some(commands) = policy.get("commands") {
            validate_patterns(commands, "block_patterns", &mut result);
            validate_patterns(commands, "allow_patterns", &mut result);
        }
        if let Some(protected_paths) = policy.get("protected_paths") {
            validate_glob_patterns(protected_paths, "blocked", &mut result);
        }
    }

    result
}

fn validate_patterns(commands: &toml::Value, field: &str, result: &mut LintResult) {
    if let Some(patterns) = commands.get(field) {
        if let Some(arr) = patterns.as_array() {
            for (i, pattern) in arr.iter().enumerate() {
                if let Some(p) = pattern.as_str() {
                    if let Err(e) = regex::Regex::new(p) {
                        result.add(LintIssue::error(
                            "invalid_regex",
                            format!("Invalid regex in {field}[{i}]: {e}"),
                        ));
                    }
                }
            }
        }
    }
}

fn validate_glob_patterns(protected_paths: &toml::Value, field: &str, result: &mut LintResult) {
    if let Some(patterns) = protected_paths.get(field) {
        if let Some(arr) = patterns.as_array() {
            for (i, pattern) in arr.iter().enumerate() {
                if let Some(p) = pattern.as_str() {
                    if let Err(e) = glob::Pattern::new(p) {
                        result.add(LintIssue::error(
                            "invalid_glob",
                            format!("Invalid glob pattern in {field}[{i}]: {e}"),
                        ));
                    }
                }
            }
        }
    }
}

/// Format lint result for human-readable output.
pub fn format_human(result: &LintResult) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    if result.issues.is_empty() {
        output.push_str("Configuration is valid\n");
        return output;
    }

    for issue in &result.issues {
        let icon = match issue.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };

        let _ = write!(output, "[{icon}] {}: {}", issue.code, issue.message);
        if let Some(ref loc) = issue.location {
            let _ = write!(output, " [{loc}]");
        }
        output.push('\n');
    }

    let _ = writeln!(
        output,
        "\n{} error(s), {} warning(s)",
        result.error_count, result.warning_count
    );

    output
}

/// Format lint result as JSON.
#[allow(dead_code)] // Used in tests; kept for future --json flag
pub fn format_json(result: &LintResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
#[allow(clippy::needless_raw_string_hashes)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn lint_str(content: &str) -> LintResult {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        lint_config(file.path())
    }

    #[test]
    fn test_lint_valid_config() {
        let result = lint_str(
            r#"
[policy]
mode = "strict"
fail_closed = true

[policy.secrets]
enabled = true

[policy.commands]
enabled = true
block_patterns = ["rm\\s+-rf\\s+/"]
"#,
        );

        assert!(!result.has_errors(), "Expected no errors: {result:?}");
    }

    #[test]
    fn test_lint_invalid_toml() {
        let result = lint_str("[invalid");

        assert!(result.has_errors());
        assert!(result.issues[0].code == "toml_parse_error");
    }

    #[test]
    fn test_lint_missing_policy() {
        let result = lint_str(
            r#"
# Empty config
"#,
        );

        // Missing policy is a warning, not an error
        assert!(!result.has_errors());
        assert!(result.warning_count > 0);
    }

    #[test]
    fn test_lint_invalid_regex() {
        let result = lint_str(
            r#"
[policy]
mode = "strict"

[policy.commands]
enabled = true
block_patterns = ["[invalid regex"]
"#,
        );

        assert!(result.has_errors());
        assert!(result.issues.iter().any(|i| i.code == "invalid_regex"));
    }

    #[test]
    fn test_format_json() {
        let mut result = LintResult::default();
        result.add(LintIssue::error("test_error", "Test message"));

        let json = format_json(&result);
        assert!(json.contains("test_error"));
        assert!(json.contains("Test message"));
    }

    #[test]
    fn test_format_human() {
        let mut result = LintResult::default();
        result.add(LintIssue::error("test_error", "Test error message"));
        result.add(LintIssue::warning("test_warning", "Test warning message"));

        let output = format_human(&result);
        assert!(output.contains("[error]"));
        assert!(output.contains("[warning]"));
        assert!(output.contains("1 error(s)"));
        assert!(output.contains("1 warning(s)"));
    }

    #[test]
    fn test_format_human_valid() {
        let result = LintResult::default();
        let output = format_human(&result);
        assert!(output.contains("Configuration is valid"));
    }
}
