// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageData, ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::{ConversionError, Result};
use image::{DynamicImage, GenericImageView, ImageFormat, Rgb, Rgba};

/// GIF format handler
///
/// Note: Animated GIFs are supported by extracting the first frame only.
/// This is a limitation for Sprint 2; full animation support may be added in Phase 2.
pub struct GifFormat;

impl GifFormat {
    /// Create a new GIF format handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for GifFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for GifFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        // Security: Validate input size before parsing to prevent memory exhaustion
        use common::limits::ResourceLimits;
        let limits = ResourceLimits::default();
        if let Err(e) = limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        // The image crate automatically extracts the first frame from animated GIFs
        let img = image::load_from_memory_with_format(data, ImageFormat::Gif).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read GIF image ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        let (width, height) = img.dimensions();

        // GIF supports transparency via palette, which is converted to RGBA by the image crate
        // We need to check if the image has transparency and preserve it
        let color_type = match img {
            DynamicImage::ImageLuma8(_) => crate::formats::traits::ColorType::Grayscale,
            DynamicImage::ImageLumaA8(_) => crate::formats::traits::ColorType::GrayscaleAlpha,
            DynamicImage::ImageRgb8(_) => crate::formats::traits::ColorType::Rgb,
            DynamicImage::ImageRgba8(_) => crate::formats::traits::ColorType::Rgba,
            _ => {
                // Convert to RGBA to preserve transparency
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
                // Convert to RGBA to preserve transparency
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

impl ImageWriter for GifFormat {
    fn write(&self, image: &ImageData, _quality: &QualitySettings) -> Result<Vec<u8>> {
        // Validate image data before processing
        crate::validation::validate_image_data(image)?;

        // GIF format supports RGB and RGBA (with transparency)
        // For other color types, we convert appropriately
        let img_buffer =
            match image.color_type {
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
                crate::formats::traits::ColorType::Grayscale => {
                    // Convert grayscale to RGB for GIF
                    let rgb_data: Vec<u8> = image
                        .data
                        .iter()
                        .flat_map(|&gray| [gray, gray, gray])
                        .collect();
                    let buffer: image::ImageBuffer<Rgb<u8>, Vec<u8>> =
                        image::ImageBuffer::from_raw(image.width, image.height, rgb_data)
                            .ok_or_else(|| {
                                ConversionError::ConversionFailed(
                                    "Invalid image dimensions".to_string(),
                                )
                            })?;
                    DynamicImage::ImageRgb8(buffer)
                }
                crate::formats::traits::ColorType::GrayscaleAlpha => {
                    // Convert grayscale+alpha to RGBA for GIF
                    let rgba_data: Vec<u8> = image
                        .data
                        .chunks(2)
                        .flat_map(|chunk| [chunk[0], chunk[0], chunk[0], chunk[1]])
                        .collect();
                    let buffer: image::ImageBuffer<Rgba<u8>, Vec<u8>> =
                        image::ImageBuffer::from_raw(image.width, image.height, rgba_data)
                            .ok_or_else(|| {
                                ConversionError::ConversionFailed(
                                    "Invalid image dimensions".to_string(),
                                )
                            })?;
                    DynamicImage::ImageRgba8(buffer)
                }
            };

        let mut buffer = Vec::new();
        img_buffer
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Gif)
            .map_err(|e| {
                ConversionError::ConversionFailed(format!(
                    "Failed to write GIF image ({}x{} {:?}): {}",
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

    // Helper to create a simple test GIF
    fn create_test_gif_rgb() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgb([(x * 85) as u8, (y * 85) as u8, 128])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgb8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Gif)
            .unwrap();
        buffer
    }

    // Helper to create a simple test GIF with RGBA (transparency)
    fn create_test_gif_rgba() -> Vec<u8> {
        let img = image::ImageBuffer::from_fn(2, 2, |x, y| {
            image::Rgba([
                (x * 85) as u8,
                (y * 85) as u8,
                128,
                if x == 0 && y == 0 { 0 } else { 255 },
            ])
        });
        let mut buffer = Vec::new();
        DynamicImage::ImageRgba8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Gif)
            .unwrap();
        buffer
    }

    #[test]
    fn test_gif_read_rgb() {
        let gif_data = create_test_gif_rgb();
        let format = GifFormat::new();
        let result = format.read(&gif_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        // GIF may be read as RGB or RGBA depending on the image crate's handling
        assert!(image.color_type == ColorType::Rgb || image.color_type == ColorType::Rgba);
        assert!(image.data.len() >= 2 * 2 * 3); // At least 2x2 RGB
    }

    #[test]
    fn test_gif_read_rgba() {
        let gif_data = create_test_gif_rgba();
        let format = GifFormat::new();
        let result = format.read(&gif_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 2);
        assert_eq!(image.height, 2);
        // GIF with transparency should be read as RGBA
        assert_eq!(image.color_type, ColorType::Rgba);
        assert_eq!(image.data.len(), 2 * 2 * 4); // 2x2 RGBA
    }

    #[test]
    fn test_gif_write_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128], // 2x2 RGB
            color_type: ColorType::Rgb,
        };
        let format = GifFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let gif_data = result.unwrap();
        assert!(!gif_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&gif_data);
        assert!(read_result.is_ok());
    }

    #[test]
    fn test_gif_write_rgba() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![
                255, 0, 0, 255, // RGBA pixel 1
                0, 255, 0, 255, // RGBA pixel 2
                0, 0, 255, 255, // RGBA pixel 3
                128, 128, 128, 0, // RGBA pixel 4 (transparent)
            ],
            color_type: ColorType::Rgba,
        };
        let format = GifFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let gif_data = result.unwrap();
        assert!(!gif_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&gif_data);
        assert!(read_result.is_ok());
        let read_image = read_result.unwrap();
        // Transparency should be preserved
        assert_eq!(read_image.color_type, ColorType::Rgba);
    }

    #[test]
    fn test_gif_write_grayscale() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![128, 200, 50, 255], // 2x2 grayscale
            color_type: ColorType::Grayscale,
        };
        let format = GifFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_ok());
        let gif_data = result.unwrap();
        assert!(!gif_data.is_empty());
    }

    #[test]
    fn test_gif_round_trip() {
        let original = create_test_gif_rgb();
        let format = GifFormat::new();
        let quality = QualitySettings::default();

        // Read
        let image = format.read(&original).unwrap();

        // Write
        let written = format.write(&image, &quality).unwrap();

        // Read again
        let image2 = format.read(&written).unwrap();

        // Should match dimensions and color type
        assert_eq!(image.width, image2.width);
        assert_eq!(image.height, image2.height);
        assert_eq!(image.color_type, image2.color_type);
    }

    #[test]
    fn test_gif_read_invalid() {
        let format = GifFormat::new();
        let invalid_data = b"not a gif file";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_gif_write_invalid_dimensions() {
        let image = ImageData {
            width: 0,
            height: 10,
            data: vec![],
            color_type: ColorType::Rgb,
        };
        let format = GifFormat::new();
        let quality = QualitySettings::default();
        let result = format.write(&image, &quality);
        assert!(result.is_err());
    }
}
