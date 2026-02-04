//! Tool-level permission checker.
//!
//! This module provides tool-level access control before parameter inspection.
//! Tools can be allowed, denied, or require user confirmation based on patterns.

use glob::Pattern;
use rg_types::{ToolsConfig, Verdict};

/// Compiled tool permission checker.
///
/// Checks tool names against allow/deny/ask patterns before parameter inspection.
/// Pattern matching order:
/// 1. Deny patterns (security-first)
/// 2. Ask patterns
/// 3. Allow patterns
/// 4. None = continue to parameter inspection
#[derive(Debug)]
pub struct ToolChecker {
    /// Patterns for tools that are completely blocked.
    deny: Vec<Pattern>,
    /// Patterns for tools that require user confirmation.
    ask: Vec<Pattern>,
    /// Patterns for tools that always proceed.
    allow: Vec<Pattern>,
    /// MCP server patterns.
    mcp_deny: Vec<Pattern>,
    mcp_ask: Vec<Pattern>,
    mcp_allow: Vec<Pattern>,
}

impl ToolChecker {
    /// Create a new `ToolChecker` from configuration.
    pub fn new(config: &ToolsConfig) -> Self {
        Self {
            deny: compile_patterns(&config.deny),
            ask: compile_patterns(&config.ask),
            allow: compile_patterns(&config.allow),
            mcp_deny: compile_mcp_patterns(&config.mcp.deny_servers),
            mcp_ask: compile_mcp_patterns(&config.mcp.ask_servers),
            mcp_allow: compile_mcp_patterns(&config.mcp.allow_servers),
        }
    }

    /// Check a tool name against permission patterns.
    ///
    /// Returns:
    /// - `Some(Verdict::Deny)` if tool matches a deny pattern
    /// - `Some(Verdict::Ask)` if tool matches an ask pattern
    /// - `Some(Verdict::Allow)` if tool matches an allow pattern
    /// - `None` if no pattern matches (continue to parameter inspection)
    pub fn check(&self, tool_name: &str) -> Option<Verdict> {
        // Check if this is an MCP tool (format: mcp__server__tool)
        if let Some(server) = extract_mcp_server(tool_name) {
            return self.check_mcp_server(server, tool_name);
        }

        // Check deny patterns first (security-first)
        for pattern in &self.deny {
            if pattern.matches(tool_name) {
                return Some(Verdict::deny(format!(
                    "Tool '{tool_name}' is blocked by policy"
                )));
            }
        }

        // Check ask patterns
        for pattern in &self.ask {
            if pattern.matches(tool_name) {
                return Some(Verdict::ask(format!(
                    "Tool '{tool_name}' requires confirmation"
                )));
            }
        }

        // Check allow patterns
        for pattern in &self.allow {
            if pattern.matches(tool_name) {
                return Some(Verdict::Allow);
            }
        }

        // No match - continue to parameter inspection
        None
    }

    /// Check MCP server permissions.
    fn check_mcp_server(&self, server: &str, tool_name: &str) -> Option<Verdict> {
        // Check deny patterns first
        for pattern in &self.mcp_deny {
            if pattern.matches(server) {
                return Some(Verdict::deny(format!(
                    "MCP server '{server}' is blocked by policy"
                )));
            }
        }

        // Check ask patterns
        for pattern in &self.mcp_ask {
            if pattern.matches(server) {
                return Some(Verdict::ask(format!(
                    "MCP server '{server}' requires confirmation"
                )));
            }
        }

        // Check allow patterns
        for pattern in &self.mcp_allow {
            if pattern.matches(server) {
                return Some(Verdict::Allow);
            }
        }

        // No MCP-specific match - check generic tool patterns
        self.check_generic(tool_name)
    }

    /// Check generic tool patterns (fallback for MCP tools).
    fn check_generic(&self, tool_name: &str) -> Option<Verdict> {
        for pattern in &self.deny {
            if pattern.matches(tool_name) {
                return Some(Verdict::deny(format!(
                    "Tool '{tool_name}' is blocked by policy"
                )));
            }
        }

        for pattern in &self.ask {
            if pattern.matches(tool_name) {
                return Some(Verdict::ask(format!(
                    "Tool '{tool_name}' requires confirmation"
                )));
            }
        }

        for pattern in &self.allow {
            if pattern.matches(tool_name) {
                return Some(Verdict::Allow);
            }
        }

        None
    }
}

/// Compile glob patterns from strings.
fn compile_patterns(patterns: &[String]) -> Vec<Pattern> {
    patterns
        .iter()
        .filter_map(|s| Pattern::new(s).ok())
        .collect()
}

/// Compile MCP server patterns (prepend mcp__ prefix matching).
fn compile_mcp_patterns(servers: &[String]) -> Vec<Pattern> {
    servers
        .iter()
        .filter_map(|s| Pattern::new(s).ok())
        .collect()
}

/// Extract MCP server name from tool name.
/// Format: `mcp__server__tool` -> Some("server")
fn extract_mcp_server(tool_name: &str) -> Option<&str> {
    if let Some(rest) = tool_name.strip_prefix("mcp__") {
        rest.split("__").next()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rg_types::{McpConfig, ToolsConfig};

    fn make_config(allow: Vec<&str>, deny: Vec<&str>, ask: Vec<&str>) -> ToolsConfig {
        ToolsConfig {
            allow: allow.into_iter().map(String::from).collect(),
            deny: deny.into_iter().map(String::from).collect(),
            ask: ask.into_iter().map(String::from).collect(),
            mcp: McpConfig::default(),
        }
    }

    #[test]
    fn test_deny_takes_precedence() {
        let config = make_config(vec!["Bash"], vec!["Bash"], vec![]);
        let checker = ToolChecker::new(&config);

        let result = checker.check("Bash");
        assert!(matches!(result, Some(Verdict::Deny { .. })));
    }

    #[test]
    fn test_ask_before_allow() {
        let config = make_config(vec!["Bash"], vec![], vec!["Bash"]);
        let checker = ToolChecker::new(&config);

        let result = checker.check("Bash");
        assert!(matches!(result, Some(Verdict::Ask { .. })));
    }

    #[test]
    fn test_allow_pattern() {
        let config = make_config(vec!["Read"], vec![], vec![]);
        let checker = ToolChecker::new(&config);

        let result = checker.check("Read");
        assert!(matches!(result, Some(Verdict::Allow)));
    }

    #[test]
    fn test_no_match_returns_none() {
        let config = make_config(vec!["Read"], vec!["Write"], vec![]);
        let checker = ToolChecker::new(&config);

        let result = checker.check("Bash");
        assert!(result.is_none());
    }

    #[test]
    fn test_glob_patterns() {
        let config = make_config(vec!["mcp__*"], vec![], vec![]);
        let checker = ToolChecker::new(&config);

        let result = checker.check("mcp__context7__query");
        assert!(matches!(result, Some(Verdict::Allow)));
    }

    #[test]
    fn test_mcp_server_extraction() {
        assert_eq!(extract_mcp_server("mcp__context7__query"), Some("context7"));
        assert_eq!(extract_mcp_server("mcp__devtools__click"), Some("devtools"));
        assert_eq!(extract_mcp_server("Bash"), None);
        assert_eq!(extract_mcp_server("Read"), None);
    }

    #[test]
    fn test_mcp_server_deny() {
        let config = ToolsConfig {
            mcp: McpConfig {
                deny_servers: vec!["evil*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let checker = ToolChecker::new(&config);

        let result = checker.check("mcp__evilserver__tool");
        assert!(matches!(result, Some(Verdict::Deny { .. })));
    }

    #[test]
    fn test_mcp_server_allow() {
        let config = ToolsConfig {
            mcp: McpConfig {
                allow_servers: vec!["context7".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let checker = ToolChecker::new(&config);

        let result = checker.check("mcp__context7__query");
        assert!(matches!(result, Some(Verdict::Allow)));
    }

    #[test]
    fn test_mcp_server_ask() {
        let config = ToolsConfig {
            mcp: McpConfig {
                ask_servers: vec!["devtools".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let checker = ToolChecker::new(&config);

        let result = checker.check("mcp__devtools__click");
        assert!(matches!(result, Some(Verdict::Ask { .. })));
    }
}
