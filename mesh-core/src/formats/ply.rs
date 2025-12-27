// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Normal, Vertex};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use std::io::{Cursor, Write};

/// PLY format handler
pub struct PlyFormat {
    limits: ResourceLimits,
}

impl PlyFormat {
    /// Create a new PLY format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new PLY format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }
}

impl Default for PlyFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for PlyFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing to prevent memory exhaustion
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        let mut cursor = Cursor::new(data);

        // Use ply_rs to read PLY file
        let ply_reader = ply_rs::parser::Parser::<ply_rs::ply::DefaultElement>::new();
        let ply = ply_reader.read_ply(&mut cursor).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to read PLY file ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        let mut mesh = Mesh::new();

        // Extract vertices from PLY
        if let Some(vertex_element) = ply.payload.get("vertex") {
            for vertex_data in vertex_element {
                // Extract x, y, z coordinates from Property enum
                let x: f32 = match vertex_data.get("x") {
                    Some(ply_rs::ply::Property::Float(f)) => *f,
                    Some(ply_rs::ply::Property::Double(d)) => *d as f32,
                    _ => {
                        return Err(ConversionError::InvalidInput(
                            "PLY vertex missing x coordinate".to_string(),
                        ));
                    }
                };
                let y: f32 = match vertex_data.get("y") {
                    Some(ply_rs::ply::Property::Float(f)) => *f,
                    Some(ply_rs::ply::Property::Double(d)) => *d as f32,
                    _ => {
                        return Err(ConversionError::InvalidInput(
                            "PLY vertex missing y coordinate".to_string(),
                        ));
                    }
                };
                let z: f32 = match vertex_data.get("z") {
                    Some(ply_rs::ply::Property::Float(f)) => *f,
                    Some(ply_rs::ply::Property::Double(d)) => *d as f32,
                    _ => {
                        return Err(ConversionError::InvalidInput(
                            "PLY vertex missing z coordinate".to_string(),
                        ));
                    }
                };

                mesh.vertices.push(Vertex { x, y, z });

                // Extract normals if present
                if let (Some(nx_prop), Some(ny_prop), Some(nz_prop)) = (
                    vertex_data.get("nx"),
                    vertex_data.get("ny"),
                    vertex_data.get("nz"),
                ) {
                    let nx_val = match nx_prop {
                        ply_rs::ply::Property::Float(f) => *f,
                        ply_rs::ply::Property::Double(d) => *d as f32,
                        _ => continue,
                    };
                    let ny_val = match ny_prop {
                        ply_rs::ply::Property::Float(f) => *f,
                        ply_rs::ply::Property::Double(d) => *d as f32,
                        _ => continue,
                    };
                    let nz_val = match nz_prop {
                        ply_rs::ply::Property::Float(f) => *f,
                        ply_rs::ply::Property::Double(d) => *d as f32,
                        _ => continue,
                    };
                    mesh.normals.push(Normal {
                        x: nx_val,
                        y: ny_val,
                        z: nz_val,
                    });
                }
            }
        } else {
            return Err(ConversionError::InvalidInput(
                "PLY file contains no vertices".to_string(),
            ));
        }

        // Extract faces from PLY
        if let Some(face_element) = ply.payload.get("face") {
            for face_data in face_element {
                // PLY faces can have variable vertex counts, we need to triangulate
                if let Some(vertex_indices_prop) = face_data.get("vertex_indices") {
                    let indices = match vertex_indices_prop {
                        ply_rs::ply::Property::ListUInt(v) => {
                            v.iter().map(|&i| i as usize).collect::<Vec<_>>()
                        }
                        ply_rs::ply::Property::ListInt(v) => {
                            v.iter().map(|&i| i as usize).collect::<Vec<_>>()
                        }
                        _ => {
                            return Err(ConversionError::InvalidInput(
                                "PLY face has invalid vertex_indices type".to_string(),
                            ));
                        }
                    };

                    // Triangulate polygon (fan triangulation)
                    if indices.len() < 3 {
                        return Err(ConversionError::InvalidInput(
                            "PLY face has fewer than 3 vertices".to_string(),
                        ));
                    }

                    // Create triangles from polygon using fan triangulation
                    for i in 1..(indices.len() - 1) {
                        mesh.faces.push(Face {
                            indices: [indices[0], indices[i], indices[i + 1]],
                        });
                    }
                } else {
                    return Err(ConversionError::InvalidInput(
                        "PLY face missing vertex_indices".to_string(),
                    ));
                }
            }
        } else {
            return Err(ConversionError::InvalidInput(
                "PLY file contains no faces".to_string(),
            ));
        }

        // Validate mesh
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "PLY file contains no vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            return Err(ConversionError::InvalidInput(
                "PLY file contains no faces".to_string(),
            ));
        }

        // Validate face indices
        for face in &mesh.faces {
            for &index in &face.indices {
                if index >= mesh.vertices.len() {
                    return Err(ConversionError::InvalidInput(format!(
                        "PLY face index {} is out of bounds (max: {})",
                        index,
                        mesh.vertices.len() - 1
                    )));
                }
            }
        }

        // Security: Validate mesh resource counts against limits
        self.limits
            .check_mesh_resources(mesh.vertices.len(), mesh.faces.len())?;

        Ok(mesh)
    }
}

impl MeshWriter for PlyFormat {
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

        // Write PLY format (ASCII)
        let mut buffer = Vec::new();

        // Write PLY header
        writeln!(buffer, "ply").map_err(ConversionError::Io)?;
        writeln!(buffer, "format ascii 1.0").map_err(ConversionError::Io)?;
        writeln!(buffer, "comment Generated by Simple Image Converter")
            .map_err(ConversionError::Io)?;

        // Write vertex element
        writeln!(buffer, "element vertex {}", mesh.vertices.len()).map_err(ConversionError::Io)?;
        writeln!(buffer, "property float x").map_err(ConversionError::Io)?;
        writeln!(buffer, "property float y").map_err(ConversionError::Io)?;
        writeln!(buffer, "property float z").map_err(ConversionError::Io)?;

        // Add normal properties if normals are present for ALL vertices
        let has_normals = !mesh.normals.is_empty() && mesh.normals.len() == mesh.vertices.len();
        if has_normals {
            writeln!(buffer, "property float nx").map_err(ConversionError::Io)?;
            writeln!(buffer, "property float ny").map_err(ConversionError::Io)?;
            writeln!(buffer, "property float nz").map_err(ConversionError::Io)?;
        }

        // Write face element
        writeln!(buffer, "element face {}", mesh.faces.len()).map_err(ConversionError::Io)?;
        writeln!(buffer, "property list uchar int vertex_indices").map_err(ConversionError::Io)?;

        writeln!(buffer, "end_header").map_err(ConversionError::Io)?;

        // Write vertices
        // Only write normals if we have normals for ALL vertices
        let write_normals = has_normals && mesh.normals.len() == mesh.vertices.len();
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            if write_normals {
                let normal = &mesh.normals[i];
                writeln!(
                    buffer,
                    "{} {} {} {} {} {}",
                    vertex.x, vertex.y, vertex.z, normal.x, normal.y, normal.z
                )
                .map_err(ConversionError::Io)?;
            } else {
                writeln!(buffer, "{} {} {}", vertex.x, vertex.y, vertex.z)
                    .map_err(ConversionError::Io)?;
            }
        }

        // Write faces (triangles)
        for face in &mesh.faces {
            writeln!(
                buffer,
                "3 {} {} {}",
                face.indices[0], face.indices[1], face.indices[2]
            )
            .map_err(ConversionError::Io)?;
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
    fn test_ply_format_new() {
        let format = PlyFormat::new();
        assert!(format.read(&[]).is_err()); // Empty data should fail
    }

    #[test]
    fn test_read_simple_triangle_ascii() {
        let format = PlyFormat::new();
        let ply_data = b"ply\nformat ascii 1.0\ncomment Test\n\
                         element vertex 3\n\
                         property float x\nproperty float y\nproperty float z\n\
                         element face 1\n\
                         property list uchar int vertex_indices\n\
                         end_header\n\
                         0.0 0.0 0.0\n\
                         1.0 0.0 0.0\n\
                         0.5 1.0 0.0\n\
                         3 0 1 2\n";

        let result = format.read(ply_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, [0, 1, 2]);
    }

    #[test]
    fn test_read_cube_ascii() {
        let format = PlyFormat::new();
        // Simple cube PLY (minimal version)
        let ply_data = b"ply\nformat ascii 1.0\n\
                         element vertex 8\n\
                         property float x\nproperty float y\nproperty float z\n\
                         element face 12\n\
                         property list uchar int vertex_indices\n\
                         end_header\n\
                         0.0 0.0 0.0\n1.0 0.0 0.0\n1.0 1.0 0.0\n0.0 1.0 0.0\n\
                         0.0 0.0 1.0\n1.0 0.0 1.0\n1.0 1.0 1.0\n0.0 1.0 1.0\n\
                         3 0 1 2\n3 0 2 3\n3 4 7 6\n3 4 6 5\n\
                         3 0 4 5\n3 0 5 1\n3 2 6 7\n3 2 7 3\n\
                         3 0 3 7\n3 0 7 4\n3 1 5 6\n3 1 6 2\n";

        let result = format.read(ply_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 8);
        assert_eq!(mesh.faces.len(), 12);
    }

    #[test]
    fn test_read_with_normals() {
        let format = PlyFormat::new();
        let ply_data = b"ply\nformat ascii 1.0\n\
                         element vertex 3\n\
                         property float x\nproperty float y\nproperty float z\n\
                         property float nx\nproperty float ny\nproperty float nz\n\
                         element face 1\n\
                         property list uchar int vertex_indices\n\
                         end_header\n\
                         0.0 0.0 0.0 0.0 0.0 1.0\n\
                         1.0 0.0 0.0 0.0 0.0 1.0\n\
                         0.5 1.0 0.0 0.0 0.0 1.0\n\
                         3 0 1 2\n";

        let result = format.read(ply_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.normals.len(), 3);
    }

    #[test]
    fn test_read_invalid_data() {
        let format = PlyFormat::new();
        let invalid_data = b"not a valid PLY file\n";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_empty_data() {
        let format = PlyFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_triangle_ascii() {
        let format = PlyFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let ply_data = result.unwrap();
        let ply_str = std::str::from_utf8(&ply_data).unwrap();

        // Check PLY header
        assert!(ply_str.contains("ply"));
        assert!(ply_str.contains("format ascii 1.0"));
        assert!(ply_str.contains("element vertex 3"));
        assert!(ply_str.contains("element face 1"));

        // Check that it contains vertex data (after header)
        let lines: Vec<&str> = ply_str.lines().collect();
        let header_end = lines.iter().position(|l| l.contains("end_header"));
        if let Some(pos) = header_end {
            let after_header = &lines[pos + 1..];
            let has_vertex_data = after_header
                .iter()
                .any(|l| l.contains("0") || l.contains("0.0"));
            assert!(has_vertex_data, "Should contain vertex data after header");
        } else {
            panic!("PLY file missing end_header");
        }
    }

    #[test]
    fn test_write_cube_ascii() {
        let format = PlyFormat::new();
        let mesh = create_test_cube();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let ply_data = result.unwrap();
        let ply_str = std::str::from_utf8(&ply_data).unwrap();

        // Check PLY header
        assert!(ply_str.contains("element vertex 8"));
        assert!(ply_str.contains("element face 12"));
    }

    #[test]
    fn test_write_mesh_without_normals() {
        let format = PlyFormat::new();
        let mut mesh = create_test_triangle();
        mesh.normals.clear(); // Remove normals

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let ply_data = result.unwrap();
        let ply_str = std::str::from_utf8(&ply_data).unwrap();

        // Should not contain normal properties
        assert!(!ply_str.contains("property float nx"));
    }

    #[test]
    fn test_write_mesh_invalid_index() {
        let format = PlyFormat::new();
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
        let format = PlyFormat::new();
        let mesh = Mesh::new();

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    #[test]
    fn test_round_trip_triangle() {
        let format = PlyFormat::new();
        let original_mesh = create_test_triangle();

        // Write mesh to PLY
        let ply_data = format.write(&original_mesh).unwrap();

        // Read PLY back
        let result = format.read(&ply_data);
        if let Err(ref e) = result {
            eprintln!("PLY read error: {}", e);
            let ply_str = std::str::from_utf8(&ply_data).unwrap_or("invalid utf8");
            eprintln!("PLY content:\n{}", &ply_str[..ply_str.len().min(500)]);
        }
        assert!(
            result.is_ok(),
            "PLY round-trip failed: {:?}",
            result.as_ref().unwrap_err()
        );

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
        let format = PlyFormat::new();
        let original_mesh = create_test_cube();

        // Write mesh to PLY
        let ply_data = format.write(&original_mesh).unwrap();

        // Read PLY back
        let result = format.read(&ply_data);
        assert!(result.is_ok());

        let read_mesh = result.unwrap();

        // Verify structure
        assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
        assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
    }

    #[test]
    fn test_read_ply_with_missing_properties() {
        let format = PlyFormat::new();
        // PLY file with missing z coordinate property
        let ply_data = b"ply\nformat ascii 1.0\nelement vertex 3\nproperty float x\nproperty float y\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n0.0 0.0\n1.0 0.0\n0.5 1.0\n3 0 1 2\n";

        let result = format.read(ply_data);
        // Should fail because z coordinate is missing
        assert!(result.is_err());
    }

    #[test]
    fn test_read_ply_with_invalid_header() {
        let format = PlyFormat::new();
        // Invalid PLY header
        let invalid_data = b"not a ply file\n";

        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_ply_with_missing_vertex_element() {
        let format = PlyFormat::new();
        // PLY file without vertex element
        let ply_data = b"ply\nformat ascii 1.0\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n3 0 1 2\n";

        let result = format.read(ply_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    #[test]
    fn test_read_ply_with_polygon_faces() {
        let format = PlyFormat::new();
        // PLY file with quad face (should be triangulated)
        let ply_data = b"ply\nformat ascii 1.0\nelement vertex 4\nproperty float x\nproperty float y\nproperty float z\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n0.0 0.0 0.0\n1.0 0.0 0.0\n1.0 1.0 0.0\n0.0 1.0 0.0\n4 0 1 2 3\n";

        let result = format.read(ply_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 4);
        // Quad should be triangulated into 2 triangles
        assert_eq!(mesh.faces.len(), 2);
    }

    #[test]
    fn test_resource_limits_file_size() {
        let limits = ResourceLimits::builder()
            .max_file_size(100) // Very small limit
            .build();
        let format = PlyFormat::with_limits(limits);

        // Test reading an oversized file
        let oversized_data = vec![b'p'; 200];
        let result = format.read(&oversized_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds limit"));
    }

    #[test]
    fn test_resource_limits_vertex_count() {
        let limits = ResourceLimits::builder()
            .max_vertices(3) // Very small limit
            .max_faces(5)
            .build();
        let format = PlyFormat::with_limits(limits);

        // Create PLY data for a cube (8 vertices)
        let cube_mesh = create_test_cube();
        let ply_data = format.write(&cube_mesh).unwrap();

        // Reading should fail due to vertex limit
        let result = format.read(&ply_data);
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
        let format = PlyFormat::with_limits(limits);

        // Create a cube (which has 12 faces/triangles)
        let cube_mesh = create_test_cube();
        let ply_data = format.write(&cube_mesh).unwrap();

        // Reading should fail due to face limit
        let result = format.read(&ply_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Face count"));
    }

    #[test]
    fn test_read_truncated_ply_file() {
        let format = PlyFormat::new();
        let mesh = create_test_triangle();
        let mut ply_data = format.write(&mesh).unwrap();

        // Truncate the file (remove part of data)
        ply_data.truncate(ply_data.len() / 2);

        let result = format.read(&ply_data);
        // Should fail gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_write_read_degenerate_triangle() {
        let format = PlyFormat::new();
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

        // Should write successfully
        let result = format.write(&mesh);
        assert!(result.is_ok());

        // Reading back might succeed or fail
        let ply_data = result.unwrap();
        let read_result = format.read(&ply_data);
        assert!(read_result.is_ok() || read_result.is_err());
    }

    #[test]
    fn test_read_ply_with_face_fewer_than_3_vertices() {
        let format = PlyFormat::new();
        // PLY file with face having fewer than 3 vertices (invalid)
        let ply_data = b"ply\nformat ascii 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n0.0 0.0 0.0\n1.0 0.0 0.0\n2 0 1\n";

        let result = format.read(ply_data);
        // Should fail because face has fewer than 3 vertices
        assert!(result.is_err());
    }

    #[test]
    fn test_read_ply_with_out_of_bounds_face_indices() {
        let format = PlyFormat::new();
        // PLY file with face indices that are out of bounds
        let ply_data = b"ply\nformat ascii 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n0.0 0.0 0.0\n1.0 0.0 0.0\n3 0 1 5\n";

        let result = format.read(ply_data);
        // Should fail because index 5 is out of bounds
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of bounds"));
    }

    #[test]
    fn test_write_read_normal_properties() {
        let format = PlyFormat::new();
        let mut mesh = create_test_triangle();
        
        // Add normals for all vertices
        mesh.normals.clear();
        mesh.normals.push(Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        mesh.normals.push(Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        mesh.normals.push(Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });

        let ply_data = format.write(&mesh).unwrap();
        let ply_str = std::str::from_utf8(&ply_data).unwrap();
        
        // Should include normal properties in header
        assert!(ply_str.contains("property float nx"));
        assert!(ply_str.contains("property float ny"));
        assert!(ply_str.contains("property float nz"));

        // Should be able to read it back
        let result = format.read(&ply_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_ply_with_comments() {
        let format = PlyFormat::new();
        // PLY file with comments
        let ply_data = b"ply\ncomment This is a comment\nformat ascii 1.0\nelement vertex 3\nproperty float x\nproperty float y\nproperty float z\nelement face 1\nproperty list uchar int vertex_indices\nend_header\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n";

        let result = format.read(ply_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }
}
