// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::quality::QualitySettings;
use common::error::Result;

/// Trait for reading image formats
pub trait ImageReader {
    /// Read an image from bytes
    fn read(&self, data: &[u8]) -> Result<ImageData>;
}

/// Trait for writing image formats
pub trait ImageWriter {
    /// Write an image to bytes
    fn write(&self, image: &ImageData, quality: &QualitySettings) -> Result<Vec<u8>>;
}

/// Image data structure
#[derive(Debug, Clone)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub color_type: ColorType,
}

/// Color type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Rgb,
    Rgba,
    Grayscale,
    GrayscaleAlpha,
}
