// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageData, ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::{ConversionError, Result};
use image::{DynamicImage, GenericImageView, ImageFormat, Rgb, Rgba};

/// PNG format handler
pub struct PngFormat;

impl PngFormat {
    /// Create a new PNG format handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for PngFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for PngFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        let img = image::load_from_memory_with_format(data, ImageFormat::Png)
            .map_err(|e| ConversionError::ConversionFailed(format!(
                "Failed to read PNG image ({} bytes): {}",
                data.len(),
                e
            )))?;

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

impl ImageWriter for PngFormat {
    fn write(&self, image: &ImageData, _quality: &QualitySettings) -> Result<Vec<u8>> {
        // Validate image data before processing
        crate::validation::validate_image_data(image)?;

        let img_buffer = match image.color_type {
            crate::formats::traits::ColorType::Rgb => {
                let buffer: image::ImageBuffer<Rgb<u8>, Vec<u8>> = 
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| ConversionError::ConversionFailed("Invalid image dimensions".to_string()))?;
                DynamicImage::ImageRgb8(buffer)
            }
            crate::formats::traits::ColorType::Rgba => {
                let buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> = 
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| ConversionError::ConversionFailed("Invalid image dimensions".to_string()))?;
                DynamicImage::ImageRgba8(buffer)
            }
            crate::formats::traits::ColorType::Grayscale => {
                DynamicImage::ImageLuma8(
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| ConversionError::ConversionFailed("Invalid image dimensions".to_string()))?,
                )
            }
            crate::formats::traits::ColorType::GrayscaleAlpha => {
                DynamicImage::ImageLumaA8(
                    image::ImageBuffer::from_raw(image.width, image.height, image.data.clone())
                        .ok_or_else(|| ConversionError::ConversionFailed("Invalid image dimensions".to_string()))?,
                )
            }
        };

        let mut buffer = Vec::new();
        img_buffer
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
            .map_err(|e| ConversionError::ConversionFailed(format!(
                "Failed to write PNG image ({}x{} {:?}): {}",
                image.width, image.height, image.color_type, e
            )))?;

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
