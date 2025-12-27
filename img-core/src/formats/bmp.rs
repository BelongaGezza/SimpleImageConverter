// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageData, ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::{ConversionError, Result};
use image::{DynamicImage, GenericImageView, ImageFormat, Rgb, Rgba};

/// BMP format handler
pub struct BmpFormat;

impl BmpFormat {
    /// Create a new BMP format handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for BmpFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for BmpFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        let img = image::load_from_memory_with_format(data, ImageFormat::Bmp).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read BMP image ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        let (width, height) = img.dimensions();
        let color_type = match img {
            DynamicImage::ImageLuma8(_) => crate::formats::traits::ColorType::Grayscale,
            DynamicImage::ImageLumaA8(_) => crate::formats::traits::ColorType::GrayscaleAlpha,
            DynamicImage::ImageRgb8(_) => crate::formats::traits::ColorType::Rgb,
            DynamicImage::ImageRgba8(_) => crate::formats::traits::ColorType::Rgba,
            _ => {
                // Convert to RGBA for other formats
                let rgba = img.to_rgba8();
                return Ok(ImageData {
                    width,
                    height,
                    data: rgba.into_raw(),
                    color_type: crate::formats::traits::ColorType::Rgba,
                });
            }
        };

        let data = match img {
            DynamicImage::ImageLuma8(img) => img.into_raw(),
            DynamicImage::ImageLumaA8(img) => img.into_raw(),
            DynamicImage::ImageRgb8(img) => img.into_raw(),
            DynamicImage::ImageRgba8(img) => img.into_raw(),
            _ => {
                let rgba = img.to_rgba8();
                rgba.into_raw()
            }
        };

        Ok(ImageData {
            width,
            height,
            data,
            color_type,
        })
    }
}

impl ImageWriter for BmpFormat {
    fn write(&self, image: &ImageData, _quality: &QualitySettings) -> Result<Vec<u8>> {
        // Validate image data before processing
        crate::validation::validate_image_data(image)?;

        let img_buffer = match image.color_type {
            crate::formats::traits::ColorType::Rgb => {
                let buffer: image::ImageBuffer<Rgb<u8>, Vec<u8>> =
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| {
                            ConversionError::ConversionFailed(
                                "Invalid image dimensions".to_string(),
                            )
                        })?;
                DynamicImage::ImageRgb8(buffer)
            }
            crate::formats::traits::ColorType::Rgba => {
                let buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> =
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| {
                            ConversionError::ConversionFailed(
                                "Invalid image dimensions".to_string(),
                            )
                        })?;
                DynamicImage::ImageRgba8(buffer)
            }
            crate::formats::traits::ColorType::Grayscale => DynamicImage::ImageLuma8(
                image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                    .ok_or_else(|| {
                        ConversionError::ConversionFailed("Invalid image dimensions".to_string())
                    })?,
            ),
            crate::formats::traits::ColorType::GrayscaleAlpha => DynamicImage::ImageLumaA8(
                image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                    .ok_or_else(|| {
                        ConversionError::ConversionFailed("Invalid image dimensions".to_string())
                    })?,
            ),
        };

        let mut buffer = Vec::new();
        img_buffer
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Bmp)
            .map_err(|e| {
                ConversionError::ConversionFailed(format!(
                    "Failed to write BMP image ({}x{} {:?}): {}",
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

    // Helper to create a simple test BMP
    fn create_test_bmp_rgb() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgb([(x * 85) as u8, (y * 85) as u8, 128])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgb8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Bmp)
            .unwrap();
        buffer
    }

    // Helper to create a simple test BMP with RGBA
    fn create_test_bmp_rgba() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgba([(x * 85) as u8, (y * 85) as u8, 128, 255])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgba8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Bmp)
            .unwrap();
        buffer
    }

    #[test]
    fn test_bmp_read_rgb() {
        let bmp_data = create_test_bmp_rgb();
        let format = BmpFormat::new();
        let result = format.read(&bmp_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        assert_eq!(image.color_type, ColorType::Rgb);
        assert_eq!(image.data.len(), 2 * 2 * 3); // 2x2 RGB
    }

    #[test]
    fn test_bmp_read_rgba() {
        let bmp_data = create_test_bmp_rgba();
        let format = BmpFormat::new();
        let result = format.read(&bmp_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        assert_eq!(image.color_type, ColorType::Rgba);
        assert_eq!(image.data.len(), 2 * 2 * 4); // 2x2 RGBA
    }

    #[test]
    fn test_bmp_write_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128], // 2x2 RGB
            color_type: ColorType::Rgb,
        };
        let format = BmpFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let bmp_data = result.unwrap();
        assert!(!bmp_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&bmp_data);
        assert!(read_result.is_ok());
    }

    #[test]
    fn test_bmp_write_rgba() {
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
        let format = BmpFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let bmp_data = result.unwrap();
        assert!(!bmp_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&bmp_data);
        assert!(read_result.is_ok());
        let read_image = read_result.unwrap();
        assert_eq!(read_image.color_type, ColorType::Rgba);
    }

    #[test]
    fn test_bmp_round_trip() {
        let original = create_test_bmp_rgb();
        let format = BmpFormat::new();
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
    fn test_bmp_read_invalid() {
        let format = BmpFormat::new();
        let invalid_data = b"not a bmp file";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_bmp_write_invalid_dimensions() {
        let image = ImageData {
            width: 0,
            height: 10,
            data: vec![],
            color_type: ColorType::Rgb,
        };
        let format = BmpFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_err());
    }
}
