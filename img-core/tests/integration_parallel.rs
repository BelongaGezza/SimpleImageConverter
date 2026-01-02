// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for parallel/concurrent image format conversion
//!
//! These tests verify that all 2D image formats work correctly when used
//! in parallel processing scenarios, ensuring thread safety and correctness.

use img_core::{FormatRegistry, ImageConverter, ImageFormat, QualitySettings};
use std::sync::{Arc, Mutex};
use std::thread;

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

/// Helper to create a test JPEG
fn create_test_jpeg() -> Vec<u8> {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x * 25) as u8, (y * 25) as u8, 128])
    });
    let mut jpeg_data = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 90);
    encoder
        .encode(img.as_raw(), 10, 10, image::ExtendedColorType::Rgb8)
        .unwrap();
    jpeg_data
}

#[test]
fn test_parallel_png_to_jpeg_conversion() {
    // Test that multiple PNG to JPEG conversions can run in parallel
    let png_data = create_test_png();
    let num_threads = 4;
    let num_conversions_per_thread = 10;

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let png_data = png_data.clone();
        let results = results.clone();

        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let mut thread_results = Vec::new();

            for _ in 0..num_conversions_per_thread {
                let result =
                    converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);
                thread_results.push(result);
            }

            results.lock().unwrap().extend(thread_results);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions succeeded
    let results = results.lock().unwrap();
    assert_eq!(results.len(), num_threads * num_conversions_per_thread);

    for result in results.iter() {
        assert!(result.is_ok(), "Conversion failed: {:?}", result);
        let jpeg_data = result.as_ref().unwrap();
        assert!(!jpeg_data.is_empty());

        // Verify we can read the JPEG back
        let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
        let read_result = jpeg_reader.read(jpeg_data);
        assert!(read_result.is_ok());
        let image = read_result.unwrap();
        assert_eq!(image.width, 10);
        assert_eq!(image.height, 10);
    }
}

#[test]
fn test_parallel_format_matrix() {
    // Test all format pairs in parallel
    let formats = vec![
        ImageFormat::Png,
        ImageFormat::Jpeg,
        ImageFormat::Bmp,
        ImageFormat::Gif,
        ImageFormat::Tiff,
        ImageFormat::WebP,
    ];

    let png_data = create_test_png();
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // Test each format pair in parallel
    for from_format in &formats {
        for to_format in &formats {
            if from_format == to_format {
                continue; // Skip same format
            }

            let png_data = png_data.clone();
            let results = results.clone();
            let from_format = *from_format;
            let to_format = *to_format;

            let handle = thread::spawn(move || {
                // First convert PNG to source format
                let png_reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
                let from_writer = FormatRegistry::get_writer(from_format).unwrap();
                let converter = ImageConverter::new();
                let quality = QualitySettings::new(90);

                let source_data = match converter.convert(
                    &png_data,
                    png_reader.as_ref(),
                    from_writer.as_ref(),
                    &quality,
                ) {
                    Ok(data) => data,
                    Err(e) => {
                        results.lock().unwrap().push(Err(e));
                        return;
                    }
                };

                // Then convert from source format to target format
                let from_reader = FormatRegistry::get_reader(from_format).unwrap();
                let to_writer = FormatRegistry::get_writer(to_format).unwrap();

                let result = converter.convert(
                    &source_data,
                    from_reader.as_ref(),
                    to_writer.as_ref(),
                    &quality,
                );

                results.lock().unwrap().push(result);
            });

            handles.push(handle);
        }
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions succeeded
    let results = results.lock().unwrap();
    for result in results.iter() {
        assert!(result.is_ok(), "Format conversion failed: {:?}", result);
    }
}

#[test]
fn test_parallel_transparency_handling() {
    // Test transparency handling in parallel (RGBA → RGB conversions)
    let png_rgba_data = create_test_png_rgba();
    let num_threads = 4;

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let png_data = png_rgba_data.clone();
        let results = results.clone();

        let handle = thread::spawn(move || {
            // Convert RGBA PNG to JPEG (should handle transparency loss)
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

            results.lock().unwrap().push(result);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions succeeded (transparency should be handled)
    let results = results.lock().unwrap();
    assert_eq!(results.len(), num_threads);

    for result in results.iter() {
        assert!(
            result.is_ok(),
            "RGBA to JPEG conversion failed: {:?}",
            result
        );
        let jpeg_data = result.as_ref().unwrap();
        assert!(!jpeg_data.is_empty());

        // Verify we can read the JPEG back (should be RGB, not RGBA)
        let jpeg_reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
        let read_result = jpeg_reader.read(jpeg_data);
        assert!(read_result.is_ok());
        let image = read_result.unwrap();
        assert_eq!(image.width, 10);
        assert_eq!(image.height, 10);
        // JPEG doesn't support transparency, so should be RGB
        assert_ne!(image.color_type, img_core::formats::traits::ColorType::Rgba);
    }
}

#[test]
fn test_parallel_quality_settings() {
    // Test different quality settings in parallel
    let png_data = create_test_png();
    let quality_levels = vec![50, 75, 90, 95, 100];

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for quality_value in quality_levels {
        let png_data = png_data.clone();
        let results = results.clone();

        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(quality_value);

            let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

            results.lock().unwrap().push((quality_value, result));
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions succeeded with different quality settings
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 5);

    for (quality_value, result) in results.iter() {
        assert!(
            result.is_ok(),
            "Conversion with quality {} failed: {:?}",
            quality_value,
            result
        );
        let jpeg_data = result.as_ref().unwrap();
        assert!(!jpeg_data.is_empty());

        // Higher quality should generally produce larger files (not always, but usually)
        // We just verify the conversion succeeded
    }
}

#[test]
fn test_parallel_error_handling() {
    // Test error handling in parallel (corrupted files)
    let corrupted_data = vec![0u8; 100]; // Invalid image data
    let num_threads = 4;

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let corrupted_data = corrupted_data.clone();
        let results = results.clone();

        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let result =
                converter.convert(&corrupted_data, reader.as_ref(), writer.as_ref(), &quality);

            results.lock().unwrap().push(result);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions failed gracefully (no panics)
    let results = results.lock().unwrap();
    assert_eq!(results.len(), num_threads);

    for result in results.iter() {
        assert!(result.is_err(), "Corrupted data should fail conversion");
    }
}

#[test]
fn test_parallel_large_batch() {
    // Test processing a large batch of conversions in parallel
    let png_data = create_test_png();
    let batch_size = 50;
    let num_threads = 8;

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    let items_per_thread = batch_size / num_threads;
    let remainder = batch_size % num_threads;

    for thread_id in 0..num_threads {
        let png_data = png_data.clone();
        let results = results.clone();

        // Distribute remainder across first threads
        let items_for_this_thread = items_per_thread + if thread_id < remainder { 1 } else { 0 };

        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let mut thread_results = Vec::new();

            for _ in 0..items_for_this_thread {
                let result =
                    converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);
                thread_results.push(result);
            }

            results.lock().unwrap().extend(thread_results);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions succeeded
    let results = results.lock().unwrap();
    assert_eq!(results.len(), batch_size);

    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(
        success_count, batch_size,
        "All {} conversions should succeed",
        batch_size
    );
}

#[test]
fn test_parallel_mixed_formats() {
    // Test parallel conversion of different formats simultaneously
    let png_data = create_test_png();
    let jpeg_data = create_test_jpeg();

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // PNG to JPEG
    {
        let png_data = png_data.clone();
        let results = results.clone();
        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Jpeg).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

            results.lock().unwrap().push(("PNG→JPEG", result));
        });
        handles.push(handle);
    }

    // JPEG to PNG
    {
        let jpeg_data = jpeg_data.clone();
        let results = results.clone();
        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Jpeg).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Png).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let result = converter.convert(&jpeg_data, reader.as_ref(), writer.as_ref(), &quality);

            results.lock().unwrap().push(("JPEG→PNG", result));
        });
        handles.push(handle);
    }

    // PNG to BMP
    {
        let png_data = png_data.clone();
        let results = results.clone();
        let handle = thread::spawn(move || {
            let reader = FormatRegistry::get_reader(ImageFormat::Png).unwrap();
            let writer = FormatRegistry::get_writer(ImageFormat::Bmp).unwrap();
            let converter = ImageConverter::new();
            let quality = QualitySettings::new(90);

            let result = converter.convert(&png_data, reader.as_ref(), writer.as_ref(), &quality);

            results.lock().unwrap().push(("PNG→BMP", result));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all conversions succeeded
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 3);

    for (conversion_type, result) in results.iter() {
        assert!(
            result.is_ok(),
            "{} conversion failed: {:?}",
            conversion_type,
            result
        );
    }
}
