// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ColorType, ImageData};

/// Convert image data to RGB format
///
/// JPEG and other formats that don't support transparency need RGB data.
/// This function handles conversion from all color types to RGB.
///
/// # Arguments
///
/// * `image` - The image data to convert
///
/// # Returns
///
/// A vector of RGB bytes (width * height * 3)
pub fn convert_to_rgb(image: &ImageData) -> Vec<u8> {
    match image.color_type {
        ColorType::Rgb => image.data.clone(),
        ColorType::Rgba => {
            // Convert RGBA to RGB by dropping alpha channel
            image
                .data
                .chunks(4)
                .flat_map(|chunk| &chunk[0..3])
                .copied()
                .collect()
        }
        ColorType::Grayscale => {
            // Convert grayscale to RGB (triple each value)
            image
                .data
                .iter()
                .flat_map(|&gray| [gray, gray, gray])
                .collect()
        }
        ColorType::GrayscaleAlpha => {
            // Convert grayscale+alpha to RGB (drop alpha, triple gray)
            image
                .data
                .chunks(2)
                .flat_map(|chunk| [chunk[0], chunk[0], chunk[0]])
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_rgb_to_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128],
            color_type: ColorType::Rgb,
        };
        let rgb = convert_to_rgb(&image);
        assert_eq!(rgb, image.data);
    }

    #[test]
    fn test_convert_rgba_to_rgb() {
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
        let rgb = convert_to_rgb(&image);
        assert_eq!(rgb.len(), 12); // 2x2x3
        assert_eq!(rgb[0..3], [255, 0, 0]); // First pixel RGB
        assert_eq!(rgb[3..6], [0, 255, 0]); // Second pixel RGB
    }

    #[test]
    fn test_convert_grayscale_to_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![128, 200, 50, 255],
            color_type: ColorType::Grayscale,
        };
        let rgb = convert_to_rgb(&image);
        assert_eq!(rgb.len(), 12); // 2x2x3
        assert_eq!(rgb[0..3], [128, 128, 128]); // First pixel
        assert_eq!(rgb[3..6], [200, 200, 200]); // Second pixel
    }

    #[test]
    fn test_convert_grayscale_alpha_to_rgb() {
        let image = ImageData {
            width: 2,
            height: 2,
            data: vec![128, 255, 200, 255, 50, 255, 100, 255],
            color_type: ColorType::GrayscaleAlpha,
        };
        let rgb = convert_to_rgb(&image);
        assert_eq!(rgb.len(), 12); // 2x2x3
        assert_eq!(rgb[0..3], [128, 128, 128]); // First pixel
        assert_eq!(rgb[3..6], [200, 200, 200]); // Second pixel
    }
}
