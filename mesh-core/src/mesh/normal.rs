// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::mesh::{Mesh, Normal};
use common::error::{ConversionError, Result};
use nalgebra::Vector3;

/// Recalculate vertex normals from face geometry
///
/// This function recalculates normals for all vertices based on the faces
/// they belong to. It uses area-weighted face normals to compute smooth
/// vertex normals.
///
/// # Arguments
///
/// * `mesh` - The mesh for which to recalculate normals
///
/// # Returns
///
/// Mesh with recalculated normals, or an error if recalculation fails.
///
/// # Algorithm
///
/// 1. For each face, calculate the face normal using cross product
/// 2. Weight the face normal by face area
/// 3. For each vertex, sum the weighted normals of all faces containing that vertex
/// 4. Normalize the resulting vertex normal
pub fn recalculate_normals(mut mesh: Mesh) -> Result<Mesh> {
    // Validate that we have vertices and faces
    if mesh.vertices.is_empty() {
        return Err(ConversionError::InvalidInput(
            "Cannot recalculate normals: mesh has no vertices".to_string(),
        ));
    }

    if mesh.faces.is_empty() {
        return Err(ConversionError::InvalidInput(
            "Cannot recalculate normals: mesh has no faces".to_string(),
        ));
    }

    // Initialize vertex normals accumulator
    let mut vertex_normals: Vec<Vector3<f32>> = vec![Vector3::zeros(); mesh.vertices.len()];

    // Calculate face normals and accumulate to vertex normals
    for face in &mesh.faces {
        let indices = face.indices;
        
        // Validate indices
        if indices[0] >= mesh.vertices.len()
            || indices[1] >= mesh.vertices.len()
            || indices[2] >= mesh.vertices.len()
        {
            return Err(ConversionError::InvalidInput(format!(
                "Invalid face indices: {:?} (vertex count: {})",
                indices,
                mesh.vertices.len()
            )));
        }

        // Get triangle vertices
        let v0 = &mesh.vertices[indices[0]];
        let v1 = &mesh.vertices[indices[1]];
        let v2 = &mesh.vertices[indices[2]];

        // Calculate edge vectors
        let edge1 = Vector3::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let edge2 = Vector3::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);

        // Calculate face normal using cross product
        let face_normal = edge1.cross(&edge2);
        
        // Calculate face area (half the magnitude of the cross product)
        let face_area = face_normal.norm() * 0.5;
        
        // If face area is too small, skip this face
        if face_area < 1e-10 {
            continue;
        }

        // Weight normal by face area and accumulate to each vertex
        let weighted_normal = face_normal * face_area;
        
        vertex_normals[indices[0]] += weighted_normal;
        vertex_normals[indices[1]] += weighted_normal;
        vertex_normals[indices[2]] += weighted_normal;
    }

    // Normalize vertex normals and store in mesh
    mesh.normals.clear();
    for normal_vec in vertex_normals {
        let length = normal_vec.norm();
        if length > 1e-6 {
            let normalized = normal_vec / length;
            mesh.normals.push(Normal {
                x: normalized.x,
                y: normalized.y,
                z: normalized.z,
            });
        } else {
            // If normal is zero (degenerate), use default up vector
            mesh.normals.push(Normal { x: 0.0, y: 0.0, z: 1.0 });
        }
    }

    // Ensure we have the same number of normals as vertices
    while mesh.normals.len() < mesh.vertices.len() {
        mesh.normals.push(Normal { x: 0.0, y: 0.0, z: 1.0 });
    }

    Ok(mesh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{Vertex, Face};

    #[test]
    fn test_recalculate_normals_simple_triangle() {
        let mut mesh = Mesh::new();
        
        // Create a simple triangle in the XY plane
        mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 });
        mesh.vertices.push(Vertex { x: 1.0, y: 0.0, z: 0.0 });
        mesh.vertices.push(Vertex { x: 0.5, y: 1.0, z: 0.0 });
        
        mesh.faces.push(Face {
            indices: [0, 1, 2],
        });

        let result = recalculate_normals(mesh).unwrap();
        
        // Should have normals for all vertices
        assert_eq!(result.normals.len(), 3);
        
        // All normals should point in +Z direction (upward from XY plane)
        for normal in &result.normals {
            assert!((normal.z - 1.0).abs() < 0.01, "Normal Z should be ~1.0, got {}", normal.z);
        }
    }

    #[test]
    fn test_recalculate_normals_empty_mesh() {
        let mesh = Mesh::new();
        let result = recalculate_normals(mesh);
        assert!(result.is_err());
    }

    #[test]
    fn test_recalculate_normals_no_faces() {
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 });
        
        let result = recalculate_normals(mesh);
        assert!(result.is_err());
    }

    #[test]
    fn test_recalculate_normals_invalid_indices() {
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 });
        mesh.faces.push(Face {
            indices: [0, 1, 2], // Invalid indices
        });
        
        let result = recalculate_normals(mesh);
        assert!(result.is_err());
    }

    #[test]
    fn test_recalculate_normals_skips_degenerate_faces() {
        let mut mesh = Mesh::new();
        
        // Create a valid triangle
        mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 });
        mesh.vertices.push(Vertex { x: 1.0, y: 0.0, z: 0.0 });
        mesh.vertices.push(Vertex { x: 0.5, y: 1.0, z: 0.0 });
        
        // Add a valid face
        mesh.faces.push(Face {
            indices: [0, 1, 2],
        });
        
        // Add a degenerate face (all vertices same point - area = 0)
        mesh.faces.push(Face {
            indices: [0, 0, 0], // Degenerate: all same vertex
        });
        
        // Should succeed - degenerate face should be skipped
        let result = recalculate_normals(mesh).unwrap();
        
        // Should still have normals for all vertices
        assert_eq!(result.normals.len(), 3);
        
        // Normals should be calculated from the valid face only
        // All normals should point in +Z direction
        for normal in &result.normals {
            assert!((normal.z - 1.0).abs() < 0.01, "Normal Z should be ~1.0, got {}", normal.z);
        }
    }
}

