// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};
use crate::limits::ResourceLimits;
use std::fs;
use std::path::Path;

/// Read file contents into a byte vector
///
/// Note: For untrusted input, prefer `read_file_bytes_checked` which validates
/// file size before reading.
pub fn read_file_bytes(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(ConversionError::Io)
}

/// Read file contents with size validation
///
/// Validates file size against the provided limits before reading to prevent
/// memory exhaustion attacks from maliciously large files.
///
/// # Example
///
/// ```no_run
/// use common::io::read_file_bytes_checked;
/// use common::limits::ResourceLimits;
/// use std::path::Path;
///
/// let limits = ResourceLimits::default();
/// let data = read_file_bytes_checked(Path::new("input.png"), &limits)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn read_file_bytes_checked(path: &Path, limits: &ResourceLimits) -> Result<Vec<u8>> {
    // Get file metadata to check size before reading
    let metadata = fs::metadata(path).map_err(|e| {
        ConversionError::InvalidInput(format!(
            "Cannot read file metadata for '{}': {}",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown"),
            e
        ))
    })?;

    let size = metadata.len() as usize;
    limits.check_file_size(size)?;

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

/// Get file size in bytes
pub fn get_file_size(path: &Path) -> Result<usize> {
    let metadata = fs::metadata(path).map_err(ConversionError::Io)?;
    Ok(metadata.len() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_extension() {
        assert_eq!(
            get_extension(Path::new("test.png")),
            Some("png".to_string())
        );
        assert_eq!(
            get_extension(Path::new("test.PNG")),
            Some("png".to_string())
        );
        assert_eq!(
            get_extension(Path::new("test.JpEg")),
            Some("jpeg".to_string())
        );
        assert_eq!(get_extension(Path::new("test")), None);
    }

    #[test]
    fn test_read_file_bytes_checked_ok() {
        // Create a small temp file
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"small content").unwrap();

        let limits = ResourceLimits::default();
        let result = read_file_bytes_checked(file.path(), &limits);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"small content");
    }

    #[test]
    fn test_read_file_bytes_checked_size_limit() {
        // Create a temp file
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"some content").unwrap();

        // Use very restrictive limits
        let limits = ResourceLimits::builder()
            .max_file_size(5) // 5 bytes max
            .build();

        let result = read_file_bytes_checked(file.path(), &limits);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds limit"));
    }
}
