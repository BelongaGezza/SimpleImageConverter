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
}

/// Result type alias for conversion operations
pub type Result<T> = std::result::Result<T, ConversionError>;
