// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
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
            "glb" => Ok(MeshFormat::Gltf), // Binary glTF
            "dxf" => Ok(MeshFormat::Dxf),
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
            MeshFormat::Dxf => Ok(Box::new(DxfFormat::new())),
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
            MeshFormat::Dxf => Ok(Box::new(DxfFormat::with_limits(limits))),
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
            MeshFormat::Dxf => Ok(Box::new(DxfFormat::new())),
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
    Dxf,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            MeshFormat::Gltf
        );
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
}
