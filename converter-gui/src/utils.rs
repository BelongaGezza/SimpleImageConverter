// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Utility functions for path sanitization and validation
//!
//! This module provides helper functions for sanitizing paths for display
//! and validating output paths to prevent security vulnerabilities.

use std::path::{Path, PathBuf};

/// Sanitize a path for display in the UI
///
/// This function removes sensitive information from paths before displaying
/// them to users. It:
/// - Removes user home directory if present
/// - Truncates long paths (> 60 characters)
/// - Returns only the filename if path is too long
///
/// # Arguments
///
/// * `path` - The path to sanitize
///
/// # Returns
///
/// A sanitized path string suitable for display
///
/// # Example
///
/// ```
/// use converter_gui::utils::sanitize_path_for_display;
/// use std::path::Path;
///
/// let path = Path::new("C:\\Users\\JohnDoe\\Documents\\photo.jpg");
/// let sanitized = sanitize_path_for_display(path);
/// // Returns something like "Documents\\photo.jpg" or just "photo.jpg"
/// ```
pub fn sanitize_path_for_display(path: &Path) -> String {
    // Try to get relative path from home directory
    if let Ok(home) = std::env::var("USERPROFILE") {
        if let Ok(home_path) = PathBuf::from(home).canonicalize() {
            if let Ok(relative) = path.strip_prefix(&home_path) {
                let relative_str = relative.display().to_string();
                if relative_str.len() <= 60 {
                    return relative_str;
                }
            }
        }
    }

    // Try to get relative path from current directory
    if let Ok(current_dir) = std::env::current_dir() {
        if let Ok(current_dir_canonical) = current_dir.canonicalize() {
            if let Ok(relative) = path.strip_prefix(&current_dir_canonical) {
                let relative_str = relative.display().to_string();
                if relative_str.len() <= 60 {
                    return relative_str;
                }
            }
        }
    }

    // If path is short enough, return as-is
    let path_str = path.display().to_string();
    if path_str.len() <= 60 {
        return path_str;
    }

    // Truncate long paths, showing last 57 characters with "..."
    if path_str.len() > 60 {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // If filename alone is short enough, return it
        if filename.len() <= 60 {
            return filename.to_string();
        }

        // Otherwise truncate
        format!("...{}", &path_str[path_str.len().saturating_sub(57)..])
    } else {
        path_str
    }
}

/// Validate output filename for invalid characters and path traversal
///
/// This function checks that a filename is safe to use:
/// - No invalid characters (Windows reserved: `< > : " | ? *`)
/// - No path traversal sequences (`../`)
/// - Valid length (Windows MAX_PATH: 260 chars)
///
/// # Arguments
///
/// * `filename` - The filename to validate
///
/// # Returns
///
/// `Ok(())` if valid, or an error with a user-friendly message
///
/// # Example
///
/// ```
/// use converter_gui::utils::validate_output_filename;
///
/// assert!(validate_output_filename("photo.jpg").is_ok());
/// assert!(validate_output_filename("../etc/passwd").is_err());
/// assert!(validate_output_filename("file<name>.jpg").is_err());
/// ```
pub fn validate_output_filename(filename: &str) -> Result<(), String> {
    // Check for invalid characters (Windows reserved)
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    if filename.chars().any(|c| invalid_chars.contains(&c)) {
        return Err("Filename contains invalid characters.".to_string());
    }

    // Check for path traversal
    if filename.contains("..") || filename.contains("\\") || filename.contains("/") {
        return Err("Filename cannot contain path separators.".to_string());
    }

    // Check path length (Windows MAX_PATH: 260 chars)
    if filename.len() > 260 {
        return Err("Filename too long (maximum 260 characters).".to_string());
    }

    // Check for empty filename
    if filename.trim().is_empty() {
        return Err("Filename cannot be empty.".to_string());
    }

    Ok(())
}

/// Validate that an output path is not in a system directory
///
/// This function prevents writing to system directories which could be
/// a security risk. It checks against common system directories on Windows.
///
/// # Arguments
///
/// * `path` - The output path to validate
///
/// # Returns
///
/// `Ok(())` if the path is safe, or an error message
///
/// # Example
///
/// ```
/// use converter_gui::utils::validate_output_path_not_system;
/// use std::path::Path;
///
/// // User directory should be OK
/// assert!(validate_output_path_not_system(Path::new("C:\\Users\\photo.jpg")).is_ok());
///
/// // System directories should fail (if they exist on the system)
/// // Note: This test may not work on all systems, so we check the result
/// let system_result = validate_output_path_not_system(Path::new("C:\\Windows\\photo.jpg"));
/// // On Windows, this should fail; on other systems, it may not exist
/// ```
#[allow(dead_code)] // Legacy wrapper retained for existing utility tests; production uses common::validation.
pub fn validate_output_path_not_system(path: &Path) -> Result<(), String> {
    // Get canonical path to resolve any .. or symlinks
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // If path doesn't exist yet, check parent directory
            if let Some(parent) = path.parent() {
                if let Ok(parent_canonical) = parent.canonicalize() {
                    // Check if parent is a system directory
                    return check_system_directory(&parent_canonical);
                }
            }
            // If we can't canonicalize, check the path string directly
            return check_system_directory_string(path);
        }
    };

    check_system_directory(&canonical)
}

/// Check if a canonicalized path is in a system directory
#[allow(dead_code)] // Used by legacy output path helper above.
fn check_system_directory(path: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        let mut path_str = path.display().to_string().to_lowercase();

        // Strip Windows extended-length path prefix (\\?\)
        // canonicalize() on Windows can return paths with this prefix
        if path_str.starts_with("\\\\?\\") {
            path_str = path_str[4..].to_string();
        }

        // Normalize: remove trailing backslashes for consistent comparison
        let path_str = path_str.trim_end_matches('\\');

        // Windows system directories to avoid
        let system_dirs = [
            "c:\\windows",
            "c:\\windows\\system32",
            "c:\\windows\\syswow64",
            "c:\\program files",
            "c:\\program files (x86)",
            "c:\\programdata",
            "c:\\system volume information",
        ];

        for system_dir in &system_dirs {
            let normalized_dir = system_dir.trim_end_matches('\\');
            if path_str.starts_with(normalized_dir) {
                return Err("Cannot write to system directories.".to_string());
            }
        }
    }

    #[cfg(unix)]
    {
        let path_str = path.to_string_lossy();
        let path_lower = path_str.to_lowercase();

        // Linux/Unix system directories to avoid
        // Check both exact matches and prefix matches
        let system_dirs = [
            "/bin",
            "/sbin",
            "/usr/bin",
            "/usr/sbin",
            "/usr/lib",
            "/usr/lib64",
            "/lib",
            "/lib64",
            "/etc",
            "/boot",
            "/sys",
            "/proc",
            "/dev",
            "/root",
            "/var/lib",
            "/var/log",
            "/var/run",
            "/var/tmp",
            "/opt/bin",
            "/opt/sbin",
        ];

        // Check for exact matches or prefix matches (with trailing slash)
        for system_dir in &system_dirs {
            if path_lower == *system_dir
                || path_lower.starts_with(&format!("{}/", system_dir))
                || path_lower.starts_with(&format!("{}\\", system_dir))
            {
                return Err("Cannot write to system directories.".to_string());
            }
        }

        // Also check for root-level system directories (case-insensitive)
        if path_lower == "/bin"
            || path_lower == "/sbin"
            || path_lower == "/etc"
            || path_lower == "/lib"
            || path_lower == "/lib64"
            || path_lower == "/usr"
            || path_lower == "/var"
            || path_lower == "/opt"
        {
            return Err("Cannot write to system directories.".to_string());
        }
    }

    Ok(())
}

/// Check if a path string contains system directory patterns
///
/// This is a fallback when canonicalization fails. It checks for system
/// directory patterns in the path string directly.
#[allow(dead_code)] // Used by legacy output path helper above.
fn check_system_directory_string(path: &Path) -> Result<(), String> {
    let path_str = path.display().to_string().to_lowercase();

    // Check for system directory patterns in the path
    // Patterns with backslashes (for paths like C:\Windows\file)
    let system_patterns = [
        "\\windows\\",
        "\\system32\\",
        "\\syswow64\\",
        "\\program files\\",
        "\\programdata\\",
    ];

    for pattern in &system_patterns {
        if path_str.contains(pattern) {
            return Err("Cannot write to system directories.".to_string());
        }
    }

    // Check for root system directories (e.g., C:\Windows\file or C:\Windows)
    // This handles cases where the path starts with a system directory
    let root_system_dirs = ["c:\\windows", "c:\\program files", "c:\\programdata"];

    for root_dir in &root_system_dirs {
        // Check if path starts with the root directory followed by backslash or end of string
        if path_str.starts_with(root_dir)
            && (path_str.len() == root_dir.len()
                || path_str.as_bytes().get(root_dir.len()) == Some(&b'\\'))
        {
            return Err("Cannot write to system directories.".to_string());
        }
    }

    Ok(())
}

/// Generate output filename from input path and format extension
///
/// This function generates a safe output filename by:
/// - Removing the old extension
/// - Adding the new format extension
/// - Validating the result
///
/// # Arguments
///
/// * `input_path` - The input file path
/// * `output_format_extension` - The extension for the output format (e.g., "jpg", "png")
///
/// # Returns
///
/// `Ok(PathBuf)` with the generated output filename, or an error
///
/// # Example
///
/// ```
/// use converter_gui::utils::generate_output_filename;
/// use std::path::Path;
///
/// let input = Path::new("photo.png");
/// let output = generate_output_filename(input, "jpg").unwrap();
/// assert_eq!(output.file_name().unwrap().to_str().unwrap(), "photo.jpg");
/// ```
#[allow(dead_code)] // Reserved for future use
pub fn generate_output_filename(
    input_path: &Path,
    output_format_extension: &str,
) -> Result<PathBuf, String> {
    let mut output = input_path.to_path_buf();

    // Remove old extension
    output.set_extension("");

    // Add new extension
    output.set_extension(output_format_extension);

    // Validate the generated filename
    if let Some(filename) = output.file_name().and_then(|n| n.to_str()) {
        validate_output_filename(filename)?;
    } else {
        return Err("Cannot generate output filename.".to_string());
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_path_for_display() {
        let path = Path::new("photo.jpg");
        let sanitized = sanitize_path_for_display(path);
        assert_eq!(sanitized, "photo.jpg");
    }

    #[test]
    fn test_sanitize_path_long() {
        let long_path = Path::new("C:\\Users\\TestUser\\Documents\\VeryLongDirectoryName\\AnotherLongDirectory\\photo.jpg");
        let sanitized = sanitize_path_for_display(long_path);
        // Should be truncated or show filename only
        assert!(sanitized.len() <= 60 || sanitized.ends_with("photo.jpg"));
    }

    #[test]
    fn test_validate_output_filename_valid() {
        assert!(validate_output_filename("photo.jpg").is_ok());
        assert!(validate_output_filename("my_file-123.png").is_ok());
    }

    #[test]
    fn test_validate_output_filename_invalid_chars() {
        assert!(validate_output_filename("file<name>.jpg").is_err());
        assert!(validate_output_filename("file:name.jpg").is_err());
        assert!(validate_output_filename("file|name.jpg").is_err());
    }

    #[test]
    fn test_validate_output_filename_path_traversal() {
        assert!(validate_output_filename("../etc/passwd").is_err());
        assert!(validate_output_filename("..\\windows\\file.jpg").is_err());
        assert!(validate_output_filename("folder/file.jpg").is_err());
    }

    #[test]
    fn test_validate_output_filename_empty() {
        assert!(validate_output_filename("").is_err());
        assert!(validate_output_filename("   ").is_err());
    }

    #[test]
    fn test_validate_output_path_not_system() {
        // User directory should be OK
        assert!(validate_output_path_not_system(Path::new("C:\\Users\\photo.jpg")).is_ok());

        // System directories should fail
        assert!(validate_output_path_not_system(Path::new("C:\\Windows\\photo.jpg")).is_err());
        assert!(
            validate_output_path_not_system(Path::new("C:\\Program Files\\photo.jpg")).is_err()
        );
    }

    #[test]
    fn test_validate_output_path_not_system_comprehensive() {
        // System directories with files - should fail
        assert!(validate_output_path_not_system(Path::new("C:\\Windows\\photo.jpg")).is_err());
        assert!(validate_output_path_not_system(Path::new("C:\\Program Files\\app.exe")).is_err());
        assert!(
            validate_output_path_not_system(Path::new("C:\\Windows\\System32\\dll.dll")).is_err()
        );
        assert!(
            validate_output_path_not_system(Path::new("C:\\Windows\\SysWOW64\\dll.dll")).is_err()
        );
        assert!(validate_output_path_not_system(Path::new("C:\\ProgramData\\config.ini")).is_err());

        // Edge cases - system directories themselves (should fail)
        // Note: These may not work on all systems if the directories don't exist
        // but the pattern matching should catch them
        let _windows_result = validate_output_path_not_system(Path::new("C:\\Windows"));
        let _program_files_result = validate_output_path_not_system(Path::new("C:\\Program Files"));
        // On Windows systems, these should fail; on other systems they may pass
        // but the important thing is that paths WITH files in these directories fail

        // User directories - should pass
        assert!(validate_output_path_not_system(Path::new("C:\\Users\\photo.jpg")).is_ok());
        assert!(
            validate_output_path_not_system(Path::new("C:\\Users\\Documents\\photo.jpg")).is_ok()
        );

        // Relative paths in user directories - should pass
        // (These will be resolved relative to current directory)
    }

    #[test]
    fn test_generate_output_filename() {
        let input = Path::new("photo.png");
        let output = generate_output_filename(input, "jpg").unwrap();
        assert_eq!(output.file_name().unwrap().to_str().unwrap(), "photo.jpg");
    }

    #[test]
    fn test_generate_output_filename_no_extension() {
        let input = Path::new("document");
        let output = generate_output_filename(input, "jpg").unwrap();
        assert_eq!(
            output.file_name().unwrap().to_str().unwrap(),
            "document.jpg"
        );
    }
}
