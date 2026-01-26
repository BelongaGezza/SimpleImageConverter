// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
#[cfg(feature = "step")]
use crate::formats::StepFormat;
use crate::formats::{DxfFormat, GltfFormat, ObjFormat, OffFormat, PlyFormat, StlFormat};
use common::error::{ConversionError, Result};
use common::io::get_extension;
use common::limits::ResourceLimits;
use std::path::Path;

/// Format registry for detecting and getting format handlers
///
/// This registry provides format detection and handler retrieval for mesh formats.
/// It supports format detection by file extension and provides reader/writer instances.
///
/// # Example
///
/// ```
/// use mesh_core::{FormatRegistry, MeshFormat};
/// use std::path::Path;
///
/// // Detect format from extension
/// let format = FormatRegistry::detect_format("stl")?;
/// assert_eq!(format, MeshFormat::Stl);
///
/// // Detect format from path
/// let path = Path::new("model.stl");
/// let format = FormatRegistry::detect_from_path(path)?;
/// assert_eq!(format, MeshFormat::Stl);
///
/// // Get format handlers
/// let reader = FormatRegistry::get_reader(MeshFormat::Stl)?;
/// let writer = FormatRegistry::get_writer(MeshFormat::Stl)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct FormatRegistry;

impl FormatRegistry {
    /// Detect format from file extension
    ///
    /// # Arguments
    ///
    /// * `extension` - File extension (case-insensitive, without leading dot)
    ///
    /// # Returns
    ///
    /// The detected `MeshFormat`, or an error if the format is unsupported.
    ///
    /// # Example
    ///
    /// ```
    /// use mesh_core::{FormatRegistry, MeshFormat};
    ///
    /// let format = FormatRegistry::detect_format("stl")?;
    /// assert_eq!(format, MeshFormat::Stl);
    ///
    /// let format = FormatRegistry::detect_format("STL")?; // Case insensitive
    /// assert_eq!(format, MeshFormat::Stl);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn detect_format(extension: &str) -> Result<MeshFormat> {
        match extension.to_lowercase().as_str() {
            "stl" => Ok(MeshFormat::Stl),
            "obj" => Ok(MeshFormat::Obj),
            "ply" => Ok(MeshFormat::Ply),
            "off" => Ok(MeshFormat::Off),
            "gltf" => Ok(MeshFormat::Gltf),
            "glb" => Ok(MeshFormat::Glb), // Binary glTF container
            "dxf" => Ok(MeshFormat::Dxf),
            "step" | "stp" => {
                #[cfg(feature = "step")]
                {
                    Ok(MeshFormat::Step)
                }
                #[cfg(not(feature = "step"))]
                {
                    Err(ConversionError::UnsupportedFormat(
                        "STEP format support requires 'step' feature flag. Enable it with: cargo build --features step".to_string()
                    ))
                }
            }
            _ => Err(ConversionError::UnsupportedFormat(format!(
                "Unsupported format: {}",
                extension
            ))),
        }
    }

    /// Detect format from file path
    ///
    /// Extracts the file extension from the path and detects the format.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to analyze
    ///
    /// # Returns
    ///
    /// The detected `MeshFormat`, or an error if:
    /// - The file has no extension
    /// - The format is unsupported
    ///
    /// # Example
    ///
    /// ```
    /// use mesh_core::{FormatRegistry, MeshFormat};
    /// use std::path::Path;
    ///
    /// let path = Path::new("model.stl");
    /// let format = FormatRegistry::detect_from_path(path)?;
    /// assert_eq!(format, MeshFormat::Stl);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn detect_from_path(path: &Path) -> Result<MeshFormat> {
        let ext = get_extension(path)
            .ok_or_else(|| ConversionError::InvalidInput("File has no extension".to_string()))?;
        Self::detect_format(&ext)
    }

    /// Best-effort signature detection from bytes.
    ///
    /// This is intentionally conservative: it only returns `Some` when a format has a
    /// clear, low-cost signature. For other formats it returns `None`.
    pub fn detect_from_bytes(data: &[u8]) -> Option<MeshFormat> {
        let data = skip_ascii_whitespace(data);

        // GLB starts with the "glTF" magic.
        if data.len() >= 4 && &data[0..4] == b"glTF" {
            return Some(MeshFormat::Glb);
        }

        // OFF family starts with an "OFF" token (e.g., OFF / COFF / NOFF / CNOFF / STOFF).
        if let Some(token) = first_ascii_token(data) {
            if token.len() >= 3
                && token.len() <= 5
                && token.iter().all(|b| b.is_ascii_alphabetic())
                && token.ends_with(b"OFF")
            {
                return Some(MeshFormat::Off);
            }
        }

        // PLY starts with "ply" (ASCII).
        if data.len() >= 3 && data[0..3].eq_ignore_ascii_case(b"ply") {
            return Some(MeshFormat::Ply);
        }

        // Text `.gltf` is JSON; we do a cheap heuristic check rather than a full parse here.
        if looks_like_gltf_json(data) {
            return Some(MeshFormat::Gltf);
        }

        None
    }

    /// Detect format using two-stage detection (extension + signature, when available).
    ///
    /// This mirrors the image pipeline: we always use the extension as the primary signal,
    /// then (when feasible) verify the content signature to reduce spoofing risk.
    pub fn detect_two_stage(path: &Path, data: &[u8]) -> Result<MeshFormat> {
        // Stage 1: extension
        let extension_format = Self::detect_from_path(path)?;

        // Stage 2: signature (best-effort)
        if let Some(signature_format) = Self::detect_from_bytes(data) {
            if signature_format != extension_format {
                return Err(ConversionError::InvalidFormat(format!(
                    "Format mismatch: extension suggests {:?} but signature indicates {:?}",
                    extension_format, signature_format
                )));
            }
        }

        Ok(extension_format)
    }

    /// Get reader for a format
    ///
    /// Returns a boxed `MeshReader` trait object for the specified format.
    ///
    /// # Arguments
    ///
    /// * `format` - The mesh format to get a reader for
    ///
    /// # Returns
    ///
    /// A boxed reader instance, or an error if the format is not yet implemented.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mesh_core::{FormatRegistry, MeshFormat};
    ///
    /// // Get a STL reader
    /// let reader = FormatRegistry::get_reader(MeshFormat::Stl)?;
    ///
    /// // Read STL data from file
    /// let stl_bytes = std::fs::read("model.stl")?;
    /// let mesh = reader.read(&stl_bytes)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_reader(format: MeshFormat) -> Result<Box<dyn MeshReader>> {
        match format {
            MeshFormat::Stl => Ok(Box::new(StlFormat::new())),
            MeshFormat::Obj => Ok(Box::new(ObjFormat::new())),
            MeshFormat::Ply => Ok(Box::new(PlyFormat::new())),
            MeshFormat::Off => Ok(Box::new(OffFormat::new())),
            MeshFormat::Gltf => Ok(Box::new(GltfFormat::new())),
            MeshFormat::Glb => Ok(Box::new(GltfFormat::new())),
            MeshFormat::Dxf => Ok(Box::new(DxfFormat::new())),
            MeshFormat::Step => {
                #[cfg(feature = "step")]
                {
                    Ok(Box::new(StepFormat::new()))
                }
                #[cfg(not(feature = "step"))]
                {
                    Err(ConversionError::UnsupportedFormat(
                        "STEP format support requires 'step' feature flag".to_string(),
                    ))
                }
            }
        }
    }

    /// Get reader for a format with custom resource limits
    ///
    /// Returns a boxed `MeshReader` trait object configured with resource limits
    /// for security validation.
    ///
    /// # Arguments
    ///
    /// * `format` - The mesh format to get a reader for
    /// * `limits` - Resource limits for validation
    ///
    /// # Returns
    ///
    /// A boxed reader instance with configured limits.
    pub fn get_reader_with_limits(
        format: MeshFormat,
        limits: ResourceLimits,
    ) -> Result<Box<dyn MeshReader>> {
        match format {
            MeshFormat::Stl => Ok(Box::new(StlFormat::with_limits(limits))),
            MeshFormat::Obj => Ok(Box::new(ObjFormat::with_limits(limits))),
            MeshFormat::Ply => Ok(Box::new(PlyFormat::with_limits(limits))),
            MeshFormat::Off => Ok(Box::new(OffFormat::with_limits(limits))),
            MeshFormat::Gltf => Ok(Box::new(GltfFormat::with_limits(limits))),
            MeshFormat::Glb => Ok(Box::new(GltfFormat::with_limits(limits))),
            MeshFormat::Dxf => Ok(Box::new(DxfFormat::with_limits(limits))),
            MeshFormat::Step => {
                #[cfg(feature = "step")]
                {
                    Ok(Box::new(StepFormat::with_limits(limits)))
                }
                #[cfg(not(feature = "step"))]
                {
                    Err(ConversionError::UnsupportedFormat(
                        "STEP format support requires 'step' feature flag".to_string(),
                    ))
                }
            }
        }
    }

    /// Get writer for a format
    ///
    /// Returns a boxed `MeshWriter` trait object for the specified format.
    ///
    /// # Arguments
    ///
    /// * `format` - The mesh format to get a writer for
    ///
    /// # Returns
    ///
    /// A boxed writer instance, or an error if the format is not yet implemented.
    ///
    /// # Example
    ///
    /// ```
    /// use mesh_core::{FormatRegistry, MeshFormat, Mesh};
    /// use mesh_core::mesh::{Vertex, Face};
    ///
    /// let writer = FormatRegistry::get_writer(MeshFormat::Stl)?;
    /// let mut mesh = Mesh::new();
    /// // Add vertices and faces to mesh
    /// mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 });
    /// mesh.vertices.push(Vertex { x: 1.0, y: 0.0, z: 0.0 });
    /// mesh.vertices.push(Vertex { x: 0.5, y: 1.0, z: 0.0 });
    /// mesh.faces.push(Face { indices: [0, 1, 2] });
    /// let stl_data = writer.write(&mesh)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_writer(format: MeshFormat) -> Result<Box<dyn MeshWriter>> {
        match format {
            MeshFormat::Stl => Ok(Box::new(StlFormat::new())),
            MeshFormat::Obj => Ok(Box::new(ObjFormat::new())),
            MeshFormat::Ply => Ok(Box::new(PlyFormat::new())),
            MeshFormat::Off => Ok(Box::new(OffFormat::new())),
            MeshFormat::Gltf => Ok(Box::new(GltfFormat::new())),
            MeshFormat::Glb => Ok(Box::new(GltfFormat::new_glb())),
            MeshFormat::Dxf => Ok(Box::new(DxfFormat::new())),
            MeshFormat::Step => {
                #[cfg(feature = "step")]
                {
                    Ok(Box::new(StepFormat::new()))
                }
                #[cfg(not(feature = "step"))]
                {
                    Err(ConversionError::UnsupportedFormat(
                        "STEP format support requires 'step' feature flag".to_string(),
                    ))
                }
            }
        }
    }
}

/// Mesh format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshFormat {
    Stl,
    Obj,
    Ply,
    Off,
    Gltf,
    Glb,
    Dxf,
    Step,
}

fn skip_ascii_whitespace(mut data: &[u8]) -> &[u8] {
    while let Some((&b, rest)) = data.split_first() {
        if b.is_ascii_whitespace() {
            data = rest;
        } else {
            break;
        }
    }
    data
}

fn first_ascii_token(data: &[u8]) -> Option<&[u8]> {
    if data.is_empty() {
        return None;
    }
    let end = data
        .iter()
        .position(|b| b.is_ascii_whitespace())
        .unwrap_or(data.len());
    Some(&data[..end])
}

fn looks_like_gltf_json(data: &[u8]) -> bool {
    let data = skip_ascii_whitespace(data);
    if data.first().copied() != Some(b'{') {
        return false;
    }

    let scan_len = data.len().min(4096);
    let Ok(prefix) = std::str::from_utf8(&data[..scan_len]) else {
        return false;
    };

    // Cheap "shape" checks for glTF 2.0 JSON.
    prefix.contains("\"asset\"") && prefix.contains("\"version\"") && prefix.contains("2.0")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{Face, Mesh, Vertex};

    #[test]
    fn test_detect_format_stl() {
        assert_eq!(
            FormatRegistry::detect_format("stl").unwrap(),
            MeshFormat::Stl
        );
        assert_eq!(
            FormatRegistry::detect_format("STL").unwrap(),
            MeshFormat::Stl
        );
        assert_eq!(
            FormatRegistry::detect_format("Stl").unwrap(),
            MeshFormat::Stl
        );
    }

    #[test]
    fn test_detect_format_obj() {
        assert_eq!(
            FormatRegistry::detect_format("obj").unwrap(),
            MeshFormat::Obj
        );
        assert_eq!(
            FormatRegistry::detect_format("OBJ").unwrap(),
            MeshFormat::Obj
        );
    }

    #[test]
    fn test_detect_format_ply() {
        assert_eq!(
            FormatRegistry::detect_format("ply").unwrap(),
            MeshFormat::Ply
        );
        assert_eq!(
            FormatRegistry::detect_format("PLY").unwrap(),
            MeshFormat::Ply
        );
    }

    #[test]
    fn test_detect_format_invalid() {
        assert!(FormatRegistry::detect_format("xyz").is_err());
        assert!(FormatRegistry::detect_format("").is_err());
    }

    #[test]
    fn test_detect_from_path() {
        let path = Path::new("model.stl");
        assert_eq!(
            FormatRegistry::detect_from_path(path).unwrap(),
            MeshFormat::Stl
        );

        let path = Path::new("mesh.OBJ");
        assert_eq!(
            FormatRegistry::detect_from_path(path).unwrap(),
            MeshFormat::Obj
        );
    }

    #[test]
    fn test_detect_from_path_no_extension() {
        let path = Path::new("model");
        assert!(FormatRegistry::detect_from_path(path).is_err());
    }

    #[test]
    fn test_get_reader_stl() {
        let reader = FormatRegistry::get_reader(MeshFormat::Stl);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_obj() {
        let reader = FormatRegistry::get_reader(MeshFormat::Obj);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_ply() {
        let reader = FormatRegistry::get_reader(MeshFormat::Ply);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_writer_stl() {
        let writer = FormatRegistry::get_writer(MeshFormat::Stl);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_obj() {
        let writer = FormatRegistry::get_writer(MeshFormat::Obj);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_ply() {
        let writer = FormatRegistry::get_writer(MeshFormat::Ply);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_detect_format_off() {
        assert_eq!(
            FormatRegistry::detect_format("off").unwrap(),
            MeshFormat::Off
        );
        assert_eq!(
            FormatRegistry::detect_format("OFF").unwrap(),
            MeshFormat::Off
        );
    }

    #[test]
    fn test_detect_format_gltf() {
        assert_eq!(
            FormatRegistry::detect_format("gltf").unwrap(),
            MeshFormat::Gltf
        );
        assert_eq!(
            FormatRegistry::detect_format("glb").unwrap(),
            MeshFormat::Glb
        );
    }

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
    fn test_detect_two_stage_glb_matches() {
        let mesh = create_test_triangle();
        let glb = GltfFormat::new_glb().write(&mesh).unwrap();
        let detected = FormatRegistry::detect_two_stage(Path::new("mesh.glb"), &glb).unwrap();
        assert_eq!(detected, MeshFormat::Glb);
    }

    #[test]
    fn test_detect_two_stage_glb_mismatch_gltf() {
        let mesh = create_test_triangle();
        let gltf = GltfFormat::new().write(&mesh).unwrap();
        let result = FormatRegistry::detect_two_stage(Path::new("mesh.glb"), &gltf);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_two_stage_off_mismatch_ply() {
        let ply_data = b"ply\nformat ascii 1.0\nend_header\n";
        let result = FormatRegistry::detect_two_stage(Path::new("mesh.off"), ply_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_two_stage_ply_mismatch_off() {
        let off_data = b"OFF\n0 0 0\n";
        let result = FormatRegistry::detect_two_stage(Path::new("mesh.ply"), off_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_format_dxf() {
        assert_eq!(
            FormatRegistry::detect_format("dxf").unwrap(),
            MeshFormat::Dxf
        );
        assert_eq!(
            FormatRegistry::detect_format("DXF").unwrap(),
            MeshFormat::Dxf
        );
    }

    #[test]
    fn test_get_reader_off() {
        let reader = FormatRegistry::get_reader(MeshFormat::Off);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_gltf() {
        let reader = FormatRegistry::get_reader(MeshFormat::Gltf);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_reader_dxf() {
        let reader = FormatRegistry::get_reader(MeshFormat::Dxf);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_get_writer_off() {
        let writer = FormatRegistry::get_writer(MeshFormat::Off);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_gltf() {
        let writer = FormatRegistry::get_writer(MeshFormat::Gltf);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_dxf() {
        let writer = FormatRegistry::get_writer(MeshFormat::Dxf);
        assert!(writer.is_ok());
    }

    #[test]
    #[cfg(feature = "step")]
    fn test_detect_format_step() {
        assert_eq!(
            FormatRegistry::detect_format("step").unwrap(),
            MeshFormat::Step
        );
        assert_eq!(
            FormatRegistry::detect_format("stp").unwrap(),
            MeshFormat::Step
        );
        assert_eq!(
            FormatRegistry::detect_format("STEP").unwrap(),
            MeshFormat::Step
        );
    }

    #[test]
    #[cfg(feature = "step")]
    fn test_get_reader_step() {
        let reader = FormatRegistry::get_reader(MeshFormat::Step);
        assert!(reader.is_ok());
    }

    #[test]
    #[cfg(feature = "step")]
    fn test_get_writer_step() {
        let writer = FormatRegistry::get_writer(MeshFormat::Step);
        assert!(writer.is_ok());
    }

    #[test]
    #[cfg(not(feature = "step"))]
    fn test_detect_format_step_without_feature() {
        // Without the feature flag, STEP should return an error
        assert!(FormatRegistry::detect_format("step").is_err());
        assert!(FormatRegistry::detect_format("stp").is_err());
    }
}
