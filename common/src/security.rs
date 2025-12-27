// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Security event logging
//!
//! This module provides security-focused logging for tracking security-relevant events
//! such as failed validations, suspicious inputs, and resource limit violations.

use crate::error::ConversionError;

/// Security event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityEventType {
    /// File size exceeded resource limit
    FileSizeExceeded,
    /// Image dimensions exceeded resource limit
    DimensionExceeded,
    /// Mesh resource count exceeded limit
    MeshResourceExceeded,
    /// Format verification failed (spoofing attempt)
    FormatMismatch,
    /// Invalid input detected
    InvalidInput,
    /// Path validation failed
    PathValidationFailed,
    /// Output validation failed
    OutputValidationFailed,
}

/// Security event
#[derive(Debug, Clone)]
pub struct SecurityEvent {
    /// Event type
    pub event_type: SecurityEventType,
    /// Event message
    pub message: String,
    /// File path (if applicable, sanitized)
    pub file_path: Option<String>,
    /// Timestamp (Unix epoch)
    pub timestamp: u64,
}

impl SecurityEvent {
    /// Create a new security event
    pub fn new(event_type: SecurityEventType, message: String) -> Self {
        Self {
            event_type,
            message,
            file_path: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Create a security event with file path
    pub fn with_path(mut self, path: &std::path::Path) -> Self {
        // Sanitize path: only keep filename to avoid leaking sensitive information
        self.file_path = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());
        self
    }

    /// Log the security event
    ///
    /// Currently logs to stderr. In the future, this could be extended to:
    /// - Write to a security log file
    /// - Send to a security monitoring system
    /// - Trigger alerts
    pub fn log(&self) {
        let event_name = match self.event_type {
            SecurityEventType::FileSizeExceeded => "FILE_SIZE_EXCEEDED",
            SecurityEventType::DimensionExceeded => "DIMENSION_EXCEEDED",
            SecurityEventType::MeshResourceExceeded => "MESH_RESOURCE_EXCEEDED",
            SecurityEventType::FormatMismatch => "FORMAT_MISMATCH",
            SecurityEventType::InvalidInput => "INVALID_INPUT",
            SecurityEventType::PathValidationFailed => "PATH_VALIDATION_FAILED",
            SecurityEventType::OutputValidationFailed => "OUTPUT_VALIDATION_FAILED",
        };

        let path_info = self
            .file_path
            .as_ref()
            .map(|p| format!(" file={}", p))
            .unwrap_or_default();

        eprintln!(
            "[SECURITY] {} timestamp={}{} message=\"{}\"",
            event_name, self.timestamp, path_info, self.message
        );
    }
}

/// Log a security event from an error
///
/// Extracts security-relevant information from a ConversionError and logs it.
pub fn log_security_error(error: &ConversionError, file_path: Option<&std::path::Path>) {
    let (event_type, message) = match error {
        ConversionError::InvalidInput(msg) => {
            // Determine specific event type from message content
            let event_type = if msg.contains("exceeds limit") {
                if msg.contains("File size") {
                    SecurityEventType::FileSizeExceeded
                } else if msg.contains("width") || msg.contains("height") {
                    SecurityEventType::DimensionExceeded
                } else if msg.contains("Vertex count") || msg.contains("Face count") {
                    SecurityEventType::MeshResourceExceeded
                } else {
                    SecurityEventType::InvalidInput
                }
            } else {
                SecurityEventType::InvalidInput
            };
            (event_type, msg.clone())
        }
        ConversionError::InvalidFormat(msg) => {
            if msg.contains("mismatch") || msg.contains("suggests") {
                (SecurityEventType::FormatMismatch, msg.clone())
            } else {
                (SecurityEventType::InvalidInput, msg.clone())
            }
        }
        ConversionError::ValidationFailed(msg) => {
            if msg.contains("path") {
                (SecurityEventType::PathValidationFailed, msg.clone())
            } else {
                (SecurityEventType::InvalidInput, msg.clone())
            }
        }
        ConversionError::ResourceLimitExceeded(msg) => {
            (SecurityEventType::FileSizeExceeded, msg.clone())
        }
        _ => {
            // Not a security-relevant error, don't log
            return;
        }
    };

    let mut event = SecurityEvent::new(event_type, message);
    if let Some(path) = file_path {
        event = event.with_path(path);
    }
    event.log();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_event_creation() {
        let event = SecurityEvent::new(
            SecurityEventType::FileSizeExceeded,
            "File too large".to_string(),
        );
        assert_eq!(event.event_type, SecurityEventType::FileSizeExceeded);
        assert_eq!(event.message, "File too large");
    }

    #[test]
    fn test_security_event_with_path() {
        let event = SecurityEvent::new(
            SecurityEventType::FormatMismatch,
            "Format mismatch".to_string(),
        )
        .with_path(std::path::Path::new("/path/to/file.png"));
        assert_eq!(event.file_path, Some("file.png".to_string()));
    }

    #[test]
    fn test_log_security_error_file_size() {
        let error = ConversionError::InvalidInput("File size 200MB exceeds limit".to_string());
        // Should not panic
        log_security_error(&error, None);
    }

    #[test]
    fn test_log_security_error_format_mismatch() {
        let error = ConversionError::InvalidFormat(
            "Format mismatch: file extension suggests Png but content is Jpeg".to_string(),
        );
        log_security_error(&error, None);
    }
}
