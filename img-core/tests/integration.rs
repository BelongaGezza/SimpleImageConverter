// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use img_core::{FormatRegistry, ImageConverter, ImageFormat, QualitySettings};

/// Helper to create a simple test PNG
fn create_test_png() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut buffer = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .unwrap();
    buffer
}

#[test]
fn test_png_to_jpeg_conversion() {
    // Create test PNG
    let png_data = create_test_png();

    // Get format handlers
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    // Verify we can read the JPEG back
    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_jpeg_to_png_conversion() {
    // Create test JPEG
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut jpeg_data = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 90);
    encoder
        .encode(img.as_raw(), 10, 10, image::ExtendedColorType::Rgb8)
        .unwrap();

    // Get format handlers
    let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&jpeg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let png_data = result.unwrap();
    assert!(!png_data.is_empty());

    // Verify we can read the PNG back
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let read_result = png_reader.read(&png_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_round_trip_png_jpeg_png() {
    // Start with PNG
    let original_png = create_test_png();

    // PNG → JPEG
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let jpeg_writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let jpeg_data = converter
        .convert(
            &original_png,
            png_reader.as_ref(),
            jpeg_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // JPEG → PNG
    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    let final_png = converter
        .convert(
            &jpeg_data,
            jpeg_reader.as_ref(),
            png_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // Verify final PNG is valid
    let final_image = png_reader.read(&final_png).unwrap();
    assert_eq!(final_image.width, 10);
    assert_eq!(final_image.height, 10);
}

#[test]
fn test_conversion_with_different_quality() {
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let jpeg_writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let converter = ImageConverter::new();

    // Test low quality
    let quality_low = QualitySettings::new(50);
    let jpeg_low = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            jpeg_writer.as_ref(),
            &quality_low,
        )
        .unwrap();

    // Test high quality
    let quality_high = QualitySettings::new(100);
    let jpeg_high = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            jpeg_writer.as_ref(),
            &quality_high,
        )
        .unwrap();

    // Both should succeed
    assert!(!jpeg_low.is_empty());
    assert!(!jpeg_high.is_empty());
}

#[test]
fn test_bmp_to_png_conversion() {
    // Create test BMP
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut bmp_data = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut bmp_data),
            image::ImageFormat::Bmp,
        )
        .unwrap();

    // Get format handlers
    let reader = FormatRegistry::get_reader(ImageFormat::Bmp).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&bmp_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let png_data = result.unwrap();
    assert!(!png_data.is_empty());

    // Verify we can read the PNG back
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let read_result = png_reader.read(&png_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_png_to_bmp_conversion() {
    // Create test PNG
    let png_data = create_test_png();

    // Get format handlers
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Bmp).unwrap();

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let bmp_data = result.unwrap();
    assert!(!bmp_data.is_empty());

    // Verify we can read the BMP back
    let bmp_reader = FormatRegistry::get_reader(ImageFormat::Bmp).unwrap();
    let read_result = bmp_reader.read(&bmp_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_gif_to_png_conversion() {
    // Create test GIF
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut gif_data = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut gif_data),
            image::ImageFormat::Gif,
        )
        .unwrap();

    // Get format handlers
    let reader = FormatRegistry::get_reader(ImageFormat::Gif).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&gif_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let png_data = result.unwrap();
    assert!(!png_data.is_empty());

    // Verify we can read the PNG back
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let read_result = png_reader.read(&png_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_png_to_gif_conversion() {
    // Create test PNG
    let png_data = create_test_png();

    // Get format handlers
    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Gif).unwrap();

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let gif_data = result.unwrap();
    assert!(!gif_data.is_empty());

    // Verify we can read the GIF back
    let gif_reader = FormatRegistry::get_reader(ImageFormat::Gif).unwrap();
    let read_result = gif_reader.read(&gif_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

// ============================================================================
// TIFF Format Integration Tests
// ============================================================================

/// Helper to create a test TIFF image
fn create_test_tiff() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut buffer = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Tiff,
        )
        .unwrap();
    buffer
}

#[test]
fn test_png_to_tiff_conversion() {
    let png_data = create_test_png();

    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Tiff).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let tiff_data = result.unwrap();
    assert!(!tiff_data.is_empty());

    // Verify we can read the TIFF back
    let tiff_reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let read_result = tiff_reader.read(&tiff_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_tiff_to_png_conversion() {
    let tiff_data = create_test_tiff();

    let reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&tiff_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let png_data = result.unwrap();
    assert!(!png_data.is_empty());

    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let read_result = png_reader.read(&png_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_tiff_to_jpeg_conversion() {
    let tiff_data = create_test_tiff();

    let reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&tiff_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_jpeg_to_tiff_conversion() {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut jpeg_data = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 90);
    encoder
        .encode(img.as_raw(), 10, 10, image::ExtendedColorType::Rgb8)
        .unwrap();

    let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Tiff).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&jpeg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let tiff_data = result.unwrap();
    assert!(!tiff_data.is_empty());

    let tiff_reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let read_result = tiff_reader.read(&tiff_data);
    assert!(read_result.is_ok());
}

// ============================================================================
// WebP Format Integration Tests
// ============================================================================

#[test]
fn test_png_to_webp_conversion() {
    let png_data = create_test_png();

    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let webp_data = result.unwrap();
    assert!(!webp_data.is_empty());

    // Verify we can read the WebP back
    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let read_result = webp_reader.read(&webp_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_webp_to_png_conversion() {
    // First create a WebP from PNG
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let webp_writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let webp_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            webp_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // Now convert WebP back to PNG
    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    let result = converter.convert(
        &webp_data,
        webp_reader.as_ref(),
        png_writer.as_ref(),
        &quality,
    );

    assert!(result.is_ok());
    let final_png = result.unwrap();
    assert!(!final_png.is_empty());

    let read_result = png_reader.read(&final_png);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_jpeg_to_webp_conversion() {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut jpeg_data = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 90);
    encoder
        .encode(img.as_raw(), 10, 10, image::ExtendedColorType::Rgb8)
        .unwrap();

    let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&jpeg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let webp_data = result.unwrap();
    assert!(!webp_data.is_empty());

    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let read_result = webp_reader.read(&webp_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_webp_to_jpeg_conversion() {
    // Create WebP from PNG first
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let webp_writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let webp_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            webp_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // Convert WebP to JPEG
    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let jpeg_writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    let result = converter.convert(
        &webp_data,
        webp_reader.as_ref(),
        jpeg_writer.as_ref(),
        &quality,
    );

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_webp_quality_settings() {
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let webp_writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();
    let converter = ImageConverter::new();

    // Low quality
    let quality_low = QualitySettings::new(30);
    let webp_low = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            webp_writer.as_ref(),
            &quality_low,
        )
        .unwrap();

    // High quality
    let quality_high = QualitySettings::new(100);
    let webp_high = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            webp_writer.as_ref(),
            &quality_high,
        )
        .unwrap();

    // Both should succeed
    assert!(!webp_low.is_empty());
    assert!(!webp_high.is_empty());
}

// ============================================================================
// SVG Format Integration Tests (read-only, converts to raster)
// ============================================================================

/// Create a simple test SVG
fn create_test_svg() -> Vec<u8> {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
  <rect x="0" y="0" width="100" height="100" fill="blue"/>
  <circle cx="50" cy="50" r="40" fill="red"/>
</svg>"#
        .as_bytes()
        .to_vec()
}

#[test]
fn test_svg_to_png_conversion() {
    let svg_data = create_test_svg();

    let reader = FormatRegistry::get_reader(ImageFormat::Svg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&svg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let png_data = result.unwrap();
    assert!(!png_data.is_empty());

    // Verify the PNG
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let read_result = png_reader.read(&png_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert!(image.width > 0);
    assert!(image.height > 0);
}

#[test]
fn test_svg_to_jpeg_conversion() {
    let svg_data = create_test_svg();

    let reader = FormatRegistry::get_reader(ImageFormat::Svg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&svg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_svg_to_webp_conversion() {
    let svg_data = create_test_svg();

    let reader = FormatRegistry::get_reader(ImageFormat::Svg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&svg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let webp_data = result.unwrap();
    assert!(!webp_data.is_empty());

    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let read_result = webp_reader.read(&webp_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_svg_to_tiff_conversion() {
    let svg_data = create_test_svg();

    let reader = FormatRegistry::get_reader(ImageFormat::Svg).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Tiff).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&svg_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let tiff_data = result.unwrap();
    assert!(!tiff_data.is_empty());

    let tiff_reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let read_result = tiff_reader.read(&tiff_data);
    assert!(read_result.is_ok());
}

// ============================================================================
// Edge Case Tests
// ============================================================================

/// Helper to create a 1x1 pixel PNG
fn create_1x1_png() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(1, 1, |_, _| image::Rgba([255, 0, 0, 255]));
    let mut buffer = Vec::new();
    image::DynamicImage::ImageRgba8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .unwrap();
    buffer
}

/// Helper to create a PNG with transparency
fn create_transparent_png() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        // Checkerboard with varying alpha
        let alpha = if (x + y) % 2 == 0 { 255 } else { 128 };
        image::Rgba([(x * 25) as u8, (y * 25) as u8, 128, alpha])
    });
    let mut buffer = Vec::new();
    image::DynamicImage::ImageRgba8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .unwrap();
    buffer
}

/// Helper to create a grayscale PNG
fn create_grayscale_png() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| image::Luma([((x + y) * 12) as u8]));
    let mut buffer = Vec::new();
    image::DynamicImage::ImageLuma8(img)
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .unwrap();
    buffer
}

#[test]
fn test_1x1_pixel_conversion() {
    let tiny_png = create_1x1_png();

    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&tiny_png, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
    let image = read_result.unwrap();
    assert_eq!(image.width, 1);
    assert_eq!(image.height, 1);
}

#[test]
fn test_transparency_to_jpeg_conversion() {
    // JPEG doesn't support transparency, so alpha should be handled
    let transparent_png = create_transparent_png();

    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&transparent_png, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    // Verify JPEG is readable
    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_transparency_to_webp_conversion() {
    // WebP supports transparency
    let transparent_png = create_transparent_png();

    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&transparent_png, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let webp_data = result.unwrap();
    assert!(!webp_data.is_empty());

    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let read_result = webp_reader.read(&webp_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_grayscale_to_color_conversion() {
    let grayscale_png = create_grayscale_png();

    let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();

    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);
    let result = converter.convert(&grayscale_png, reader.as_ref(), writer.as_ref(), &quality);

    assert!(result.is_ok());
    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let read_result = jpeg_reader.read(&jpeg_data);
    assert!(read_result.is_ok());
}

#[test]
fn test_round_trip_tiff() {
    let png_data = create_test_png();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(100);

    // PNG → TIFF
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let tiff_writer = FormatRegistry::get_writer(ImageFormat::Tiff).unwrap();
    let tiff_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            tiff_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // TIFF → PNG
    let tiff_reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();
    let final_png = converter
        .convert(
            &tiff_data,
            tiff_reader.as_ref(),
            png_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // Verify
    let image = png_reader.read(&final_png).unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_round_trip_webp() {
    let png_data = create_test_png();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(100);

    // PNG → WebP
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let webp_writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();
    let webp_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            webp_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // WebP → PNG
    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();
    let final_png = converter
        .convert(
            &webp_data,
            webp_reader.as_ref(),
            png_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // Verify
    let image = png_reader.read(&final_png).unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}

#[test]
fn test_cross_format_chain_conversion() {
    // PNG → JPEG → TIFF → WebP → BMP → GIF
    let png_data = create_test_png();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    // PNG → JPEG
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let jpeg_writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let jpeg_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            jpeg_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // JPEG → TIFF
    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let tiff_writer = FormatRegistry::get_writer(ImageFormat::Tiff).unwrap();
    let tiff_data = converter
        .convert(
            &jpeg_data,
            jpeg_reader.as_ref(),
            tiff_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // TIFF → WebP
    let tiff_reader = FormatRegistry::get_reader(ImageFormat::Tiff).unwrap();
    let webp_writer = FormatRegistry::get_writer(ImageFormat::WebP).unwrap();
    let webp_data = converter
        .convert(
            &tiff_data,
            tiff_reader.as_ref(),
            webp_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // WebP → BMP
    let webp_reader = FormatRegistry::get_reader(ImageFormat::WebP).unwrap();
    let bmp_writer = FormatRegistry::get_writer(ImageFormat::Bmp).unwrap();
    let bmp_data = converter
        .convert(
            &webp_data,
            webp_reader.as_ref(),
            bmp_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // BMP → GIF
    let bmp_reader = FormatRegistry::get_reader(ImageFormat::Bmp).unwrap();
    let gif_writer = FormatRegistry::get_writer(ImageFormat::Gif).unwrap();
    let gif_data = converter
        .convert(
            &bmp_data,
            bmp_reader.as_ref(),
            gif_writer.as_ref(),
            &quality,
        )
        .unwrap();

    // Verify final GIF
    let gif_reader = FormatRegistry::get_reader(ImageFormat::Gif).unwrap();
    let image = gif_reader.read(&gif_data).unwrap();
    assert_eq!(image.width, 10);
    assert_eq!(image.height, 10);
}
