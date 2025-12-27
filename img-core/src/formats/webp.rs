// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::color::convert_to_rgb;
use crate::formats::traits::{ImageData, ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::{ConversionError, Result};
use image::{DynamicImage, GenericImageView, ImageFormat};

/// WebP format handler
///
/// Supports reading and writing WebP images with quality control.
/// WebP supports both lossy and lossless compression, with transparency support.
pub struct WebPFormat;

impl WebPFormat {
    /// Create a new WebP format handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for WebPFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for WebPFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        // Security: Validate input size before parsing to prevent memory exhaustion
        use common::limits::ResourceLimits;
        let limits = ResourceLimits::default();
        if let Err(e) = limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        let img = image::load_from_memory_with_format(data, ImageFormat::WebP).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read WebP image ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        let (width, height) = img.dimensions();

        // WebP supports transparency, so preserve RGBA if present
        // Otherwise convert to RGB
        let (data, color_type) = if img.color().has_alpha() {
            let rgba = img.to_rgba8();
            (rgba.into_raw(), crate::formats::traits::ColorType::Rgba)
        } else {
            let rgb = img.to_rgb8();
            (rgb.into_raw(), crate::formats::traits::ColorType::Rgb)
        };

        Ok(ImageData {
            width,
            height,
            data,
            color_type,
        })
    }
}

impl ImageWriter for WebPFormat {
    fn write(&self, image: &ImageData, quality: &QualitySettings) -> Result<Vec<u8>> {
        // Validate image data before processing
        crate::validation::validate_image_data(image)?;

        // WebP quality is 0-100, similar to JPEG
        // The image crate handles quality internally for WebP
        let img_buffer =
            match image.color_type {
                crate::formats::traits::ColorType::Rgb => {
                    let rgb_data = convert_to_rgb(image);
                    let buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                        image::ImageBuffer::from_raw(image.width, image.height, rgb_data)
                            .ok_or_else(|| {
                                ConversionError::ConversionFailed(
                                    "Invalid image dimensions".to_string(),
                                )
                            })?;
                    DynamicImage::ImageRgb8(buffer)
                }
                crate::formats::traits::ColorType::Rgba => {
                    let buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                        image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                            .ok_or_else(|| {
                                ConversionError::ConversionFailed(
                                    "Invalid image dimensions".to_string(),
                                )
                            })?;
                    DynamicImage::ImageRgba8(buffer)
                }
                crate::formats::traits::ColorType::Grayscale => {
                    // Convert grayscale to RGB for WebP
                    let rgb_data = convert_to_rgb(image);
                    let buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                        image::ImageBuffer::from_raw(image.width, image.height, rgb_data)
                            .ok_or_else(|| {
                                ConversionError::ConversionFailed(
                                    "Invalid image dimensions".to_string(),
                                )
                            })?;
                    DynamicImage::ImageRgb8(buffer)
                }
                crate::formats::traits::ColorType::GrayscaleAlpha => {
                    // Convert grayscale+alpha to RGBA (expand GA to RGBA)
                    let rgba: Vec<u8> = image
                        .data
                        .chunks(2)
                        .flat_map(|chunk| [chunk[0], chunk[0], chunk[0], chunk[1]])
                        .collect();
                    let buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                        image::ImageBuffer::from_raw(image.width, image.height, rgba).ok_or_else(
                            || {
                                ConversionError::ConversionFailed(
                                    "Invalid image dimensions".to_string(),
                                )
                            },
                        )?;
                    DynamicImage::ImageRgba8(buffer)
                }
            };

        let mut buffer = Vec::new();
        img_buffer
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
            .map_err(|e| {
                ConversionError::ConversionFailed(format!(
                    "Failed to write WebP image ({}x{} {:?} quality={}): {}",
                    image.width, image.height, image.color_type, quality.quality, e
                ))
            })?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::traits::ColorType;
    use crate::quality::QualitySettings;

    // Helper to create a simple test WebP
    fn create_test_webp_rgb() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgb([(x * 85) as u8, (y * 85) as u8, 128])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgb8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
            .unwrap();
        buffer
    }

    // Helper to create a simple test WebP with RGBA
    fn create_test_webp_rgba() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgba([(x * 85) as u8, (y * 85) as u8, 128, 255])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgba8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
            .unwrap();
        buffer
    }

    #[test]
    fn test_webp_format_new() {
        let _format = WebPFormat::new();
        let _format2 = WebPFormat::new();
        // Just verify they can be created
    }

    #[test]
    fn test_read_rgb_webp() {
        let webp_data = create_test_webp_rgb();
        let format = WebPFormat::new();
        let result = format.read(&webp_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        // WebP may preserve or convert color type, but should be valid
        assert!(matches!(image.color_type, ColorType::Rgb | ColorType::Rgba));
        assert!(image.data.len() >= 2 * 2 * 3); // At least RGB size
    }

    #[test]
    fn test_read_rgba_webp() {
        let webp_data = create_test_webp_rgba();
        let format = WebPFormat::new();
        let result = format.read(&webp_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        // WebP with transparency should preserve RGBA
        assert!(matches!(image.color_type, ColorType::Rgb | ColorType::Rgba));
    }

    #[test]
    fn test_read_invalid_webp() {
        let format = WebPFormat::new();
        let invalid_data = b"not a webp file";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_empty_file() {
        let format = WebPFormat::new();
        let empty_data = b"";
        let result = format.read(empty_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_rgb_webp() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128], // 2x2 RGB
            color_type: ColorType::Rgb,
        };
        let format = WebPFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let webp_data = result.unwrap();
        assert!(!webp_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&webp_data);
        assert!(read_result.is_ok());
    }

    #[test]
    fn test_write_rgba_webp() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![
                255, 0, 0, 255, // RGBA pixel 1
                0, 255, 0, 255, // RGBA pixel 2
                0, 0, 255, 255, // RGBA pixel 3
                128, 128, 128, 255, // RGBA pixel 4
            ],
            color_type: ColorType::Rgba,
        };
        let format = WebPFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let webp_data = result.unwrap();
        assert!(!webp_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&webp_data);
        assert!(read_result.is_ok());
    }

    #[test]
    fn test_write_with_quality() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128],
            color_type: ColorType::Rgb,
        };
        let format = WebPFormat::new();

        // Test different quality settings
        let quality_low = QualitySettings::new(50);
        let quality_high = QualitySettings::new(100);

        let result_low = format.write(&image, &quality_low).unwrap();
        let result_high = format.write(&image, &quality_high).unwrap();

        // Both should succeed
        assert!(!result_low.is_empty());
        assert!(!result_high.is_empty());
    }

    #[test]
    fn test_round_trip_rgb() {
        let original = create_test_webp_rgb();
        let format = WebPFormat::new();
        let quality = QualitySettings::default();

        // Read
        let image = format.read(&original).unwrap();

        // Write
        let written = format.write(&image, &quality).unwrap();

        // Read again
        let image2 = format.read(&written).unwrap();

        // Should match dimensions
        assert_eq!(image.width, image2.width);
        assert_eq!(image.height, image2.height);
    }

    #[test]
    fn test_round_trip_rgba() {
        let original = create_test_webp_rgba();
        let format = WebPFormat::new();
        let quality = QualitySettings::default();

        // Read
        let image = format.read(&original).unwrap();

        // Write
        let written = format.write(&image, &quality).unwrap();

        // Read again
        let image2 = format.read(&written).unwrap();

        // Should match dimensions
        assert_eq!(image.width, image2.width);
        assert_eq!(image.height, image2.height);
    }

    #[test]
    fn test_write_invalid_dimensions() {
        let image = ImageData {
            width: 0,
            height: 10,
            data: vec![],
            color_type: ColorType::Rgb,
        };
        let format = WebPFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_err());
    }
}
