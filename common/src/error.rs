// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use thiserror::Error;

/// Common error type for conversion operations
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Conversion failed: {0}")]
    ConversionFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
}

impl ConversionError {
    /// Get a user-safe error message (sanitized for display)
    ///
    /// This method returns an error message suitable for displaying to end users,
    /// with sensitive information like full file paths removed.
    pub fn user_message(&self) -> String {
        match self {
            ConversionError::Io(e) => format!("File error: {}", e.kind()),
            ConversionError::InvalidFormat(msg) => {
                format!("Invalid format: {}", Self::sanitize(msg))
            }
            ConversionError::UnsupportedFormat(msg) => {
                format!("Unsupported format: {}", Self::sanitize(msg))
            }
            ConversionError::ConversionFailed(msg) => {
                format!("Conversion failed: {}", Self::sanitize(msg))
            }
            ConversionError::ValidationFailed(msg) => {
                format!("Validation failed: {}", Self::sanitize(msg))
            }
            ConversionError::InvalidInput(msg) => {
                format!("Invalid input: {}", Self::sanitize(msg))
            }
            ConversionError::ResourceLimitExceeded(msg) => {
                format!("Resource limit exceeded: {}", Self::sanitize(msg))
            }
        }
    }

    /// Sanitize an error message for user display
    ///
    /// - Limits message length to prevent log flooding
    /// - Could be extended to remove full paths, internal details, etc.
    fn sanitize(msg: &str) -> String {
        // Limit length to 200 characters
        if msg.len() > 200 {
            format!("{}...", &msg[..197])
        } else {
            msg.to_string()
        }
    }

    /// Check if this is a resource limit error
    pub fn is_resource_limit(&self) -> bool {
        matches!(self, ConversionError::ResourceLimitExceeded(_))
            || matches!(self, ConversionError::InvalidInput(msg) if msg.contains("exceeds limit"))
    }
}

/// Result type alias for conversion operations
pub type Result<T> = std::result::Result<T, ConversionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_message_sanitization() {
        let long_msg = "a".repeat(300);
        let error = ConversionError::InvalidInput(long_msg);
        let user_msg = error.user_message();
        assert!(user_msg.len() <= 220); // "Invalid input: " + 200 chars + "..."
    }

    #[test]
    fn test_is_resource_limit() {
        let limit_error = ConversionError::ResourceLimitExceeded("too big".to_string());
        assert!(limit_error.is_resource_limit());

        let input_error = ConversionError::InvalidInput("File size exceeds limit".to_string());
        assert!(input_error.is_resource_limit());

        let other_error = ConversionError::InvalidFormat("bad format".to_string());
        assert!(!other_error.is_resource_limit());
    }
}
