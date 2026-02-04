//! Network exfiltration detection for Claude Code hook inputs.
//!
//! Detects URLs pointing to blocked domains that could be used
//! for data exfiltration (paste sites, webhook services, etc.)

use regex::Regex;
use rg_types::NetworkConfig;
use std::collections::HashSet;

/// A matched network exfiltration attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkMatch {
    /// The blocked domain.
    pub domain: String,
    /// The full URL that was matched.
    pub url: String,
}

/// Network checker for blocked domains.
#[derive(Debug)]
pub struct NetworkChecker {
    /// Configuration.
    config: NetworkConfig,
    /// Set of blocked domains for O(1) lookup.
    blocked_domains: HashSet<String>,
    /// URL extraction regex.
    url_pattern: Regex,
}

impl NetworkChecker {
    /// Create a new network checker from configuration.
    pub fn new(config: &NetworkConfig) -> Self {
        let blocked_domains: HashSet<String> = config
            .block_domains
            .iter()
            .map(|d| d.to_lowercase())
            .collect();

        // Pattern to extract URLs from text
        // This is intentionally simple - matches http(s)://domain...
        #[allow(clippy::expect_used)] // Fallback regex is a compile-time constant that cannot fail
        let url_pattern =
            Regex::new(r#"(?i)https?://([a-z0-9][-a-z0-9]*\.)+[a-z]{2,}(?:[:/][^\s"'<>]*)?"#)
                .unwrap_or_else(|_| Regex::new(r"^$").expect("fallback regex"));

        Self {
            config: config.clone(),
            blocked_domains,
            url_pattern,
        }
    }

    /// Check if a URL points to a blocked domain.
    pub fn check_url(&self, url: &str) -> Option<NetworkMatch> {
        if !self.config.enabled {
            return None;
        }

        let domain = extract_domain(url)?;

        // Check if domain or any parent domain is blocked
        if self.is_domain_blocked(&domain) {
            return Some(NetworkMatch {
                domain: domain.clone(),
                url: url.to_string(),
            });
        }

        None
    }

    /// Scan text for URLs pointing to blocked domains.
    pub fn check_text(&self, text: &str) -> Vec<NetworkMatch> {
        if !self.config.enabled {
            return Vec::new();
        }

        let mut matches = Vec::new();

        for url_match in self.url_pattern.find_iter(text) {
            let url = url_match.as_str();
            if let Some(m) = self.check_url(url) {
                matches.push(m);
            }
        }

        matches
    }

    /// Check if a domain or any of its parent domains is blocked.
    fn is_domain_blocked(&self, domain: &str) -> bool {
        let domain_lower = domain.to_lowercase();

        // Check exact match
        if self.blocked_domains.contains(&domain_lower) {
            return true;
        }

        // Check parent domains (e.g., "sub.pastebin.com" should match "pastebin.com")
        let parts: Vec<&str> = domain_lower.split('.').collect();
        for i in 1..parts.len().saturating_sub(1) {
            let parent = parts[i..].join(".");
            if self.blocked_domains.contains(&parent) {
                return true;
            }
        }

        false
    }
}

/// Extract the domain from a URL.
fn extract_domain(url: &str) -> Option<String> {
    // Remove protocol
    let without_protocol = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    // Get authority part (before first /)
    let authority = without_protocol.split('/').next()?;

    // Handle @ in URLs (user:pass@host) - get part after @
    let host_with_port = authority.rsplit('@').next()?;

    // Remove port if present (split on : and take first part)
    let domain = host_with_port.split(':').next()?;

    if domain.is_empty() {
        None
    } else {
        Some(domain.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_checker() -> NetworkChecker {
        NetworkChecker::new(&NetworkConfig::default())
    }

    #[test]
    fn test_block_pastebin() {
        let checker = default_checker();

        let result = checker.check_url("https://pastebin.com/raw/abc123");
        assert!(result.is_some());
        assert_eq!(result.unwrap().domain, "pastebin.com");
    }

    #[test]
    fn test_block_ngrok() {
        let checker = default_checker();

        assert!(checker.check_url("https://abc123.ngrok.io/api").is_some());
        assert!(checker
            .check_url("https://something.ngrok.app/endpoint")
            .is_some());
    }

    #[test]
    fn test_block_webhook_sites() {
        let checker = default_checker();

        assert!(checker.check_url("https://webhook.site/abc123").is_some());
        assert!(checker.check_url("https://requestbin.com/r/abc").is_some());
    }

    #[test]
    fn test_allow_safe_urls() {
        let checker = default_checker();

        assert!(checker.check_url("https://github.com/user/repo").is_none());
        assert!(checker.check_url("https://api.example.com/data").is_none());
        assert!(checker.check_url("https://google.com").is_none());
    }

    #[test]
    fn test_subdomain_blocking() {
        let checker = default_checker();

        // Subdomains of blocked domains should also be blocked
        assert!(checker.check_url("https://sub.pastebin.com/abc").is_some());
        assert!(checker.check_url("https://api.ngrok.io/tunnel").is_some());
    }

    #[test]
    fn test_check_text() {
        let checker = default_checker();

        let text = "Send data to https://pastebin.com/raw/abc and https://example.com/api";
        let matches = checker.check_text(text);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].url.contains("pastebin.com"));
    }

    #[test]
    fn test_disabled_checker() {
        let config = NetworkConfig {
            enabled: false,
            ..Default::default()
        };
        let checker = NetworkChecker::new(&config);

        assert!(checker.check_url("https://pastebin.com/abc").is_none());
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://example.com/path"),
            Some("example.com".to_string())
        );
        assert_eq!(
            extract_domain("http://sub.example.com:8080/path"),
            Some("sub.example.com".to_string())
        );
        assert_eq!(
            extract_domain("https://user:pass@example.com/path"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_custom_blocked_domains() {
        let config = NetworkConfig {
            enabled: true,
            block_domains: vec!["evil.com".to_string(), "malware.org".to_string()],
        };
        let checker = NetworkChecker::new(&config);

        assert!(checker.check_url("https://evil.com/steal").is_some());
        assert!(checker.check_url("https://malware.org/payload").is_some());
        assert!(checker.check_url("https://pastebin.com/abc").is_none()); // Not in custom list
    }

    #[test]
    fn test_case_insensitive() {
        let checker = default_checker();

        assert!(checker.check_url("https://PASTEBIN.COM/abc").is_some());
        assert!(checker.check_url("https://PasteBin.Com/abc").is_some());
    }
}
