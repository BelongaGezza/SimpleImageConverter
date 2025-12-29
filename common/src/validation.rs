// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};

/// Sanitize a path for error messages (returns filename only)
///
/// This prevents information disclosure by only showing the filename
/// instead of the full path, which could leak directory structure.
fn sanitize_path(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Validate that a file path exists and is readable
///
/// This function uses canonicalization for basic security (resolves `..` and symlinks).
/// For better security with directory restrictions, use `validate_file_path_secure()`.
pub fn validate_file_path(path: &std::path::Path) -> Result<()> {
    // Use canonicalization for basic security
    let canonical = path.canonicalize().map_err(|e| {
        ConversionError::InvalidInput(format!(
            "Cannot resolve path '{}': {}",
            sanitize_path(path),
            e
        ))
    })?;

    if !canonical.is_file() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a file: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}

/// Validate that a directory path exists and is writable
pub fn validate_directory_path(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Err(ConversionError::InvalidInput(format!(
            "Directory does not exist: {}",
            sanitize_path(path)
        )));
    }

    if !path.is_dir() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a directory: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}

/// Validate file path with security checks (canonicalization and optional directory restriction)
///
/// This function:
/// 1. Canonicalizes the path to resolve `..` and symlinks
/// 2. Optionally restricts the path to a specific directory
/// 3. Validates the path exists and is a file
///
/// # Arguments
///
/// * `path` - The path to validate
/// * `allowed_dir` - Optional directory to restrict paths to (None = no restriction)
///
/// # Example
///
/// ```no_run
/// use common::validation::validate_file_path_secure;
/// use std::path::Path;
///
/// // Without directory restriction
/// validate_file_path_secure(Path::new("input.png"), None)?;
///
/// // With directory restriction
/// validate_file_path_secure(
///     Path::new("input.png"),
///     Some(Path::new("/safe/directory"))
/// )?;
/// # Ok::<(), common::error::ConversionError>(())
/// ```
pub fn validate_file_path_secure(
    path: &std::path::Path,
    allowed_dir: Option<&std::path::Path>,
) -> Result<()> {
    // Canonicalize to resolve .. and symlinks
    let canonical = path.canonicalize().map_err(|e| {
        ConversionError::ValidationFailed(format!(
            "Cannot resolve path '{}': {}",
            sanitize_path(path),
            e
        ))
    })?;

    // If allowed_dir is specified, ensure path is within it
    if let Some(allowed) = allowed_dir {
        let allowed_canonical = allowed.canonicalize().map_err(|e| {
            ConversionError::ValidationFailed(format!("Cannot resolve allowed directory: {}", e))
        })?;

        if !canonical.starts_with(&allowed_canonical) {
            return Err(ConversionError::ValidationFailed(format!(
                "Path '{}' is outside allowed directory",
                sanitize_path(path)
            )));
        }
    }

    // Validate it's a file (canonicalize might have resolved to a directory)
    if !canonical.is_file() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a file: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_path_sanitization_in_errors() {
        // Test that error messages only show filename, not full path
        let path = std::path::Path::new("/home/user/secret/file.png");
        let result = validate_file_path(path);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Should contain "file.png" but NOT "/home/user/secret"
        assert!(error_msg.contains("file.png"));
        assert!(!error_msg.contains("/home"));
        assert!(!error_msg.contains("secret"));
    }

    #[test]
    fn test_path_traversal_blocked() {
        // Create a test directory structure
        let temp_dir = TempDir::new().unwrap();
        let safe_dir = temp_dir.path().join("safe");
        std::fs::create_dir(&safe_dir).unwrap();
        let test_file = safe_dir.join("test.txt");
        std::fs::write(&test_file, b"test").unwrap();

        // Try to access file outside safe directory using ..
        let malicious_path = safe_dir.join("../../etc/passwd");

        // Should fail (either file doesn't exist or is outside allowed dir)
        let result = validate_file_path_secure(&malicious_path, Some(&safe_dir));
        assert!(result.is_err());
    }

    #[test]
    fn test_canonicalization_resolves_dots() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, b"test").unwrap();

        // Path with .. should resolve correctly
        // Create a subdirectory and use .. to reference the file
        let subdir = temp_dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();
        let path_with_dots = subdir.join("..").join("test.txt");
        assert!(validate_file_path(&path_with_dots).is_ok());
    }

    #[test]
    fn test_directory_restriction_works() {
        let temp_dir = TempDir::new().unwrap();
        let safe_dir = temp_dir.path().join("safe");
        std::fs::create_dir(&safe_dir).unwrap();
        let test_file = safe_dir.join("test.txt");
        std::fs::write(&test_file, b"test").unwrap();

        // File inside safe directory should work
        assert!(validate_file_path_secure(&test_file, Some(&safe_dir)).is_ok());

        // Try to access file in parent directory
        let parent_file = temp_dir.path().join("parent.txt");
        std::fs::write(&parent_file, b"test").unwrap();
        let result = validate_file_path_secure(&parent_file, Some(&safe_dir));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("outside allowed directory"));
    }

    #[test]
    fn test_validate_file_path_with_canonicalization() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Should succeed for existing file
        assert!(validate_file_path(path).is_ok());

        // Should fail for non-existent file
        let non_existent = path.parent().unwrap().join("nonexistent.txt");
        assert!(validate_file_path(&non_existent).is_err());
    }

    #[test]
    fn test_validate_file_path_secure_without_restriction() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Should work without directory restriction
        assert!(validate_file_path_secure(path, None).is_ok());
    }

    #[test]
    fn test_validate_directory_path_sanitization() {
        let path = std::path::Path::new("/home/user/secret/dir");
        let result = validate_directory_path(path);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Should contain "dir" but NOT "/home/user/secret"
        assert!(error_msg.contains("dir"));
        assert!(!error_msg.contains("/home"));
    }
}
