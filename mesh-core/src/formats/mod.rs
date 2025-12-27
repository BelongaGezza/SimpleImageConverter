// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

pub mod registry;
pub mod stl;
pub mod traits;

// Format implementations to be added in Sprint 3+
// pub mod obj;
// pub mod ply;

pub use registry::{FormatRegistry, MeshFormat};
pub use stl::StlFormat;
