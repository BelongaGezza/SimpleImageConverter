// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::color::convert_to_rgb;
use crate::formats::traits::{ImageData, ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::{ConversionError, Result};
use image::{DynamicImage, GenericImageView, ImageFormat};

/// JPEG format handler
pub struct JpegFormat;

impl JpegFormat {
    /// Create a new JPEG format handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for JpegFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for JpegFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        // Security: Validate input size before parsing to prevent memory exhaustion
        use common::limits::ResourceLimits;
        let limits = ResourceLimits::default();
        if let Err(e) = limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        let img = image::load_from_memory_with_format(data, ImageFormat::Jpeg).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read JPEG image ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        let (width, height) = img.dimensions();

        // JPEG doesn't support transparency, so convert to RGB
        let rgb_img = img.to_rgb8();

        Ok(ImageData {
            width,
            height,
            data: rgb_img.into_raw(),
            color_type: crate::formats::traits::ColorType::Rgb,
        })
    }
}

impl ImageWriter for JpegFormat {
    fn write(&self, image: &ImageData, quality: &QualitySettings) -> Result<Vec<u8>> {
        // Validate image data before processing
        crate::validation::validate_image_data(image)?;

        // Convert to RGB if needed (JPEG doesn't support transparency)
        let rgb_data = convert_to_rgb(image);

        let img_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(image.width, image.height, rgb_data).ok_or_else(|| {
                ConversionError::ConversionFailed("Invalid image dimensions".to_string())
            })?;

        let img = DynamicImage::ImageRgb8(img_buffer);

        let mut buffer = Vec::new();
        let mut encoder =
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality.quality);
        let rgb_bytes = img.as_bytes();
        encoder
            .encode(
                rgb_bytes,
                image.width,
                image.height,
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|e| {
                ConversionError::ConversionFailed(format!(
                    "Failed to encode JPEG image ({}x{} quality={}): {}",
                    image.width, image.height, quality.quality, e
                ))
            })?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::traits::ColorType;

    // Helper to create a simple test JPEG
    fn create_test_jpeg() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgb([(x * 85) as u8, (y * 85) as u8, 128])
        });
        let mut buffer = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 90);
        encoder
            .encode(img.as_raw(), 2, 2, image::ExtendedColorType::Rgb8)
            .unwrap();
        buffer
    }

    #[test]
    fn test_jpeg_read() {
        let jpeg_data = create_test_jpeg();
        let format = JpegFormat::new();
        let result = format.read(&jpeg_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        assert_eq!(image.color_type, ColorType::Rgb);
        assert_eq!(image.data.len(), 2 * 2 * 3); // 2x2 RGB
    }

    #[test]
    fn test_jpeg_write_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128], // 2x2 RGB
            color_type: ColorType::Rgb,
        };
        let format = JpegFormat::new();
        let quality = QualitySettings::new(90);
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let jpeg_data = result.unwrap();
        assert!(!jpeg_data.is_empty());
    }

    #[test]
    fn test_jpeg_write_rgba_conversion() {
        // Test RGBA to RGB conversion
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
        let format = JpegFormat::new();
        let quality = QualitySettings::new(90);
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let jpeg_data = result.unwrap();
        assert!(!jpeg_data.is_empty());

        // Verify we can read it back as RGB
        let read_result = format.read(&jpeg_data);
        assert!(read_result.is_ok());
        let read_image = read_result.unwrap();
        assert_eq!(read_image.color_type, ColorType::Rgb);
    }

    #[test]
    fn test_jpeg_write_grayscale_conversion() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![128, 200, 50, 255], // 2x2 grayscale
            color_type: ColorType::Grayscale,
        };
        let format = JpegFormat::new();
        let quality = QualitySettings::new(90);
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
    }

    #[test]
    fn test_jpeg_read_invalid() {
        let format = JpegFormat::new();
        let invalid_data = b"not a jpeg file";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_jpeg_write_quality() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128],
            color_type: ColorType::Rgb,
        };
        let format = JpegFormat::new();

        // Test different quality settings
        let quality_low = QualitySettings::new(50);
        let quality_high = QualitySettings::new(100);

        let result_low = format.write(&image, &quality_low).unwrap();
        let result_high = format.write(&image, &quality_high).unwrap();

        // Higher quality should generally produce larger files (though not always guaranteed)
        // At minimum, both should succeed
        assert!(!result_low.is_empty());
        assert!(!result_high.is_empty());
    }
}
