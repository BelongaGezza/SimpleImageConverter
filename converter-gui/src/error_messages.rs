// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Error message mapping for user-friendly display
//!
//! This module converts technical error types into user-friendly messages
//! suitable for display in the GUI. All messages are sanitized to avoid
//! leaking sensitive information like full file paths or system details.

use common::error::ConversionError;

/// Format an error into a user-friendly message
///
/// This function converts `ConversionError` variants into simple, actionable
/// messages that non-technical users can understand. All technical jargon
/// and sensitive information (like full paths) are removed.
///
/// # Arguments
///
/// * `error` - The conversion error to format
///
/// # Returns
///
/// A user-friendly error message string
///
/// # Example
///
/// ```
/// use converter_gui::error_messages::format_user_message;
/// use common::error::ConversionError;
///
/// let error = ConversionError::UnsupportedFormat("xyz".to_string());
/// let message = format_user_message(&error);
/// assert_eq!(message, "File type not supported.");
/// ```
pub fn format_user_message(error: &ConversionError) -> String {
    match error {
        ConversionError::InvalidInput(msg) => {
            // Check for specific error patterns in the message
            if msg.contains("extension") || msg.contains("format") || msg.contains("Unsupported") {
                "File type not supported.".to_string()
            } else if msg.contains("size") || msg.contains("too large") || msg.contains("exceeds limit") {
                if msg.contains("dimension") || msg.contains("width") || msg.contains("height") {
                    "Image too large. Maximum dimension is 65535 pixels.".to_string()
                } else if msg.contains("vertices") || msg.contains("vertex") {
                    "Mesh too large. Maximum vertices is 10,000,000.".to_string()
                } else if msg.contains("faces") || msg.contains("face") {
                    "Mesh too large. Maximum faces is 10,000,000.".to_string()
                } else {
                    "File too large. Maximum size is 100 MB.".to_string()
                }
            } else if msg.contains("dimension") || msg.contains("width") || msg.contains("height") {
                "Image too large. Maximum dimension is 65535 pixels.".to_string()
            } else if msg.contains("vertices") || msg.contains("vertex") {
                "Mesh too large. Maximum vertices is 10,000,000.".to_string()
            } else if msg.contains("faces") || msg.contains("face") {
                "Mesh too large. Maximum faces is 10,000,000.".to_string()
            } else {
                "Invalid file. Check if file exists and is readable.".to_string()
            }
        }
        ConversionError::UnsupportedFormat(_) => {
            "File type not supported.".to_string()
        }
        ConversionError::InvalidFormat(msg) => {
            // Format mismatch (extension vs magic bytes)
            if msg.contains("mismatch") {
                "File type doesn't match file content.".to_string()
            } else {
                "Invalid file format.".to_string()
            }
        }
        ConversionError::Io(_) => {
            "Can't read file. Check if file exists.".to_string()
        }
        ConversionError::ConversionFailed(msg) => {
            if msg.contains("corrupted") || msg.contains("invalid") {
                "File may be corrupted or invalid.".to_string()
            } else if msg.contains("transform") || msg.contains("coordinate") {
                "Coordinate system transformation failed.".to_string()
            } else if msg.contains("normal") {
                "Normal calculation failed.".to_string()
            } else {
                "Conversion failed. Please try again.".to_string()
            }
        }
        ConversionError::ValidationFailed(msg) => {
            if msg.contains("path") {
                "Invalid file path.".to_string()
            } else if msg.contains("mesh") || msg.contains("topology") || msg.contains("geometry") {
                "Mesh validation failed. Check if mesh is valid.".to_string()
            } else {
                "File validation failed.".to_string()
            }
        }
        ConversionError::ResourceLimitExceeded(msg) => {
            if msg.contains("dimension") || msg.contains("width") || msg.contains("height") {
                "Image too large. Maximum dimension is 65535 pixels.".to_string()
            } else if msg.contains("vertices") || msg.contains("vertex") {
                "Mesh too large. Maximum vertices is 10,000,000.".to_string()
            } else if msg.contains("faces") || msg.contains("face") {
                "Mesh too large. Maximum faces is 10,000,000.".to_string()
            } else if msg.contains("size") {
                "File too large. Maximum size is 100 MB.".to_string()
            } else {
                "Resource limit exceeded.".to_string()
            }
        }
    }
}

/// Format a quality validation error
///
/// Specialized function for quality setting validation errors.
///
/// # Arguments
///
/// * `quality` - The invalid quality value
///
/// # Returns
///
/// A user-friendly error message
#[allow(dead_code)] // May be used for quality validation in future
pub fn format_quality_error(quality: u8) -> String {
    if quality > 100 {
        "Quality must be between 1 and 100.".to_string()
    } else {
        "Quality must be between 1 and 100.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_unsupported_format() {
        let error = ConversionError::UnsupportedFormat("xyz".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "File type not supported.");
    }

    #[test]
    fn test_format_io_error() {
        let error = ConversionError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));
        let message = format_user_message(&error);
        assert_eq!(message, "Can't read file. Check if file exists.");
    }

    #[test]
    fn test_format_invalid_input_extension() {
        let error = ConversionError::InvalidInput("Unsupported extension: xyz".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "File type not supported.");
    }

    #[test]
    fn test_format_invalid_input_size() {
        let error = ConversionError::InvalidInput("File size exceeds limit".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "File too large. Maximum size is 100 MB.");
    }

    #[test]
    fn test_format_invalid_input_dimension() {
        let error = ConversionError::InvalidInput("Image width exceeds dimension limit".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "Image too large. Maximum dimension is 65535 pixels.");
    }

    #[test]
    fn test_format_invalid_format_mismatch() {
        let error = ConversionError::InvalidFormat(
            "Format mismatch: extension suggests Png but magic bytes indicate Jpeg".to_string(),
        );
        let message = format_user_message(&error);
        assert_eq!(message, "File type doesn't match file content.");
    }

    #[test]
    fn test_format_resource_limit_dimension() {
        let error = ConversionError::ResourceLimitExceeded(
            "Image dimension 100000 exceeds limit".to_string(),
        );
        let message = format_user_message(&error);
        assert_eq!(message, "Image too large. Maximum dimension is 65535 pixels.");
    }

    #[test]
    fn test_format_resource_limit_size() {
        let error = ConversionError::ResourceLimitExceeded(
            "File size exceeds limit".to_string(),
        );
        let message = format_user_message(&error);
        assert_eq!(message, "File too large. Maximum size is 100 MB.");
    }

    #[test]
    fn test_format_quality_error() {
        let message = format_quality_error(150);
        assert_eq!(message, "Quality must be between 1 and 100.");
    }

    #[test]
    fn test_format_mesh_vertices_error() {
        let error = ConversionError::ResourceLimitExceeded(
            "Mesh vertices exceed limit".to_string(),
        );
        let message = format_user_message(&error);
        assert_eq!(message, "Mesh too large. Maximum vertices is 10,000,000.");
    }

    #[test]
    fn test_format_mesh_faces_error() {
        let error = ConversionError::ResourceLimitExceeded(
            "Mesh faces exceed limit".to_string(),
        );
        let message = format_user_message(&error);
        assert_eq!(message, "Mesh too large. Maximum faces is 10,000,000.");
    }

    #[test]
    fn test_format_mesh_validation_error() {
        let error = ConversionError::ValidationFailed(
            "Mesh topology validation failed".to_string(),
        );
        let message = format_user_message(&error);
        assert_eq!(message, "Mesh validation failed. Check if mesh is valid.");
    }

    #[test]
    fn test_format_invalid_input_vertices() {
        let error = ConversionError::InvalidInput("Mesh vertices exceed limit".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "Mesh too large. Maximum vertices is 10,000,000.");
    }

    #[test]
    fn test_format_invalid_input_faces() {
        let error = ConversionError::InvalidInput("Mesh faces exceed limit".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "Mesh too large. Maximum faces is 10,000,000.");
    }
}

