// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

#[cfg(feature = "step")]
use crate::formats::traits::{MeshReader, MeshWriter};
#[cfg(feature = "step")]
use crate::mesh::Mesh;
#[cfg(feature = "step")]
use common::error::{ConversionError, Result};
#[cfg(feature = "step")]
use common::limits::ResourceLimits;
// Note: We're implementing FACETED_BREP extraction directly (no truck Shell conversion)
// Truck dependencies are kept for potential future use (v0.3.0 opencascade-rs integration)
// ruststep for STEP file parsing
#[cfg(feature = "step")]
use ruststep::parser;
// AP203 types for entity deserialization
#[cfg(feature = "step")]
use ruststep::ap203::config_control_design::Tables;
// TableInit trait for populating Tables from Exchange.data
#[cfg(feature = "step")]
use ruststep::tables::TableInit;
// IntoOwned trait for resolving entity references (used in FACETED_BREP extraction)
#[cfg(feature = "step")]
use ruststep::tables::IntoOwned;

/// STEP format handler
///
/// Supports reading STEP files using the truck library.
/// STEP writing is not yet supported as it requires complex CAD modeling.
#[cfg(feature = "step")]
pub struct StepFormat {
    limits: ResourceLimits,
}

#[cfg(feature = "step")]
impl StepFormat {
    /// Create a new STEP format handler with default resource limits
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Create a new STEP format handler with custom resource limits
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Extract FACETED_BREP entities from AP203 Tables and convert to Mesh
    ///
    /// This method implements the architect-approved approach for v0.2.0:
    /// Direct extraction of FACETED_BREP entities (pre-tessellated geometry)
    /// without requiring truck Shell conversion.
    ///
    /// Entity traversal path:
    /// FACETED_BREP → CLOSED_SHELL → FACE → FACE_BOUND → EDGE_LOOP →
    /// ORIENTED_EDGE → EDGE → VERTEX_POINT → CARTESIAN_POINT
    fn extract_faceted_brep(&self, tables: &Tables) -> Result<Mesh> {
        // Check for FACETED_BREP entities first (v0.2.0 supported format)
        // Also check for other entity types to provide better error messages
        let msb_holders = tables.manifold_solid_brep_holders();
        let cs_holders = tables.closed_shell_holders();
        let fb_holders = tables.faceted_brep_holders();

        if fb_holders.is_empty() {
            // Check if file has other entity types that aren't supported
            if !msb_holders.is_empty() || !cs_holders.is_empty() {
                return Err(ConversionError::ConversionFailed(
                    "STEP file contains MANIFOLD_SOLID_BREP or CLOSED_SHELL entities, but no FACETED_BREP entities. \
                     For v0.2.0, only FACETED_BREP (pre-tessellated) geometry is supported. \
                     \
                     Your file likely contains curved surfaces (NURBS, cylinders, spheres, etc.) which require \
                     full B-Rep support (planned for v0.3.0). \
                     \
                     SOLUTION: Please export your STEP file with tessellation enabled to create FACETED_BREP entities. \
                     See docs/CAD_EXPORT_GUIDE.md for CAD software-specific instructions."
                        .to_string(),
                ));
            } else {
                return Err(ConversionError::ConversionFailed(
                    "STEP file contains no supported geometric entities. \
                     For v0.2.0, only FACETED_BREP (pre-tessellated) geometry is supported. \
                     \
                     SOLUTION: Please export your STEP file with tessellation enabled. \
                     See docs/CAD_EXPORT_GUIDE.md for CAD software-specific instructions."
                        .to_string(),
                ));
            }
        }

        // Extract geometry from FACETED_BREP entities
        // Entity traversal path:
        // FACETED_BREP → CLOSED_SHELL → FACE → FACE_BOUND → EDGE_LOOP →
        // ORIENTED_EDGE → EDGE → VERTEX_POINT → CARTESIAN_POINT

        let mut all_vertices = Vec::new();
        let mut all_faces = Vec::new();
        // Use ordered floats for deduplication (wrap in a newtype for hashing)
        let mut vertex_map = std::collections::HashMap::<[i64; 3], usize>::new();

        // Iterate through all FACETED_BREP entities
        for (id, holder) in fb_holders.iter() {
            // Resolve FACETED_BREP entity (fully resolve all references)
            // into_owned() returns the entity with all nested references resolved
            let faceted_brep = holder.clone().into_owned(tables).map_err(|e| {
                ConversionError::ConversionFailed(format!(
                    "Failed to resolve FACETED_BREP entity #{}: {:?}. \
                     This may indicate a corrupted or incomplete STEP file.",
                    id, e
                ))
            })?;

            // Get the outer CLOSED_SHELL directly from the resolved entity
            // into_owned() already resolved all nested references, so we can traverse directly
            let closed_shell = self.get_closed_shell_from_faceted_brep(&faceted_brep);

            // Extract faces from CLOSED_SHELL
            self.extract_faces_from_shell(
                closed_shell,
                &mut all_vertices,
                &mut all_faces,
                &mut vertex_map,
            )?;
        }

        // Validate that we extracted geometry
        if all_vertices.is_empty() {
            return Err(ConversionError::ConversionFailed(
                "No vertices extracted from FACETED_BREP entities. \
                 The STEP file may contain FACETED_BREP entities but no extractable geometry. \
                 This may indicate a corrupted or unsupported STEP file structure."
                    .to_string(),
            ));
        }

        if all_faces.is_empty() {
            return Err(ConversionError::ConversionFailed(
                "No faces extracted from FACETED_BREP entities. \
                 The STEP file may contain FACETED_BREP entities but no extractable faces. \
                 This may indicate a corrupted or unsupported STEP file structure."
                    .to_string(),
            ));
        }

        // Calculate normals for all faces
        let normals = self.calculate_normals(&all_vertices, &all_faces);

        // Build final mesh
        let mesh = Mesh {
            vertices: all_vertices,
            faces: all_faces,
            normals,
        };

        // Validate mesh using existing validation function
        crate::mesh::validate::validate_mesh(&mesh)?;

        Ok(mesh)
    }

    /// Parse STEP file and convert to mesh
    fn parse_step(&self, data: &[u8]) -> Result<Mesh> {
        // Security: Validate input size BEFORE parsing
        if let Err(e) = self.limits.check_file_size(data.len()) {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        // Convert bytes to string (STEP files are ASCII)
        let step_text = std::str::from_utf8(data).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "STEP file is not valid UTF-8 (file size: {} bytes). \
                 STEP files must be ASCII text format (ISO 10303-21). \
                 Error: {} \
                 \
                 The file may be corrupted or in a different format.",
                data.len(),
                e
            ))
        })?;

        // Parse STEP file using ruststep
        let exchange = parser::parse(step_text).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to parse STEP file: {} \
                 \
                 The file may be corrupted, incomplete, or not a valid STEP file (ISO 10303-21 format). \
                 Please verify the file is a valid STEP file and try again.",
                e
            ))
        })?;

        // Build AP203 Tables from Exchange.data for entity deserialization
        // Tables allows us to deserialize Records into AP203 structs and resolve references
        // Using TableInit::from_data_sections() to populate Tables from parsed STEP data
        let tables = Tables::from_data_sections(&exchange.data).map_err(|e| {
            ConversionError::ConversionFailed(format!(
                "Failed to deserialize STEP entities into AP203 Tables: {:?} \
                 \
                 This may indicate: \
                 - The file uses an unsupported Application Protocol (AP203 is supported, AP214/AP242 may have limited support) \
                 - Schema mismatch or incompatible STEP variant \
                 - Corrupted or malformed entity data \
                 \
                 Please verify the file is a valid AP203 STEP file and try again.",
                e
            ))
        })?;

        // Extract FACETED_BREP entities and convert directly to Mesh (v0.2.0 approach)
        // This bypasses truck Shell conversion as approved by the architect
        let mesh = self.extract_faceted_brep(&tables)?;

        // Security: Validate resource usage
        if let Err(e) = self
            .limits
            .check_mesh_resources(mesh.vertices.len(), mesh.faces.len())
        {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        Ok(mesh)
    }

    /// Get CLOSED_SHELL from FACETED_BREP
    ///
    /// This is a helper method to extract the `outer` field from FacetedBrep.
    /// ruststep API: FacetedBrep -> ManifoldSolidBrep -> ClosedShellAny -> ClosedShell
    #[cfg(feature = "step")]
    fn get_closed_shell_from_faceted_brep<'a>(
        &self,
        faceted_brep: &'a ruststep::ap203::config_control_design::FacetedBrep,
    ) -> &'a ruststep::ap203::config_control_design::ClosedShell {
        use ruststep::ap203::config_control_design::ClosedShellAny;

        // FacetedBrep has: manifold_solid_brep: ManifoldSolidBrep
        // ManifoldSolidBrep has: outer: ClosedShellAny
        let closed_shell_any = &faceted_brep.manifold_solid_brep.outer;

        // ClosedShellAny is an enum with variants:
        // - ClosedShell(Box<ClosedShell>)
        // - OrientedClosedShell(Box<OrientedClosedShell>)
        match closed_shell_any {
            ClosedShellAny::ClosedShell(cs) => cs.as_ref(),
            ClosedShellAny::OrientedClosedShell(ocs) => {
                // OrientedClosedShell implements Deref<Target = ClosedShell>
                // It also has a closed_shell field we can access directly
                &ocs.closed_shell
            }
        }
    }

    /// Extract faces from CLOSED_SHELL and build mesh data
    ///
    /// Traverses the CLOSED_SHELL → FACE → FACE_BOUND → EDGE_LOOP → vertices
    /// and builds the mesh vertex/face data.
    #[cfg(feature = "step")]
    fn extract_faces_from_shell(
        &self,
        closed_shell: &ruststep::ap203::config_control_design::ClosedShell,
        vertices: &mut Vec<crate::mesh::Vertex>,
        faces: &mut Vec<crate::mesh::Face>,
        vertex_map: &mut std::collections::HashMap<[i64; 3], usize>,
    ) -> Result<()> {
        use ruststep::ap203::config_control_design::FaceBoundAny;

        // Access cfs_faces from CLOSED_SHELL
        // ClosedShell has: connected_face_set: ConnectedFaceSet
        // ConnectedFaceSet has: cfs_faces: Vec<FaceAny>
        let face_list = &closed_shell.connected_face_set.cfs_faces;

        // Iterate through FACE entities
        for face_any in face_list {
            // Get the Face from FaceAny (may be Face, FaceSurface, or OrientedFace)
            // FaceAny implements AsRef<Face>, so we can just use as_ref()
            let face: &ruststep::ap203::config_control_design::Face = face_any.as_ref();

            // For each FACE, access `bounds` (Vec<FaceBoundAny>)
            // In STEP files, a FACE has:
            // - One outer bound (FaceOuterBound) - defines the face perimeter
            // - Zero or more inner bounds (FaceBound) - defines holes in the face
            // For v0.2.0, we only process the outer bound. Hole handling is planned for v0.3.0.
            let mut outer_bound_found = false;
            for face_bound_any in &face.bounds {
                match face_bound_any {
                    FaceBoundAny::FaceOuterBound(fob) => {
                        // Process outer bound only
                        if outer_bound_found {
                            return Err(ConversionError::ConversionFailed(
                                "Face has multiple outer bounds - invalid STEP file structure. \
                                 Each face should have exactly one outer bound."
                                    .to_string(),
                            ));
                        }
                        outer_bound_found = true;

                        // Get the bound (LoopAny) from FaceBound
                        let loop_any = &fob.face_bound.bound;

                        // Extract vertices from the loop
                        let face_vertex_indices =
                            self.extract_vertices_from_loop(loop_any, vertices, vertex_map)?;

                        // Build face indices (triangulate if needed)
                        // FACETED_BREP should have triangular faces, but we handle polygons too
                        if face_vertex_indices.len() >= 3 {
                            // Fan triangulation for polygons
                            for i in 1..(face_vertex_indices.len() - 1) {
                                faces.push(crate::mesh::Face {
                                    indices: [
                                        face_vertex_indices[0],
                                        face_vertex_indices[i],
                                        face_vertex_indices[i + 1],
                                    ],
                                });
                            }
                        }
                    }
                    FaceBoundAny::FaceBound(_) => {
                        // Skip inner bounds (holes) for v0.2.0
                        // Inner bounds define holes in faces, which require more complex handling
                        // This is planned for v0.3.0
                        continue;
                    }
                }
            }

            // Validate that we found an outer bound
            if !outer_bound_found {
                return Err(ConversionError::ConversionFailed(
                    "Face has no outer bound - invalid STEP file structure. \
                     Each face must have exactly one outer bound."
                        .to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Extract vertices from a loop (EdgeLoop, PolyLoop, etc.)
    #[cfg(feature = "step")]
    fn extract_vertices_from_loop(
        &self,
        loop_any: &ruststep::ap203::config_control_design::LoopAny,
        vertices: &mut Vec<crate::mesh::Vertex>,
        vertex_map: &mut std::collections::HashMap<[i64; 3], usize>,
    ) -> Result<Vec<usize>> {
        use ruststep::ap203::config_control_design::LoopAny;

        match loop_any {
            LoopAny::EdgeLoop(el) => {
                // EdgeLoop has: path: Path, which has: edge_list: Vec<OrientedEdge>
                self.extract_vertices_from_edge_loop(el.as_ref(), vertices, vertex_map)
            }
            LoopAny::PolyLoop(pl) => {
                // PolyLoop has direct polygon points
                self.extract_vertices_from_poly_loop(pl.as_ref(), vertices, vertex_map)
            }
            LoopAny::VertexLoop(_vl) => {
                // VertexLoop represents a single vertex, not a face boundary
                // This cannot form a valid face - return error
                Err(ConversionError::ConversionFailed(
                    "Face bound uses VertexLoop which cannot form a face boundary. \
                     VertexLoop represents a single vertex, not a closed loop. \
                     This may indicate an invalid or unsupported STEP file structure."
                        .to_string(),
                ))
            }
            LoopAny::Loop(_) => {
                // Base Loop type - not enough info to extract vertices
                Ok(vec![])
            }
        }
    }

    /// Extract vertices from an EdgeLoop
    #[cfg(feature = "step")]
    fn extract_vertices_from_edge_loop(
        &self,
        edge_loop: &ruststep::ap203::config_control_design::EdgeLoop,
        vertices: &mut Vec<crate::mesh::Vertex>,
        vertex_map: &mut std::collections::HashMap<[i64; 3], usize>,
    ) -> Result<Vec<usize>> {
        let mut face_vertex_indices = Vec::new();

        // EdgeLoop has: path: Path
        // Path has: edge_list: Vec<OrientedEdge>
        for oriented_edge in &edge_loop.path.edge_list {
            // OrientedEdge has: edge: Edge, edge_element: EdgeAny, orientation: bool
            // Use the edge field which has edge_start and edge_end
            let edge = &oriented_edge.edge;

            // Extract start vertex (we only need one vertex per edge to avoid duplicates)
            // The vertices will connect to form the loop
            let start_vertex = if oriented_edge.orientation {
                &edge.edge_start
            } else {
                &edge.edge_end
            };

            let coords = self.extract_vertex_coords(start_vertex)?;
            let idx = self.add_vertex_with_dedup(coords, vertex_map, vertices);
            face_vertex_indices.push(idx);
        }

        Ok(face_vertex_indices)
    }

    /// Extract vertices from a PolyLoop (polygon with direct point list)
    #[cfg(feature = "step")]
    fn extract_vertices_from_poly_loop(
        &self,
        poly_loop: &ruststep::ap203::config_control_design::PolyLoop,
        vertices: &mut Vec<crate::mesh::Vertex>,
        vertex_map: &mut std::collections::HashMap<[i64; 3], usize>,
    ) -> Result<Vec<usize>> {
        let mut face_vertex_indices = Vec::new();

        // PolyLoop has: polygon: Vec<CartesianPoint>
        for cartesian_point in &poly_loop.polygon {
            let coords = self.extract_cartesian_point_coords(cartesian_point)?;
            let idx = self.add_vertex_with_dedup(coords, vertex_map, vertices);
            face_vertex_indices.push(idx);
        }

        Ok(face_vertex_indices)
    }

    /// Extract coordinates from a VertexAny
    #[cfg(feature = "step")]
    fn extract_vertex_coords(
        &self,
        vertex_any: &ruststep::ap203::config_control_design::VertexAny,
    ) -> Result<(f64, f64, f64)> {
        use ruststep::ap203::config_control_design::{PointAny, VertexAny};

        match vertex_any {
            VertexAny::VertexPoint(vp) => {
                // VertexPoint has: vertex_geometry: PointAny
                match &vp.vertex_geometry {
                    PointAny::CartesianPoint(cp) => {
                        self.extract_cartesian_point_coords(cp.as_ref())
                    }
                    _ => {
                        // Other point types - try to get coordinates if possible
                        Err(ConversionError::ConversionFailed(
                            "Unsupported point type in vertex geometry".to_string(),
                        ))
                    }
                }
            }
            VertexAny::Vertex(_) => {
                // Base Vertex type has no geometry
                Err(ConversionError::ConversionFailed(
                    "Vertex has no geometry information".to_string(),
                ))
            }
        }
    }

    /// Extract coordinates from a CartesianPoint
    #[cfg(feature = "step")]
    fn extract_cartesian_point_coords(
        &self,
        cp: &ruststep::ap203::config_control_design::CartesianPoint,
    ) -> Result<(f64, f64, f64)> {
        // CartesianPoint has: coordinates: Vec<LengthMeasure>
        // LengthMeasure is a tuple struct wrapping f64
        // STEP spec requires 2-3 coordinates (2D or 3D point)
        let coords = &cp.coordinates;

        if coords.is_empty() {
            return Err(ConversionError::ConversionFailed(
                "CartesianPoint has no coordinates - invalid STEP file structure. \
                 CartesianPoint must have at least 2 coordinates (X, Y)."
                    .to_string(),
            ));
        }

        if coords.len() < 2 {
            return Err(ConversionError::ConversionFailed(format!(
                "CartesianPoint has only {} coordinate(s), expected 2-3. \
                 Invalid STEP file structure.",
                coords.len()
            )));
        }

        let x = coords[0].0;
        let y = coords[1].0;
        // Z defaults to 0.0 for 2D points (STEP allows 2D CartesianPoint)
        let z = coords.get(2).map(|lm| lm.0).unwrap_or(0.0);

        Ok((x, y, z))
    }

    /// Add a vertex with deduplication
    ///
    /// Uses a hash map to deduplicate vertices with the same coordinates.
    /// Coordinates are converted to integers (scaled by 1e6) for hashing.
    #[cfg(feature = "step")]
    fn add_vertex_with_dedup(
        &self,
        coords: (f64, f64, f64),
        vertex_map: &mut std::collections::HashMap<[i64; 3], usize>,
        vertices: &mut Vec<crate::mesh::Vertex>,
    ) -> usize {
        // Convert to integer key for hashing (scale by 1e6 for precision)
        // This handles floating point comparison issues
        const SCALE: f64 = 1_000_000.0;
        let key = [
            (coords.0 * SCALE).round() as i64,
            (coords.1 * SCALE).round() as i64,
            (coords.2 * SCALE).round() as i64,
        ];

        *vertex_map.entry(key).or_insert_with(|| {
            let idx = vertices.len();
            vertices.push(crate::mesh::Vertex {
                x: coords.0 as f32,
                y: coords.1 as f32,
                z: coords.2 as f32,
            });
            idx
        })
    }

    /// Calculate normals for all faces
    #[cfg(feature = "step")]
    fn calculate_normals(
        &self,
        vertices: &[crate::mesh::Vertex],
        faces: &[crate::mesh::Face],
    ) -> Vec<crate::mesh::Normal> {
        // Use existing normal calculation from mesh module
        let mut normals = vec![
            crate::mesh::Normal {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            vertices.len()
        ];

        for face in faces {
            let v0 = &vertices[face.indices[0]];
            let v1 = &vertices[face.indices[1]];
            let v2 = &vertices[face.indices[2]];

            // Calculate face normal using cross product
            let dx1 = v1.x - v0.x;
            let dy1 = v1.y - v0.y;
            let dz1 = v1.z - v0.z;
            let dx2 = v2.x - v0.x;
            let dy2 = v2.y - v0.y;
            let dz2 = v2.z - v0.z;

            let nx = dy1 * dz2 - dz1 * dy2;
            let ny = dz1 * dx2 - dx1 * dz2;
            let nz = dx1 * dy2 - dy1 * dx2;

            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            if len > 0.0 {
                let inv_len = 1.0 / len;
                let normal = crate::mesh::Normal {
                    x: nx * inv_len,
                    y: ny * inv_len,
                    z: nz * inv_len,
                };

                // Add normal to each vertex (for smooth shading)
                normals[face.indices[0]].x += normal.x;
                normals[face.indices[0]].y += normal.y;
                normals[face.indices[0]].z += normal.z;
                normals[face.indices[1]].x += normal.x;
                normals[face.indices[1]].y += normal.y;
                normals[face.indices[1]].z += normal.z;
                normals[face.indices[2]].x += normal.x;
                normals[face.indices[2]].y += normal.y;
                normals[face.indices[2]].z += normal.z;
            }
        }

        // Normalize accumulated normals
        for normal in &mut normals {
            let len = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            if len > 0.0 {
                let inv_len = 1.0 / len;
                normal.x *= inv_len;
                normal.y *= inv_len;
                normal.z *= inv_len;
            }
        }

        normals
    }
}

#[cfg(feature = "step")]
impl Default for StepFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "step")]
impl MeshReader for StepFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Strategy 1: Try FACETED_BREP first (pure Rust, fast, always available)
        match self.parse_step(data) {
            Ok(mesh) => return Ok(mesh),
            Err(e) => {
                // Check if error indicates curved surfaces (MANIFOLD_SOLID_BREP)
                // If so, try opencascade-rs fallback (if available)
                let error_msg = e.to_string();
                if error_msg.contains("MANIFOLD_SOLID_BREP")
                    || error_msg.contains("curved surfaces")
                    || error_msg.contains("NURBS")
                {
                    // Try opencascade-rs fallback
                    #[cfg(feature = "step-opencascade")]
                    {
                        if let Ok(mesh) = self.extract_with_opencascade(data) {
                            return Ok(mesh);
                        }
                        // If opencascade-rs also fails, return original error with additional context
                        return Err(ConversionError::ConversionFailed(format!(
                            "{}\n\n\
                             Attempted opencascade-rs fallback but it also failed. \
                             This may indicate: \
                             - OCCT is not installed or not found \
                             - The STEP file is corrupted or invalid \
                             - opencascade-rs integration needs further development",
                            error_msg
                        )));
                    }

                    // If opencascade-rs not available, return original error with suggestion
                    return Err(ConversionError::ConversionFailed(format!(
                        "{}\n\n\
                         SOLUTION: Build with --features step-opencascade for full B-Rep support. \
                         This requires OpenCASCADE Technology (OCCT) 7.7+ to be installed. \
                         See RESEARCH_OPENCASCADE_RS_SPRINT9.md for installation instructions.",
                        error_msg
                    )));
                }
                // For other errors (parsing, validation, etc.), return as-is
                return Err(e);
            }
        }
    }
}

#[cfg(feature = "step")]
impl StepFormat {
    /// Extract mesh using opencascade-rs (fallback for curved surfaces)
    ///
    /// This method is called when FACETED_BREP extraction fails due to curved surfaces.
    /// It requires the step-opencascade feature to be enabled and OCCT to be installed.
    #[cfg(feature = "step-opencascade")]
    fn extract_with_opencascade(&self, data: &[u8]) -> Result<Mesh> {
        use crate::formats::step_opencascade;

        // Use default tessellation quality (0.01 = 1% of bounding box)
        // TODO: Make this configurable via ConversionOptions
        const DEFAULT_DEFLECTION: f64 = 0.01;

        step_opencascade::extract_mesh(data, &self.limits, DEFAULT_DEFLECTION)
    }
}

#[cfg(feature = "step")]
impl MeshWriter for StepFormat {
    fn write(&self, _mesh: &Mesh) -> Result<Vec<u8>> {
        // STEP writing requires complex CAD modeling capabilities
        // truck library focuses on reading, not writing
        Err(ConversionError::UnsupportedFormat(
            "STEP writing is not supported. \
             STEP files require complex CAD modeling capabilities (B-Rep reconstruction, parametric surfaces, etc.) \
             that are beyond the scope of this converter. \
             \
             This converter focuses on reading and converting STEP files to mesh formats (STL, OBJ, PLY, etc.). \
             To create STEP files, please use CAD software (SolidWorks, FreeCAD, Fusion 360, etc.)."
                .to_string()
        ))
    }
}

#[cfg(test)]
#[cfg(feature = "step")]
mod tests {
    use super::*;

    #[test]
    fn test_step_format_new() {
        let _format = StepFormat::new();
        // Just verify it can be created (no panic)
    }

    #[test]
    fn test_step_format_with_limits() {
        let limits = ResourceLimits::default();
        let _format = StepFormat::with_limits(limits);
        // Just verify it can be created (no panic)
    }

    #[test]
    fn test_read_empty_data() {
        let format = StepFormat::new();
        let result = format.read(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_invalid_utf8() {
        let format = StepFormat::new();
        let invalid_data = [0xFF, 0xFE, 0xFD];
        let result = format.read(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_unsupported() {
        let format = StepFormat::new();
        let mesh = Mesh::new();
        let result = format.write(&mesh);
        assert!(result.is_err());
        if let Err(ConversionError::UnsupportedFormat(_)) = result {
            // Expected error
        } else {
            panic!("Expected UnsupportedFormat error");
        }
    }
}
