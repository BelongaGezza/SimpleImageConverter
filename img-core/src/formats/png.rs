// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageData, ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use image::{DynamicImage, ImageFormat, Rgb, Rgba};

/// PNG format handler
pub struct PngFormat {
    limits: ResourceLimits,
}

impl PngFormat {
    /// Create a new PNG format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new PNG format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }
}

impl Default for PngFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for PngFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        let img = crate::formats::decode::read_dynamic_image(data, ImageFormat::Png, &self.limits)
            .inspect_err(|e| common::security::log_security_error(e, None))?;
        crate::formats::decode::dynamic_to_image_data(img, &self.limits)
    }
}

impl ImageWriter for PngFormat {
    fn write(&self, image: &ImageData, _quality: &QualitySettings) -> Result<Vec<u8>> {
        // Validate image data before processing
        crate::validation::validate_image_data(image)?;

        let img_buffer = match image.color_type {
            crate::formats::traits::ColorType::Rgb => {
                let buffer: image::ImageBuffer<Rgb<u8>, Vec<u8>> =
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| {
                            ConversionError::ConversionFailed(format!(
                                "Cannot create {}x{} RGB image: data size {} doesn't match expected {}",
                                image.width, image.height, image.data.len(),
                                image.width as usize * image.height as usize * 3
                            ))
                        })?;
                DynamicImage::ImageRgb8(buffer)
            }
            crate::formats::traits::ColorType::Rgba => {
                let buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> =
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| {
                            ConversionError::ConversionFailed(format!(
                                "Cannot create {}x{} RGBA image: data size {} doesn't match expected {}",
                                image.width, image.height, image.data.len(),
                                image.width as usize * image.height as usize * 4
                            ))
                        })?;
                DynamicImage::ImageRgba8(buffer)
            }
            crate::formats::traits::ColorType::Grayscale => DynamicImage::ImageLuma8(
                image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                    .ok_or_else(|| {
                        ConversionError::ConversionFailed(format!(
                            "Cannot create {}x{} Grayscale image: data size {} doesn't match expected {}",
                            image.width, image.height, image.data.len(),
                            image.width as usize * image.height as usize
                        ))
                    })?,
            ),
            crate::formats::traits::ColorType::GrayscaleAlpha => DynamicImage::ImageLumaA8(
                image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                    .ok_or_else(|| {
                        ConversionError::ConversionFailed(format!(
                            "Cannot create {}x{} GrayscaleAlpha image: data size {} doesn't match expected {}",
                            image.width, image.height, image.data.len(),
                            image.width as usize * image.height as usize * 2
                        ))
                    })?,
            ),
        };

        let mut buffer = Vec::new();
        img_buffer
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
            .map_err(|e| {
                ConversionError::ConversionFailed(format!(
                    "Failed to write PNG image ({}x{} {:?}): {}",
                    image.width, image.height, image.color_type, e
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

    // Helper to create a simple test PNG
    fn create_test_png_rgb() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgb([(x * 85) as u8, (y * 85) as u8, 128])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgb8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
            .unwrap();
        buffer
    }

    #[test]
    fn test_png_read_rgb() {
        let png_data = create_test_png_rgb();
        let format = PngFormat::new();
        let result = format.read(&png_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        assert_eq!(image.color_type, ColorType::Rgb);
        assert_eq!(image.data.len(), 2 * 2 * 3); // 2x2 RGB
    }

    #[test]
    fn test_png_write_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128], // 2x2 RGB
            color_type: ColorType::Rgb,
        };
        let format = PngFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&png_data);
        assert!(read_result.is_ok());
    }

    #[test]
    fn test_png_round_trip() {
        let original = create_test_png_rgb();
        let format = PngFormat::new();
        let quality = QualitySettings::default();

        // Read
        let image = format.read(&original).unwrap();

        // Write
        let written = format.write(&image, &quality).unwrap();

        // Read again
        let image2 = format.read(&written).unwrap();

        // Should match
        assert_eq!(image.width, image2.width);
        assert_eq!(image.height, image2.height);
        assert_eq!(image.color_type, image2.color_type);
        assert_eq!(image.data.len(), image2.data.len());
    }

    #[test]
    fn test_png_read_invalid() {
        let format = PngFormat::new();
        let invalid_data = b"not a png file";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_png_read_rejects_decoded_image_limit() {
        let png_data = create_test_png_rgb();
        let limits = ResourceLimits::builder().max_decoded_image_bytes(8).build();
        let format = PngFormat::with_limits(limits);
        let result = format.read(&png_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_resource_limit());
    }

    #[test]
    fn test_png_write_invalid_dimensions() {
        let image = ImageData {
            width: 0,
            height: 10,
            data: vec![],
            color_type: ColorType::Rgb,
        };
        let format = PngFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_err());
    }
}
