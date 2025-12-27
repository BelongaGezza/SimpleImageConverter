// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Format capability information
//!
//! This module provides format capability queries to determine what features
//! each format supports (transparency, animation, lossy compression, etc.)

use crate::formats::registry::ImageFormat;

/// Format capability information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatInfo {
    /// Format name
    pub name: &'static str,
    /// Supported file extensions
    pub extensions: &'static [&'static str],
    /// Whether the format supports transparency (alpha channel)
    pub supports_transparency: bool,
    /// Whether the format supports animation
    pub supports_animation: bool,
    /// Whether the format uses lossy compression
    pub is_lossy: bool,
    /// Whether the format supports multiple pages/frames
    pub supports_multipage: bool,
}

impl FormatInfo {
    /// Get format info for a given format
    pub fn for_format(format: ImageFormat) -> Self {
        match format {
            ImageFormat::Png => FormatInfo {
                name: "PNG",
                extensions: &["png"],
                supports_transparency: true,
                supports_animation: false, // APNG not yet supported
                is_lossy: false,
                supports_multipage: false,
            },
            ImageFormat::Jpeg => FormatInfo {
                name: "JPEG",
                extensions: &["jpg", "jpeg"],
                supports_transparency: false,
                supports_animation: false,
                is_lossy: true,
                supports_multipage: false,
            },
            ImageFormat::Bmp => FormatInfo {
                name: "BMP",
                extensions: &["bmp"],
                supports_transparency: true, // BMP can have alpha channel
                supports_animation: false,
                is_lossy: false,
                supports_multipage: false,
            },
            ImageFormat::Gif => FormatInfo {
                name: "GIF",
                extensions: &["gif"],
                supports_transparency: true,
                supports_animation: true, // Animated GIFs supported (first frame only currently)
                is_lossy: true, // GIF uses lossy compression (palette-based)
                supports_multipage: false,
            },
        }
    }
}

/// Format capability queries
pub struct FormatCapabilities;

impl FormatCapabilities {
    /// Check if a format supports transparency
    pub fn supports_transparency(format: ImageFormat) -> bool {
        FormatInfo::for_format(format).supports_transparency
    }

    /// Check if a format supports animation
    pub fn supports_animation(format: ImageFormat) -> bool {
        FormatInfo::for_format(format).supports_animation
    }

    /// Check if a format uses lossy compression
    pub fn is_lossy(format: ImageFormat) -> bool {
        FormatInfo::for_format(format).is_lossy
    }

    /// Check if a format supports multiple pages/frames
    pub fn supports_multipage(format: ImageFormat) -> bool {
        FormatInfo::for_format(format).supports_multipage
    }

    /// Get all format information
    pub fn info(format: ImageFormat) -> FormatInfo {
        FormatInfo::for_format(format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_capabilities() {
        let info = FormatInfo::for_format(ImageFormat::Png);
        assert_eq!(info.name, "PNG");
        assert!(info.supports_transparency);
        assert!(!info.supports_animation);
        assert!(!info.is_lossy);
    }

    #[test]
    fn test_jpeg_capabilities() {
        let info = FormatInfo::for_format(ImageFormat::Jpeg);
        assert_eq!(info.name, "JPEG");
        assert!(!info.supports_transparency);
        assert!(info.is_lossy);
    }

    #[test]
    fn test_gif_capabilities() {
        let info = FormatInfo::for_format(ImageFormat::Gif);
        assert_eq!(info.name, "GIF");
        assert!(info.supports_transparency);
        assert!(info.supports_animation);
        assert!(info.is_lossy);
    }

    #[test]
    fn test_capability_queries() {
        assert!(FormatCapabilities::supports_transparency(ImageFormat::Png));
        assert!(!FormatCapabilities::supports_transparency(ImageFormat::Jpeg));
        assert!(FormatCapabilities::is_lossy(ImageFormat::Jpeg));
        assert!(!FormatCapabilities::is_lossy(ImageFormat::Png));
    }
}

