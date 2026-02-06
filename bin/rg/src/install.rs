//! Install Railgun as a Claude Code hook.

use std::path::PathBuf;

use eyre::{Context, Result};
use serde_json::{json, Value};

/// Get the path to Claude Code settings file.
fn get_settings_path() -> Result<PathBuf> {
    let home =
        dirs_next::home_dir().ok_or_else(|| eyre::eyre!("Could not determine home directory"))?;
    Ok(home.join(".claude").join("settings.json"))
}

/// Install Railgun as a Claude Code hook.
pub fn run_install() -> Result<()> {
    let settings_path = get_settings_path()?;

    // Get current binary path
    let binary_path =
        std::env::current_exe().with_context(|| "Could not determine current executable path")?;

    let binary_str = binary_path.to_string_lossy();

    // Read existing settings or create new
    let mut settings: Value = if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)
            .with_context(|| format!("Failed to read {}", settings_path.display()))?;
        serde_json::from_str(&content).with_context(|| "Failed to parse settings.json")?
    } else {
        json!({})
    };

    // Ensure hooks object exists
    if settings.get("hooks").is_none() {
        settings["hooks"] = json!({});
    }

    // Create hook command
    let hook_command = format!("{binary_str} hook");

    // Check if PreToolUse already has our hook
    let hooks = settings["hooks"]
        .as_object_mut()
        .ok_or_else(|| eyre::eyre!("hooks is not an object"))?;

    let pre_tool_use = hooks.entry("PreToolUse").or_insert(json!([]));

    if let Some(arr) = pre_tool_use.as_array_mut() {
        // Check if hook already exists (look inside hooks arrays)
        let already_installed = arr.iter().any(|entry| {
            if let Some(obj) = entry.as_object() {
                // Check nested hooks array
                if let Some(hooks_arr) = obj.get("hooks").and_then(|h| h.as_array()) {
                    return hooks_arr.iter().any(|hook| {
                        hook.get("command")
                            .and_then(|c| c.as_str())
                            .is_some_and(|s| s.contains("railgun"))
                    });
                }
            }
            false
        });

        if already_installed {
            println!("Railgun hook is already installed.");
            return Ok(());
        }

        // Add new hook entry with correct format (hooks array wrapper, no matcher = all tools)
        arr.push(json!({
            "hooks": [
                {
                    "type": "command",
                    "command": hook_command
                }
            ]
        }));
    } else {
        // PreToolUse exists but isn't an array - replace it
        *pre_tool_use = json!([{
            "hooks": [
                {
                    "type": "command",
                    "command": hook_command
                }
            ]
        }]);
    }

    // Ensure parent directory exists
    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    // Write settings back
    let content =
        serde_json::to_string_pretty(&settings).with_context(|| "Failed to serialize settings")?;
    std::fs::write(&settings_path, content)
        .with_context(|| format!("Failed to write {}", settings_path.display()))?;

    println!("Successfully installed Railgun hook!");
    println!();
    println!("Hook added to: {}", settings_path.display());
    println!("Command: {hook_command}");
    println!();
    println!("Configuration file: railgun.toml (in current directory)");
    println!();
    println!(
        "To test: echo '{{\"tool_name\":\"Bash\",\"tool_input\":{{\"command\":\"ls\"}}}}' | {binary_str} hook"
    );

    Ok(())
}

/// Uninstall Railgun hook from Claude Code settings.
pub fn run_uninstall() -> Result<()> {
    let settings_path = get_settings_path()?;

    if !settings_path.exists() {
        println!("No settings file found at {}", settings_path.display());
        return Ok(());
    }

    let content = std::fs::read_to_string(&settings_path)
        .with_context(|| format!("Failed to read {}", settings_path.display()))?;

    let mut settings: Value =
        serde_json::from_str(&content).with_context(|| "Failed to parse settings.json")?;

    // Remove railgun from PreToolUse
    if let Some(hooks) = settings.get_mut("hooks") {
        if let Some(pre_tool_use) = hooks.get_mut("PreToolUse") {
            if let Some(arr) = pre_tool_use.as_array_mut() {
                arr.retain(|entry| {
                    if let Some(obj) = entry.as_object() {
                        // Check nested hooks array for railgun
                        if let Some(hooks_arr) = obj.get("hooks").and_then(|h| h.as_array()) {
                            let has_railgun = hooks_arr.iter().any(|hook| {
                                hook.get("command")
                                    .and_then(|c| c.as_str())
                                    .is_some_and(|s| s.contains("railgun"))
                            });
                            return !has_railgun;
                        }
                    }
                    true
                });
            }
        }
    }

    let content =
        serde_json::to_string_pretty(&settings).with_context(|| "Failed to serialize settings")?;
    std::fs::write(&settings_path, content)
        .with_context(|| format!("Failed to write {}", settings_path.display()))?;

    println!("Successfully uninstalled Railgun hook.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_settings_path() {
        let path = get_settings_path();
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains(".claude"));
        assert!(path.to_string_lossy().ends_with("settings.json"));
    }
}
