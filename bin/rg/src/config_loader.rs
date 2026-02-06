//! Configuration file loading.

use eyre::{Context, Result};
use rg_types::Config;
use std::path::{Path, PathBuf};

/// Get the global config path (~/.config/railgun/railgun.toml)
fn global_config_path() -> Option<PathBuf> {
    dirs_next::config_dir().map(|p| p.join("railgun").join("railgun.toml"))
}

/// Load and parse the Railgun configuration file.
///
/// Config resolution order:
/// 1. Specified path (if exists)
/// 2. ~/.config/railgun/railgun.toml (if exists)
/// 3. Default config
pub fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let path = path.as_ref();

    // Try specified path first
    if path.exists() {
        return load_from_path(path);
    }

    // Try global config
    if let Some(global_path) = global_config_path() {
        if global_path.exists() {
            return load_from_path(&global_path);
        }
    }

    // Return default config
    Ok(Config::default())
}

fn load_from_path(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: Config =
        toml::from_str(&content).with_context(|| "Failed to parse config file as TOML")?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_basic() {
        let config_content = r#"
[policy]
mode = "strict"
fail_closed = true

[policy.secrets]
enabled = true

[policy.commands]
enabled = true
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let config = load_config(temp_file.path()).unwrap();
        assert!(config.policy.fail_closed);
    }

    #[test]
    fn test_load_config_default_on_missing() {
        let config = load_config("/nonexistent/path/config.toml").unwrap();
        // Should return default config
        assert!(config.policy.secrets.enabled);
    }

    #[test]
    fn test_load_config_with_all_sections() {
        let config_content = r#"
[policy]
mode = "monitor"
fail_closed = false

[policy.secrets]
enabled = true
entropy_threshold = 4.0
detect_aws_keys = true

[policy.commands]
enabled = true
block_patterns = ["rm -rf"]
allow_patterns = ["rm -rf node_modules"]

[policy.protected_paths]
enabled = true
blocked = ["**/.env"]

[policy.network]
enabled = true
block_domains = ["evil.com"]
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let config = load_config(temp_file.path()).unwrap();
        assert!(!config.policy.fail_closed);
        assert!((config.policy.secrets.entropy_threshold - 4.0).abs() < f64::EPSILON);
    }
}
