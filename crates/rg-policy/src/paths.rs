//! Protected path matching for Claude Code hook inputs.
//!
//! Uses glob patterns to block access to sensitive paths like
//! .env files, private keys, and SSH configurations.

use glob::Pattern;
use rg_types::ProtectedPathsConfig;
use std::path::Path;

/// A matched protected path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathMatch {
    /// The path that was matched.
    pub path: String,
    /// The pattern that matched.
    pub pattern: String,
}

/// Alias for `PathProtector` (for backward compatibility).
pub type PathMatcher = PathProtector;

/// Path protector with compiled glob patterns.
#[derive(Debug)]
pub struct PathProtector {
    /// Configuration.
    config: ProtectedPathsConfig,
    /// Compiled glob patterns.
    patterns: Vec<(String, Pattern)>,
}

impl PathProtector {
    /// Create a new path matcher from configuration.
    pub fn new(config: &ProtectedPathsConfig) -> Self {
        let patterns: Vec<(String, Pattern)> = config
            .blocked
            .iter()
            .filter_map(|p| Pattern::new(p).ok().map(|pat| (p.clone(), pat)))
            .collect();

        Self {
            config: config.clone(),
            patterns,
        }
    }

    /// Check if a path should be blocked.
    ///
    /// Returns true if the path matches any blocked pattern.
    pub fn is_blocked(&self, path: &str) -> bool {
        self.check(path).is_some()
    }

    /// Check if a path should be blocked.
    ///
    /// Returns `Some(PathMatch)` if the path matches any blocked pattern.
    pub fn check(&self, path: &str) -> Option<PathMatch> {
        if !self.config.enabled {
            return None;
        }

        // Normalize the path for matching
        let normalized = normalize_path(path);

        for (pattern_str, pattern) in &self.patterns {
            if pattern.matches(&normalized) || pattern.matches(path) {
                return Some(PathMatch {
                    path: path.to_string(),
                    pattern: pattern_str.clone(),
                });
            }

            // Also check the filename alone for patterns like "**/.env"
            if let Some(filename) = Path::new(path).file_name().and_then(|f| f.to_str()) {
                // For patterns like "**/.env", extract the filename part
                let pattern_filename = pattern_str.rsplit('/').next().unwrap_or(pattern_str);

                // Skip if the filename pattern is just ** (would match everything)
                if pattern_filename == "**" || pattern_filename == "*" {
                    continue;
                }

                // Check if filename matches the pattern's filename part
                if let Ok(filename_pattern) = Pattern::new(pattern_filename) {
                    if filename_pattern.matches(filename) {
                        return Some(PathMatch {
                            path: path.to_string(),
                            pattern: pattern_str.clone(),
                        });
                    }
                }
            }
        }

        None
    }
}

/// Normalize a path for matching.
fn normalize_path(path: &str) -> String {
    // Remove leading ./ if present
    let path = path.strip_prefix("./").unwrap_or(path);

    // Normalize multiple slashes
    let mut result = String::with_capacity(path.len());
    let mut prev_slash = false;

    for c in path.chars() {
        if c == '/' || c == '\\' {
            if !prev_slash {
                result.push('/');
            }
            prev_slash = true;
        } else {
            result.push(c);
            prev_slash = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_protector() -> PathProtector {
        PathProtector::new(&ProtectedPathsConfig::default())
    }

    #[test]
    fn test_block_env_files() {
        let protector = default_protector();

        assert!(protector.is_blocked(".env"));
        assert!(protector.is_blocked("/app/.env"));
        assert!(protector.is_blocked("./project/.env"));
        assert!(protector.is_blocked(".env.local"));
        assert!(protector.is_blocked(".env.production"));
    }

    #[test]
    fn test_block_private_keys() {
        let protector = default_protector();

        assert!(protector.is_blocked("server.pem"));
        assert!(protector.is_blocked("/etc/ssl/private/key.pem"));
        assert!(protector.is_blocked("id_rsa"));
        assert!(protector.is_blocked("~/.ssh/id_rsa"));
        assert!(protector.is_blocked("id_ed25519"));
        assert!(protector.is_blocked("/home/user/.ssh/id_ed25519"));
    }

    #[test]
    fn test_block_ssh_config() {
        let protector = default_protector();

        assert!(protector.is_blocked("~/.ssh/config"));
        assert!(protector.is_blocked("/home/user/.ssh/known_hosts"));
    }

    #[test]
    fn test_block_aws_credentials() {
        let protector = default_protector();

        assert!(protector.is_blocked("~/.aws/credentials"));
        assert!(protector.is_blocked("/home/user/.aws/credentials"));
    }

    #[test]
    fn test_allow_safe_paths() {
        let protector = default_protector();

        assert!(!protector.is_blocked("src/main.rs"));
        assert!(!protector.is_blocked("README.md"));
        assert!(!protector.is_blocked("package.json"));
        assert!(!protector.is_blocked("/tmp/test.txt"));
    }

    #[test]
    fn test_disabled_protector() {
        let config = ProtectedPathsConfig {
            enabled: false,
            ..Default::default()
        };
        let protector = PathProtector::new(&config);

        assert!(!protector.is_blocked(".env"));
    }

    #[test]
    fn test_custom_patterns() {
        let config = ProtectedPathsConfig {
            enabled: true,
            blocked: vec!["**/secrets/**".to_string(), "**/*.secret".to_string()],
        };
        let protector = PathProtector::new(&config);

        assert!(protector.is_blocked("/app/secrets/api_key"));
        assert!(protector.is_blocked("config.secret"));
        assert!(!protector.is_blocked("normal.txt"));
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("./foo/bar"), "foo/bar");
        assert_eq!(normalize_path("foo//bar"), "foo/bar");
        assert_eq!(normalize_path("foo\\bar"), "foo/bar");
    }
}
