// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Security-focused tests for image format readers
//!
//! These tests verify that format readers properly handle malicious or malformed input
//! without panicking, leaking memory, or causing denial of service.

use img_core::formats::traits::ImageReader;
use img_core::formats::{BmpFormat, GifFormat, JpegFormat, PngFormat};
use common::limits::ResourceLimits;

#[test]
fn test_png_reject_oversized_input() {
    let format = PngFormat::new();
    let limits = ResourceLimits::default();
    
    // Create data larger than limit (but still valid PNG header)
    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG header
    oversized_data.resize(oversized_size, 0);
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds limit"));
}

#[test]
fn test_jpeg_reject_oversized_input() {
    let format = JpegFormat::new();
    let limits = ResourceLimits::default();
    
    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0xFF, 0xD8, 0xFF]; // JPEG header
    oversized_data.resize(oversized_size, 0);
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_bmp_reject_oversized_input() {
    let format = BmpFormat::new();
    let limits = ResourceLimits::default();
    
    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0x42, 0x4D]; // BMP header
    oversized_data.resize(oversized_size, 0);
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_gif_reject_oversized_input() {
    let format = GifFormat::new();
    let limits = ResourceLimits::default();
    
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
    use img_core::validation::validate_image_data;
    use img_core::formats::traits::{ColorType, ImageData};
    
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

