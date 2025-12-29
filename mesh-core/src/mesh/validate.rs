// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::mesh::Mesh;
use common::error::{ConversionError, Result};

/// Validation errors found in a mesh
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub severity: ValidationSeverity,
}

/// Severity of a validation error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    /// Warning - mesh may have issues but is usable
    Warning,
    /// Error - mesh has serious issues
    Error,
}

/// Validate a mesh for common issues
///
/// This function performs various checks on the mesh:
/// - Vertex count validation
/// - Face index validation
/// - Duplicate vertices detection
/// - Degenerate faces detection
/// - Normal consistency checks
///
/// # Arguments
///
/// * `mesh` - The mesh to validate
///
/// # Returns
///
/// Ok(()) if mesh is valid, or an error describing validation failures.
pub fn validate_mesh(mesh: &Mesh) -> Result<()> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check for empty mesh
    if mesh.vertices.is_empty() {
        errors.push(ValidationError {
            message: "Mesh has no vertices".to_string(),
            severity: ValidationSeverity::Error,
        });
    }

    if mesh.faces.is_empty() {
        warnings.push(ValidationError {
            message: "Mesh has no faces".to_string(),
            severity: ValidationSeverity::Warning,
        });
    }

    // Validate face indices
    for (face_idx, face) in mesh.faces.iter().enumerate() {
        for &index in &face.indices {
            if index >= mesh.vertices.len() {
                errors.push(ValidationError {
                    message: format!(
                        "Face {} has invalid vertex index {} (vertex count: {})",
                        face_idx,
                        index,
                        mesh.vertices.len()
                    ),
                    severity: ValidationSeverity::Error,
                });
            }
        }

        // Check for degenerate faces (all indices the same)
        if face.indices[0] == face.indices[1]
            || face.indices[1] == face.indices[2]
            || face.indices[0] == face.indices[2]
        {
            warnings.push(ValidationError {
                message: format!("Face {} is degenerate (duplicate vertex indices)", face_idx),
                severity: ValidationSeverity::Warning,
            });
        }
    }

    // Check normal count matches vertex count (if normals present)
    if !mesh.normals.is_empty() && mesh.normals.len() != mesh.vertices.len() {
        warnings.push(ValidationError {
            message: format!(
                "Normal count ({}) does not match vertex count ({})",
                mesh.normals.len(),
                mesh.vertices.len()
            ),
            severity: ValidationSeverity::Warning,
        });
    }

    // Check for duplicate vertices (simple check - same coordinates)
    // This is a basic check; a full duplicate detection would use spatial hashing
    let mut duplicate_count = 0;
    for i in 0..mesh.vertices.len() {
        for j in (i + 1)..mesh.vertices.len() {
            let v1 = &mesh.vertices[i];
            let v2 = &mesh.vertices[j];
            let dx = (v1.x - v2.x).abs();
            let dy = (v1.y - v2.y).abs();
            let dz = (v1.z - v2.z).abs();

            if dx < 1e-6 && dy < 1e-6 && dz < 1e-6 {
                duplicate_count += 1;
            }
        }
    }

    if duplicate_count > 0 {
        warnings.push(ValidationError {
            message: format!(
                "Found {} potential duplicate vertices (within 1e-6 tolerance)",
                duplicate_count
            ),
            severity: ValidationSeverity::Warning,
        });
    }

    // If there are errors, return them
    if !errors.is_empty() {
        let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
        return Err(ConversionError::InvalidInput(format!(
            "Mesh validation failed:\n{}",
            error_messages.join("\n")
        )));
    }

    // If there are only warnings, log them but don't fail
    if !warnings.is_empty() {
        let warning_messages: Vec<String> = warnings.iter().map(|w| w.message.clone()).collect();
        // For now, we just log warnings but don't fail
        // In the future, we could return a ValidationResult with warnings
        eprintln!("Mesh validation warnings:\n{}", warning_messages.join("\n"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{Face, Vertex};

    #[test]
    fn test_validate_empty_mesh() {
        let mesh = Mesh::new();
        let result = validate_mesh(&mesh);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_mesh() {
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

        let result = validate_mesh(&mesh);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_face_indices() {
        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.faces.push(Face {
            indices: [0, 1, 2], // Invalid indices
        });

        let result = validate_mesh(&mesh);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_degenerate_face() {
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
        mesh.faces.push(Face {
            indices: [0, 0, 1], // Degenerate face
        });

        // Should pass validation but generate warning
        // Degenerate faces are warnings, not errors, so validation should succeed
        let result = validate_mesh(&mesh);
        assert!(
            result.is_ok(),
            "Degenerate faces should only generate warnings, not errors"
        );
    }

    #[test]
    fn test_validate_normal_count_mismatch() {
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

        // Add only 2 normals when we have 3 vertices (mismatch)
        mesh.normals.push(crate::mesh::Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        mesh.normals.push(crate::mesh::Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });

        // Should pass validation but generate warning about normal count mismatch
        let result = validate_mesh(&mesh);
        assert!(
            result.is_ok(),
            "Normal count mismatch should only generate warnings, not errors"
        );
    }
}
