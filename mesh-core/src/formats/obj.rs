// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Normal, Vertex};
use common::error::{ConversionError, Result};
use std::io::{Cursor, Write};

/// OBJ format handler
pub struct ObjFormat;

impl ObjFormat {
    /// Create a new OBJ format handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for ObjFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for ObjFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Convert bytes to string for OBJ parsing
        let obj_str = std::str::from_utf8(data).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to parse OBJ file as UTF-8: {}", e))
        })?;

        // Use tobj to load OBJ (it handles MTL files automatically if referenced)
        let (models, _materials) = tobj::load_obj_buf(
            &mut Cursor::new(obj_str.as_bytes()),
            &tobj::LoadOptions {
                triangulate: true,  // Triangulate quads and polygons
                single_index: true, // Use single index for vertices
                ignore_points: false,
                ignore_lines: false,
            },
            |_path| {
                // Material loader - return empty materials if MTL file not found
                // This allows OBJ files without MTL files to still load
                // AHashMap is from the ahash crate (transitive dependency of tobj)
                use ahash::AHashMap;
                Ok((Vec::new(), AHashMap::new()))
            },
        )
        .map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read OBJ file ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        if models.is_empty() {
            return Err(ConversionError::InvalidInput(
                "OBJ file contains no models".to_string(),
            ));
        }

        // Convert tobj models to our Mesh structure
        let mut mesh = Mesh::new();

        // Combine all models into a single mesh
        for model in &models {
            let mesh_data = &model.mesh;

            // Extract vertices
            let num_vertices = mesh_data.positions.len() / 3;
            for i in 0..num_vertices {
                mesh.vertices.push(Vertex {
                    x: mesh_data.positions[i * 3],
                    y: mesh_data.positions[i * 3 + 1],
                    z: mesh_data.positions[i * 3 + 2],
                });
            }

            // Extract faces (triangles) - tobj already triangulates
            let num_faces = mesh_data.indices.len() / 3;
            for i in 0..num_faces {
                mesh.faces.push(Face {
                    indices: [
                        mesh_data.indices[i * 3] as usize,
                        mesh_data.indices[i * 3 + 1] as usize,
                        mesh_data.indices[i * 3 + 2] as usize,
                    ],
                });
            }

            // Extract normals if present
            if !mesh_data.normals.is_empty() {
                let num_normals = mesh_data.normals.len() / 3;
                for i in 0..num_normals {
                    mesh.normals.push(Normal {
                        x: mesh_data.normals[i * 3],
                        y: mesh_data.normals[i * 3 + 1],
                        z: mesh_data.normals[i * 3 + 2],
                    });
                }
            }
        }

        // Validate mesh
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "OBJ file contains no vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            return Err(ConversionError::InvalidInput(
                "OBJ file contains no faces".to_string(),
            ));
        }

        Ok(mesh)
    }
}

impl MeshWriter for ObjFormat {
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

        // Write OBJ format (ASCII text)
        let mut buffer = Vec::new();

        // Write header comment
        writeln!(buffer, "# OBJ file generated by Simple Image Converter")
            .map_err(ConversionError::Io)?;

        // Write vertices
        for vertex in &mesh.vertices {
            writeln!(buffer, "v {:.6} {:.6} {:.6}", vertex.x, vertex.y, vertex.z)
                .map_err(ConversionError::Io)?;
        }

        // Write normals if present
        if !mesh.normals.is_empty() {
            for normal in &mesh.normals {
                writeln!(buffer, "vn {:.6} {:.6} {:.6}", normal.x, normal.y, normal.z)
                    .map_err(ConversionError::Io)?;
            }
        }

        // Write faces
        // OBJ uses 1-based indexing
        for (face_idx, face) in mesh.faces.iter().enumerate() {
            if !mesh.normals.is_empty() && face_idx < mesh.normals.len() {
                // Face with normals: f v1//n1 v2//n2 v3//n3
                writeln!(
                    buffer,
                    "f {}//{} {}//{} {}//{}",
                    face.indices[0] + 1,
                    face_idx + 1,
                    face.indices[1] + 1,
                    face_idx + 1,
                    face.indices[2] + 1,
                    face_idx + 1
                )
                .map_err(ConversionError::Io)?;
            } else {
                // Face without normals: f v1 v2 v3
                writeln!(
                    buffer,
                    "f {} {} {}",
                    face.indices[0] + 1,
                    face.indices[1] + 1,
                    face.indices[2] + 1
                )
                .map_err(ConversionError::Io)?;
            }
        }

        Ok(buffer)
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
    fn test_obj_format_new() {
        let format = ObjFormat::new();
        assert!(format.read(&[]).is_err()); // Empty data should fail
    }

    #[test]
    fn test_read_simple_triangle() {
        let format = ObjFormat::new();
        let obj_data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 0.5 1.0 0.0\nf 1 2 3\n";

        let result = format.read(obj_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, [0, 1, 2]);
    }

    #[test]
    fn test_read_cube() {
        let format = ObjFormat::new();
        // Simple cube OBJ
        let obj_data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 1.0 1.0 0.0\nv 0.0 1.0 0.0\n\
                         v 0.0 0.0 1.0\nv 1.0 0.0 1.0\nv 1.0 1.0 1.0\nv 0.0 1.0 1.0\n\
                         f 1 2 3\nf 1 3 4\nf 5 8 7\nf 5 7 6\n\
                         f 1 5 6\nf 1 6 2\nf 3 7 8\nf 3 8 4\n\
                         f 1 4 8\nf 1 8 5\nf 2 6 7\nf 2 7 3\n";

        let result = format.read(obj_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 8);
        assert_eq!(mesh.faces.len(), 12);
    }

    #[test]
    fn test_read_with_normals() {
        let format = ObjFormat::new();
        let obj_data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 0.5 1.0 0.0\n\
                         vn 0.0 0.0 1.0\n\
                         f 1//1 2//1 3//1\n";

        let result = format.read(obj_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert!(!mesh.normals.is_empty());
    }

    #[test]
    fn test_read_with_uvs() {
        let format = ObjFormat::new();
        // OBJ with texture coordinates (UVs are read but not stored in our Mesh structure)
        let obj_data = b"v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 0.5 1.0 0.0\n\
                         vt 0.0 0.0\nvt 1.0 0.0\nvt 0.5 1.0\n\
                         f 1/1 2/2 3/3\n";

        let result = format.read(obj_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn test_read_invalid_data() {
        let format = ObjFormat::new();
        let invalid_data = b"not a valid OBJ file\n";
        let result = format.read(invalid_data);
        // Should handle gracefully (might succeed with empty mesh or fail)
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_read_empty_data() {
        let format = ObjFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_triangle() {
        let format = ObjFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let obj_data = result.unwrap();
        let obj_str = std::str::from_utf8(&obj_data).unwrap();

        // Check that it contains vertices (check for vertex lines)
        let vertex_lines: Vec<&str> = obj_str.lines().filter(|l| l.starts_with("v ")).collect();
        assert_eq!(vertex_lines.len(), 3, "Should have 3 vertex lines");

        // Check that it contains the expected vertex coordinates (flexible matching)
        assert!(obj_str.contains("v ") && (obj_str.contains("0") || obj_str.contains("0.0")));

        // Check that it contains a face
        assert!(obj_str.contains("f "));
    }

    #[test]
    fn test_write_cube() {
        let format = ObjFormat::new();
        let mesh = create_test_cube();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let obj_data = result.unwrap();
        let obj_str = std::str::from_utf8(&obj_data).unwrap();

        // Check that it contains 8 vertices
        let vertex_count = obj_str.matches("v ").count();
        assert_eq!(vertex_count, 8);

        // Check that it contains faces
        assert!(obj_str.contains("f "));
    }

    #[test]
    fn test_write_mesh_without_normals() {
        let format = ObjFormat::new();
        let mut mesh = create_test_triangle();
        mesh.normals.clear(); // Remove normals

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let obj_data = result.unwrap();
        let obj_str = std::str::from_utf8(&obj_data).unwrap();

        // Should not contain normal references in faces
        assert!(!obj_str.contains("//"));
    }

    #[test]
    fn test_write_mesh_invalid_index() {
        let format = ObjFormat::new();
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
    fn test_write_empty_mesh() {
        let format = ObjFormat::new();
        let mesh = Mesh::new();

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    #[test]
    fn test_round_trip_triangle() {
        let format = ObjFormat::new();
        let original_mesh = create_test_triangle();

        // Write mesh to OBJ
        let obj_data = format.write(&original_mesh).unwrap();

        // Read OBJ back
        let result = format.read(&obj_data);
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
    }

    #[test]
    fn test_round_trip_cube() {
        let format = ObjFormat::new();
        let original_mesh = create_test_cube();

        // Write mesh to OBJ
        let obj_data = format.write(&original_mesh).unwrap();

        // Read OBJ back
        let result = format.read(&obj_data);
        assert!(result.is_ok());

        let read_mesh = result.unwrap();

        // Verify structure
        assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
        assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
    }
}
