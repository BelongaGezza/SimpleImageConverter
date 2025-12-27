// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Core traits and types for image format handling.
//!
//! This module defines the fundamental abstractions for reading and writing
//! image formats in a format-agnostic way.

use crate::quality::QualitySettings;
use common::error::Result;

/// Trait for reading image formats.
///
/// Implement this trait to add support for reading a new image format.
/// Each format handler should validate input data and return appropriate
/// errors for malformed or unsupported files.
///
/// # Security
///
/// Implementations must:
/// - Validate file size against resource limits before parsing
/// - Handle malformed input gracefully without panicking
/// - Log security-relevant errors for auditing
///
/// # Example
///
/// ```ignore
/// use img_core::formats::traits::{ImageReader, ImageData};
/// use common::error::Result;
///
/// struct MyFormatReader;
///
/// impl ImageReader for MyFormatReader {
///     fn read(&self, data: &[u8]) -> Result<ImageData> {
///         // Validate and parse the format
///         // Return ImageData with decoded pixel data
///         todo!()
///     }
/// }
/// ```
pub trait ImageReader {
    /// Read an image from raw bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw file data to decode
    ///
    /// # Returns
    ///
    /// The decoded image data, or an error if the format is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The data is empty or too large
    /// - The format is invalid or unsupported
    /// - The image dimensions exceed limits
    fn read(&self, data: &[u8]) -> Result<ImageData>;
}

/// Trait for writing image formats.
///
/// Implement this trait to add support for writing a new image format.
/// Writers should respect quality settings where applicable.
///
/// # Example
///
/// ```ignore
/// use img_core::formats::traits::{ImageWriter, ImageData};
/// use img_core::quality::QualitySettings;
/// use common::error::Result;
///
/// struct MyFormatWriter;
///
/// impl ImageWriter for MyFormatWriter {
///     fn write(&self, image: &ImageData, quality: &QualitySettings) -> Result<Vec<u8>> {
///         // Encode the image to the target format
///         // Use quality.quality for lossy formats (1-100)
///         todo!()
///     }
/// }
/// ```
pub trait ImageWriter {
    /// Write an image to raw bytes.
    ///
    /// # Arguments
    ///
    /// * `image` - The image data to encode
    /// * `quality` - Quality settings (1-100 for lossy formats)
    ///
    /// # Returns
    ///
    /// The encoded image data, or an error if encoding fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The image data is invalid
    /// - The dimensions are zero or too large
    /// - The format doesn't support the color type
    fn write(&self, image: &ImageData, quality: &QualitySettings) -> Result<Vec<u8>>;
}

/// Decoded image data with pixel buffer and metadata.
///
/// This struct holds the raw pixel data along with dimensions and color type.
/// The pixel data is stored in row-major order (left-to-right, top-to-bottom).
///
/// # Pixel Data Layout
///
/// The `data` field contains pixels in the following formats:
///
/// | ColorType        | Bytes per pixel | Layout          |
/// |------------------|-----------------|-----------------|
/// | Rgb              | 3               | R, G, B         |
/// | Rgba             | 4               | R, G, B, A      |
/// | Grayscale        | 1               | L               |
/// | GrayscaleAlpha   | 2               | L, A            |
///
/// # Example
///
/// ```
/// use img_core::formats::traits::{ImageData, ColorType};
///
/// // Create a 2x2 red RGB image
/// let image = ImageData {
///     width: 2,
///     height: 2,
///     data: vec![
///         255, 0, 0,  255, 0, 0,  // Row 1: red, red
///         255, 0, 0,  255, 0, 0,  // Row 2: red, red
///     ],
///     color_type: ColorType::Rgb,
/// };
///
/// assert_eq!(image.data.len(), 2 * 2 * 3); // width * height * bytes_per_pixel
/// ```
#[derive(Debug, Clone)]
pub struct ImageData {
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
    /// Raw pixel data (see ColorType for layout)
    pub data: Vec<u8>,
    /// The color type determining pixel data layout
    pub color_type: ColorType,
}

/// Color type enumeration defining pixel data layout.
///
/// This enum specifies how pixel data is organized in the `ImageData.data` buffer.
/// Each variant determines the number of bytes per pixel and their meaning.
///
/// # Bytes Per Pixel
///
/// - `Rgb`: 3 bytes (Red, Green, Blue)
/// - `Rgba`: 4 bytes (Red, Green, Blue, Alpha)
/// - `Grayscale`: 1 byte (Luminance)
/// - `GrayscaleAlpha`: 2 bytes (Luminance, Alpha)
///
/// # Format Support
///
/// | Format | Rgb | Rgba | Grayscale | GrayscaleAlpha |
/// |--------|-----|------|-----------|----------------|
/// | PNG    | ✅  | ✅   | ✅        | ✅             |
/// | JPEG   | ✅  | ❌   | ✅        | ❌             |
/// | WebP   | ✅  | ✅   | ❌*       | ❌*            |
/// | TIFF   | ✅  | ✅   | ✅        | ✅             |
/// | BMP    | ✅  | ✅   | ❌        | ❌             |
/// | GIF    | ✅  | ✅   | ❌        | ❌             |
///
/// *Grayscale is converted to RGB when writing WebP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    /// RGB color (3 bytes per pixel: Red, Green, Blue)
    Rgb,
    /// RGBA color with alpha channel (4 bytes per pixel)
    Rgba,
    /// Grayscale (1 byte per pixel: Luminance)
    Grayscale,
    /// Grayscale with alpha channel (2 bytes per pixel)
    GrayscaleAlpha,
}
