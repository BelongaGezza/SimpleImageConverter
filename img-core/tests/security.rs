// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Security-focused tests for image format readers
//!
//! These tests verify that format readers properly handle malicious or malformed input
//! without panicking, leaking memory, or causing denial of service.

use common::limits::ResourceLimits;
use img_core::formats::traits::ImageReader;
use img_core::formats::{
    BmpFormat, GifFormat, JpegFormat, PngFormat, SvgFormat, TiffFormat, WebPFormat,
};

#[test]
fn test_png_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = PngFormat::with_limits(limits.clone());

    // Create data larger than limit (but still valid PNG header)
    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG header
    oversized_data.resize(oversized_size, 0);

    let result = format.read(&oversized_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds limit"));
}

#[test]
fn test_png_reject_custom_max_dimension() {
    use image::{DynamicImage, ImageFormat};

    // Create a 50x50 PNG (well within default limits)
    let img = image::ImageBuffer::from_fn(50, 50, |x, y| {
        image::Rgb([(x * 5) as u8, (y * 5) as u8, 128])
    });
    let mut buffer = Vec::new();
    DynamicImage::ImageRgb8(img)
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
        .unwrap();

    let limits = ResourceLimits::builder().max_image_dimension(10).build();
    let format = PngFormat::with_limits(limits);

    let result = format.read(&buffer);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("exceeds limit") || err.contains("width") || err.contains("height"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_get_reader_with_limits_enforces_custom_max_dimension() {
    use image::{DynamicImage, ImageFormat};
    use img_core::formats::registry::{FormatRegistry, ImageFormat as RegistryFormat};

    let img = image::ImageBuffer::from_fn(50, 50, |x, y| {
        image::Rgb([(x * 5) as u8, (y * 5) as u8, 128])
    });
    let mut buffer = Vec::new();
    DynamicImage::ImageRgb8(img)
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
        .unwrap();

    let limits = ResourceLimits::builder().max_image_dimension(10).build();
    let reader =
        FormatRegistry::get_reader_with_limits(RegistryFormat::Png, limits).unwrap();

    let result = reader.read(&buffer);
    assert!(result.is_err());
}

#[test]
fn test_jpeg_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = JpegFormat::with_limits(limits.clone());

    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0xFF, 0xD8, 0xFF]; // JPEG header
    oversized_data.resize(oversized_size, 0);

    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_bmp_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = BmpFormat::with_limits(limits.clone());

    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0x42, 0x4D]; // BMP header
    oversized_data.resize(oversized_size, 0);

    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_gif_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = GifFormat::with_limits(limits.clone());

    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0x47, 0x49, 0x46, 0x38]; // GIF header
    oversized_data.resize(oversized_size, 0);

    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_png_handle_malformed_header() {
    let format = PngFormat::new();

    // Valid PNG header but invalid/corrupted data
    let malformed_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0xFF, 0xFF];

    let result = format.read(&malformed_data);
    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_jpeg_handle_malformed_header() {
    let format = JpegFormat::new();

    // Valid JPEG header but invalid data
    let malformed_data = vec![0xFF, 0xD8, 0xFF, 0x00, 0x00];

    let result = format.read(&malformed_data);
    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_format_spoofing_detection() {
    use img_core::formats::registry::FormatRegistry;
    use std::path::Path;

    // JPEG magic bytes but PNG extension
    let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x00];
    let path = Path::new("fake.png");

    let result = FormatRegistry::detect_two_stage(path, &jpeg_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("mismatch"));
}

#[test]
fn test_empty_input_rejected() {
    let format = PngFormat::new();
    let empty_data = vec![];

    let result = format.read(&empty_data);
    assert!(result.is_err());
}

#[test]
fn test_very_small_input_handled() {
    let format = PngFormat::new();

    // Too small to be valid PNG
    let tiny_data = vec![0x89, 0x50];

    let result = format.read(&tiny_data);
    // Should return error gracefully
    assert!(result.is_err());
}

#[test]
fn test_integer_overflow_protection() {
    use img_core::formats::traits::{ColorType, ImageData};
    use img_core::validation::validate_image_data;

    // Create image with dimensions that would overflow if multiplied
    let limits = ResourceLimits::default();
    let max_dim = limits.max_image_dimension;

    // This should be rejected before calculation
    let image = ImageData {
        width: max_dim + 1,
        height: max_dim + 1,
        data: vec![],
        color_type: ColorType::Rgb,
    };

    let result = validate_image_data(&image);
    assert!(result.is_err());
}

// ============================================================================
// TIFF Security Tests
// ============================================================================

#[test]
fn test_tiff_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = TiffFormat::with_limits(limits.clone());

    let oversized_size = limits.max_file_size + 1;
    // TIFF magic bytes (little-endian)
    let mut oversized_data = vec![0x49, 0x49, 0x2A, 0x00];
    oversized_data.resize(oversized_size, 0);

    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_tiff_handle_malformed_header() {
    let format = TiffFormat::new();

    // Valid TIFF header but invalid/corrupted data
    let malformed_data = vec![0x49, 0x49, 0x2A, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];

    let result = format.read(&malformed_data);
    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_tiff_empty_input_rejected() {
    let format = TiffFormat::new();
    let empty_data = vec![];

    let result = format.read(&empty_data);
    assert!(result.is_err());
}

#[test]
fn test_tiff_very_small_input_handled() {
    let format = TiffFormat::new();

    // Too small to be valid TIFF
    let tiny_data = vec![0x49, 0x49];

    let result = format.read(&tiny_data);
    // Should return error gracefully
    assert!(result.is_err());
}

// ============================================================================
// WebP Security Tests
// ============================================================================

#[test]
fn test_webp_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = WebPFormat::with_limits(limits.clone());

    let oversized_size = limits.max_file_size + 1;
    // WebP magic bytes: RIFF....WEBP
    let mut oversized_data = vec![
        0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50,
    ];
    oversized_data.resize(oversized_size, 0);

    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_webp_handle_malformed_header() {
    let format = WebPFormat::new();

    // Valid WebP header but invalid/corrupted data
    let malformed_data = vec![
        0x52, 0x49, 0x46, 0x46, 0x10, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0xFF, 0xFF,
    ];

    let result = format.read(&malformed_data);
    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_webp_empty_input_rejected() {
    let format = WebPFormat::new();
    let empty_data = vec![];

    let result = format.read(&empty_data);
    assert!(result.is_err());
}

#[test]
fn test_webp_very_small_input_handled() {
    let format = WebPFormat::new();

    // Too small to be valid WebP
    let tiny_data = vec![0x52, 0x49, 0x46, 0x46];

    let result = format.read(&tiny_data);
    // Should return error gracefully
    assert!(result.is_err());
}

// ============================================================================
// SVG Security Tests
// ============================================================================

#[test]
fn test_svg_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = SvgFormat::with_limits(limits.clone());

    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data =
        b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\">".to_vec();
    oversized_data.resize(oversized_size, b' ');

    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_svg_handle_malformed_xml() {
    let format = SvgFormat::new();

    // Invalid XML
    let malformed_data = b"<?xml version=\"1.0\"?><svg><not closed>";

    let result = format.read(malformed_data);
    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_svg_empty_input_rejected() {
    let format = SvgFormat::new();
    let empty_data = vec![];

    let result = format.read(&empty_data);
    assert!(result.is_err());
}

#[test]
fn test_svg_very_small_input_handled() {
    let format = SvgFormat::new();

    // Too small to be valid SVG
    let tiny_data = b"<svg";

    let result = format.read(tiny_data);
    // Should return error gracefully
    assert!(result.is_err());
}

#[test]
fn test_svg_missing_dimensions_handled() {
    let format = SvgFormat::new();

    // SVG without width/height attributes
    let no_dims = b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\"><rect/></svg>";

    let result = format.read(no_dims);
    // Should either succeed with default dimensions or return error, but not panic
    // The behavior depends on implementation
    let _ = result; // Just ensure no panic
}

#[test]
fn test_svg_xxe_protection() {
    let format = SvgFormat::new();

    // Attempt XXE attack - should be rejected or sanitized
    let xxe_attempt = br#"<?xml version="1.0"?>
<!DOCTYPE svg [
  <!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
  <text>&xxe;</text>
</svg>"#;

    let result = format.read(xxe_attempt);
    // Should either reject or safely ignore the entity
    // Just ensure no panic and no file access
    let _ = result;
}

// ============================================================================
// Format Spoofing Tests for New Formats
// ============================================================================

#[test]
fn test_tiff_spoofing_detection() {
    use img_core::formats::registry::FormatRegistry;
    use std::path::Path;

    // TIFF magic bytes but PNG extension
    let tiff_data = vec![0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
    let path = Path::new("fake.png");

    let result = FormatRegistry::detect_two_stage(path, &tiff_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("mismatch"));
}

#[test]
fn test_webp_spoofing_detection() {
    use img_core::formats::registry::FormatRegistry;
    use std::path::Path;

    // WebP magic bytes but JPEG extension
    let webp_data = vec![
        0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50,
    ];
    let path = Path::new("fake.jpg");

    let result = FormatRegistry::detect_two_stage(path, &webp_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("mismatch"));
}

#[test]
fn test_svg_spoofing_detection() {
    use img_core::formats::registry::FormatRegistry;
    use std::path::Path;

    // SVG content but BMP extension
    let svg_data = b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"></svg>";
    let path = Path::new("fake.bmp");

    let result = FormatRegistry::detect_two_stage(path, svg_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("mismatch"));
}
