// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Format helper functions for GUI integration
//!
//! This module provides helper functions for format detection and filtering
//! to support the GUI format selection UI components.

use img_core::{FormatRegistry, ImageFormat};
use mesh_core::{FormatRegistry as MeshFormatRegistry, MeshFormat};
use std::path::Path;

/// Get list of writable image formats (excludes read-only formats like SVG)
///
/// This function returns all image formats that can be used as output formats.
/// SVG is excluded because it's a vector format and cannot be written as raster.
///
/// # Returns
///
/// A vector of writable image formats, sorted alphabetically by name.
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::get_writable_image_formats;
///
/// let formats = get_writable_image_formats();
/// assert!(formats.contains(&img_core::ImageFormat::Png));
/// assert!(formats.contains(&img_core::ImageFormat::Jpeg));
/// // SVG is excluded
/// assert!(!formats.contains(&img_core::ImageFormat::Svg));
/// ```
pub fn get_writable_image_formats() -> Vec<ImageFormat> {
    vec![
        ImageFormat::Bmp,
        ImageFormat::Gif,
        ImageFormat::Jpeg,
        ImageFormat::Png,
        ImageFormat::Tiff,
        ImageFormat::WebP,
        // SVG excluded - read-only (vector format)
    ]
}

/// Get list of writable mesh formats (excludes read-only formats like STEP)
///
/// This function returns all mesh formats that can be used as output formats.
/// STEP is excluded because it's read-only in this application context.
///
/// # Returns
///
/// A vector of writable mesh formats, sorted alphabetically by name.
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::get_writable_mesh_formats;
///
/// let formats = get_writable_mesh_formats();
/// assert!(formats.contains(&mesh_core::MeshFormat::Stl));
/// assert!(formats.contains(&mesh_core::MeshFormat::Obj));
/// // STEP is excluded
/// assert!(!formats.contains(&mesh_core::MeshFormat::Step));
/// ```
pub fn get_writable_mesh_formats() -> Vec<MeshFormat> {
    vec![
        MeshFormat::Dxf,
        MeshFormat::Gltf,
        MeshFormat::Obj,
        MeshFormat::Off,
        MeshFormat::Ply,
        MeshFormat::Stl,
        // STEP excluded - read-only (feature-gated, read-only in this context)
    ]
}

/// Check if an image format supports quality settings
///
/// Quality settings are only applicable to lossy formats (JPEG, WebP).
///
/// # Arguments
///
/// * `format` - The image format to check
///
/// # Returns
///
/// `true` if the format supports quality settings, `false` otherwise.
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::format_supports_quality;
/// use img_core::ImageFormat;
///
/// assert!(format_supports_quality(ImageFormat::Jpeg));
/// assert!(format_supports_quality(ImageFormat::WebP));
/// assert!(!format_supports_quality(ImageFormat::Png));
/// ```
pub fn format_supports_quality(format: ImageFormat) -> bool {
    matches!(format, ImageFormat::Jpeg | ImageFormat::WebP)
}

/// Get the file extension for an image format
///
/// Returns the standard file extension (lowercase, without leading dot)
/// for the given image format.
///
/// # Arguments
///
/// * `format` - The image format
///
/// # Returns
///
/// The file extension as a string (e.g., "png", "jpg", "webp")
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::get_format_extension;
/// use img_core::ImageFormat;
///
/// assert_eq!(get_format_extension(ImageFormat::Png), "png");
/// assert_eq!(get_format_extension(ImageFormat::Jpeg), "jpg");
/// ```
pub fn get_format_extension(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Bmp => "bmp",
        ImageFormat::Gif => "gif",
        ImageFormat::Tiff => "tiff",
        ImageFormat::WebP => "webp",
        ImageFormat::Svg => "svg",
    }
}

/// Get the file extension for a mesh format
///
/// Returns the standard file extension (lowercase, without leading dot)
/// for the given mesh format.
///
/// # Arguments
///
/// * `format` - The mesh format
///
/// # Returns
///
/// The file extension as a string (e.g., "stl", "obj", "ply")
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::get_mesh_format_extension;
/// use mesh_core::MeshFormat;
///
/// assert_eq!(get_mesh_format_extension(MeshFormat::Stl), "stl");
/// assert_eq!(get_mesh_format_extension(MeshFormat::Obj), "obj");
/// ```
pub fn get_mesh_format_extension(format: MeshFormat) -> &'static str {
    match format {
        MeshFormat::Stl => "stl",
        MeshFormat::Obj => "obj",
        MeshFormat::Ply => "ply",
        MeshFormat::Off => "off",
        MeshFormat::Gltf => "gltf",
        MeshFormat::Dxf => "dxf",
        MeshFormat::Step => "step",
    }
}

/// Get display name for an image format
///
/// Returns a user-friendly name for the format, suitable for display in the UI.
///
/// # Arguments
///
/// * `format` - The image format
///
/// # Returns
///
/// A user-friendly format name (e.g., "PNG", "JPEG", "WebP")
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::get_image_format_name;
/// use img_core::ImageFormat;
///
/// assert_eq!(get_image_format_name(ImageFormat::Png), "PNG");
/// assert_eq!(get_image_format_name(ImageFormat::Jpeg), "JPEG");
/// ```
pub fn get_image_format_name(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "PNG",
        ImageFormat::Jpeg => "JPEG",
        ImageFormat::Bmp => "BMP",
        ImageFormat::Gif => "GIF",
        ImageFormat::Tiff => "TIFF",
        ImageFormat::WebP => "WebP",
        ImageFormat::Svg => "SVG",
    }
}

/// Get display name for a mesh format
///
/// Returns a user-friendly name for the format, suitable for display in the UI.
///
/// # Arguments
///
/// * `format` - The mesh format
///
/// # Returns
///
/// A user-friendly format name (e.g., "STL", "OBJ", "glTF")
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::get_mesh_format_name;
/// use mesh_core::MeshFormat;
///
/// assert_eq!(get_mesh_format_name(MeshFormat::Stl), "STL");
/// assert_eq!(get_mesh_format_name(MeshFormat::Obj), "OBJ");
/// assert_eq!(get_mesh_format_name(MeshFormat::Gltf), "glTF");
/// ```
pub fn get_mesh_format_name(format: MeshFormat) -> &'static str {
    match format {
        MeshFormat::Stl => "STL",
        MeshFormat::Obj => "OBJ",
        MeshFormat::Ply => "PLY",
        MeshFormat::Off => "OFF",
        MeshFormat::Gltf => "glTF",
        MeshFormat::Dxf => "DXF",
        MeshFormat::Step => "STEP",
    }
}

/// Detect if a file is an image format
///
/// Attempts to detect the image format from the file path.
/// This is a convenience wrapper around `FormatRegistry::detect_from_path`.
///
/// # Arguments
///
/// * `path` - The file path to check
///
/// # Returns
///
/// `Ok(Some(format))` if the file is a recognized image format,
/// `Ok(None)` if the file is not an image format,
/// `Err` if there's an error reading the file.
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::detect_image_format;
/// use std::path::Path;
///
/// let path = Path::new("photo.png");
/// let format = detect_image_format(path).unwrap();
/// assert!(format.is_some());
/// ```
#[allow(dead_code)] // May be used for format detection utilities in future
pub fn detect_image_format(
    path: &Path,
) -> Result<Option<ImageFormat>, common::error::ConversionError> {
    match FormatRegistry::detect_from_path(path) {
        Ok(format) => Ok(Some(format)),
        Err(_) => Ok(None), // Not an image format, but not an error
    }
}

/// Detect if a file is a mesh format
///
/// Attempts to detect the mesh format from the file path.
/// This is a convenience wrapper around `MeshFormatRegistry::detect_from_path`.
///
/// # Arguments
///
/// * `path` - The file path to check
///
/// # Returns
///
/// `Ok(Some(format))` if the file is a recognized mesh format,
/// `Ok(None)` if the file is not a mesh format,
/// `Err` if there's an error reading the file.
///
/// # Example
///
/// ```
/// use converter_gui::format_helpers::detect_mesh_format;
/// use std::path::Path;
///
/// let path = Path::new("model.stl");
/// let format = detect_mesh_format(path).unwrap();
/// assert!(format.is_some());
/// ```
#[allow(dead_code)] // May be used for format detection utilities in future
pub fn detect_mesh_format(
    path: &Path,
) -> Result<Option<MeshFormat>, common::error::ConversionError> {
    match MeshFormatRegistry::detect_from_path(path) {
        Ok(format) => Ok(Some(format)),
        Err(_) => Ok(None), // Not a mesh format, but not an error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_writable_image_formats() {
        let formats = get_writable_image_formats();
        assert!(formats.contains(&ImageFormat::Png));
        assert!(formats.contains(&ImageFormat::Jpeg));
        assert!(formats.contains(&ImageFormat::Bmp));
        assert!(formats.contains(&ImageFormat::Gif));
        assert!(formats.contains(&ImageFormat::Tiff));
        assert!(formats.contains(&ImageFormat::WebP));
        // SVG should be excluded
        assert!(!formats.contains(&ImageFormat::Svg));
    }

    #[test]
    fn test_get_writable_mesh_formats() {
        let formats = get_writable_mesh_formats();
        assert!(formats.contains(&MeshFormat::Stl));
        assert!(formats.contains(&MeshFormat::Obj));
        assert!(formats.contains(&MeshFormat::Ply));
        assert!(formats.contains(&MeshFormat::Off));
        assert!(formats.contains(&MeshFormat::Gltf));
        assert!(formats.contains(&MeshFormat::Dxf));
        // STEP should be excluded
        assert!(!formats.contains(&MeshFormat::Step));
    }

    #[test]
    fn test_format_supports_quality() {
        assert!(format_supports_quality(ImageFormat::Jpeg));
        assert!(format_supports_quality(ImageFormat::WebP));
        assert!(!format_supports_quality(ImageFormat::Png));
        assert!(!format_supports_quality(ImageFormat::Bmp));
        assert!(!format_supports_quality(ImageFormat::Gif));
        assert!(!format_supports_quality(ImageFormat::Tiff));
        assert!(!format_supports_quality(ImageFormat::Svg));
    }

    #[test]
    fn test_get_format_extension() {
        assert_eq!(get_format_extension(ImageFormat::Png), "png");
        assert_eq!(get_format_extension(ImageFormat::Jpeg), "jpg");
        assert_eq!(get_format_extension(ImageFormat::Bmp), "bmp");
        assert_eq!(get_format_extension(ImageFormat::Gif), "gif");
        assert_eq!(get_format_extension(ImageFormat::Tiff), "tiff");
        assert_eq!(get_format_extension(ImageFormat::WebP), "webp");
        assert_eq!(get_format_extension(ImageFormat::Svg), "svg");
    }

    #[test]
    fn test_get_mesh_format_extension() {
        assert_eq!(get_mesh_format_extension(MeshFormat::Stl), "stl");
        assert_eq!(get_mesh_format_extension(MeshFormat::Obj), "obj");
        assert_eq!(get_mesh_format_extension(MeshFormat::Ply), "ply");
        assert_eq!(get_mesh_format_extension(MeshFormat::Off), "off");
        assert_eq!(get_mesh_format_extension(MeshFormat::Gltf), "gltf");
        assert_eq!(get_mesh_format_extension(MeshFormat::Dxf), "dxf");
    }

    #[test]
    fn test_detect_image_format() {
        use std::path::Path;

        let png_path = Path::new("test.png");
        let result = detect_image_format(png_path).unwrap();
        assert_eq!(result, Some(ImageFormat::Png));

        let jpg_path = Path::new("test.jpg");
        let result = detect_image_format(jpg_path).unwrap();
        assert_eq!(result, Some(ImageFormat::Jpeg));

        let txt_path = Path::new("test.txt");
        let result = detect_image_format(txt_path).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_mesh_format() {
        use std::path::Path;

        let stl_path = Path::new("test.stl");
        let result = detect_mesh_format(stl_path).unwrap();
        assert_eq!(result, Some(MeshFormat::Stl));

        let obj_path = Path::new("test.obj");
        let result = detect_mesh_format(obj_path).unwrap();
        assert_eq!(result, Some(MeshFormat::Obj));

        let txt_path = Path::new("test.txt");
        let result = detect_mesh_format(txt_path).unwrap();
        assert_eq!(result, None);
    }
}
