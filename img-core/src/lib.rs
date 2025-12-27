// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! # img-core
//!
//! A Rust library for converting between 2D image formats with security-first design.
//!
//! ## Supported Formats
//!
//! | Format | Read | Write | Notes |
//! |--------|------|-------|-------|
//! | PNG    | ✅   | ✅    | Lossless, transparency support |
//! | JPEG   | ✅   | ✅    | Lossy compression, quality 1-100 |
//! | BMP    | ✅   | ✅    | Uncompressed bitmap |
//! | GIF    | ✅   | ✅    | 256 color palette |
//! | TIFF   | ✅   | ✅    | Multi-page support (first page only) |
//! | WebP   | ✅   | ✅    | Modern format, lossy/lossless |
//! | SVG    | ✅   | ❌    | Vector to raster conversion only |
//!
//! ## Quick Start
//!
//! ```no_run
//! use img_core::{ImageConverter, FormatRegistry, ImageFormat, QualitySettings};
//!
//! // Detect and convert an image
//! let input_data = std::fs::read("photo.png")?;
//!
//! // Get appropriate readers/writers
//! let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
//! let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;
//!
//! // Convert with quality setting
//! let converter = ImageConverter::new();
//! let quality = QualitySettings::new(85);
//! let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref(), &quality)?;
//!
//! std::fs::write("photo.jpg", output_data)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Format Detection
//!
//! The library provides secure two-stage format detection that verifies both
//! file extension and magic bytes to prevent format spoofing attacks:
//!
//! ```no_run
//! use img_core::FormatRegistry;
//! use std::path::Path;
//!
//! let path = Path::new("image.png");
//! let data = std::fs::read(path)?;
//!
//! // Detects format from extension AND verifies magic bytes match
//! let format = FormatRegistry::detect_two_stage(path, &data)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Security Features
//!
//! - **Resource limits**: Configurable file size and dimension limits
//! - **Format verification**: Two-stage detection prevents format spoofing
//! - **Input validation**: All inputs are validated before processing
//! - **No unsafe code**: Pure safe Rust implementation

pub mod color;
pub mod convert;
pub mod formats;
pub mod quality;
pub mod validation;

pub use convert::ImageConverter;
pub use formats::traits::{ColorType, ImageData, ImageReader, ImageWriter};
pub use formats::{info::FormatCapabilities, registry::ImageFormat, FormatRegistry};
pub use quality::QualitySettings;
