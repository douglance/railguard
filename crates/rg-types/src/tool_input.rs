//! Claude Code hook input types.

use serde::{Deserialize, Serialize};

/// Input received from Claude Code via stdin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookInput {
    /// The name of the tool being invoked (e.g., "Bash", "Write", "Edit")
    pub tool_name: String,
    /// The tool-specific input as raw JSON
    pub tool_input: serde_json::Value,
}

/// Parsed tool input for specific tool types.
#[derive(Debug, Clone)]
pub enum ToolInput {
    /// Execute a shell command.
    Bash {
        /// The command to execute.
        command: String,
    },
    /// Write content to a file.
    Write {
        /// Path to the file to write.
        file_path: String,
        /// Content to write to the file.
        content: String,
    },
    /// Edit a file by replacing text.
    Edit {
        /// Path to the file to edit.
        file_path: String,
        /// Text to find and replace.
        old_string: String,
        /// Replacement text.
        new_string: String,
    },
    /// Read a file's contents.
    Read {
        /// Path to the file to read.
        file_path: String,
    },
    /// Find files matching a glob pattern.
    Glob {
        /// The glob pattern to match.
        pattern: String,
    },
    /// Search for text in files.
    Grep {
        /// The regex pattern to search for.
        pattern: String,
        /// Optional path to search in.
        path: Option<String>,
    },
    /// Fetch content from a URL.
    WebFetch {
        /// The URL to fetch.
        url: String,
    },
    /// Search the web.
    WebSearch {
        /// The search query.
        query: String,
    },
    /// Spawn a subagent task.
    Task {
        /// The prompt for the subagent.
        prompt: String,
    },
    /// Unknown tool type.
    Unknown {
        /// The name of the unrecognized tool.
        tool_name: String,
        /// The raw JSON input.
        raw: serde_json::Value,
    },
}

impl HookInput {
    /// Parse the raw tool input into a typed `ToolInput`.
    pub fn parse(&self) -> ToolInput {
        match self.tool_name.as_str() {
            "Bash" => {
                if let Some(command) = self.tool_input.get("command").and_then(|v| v.as_str()) {
                    ToolInput::Bash {
                        command: command.to_string(),
                    }
                } else {
                    ToolInput::Unknown {
                        tool_name: self.tool_name.clone(),
                        raw: self.tool_input.clone(),
                    }
                }
            }
            "Write" => {
                let file_path = self
                    .tool_input
                    .get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let content = self
                    .tool_input
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::Write { file_path, content }
            }
            "Edit" => {
                let file_path = self
                    .tool_input
                    .get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let old_string = self
                    .tool_input
                    .get("old_string")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let new_string = self
                    .tool_input
                    .get("new_string")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::Edit {
                    file_path,
                    old_string,
                    new_string,
                }
            }
            "Read" => {
                let file_path = self
                    .tool_input
                    .get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::Read { file_path }
            }
            "Glob" => {
                let pattern = self
                    .tool_input
                    .get("pattern")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::Glob { pattern }
            }
            "Grep" => {
                let pattern = self
                    .tool_input
                    .get("pattern")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let path = self
                    .tool_input
                    .get("path")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                ToolInput::Grep { pattern, path }
            }
            "WebFetch" => {
                let url = self
                    .tool_input
                    .get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::WebFetch { url }
            }
            "WebSearch" => {
                let query = self
                    .tool_input
                    .get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::WebSearch { query }
            }
            "Task" => {
                let prompt = self
                    .tool_input
                    .get("prompt")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ToolInput::Task { prompt }
            }
            _ => ToolInput::Unknown {
                tool_name: self.tool_name.clone(),
                raw: self.tool_input.clone(),
            },
        }
    }

    /// Get all text content that should be scanned for secrets/dangerous patterns.
    ///
    /// Note: This method returns an empty vec because the parsed `ToolInput`
    /// contains owned Strings that cannot outlive this method call.
    /// Callers should use `parse()` directly and extract content from the result.
    pub fn scannable_content(&self) -> Vec<&str> {
        // The parse() method creates owned Strings, so we cannot return
        // references to them. Callers should use parse() directly.
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bash_input() {
        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "ls -la" }),
        };

        match input.parse() {
            ToolInput::Bash { command } => assert_eq!(command, "ls -la"),
            _ => panic!("Expected Bash variant"),
        }
    }

    #[test]
    fn test_parse_write_input() {
        let input = HookInput {
            tool_name: "Write".to_string(),
            tool_input: serde_json::json!({
                "file_path": "/tmp/test.txt",
                "content": "hello world"
            }),
        };

        match input.parse() {
            ToolInput::Write { file_path, content } => {
                assert_eq!(file_path, "/tmp/test.txt");
                assert_eq!(content, "hello world");
            }
            _ => panic!("Expected Write variant"),
        }
    }

    #[test]
    fn test_parse_unknown_tool() {
        let input = HookInput {
            tool_name: "CustomTool".to_string(),
            tool_input: serde_json::json!({ "foo": "bar" }),
        };

        match input.parse() {
            ToolInput::Unknown { tool_name, .. } => assert_eq!(tool_name, "CustomTool"),
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_hook_input_deserialize() {
        let json = r#"{"tool_name":"Bash","tool_input":{"command":"rm -rf /"}}"#;
        let input: HookInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.tool_name, "Bash");
    }
}
