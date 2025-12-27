// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Vertex};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use std::io::{Cursor, Write};

/// DXF format handler
pub struct DxfFormat {
    limits: ResourceLimits,
}

impl DxfFormat {
    /// Create a new DXF format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new DXF format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Parse DXF format from bytes
    fn parse_dxf(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        // Parse DXF using dxf crate (requires a Read trait)
        let mut cursor = Cursor::new(data);
        let drawing = dxf::Drawing::load(&mut cursor).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to parse DXF file: {}", e))
        })?;

        let mut mesh = Mesh::new();

        // Extract 3D entities from DXF
        for entity in drawing.entities() {
            match &entity.specific {
                dxf::entities::EntityType::Line(_line) => {
                    // Convert 3D line to two vertices (or skip - lines aren't faces)
                    // For now, we'll skip lines as they don't form faces
                }
                dxf::entities::EntityType::Face3D(face) => {
                    // Extract 3DFACE - this is a quad face
                    let v0_idx = mesh.vertices.len();
                    mesh.vertices.push(Vertex {
                        x: face.first_corner.x as f32,
                        y: face.first_corner.y as f32,
                        z: face.first_corner.z as f32,
                    });

                    let v1_idx = mesh.vertices.len();
                    mesh.vertices.push(Vertex {
                        x: face.second_corner.x as f32,
                        y: face.second_corner.y as f32,
                        z: face.second_corner.z as f32,
                    });

                    let v2_idx = mesh.vertices.len();
                    mesh.vertices.push(Vertex {
                        x: face.third_corner.x as f32,
                        y: face.third_corner.y as f32,
                        z: face.third_corner.z as f32,
                    });

                    let v3_idx = mesh.vertices.len();
                    mesh.vertices.push(Vertex {
                        x: face.fourth_corner.x as f32,
                        y: face.fourth_corner.y as f32,
                        z: face.fourth_corner.z as f32,
                    });

                    // Triangulate quad into two triangles
                    mesh.faces.push(Face {
                        indices: [v0_idx, v1_idx, v2_idx],
                    });
                    mesh.faces.push(Face {
                        indices: [v0_idx, v2_idx, v3_idx],
                    });
                }
                dxf::entities::EntityType::Polyline(polyline) => {
                    // Extract 3D polyline vertices (check if it's actually 3D)
                    let vertices: Vec<_> = polyline.vertices().collect();
                    if vertices.is_empty() {
                        continue;
                    }

                    let vertex_start = mesh.vertices.len();

                    // Add all vertices from polyline
                    for vertex in &vertices {
                        mesh.vertices.push(Vertex {
                            x: vertex.location.x as f32,
                            y: vertex.location.y as f32,
                            z: vertex.location.z as f32,
                        });
                    }

                    // Create faces from polyline segments
                    // For closed polylines, create triangles
                    if polyline.is_closed() && vertices.len() >= 3 {
                        // Fan triangulation for closed polygon
                        for i in 1..(vertices.len() - 1) {
                            mesh.faces.push(Face {
                                indices: [vertex_start, vertex_start + i, vertex_start + i + 1],
                            });
                        }
                    }
                }
                _ => {
                    // Ignore other entity types (2D entities, etc.)
                }
            }
        }

        // Validate mesh
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "DXF file contains no 3D vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            // If we have vertices but no faces, that's okay for some DXF files
            // (e.g., wireframe models)
            // We could optionally generate edges, but for now we'll return an error
            return Err(ConversionError::InvalidInput(
                "DXF file contains no 3D faces (3DFACE entities)".to_string(),
            ));
        }

        // Security: Validate mesh resource counts
        self.limits
            .check_mesh_resources(mesh.vertices.len(), mesh.faces.len())?;

        Ok(mesh)
    }
}

impl Default for DxfFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for DxfFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        self.parse_dxf(data)
    }
}

impl MeshWriter for DxfFormat {
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

        // Write DXF format (ASCII)
        let mut buffer = Vec::new();

        // Write DXF header
        writeln!(buffer, "0").map_err(ConversionError::Io)?;
        writeln!(buffer, "SECTION").map_err(ConversionError::Io)?;
        writeln!(buffer, "2").map_err(ConversionError::Io)?;
        writeln!(buffer, "HEADER").map_err(ConversionError::Io)?;
        writeln!(buffer, "9").map_err(ConversionError::Io)?;
        writeln!(buffer, "$ACADVER").map_err(ConversionError::Io)?;
        writeln!(buffer, "1").map_err(ConversionError::Io)?;
        writeln!(buffer, "AC1015").map_err(ConversionError::Io)?;
        writeln!(buffer, "0").map_err(ConversionError::Io)?;
        writeln!(buffer, "ENDSEC").map_err(ConversionError::Io)?;

        // Write ENTITIES section
        writeln!(buffer, "0").map_err(ConversionError::Io)?;
        writeln!(buffer, "SECTION").map_err(ConversionError::Io)?;
        writeln!(buffer, "2").map_err(ConversionError::Io)?;
        writeln!(buffer, "ENTITIES").map_err(ConversionError::Io)?;

        // Write 3DFACE entities for each triangle
        for face in &mesh.faces {
            let v0 = &mesh.vertices[face.indices[0]];
            let v1 = &mesh.vertices[face.indices[1]];
            let v2 = &mesh.vertices[face.indices[2]];

            // Write 3DFACE entity
            writeln!(buffer, "0").map_err(ConversionError::Io)?;
            writeln!(buffer, "3DFACE").map_err(ConversionError::Io)?;
            writeln!(buffer, "8").map_err(ConversionError::Io)?;
            writeln!(buffer, "0").map_err(ConversionError::Io)?; // Layer

            // First corner
            writeln!(buffer, "10").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v0.x).map_err(ConversionError::Io)?;
            writeln!(buffer, "20").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v0.y).map_err(ConversionError::Io)?;
            writeln!(buffer, "30").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v0.z).map_err(ConversionError::Io)?;

            // Second corner
            writeln!(buffer, "11").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v1.x).map_err(ConversionError::Io)?;
            writeln!(buffer, "21").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v1.y).map_err(ConversionError::Io)?;
            writeln!(buffer, "31").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v1.z).map_err(ConversionError::Io)?;

            // Third corner
            writeln!(buffer, "12").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v2.x).map_err(ConversionError::Io)?;
            writeln!(buffer, "22").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v2.y).map_err(ConversionError::Io)?;
            writeln!(buffer, "32").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v2.z).map_err(ConversionError::Io)?;

            // Fourth corner (same as third for triangles)
            writeln!(buffer, "13").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v2.x).map_err(ConversionError::Io)?;
            writeln!(buffer, "23").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v2.y).map_err(ConversionError::Io)?;
            writeln!(buffer, "33").map_err(ConversionError::Io)?;
            writeln!(buffer, "{:.6}", v2.z).map_err(ConversionError::Io)?;
        }

        // End ENTITIES section
        writeln!(buffer, "0").map_err(ConversionError::Io)?;
        writeln!(buffer, "ENDSEC").map_err(ConversionError::Io)?;

        // End of file
        writeln!(buffer, "0").map_err(ConversionError::Io)?;
        writeln!(buffer, "EOF").map_err(ConversionError::Io)?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a simple test mesh (a single triangle)
    fn create_test_triangle() -> Mesh {
        let mut mesh = Mesh::new();

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

        mesh.faces.push(Face { indices: [0, 1, 2] });

        mesh
    }

    #[test]
    fn test_dxf_format_new() {
        let format = DxfFormat::new();
        // Empty data should fail
        assert!(format.read(&[]).is_err());
    }

    #[test]
    fn test_read_invalid_dxf() {
        let format = DxfFormat::new();
        let invalid_data = b"not a valid DXF file";
        let result = format.read(invalid_data);
        // Should handle gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_read_empty_data() {
        let format = DxfFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_dxf_2d_only() {
        let format = DxfFormat::new();
        // Minimal DXF with only 2D entities (should handle gracefully)
        let dxf_data =
            b"0\nSECTION\n2\nHEADER\n0\nENDSEC\n0\nSECTION\n2\nENTITIES\n0\nENDSEC\n0\nEOF\n";
        let result = format.read(dxf_data);
        // Should fail because no 3D entities
        assert!(result.is_err());
    }

    #[test]
    fn test_write_simple_dxf() {
        let format = DxfFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let dxf_data = result.unwrap();
        let dxf_str = std::str::from_utf8(&dxf_data).unwrap();

        // Check DXF structure
        assert!(dxf_str.contains("SECTION"));
        assert!(dxf_str.contains("ENTITIES"));
        assert!(dxf_str.contains("3DFACE"));
        assert!(dxf_str.contains("EOF"));
    }

    #[test]
    fn test_write_dxf_3dface() {
        let format = DxfFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let dxf_data = result.unwrap();
        let dxf_str = std::str::from_utf8(&dxf_data).unwrap();

        // Should contain 3DFACE entity codes
        assert!(dxf_str.contains("3DFACE"));
        assert!(dxf_str.contains("10")); // X coordinate code
        assert!(dxf_str.contains("20")); // Y coordinate code
        assert!(dxf_str.contains("30")); // Z coordinate code
    }

    #[test]
    fn test_write_mesh_invalid_index() {
        let format = DxfFormat::new();
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
        let format = DxfFormat::new();
        let mesh = Mesh::new();

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    // Note: Testing actual DXF file reading requires sample DXF files with 3D entities
    // These would be integration tests with actual DXF test data
}
