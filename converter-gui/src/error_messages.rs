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
/// and sensitive information (like full paths) are removed. Messages include
/// suggestions for resolving common issues where applicable.
///
/// # Arguments
///
/// * `error` - The conversion error to format
///
/// # Returns
///
/// A user-friendly error message string with actionable guidance
///
/// # Example
///
/// ```
/// use converter_gui::error_messages::format_user_message;
/// use common::error::ConversionError;
///
/// let error = ConversionError::UnsupportedFormat("xyz".to_string());
/// let message = format_user_message(&error);
/// assert!(message.contains("File type not supported"));
/// ```
pub fn format_user_message(error: &ConversionError) -> String {
    match error {
        ConversionError::InvalidInput(msg) => {
            // Check for specific error patterns in the message
            if msg.contains("extension") || msg.contains("format") || msg.contains("Unsupported") {
                "File type not supported. Please use a supported image or mesh format.".to_string()
            } else if msg.contains("Quality") || msg.contains("quality") {
                "Quality setting must be between 1 and 100. Please adjust the quality slider.".to_string()
            } else if msg.contains("size")
                || msg.contains("too large")
                || msg.contains("exceeds limit")
            {
                if msg.contains("dimension") || msg.contains("width") || msg.contains("height") {
                    "Image dimensions too large. Maximum dimension is 65,535 pixels. Please use a smaller image or resize it before converting.".to_string()
                } else if msg.contains("vertices") || msg.contains("vertex") {
                    "Mesh has too many vertices. Maximum is 10,000,000. Please use a mesh with fewer vertices or simplify the model.".to_string()
                } else if msg.contains("faces") || msg.contains("face") {
                    "Mesh has too many faces. Maximum is 10,000,000. Please use a mesh with fewer faces or simplify the model.".to_string()
                } else {
                    "File size too large. Maximum size is 100 MB. Please use a smaller file or compress it first.".to_string()
                }
            } else if msg.contains("dimension") || msg.contains("width") || msg.contains("height") {
                "Image dimensions too large. Maximum dimension is 65,535 pixels. Please use a smaller image.".to_string()
            } else if msg.contains("vertices") || msg.contains("vertex") {
                "Mesh has too many vertices. Maximum is 10,000,000. Please simplify the model.".to_string()
            } else if msg.contains("faces") || msg.contains("face") {
                "Mesh has too many faces. Maximum is 10,000,000. Please simplify the model.".to_string()
            } else if msg.contains("filename") || msg.contains("Output filename") {
                "Invalid output filename. Please use a valid filename without special characters.".to_string()
            } else {
                "Invalid file. Please check that the file exists, is readable, and is not corrupted.".to_string()
            }
        }
        ConversionError::UnsupportedFormat(_) => {
            "File type not supported. Please select a supported image (PNG, JPEG, BMP, GIF, WebP) or mesh (STL, OBJ, PLY, OFF, DXF, glTF) format.".to_string()
        }
        ConversionError::InvalidFormat(msg) => {
            // Format mismatch (extension vs magic bytes)
            if msg.contains("mismatch") {
                "File extension doesn't match file content. The file may be corrupted or have the wrong extension. Please verify the file is valid.".to_string()
            } else {
                "Invalid file format. The file may be corrupted or in an unsupported format. Please check the file and try again.".to_string()
            }
        }
        ConversionError::Io(err) => {
            // Provide more specific I/O error messages
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    "File not found. Please check that the file exists and the path is correct.".to_string()
                }
                std::io::ErrorKind::PermissionDenied => {
                    "Permission denied. Please check file permissions or try running as administrator.".to_string()
                }
                std::io::ErrorKind::AlreadyExists => {
                    "File already exists at the output location. Please choose a different filename or location.".to_string()
                }
                std::io::ErrorKind::OutOfMemory => {
                    "Out of memory. The file is too large to process. Please use a smaller file or close other applications.".to_string()
                }
                _ => {
                    format!("Cannot read file: {}. Please check that the file exists and is accessible.", 
                        err.kind().to_string().replace("ErrorKind::", "").to_lowercase())
                }
            }
        }
        ConversionError::ConversionFailed(msg) => {
            if msg.contains("corrupted") || msg.contains("invalid") {
                "File appears to be corrupted or invalid. Please verify the file is valid and try again.".to_string()
            } else if msg.contains("transform") || msg.contains("coordinate") {
                "Coordinate system transformation failed. Please try converting without coordinate transformation.".to_string()
            } else if msg.contains("normal") {
                "Failed to calculate mesh normals. Try enabling 'Recalculate Normals' option or check if the mesh is valid.".to_string()
            } else if msg.contains("write") || msg.contains("output") {
                "Failed to write output file. Please check that you have write permissions and sufficient disk space.".to_string()
            } else {
                "Conversion failed. Please check that the file is valid and try again. If the problem persists, the file format may not be fully supported.".to_string()
            }
        }
        ConversionError::ValidationFailed(msg) => {
            if msg.contains("path") || msg.contains("Path") {
                "Invalid file path. Please check that the path is valid and doesn't contain invalid characters.".to_string()
            } else if msg.contains("mesh") || msg.contains("topology") || msg.contains("geometry") {
                "Mesh validation failed. The mesh may have invalid geometry. Try enabling mesh validation options or check if the mesh file is valid.".to_string()
            } else if msg.contains("system") || msg.contains("System") {
                "Cannot write to system directories. Please choose a different output location.".to_string()
            } else {
                "File validation failed. Please check that the file is valid and try again.".to_string()
            }
        }
        ConversionError::ResourceLimitExceeded(msg) => {
            if msg.contains("dimension") || msg.contains("width") || msg.contains("height") {
                "Image dimensions exceed limit (65,535 pixels). Please use a smaller image or resize it before converting.".to_string()
            } else if msg.contains("vertices") || msg.contains("vertex") {
                "Mesh vertex count exceeds limit (10,000,000). Please use a simpler mesh or reduce the vertex count.".to_string()
            } else if msg.contains("faces") || msg.contains("face") {
                "Mesh face count exceeds limit (10,000,000). Please use a simpler mesh or reduce the face count.".to_string()
            } else if msg.contains("size") {
                "File size exceeds limit (100 MB). Please use a smaller file or compress it before converting.".to_string()
            } else {
                "Resource limit exceeded. The file is too large or complex to process. Please use a smaller or simpler file.".to_string()
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
pub fn format_quality_error(_quality: u8) -> String {
    "Quality must be between 1 and 100.".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_unsupported_format() {
        let error = ConversionError::UnsupportedFormat("xyz".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("File type not supported"));
        assert!(message.contains("supported"));
    }

    #[test]
    fn test_format_io_error() {
        let error = ConversionError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));
        let message = format_user_message(&error);
        assert!(message.contains("File not found"));
        assert!(message.contains("exists"));
    }

    #[test]
    fn test_format_invalid_input_extension() {
        let error = ConversionError::InvalidInput("Unsupported extension: xyz".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("File type not supported"));
    }

    #[test]
    fn test_format_invalid_input_size() {
        let error = ConversionError::InvalidInput("File size exceeds limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("File size too large"));
        assert!(message.contains("100 MB"));
    }

    #[test]
    fn test_format_invalid_input_dimension() {
        let error =
            ConversionError::InvalidInput("Image width exceeds dimension limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("Image dimensions too large"));
        assert!(message.contains("65,535 pixels"));
    }

    #[test]
    fn test_format_invalid_format_mismatch() {
        let error = ConversionError::InvalidFormat(
            "Format mismatch: extension suggests Png but magic bytes indicate Jpeg".to_string(),
        );
        let message = format_user_message(&error);
        assert!(message.contains("extension doesn't match"));
        assert!(message.contains("file content"));
    }

    #[test]
    fn test_format_resource_limit_dimension() {
        let error = ConversionError::ResourceLimitExceeded(
            "Image dimension 100000 exceeds limit".to_string(),
        );
        let message = format_user_message(&error);
        assert!(message.contains("Image dimensions exceed limit"));
        assert!(message.contains("65,535 pixels"));
    }

    #[test]
    fn test_format_resource_limit_size() {
        let error = ConversionError::ResourceLimitExceeded("File size exceeds limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("File size exceeds limit"));
        assert!(message.contains("100 MB"));
    }

    #[test]
    fn test_format_quality_error() {
        let message = format_quality_error(150);
        assert!(message.contains("Quality"));
        assert!(message.contains("1 and 100"));
    }

    #[test]
    fn test_format_mesh_vertices_error() {
        let error =
            ConversionError::ResourceLimitExceeded("Mesh vertices exceed limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("vertex count exceeds limit"));
        assert!(message.contains("10,000,000"));
    }

    #[test]
    fn test_format_mesh_faces_error() {
        let error = ConversionError::ResourceLimitExceeded("Mesh faces exceed limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("face count exceeds limit"));
        assert!(message.contains("10,000,000"));
    }

    #[test]
    fn test_format_mesh_validation_error() {
        let error =
            ConversionError::ValidationFailed("Mesh topology validation failed".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("Mesh validation failed"));
        assert!(message.contains("mesh"));
    }

    #[test]
    fn test_format_invalid_input_vertices() {
        let error = ConversionError::InvalidInput("Mesh vertices exceed limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("vertices"));
        assert!(message.contains("10,000,000"));
    }

    #[test]
    fn test_format_invalid_input_faces() {
        let error = ConversionError::InvalidInput("Mesh faces exceed limit".to_string());
        let message = format_user_message(&error);
        assert!(message.contains("faces"));
        assert!(message.contains("10,000,000"));
    }

    #[test]
    fn test_format_io_error_permission_denied() {
        let error = ConversionError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Permission denied",
        ));
        let message = format_user_message(&error);
        assert!(message.contains("Permission denied"));
    }

    #[test]
    fn test_format_io_error_already_exists() {
        let error = ConversionError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "File exists",
        ));
        let message = format_user_message(&error);
        assert!(message.contains("already exists"));
    }
}
