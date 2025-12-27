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
}
