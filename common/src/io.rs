// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};
use std::fs;
use std::path::Path;

/// Read file contents into a byte vector
pub fn read_file_bytes(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(ConversionError::Io)
}

/// Write byte vector to file
pub fn write_file_bytes(path: &Path, data: &[u8]) -> Result<()> {
    fs::write(path, data).map_err(ConversionError::Io)
}

/// Get file extension (lowercase, without dot)
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}
