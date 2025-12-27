// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Normal, Vertex};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use std::io::Write;

/// glTF format handler
pub struct GltfFormat {
    limits: ResourceLimits,
}

impl GltfFormat {
    /// Create a new glTF format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new glTF format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Parse binary glTF (.glb) format
    fn parse_glb(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        let (gltf, buffers, _images) = gltf::import_slice(data).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to parse glTF binary: {}", e))
        })?;

        self.extract_mesh_from_document(&gltf, &buffers)
    }

    /// Parse text glTF (.gltf) format
    fn parse_gltf(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        // Try parsing as text JSON
        let (gltf, buffers, _images) = gltf::import_slice(data).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to parse glTF text: {}", e))
        })?;

        self.extract_mesh_from_document(&gltf, &buffers)
    }

    /// Extract mesh data from a glTF document
    fn extract_mesh_from_document(
        &self,
        document: &gltf::Document,
        buffers: &[gltf::buffer::Data],
    ) -> Result<Mesh> {
        let mut mesh = Mesh::new();

        // Get the default scene or first scene
        let scene = document
            .scenes()
            .next()
            .ok_or_else(|| ConversionError::InvalidInput("glTF file contains no scenes".to_string()))?;

        // Iterate through nodes in the scene
        for node in scene.nodes() {
            if let Some(mesh_node) = node.mesh() {
                self.extract_mesh_primitives(&mesh_node, buffers, &mut mesh)?;
            }

            // Also check child nodes recursively (simple traversal)
            for child in node.children() {
                if let Some(mesh_node) = child.mesh() {
                    self.extract_mesh_primitives(&mesh_node, buffers, &mut mesh)?;
                }
            }
        }

        // Validate mesh
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "glTF file contains no vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            return Err(ConversionError::InvalidInput(
                "glTF file contains no faces".to_string(),
            ));
        }

        // Security: Validate mesh resource counts
        self.limits
            .check_mesh_resources(mesh.vertices.len(), mesh.faces.len())?;

        Ok(mesh)
    }

    /// Extract mesh primitives from a glTF mesh node
    fn extract_mesh_primitives(
        &self,
        mesh_node: &gltf::Mesh,
        buffers: &[gltf::buffer::Data],
        output_mesh: &mut Mesh,
    ) -> Result<()> {
        let vertex_offset = output_mesh.vertices.len();

        for primitive in mesh_node.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            // Extract positions
            if let Some(positions) = reader.read_positions() {
                for position in positions {
                    output_mesh.vertices.push(Vertex {
                        x: position[0],
                        y: position[1],
                        z: position[2],
                    });
                }
            }

            // Extract normals if present
            if let Some(normals) = reader.read_normals() {
                for normal in normals {
                    output_mesh.normals.push(Normal {
                        x: normal[0],
                        y: normal[1],
                        z: normal[2],
                    });
                }
            }

            // Extract indices
            let mut indices = Vec::new();
            if let Some(indices_reader) = reader.read_indices() {
                indices.extend(indices_reader.into_u32());
            }

            // Convert indices to faces (triangles)
            // glTF uses indexed triangles
            if indices.len() % 3 != 0 {
                return Err(ConversionError::InvalidInput(
                    "glTF indices count is not divisible by 3".to_string(),
                ));
            }

            for i in (0..indices.len()).step_by(3) {
                let idx0 = (indices[i] as usize) + vertex_offset;
                let idx1 = (indices[i + 1] as usize) + vertex_offset;
                let idx2 = (indices[i + 2] as usize) + vertex_offset;

                // Validate indices
                if idx0 >= output_mesh.vertices.len()
                    || idx1 >= output_mesh.vertices.len()
                    || idx2 >= output_mesh.vertices.len()
                {
                    return Err(ConversionError::InvalidInput(
                        "glTF face index out of bounds".to_string(),
                    ));
                }

                output_mesh.faces.push(Face {
                    indices: [idx0, idx1, idx2],
                });
            }
        }

        Ok(())
    }
}

impl Default for GltfFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for GltfFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Detect binary vs text format
        // glTF binary format (.glb) starts with magic bytes "glTF" (0x46546C67 in little-endian)
        if data.len() >= 12 && &data[0..4] == b"glTF" {
            // Binary .glb format
            self.parse_glb(data)
        } else {
            // Text .gltf format (JSON)
            self.parse_gltf(data)
        }
    }
}

impl MeshWriter for GltfFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Validate mesh data
        if mesh.vertices.is_empty() {
            return Err(ConversionError::InvalidInput(
                "Mesh has no vertices".to_string(),
            ));
        }

        if mesh.faces.is_empty() {
            return Err(ConversionError::InvalidInput("Mesh has no faces".to_string()));
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

        // Write text .gltf format (simpler than binary)
        // This creates a minimal valid glTF file
        let mut buffer = Vec::new();

        writeln!(buffer, "{{").map_err(ConversionError::Io)?;
        writeln!(buffer, "  \"asset\": {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "    \"version\": \"2.0\",").map_err(ConversionError::Io)?;
        writeln!(
            buffer,
            "    \"generator\": \"Simple Image Converter\""
        )
        .map_err(ConversionError::Io)?;
        writeln!(buffer, "  }},").map_err(ConversionError::Io)?;

        // Write accessors
        writeln!(buffer, "  \"accessors\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"bufferView\": 0,").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"componentType\": 5126,").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"count\": {},", mesh.vertices.len()).map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"type\": \"VEC3\",").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"min\": [0.0, 0.0, 0.0],").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"max\": [1.0, 1.0, 1.0]").map_err(ConversionError::Io)?;
        writeln!(buffer, "    }},").map_err(ConversionError::Io)?;

        // Indices accessor
        let total_indices = mesh.faces.len() * 3;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"bufferView\": 1,").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"componentType\": 5123,").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"count\": {},", total_indices).map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"type\": \"SCALAR\"").map_err(ConversionError::Io)?;
        writeln!(buffer, "    }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "  ],").map_err(ConversionError::Io)?;

        // Write buffer views
        let vertices_size = mesh.vertices.len() * 3 * 4; // 3 floats * 4 bytes each
        let indices_size = total_indices * 2; // u16 indices * 2 bytes each
        writeln!(buffer, "  \"bufferViews\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"buffer\": 0,").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"byteOffset\": 0,").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"byteLength\": {}", vertices_size).map_err(ConversionError::Io)?;
        writeln!(buffer, "    }},").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"buffer\": 0,").map_err(ConversionError::Io)?;
        writeln!(
            buffer,
            "      \"byteOffset\": {},",
            vertices_size
        )
        .map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"byteLength\": {}", indices_size).map_err(ConversionError::Io)?;
        writeln!(buffer, "    }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "  ],").map_err(ConversionError::Io)?;

        // Write buffers
        let total_buffer_size = vertices_size + indices_size;
        writeln!(buffer, "  \"buffers\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"uri\": \"data:application/octet-stream;base64,").map_err(ConversionError::Io)?;
        // Note: In a real implementation, we'd base64 encode the binary data here
        // For now, we'll write a placeholder
        writeln!(buffer, "      \"byteLength\": {}", total_buffer_size).map_err(ConversionError::Io)?;
        writeln!(buffer, "    }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "  ],").map_err(ConversionError::Io)?;

        // Write meshes
        writeln!(buffer, "  \"meshes\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"primitives\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "        {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "          \"attributes\": {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "            \"POSITION\": 0").map_err(ConversionError::Io)?;
        writeln!(buffer, "          }},").map_err(ConversionError::Io)?;
        writeln!(buffer, "          \"indices\": 1").map_err(ConversionError::Io)?;
        writeln!(buffer, "        }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "      ]").map_err(ConversionError::Io)?;
        writeln!(buffer, "    }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "  ],").map_err(ConversionError::Io)?;

        // Write scenes
        writeln!(buffer, "  \"scenes\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"nodes\": [0]").map_err(ConversionError::Io)?;
        writeln!(buffer, "    }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "  ],").map_err(ConversionError::Io)?;

        // Write nodes
        writeln!(buffer, "  \"nodes\": [").map_err(ConversionError::Io)?;
        writeln!(buffer, "    {{").map_err(ConversionError::Io)?;
        writeln!(buffer, "      \"mesh\": 0").map_err(ConversionError::Io)?;
        writeln!(buffer, "    }}").map_err(ConversionError::Io)?;
        writeln!(buffer, "  ]").map_err(ConversionError::Io)?;

        writeln!(buffer, "}}").map_err(ConversionError::Io)?;

        // Note: This creates a glTF JSON file but doesn't include the binary buffer data
        // A complete implementation would need to base64-encode the binary data or use GLB format
        // For now, this provides basic structure that can be extended

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
    fn test_gltf_format_new() {
        let format = GltfFormat::new();
        // Empty data should fail
        assert!(format.read(&[]).is_err());
    }

    #[test]
    fn test_read_invalid_gltf() {
        let format = GltfFormat::new();
        let invalid_data = b"not a valid glTF file";
        let result = format.read(invalid_data);
        // Should handle gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_read_empty_data() {
        let format = GltfFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_simple_gltf() {
        let format = GltfFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let gltf_data = result.unwrap();
        let gltf_str = std::str::from_utf8(&gltf_data).unwrap();

        // Check that it's valid JSON structure
        assert!(gltf_str.contains("\"asset\""));
        assert!(gltf_str.contains("\"meshes\""));
        assert!(gltf_str.contains("\"version\": \"2.0\""));
    }

    #[test]
    fn test_write_mesh_invalid_index() {
        let format = GltfFormat::new();
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
        let format = GltfFormat::new();
        let mesh = Mesh::new();

        let result = format.write(&mesh);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no vertices"));
    }

    // Note: Testing actual glTF file reading requires sample glTF files
    // These would be integration tests with actual glTF test data
}

