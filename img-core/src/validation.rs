// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ColorType, ImageData};
use common::error::{ConversionError, Result};

/// Validate image data dimensions and data length
pub fn validate_image_data(image: &ImageData) -> Result<()> {
    // Check dimensions are valid
    if image.width == 0 || image.height == 0 {
        return Err(ConversionError::InvalidInput(format!(
            "Image dimensions must be greater than zero: {}x{}",
            image.width, image.height
        )));
    }

    // Check for overflow in size calculation
    let width = image.width as u64;
    let height = image.height as u64;
    let expected_len = match image.color_type {
        ColorType::Rgb => {
            width
                .checked_mul(height)
                .and_then(|x| x.checked_mul(3))
                .ok_or_else(|| {
                    ConversionError::InvalidInput("Image dimensions too large for RGB".to_string())
                })?
        }
        ColorType::Rgba => {
            width
                .checked_mul(height)
                .and_then(|x| x.checked_mul(4))
                .ok_or_else(|| {
                    ConversionError::InvalidInput("Image dimensions too large for RGBA".to_string())
                })?
        }
        ColorType::Grayscale => {
            width
                .checked_mul(height)
                .ok_or_else(|| {
                    ConversionError::InvalidInput(
                        "Image dimensions too large for Grayscale".to_string(),
                    )
                })?
        }
        ColorType::GrayscaleAlpha => {
            width
                .checked_mul(height)
                .and_then(|x| x.checked_mul(2))
                .ok_or_else(|| {
                    ConversionError::InvalidInput(
                        "Image dimensions too large for GrayscaleAlpha".to_string(),
                    )
                })?
        }
    };

    // Check data length matches expected
    if image.data.len() != expected_len as usize {
        return Err(ConversionError::InvalidInput(format!(
            "Image data length mismatch: expected {} bytes for {}x{} {:?} image, got {} bytes",
            expected_len, image.width, image.height, image.color_type, image.data.len()
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_rgb_image() {
        let image = ImageData {
            width: 10,
            height: 10,
            data: vec![0; 10 * 10 * 3],
            color_type: ColorType::Rgb,
        };
        assert!(validate_image_data(&image).is_ok());
    }

    #[test]
    fn test_validate_rgba_image() {
        let image = ImageData {
            width: 10,
            height: 10,
            data: vec![0; 10 * 10 * 4],
            color_type: ColorType::Rgba,
        };
        assert!(validate_image_data(&image).is_ok());
    }

    #[test]
    fn test_validate_zero_width() {
        let image = ImageData {
            width: 0,
            height: 10,
            data: vec![],
            color_type: ColorType::Rgb,
        };
        assert!(validate_image_data(&image).is_err());
    }

    #[test]
    fn test_validate_zero_height() {
        let image = ImageData {
            width: 10,
            height: 0,
            data: vec![],
            color_type: ColorType::Rgb,
        };
        assert!(validate_image_data(&image).is_err());
    }

    #[test]
    fn test_validate_length_mismatch() {
        let image = ImageData {
            width: 10,
            height: 10,
            data: vec![0; 100], // Should be 300 for RGB
            color_type: ColorType::Rgb,
        };
        assert!(validate_image_data(&image).is_err());
    }
}

