// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{ImageReader, ImageWriter};
use crate::quality::QualitySettings;
use common::error::Result;

/// Main image converter orchestrator
///
/// This struct provides a high-level interface for converting images between formats.
/// It coordinates the reading and writing operations using format-specific handlers.
///
/// # Example
///
/// ```no_run
/// use img_core::{ImageConverter, FormatRegistry, ImageFormat, QualitySettings};
///
/// // Get format handlers
/// let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
/// let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;
///
/// // Create converter
/// let converter = ImageConverter::new();
/// let quality = QualitySettings::new(90);
///
/// // Convert image
/// let input_data = std::fs::read("input.png")?;
/// let output_data = converter.convert(
///     &input_data,
///     reader.as_ref(),
///     writer.as_ref(),
///     &quality
/// )?;
///
/// std::fs::write("output.jpg", output_data)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct ImageConverter;

impl ImageConverter {
    /// Create a new image converter
    ///
    /// # Example
    ///
    /// ```
    /// use img_core::ImageConverter;
    ///
    /// let converter = ImageConverter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Convert image from one format to another
    ///
    /// This method reads an image using the provided reader, then writes it
    /// using the provided writer with the specified quality settings.
    ///
    /// # Arguments
    ///
    /// * `input_data` - The raw image data to convert
    /// * `reader` - Format-specific reader for the input format
    /// * `writer` - Format-specific writer for the output format
    /// * `quality` - Quality settings for the conversion
    ///
    /// # Returns
    ///
    /// The converted image data as a byte vector, or an error if conversion fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The input data cannot be read by the reader
    /// - The image data is invalid
    /// - The image cannot be written by the writer
    ///
    /// # Example
    ///
    /// ```no_run
    /// use img_core::{ImageConverter, FormatRegistry, ImageFormat, QualitySettings};
    ///
    /// let converter = ImageConverter::new();
    /// let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
    /// let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;
    /// let quality = QualitySettings::new(90);
    ///
    /// let input = std::fs::read("photo.png")?;
    /// let output = converter.convert(&input, reader.as_ref(), writer.as_ref(), &quality)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn convert(
        &self,
        input_data: &[u8],
        reader: &dyn ImageReader,
        writer: &dyn ImageWriter,
        quality: &QualitySettings,
    ) -> Result<Vec<u8>> {
        let image = reader.read(input_data)?;
        writer.write(&image, quality)
    }
}

impl Default for ImageConverter {
    fn default() -> Self {
        Self::new()
    }
}
