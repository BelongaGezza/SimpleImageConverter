// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::formats::StlFormat;
use common::error::{ConversionError, Result};
use common::io::get_extension;
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
            _ => Err(ConversionError::UnsupportedFormat(format!(
                "Format not yet implemented: {:?}",
                format
            ))),
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
            _ => Err(ConversionError::UnsupportedFormat(format!(
                "Format not yet implemented: {:?}",
                format
            ))),
        }
    }
}

/// Mesh format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshFormat {
    Stl,
    Obj,
    Ply,
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
    fn test_get_reader_unsupported() {
        let reader = FormatRegistry::get_reader(MeshFormat::Obj);
        assert!(reader.is_err());
    }

    #[test]
    fn test_get_writer_stl() {
        let writer = FormatRegistry::get_writer(MeshFormat::Stl);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_get_writer_unsupported() {
        let writer = FormatRegistry::get_writer(MeshFormat::Ply);
        assert!(writer.is_err());
    }
}
