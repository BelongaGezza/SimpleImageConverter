// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageReader, ImageWriter};
use crate::formats::{
    BmpFormat, GifFormat, JpegFormat, PngFormat, SvgFormat, TiffFormat, WebPFormat,
};
use common::error::{ConversionError, Result};
use common::io::get_extension;
use common::limits::ResourceLimits;
use std::path::Path;

/// Format registry for detecting and getting format handlers
///
/// This registry provides format detection and handler retrieval for image formats.
/// It supports format detection by file extension and provides reader/writer instances.
///
/// # Example
///
/// ```
/// use img_core::{FormatRegistry, ImageFormat};
/// use std::path::Path;
///
/// // Detect format from extension
/// let format = FormatRegistry::detect_format("png")?;
/// assert_eq!(format, ImageFormat::Png);
///
/// // Detect format from path
/// let path = Path::new("photo.jpg");
/// let format = FormatRegistry::detect_from_path(path)?;
/// assert_eq!(format, ImageFormat::Jpeg);
///
/// // Get format handlers
/// let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
/// let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct FormatRegistry;

impl FormatRegistry {
    /// Detect format from file extension
    ///
    /// # Arguments
    ///
    /// * `extension` - File extension (case-insensitive, without leading dot)
    ///
    /// # Returns
    ///
    /// The detected `ImageFormat`, or an error if the format is unsupported.
    ///
    /// # Example
    ///
    /// ```
    /// use img_core::{FormatRegistry, ImageFormat};
    ///
    /// let format = FormatRegistry::detect_format("png")?;
    /// assert_eq!(format, ImageFormat::Png);
    ///
    /// let format = FormatRegistry::detect_format("JPEG")?; // Case insensitive
    /// assert_eq!(format, ImageFormat::Jpeg);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn detect_format(extension: &str) -> Result<ImageFormat> {
        match extension.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "bmp" => Ok(ImageFormat::Bmp),
            "gif" => Ok(ImageFormat::Gif),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            "webp" => Ok(ImageFormat::WebP),
            "svg" => Ok(ImageFormat::Svg),
            _ => Err(ConversionError::UnsupportedFormat(format!(
                "Unsupported format: {}",
                extension
            ))),
        }
    }

    /// Detect format from file path
    ///
    /// Extracts the file extension from the path and detects the format.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to analyze
    ///
    /// # Returns
    ///
    /// The detected `ImageFormat`, or an error if:
    /// - The file has no extension
    /// - The format is unsupported
    ///
    /// # Example
    ///
    /// ```
    /// use img_core::{FormatRegistry, ImageFormat};
    /// use std::path::Path;
    ///
    /// let path = Path::new("photo.png");
    /// let format = FormatRegistry::detect_from_path(path)?;
    /// assert_eq!(format, ImageFormat::Png);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn detect_from_path(path: &Path) -> Result<ImageFormat> {
        let ext = get_extension(path)
            .ok_or_else(|| ConversionError::InvalidInput("File has no extension".to_string()))?;
        Self::detect_format(&ext)
    }

    /// Get reader for a format
    ///
    /// Returns a boxed `ImageReader` trait object for the specified format.
    ///
    /// # Arguments
    ///
    /// * `format` - The image format to get a reader for
    ///
    /// # Returns
    ///
    /// A boxed reader instance, or an error if the format is not yet implemented.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use img_core::{FormatRegistry, ImageFormat};
    ///
    /// // Get a PNG reader
    /// let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
    ///
    /// // Read PNG data from file
    /// let png_bytes = std::fs::read("image.png")?;
    /// let image_data = reader.read(&png_bytes)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_reader(format: ImageFormat) -> Result<Box<dyn ImageReader>> {
        Self::get_reader_with_limits(format, ResourceLimits::default())
    }

    /// Get reader for a format with custom resource limits
    ///
    /// Returns a boxed `ImageReader` trait object configured with resource limits
    /// for security validation.
    ///
    /// # Arguments
    ///
    /// * `format` - The image format to get a reader for
    /// * `limits` - Resource limits for validation
    ///
    /// # Returns
    ///
    /// A boxed reader instance with configured limits.
    pub fn get_reader_with_limits(
        format: ImageFormat,
        limits: ResourceLimits,
    ) -> Result<Box<dyn ImageReader>> {
        match format {
            ImageFormat::Png => Ok(Box::new(PngFormat::with_limits(limits))),
            ImageFormat::Jpeg => Ok(Box::new(JpegFormat::with_limits(limits))),
            ImageFormat::Bmp => Ok(Box::new(BmpFormat::with_limits(limits))),
            ImageFormat::Gif => Ok(Box::new(GifFormat::with_limits(limits))),
            ImageFormat::Tiff => Ok(Box::new(TiffFormat::with_limits(limits))),
            ImageFormat::WebP => Ok(Box::new(WebPFormat::with_limits(limits))),
            ImageFormat::Svg => Ok(Box::new(SvgFormat::with_limits(limits))),
        }
    }

    /// Get writer for a format
    ///
    /// Returns a boxed `ImageWriter` trait object for the specified format.
    ///
    /// # Arguments
    ///
    /// * `format` - The image format to get a writer for
    ///
    /// # Returns
    ///
    /// A boxed writer instance, or an error if the format is not yet implemented.
    ///
    /// # Example
    ///
    /// ```
    /// use img_core::{FormatRegistry, ImageFormat, QualitySettings, ImageData, ColorType};
    ///
    /// let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;
    /// let image_data = ImageData {
    ///     width: 10,
    ///     height: 10,
    ///     data: vec![0u8; 300], // 10x10 RGB
    ///     color_type: ColorType::Rgb,
    /// };
    /// let jpeg_data = writer.write(&image_data, &QualitySettings::new(90))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_writer(format: ImageFormat) -> Result<Box<dyn ImageWriter>> {
        match format {
            ImageFormat::Png => Ok(Box::new(PngFormat::new())),
            ImageFormat::Jpeg => Ok(Box::new(JpegFormat::new())),
            ImageFormat::Bmp => Ok(Box::new(BmpFormat::new())),
            ImageFormat::Gif => Ok(Box::new(GifFormat::new())),
            ImageFormat::Tiff => Ok(Box::new(TiffFormat::new())),
            ImageFormat::WebP => Ok(Box::new(WebPFormat::new())),
            // SVG is a vector format and cannot be written as raster
            ImageFormat::Svg => Err(ConversionError::UnsupportedFormat(
                "SVG is a vector format and cannot be written as raster".to_string(),
            )),
        }
    }

    /// Detect format from file magic bytes
    ///
    /// This provides more reliable format detection than extension-based detection,
    /// as it examines the actual file content.
    ///
    /// # Arguments
    ///
    /// * `data` - The file data (at least first 8 bytes needed)
    ///
    /// # Returns
    ///
    /// The detected `ImageFormat`, or `None` if the format is not recognized.
    ///
    /// # Example
    ///
    /// ```
    /// use img_core::{FormatRegistry, ImageFormat};
    ///
    /// // PNG magic bytes
    /// let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    /// assert_eq!(FormatRegistry::detect_from_bytes(&png_data), Some(ImageFormat::Png));
    ///
    /// // JPEG magic bytes
    /// let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x00, 0x00, 0x00];
    /// assert_eq!(FormatRegistry::detect_from_bytes(&jpeg_data), Some(ImageFormat::Jpeg));
    /// ```
    pub fn detect_from_bytes(data: &[u8]) -> Option<ImageFormat> {
        if data.len() < 4 {
            return None;
        }

        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if data.len() >= 8 && data[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
            return Some(ImageFormat::Png);
        }

        // JPEG: FF D8 FF
        if data[0..3] == [0xFF, 0xD8, 0xFF] {
            return Some(ImageFormat::Jpeg);
        }

        // BMP: 42 4D ("BM")
        if data[0..2] == [0x42, 0x4D] {
            return Some(ImageFormat::Bmp);
        }

        // GIF: 47 49 46 38 ("GIF8")
        if data[0..4] == [0x47, 0x49, 0x46, 0x38] {
            return Some(ImageFormat::Gif);
        }

        // TIFF: 49 49 2A 00 (little-endian) or 4D 4D 00 2A (big-endian)
        if data.len() >= 4
            && (data[0..4] == [0x49, 0x49, 0x2A, 0x00] || data[0..4] == [0x4D, 0x4D, 0x00, 0x2A])
        {
            return Some(ImageFormat::Tiff);
        }

        // WebP: 52 49 46 46 ?? ?? ?? ?? 57 45 42 50 (RIFF...WEBP)
        if data.len() >= 12
            && data[0..4] == [0x52, 0x49, 0x46, 0x46]
            && data[8..12] == [0x57, 0x45, 0x42, 0x50]
        {
            return Some(ImageFormat::WebP);
        }

        // SVG: Check for XML declaration or <svg tag
        if data.len() >= 5 {
            let start = String::from_utf8_lossy(&data[0..data.len().min(100)]);
            if start.trim_start().starts_with("<?xml") || start.trim_start().starts_with("<svg") {
                return Some(ImageFormat::Svg);
            }
        }

        None
    }

    /// Verify that file content matches the expected format
    ///
    /// This performs a two-stage verification: checks if the magic bytes match
    /// the expected format based on file extension.
    ///
    /// # Arguments
    ///
    /// * `data` - The file data
    /// * `expected` - The expected format (usually from file extension)
    ///
    /// # Returns
    ///
    /// `Ok(())` if the format matches or cannot be determined from bytes,
    /// `Err` if there's a clear mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// use img_core::{FormatRegistry, ImageFormat};
    ///
    /// // JPEG data but expecting PNG - should error
    /// let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x00, 0x00, 0x00];
    /// let result = FormatRegistry::verify_format(&jpeg_data, ImageFormat::Png);
    /// assert!(result.is_err());
    ///
    /// // PNG data expecting PNG - should pass
    /// let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    /// let result = FormatRegistry::verify_format(&png_data, ImageFormat::Png);
    /// assert!(result.is_ok());
    /// ```
    pub fn verify_format(data: &[u8], expected: ImageFormat) -> Result<()> {
        if let Some(detected) = Self::detect_from_bytes(data) {
            if detected != expected {
                return Err(ConversionError::InvalidFormat(format!(
                    "Format mismatch: file extension suggests {:?} but content is {:?}",
                    expected, detected
                )));
            }
        }
        // If we can't detect the format, we allow it (could be a valid format we don't recognize)
        Ok(())
    }

    /// Detect format using two-stage detection (extension + magic bytes)
    ///
    /// This is the recommended method for format detection as it provides
    /// defense-in-depth against format spoofing.
    ///
    /// # Arguments
    ///
    /// * `path` - File path (for extension detection)
    /// * `data` - File data (for magic byte detection)
    ///
    /// # Returns
    ///
    /// The detected format, or an error if:
    /// - Extension and magic bytes don't match
    /// - Format cannot be determined
    ///
    /// # Example
    ///
    /// ```no_run
    /// use img_core::{FormatRegistry, ImageFormat};
    /// use std::path::Path;
    ///
    /// let path = Path::new("photo.png");
    /// let data = std::fs::read(path)?;
    /// let format = FormatRegistry::detect_two_stage(path, &data)?;
    /// assert_eq!(format, ImageFormat::Png);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn detect_two_stage(path: &Path, data: &[u8]) -> Result<ImageFormat> {
        // Stage 1: Detect from extension
        let extension_format = Self::detect_from_path(path)?;

        // Stage 2: Verify with magic bytes
        if let Some(magic_format) = Self::detect_from_bytes(data) {
            if magic_format != extension_format {
                return Err(ConversionError::InvalidFormat(format!(
                    "Format mismatch: extension suggests {:?} but magic bytes indicate {:?}",
                    extension_format, magic_format
                )));
            }
        }

        Ok(extension_format)
    }
}

/// Image format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
    Gif,
    Tiff,
    WebP,
    Svg,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_png() {
        assert_eq!(
            FormatRegistry::detect_format("png").unwrap(),
            ImageFormat::Png
        );
        assert_eq!(
            FormatRegistry::detect_format("PNG").unwrap(),
            ImageFormat::Png
        );
        assert_eq!(
            FormatRegistry::detect_format("Png").unwrap(),
            ImageFormat::Png
        );
    }

    #[test]
    fn test_detect_format_jpeg() {
        assert_eq!(
            FormatRegistry::detect_format("jpg").unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            FormatRegistry::detect_format("jpeg").unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            FormatRegistry::detect_format("JPG").unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            FormatRegistry::detect_format("JPEG").unwrap(),
            ImageFormat::Jpeg
        );
    }

    #[test]
    fn test_detect_format_bmp() {
        assert_eq!(
            FormatRegistry::detect_format("bmp").unwrap(),
            ImageFormat::Bmp
        );
    }

    #[test]
    fn test_detect_format_gif() {
        assert_eq!(
            FormatRegistry::detect_format("gif").unwrap(),
            ImageFormat::Gif
        );
    }

    #[test]
    fn test_detect_format_invalid() {
        assert!(FormatRegistry::detect_format("xyz").is_err());
        assert!(FormatRegistry::detect_format("").is_err());
    }

    #[test]
    fn test_detect_from_path() {
        let path = Path::new("test.png");
        assert_eq!(
            FormatRegistry::detect_from_path(path).unwrap(),
            ImageFormat::Png
        );

        let path = Path::new("photo.JPEG");
        assert_eq!(
            FormatRegistry::detect_from_path(path).unwrap(),
            ImageFormat::Jpeg
        );
    }

    #[test]
    fn test_detect_from_path_no_extension() {
        let path = Path::new("test");
        assert!(FormatRegistry::detect_from_path(path).is_err());
    }

    #[test]
    fn test_get_reader_with_limits_png() {
        let limits = ResourceLimits::builder().max_image_dimension(4096).build();
        let reader = FormatRegistry::get_reader_with_limits(ImageFormat::Png, limits);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_png() {
        let reader = FormatRegistry::get_reader(ImageFormat::Png);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_jpeg() {
        let reader = FormatRegistry::get_reader(ImageFormat::Jpeg);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_bmp() {
        let reader = FormatRegistry::get_reader(ImageFormat::Bmp);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_gif() {
        let reader = FormatRegistry::get_reader(ImageFormat::Gif);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_writer_png() {
        let writer = FormatRegistry::get_writer(ImageFormat::Png);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_jpeg() {
        let writer = FormatRegistry::get_writer(ImageFormat::Jpeg);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_bmp() {
        let writer = FormatRegistry::get_writer(ImageFormat::Bmp);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_gif() {
        let writer = FormatRegistry::get_writer(ImageFormat::Gif);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_detect_format_tiff() {
        assert_eq!(
            FormatRegistry::detect_format("tiff").unwrap(),
            ImageFormat::Tiff
        );
        assert_eq!(
            FormatRegistry::detect_format("tif").unwrap(),
            ImageFormat::Tiff
        );
        assert_eq!(
            FormatRegistry::detect_format("TIFF").unwrap(),
            ImageFormat::Tiff
        );
    }

    #[test]
    fn test_detect_format_webp() {
        assert_eq!(
            FormatRegistry::detect_format("webp").unwrap(),
            ImageFormat::WebP
        );
        assert_eq!(
            FormatRegistry::detect_format("WEBP").unwrap(),
            ImageFormat::WebP
        );
    }

    #[test]
    fn test_detect_format_svg() {
        assert_eq!(
            FormatRegistry::detect_format("svg").unwrap(),
            ImageFormat::Svg
        );
        assert_eq!(
            FormatRegistry::detect_format("SVG").unwrap(),
            ImageFormat::Svg
        );
    }

    #[test]
    fn test_get_reader_tiff() {
        let reader = FormatRegistry::get_reader(ImageFormat::Tiff);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_webp() {
        let reader = FormatRegistry::get_reader(ImageFormat::WebP);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_svg() {
        let reader = FormatRegistry::get_reader(ImageFormat::Svg);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_writer_tiff() {
        let writer = FormatRegistry::get_writer(ImageFormat::Tiff);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_webp() {
        let writer = FormatRegistry::get_writer(ImageFormat::WebP);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_svg_fails() {
        // SVG is read-only, so get_writer should fail
        let writer = FormatRegistry::get_writer(ImageFormat::Svg);
        assert!(writer.is_err());
        match writer {
            Err(e) => {
                let err_msg = e.to_string();
                assert!(err_msg.contains("vector format"));
            }
            Ok(_) => panic!("Expected error for SVG writer"),
        }
    }

    #[test]
    fn test_detect_tiff_magic_bytes() {
        // TIFF little-endian
        let tiff_data_le = [0x49, 0x49, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&tiff_data_le),
            Some(ImageFormat::Tiff)
        );

        // TIFF big-endian
        let tiff_data_be = [0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&tiff_data_be),
            Some(ImageFormat::Tiff)
        );
    }

    #[test]
    fn test_detect_webp_magic_bytes() {
        // WebP: RIFF...WEBP
        let webp_data = [
            0x52, 0x49, 0x46, 0x46, // RIFF
            0x00, 0x00, 0x00, 0x00, // size (dummy)
            0x57, 0x45, 0x42, 0x50, // WEBP
        ];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&webp_data),
            Some(ImageFormat::WebP)
        );
    }

    #[test]
    fn test_detect_svg_magic_bytes() {
        // SVG with XML declaration
        let svg_data_xml = b"<?xml version=\"1.0\"?><svg></svg>";
        assert_eq!(
            FormatRegistry::detect_from_bytes(svg_data_xml),
            Some(ImageFormat::Svg)
        );

        // SVG without XML declaration
        let svg_data_direct = b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";
        assert_eq!(
            FormatRegistry::detect_from_bytes(svg_data_direct),
            Some(ImageFormat::Svg)
        );
    }

    // Magic byte detection tests

    #[test]
    fn test_detect_png_magic_bytes() {
        let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&png_data),
            Some(ImageFormat::Png)
        );
    }

    #[test]
    fn test_detect_jpeg_magic_bytes() {
        let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&jpeg_data),
            Some(ImageFormat::Jpeg)
        );
    }

    #[test]
    fn test_detect_bmp_magic_bytes() {
        let bmp_data = [0x42, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&bmp_data),
            Some(ImageFormat::Bmp)
        );
    }

    #[test]
    fn test_detect_gif_magic_bytes() {
        let gif_data = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x00, 0x00];
        assert_eq!(
            FormatRegistry::detect_from_bytes(&gif_data),
            Some(ImageFormat::Gif)
        );
    }

    #[test]
    fn test_detect_unknown_magic_bytes() {
        let unknown_data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        assert_eq!(FormatRegistry::detect_from_bytes(&unknown_data), None);
    }

    #[test]
    fn test_detect_too_short_data() {
        let short_data = [0x89, 0x50, 0x4E];
        assert_eq!(FormatRegistry::detect_from_bytes(&short_data), None);
    }

    #[test]
    fn test_verify_format_match() {
        let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(FormatRegistry::verify_format(&png_data, ImageFormat::Png).is_ok());
    }

    #[test]
    fn test_verify_format_mismatch() {
        // JPEG data but expecting PNG
        let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let result = FormatRegistry::verify_format(&jpeg_data, ImageFormat::Png);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("mismatch"));
    }

    #[test]
    fn test_verify_format_unknown_allows() {
        // Unknown format should be allowed (we can't disprove it)
        let unknown_data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        assert!(FormatRegistry::verify_format(&unknown_data, ImageFormat::Png).is_ok());
    }
}
