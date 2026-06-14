// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

pub mod bmp;
mod decode;
pub mod gif;
pub mod info;
pub mod jpg;
pub mod png;
pub mod registry;
pub mod svg;
pub mod tiff;
pub mod traits;
pub mod webp;

pub use bmp::BmpFormat;
pub use gif::GifFormat;
pub use jpg::JpegFormat;
pub use png::PngFormat;
pub use registry::FormatRegistry;
pub use svg::SvgFormat;
pub use tiff::TiffFormat;
pub use webp::WebPFormat;
