// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::mesh::Mesh;
use common::error::{ConversionError, Result};

/// Epsilon value for floating point comparisons
const EPSILON: f32 = 1e-6;

/// Coordinate system types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSystem {
    /// Y-up coordinate system (used in OpenGL, glTF)
    YUp,
    /// Z-up coordinate system (used in CAD, STL)
    ZUp,
}

/// Transform a mesh from one coordinate system to another
///
/// # Arguments
///
/// * `mesh` - The mesh to transform
/// * `from` - Source coordinate system
/// * `to` - Target coordinate system
///
/// # Returns
///
/// Transformed mesh, or an error if transformation fails.
///
/// # Example
///
/// ```
/// use mesh_core::mesh::{transform_coordinates, CoordinateSystem, Mesh};
///
/// let mut mesh = Mesh::new();
/// // ... add vertices ...
/// let transformed = transform_coordinates(mesh, CoordinateSystem::ZUp, CoordinateSystem::YUp).unwrap();
/// ```
pub fn transform_coordinates(
    mut mesh: Mesh,
    from: CoordinateSystem,
    to: CoordinateSystem,
) -> Result<Mesh> {
    // No transformation needed if systems are the same
    if from == to {
        return Ok(mesh);
    }

    // Transform vertices: swap Y and Z coordinates
    // Y-up: (x, y, z) -> (x, z, -y)
    // Z-up: (x, y, z) -> (x, -z, y)
    for vertex in &mut mesh.vertices {
        let (x, y, z) = (vertex.x, vertex.y, vertex.z);
        let (new_x, new_y, new_z) = transform_coord(x, y, z, from, to);
        vertex.x = new_x;
        vertex.y = new_y;
        vertex.z = new_z;
    }

    // Transform normals: same transformation as vertices
    for normal in &mut mesh.normals {
        let (x, y, z) = (normal.x, normal.y, normal.z);
        let (new_x, new_y, new_z) = transform_coord(x, y, z, from, to);
        normal.x = new_x;
        normal.y = new_y;
        normal.z = new_z;

        // Normalize the transformed normal
        let length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
        if length > EPSILON {
            normal.x /= length;
            normal.y /= length;
            normal.z /= length;
        }
    }

    Ok(mesh)
}

/// Transform a single coordinate point between coordinate systems
///
/// This is a helper function that performs the coordinate transformation
/// logic for a single point, reducing code duplication.
///
/// # Arguments
///
/// * `x, y, z` - Input coordinates
/// * `from` - Source coordinate system
/// * `to` - Target coordinate system
///
/// # Returns
///
/// Transformed coordinates as (x, y, z) tuple
fn transform_coord(
    x: f32,
    y: f32,
    z: f32,
    from: CoordinateSystem,
    to: CoordinateSystem,
) -> (f32, f32, f32) {
    match (from, to) {
        (CoordinateSystem::ZUp, CoordinateSystem::YUp) => {
            // Z-up to Y-up: (x, y, z) -> (x, z, -y)
            (x, z, -y)
        }
        (CoordinateSystem::YUp, CoordinateSystem::ZUp) => {
            // Y-up to Z-up: (x, y, z) -> (x, -z, y)
            (x, -z, y)
        }
        _ => {
            // No transformation needed
            (x, y, z)
        }
    }
}

/// Parse coordinate system from string
///
/// Accepts: "y-up", "y_up", "YUp", "z-up", "z_up", "ZUp"
pub fn parse_coordinate_system(s: &str) -> Result<CoordinateSystem> {
    match s.to_lowercase().replace("_", "-").as_str() {
        "y-up" | "yup" => Ok(CoordinateSystem::YUp),
        "z-up" | "zup" => Ok(CoordinateSystem::ZUp),
        _ => Err(ConversionError::InvalidInput(format!(
            "Invalid coordinate system: '{}'. Use 'y-up' or 'z-up'",
            s
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::Vertex;

    #[test]
    fn test_transform_zup_to_yup() {
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });

        let transformed =
            transform_coordinates(mesh, CoordinateSystem::ZUp, CoordinateSystem::YUp).unwrap();

        assert_eq!(transformed.vertices[0].x, 1.0);
        assert_eq!(transformed.vertices[0].y, 3.0);
        assert_eq!(transformed.vertices[0].z, -2.0);
    }

    #[test]
    fn test_transform_yup_to_zup() {
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });

        let transformed =
            transform_coordinates(mesh, CoordinateSystem::YUp, CoordinateSystem::ZUp).unwrap();

        assert_eq!(transformed.vertices[0].x, 1.0);
        assert_eq!(transformed.vertices[0].y, -3.0);
        assert_eq!(transformed.vertices[0].z, 2.0);
    }

    #[test]
    fn test_transform_no_op() {
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });

        let transformed =
            transform_coordinates(mesh.clone(), CoordinateSystem::ZUp, CoordinateSystem::ZUp)
                .unwrap();

        assert_eq!(transformed.vertices[0].x, mesh.vertices[0].x);
        assert_eq!(transformed.vertices[0].y, mesh.vertices[0].y);
        assert_eq!(transformed.vertices[0].z, mesh.vertices[0].z);
    }

    #[test]
    fn test_parse_coordinate_system() {
        assert_eq!(
            parse_coordinate_system("y-up").unwrap(),
            CoordinateSystem::YUp
        );
        assert_eq!(
            parse_coordinate_system("Y-Up").unwrap(),
            CoordinateSystem::YUp
        );
        assert_eq!(
            parse_coordinate_system("y_up").unwrap(),
            CoordinateSystem::YUp
        );
        assert_eq!(
            parse_coordinate_system("z-up").unwrap(),
            CoordinateSystem::ZUp
        );
        assert_eq!(
            parse_coordinate_system("Z-Up").unwrap(),
            CoordinateSystem::ZUp
        );
        assert_eq!(
            parse_coordinate_system("z_up").unwrap(),
            CoordinateSystem::ZUp
        );

        assert!(parse_coordinate_system("invalid").is_err());
    }
}
