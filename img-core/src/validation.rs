// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ColorType, ImageData};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;

/// Validate image data dimensions and data length using default limits
pub fn validate_image_data(image: &ImageData) -> Result<()> {
    validate_image_data_with_limits(image, &ResourceLimits::default())
}

/// Validate image data dimensions and data length with custom limits
///
/// Performs security validation:
/// - Checks dimensions are greater than zero
/// - Checks dimensions don't exceed resource limits
/// - Checks for integer overflow in size calculations
/// - Checks data length matches expected size
pub fn validate_image_data_with_limits(image: &ImageData, limits: &ResourceLimits) -> Result<()> {
    // Check dimensions are valid (non-zero)
    if image.width == 0 || image.height == 0 {
        return Err(ConversionError::InvalidInput(format!(
            "Image dimensions must be greater than zero: {}x{}",
            image.width, image.height
        )));
    }

    // Check dimensions against resource limits (security)
    limits.check_image_dimensions(image.width, image.height)?;

    // Check for overflow in size calculation
    let width = image.width as u64;
    let height = image.height as u64;
    let expected_len = match image.color_type {
        ColorType::Rgb => width
            .checked_mul(height)
            .and_then(|x| x.checked_mul(3))
            .ok_or_else(|| {
                ConversionError::InvalidInput("Image dimensions too large for RGB".to_string())
            })?,
        ColorType::Rgba => width
            .checked_mul(height)
            .and_then(|x| x.checked_mul(4))
            .ok_or_else(|| {
                ConversionError::InvalidInput("Image dimensions too large for RGBA".to_string())
            })?,
        ColorType::Grayscale => width.checked_mul(height).ok_or_else(|| {
            ConversionError::InvalidInput("Image dimensions too large for Grayscale".to_string())
        })?,
        ColorType::GrayscaleAlpha => width
            .checked_mul(height)
            .and_then(|x| x.checked_mul(2))
            .ok_or_else(|| {
                ConversionError::InvalidInput(
                    "Image dimensions too large for GrayscaleAlpha".to_string(),
                )
            })?,
    };

    // Check data length matches expected
    if image.data.len() != expected_len as usize {
        return Err(ConversionError::InvalidInput(format!(
            "Image data length mismatch: expected {} bytes for {}x{} {:?} image, got {} bytes",
            expected_len,
            image.width,
            image.height,
            image.color_type,
            image.data.len()
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

    // Security tests

    #[test]
    fn test_validate_dimension_exceeds_limit() {
        // Create restrictive limits
        let limits = ResourceLimits::builder().max_image_dimension(100).build();

        let image = ImageData {
            width: 200,
            height: 50,
            data: vec![0; 200 * 50 * 3],
            color_type: ColorType::Rgb,
        };

        let result = validate_image_data_with_limits(&image, &limits);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds limit"));
    }

    #[test]
    fn test_validate_at_dimension_limit() {
        let limits = ResourceLimits::builder().max_image_dimension(100).build();

        let image = ImageData {
            width: 100,
            height: 100,
            data: vec![0; 100 * 100 * 3],
            color_type: ColorType::Rgb,
        };

        // At limit should pass
        assert!(validate_image_data_with_limits(&image, &limits).is_ok());
    }

    #[test]
    fn test_validate_with_permissive_limits() {
        let limits = ResourceLimits::builder()
            .max_image_dimension(100_000)
            .max_decoded_image_bytes(100_000 * 100 * 3)
            .build();

        let image = ImageData {
            width: 100_000,
            height: 100,
            data: vec![], // Won't pass length check but tests dimension check
            color_type: ColorType::Rgb,
        };

        // With permissive limits, large dimensions should pass dimension check
        // but fail on length mismatch
        let result = validate_image_data_with_limits(&image, &limits);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("length mismatch"));
    }
}
