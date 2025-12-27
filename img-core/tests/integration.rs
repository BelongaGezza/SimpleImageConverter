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
        .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
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
        .convert(&original_png, png_reader.as_ref(), jpeg_writer.as_ref(), &quality)
        .unwrap();
    
    // JPEG → PNG
    let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
    let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();
    
    let final_png = converter
        .convert(&jpeg_data, jpeg_reader.as_ref(), png_writer.as_ref(), &quality)
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
        .convert(&png_data, png_reader.as_ref(), jpeg_writer.as_ref(), &quality_low)
        .unwrap();
    
    // Test high quality
    let quality_high = QualitySettings::new(100);
    let jpeg_high = converter
        .convert(&png_data, png_reader.as_ref(), jpeg_writer.as_ref(), &quality_high)
        .unwrap();
    
    // Both should succeed
    assert!(!jpeg_low.is_empty());
    assert!(!jpeg_high.is_empty());
}

#[test]
fn test_conversion_invalid_format() {
    // Try to get unsupported format
    let result = FormatRegistry::get_reader(ImageFormat::Bmp);
    assert!(result.is_err());
    
    let result = FormatRegistry::get_writer(ImageFormat::Gif);
    assert!(result.is_err());
}

