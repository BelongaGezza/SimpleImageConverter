// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

pub mod convert;
pub mod formats;
pub mod mesh;

pub use convert::{ConversionOptions, MeshConverter};
pub use formats::{FormatRegistry, MeshFormat, StlFormat};
pub use mesh::{
    transform_coordinates, parse_coordinate_system, recalculate_normals, validate_mesh,
    CoordinateSystem, Face, Mesh, Normal, Vertex,
};
