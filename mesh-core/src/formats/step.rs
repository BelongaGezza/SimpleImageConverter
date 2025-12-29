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
// Truck types for geometry and tessellation
#[cfg(feature = "step")]
use truck_modeling::Shell;
// Tessellation imports (for future use when implementing convert_truck_to_mesh)
// use truck_meshalgo::prelude::*;
// use truck_polymesh::PolygonMesh;
// use crate::mesh::{Face, Normal, Vertex};
// use nalgebra::Vector3;
// ruststep for STEP file parsing
#[cfg(feature = "step")]
use ruststep::parser;
// AP203 types for entity deserialization
#[cfg(feature = "step")]
use ruststep::ap203::config_control_design::Tables;
// TableInit trait for populating Tables from Exchange.data
#[cfg(feature = "step")]
use ruststep::tables::TableInit;
// IntoOwned trait for resolving entity references
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

    /// Extract geometric entities from AP203 Tables
    ///
    /// This method accesses the deserialized entities from Tables and attempts to
    /// extract geometry that can be converted to truck Shell objects.
    fn extract_entities_from_tables(&self, tables: &Tables) -> Result<Vec<Shell>> {
        let shells = Vec::new();

        // Check for MANIFOLD_SOLID_BREP entities
        // These are the main entity type for B-Rep solids
        let msb_holders = tables.manifold_solid_brep_holders();
        let msb_count = msb_holders.len();

        if msb_count > 0 {
            eprintln!("Found {} MANIFOLD_SOLID_BREP entities in Tables", msb_count);

            for (entity_id, holder) in msb_holders.iter() {
                eprintln!("  Entity #{}: ManifoldSolidBrep holder found", entity_id);

                // Try to resolve the holder into an owned ManifoldSolidBrep
                match holder.clone().into_owned(tables) {
                    Ok(_msb) => {
                        eprintln!("    ✓ Successfully resolved ManifoldSolidBrep");
                        // Now we have the resolved ManifoldSolidBrep (_msb)
                        // Next step: Convert to truck Shell (Task 2.4)
                        // This requires mapping AP203 geometry to truck geometry
                        // For now, just log success - conversion to come
                        eprintln!("    Note: Shell conversion not yet implemented");
                    }
                    Err(e) => {
                        eprintln!("    ✗ Failed to resolve ManifoldSolidBrep: {:?}", e);
                    }
                }
            }
        }

        // Check for CLOSED_SHELL entities (can exist independently)
        let cs_holders = tables.closed_shell_holders();
        let cs_count = cs_holders.len();

        if cs_count > 0 {
            eprintln!("Found {} CLOSED_SHELL entities in Tables", cs_count);

            for (entity_id, holder) in cs_holders.iter() {
                eprintln!("  Entity #{}: ClosedShell holder found", entity_id);

                match holder.clone().into_owned(tables) {
                    Ok(_cs) => {
                        eprintln!("    ✓ Successfully resolved ClosedShell");
                        // ClosedShell (_cs) contains faces that define the shell geometry
                        // This is the core data we need for tessellation (Task 2.4)
                        eprintln!("    Note: Shell conversion not yet implemented");
                    }
                    Err(e) => {
                        eprintln!("    ✗ Failed to resolve ClosedShell: {:?}", e);
                    }
                }
            }
        }

        // Log summary of what we found
        eprintln!("\nEntity extraction summary:");
        eprintln!("  - MANIFOLD_SOLID_BREP: {}", msb_count);
        eprintln!("  - CLOSED_SHELL: {}", cs_count);

        if shells.is_empty() && (msb_count > 0 || cs_count > 0) {
            // We found entities but couldn't convert them yet
            // CRITICAL FINDING: truck-stepio does not have input (reading) functionality yet
            // The "in" module is marked as "not yet implemented" in truck-stepio 0.3.0
            // See: https://docs.rs/truck-stepio/0.3.0/truck_stepio/
            //
            // Options to consider:
            // 1. Implement custom conversion from AP203 entities to truck Shell (very complex)
            // 2. Wait for truck-stepio input support (uncertain timeline)
            // 3. Use a different approach/library for STEP reading
            //
            // This is a significant architectural challenge that requires Senior Engineer input.
            eprintln!("\n⚠️ STEP Reading Limitation:");
            eprintln!("  Entities were successfully parsed and deserialized from STEP file.");
            eprintln!("  However, truck-stepio input functionality is not yet implemented.");
            eprintln!("  Converting AP203 entities to truck Shell requires custom implementation.");
            eprintln!("  This is a complex task that may require architectural review.");
        }

        Ok(shells)
    }

    /// Convert truck Shell objects to our Mesh format
    fn convert_truck_to_mesh(&self, _shells: Vec<Shell>) -> Result<Mesh> {
        // TODO: Implement tessellation using truck-meshalgo
        // The triangulation() method returns Shell<Point3, PolylineCurve, Option<PolygonMesh>>
        // We need to extract PolygonMesh from each face's surface Option<PolygonMesh>
        // This requires iterating through the shell's faces and collecting all PolygonMeshes

        Err(ConversionError::ConversionFailed(
            "Tessellation implementation in progress. \
            Shell tessellation requires extracting PolygonMesh from each face of the tessellated shell. \
            Entity conversion framework is in place, tessellation to be completed next.".to_string()
        ))

        // Implementation outline:
        // 1. For each shell: shell.triangulation(tolerance) -> Shell<Point3, PolylineCurve, Option<PolygonMesh>>
        // 2. Iterate through shell faces, extract Option<PolygonMesh> from each surface
        // 3. Collect all PolygonMeshes and merge them into a single mesh
        // 4. Convert to our Mesh format with vertices, faces, and normals
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
                "STEP file is not valid UTF-8 ({} bytes): {}",
                data.len(),
                e
            ))
        })?;

        // Parse STEP file using ruststep
        let exchange = parser::parse(step_text).map_err(|e| {
            ConversionError::ConversionFailed(format!("Failed to parse STEP file: {}", e))
        })?;

        // Build AP203 Tables from Exchange.data for entity deserialization
        // Tables allows us to deserialize Records into AP203 structs and resolve references
        // Using TableInit::from_data_sections() to populate Tables from parsed STEP data
        let tables = match Tables::from_data_sections(&exchange.data) {
            Ok(t) => t,
            Err(e) => {
                // If Tables construction fails, it might be due to schema mismatch
                // Log the error but continue with default tables for entity identification
                // This allows us to still parse and identify entities even if full deserialization fails
                eprintln!(
                    "Warning: Could not fully deserialize STEP entities into AP203 Tables: {:?}",
                    e
                );
                eprintln!("Falling back to entity identification mode (limited functionality)");
                Tables::default()
            }
        };

        // Extract geometric entities from Tables (new approach using deserialized entities)
        // This uses the AP203 Tables for proper entity deserialization and reference resolution
        let shells = self.extract_entities_from_tables(&tables)?;

        // Check if we found any shells
        if shells.is_empty() {
            return Err(ConversionError::ConversionFailed(format!(
                "STEP file parsed successfully ({} bytes), but no geometric entities could be converted to shells.\n\
                \n\
                This indicates that:\n\
                - Either the STEP file doesn't contain supported geometric entity types, or\n\
                - Entity conversion from STEP entities to truck Shell types is not yet fully implemented.\n\
                \n\
                Current status:\n\
                - ✅ STEP file parsing working\n\
                - ✅ Entity extraction framework in place\n\
                - 🚧 STEP entity → truck Shell conversion - in progress\n\
                - ⏳ Tessellation - pending\n\
                \n\
                The entity conversion requires mapping STEP entity structures (MANIFOLD_SOLID_BREP, CLOSED_SHELL, etc.) \
                to truck Shell objects, which is a complex task requiring STEP entity semantics knowledge.",
                data.len()
            )));
        }

        // Security: Estimate resource usage before tessellation
        let estimated_vertices = shells.len() * 1000; // Conservative estimate
        let estimated_faces = shells.len() * 2000; // Conservative estimate
        if let Err(e) = self
            .limits
            .check_mesh_resources(estimated_vertices, estimated_faces)
        {
            common::security::log_security_error(&e, None);
            return Err(e);
        }

        // Convert truck shells to our mesh format
        self.convert_truck_to_mesh(shells)
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
        self.parse_step(data)
    }
}

#[cfg(feature = "step")]
impl MeshWriter for StepFormat {
    fn write(&self, _mesh: &Mesh) -> Result<Vec<u8>> {
        // STEP writing requires complex CAD modeling capabilities
        // truck library focuses on reading, not writing
        Err(ConversionError::UnsupportedFormat(
            "STEP writing is not supported. STEP files require complex CAD modeling that is beyond the scope of this converter.".to_string()
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
