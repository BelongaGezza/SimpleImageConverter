// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageReader, ImageWriter};
use crate::formats::{BmpFormat, GifFormat, JpegFormat, PngFormat};
use common::error::{ConversionError, Result};
use common::io::get_extension;
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
        match format {
            ImageFormat::Png => Ok(Box::new(PngFormat::new())),
            ImageFormat::Jpeg => Ok(Box::new(JpegFormat::new())),
            ImageFormat::Bmp => Ok(Box::new(BmpFormat::new())),
            ImageFormat::Gif => Ok(Box::new(GifFormat::new())),
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
}

/// Image format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
    Gif,
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
