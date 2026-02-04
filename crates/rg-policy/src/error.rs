//! Error types for the policy engine.

use thiserror::Error;

/// Errors that can occur during policy operations.
#[derive(Debug, Error)]
pub enum PolicyError {
    /// Invalid regex pattern in configuration.
    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
