// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! opencascade-rs backend for STEP file reading
//!
//! This module provides full STEP B-Rep support using opencascade-rs (OpenCASCADE Technology).
//! It can handle MANIFOLD_SOLID_BREP entities with curved surfaces (NURBS, cylinders, spheres, etc.)
//! that cannot be handled by the pure Rust FACETED_BREP extraction.
//!
//! **Requirements:**
//! - OpenCASCADE Technology (OCCT) 7.7+ must be installed on the system
//! - See RESEARCH_OPENCASCADE_RS_SPRINT9.md for installation instructions
//!
//! **Implementation Status:** Full implementation for Sprint 10 (v0.3.0).

#[cfg(feature = "step-opencascade")]
use crate::mesh::{Face, Mesh, Normal, Vertex};
#[cfg(feature = "step-opencascade")]
use common::error::{ConversionError, Result};
#[cfg(feature = "step-opencascade")]
use common::limits::ResourceLimits;

/// Extract mesh from STEP file using opencascade-rs
///
/// This function reads a STEP file using OpenCASCADE Technology, tessellates any curved surfaces,
/// and extracts the resulting mesh data.
///
/// # Arguments
/// * `data` - STEP file data as bytes
/// * `limits` - Resource limits for validation
/// * `deflection` - Tessellation quality (smaller = higher quality, default: 0.01)
///
/// # Returns
/// * `Result<Mesh>` - Extracted mesh or error
///
/// # Errors
/// * Returns `ConversionError` if STEP file cannot be read, tessellated, or if resource limits are exceeded
#[cfg(feature = "step-opencascade")]
pub fn extract_mesh(data: &[u8], limits: &ResourceLimits, deflection: f64) -> Result<Mesh> {
    // Security: Validate input size BEFORE processing
    limits.check_file_size(data.len())?;

    // Note: opencascade-rs typically expects a file path, not bytes
    // We need to write to a temporary file
    // TODO: Consider using in-memory approach if opencascade-rs supports it

    // Create temporary file (automatically cleaned up on drop, even on panic)
    let temp_file = tempfile::NamedTempFile::new().map_err(|e| {
        ConversionError::ConversionFailed(format!(
            "Failed to create temporary file: {}. \
             This may indicate a filesystem permission issue.",
            e
        ))
    })?;

    // Write STEP data to temporary file
    std::fs::write(temp_file.path(), data).map_err(|e| {
        ConversionError::ConversionFailed(format!(
            "Failed to write temporary STEP file: {}. \
             This may indicate a filesystem permission issue.",
            e
        ))
    })?;

    // Process file (temp_file automatically cleaned up when dropped, even on panic)
    extract_mesh_from_file(temp_file.path(), limits, deflection)
}

/// Extract mesh from STEP file path using opencascade-rs
///
/// Internal function that performs the actual OCCT processing.
#[cfg(feature = "step-opencascade")]
fn extract_mesh_from_file(
    file_path: &std::path::Path,
    limits: &ResourceLimits,
    deflection: f64,
) -> Result<Mesh> {
    // Implementation of OCCT STEP reading and tessellation
    // Based on opencascade-rs 0.2.0 API and OCCT patterns

    use opencascade::prelude::*;

    // Step 1: Read STEP file using STEPControl_Reader
    let mut reader = STEPControl_Reader::default();
    let file_path_str = file_path.to_string_lossy();

    let status = reader.read_file(&file_path_str).map_err(|e| {
        ConversionError::ConversionFailed(format!(
            "Failed to read STEP file with OpenCASCADE: {}. \
             The file may be corrupted, incomplete, or not a valid STEP file. \
             Ensure OCCT is properly installed and the file path is accessible.",
            e
        ))
    })?;

    // Check if reading was successful
    if status != IFSelect_ReturnStatus::IFSelect_RetDone {
        return Err(ConversionError::ConversionFailed(format!(
            "Failed to read STEP file with OpenCASCADE. \
             Status: {:?}. \
             The file may be corrupted, incomplete, or not a valid STEP file.",
            status
        )));
    }

    // Step 2: Transfer root entities
    let num_roots = reader.nb_roots();
    if num_roots == 0 {
        return Err(ConversionError::ConversionFailed(
            "STEP file contains no root entities. \
             The file may be empty or invalid."
                .to_string(),
        ));
    }

    // Transfer all root entities
    for i in 1..=num_roots {
        reader.transfer_root(i);
    }

    // Step 3: Get shape (combine all root shapes)
    let shape = reader.one_shape();

    // Step 4: Tessellate the shape
    // BRepMesh_IncrementalMesh performs tessellation
    // Deflection is relative to the bounding box size (0.01 = 1%)
    let mut mesher = BRepMesh_IncrementalMesh::new(&shape, deflection);
    mesher.perform();

    // Step 5: Extract triangulation data
    // This requires traversing the shape and extracting mesh data from each face
    let (vertices, faces) = extract_triangulation(&shape)?;

    // Validate we extracted some geometry
    if vertices.is_empty() || faces.is_empty() {
        return Err(ConversionError::ConversionFailed(
            "No geometry could be extracted from STEP file. \
             The file may contain unsupported geometry types or the tessellation failed."
                .to_string(),
        ));
    }

    // Step 6: Validate resource usage
    limits.check_mesh_resources(vertices.len(), faces.len())?;

    // Step 7: Calculate normals from geometry
    // Create a temporary mesh to calculate normals
    let temp_mesh = Mesh {
        vertices: vertices.clone(),
        faces: faces.clone(),
        normals: Vec::new(), // Will be calculated
    };

    let mesh_with_normals = crate::mesh::recalculate_normals(temp_mesh)?;

    // Step 8: Build final mesh
    Ok(mesh_with_normals)
}

/// Extract triangulation data from tessellated shape
///
/// This function traverses a tessellated TopoDS_Shape and extracts vertex and face data.
/// Normals are calculated separately using recalculate_normals.
///
/// # Arguments
/// * `shape` - Tessellated TopoDS_Shape from OCCT
///
/// # Returns
/// * `Result<(Vec<Vertex>, Vec<Face>)>` - Extracted mesh data (vertices and faces)
#[cfg(feature = "step-opencascade")]
fn extract_triangulation(
    shape: &opencascade::prelude::TopoDS_Shape,
) -> Result<(Vec<Vertex>, Vec<Face>)> {
    use opencascade::prelude::*;
    use std::collections::HashMap;

    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    // Use a map to deduplicate vertices based on coordinate precision
    // Key: quantized coordinates (i64 tuple)
    // Value: vertex index in vertices vector
    let mut vertex_map: HashMap<[i64; 3], usize> = HashMap::new();

    // Traverse shape faces
    let mut explorer = TopExp_Explorer::new(shape, TopAbs_ShapeEnum::TopAbs_FACE);

    while explorer.more() {
        let face = TopoDS::Face::down_cast(explorer.current()).map_err(|_| {
            ConversionError::ConversionFailed(
                "Failed to extract face from shape. This may indicate corrupted geometry."
                    .to_string(),
            )
        })?;

        // Get triangulation from face
        let location = face.location();
        let triangulation = BRep_Tool::triangulation(&face, &location);

        if let Some(tri) = triangulation {
            // Get the number of nodes and triangles
            let num_nodes = tri.nb_nodes();
            let num_triangles = tri.nb_triangles();

            // Extract and deduplicate vertices
            // Create a mapping from local face vertex indices to global vertex indices
            let mut local_to_global: Vec<usize> = Vec::with_capacity(num_nodes as usize);

            for i in 1..=num_nodes {
                // OCCT uses 1-based indexing
                let node_index = i as i32;
                let point = tri.node(node_index);

                let vertex = Vertex {
                    x: point.x() as f32,
                    y: point.y() as f32,
                    z: point.z() as f32,
                };

                // Deduplicate vertices using quantized coordinates
                // This helps merge vertices that are essentially the same (within floating point precision)
                const PRECISION: f32 = 1_000_000.0; // 6 decimal places
                let key = [
                    (vertex.x * PRECISION).round() as i64,
                    (vertex.y * PRECISION).round() as i64,
                    (vertex.z * PRECISION).round() as i64,
                ];

                let global_idx = *vertex_map.entry(key).or_insert_with(|| {
                    let idx = vertices.len();
                    vertices.push(vertex);
                    idx
                });

                local_to_global.push(global_idx);
            }

            // Extract triangles
            // OCCT triangle indices are 1-based and refer to nodes within this face's triangulation
            for i in 1..=num_triangles {
                let triangle_index = i as i32;
                let triangle = tri.triangle(triangle_index);

                // OCCT uses 1-based indexing for triangle vertices
                let i0_local = triangle.value(1) as usize - 1;
                let i1_local = triangle.value(2) as usize - 1;
                let i2_local = triangle.value(3) as usize - 1;

                // Validate indices
                if i0_local >= local_to_global.len()
                    || i1_local >= local_to_global.len()
                    || i2_local >= local_to_global.len()
                {
                    // Skip invalid triangles (shouldn't happen, but be defensive)
                    continue;
                }

                // Convert local indices to global indices
                let i0 = local_to_global[i0_local];
                let i1 = local_to_global[i1_local];
                let i2 = local_to_global[i2_local];

                // Skip degenerate triangles (all vertices same)
                if i0 == i1 || i1 == i2 || i0 == i2 {
                    continue;
                }

                faces.push(Face {
                    indices: [i0, i1, i2],
                });
            }
        }

        explorer.next();
    }

    // Validate we extracted some geometry
    if vertices.is_empty() {
        return Err(ConversionError::ConversionFailed(
            "No vertices could be extracted from STEP file. \
             The tessellation may have failed or the shape contains no geometry."
                .to_string(),
        ));
    }

    if faces.is_empty() {
        return Err(ConversionError::ConversionFailed(
            "No faces could be extracted from STEP file. \
             The tessellation may have failed or the shape contains no triangulated geometry."
                .to_string(),
        ));
    }

    Ok((vertices, faces))
}

#[cfg(test)]
#[cfg(feature = "step-opencascade")]
mod tests {
    use super::*;

    #[test]
    fn test_extract_mesh_empty_file() {
        // Test with minimal STEP file data (should fail with appropriate error)
        let data = b"ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;";
        let limits = ResourceLimits::default();
        let result = extract_mesh(data, &limits, 0.01);

        // Should return an error (either file read error or no geometry error)
        assert!(result.is_err());

        // Error should be ConversionFailed
        if let Err(ConversionError::ConversionFailed(_msg)) = result {
            // Expected - empty or invalid STEP file
        } else {
            panic!("Expected ConversionFailed error for empty STEP file");
        }
    }

    // Note: Full integration tests require:
    // 1. OCCT to be installed on the system
    // 2. Sample STEP files with curved surfaces (MANIFOLD_SOLID_BREP)
    // 3. OCCT build system to work correctly
    //
    // These tests should be added in integration test suite once OCCT is available.
}
