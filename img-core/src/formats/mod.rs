// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

pub mod traits;
pub mod png;
pub mod jpg;
pub mod registry;

// Format implementations will be added in Sprint 2+
// pub mod bmp;
// pub mod gif;

pub use png::PngFormat;
pub use jpg::JpegFormat;
pub use registry::FormatRegistry;
