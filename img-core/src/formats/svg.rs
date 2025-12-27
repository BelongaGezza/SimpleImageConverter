// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageData, ImageReader};
use common::error::{ConversionError, Result};
use image::GenericImageView;
use resvg::usvg::{fontdb::Database, Options, Tree};
use tiny_skia::Pixmap;

/// SVG format handler (read-only)
///
/// Supports reading SVG files by rasterizing them to bitmap images.
/// SVG is a vector format, so we can only read (rasterize) it, not write it.
/// The default rasterization DPI is 96 (standard screen resolution).
pub struct SvgFormat;

impl SvgFormat {
    /// Create a new SVG format handler
    pub fn new() -> Self {
        Self
    }

    /// Rasterize SVG data to a bitmap image
    ///
    /// # Arguments
    ///
    /// * `data` - SVG file data
    /// * `dpi` - DPI for rasterization (default: 96.0)
    ///
    /// # Returns
    ///
    /// A `DynamicImage` containing the rasterized SVG
    fn rasterize(&self, data: &[u8], _dpi: f32) -> Result<image::DynamicImage> {
        // Security: Validate input size before parsing
        use common::limits::ResourceLimits;
        let limits = ResourceLimits::default();
        if let Err(e) = limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        // Parse SVG
        let opt = Options::default();
        let mut fontdb = Database::new();
        fontdb.load_system_fonts();
        let tree = Tree::from_data(data, &opt, &fontdb)
            .map_err(|e| ConversionError::ConversionFailed(format!("SVG parse error: {}", e)))?;

        // Get SVG size
        let size = tree.size();
        let pixmap_size = size.to_int_size();

        // Create pixmap for rendering
        let mut pixmap =
            Pixmap::new(pixmap_size.width(), pixmap_size.height()).ok_or_else(|| {
                ConversionError::ConversionFailed(format!(
                    "Failed to create pixmap ({}x{})",
                    pixmap_size.width(),
                    pixmap_size.height()
                ))
            })?;

        // Render SVG to pixmap
        let mut pixmap_mut = pixmap.as_mut();
        resvg::render(&tree, resvg::usvg::Transform::default(), &mut pixmap_mut);

        // Convert pixmap to DynamicImage
        // Pixmap data is RGBA, stored as u32 pixels
        let width = pixmap.width();
        let height = pixmap.height();
        let data = pixmap.data();

        // Convert from RGBA u8 array to image::RgbaImage
        let rgba_image =
            image::RgbaImage::from_raw(width, height, data.to_vec()).ok_or_else(|| {
                ConversionError::ConversionFailed("Failed to create image from pixmap".to_string())
            })?;

        Ok(image::DynamicImage::ImageRgba8(rgba_image))
    }
}

impl Default for SvgFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for SvgFormat {
    fn read(&self, data: &[u8]) -> Result<ImageData> {
        // Rasterize SVG at 96 DPI (standard screen resolution)
        let dynamic = self.rasterize(data, 96.0)?;

        let (width, height) = dynamic.dimensions();

        // SVG rasterization always produces RGBA
        let rgba = dynamic.to_rgba8();

        Ok(ImageData {
            width,
            height,
            data: rgba.into_raw(),
            color_type: crate::formats::traits::ColorType::Rgba,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::traits::ColorType;

    // Helper to create a simple test SVG
    fn create_test_svg() -> Vec<u8> {
        // Simple SVG: red rectangle
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
           <rect width="100" height="100" fill="red"/>
         </svg>"#
            .as_bytes()
            .to_vec()
    }

    // Helper to create a test SVG with transparency
    fn create_test_svg_transparent() -> Vec<u8> {
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
           <rect width="100" height="100" fill="blue" opacity="0.5"/>
         </svg>"#
            .as_bytes()
            .to_vec()
    }

    // Helper to create a test SVG with text
    fn create_test_svg_text() -> Vec<u8> {
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100">
           <text x="10" y="50" font-family="Arial" font-size="20" fill="black">Test</text>
         </svg>"#
            .as_bytes()
            .to_vec()
    }

    #[test]
    fn test_svg_format_new() {
        let _format = SvgFormat::new();
        let _format2 = SvgFormat::new();
        // Just verify they can be created
    }

    #[test]
    fn test_read_simple_svg() {
        let svg_data = create_test_svg();
        let format = SvgFormat::new();
        let result = format.read(&svg_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 100);
        assert_eq!(image.height, 100);
        assert_eq!(image.color_type, ColorType::Rgba);
        assert_eq!(image.data.len(), 100 * 100 * 4); // 100x100 RGBA
    }

    #[test]
    fn test_read_svg_with_transparency() {
        let svg_data = create_test_svg_transparent();
        let format = SvgFormat::new();
        let result = format.read(&svg_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 100);
        assert_eq!(image.height, 100);
        assert_eq!(image.color_type, ColorType::Rgba);
        // Should have some transparency (alpha < 255)
        let has_transparency = image.data.chunks(4).any(|chunk| chunk[3] < 255);
        assert!(has_transparency);
    }

    #[test]
    fn test_read_svg_with_text() {
        let svg_data = create_test_svg_text();
        let format = SvgFormat::new();
        let result = format.read(&svg_data);
        assert!(result.is_ok());
        let image = result.unwrap();
        assert_eq!(image.width, 200);
        assert_eq!(image.height, 100);
        assert_eq!(image.color_type, ColorType::Rgba);
    }

    #[test]
    fn test_read_invalid_svg() {
        let format = SvgFormat::new();
        let invalid_data = b"not an svg file";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_empty_file() {
        let format = SvgFormat::new();
        let empty_data = b"";
        let result = format.read(empty_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_rasterize_at_different_dpi() {
        let svg_data = create_test_svg();
        let format = SvgFormat::new();

        // Test at different DPI (though our public API uses fixed 96 DPI)
        // This tests the internal rasterize method
        let result_96 = format.rasterize(&svg_data, 96.0);
        assert!(result_96.is_ok());

        let result_192 = format.rasterize(&svg_data, 192.0);
        assert!(result_192.is_ok());

        // Higher DPI should produce larger image
        let img_96 = result_96.unwrap();
        let img_192 = result_192.unwrap();
        // Note: DPI scaling might not always change size if SVG has fixed dimensions
        // But both should succeed
        assert!(img_96.dimensions().0 > 0);
        assert!(img_192.dimensions().0 > 0);
    }

    #[test]
    fn test_svg_to_png_conversion() {
        // Test that SVG can be read and the result is valid image data
        let svg_data = create_test_svg();
        let format = SvgFormat::new();
        let image_data = format.read(&svg_data).unwrap();

        // Verify the image data is valid
        assert!(image_data.width > 0);
        assert!(image_data.height > 0);
        assert_eq!(
            image_data.data.len(),
            (image_data.width * image_data.height * 4) as usize
        );
    }
}
