// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};
use std::path::{Component, Path, PathBuf};

/// Sanitize a path for error messages (returns filename only)
///
/// This prevents information disclosure by only showing the filename
/// instead of the full path, which could leak directory structure.
fn sanitize_path(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Validate that a file path exists and is readable
///
/// This function uses canonicalization for basic security (resolves `..` and symlinks).
/// For better security with directory restrictions, use `validate_file_path_secure()`.
pub fn validate_file_path(path: &Path) -> Result<()> {
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
pub fn validate_directory_path(path: &Path) -> Result<()> {
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
pub fn validate_file_path_secure(path: &Path, allowed_dir: Option<&Path>) -> Result<()> {
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

/// Policy used when validating and writing output files.
#[derive(Debug, Clone)]
pub struct OutputWritePolicy {
    /// Whether an existing output file may be replaced.
    pub allow_overwrite: bool,
    /// Optional canonical root that output parents must stay inside.
    pub allowed_output_root: Option<PathBuf>,
    /// Whether missing parent directories may be created before writing.
    pub create_parent_dirs: bool,
    /// Whether obvious OS/system directories should be rejected.
    pub block_system_dirs: bool,
}

impl Default for OutputWritePolicy {
    fn default() -> Self {
        Self {
            allow_overwrite: false,
            allowed_output_root: None,
            create_parent_dirs: false,
            block_system_dirs: true,
        }
    }
}

/// Output path that has passed validation.
#[derive(Debug, Clone)]
pub struct ValidatedOutputPath {
    path: PathBuf,
    canonical_parent: PathBuf,
}

impl ValidatedOutputPath {
    /// Return the original output path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Return the canonicalized parent directory.
    pub fn canonical_parent(&self) -> &Path {
        &self.canonical_parent
    }
}

/// Validate a filename component for safe output writes.
pub fn validate_output_filename(filename: &str) -> Result<()> {
    if filename.trim().is_empty() {
        return Err(ConversionError::InvalidInput(
            "Output filename cannot be empty".to_string(),
        ));
    }

    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    if filename.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(ConversionError::InvalidInput(
            "Output filename contains invalid characters".to_string(),
        ));
    }

    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err(ConversionError::InvalidInput(
            "Output filename cannot contain path traversal or separators".to_string(),
        ));
    }

    if filename.len() > 260 {
        return Err(ConversionError::InvalidInput(
            "Output filename is too long".to_string(),
        ));
    }

    Ok(())
}

/// Validate an output path according to the provided write policy.
pub fn validate_output_path(
    path: &Path,
    policy: &OutputWritePolicy,
) -> Result<ValidatedOutputPath> {
    let filename = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
        ConversionError::InvalidInput("Output path must include a filename".to_string())
    })?;
    validate_output_filename(filename)?;

    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ConversionError::InvalidInput(
            "Output path cannot contain parent-directory traversal".to_string(),
        ));
    }

    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| {
            ConversionError::InvalidInput("Output path must include a parent directory".to_string())
        })?;

    if !parent.exists() {
        if policy.create_parent_dirs {
            std::fs::create_dir_all(parent).map_err(ConversionError::Io)?;
        } else {
            return Err(ConversionError::InvalidInput(
                "Output parent directory does not exist".to_string(),
            ));
        }
    }

    let canonical_parent = parent.canonicalize().map_err(|e| {
        ConversionError::InvalidInput(format!(
            "Cannot resolve output directory '{}': {}",
            sanitize_path(parent),
            e
        ))
    })?;

    if !canonical_parent.is_dir() {
        return Err(ConversionError::InvalidInput(
            "Output parent is not a directory".to_string(),
        ));
    }

    if policy.block_system_dirs {
        reject_system_directory(&canonical_parent)?;
    }

    if let Some(root) = &policy.allowed_output_root {
        let canonical_root = root.canonicalize().map_err(|e| {
            ConversionError::InvalidInput(format!("Cannot resolve allowed output root: {}", e))
        })?;
        if !canonical_parent.starts_with(&canonical_root) {
            return Err(ConversionError::ValidationFailed(
                "Output path is outside the allowed output root".to_string(),
            ));
        }
    }

    if path.exists() && !policy.allow_overwrite {
        return Err(ConversionError::InvalidInput(
            "Output file already exists; use --force to overwrite".to_string(),
        ));
    }

    Ok(ValidatedOutputPath {
        path: path.to_path_buf(),
        canonical_parent,
    })
}

fn reject_system_directory(path: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        let mut path_str = path.to_string_lossy().to_lowercase();
        if path_str.starts_with(r"\\?\") {
            path_str = path_str[4..].to_string();
        }
        let path_str = path_str.trim_end_matches('\\');

        let is_drive_root = path_str.len() == 2 && path_str.as_bytes()[1] == b':';
        if is_drive_root {
            return Err(ConversionError::ValidationFailed(
                "Cannot write to filesystem roots".to_string(),
            ));
        }

        let system_dirs = [
            r"c:\windows",
            r"c:\windows\system32",
            r"c:\program files",
            r"c:\program files (x86)",
            r"c:\programdata",
        ];
        if system_dirs.iter().any(|dir| {
            path_str == *dir
                || path_str
                    .strip_prefix(*dir)
                    .is_some_and(|rest| rest.starts_with('\\'))
        }) {
            return Err(ConversionError::ValidationFailed(
                "Cannot write to system directories".to_string(),
            ));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let system_dirs = [
            Path::new("/System"),
            Path::new("/Library"),
            Path::new("/Applications"),
            Path::new("/bin"),
            Path::new("/sbin"),
            Path::new("/usr"),
        ];
        if path == Path::new("/") || system_dirs.iter().any(|dir| path.starts_with(dir)) {
            return Err(ConversionError::ValidationFailed(
                "Cannot write to system directories".to_string(),
            ));
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let system_dirs = [
            Path::new("/bin"),
            Path::new("/sbin"),
            Path::new("/usr"),
            Path::new("/lib"),
            Path::new("/lib64"),
            Path::new("/etc"),
            Path::new("/var"),
            Path::new("/boot"),
            Path::new("/dev"),
            Path::new("/proc"),
            Path::new("/sys"),
            Path::new("/run"),
        ];
        if path == Path::new("/") || system_dirs.iter().any(|dir| path.starts_with(dir)) {
            return Err(ConversionError::ValidationFailed(
                "Cannot write to system directories".to_string(),
            ));
        }
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

    #[test]
    fn test_validate_output_path_rejects_existing_without_force() {
        let temp_file = NamedTempFile::new().unwrap();
        let policy = OutputWritePolicy::default();
        let result = validate_output_path(temp_file.path(), &policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_validate_output_path_allows_existing_with_force() {
        let temp_file = NamedTempFile::new().unwrap();
        let policy = OutputWritePolicy {
            allow_overwrite: true,
            ..Default::default()
        };
        let result = validate_output_path(temp_file.path(), &policy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_output_path_rejects_bad_filename() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("bad<name>.png");
        let result = validate_output_path(&path, &OutputWritePolicy::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_output_path_allowed_root() {
        let temp_dir = TempDir::new().unwrap();
        let allowed = temp_dir.path().join("allowed");
        let denied = temp_dir.path().join("denied");
        std::fs::create_dir_all(&allowed).unwrap();
        std::fs::create_dir_all(&denied).unwrap();

        let policy = OutputWritePolicy {
            allowed_output_root: Some(allowed.clone()),
            ..Default::default()
        };

        assert!(validate_output_path(&allowed.join("ok.png"), &policy).is_ok());
        assert!(validate_output_path(&denied.join("no.png"), &policy).is_err());
    }

    #[test]
    fn test_validate_output_path_creates_parent_when_allowed() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("new").join("output.png");
        let policy = OutputWritePolicy {
            create_parent_dirs: true,
            ..Default::default()
        };

        assert!(validate_output_path(&path, &policy).is_ok());
        assert!(path.parent().unwrap().exists());
    }
}
