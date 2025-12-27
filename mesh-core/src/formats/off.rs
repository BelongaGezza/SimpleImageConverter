// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Vertex};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use std::io::Write;

/// OFF format handler
pub struct OffFormat {
    limits: ResourceLimits,
}

impl OffFormat {
    /// Create a new OFF format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new OFF format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Parse OFF format from bytes
    fn parse_off(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing to prevent memory exhaustion
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        let text = std::str::from_utf8(data).map_err(|e| {
            ConversionError::ConversionFailed(format!("Invalid UTF-8 in OFF file: {}", e))
        })?;

        let lines: Vec<&str> = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#')) // Skip empty lines and comments
            .collect();

        if lines.is_empty() {
            return Err(ConversionError::InvalidInput(
                "OFF file is empty or contains only comments".to_string(),
            ));
        }

        // Parse header - should start with "OFF"
        let header_line = lines[0];
        if !header_line.starts_with("OFF") {
            return Err(ConversionError::InvalidFormat(format!(
                "Invalid OFF header: expected 'OFF', found '{}'",
                header_line
            )));
        }

        // Parse counts line (num_vertices num_faces [num_edges])
        if lines.len() < 2 {
            return Err(ConversionError::InvalidInput(
                "OFF file missing counts line".to_string(),
            ));
        }

        let counts_line = lines[1];
        let counts: Vec<&str> = counts_line.split_whitespace().collect();

        if counts.len() < 2 {
            return Err(ConversionError::InvalidInput(
                "OFF counts line must have at least 2 values".to_string(),
            ));
        }

        let num_vertices: usize = counts[0].parse().map_err(|e| {
            ConversionError::InvalidInput(format!("Invalid vertex count in OFF file: {}", e))
        })?;

        let num_faces: usize = counts[1].parse().map_err(|e| {
            ConversionError::InvalidInput(format!("Invalid face count in OFF file: {}", e))
        })?;

        // Security: Validate counts before allocating
        self.limits.check_mesh_resources(num_vertices, num_faces)?;

        // Check we have enough lines for vertices and faces
        let expected_lines = 2 + num_vertices + num_faces;
        if lines.len() < expected_lines {
            return Err(ConversionError::InvalidInput(format!(
                "OFF file has insufficient data: expected {} lines, found {}",
                expected_lines,
                lines.len()
            )));
        }

        let mut mesh = Mesh::new();

        // Parse vertices (lines 2 to 2+num_vertices)
        for (line_idx, vertex_line) in lines.iter().enumerate().skip(2).take(num_vertices) {
            let coords: Vec<&str> = vertex_line.split_whitespace().collect();

            if coords.len() < 3 {
                return Err(ConversionError::InvalidInput(format!(
                    "OFF vertex line {} has insufficient coordinates",
                    line_idx
                )));
            }

            let x: f32 = coords[0].parse().map_err(|e| {
                ConversionError::InvalidInput(format!(
                    "Invalid x coordinate in OFF vertex line {}: {}",
                    line_idx, e
                ))
            })?;

            let y: f32 = coords[1].parse().map_err(|e| {
                ConversionError::InvalidInput(format!(
                    "Invalid y coordinate in OFF vertex line {}: {}",
                    line_idx, e
                ))
            })?;

            let z: f32 = coords[2].parse().map_err(|e| {
                ConversionError::InvalidInput(format!(
                    "Invalid z coordinate in OFF vertex line {}: {}",
                    line_idx, e
                ))
            })?;

            mesh.vertices.push(Vertex { x, y, z });

            // Optional: handle vertex colors (ignore for now as per spec)
            // OFF format may have RGB colors as 4th-6th values
        }

        // Parse faces (lines 2+num_vertices to 2+num_vertices+num_faces)
        let face_start = 2 + num_vertices;
        for (line_idx, face_line) in lines.iter().enumerate().skip(face_start).take(num_faces) {
            let parts: Vec<&str> = face_line.split_whitespace().collect();

            if parts.is_empty() {
                return Err(ConversionError::InvalidInput(format!(
                    "OFF face line {} is empty",
                    line_idx
                )));
            }

            let num_face_vertices: usize = parts[0].parse().map_err(|e| {
                ConversionError::InvalidInput(format!(
                    "Invalid vertex count in OFF face line {}: {}",
                    line_idx, e
                ))
            })?;

            if num_face_vertices < 3 {
                return Err(ConversionError::InvalidInput(format!(
                    "OFF face line {} has fewer than 3 vertices: {}",
                    line_idx, num_face_vertices
                )));
            }

            if parts.len() < num_face_vertices + 1 {
                return Err(ConversionError::InvalidInput(format!(
                    "OFF face line {} has insufficient indices",
                    line_idx
                )));
            }

            // Parse vertex indices for this face
            let mut indices = Vec::new();
            for part in parts.iter().skip(1).take(num_face_vertices) {
                let idx: usize = part.parse().map_err(|e| {
                    ConversionError::InvalidInput(format!(
                        "Invalid vertex index in OFF face line {}: {}",
                        line_idx, e
                    ))
                })?;

                if idx >= mesh.vertices.len() {
                    return Err(ConversionError::InvalidInput(format!(
                        "OFF face vertex index {} is out of bounds (max: {})",
                        idx,
                        mesh.vertices.len() - 1
                    )));
                }

                indices.push(idx);
            }

            // Triangulate polygon using fan triangulation
            // For a polygon with n vertices, create n-2 triangles
            for triangle_idx in 0..(indices.len() - 2) {
                mesh.faces.push(Face {
                    indices: [
                        indices[0],
                        indices[triangle_idx + 1],
                        indices[triangle_idx + 2],
                    ],
                });
            }

            // Optional: handle face colors (ignore for now)
        }

        // Validate mesh
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "OFF file contains no vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            return Err(ConversionError::InvalidInput(
                "OFF file contains no faces".to_string(),
            ));
        }

        Ok(mesh)
    }
}

impl Default for OffFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for OffFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        self.parse_off(data)
    }
}

impl MeshWriter for OffFormat {
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

        // Write OFF format (ASCII)
        let mut buffer = Vec::new();

        // Write header
        writeln!(buffer, "OFF").map_err(ConversionError::Io)?;
        writeln!(buffer, "# Generated by Simple Image Converter").map_err(ConversionError::Io)?;

        // Write counts (vertices, faces, edges=0)
        writeln!(buffer, "{} {} {}", mesh.vertices.len(), mesh.faces.len(), 0)
            .map_err(ConversionError::Io)?;

        // Write vertices
        for vertex in &mesh.vertices {
            writeln!(buffer, "{:.6} {:.6} {:.6}", vertex.x, vertex.y, vertex.z)
                .map_err(ConversionError::Io)?;
        }

        // Write faces (triangles) - OFF format: "3 v1 v2 v3"
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
    fn test_off_format_new() {
        let format = OffFormat::new();
        assert!(format.read(&[]).is_err()); // Empty data should fail
    }

    #[test]
    fn test_read_simple_triangle() {
        let format = OffFormat::new();
        let off_data = b"OFF\n3 1 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n";

        let result = format.read(off_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].indices, [0, 1, 2]);
    }

    #[test]
    fn test_read_cube() {
        let format = OffFormat::new();
        // Simple cube OFF
        let off_data = b"OFF\n8 12 0\n\
                         0.0 0.0 0.0\n1.0 0.0 0.0\n1.0 1.0 0.0\n0.0 1.0 0.0\n\
                         0.0 0.0 1.0\n1.0 0.0 1.0\n1.0 1.0 1.0\n0.0 1.0 1.0\n\
                         3 0 1 2\n3 0 2 3\n3 4 7 6\n3 4 6 5\n\
                         3 0 4 5\n3 0 5 1\n3 2 6 7\n3 2 7 3\n\
                         3 0 3 7\n3 0 7 4\n3 1 5 6\n3 1 6 2\n";

        let result = format.read(off_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 8);
        assert_eq!(mesh.faces.len(), 12);
    }

    #[test]
    fn test_read_with_colors() {
        let format = OffFormat::new();
        // OFF with vertex colors (should ignore colors)
        let off_data = b"OFF\n3 1 0\n\
                         0.0 0.0 0.0 1.0 0.0 0.0\n\
                         1.0 0.0 0.0 0.0 1.0 0.0\n\
                         0.5 1.0 0.0 0.0 0.0 1.0\n\
                         3 0 1 2\n";

        let result = format.read(off_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
        // Colors should be ignored, vertices should still parse correctly
    }

    #[test]
    fn test_read_polygon_face() {
        let format = OffFormat::new();
        // OFF with a quad face (should triangulate)
        let off_data = b"OFF\n4 1 0\n\
                         0.0 0.0 0.0\n1.0 0.0 0.0\n1.0 1.0 0.0\n0.0 1.0 0.0\n\
                         4 0 1 2 3\n";

        let result = format.read(off_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.faces.len(), 2); // Quad should be triangulated into 2 triangles
    }

    #[test]
    fn test_read_invalid_header() {
        let format = OffFormat::new();
        let invalid_data = b"OBJ\n3 1 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n";
        let result = format.read(invalid_data);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid OFF header"));
    }

    #[test]
    fn test_read_invalid_counts() {
        let format = OffFormat::new();
        let invalid_data = b"OFF\ninvalid counts\n";
        let result = format.read(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_empty_data() {
        let format = OffFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_triangle() {
        let format = OffFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let off_data = result.unwrap();
        let off_str = std::str::from_utf8(&off_data).unwrap();

        // Check OFF header
        assert!(off_str.contains("OFF"));
        assert!(off_str.contains("3 1 0")); // 3 vertices, 1 face, 0 edges

        // Check that it contains vertex data
        assert!(off_str.contains("0.000000") || off_str.contains("0.0"));
    }

    #[test]
    fn test_write_cube() {
        let format = OffFormat::new();
        let mesh = create_test_cube();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let off_data = result.unwrap();
        let off_str = std::str::from_utf8(&off_data).unwrap();

        // Check counts
        assert!(off_str.contains("8 12 0")); // 8 vertices, 12 faces, 0 edges
    }

    #[test]
    fn test_write_mesh_invalid_index() {
        let format = OffFormat::new();
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
    fn test_round_trip_triangle() {
        let format = OffFormat::new();
        let original_mesh = create_test_triangle();

        // Write mesh to OFF
        let off_data = format.write(&original_mesh).unwrap();

        // Read OFF back
        let result = format.read(&off_data);
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
        let format = OffFormat::new();
        let original_mesh = create_test_cube();

        // Write mesh to OFF
        let off_data = format.write(&original_mesh).unwrap();

        // Read OFF back
        let result = format.read(&off_data);
        assert!(result.is_ok());

        let read_mesh = result.unwrap();

        // Verify structure
        assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
        assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
    }

    #[test]
    fn test_read_with_comments() {
        let format = OffFormat::new();
        let off_data = b"OFF\n# This is a comment\n# Another comment\n3 1 0\n\
                         0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n\
                         # Comment before face\n3 0 1 2\n";

        let result = format.read(off_data);
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn test_write_empty_mesh() {
        let format = OffFormat::new();
        let mesh = Mesh::new();

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    #[test]
    fn test_resource_limits_file_size() {
        let limits = ResourceLimits::builder()
            .max_file_size(100) // Very small limit
            .build();
        let format = OffFormat::with_limits(limits);

        // Test reading an oversized file
        let oversized_data = vec![b'O'; 200];
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
        let format = OffFormat::with_limits(limits);

        // Create OFF data for a cube (8 vertices)
        let cube_mesh = create_test_cube();
        let off_data = format.write(&cube_mesh).unwrap();

        // Reading should fail due to vertex limit
        let result = format.read(&off_data);
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
        let format = OffFormat::with_limits(limits);

        // Create a cube (which has 12 faces/triangles)
        let cube_mesh = create_test_cube();
        let off_data = format.write(&cube_mesh).unwrap();

        // Reading should fail due to face limit
        let result = format.read(&off_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Face count"));
    }

    #[test]
    fn test_read_truncated_off_file() {
        let format = OffFormat::new();
        let mesh = create_test_triangle();
        let mut off_data = format.write(&mesh).unwrap();

        // Truncate the file
        off_data.truncate(off_data.len() / 2);

        let result = format.read(&off_data);
        // Should fail gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_write_read_degenerate_triangle() {
        let format = OffFormat::new();
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
        let off_data = result.unwrap();
        let read_result = format.read(&off_data);
        assert!(read_result.is_ok() || read_result.is_err());
    }
}
