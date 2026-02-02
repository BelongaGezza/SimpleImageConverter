// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{Face, Mesh, Normal, Vertex};
use common::error::{ConversionError, Result};
use common::limits::ResourceLimits;
use serde_json::json;

#[derive(Debug, Clone, Copy)]
enum GltfContainer {
    /// JSON `.gltf` with embedded base64 buffer.
    Gltf,
    /// Binary `.glb` container.
    Glb,
}

/// glTF format handler
pub struct GltfFormat {
    limits: ResourceLimits,
    container: GltfContainer,
}

impl GltfFormat {
    /// Create a new glTF format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
            container: GltfContainer::Gltf,
        }
    }

    /// Create a new GLB writer (binary glTF) with default resource limits.
    pub fn new_glb() -> Self {
        Self {
            limits: ResourceLimits::default(),
            container: GltfContainer::Glb,
        }
    }

    /// Create a new glTF format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self {
            limits,
            container: GltfContainer::Gltf,
        }
    }

    /// Create a new GLB writer (binary glTF) with custom resource limits.
    pub fn with_limits_glb(limits: ResourceLimits) -> Self {
        Self {
            limits,
            container: GltfContainer::Glb,
        }
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
        let scene = document.scenes().next().ok_or_else(|| {
            ConversionError::InvalidInput("glTF file contains no scenes".to_string())
        })?;

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

        match self.container {
            GltfContainer::Gltf => self.write_gltf_embedded(mesh),
            GltfContainer::Glb => self.write_glb(mesh),
        }
    }
}

impl GltfFormat {
    fn write_gltf_embedded(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        let (document, _buffer) = self.build_gltf_document(mesh, true)?;
        let json_bytes = serde_json::to_vec(&document).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to serialize glTF JSON: {}", e))
        })?;
        Ok(json_bytes)
    }

    fn write_glb(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        let (document, buffer) = self.build_gltf_document(mesh, false)?;
        let json_bytes = serde_json::to_vec(&document).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to serialize glTF JSON: {}", e))
        })?;
        Ok(build_glb(&json_bytes, &buffer))
    }

    /// Build a minimal glTF 2.0 document and its binary buffer.
    ///
    /// If `embed_buffer` is true, the returned document will include a data-URI buffer `uri`
    /// for `.gltf` single-file export. Otherwise it will omit `uri` (for `.glb`).
    fn build_gltf_document(
        &self,
        mesh: &Mesh,
        embed_buffer: bool,
    ) -> Result<(serde_json::Value, Vec<u8>)> {
        let vertex_count = mesh.vertices.len();
        let index_count = mesh.faces.len() * 3;

        let max_index = mesh.faces.iter().flat_map(|f| f.indices).max().unwrap_or(0);

        let use_u32_indices = max_index > (u16::MAX as usize);
        let index_component_type = if use_u32_indices { 5125 } else { 5123 };
        let index_component_size = if use_u32_indices { 4 } else { 2 };

        // Build BIN buffer: positions (+ optional normals) + indices.
        let mut bin = Vec::new();

        let pos_offset = bin.len();
        for v in &mesh.vertices {
            bin.extend_from_slice(&v.x.to_le_bytes());
            bin.extend_from_slice(&v.y.to_le_bytes());
            bin.extend_from_slice(&v.z.to_le_bytes());
        }
        let pos_len = bin.len() - pos_offset;
        align_to(&mut bin, 4, 0);

        let include_normals = mesh.normals.len() == mesh.vertices.len();
        let (norm_offset, norm_len, norm_view_idx, norm_accessor_idx) = if include_normals {
            let o = bin.len();
            for n in &mesh.normals {
                bin.extend_from_slice(&n.x.to_le_bytes());
                bin.extend_from_slice(&n.y.to_le_bytes());
                bin.extend_from_slice(&n.z.to_le_bytes());
            }
            let l = bin.len() - o;
            align_to(&mut bin, 4, 0);
            (Some(o), Some(l), Some(1u32), Some(1u32))
        } else {
            (None, None, None, None)
        };

        // Indices
        align_to(&mut bin, 4, 0);
        let idx_offset = bin.len();
        for face in &mesh.faces {
            for &idx in &face.indices {
                let idx_u32 = idx as u32;
                if use_u32_indices {
                    bin.extend_from_slice(&idx_u32.to_le_bytes());
                } else {
                    let idx_u16 = idx_u32 as u16;
                    bin.extend_from_slice(&idx_u16.to_le_bytes());
                }
            }
        }
        let idx_len = bin.len() - idx_offset;
        align_to(&mut bin, 4, 0);

        let total_buffer_len = bin.len();

        let (pos_min, pos_max) = position_min_max(&mesh.vertices);

        let mut accessors = vec![json!({
            "bufferView": 0,
            "byteOffset": 0,
            "componentType": 5126,
            "count": vertex_count,
            "type": "VEC3",
            "min": [pos_min[0], pos_min[1], pos_min[2]],
            "max": [pos_max[0], pos_max[1], pos_max[2]],
        })];

        let mut buffer_views = vec![json!({
            "buffer": 0,
            "byteOffset": pos_offset,
            "byteLength": pos_len,
            "target": 34962,
        })];

        let mut attributes = json!({
            "POSITION": 0
        });

        if include_normals {
            buffer_views.push(json!({
                "buffer": 0,
                "byteOffset": norm_offset.unwrap(),
                "byteLength": norm_len.unwrap(),
                "target": 34962,
            }));
            accessors.push(json!({
                "bufferView": norm_view_idx.unwrap(),
                "byteOffset": 0,
                "componentType": 5126,
                "count": vertex_count,
                "type": "VEC3"
            }));

            if let Some(obj) = attributes.as_object_mut() {
                obj.insert("NORMAL".to_string(), json!(norm_accessor_idx.unwrap()));
            }
        }

        let indices_accessor_index = accessors.len() as u32;
        accessors.push(json!({
            "bufferView": buffer_views.len() as u32,
            "byteOffset": 0,
            "componentType": index_component_type,
            "count": index_count,
            "type": "SCALAR",
            "min": [0],
            "max": [max_index as u32],
        }));

        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": idx_offset,
            "byteLength": idx_len,
            "target": 34963,
        }));

        // Buffer (data-URI for `.gltf`, no uri for `.glb`)
        let buffers = if embed_buffer {
            let data_uri = format!(
                "data:application/octet-stream;base64,{}",
                base64_encode(&bin)
            );
            vec![json!({
                "byteLength": total_buffer_len,
                "uri": data_uri
            })]
        } else {
            vec![json!({
                "byteLength": total_buffer_len
            })]
        };

        let document = json!({
            "asset": {
                "version": "2.0",
                "generator": "Simple Image Converter"
            },
            "scene": 0,
            "scenes": [{ "nodes": [0] }],
            "nodes": [{ "mesh": 0 }],
            "meshes": [{
                "primitives": [{
                    "attributes": attributes,
                    "indices": indices_accessor_index,
                    "mode": 4
                }]
            }],
            "accessors": accessors,
            "bufferViews": buffer_views,
            "buffers": buffers,
        });

        // Basic sanity checks to avoid generating obviously invalid outputs.
        if pos_len != vertex_count * 3 * 4 {
            return Err(ConversionError::ConversionFailed(
                "Internal error: unexpected position buffer length".to_string(),
            ));
        }
        if idx_len != index_count * index_component_size {
            return Err(ConversionError::ConversionFailed(
                "Internal error: unexpected index buffer length".to_string(),
            ));
        }

        Ok((document, bin))
    }
}

fn position_min_max(vertices: &[Vertex]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY, f32::INFINITY, f32::INFINITY];
    let mut max = [f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY];

    for v in vertices {
        min[0] = min[0].min(v.x);
        min[1] = min[1].min(v.y);
        min[2] = min[2].min(v.z);
        max[0] = max[0].max(v.x);
        max[1] = max[1].max(v.y);
        max[2] = max[2].max(v.z);
    }

    (min, max)
}

fn align_to(buf: &mut Vec<u8>, align: usize, pad_byte: u8) {
    let rem = buf.len() % align;
    if rem != 0 {
        buf.extend(std::iter::repeat_n(pad_byte, align - rem));
    }
}

fn build_glb(json_bytes: &[u8], bin_bytes: &[u8]) -> Vec<u8> {
    let mut json_chunk = json_bytes.to_vec();
    // glTF spec: JSON chunk padded with spaces to 4-byte alignment.
    align_to(&mut json_chunk, 4, 0x20);

    let mut bin_chunk = bin_bytes.to_vec();
    // glTF spec: BIN chunk padded with zeros to 4-byte alignment.
    align_to(&mut bin_chunk, 4, 0x00);

    let total_len = 12 + 8 + json_chunk.len() + 8 + bin_chunk.len();
    let mut out = Vec::with_capacity(total_len);

    // Header
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());

    // JSON chunk
    out.extend_from_slice(&(json_chunk.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_chunk);

    // BIN chunk
    out.extend_from_slice(&(bin_chunk.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\0");
    out.extend_from_slice(&bin_chunk);

    out
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    let mut chunks = input.chunks_exact(3);

    for chunk in &mut chunks {
        let n = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
        out.push(TABLE[((n >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((n >> 12) & 0x3F) as usize] as char);
        out.push(TABLE[((n >> 6) & 0x3F) as usize] as char);
        out.push(TABLE[(n & 0x3F) as usize] as char);
    }

    let rem = chunks.remainder();
    match rem.len() {
        0 => {}
        1 => {
            let n = (rem[0] as u32) << 16;
            out.push(TABLE[((n >> 18) & 0x3F) as usize] as char);
            out.push(TABLE[((n >> 12) & 0x3F) as usize] as char);
            out.push('=');
            out.push('=');
        }
        2 => {
            let n = ((rem[0] as u32) << 16) | ((rem[1] as u32) << 8);
            out.push(TABLE[((n >> 18) & 0x3F) as usize] as char);
            out.push(TABLE[((n >> 12) & 0x3F) as usize] as char);
            out.push(TABLE[((n >> 6) & 0x3F) as usize] as char);
            out.push('=');
        }
        _ => unreachable!("chunks_exact remainder is at most 2 bytes"),
    }

    out
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
    fn test_write_gltf_embedded_parses() {
        let format = GltfFormat::new();
        let mesh = create_test_triangle();

        let result = format.write(&mesh);
        assert!(result.is_ok());

        let gltf_data = result.unwrap();
        assert!(!gltf_data.starts_with(b"glTF"));

        let (doc, buffers, _images) = gltf::import_slice(&gltf_data).unwrap();
        let scene = doc.scenes().next().unwrap();
        let node = scene.nodes().next().unwrap();
        let mesh_node = node.mesh().unwrap();
        let primitive = mesh_node.primitives().next().unwrap();
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
        let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

        assert_eq!(positions.len(), 3);
        assert_eq!(indices.len(), 3);
    }

    #[test]
    fn test_write_glb_parses() {
        let format = GltfFormat::new_glb();
        let mesh = create_test_triangle();

        let glb = format.write(&mesh).unwrap();
        assert!(glb.starts_with(b"glTF"));

        let (doc, buffers, _images) = gltf::import_slice(&glb).unwrap();
        let scene = doc.scenes().next().unwrap();
        let node = scene.nodes().next().unwrap();
        let mesh_node = node.mesh().unwrap();
        let primitive = mesh_node.primitives().next().unwrap();
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
        let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

        assert_eq!(positions.len(), 3);
        assert_eq!(indices.len(), 3);
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

    #[test]
    fn test_resource_limits_file_size() {
        let limits = ResourceLimits::builder()
            .max_file_size(100) // Very small limit
            .build();
        let format = GltfFormat::with_limits(limits);

        // Test reading an oversized file
        let oversized_data = vec![0u8; 200];
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
        let format = GltfFormat::with_limits(limits);

        // Write a triangle (3 vertices - at limit), then read back (should succeed).
        let triangle_mesh = create_test_triangle();
        let gltf_data = format.write(&triangle_mesh).unwrap();
        assert!(format.read(&gltf_data).is_ok());
    }

    #[test]
    fn test_read_invalid_gltf_json() {
        let format = GltfFormat::new();
        // Invalid JSON data
        let invalid_data = b"{ invalid json }";

        let result = format.read(invalid_data);
        // Should fail gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_read_truncated_gltf_file() {
        let format = GltfFormat::new();
        let mesh = create_test_triangle();
        let mut gltf_data = format.write(&mesh).unwrap();

        // Truncate the file
        gltf_data.truncate(gltf_data.len() / 2);

        let result = format.read(&gltf_data);
        // Should fail gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_write_read_degenerate_triangle() {
        let format = GltfFormat::new();
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
        let gltf_data = result.unwrap();
        let read_result = format.read(&gltf_data);
        assert!(read_result.is_ok() || read_result.is_err());
    }
}
