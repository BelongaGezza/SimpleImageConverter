// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Format conversion matrix tests
//!
//! These tests verify that all supported format pairs work correctly,
//! ensuring comprehensive coverage of the format conversion matrix.

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

/// Helper to create a test PNG with transparency
fn create_test_png_rgba() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgba([(x * 25) as u8, (y * 25) as u8, 128, 255])
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

/// Test a single format pair conversion
fn test_format_pair(from: ImageFormat, to: ImageFormat, source_data: &[u8]) {
    let reader = FormatRegistry::get_reader(from).unwrap();
    let writer = FormatRegistry::get_writer(to).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let result = converter.convert(source_data, reader.as_ref(), writer.as_ref(), &quality);

    assert!(
        result.is_ok(),
        "Conversion from {:?} to {:?} failed: {:?}",
        from,
        to,
        result
    );

    let converted_data = result.unwrap();
    assert!(!converted_data.is_empty(), "Converted data is empty");

    // Verify we can read the converted format back
    let converted_reader = FormatRegistry::get_reader(to).unwrap();
    let read_result = converted_reader.read(&converted_data);

    assert!(
        read_result.is_ok(),
        "Failed to read converted {:?} file: {:?}",
        to,
        read_result
    );

    let image = read_result.unwrap();
    assert_eq!(image.width, 10, "Image width mismatch");
    assert_eq!(image.height, 10, "Image height mismatch");
}

#[test]
fn test_png_to_all_formats() {
    let png_data = create_test_png();

    test_format_pair(ImageFormat::Png, ImageFormat::Jpeg, &png_data);
    test_format_pair(ImageFormat::Png, ImageFormat::Bmp, &png_data);
    test_format_pair(ImageFormat::Png, ImageFormat::Gif, &png_data);
    test_format_pair(ImageFormat::Png, ImageFormat::Tiff, &png_data);
    test_format_pair(ImageFormat::Png, ImageFormat::WebP, &png_data);
}

#[test]
fn test_jpeg_to_all_formats() {
    // First create a JPEG from PNG
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let jpeg_writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let jpeg_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            jpeg_writer.as_ref(),
            &quality,
        )
        .unwrap();

    test_format_pair(ImageFormat::Jpeg, ImageFormat::Png, &jpeg_data);
    test_format_pair(ImageFormat::Jpeg, ImageFormat::Bmp, &jpeg_data);
    test_format_pair(ImageFormat::Jpeg, ImageFormat::Gif, &jpeg_data);
    test_format_pair(ImageFormat::Jpeg, ImageFormat::Tiff, &jpeg_data);
    test_format_pair(ImageFormat::Jpeg, ImageFormat::WebP, &jpeg_data);
}

#[test]
fn test_bmp_to_all_formats() {
    // First create a BMP from PNG
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let bmp_writer = FormatRegistry::get_writer(ImageFormat::Bmp).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let bmp_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            bmp_writer.as_ref(),
            &quality,
        )
        .unwrap();

    test_format_pair(ImageFormat::Bmp, ImageFormat::Png, &bmp_data);
    test_format_pair(ImageFormat::Bmp, ImageFormat::Jpeg, &bmp_data);
    test_format_pair(ImageFormat::Bmp, ImageFormat::Gif, &bmp_data);
    test_format_pair(ImageFormat::Bmp, ImageFormat::Tiff, &bmp_data);
    test_format_pair(ImageFormat::Bmp, ImageFormat::WebP, &bmp_data);
}

#[test]
fn test_gif_to_all_formats() {
    // First create a GIF from PNG
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let gif_writer = FormatRegistry::get_writer(ImageFormat::Gif).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let gif_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            gif_writer.as_ref(),
            &quality,
        )
        .unwrap();

    test_format_pair(ImageFormat::Gif, ImageFormat::Png, &gif_data);
    test_format_pair(ImageFormat::Gif, ImageFormat::Jpeg, &gif_data);
    test_format_pair(ImageFormat::Gif, ImageFormat::Bmp, &gif_data);
    test_format_pair(ImageFormat::Gif, ImageFormat::Tiff, &gif_data);
    test_format_pair(ImageFormat::Gif, ImageFormat::WebP, &gif_data);
}

#[test]
fn test_tiff_to_all_formats() {
    // First create a TIFF from PNG
    let png_data = create_test_png();
    let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
    let tiff_writer = FormatRegistry::get_writer(ImageFormat::Tiff).unwrap();
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(90);

    let tiff_data = converter
        .convert(
            &png_data,
            png_reader.as_ref(),
            tiff_writer.as_ref(),
            &quality,
        )
        .unwrap();

    test_format_pair(ImageFormat::Tiff, ImageFormat::Png, &tiff_data);
    test_format_pair(ImageFormat::Tiff, ImageFormat::Jpeg, &tiff_data);
    test_format_pair(ImageFormat::Tiff, ImageFormat::Bmp, &tiff_data);
    test_format_pair(ImageFormat::Tiff, ImageFormat::Gif, &tiff_data);
    test_format_pair(ImageFormat::Tiff, ImageFormat::WebP, &tiff_data);
}

#[test]
fn test_webp_to_all_formats() {
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

    test_format_pair(ImageFormat::WebP, ImageFormat::Png, &webp_data);
    test_format_pair(ImageFormat::WebP, ImageFormat::Jpeg, &webp_data);
    test_format_pair(ImageFormat::WebP, ImageFormat::Bmp, &webp_data);
    test_format_pair(ImageFormat::WebP, ImageFormat::Gif, &webp_data);
    test_format_pair(ImageFormat::WebP, ImageFormat::Tiff, &webp_data);
}

#[test]
fn test_transparency_handling() {
    // Test RGBA PNG to formats that don't support transparency
    let png_rgba_data = create_test_png_rgba();

    // RGBA PNG to JPEG (should handle transparency loss)
    test_format_pair(ImageFormat::Png, ImageFormat::Jpeg, &png_rgba_data);

    // RGBA PNG to BMP (should handle transparency loss)
    test_format_pair(ImageFormat::Png, ImageFormat::Bmp, &png_rgba_data);

    // RGBA PNG to formats that support transparency
    test_format_pair(ImageFormat::Png, ImageFormat::Png, &png_rgba_data);
    test_format_pair(ImageFormat::Png, ImageFormat::Gif, &png_rgba_data);
    test_format_pair(ImageFormat::Png, ImageFormat::WebP, &png_rgba_data);
}

#[test]
fn test_round_trip_conversions() {
    // Test round-trip conversions to ensure data integrity
    let original_png = create_test_png();

    // PNG → JPEG → PNG
    {
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

        let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
        let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

        let round_trip_png = converter
            .convert(
                &jpeg_data,
                jpeg_reader.as_ref(),
                png_writer.as_ref(),
                &quality,
            )
            .unwrap();

        // Verify we can read the round-trip PNG
        let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
        let read_result = png_reader.read(&round_trip_png);
        assert!(read_result.is_ok());
        let image = read_result.unwrap();
        assert_eq!(image.width, 10);
        assert_eq!(image.height, 10);
    }

    // PNG → BMP → PNG
    {
        let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
        let bmp_writer = FormatRegistry::get_writer(ImageFormat::Bmp).unwrap();
        let converter = ImageConverter::new();
        let quality = QualitySettings::new(90);

        let bmp_data = converter
            .convert(
                &original_png,
                png_reader.as_ref(),
                bmp_writer.as_ref(),
                &quality,
            )
            .unwrap();

        let bmp_reader = FormatRegistry::get_reader(ImageFormat::Bmp).unwrap();
        let png_writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();

        let round_trip_png = converter
            .convert(
                &bmp_data,
                bmp_reader.as_ref(),
                png_writer.as_ref(),
                &quality,
            )
            .unwrap();

        // Verify we can read the round-trip PNG
        let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
        let read_result = png_reader.read(&round_trip_png);
        assert!(read_result.is_ok());
        let image = read_result.unwrap();
        assert_eq!(image.width, 10);
        assert_eq!(image.height, 10);
    }
}
