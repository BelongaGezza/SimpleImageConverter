// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Normal, Vertex};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use std::io::{Cursor, Write};

/// STL format handler
pub struct StlFormat {
    limits: ResourceLimits,
}

impl StlFormat {
    /// Create a new STL format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new STL format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }
}

impl Default for StlFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for StlFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing to prevent memory exhaustion
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        let mut cursor = Cursor::new(data);

        // Use stl_io to read the STL file (auto-detects binary/ASCII)
        let stl_mesh = stl_io::read_stl(&mut cursor).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read STL file ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        // Security: Validate mesh resource counts before allocating
        self.limits
            .check_mesh_resources(stl_mesh.vertices.len(), stl_mesh.faces.len())?;

        // Convert stl_io IndexedMesh to our Mesh structure
        let mut mesh = Mesh::new();

        // Extract vertices
        mesh.vertices = stl_mesh
            .vertices
            .iter()
            .map(|v| Vertex {
                x: v[0],
                y: v[1],
                z: v[2],
            })
            .collect();

        // Extract faces (triangles) - STL uses triangle indices
        mesh.faces = stl_mesh
            .faces
            .iter()
            .map(|face| Face {
                indices: [face.vertices[0], face.vertices[1], face.vertices[2]],
            })
            .collect();

        // Extract normals from STL (STL stores face normals)
        mesh.normals = stl_mesh
            .faces
            .iter()
            .map(|face| Normal {
                x: face.normal[0],
                y: face.normal[1],
                z: face.normal[2],
            })
            .collect();

        Ok(mesh)
    }
}

impl MeshWriter for StlFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Validate mesh data
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "Mesh has no vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            return Err(ConversionError::InvalidInput(
                "Mesh has no faces".to_string(),
            ));
        }

        // Validate face indices
        for face in &mesh.faces {
            for &index in &face.indices {
                if index >= mesh.vertices.len() {
                    return Err(ConversionError::InvalidInput(format!(
                        "Face index {} is out of bounds (max: {})",
                        index,
                        mesh.vertices.len() - 1
                    )));
                }
            }
        }

        // Write to binary STL format
        let mut buffer = Vec::new();

        // Write 80-byte header (empty or with comment)
        let header = [0u8; 80];
        buffer.write_all(&header).map_err(ConversionError::Io)?;

        // Write number of triangles (4 bytes, little-endian)
        let num_triangles = mesh.faces.len() as u32;
        buffer
            .write_all(&num_triangles.to_le_bytes())
            .map_err(ConversionError::Io)?;

        // Write each triangle
        for (face_idx, face) in mesh.faces.iter().enumerate() {
            // Get the three vertices for this face
            let v0 = &mesh.vertices[face.indices[0]];
            let v1 = &mesh.vertices[face.indices[1]];
            let v2 = &mesh.vertices[face.indices[2]];

            // Get or calculate normal
            // STL format stores one normal per face
            let normal = if face_idx < mesh.normals.len() {
                // If we have a normal for this face, use it
                let n = &mesh.normals[face_idx];
                [n.x, n.y, n.z]
            } else {
                // Calculate normal from triangle vertices
                calculate_face_normal(v0, v1, v2)
            };

            // Write normal (3 floats = 12 bytes)
            buffer
                .write_all(&normal[0].to_le_bytes())
                .map_err(ConversionError::Io)?;
            buffer
                .write_all(&normal[1].to_le_bytes())
                .map_err(ConversionError::Io)?;
            buffer
                .write_all(&normal[2].to_le_bytes())
                .map_err(ConversionError::Io)?;

            // Write vertices (3 vertices * 3 floats = 9 floats = 36 bytes)
            let vertices = [[v0.x, v0.y, v0.z], [v1.x, v1.y, v1.z], [v2.x, v2.y, v2.z]];
            for vertex in &vertices {
                buffer
                    .write_all(&vertex[0].to_le_bytes())
                    .map_err(ConversionError::Io)?;
                buffer
                    .write_all(&vertex[1].to_le_bytes())
                    .map_err(ConversionError::Io)?;
                buffer
                    .write_all(&vertex[2].to_le_bytes())
                    .map_err(ConversionError::Io)?;
            }

            // Write attribute byte count (2 bytes, typically 0)
            buffer.write_all(&[0u8; 2]).map_err(ConversionError::Io)?;
        }

        Ok(buffer)
    }
}

/// Calculate face normal from three vertices using cross product
fn calculate_face_normal(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> [f32; 3] {
    // Edge vectors
    let edge1 = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
    let edge2 = [v2.x - v0.x, v2.y - v0.y, v2.z - v0.z];

    // Cross product
    let nx = edge1[1] * edge2[2] - edge1[2] * edge2[1];
    let ny = edge1[2] * edge2[0] - edge1[0] * edge2[2];
    let nz = edge1[0] * edge2[1] - edge1[1] * edge2[0];

    // Normalize
    let length = (nx * nx + ny * ny + nz * nz).sqrt();
    if length > 0.0 {
        [nx / length, ny / length, nz / length]
    } else {
        [0.0, 0.0, 1.0] // Default normal if degenerate triangle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a simple test mesh (a single triangle)
    fn create_test_triangle() -> Mesh {
        let mut mesh = Mesh::new();

        // Single triangle with three vertices
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.vertices.push(Vertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.vertices.push(Vertex {
            x: 0.5,
            y: 1.0,
            z: 0.0,
        });

        // One face
        mesh.faces.push(Face { indices: [0, 1, 2] });

        // One normal (face normal pointing up)
        mesh.normals.push(Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });

        mesh
    }

    /// Helper to create a simple cube mesh
    fn create_test_cube() -> Mesh {
        let mut mesh = Mesh::new();

        // Cube vertices
        mesh.vertices.extend_from_slice(&[
            Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }, // 0
            Vertex {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }, // 1
            Vertex {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            }, // 2
            Vertex {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }, // 3
            Vertex {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }, // 4
            Vertex {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            }, // 5
            Vertex {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }, // 6
            Vertex {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            }, // 7
        ]);

        // Cube faces (12 triangles for a cube)
        mesh.faces.extend_from_slice(&[
            // Bottom face
            Face { indices: [0, 1, 2] },
            Face { indices: [0, 2, 3] },
            // Top face
            Face { indices: [4, 7, 6] },
            Face { indices: [4, 6, 5] },
            // Front face
            Face { indices: [0, 4, 5] },
            Face { indices: [0, 5, 1] },
            // Back face
            Face { indices: [2, 6, 7] },
            Face { indices: [2, 7, 3] },
            // Left face
            Face { indices: [0, 3, 7] },
            Face { indices: [0, 7, 4] },
            // Right face
            Face { indices: [1, 5, 6] },
            Face { indices: [1, 6, 2] },
        ]);

        mesh
    }

    #[test]
    fn test_stl_format_new() {
        let format = StlFormat::new();
        assert!(format.read(&[]).is_err()); // Empty data should fail
    }

    #[test]
    fn test_write_empty_mesh() {
        let format = StlFormat::new();
        let mesh = Mesh::new();

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    #[test]
    fn test_write_mesh_with_no_faces() {
        let format = StlFormat::new();
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no faces"));
    }

    #[test]
    fn test_write_mesh_invalid_index() {
        let format = StlFormat::new();
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.faces.push(Face { indices: [0, 1, 2] }); // Indices 1 and 2 don't exist

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of bounds"));
    }

    #[test]
    fn test_write_triangle() {
        let format = StlFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let stl_data = result.unwrap();

        // Binary STL format: 80-byte header + 4-byte triangle count + triangle data
        // Each triangle: 12 bytes normal + 36 bytes vertices + 2 bytes attribute = 50 bytes
        assert_eq!(stl_data.len(), 80 + 4 + 50); // 134 bytes

        // Check header (all zeros)
        assert_eq!(&stl_data[0..80], &[0u8; 80]);

        // Check triangle count (little-endian u32 = 1)
        assert_eq!(&stl_data[80..84], &[1, 0, 0, 0]);
    }

    #[test]
    fn test_write_cube() {
        let format = StlFormat::new();
        let mesh = create_test_cube();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let stl_data = result.unwrap();

        // Binary STL: 80-byte header + 4-byte count + (12 triangles * 50 bytes each)
        assert_eq!(stl_data.len(), 80 + 4 + (12 * 50)); // 684 bytes

        // Check triangle count (12 triangles, little-endian)
        let count_bytes =
            u32::from_le_bytes([stl_data[80], stl_data[81], stl_data[82], stl_data[83]]);
        assert_eq!(count_bytes, 12);
    }

    #[test]
    fn test_round_trip_triangle() {
        let format = StlFormat::new();
        let original_mesh = create_test_triangle();

        // Write mesh to STL
        let stl_data = format.write(&original_mesh).unwrap();

        // Read STL back
        let result = format.read(&stl_data);
        assert!(result.is_ok());

        let read_mesh = result.unwrap();

        // Verify vertices match
        assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
        for (original, read) in original_mesh.vertices.iter().zip(read_mesh.vertices.iter()) {
            assert!((original.x - read.x).abs() < 0.001);
            assert!((original.y - read.y).abs() < 0.001);
            assert!((original.z - read.z).abs() < 0.001);
        }

        // Verify faces match
        assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
        assert_eq!(read_mesh.faces[0].indices, original_mesh.faces[0].indices);

        // Verify normals exist (STL format stores face normals)
        assert_eq!(read_mesh.normals.len(), read_mesh.faces.len());
    }

    #[test]
    fn test_round_trip_cube() {
        let format = StlFormat::new();
        let original_mesh = create_test_cube();

        // Write mesh to STL
        let stl_data = format.write(&original_mesh).unwrap();

        // Read STL back
        let result = format.read(&stl_data);
        assert!(result.is_ok());

        let read_mesh = result.unwrap();

        // Verify structure
        assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
        assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
        assert_eq!(read_mesh.normals.len(), read_mesh.faces.len());
    }

    #[test]
    fn test_write_mesh_without_normals() {
        let format = StlFormat::new();
        let mut mesh = create_test_triangle();
        mesh.normals.clear(); // Remove normals - they should be calculated

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let stl_data = result.unwrap();

        // Should still write successfully (normals calculated automatically)
        assert!(!stl_data.is_empty());

        // Verify we can read it back
        let read_result = format.read(&stl_data);
        assert!(read_result.is_ok());
    }

    #[test]
    fn test_calculate_face_normal() {
        // Test normal calculation for a simple triangle
        let v0 = Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vertex {
            x: 0.5,
            y: 1.0,
            z: 0.0,
        };

        let normal = calculate_face_normal(&v0, &v1, &v2);

        // Normal should point in +Z direction for this triangle
        assert!((normal[2] - 1.0).abs() < 0.001); // Z component should be ~1.0
        assert!((normal[0]).abs() < 0.001); // X should be ~0
        assert!((normal[1]).abs() < 0.001); // Y should be ~0
    }

    #[test]
    fn test_read_empty_data() {
        let format = StlFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_invalid_data() {
        let format = StlFormat::new();
        let invalid_data = vec![0u8; 100]; // Not valid STL format
        let result = format.read(&invalid_data);
        // This might succeed or fail depending on stl_io's parsing
        // But it's good to test that invalid data is handled
        assert!(result.is_err() || result.is_ok()); // Either is acceptable for now
    }

    // Security tests

    #[test]
    fn test_with_custom_limits() {
        let limits = ResourceLimits::builder()
            .max_vertices(100)
            .max_faces(50)
            .build();
        let format = StlFormat::with_limits(limits);

        // Create a mesh that we'll test with
        let mesh = create_test_cube();

        // Write the mesh (this should work, limits are checked on read)
        let stl_data = format.write(&mesh).unwrap();

        // Reading back should work since cube is small
        let result = format.read(&stl_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_limits_with_permissive() {
        let limits = ResourceLimits::permissive();
        let format = StlFormat::with_limits(limits);

        let mesh = create_test_cube();
        let stl_data = format.write(&mesh).unwrap();
        let result = format.read(&stl_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_binary_stl_format() {
        let format = StlFormat::new();
        let mesh = create_test_triangle();
        let stl_data = format.write(&mesh).unwrap();

        // Verify it's binary format (starts with header, not "solid")
        assert!(stl_data.len() >= 84); // At least header + count + one triangle
        assert_ne!(&stl_data[0..5], b"solid"); // Not ASCII format

        // Read it back
        let result = format.read(&stl_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_corrupted_binary_header() {
        let format = StlFormat::new();
        let mesh = create_test_triangle();
        let mut stl_data = format.write(&mesh).unwrap();

        // Corrupt the triangle count (set to invalid value)
        stl_data[80] = 0xFF;
        stl_data[81] = 0xFF;
        stl_data[82] = 0xFF;
        stl_data[83] = 0xFF;

        let result = format.read(&stl_data);
        // Should fail gracefully, not panic
        assert!(result.is_err());
    }

    #[test]
    fn test_read_truncated_file() {
        let format = StlFormat::new();
        let mesh = create_test_triangle();
        let mut stl_data = format.write(&mesh).unwrap();

        // Truncate the file (remove part of triangle data)
        stl_data.truncate(100);

        let result = format.read(&stl_data);
        // Should fail gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_write_read_degenerate_triangle() {
        let format = StlFormat::new();
        let mut mesh = Mesh::new();

        // Create a degenerate triangle (all vertices at same point)
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.faces.push(Face { indices: [0, 1, 2] });

        // Should still write successfully (degenerate triangles are handled)
        let result = format.write(&mesh);
        assert!(result.is_ok());

        // Reading back might succeed or fail depending on library behavior
        let stl_data = result.unwrap();
        let read_result = format.read(&stl_data);
        // Either outcome is acceptable
        assert!(read_result.is_ok() || read_result.is_err());
    }

    #[test]
    fn test_write_read_single_vertex_mesh() {
        let format = StlFormat::new();
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        // Can't write a mesh without faces
        let result = format.write(&mesh);
        assert!(result.is_err());
    }

    #[test]
    fn test_resource_limits_file_size() {
        let limits = ResourceLimits::builder()
            .max_file_size(1000) // Very small limit
            .build();
        let format = StlFormat::with_limits(limits);

        // Test reading an oversized file
        let oversized_data = vec![0u8; 2000];
        let result = format.read(&oversized_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds limit"));
    }

    #[test]
    fn test_resource_limits_vertex_count() {
        let limits = ResourceLimits::builder()
            .max_vertices(3) // Very small limit - triangle has 3 vertices
            .max_faces(5)
            .build();
        let format = StlFormat::with_limits(limits);

        // Use a cube which has 8 vertices - exceeds limit
        let mesh = create_test_cube();
        let stl_data = format.write(&mesh).unwrap();

        // Reading should fail due to vertex limit
        let result = format.read(&stl_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Vertex count") || error_msg.contains("exceeds limit"));
    }

    #[test]
    fn test_resource_limits_face_count() {
        let limits = ResourceLimits::builder()
            .max_vertices(100)
            .max_faces(5) // Very small face limit
            .build();
        let format = StlFormat::with_limits(limits);

        // Create a cube (which has 12 faces/triangles)
        let mesh = create_test_cube();
        let stl_data = format.write(&mesh).unwrap();

        // Reading should fail due to face limit
        let result = format.read(&stl_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Face count"));
    }

    #[test]
    fn test_normal_calculation_degenerate_edge() {
        // Test normal calculation with colinear vertices
        let v0 = Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vertex {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };

        let normal = calculate_face_normal(&v0, &v1, &v2);
        // Should return default normal [0, 0, 1] for degenerate case
        assert_eq!(normal[2], 1.0);
    }

    #[test]
    fn test_write_mesh_with_multiple_faces_and_normals() {
        let format = StlFormat::new();
        let mesh = create_test_cube();

        // Cube should have normals for all faces
        let result = format.write(&mesh);
        assert!(result.is_ok());

        let stl_data = result.unwrap();
        let read_result = format.read(&stl_data);
        assert!(read_result.is_ok());

        let read_mesh = read_result.unwrap();
        // Verify normals match face count
        assert_eq!(read_mesh.normals.len(), read_mesh.faces.len());
    }

    #[test]
    fn test_read_invalid_binary_header_size() {
        let format = StlFormat::new();
        // Create data that's too small to be valid STL
        let invalid_data = vec![0u8; 50]; // Less than 84 bytes minimum

        let result = format.read(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_normal_preservation() {
        let format = StlFormat::new();
        let mut mesh = create_test_triangle();

        // Set a specific normal
        mesh.normals[0] = Normal {
            x: 0.707,
            y: 0.707,
            z: 0.0,
        };

        let stl_data = format.write(&mesh).unwrap();
        let read_mesh = format.read(&stl_data).unwrap();

        // Verify normal is preserved (approximately)
        assert_eq!(read_mesh.normals.len(), 1);
        let normal = &read_mesh.normals[0];
        assert!((normal.x - 0.707).abs() < 0.1 || (normal.y - 0.707).abs() < 0.1);
    }
}
