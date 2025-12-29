// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

pub mod normal;
pub mod transform;
pub mod validate;

pub use normal::recalculate_normals;
pub use transform::{parse_coordinate_system, transform_coordinates, CoordinateSystem};
pub use validate::validate_mesh;

/// 3D mesh data structure
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub normals: Vec<Normal>,
}

/// Vertex position
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Face (triangle) indices
#[derive(Debug, Clone)]
pub struct Face {
    pub indices: [usize; 3],
}

/// Normal vector
#[derive(Debug, Clone, Copy)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mesh {
    /// Create a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            normals: Vec::new(),
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
