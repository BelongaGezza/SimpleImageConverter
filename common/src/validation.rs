// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};

/// Validate that a file path exists and is readable
pub fn validate_file_path(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Err(ConversionError::InvalidInput(format!(
            "File does not exist: {}",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a file: {}",
            path.display()
        )));
    }

    Ok(())
}

/// Validate that a directory path exists and is writable
pub fn validate_directory_path(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Err(ConversionError::InvalidInput(format!(
            "Directory does not exist: {}",
            path.display()
        )));
    }

    if !path.is_dir() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a directory: {}",
            path.display()
        )));
    }

    Ok(())
}
