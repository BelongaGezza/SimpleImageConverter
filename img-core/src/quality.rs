// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

/// Quality settings for image conversion
/// 
/// Controls the quality and compression level for image formats that support it.
/// Different formats interpret these values differently:
/// - JPEG: `quality` (0-100) controls compression, higher = better quality
/// - PNG: `compression` (0-9) controls compression level, higher = smaller file
/// 
/// # Example
/// 
/// ```
/// use img_core::QualitySettings;
/// 
/// // High quality JPEG
/// let high_quality = QualitySettings::new(95);
/// 
/// // Default quality (90)
/// let default = QualitySettings::default();
/// 
/// // Low quality for smaller file size
/// let low_quality = QualitySettings::new(70);
/// ```
#[derive(Debug, Clone)]
pub struct QualitySettings {
    /// Quality level (0-100)
    /// 
    /// For JPEG: 0 = lowest quality, 100 = highest quality
    /// Values above 100 are clamped to 100.
    pub quality: u8,
    /// Compression level (0-9, format-dependent)
    /// 
    /// For PNG: 0 = no compression, 9 = maximum compression
    pub compression: u8,
}

impl Default for QualitySettings {
    fn default() -> Self {
        Self {
            quality: 90,
            compression: 6,
        }
    }
}

impl QualitySettings {
    /// Create new quality settings with specified quality level
    /// 
    /// # Arguments
    /// 
    /// * `quality` - Quality level (0-100). Values above 100 are clamped to 100.
    /// 
    /// # Example
    /// 
    /// ```
    /// use img_core::QualitySettings;
    /// 
    /// let settings = QualitySettings::new(85);
    /// assert_eq!(settings.quality, 85);
    /// 
    /// // Values above 100 are clamped
    /// let clamped = QualitySettings::new(150);
    /// assert_eq!(clamped.quality, 100);
    /// ```
    pub fn new(quality: u8) -> Self {
        Self {
            quality: quality.min(100),
            compression: 6,
        }
    }
}
