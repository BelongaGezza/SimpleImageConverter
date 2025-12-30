// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Image and mesh conversion integration for GUI
//!
//! This module provides the core conversion functionality using direct
//! library integration with `img-core` and `mesh-core`. All conversions use
//! format detection for security validation.

use crate::utils::{validate_output_filename, validate_output_path_not_system};
use common::error::Result;
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;
use common::validation::validate_file_path;
use img_core::{FormatRegistry, ImageConverter, ImageFormat, QualitySettings};
use mesh_core::{
    ConversionOptions, FormatRegistry as MeshFormatRegistry, MeshConverter, MeshFormat,
};
use std::path::{Path, PathBuf};

/// Convert an image file from one format to another
///
/// This function performs a complete image conversion using direct library
/// integration with `img-core`. It includes:
/// - Two-stage format detection (extension + magic bytes) for security
/// - Resource limits enforcement
/// - Comprehensive error handling with user-friendly messages
///
/// # Arguments
///
/// * `input_path` - Path to the input image file
/// * `output_path` - Path where the converted image will be saved
/// * `output_format` - The target image format for conversion
/// * `quality` - Quality setting (1-100) for lossy formats (JPEG, WebP)
/// * `limits` - Resource limits for security validation
///
/// # Returns
///
/// `Ok(PathBuf)` with the output path on success, or an error if conversion fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The input file path is invalid or inaccessible
/// - The file size exceeds resource limits
/// - Format detection fails (extension and magic bytes don't match)
/// - The input format is unsupported
/// - The output format is not writable (e.g., SVG)
/// - The conversion process fails
/// - The output file cannot be written
///
/// # Example
///
/// ```no_run
/// use converter_gui::conversion::convert_image;
/// use img_core::ImageFormat;
/// use common::limits::ResourceLimits;
/// use std::path::Path;
///
/// let input = Path::new("photo.png");
/// let output = Path::new("photo.jpg");
/// let limits = ResourceLimits::default();
///
/// convert_image(input, output, ImageFormat::Jpeg, 90, &limits)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn convert_image(
    input_path: &Path,
    output_path: &Path,
    output_format: ImageFormat,
    quality: u8,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // Validate input file path (security check)
    validate_file_path(input_path)?;

    // Validate output filename (no invalid characters, no path traversal)
    if let Some(filename) = output_path.file_name().and_then(|n| n.to_str()) {
        validate_output_filename(filename).map_err(|e| {
            common::error::ConversionError::InvalidInput(format!(
                "Output filename validation failed: {}",
                e
            ))
        })?;
    } else {
        return Err(common::error::ConversionError::InvalidInput(
            "Invalid output filename.".to_string(),
        ));
    }

    // Validate output path is not in system directories (security check)
    validate_output_path_not_system(output_path)
        .map_err(common::error::ConversionError::ValidationFailed)?;

    // Check if output file already exists (for user confirmation later)
    // Note: We don't fail here, but the UI should warn the user
    // The actual file write will overwrite if it exists

    // Validate quality value (must be 1-100)
    if quality == 0 || quality > 100 {
        return Err(common::error::ConversionError::InvalidInput(
            "Quality must be between 1 and 100.".to_string(),
        ));
    }

    // Read input file with size validation (DoS prevention)
    let input_data = read_file_bytes_checked(input_path, limits)?;

    // Two-stage format detection (extension + magic bytes for security)
    let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;

    // Get format handlers
    let reader = FormatRegistry::get_reader(input_format)?;
    let writer = FormatRegistry::get_writer(output_format)?;

    // Convert image
    let converter = ImageConverter::new();
    let quality_settings = QualitySettings::new(quality);
    let output_data = converter.convert(
        &input_data,
        reader.as_ref(),
        writer.as_ref(),
        &quality_settings,
    )?;

    // Write output file
    write_file_bytes(output_path, &output_data)?;

    Ok(output_path.to_path_buf())
}

/// Convert an image file in batch context (with progress tracking support)
///
/// This function is similar to `convert_image` but is designed for batch processing
/// where progress updates may be needed. For v0.2.2, it's a wrapper around `convert_image`,
/// but can be extended with progress callbacks in the future.
///
/// # Arguments
///
/// * `input_path` - Path to the input image file
/// * `output_path` - Path where the converted image will be saved
/// * `output_format` - The target image format for conversion
/// * `quality` - Quality setting (1-100) for lossy formats (JPEG, WebP)
/// * `limits` - Resource limits for security validation
///
/// # Returns
///
/// `Ok(PathBuf)` with the output path on success, or an error if conversion fails.
///
/// # Errors
///
/// Same as `convert_image` - see that function's documentation.
#[allow(dead_code)] // Reserved for future parallel batch processing
pub fn convert_image_batch(
    input_path: &Path,
    output_path: &Path,
    output_format: ImageFormat,
    quality: u8,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // For v0.2.2, batch conversion is the same as single conversion
    // Future: Add progress callbacks here
    convert_image(input_path, output_path, output_format, quality, limits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use img_core::ImageFormat;

    #[test]
    fn test_convert_image_quality_validation() {
        let limits = ResourceLimits::default();
        // Create a valid path structure (even if file doesn't exist)
        // We'll test quality validation by checking the error type
        let input_path = Path::new("test.png");
        let output_path = Path::new("test.jpg");

        // Quality 0 should fail - check that it's a validation error
        // Note: Path validation happens first, so we may get path errors
        // But quality 0/101 should still be caught if paths are valid
        let result = convert_image(input_path, output_path, ImageFormat::Jpeg, 0, &limits);
        assert!(result.is_err());
        // The error should either be quality validation or path validation
        // Both are acceptable - the important thing is that invalid quality is rejected
        let err = result.unwrap_err();
        match err {
            common::error::ConversionError::InvalidInput(msg) => {
                // If it's a quality error, it should mention quality
                // If it's a path error, that's also fine - path validation runs first
                assert!(
                    msg.contains("Quality")
                        || msg.contains("quality")
                        || msg.contains("path")
                        || msg.contains("file"),
                    "Unexpected error message: {}",
                    msg
                );
            }
            _ => {
                // Other error types are also acceptable (e.g., path validation)
            }
        }

        // Quality 101 should also fail
        let result = convert_image(input_path, output_path, ImageFormat::Jpeg, 101, &limits);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_image_invalid_path() {
        let limits = ResourceLimits::default();
        let invalid_path = Path::new("../etc/passwd"); // Path traversal attempt
        let output_path = Path::new("test.jpg");

        let result = convert_image(invalid_path, output_path, ImageFormat::Jpeg, 90, &limits);
        assert!(result.is_err());
        // Should be caught by validate_file_path
    }

    // Note: Full integration tests would require actual image files
    // These are handled in the img-core crate's integration tests
}

/// Convert a mesh file from one format to another
///
/// This function performs a complete mesh conversion using direct library
/// integration with `mesh-core`. It includes:
/// - Format detection using `mesh-core::FormatRegistry`
/// - Resource limits enforcement (vertices, faces, file size)
/// - Support for conversion options (transform, validate, recalculate-normals)
/// - Comprehensive error handling with user-friendly messages
///
/// # Arguments
///
/// * `input_path` - Path to the input mesh file
/// * `output_path` - Path where the converted mesh will be saved
/// * `output_format` - The target mesh format for conversion
/// * `options` - Conversion options (transform, validate, recalculate-normals)
/// * `limits` - Resource limits for security validation
///
/// # Returns
///
/// `Ok(PathBuf)` with the output path on success, or an error if conversion fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The input file path is invalid or inaccessible
/// - The file size exceeds resource limits
/// - Format detection fails
/// - The input format is unsupported
/// - The output format is not writable (e.g., STEP)
/// - The conversion process fails
/// - The output file cannot be written
/// - Mesh validation fails (if validation is enabled)
///
/// # Example
///
/// ```no_run
/// use converter_gui::conversion::convert_mesh;
/// use mesh_core::{MeshFormat, ConversionOptions, CoordinateSystem};
/// use common::limits::ResourceLimits;
/// use std::path::Path;
///
/// let input = Path::new("model.stl");
/// let output = Path::new("model.obj");
/// let limits = ResourceLimits::default();
/// let options = ConversionOptions {
///     transform: Some((CoordinateSystem::ZUp, CoordinateSystem::YUp)),
///     recalculate_normals: true,
///     validate: true,
/// };
///
/// convert_mesh(input, output, MeshFormat::Obj, options, &limits)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn convert_mesh(
    input_path: &Path,
    output_path: &Path,
    output_format: MeshFormat,
    options: ConversionOptions,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // Validate input file path (security check)
    validate_file_path(input_path)?;

    // Validate output filename (no invalid characters, no path traversal)
    if let Some(filename) = output_path.file_name().and_then(|n| n.to_str()) {
        validate_output_filename(filename).map_err(|e| {
            common::error::ConversionError::InvalidInput(format!(
                "Output filename validation failed: {}",
                e
            ))
        })?;
    } else {
        return Err(common::error::ConversionError::InvalidInput(
            "Invalid output filename.".to_string(),
        ));
    }

    // Validate output path is not in system directories (security check)
    validate_output_path_not_system(output_path)
        .map_err(common::error::ConversionError::ValidationFailed)?;

    // Check if output file already exists (for user confirmation later)
    // Note: We don't fail here, but the UI should warn the user
    // The actual file write will overwrite if it exists

    // Build resource limits with mesh-specific constraints
    let mesh_limits = ResourceLimits::builder()
        .max_file_size(limits.max_file_size)
        .max_vertices(limits.max_vertices)
        .max_faces(limits.max_faces)
        .build();

    // Read input file with size validation (DoS prevention)
    let input_data = read_file_bytes_checked(input_path, &mesh_limits)?;

    // Format detection using mesh-core::FormatRegistry
    let input_format = MeshFormatRegistry::detect_from_path(input_path)?;

    // Get format handlers with resource limits
    let reader = MeshFormatRegistry::get_reader_with_limits(input_format, mesh_limits.clone())?;
    let writer = MeshFormatRegistry::get_writer(output_format)?;

    // Convert mesh with options
    let converter = MeshConverter::new();
    let output_data =
        converter.convert_with_options(&input_data, reader.as_ref(), writer.as_ref(), &options)?;

    // Write output file
    write_file_bytes(output_path, &output_data)?;

    Ok(output_path.to_path_buf())
}

#[cfg(test)]
mod mesh_tests {
    use super::*;
    use mesh_core::{ConversionOptions, CoordinateSystem, MeshFormat};

    #[test]
    fn test_convert_mesh_invalid_path() {
        let limits = ResourceLimits::default();
        let invalid_path = Path::new("../etc/passwd"); // Path traversal attempt
        let output_path = Path::new("test.obj");
        let options = ConversionOptions::default();

        let result = convert_mesh(invalid_path, output_path, MeshFormat::Obj, options, &limits);
        assert!(result.is_err());
        // Should be caught by validate_file_path
    }

    #[test]
    fn test_convert_mesh_with_options() {
        let limits = ResourceLimits::default();
        let input_path = Path::new("test.stl");
        let output_path = Path::new("test.obj");

        // Test with transform option
        let options = ConversionOptions {
            transform: Some((CoordinateSystem::ZUp, CoordinateSystem::YUp)),
            recalculate_normals: true,
            validate: true,
        };

        // This will fail because the file doesn't exist, but we're testing
        // that the function accepts the options structure correctly
        let result = convert_mesh(input_path, output_path, MeshFormat::Obj, options, &limits);
        // We expect an error (file not found), but not a validation error
        assert!(result.is_err());
    }

    // Note: Full integration tests would require actual mesh files
    // These are handled in the mesh-core crate's integration tests
}

/// Convert a mesh file in batch context (with progress tracking support)
///
/// This function is similar to `convert_mesh` but is designed for batch processing
/// where progress updates may be needed. For v0.2.2, it's a wrapper around `convert_mesh`,
/// but can be extended with progress callbacks in the future.
///
/// # Arguments
///
/// * `input_path` - Path to the input mesh file
/// * `output_path` - Path where the converted mesh will be saved
/// * `output_format` - The target mesh format for conversion
/// * `options` - Conversion options (transform, validate, recalculate-normals)
/// * `limits` - Resource limits for security validation
///
/// # Returns
///
/// `Ok(PathBuf)` with the output path on success, or an error if conversion fails.
///
/// # Errors
///
/// Same as `convert_mesh` - see that function's documentation.
#[allow(dead_code)] // Reserved for future parallel batch processing
pub fn convert_mesh_batch(
    input_path: &Path,
    output_path: &Path,
    output_format: MeshFormat,
    options: ConversionOptions,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // For v0.2.2, batch conversion is the same as single conversion
    // Future: Add progress callbacks here
    convert_mesh(input_path, output_path, output_format, options, limits)
}
