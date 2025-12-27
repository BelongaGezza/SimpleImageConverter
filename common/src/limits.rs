// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Resource limits for security validation
//!
//! This module provides centralized resource limits to prevent denial-of-service
//! attacks via maliciously crafted files with extreme sizes or dimensions.

use crate::error::{ConversionError, Result};

/// Default maximum file size: 100MB
pub const DEFAULT_MAX_FILE_SIZE: usize = 100 * 1024 * 1024;

/// Default maximum image dimension: 65535 pixels
pub const DEFAULT_MAX_IMAGE_DIMENSION: u32 = 65535;

/// Default maximum vertices: 10 million
pub const DEFAULT_MAX_VERTICES: usize = 10_000_000;

/// Default maximum faces: 10 million
pub const DEFAULT_MAX_FACES: usize = 10_000_000;

/// Centralized resource limits for security
///
/// All file operations should validate against these limits
/// before allocating memory or processing data.
///
/// # Example
///
/// ```
/// use common::limits::ResourceLimits;
///
/// let limits = ResourceLimits::default();
/// assert!(limits.check_file_size(1024).is_ok());  // 1KB OK
/// assert!(limits.check_file_size(200 * 1024 * 1024).is_err());  // 200MB too large
/// ```
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum file size in bytes
    pub max_file_size: usize,

    /// Maximum image dimension (width or height) in pixels
    pub max_image_dimension: u32,

    /// Maximum number of mesh vertices
    pub max_vertices: usize,

    /// Maximum number of mesh faces
    pub max_faces: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_file_size: DEFAULT_MAX_FILE_SIZE,
            max_image_dimension: DEFAULT_MAX_IMAGE_DIMENSION,
            max_vertices: DEFAULT_MAX_VERTICES,
            max_faces: DEFAULT_MAX_FACES,
        }
    }
}

impl ResourceLimits {
    /// Create new ResourceLimits with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create permissive limits for trusted input only
    ///
    /// WARNING: Only use for trusted input sources!
    pub fn permissive() -> Self {
        Self {
            max_file_size: 1024 * 1024 * 1024, // 1GB
            max_image_dimension: 131072,       // 128K
            max_vertices: 100_000_000,         // 100M
            max_faces: 100_000_000,            // 100M
        }
    }

    /// Create a builder for custom limits
    pub fn builder() -> ResourceLimitsBuilder {
        ResourceLimitsBuilder::new()
    }

    /// Validate file size against limit
    pub fn check_file_size(&self, size: usize) -> Result<()> {
        if size > self.max_file_size {
            return Err(ConversionError::InvalidInput(format!(
                "File size {} bytes exceeds limit of {} bytes ({} MB)",
                size,
                self.max_file_size,
                self.max_file_size / (1024 * 1024)
            )));
        }
        Ok(())
    }

    /// Validate image dimensions against limit
    pub fn check_image_dimensions(&self, width: u32, height: u32) -> Result<()> {
        if width > self.max_image_dimension {
            return Err(ConversionError::InvalidInput(format!(
                "Image width {} exceeds limit of {}",
                width, self.max_image_dimension
            )));
        }
        if height > self.max_image_dimension {
            return Err(ConversionError::InvalidInput(format!(
                "Image height {} exceeds limit of {}",
                height, self.max_image_dimension
            )));
        }
        Ok(())
    }

    /// Validate mesh vertex count against limit
    pub fn check_vertex_count(&self, count: usize) -> Result<()> {
        if count > self.max_vertices {
            return Err(ConversionError::InvalidInput(format!(
                "Vertex count {} exceeds limit of {}",
                count, self.max_vertices
            )));
        }
        Ok(())
    }

    /// Validate mesh face count against limit
    pub fn check_face_count(&self, count: usize) -> Result<()> {
        if count > self.max_faces {
            return Err(ConversionError::InvalidInput(format!(
                "Face count {} exceeds limit of {}",
                count, self.max_faces
            )));
        }
        Ok(())
    }

    /// Validate all mesh resources at once
    pub fn check_mesh_resources(&self, vertices: usize, faces: usize) -> Result<()> {
        self.check_vertex_count(vertices)?;
        self.check_face_count(faces)?;
        Ok(())
    }
}

/// Builder for customizing ResourceLimits
#[derive(Debug, Clone)]
pub struct ResourceLimitsBuilder {
    limits: ResourceLimits,
}

impl ResourceLimitsBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }

    /// Set maximum file size in bytes
    pub fn max_file_size(mut self, size: usize) -> Self {
        self.limits.max_file_size = size;
        self
    }

    /// Set maximum file size in megabytes
    pub fn max_file_size_mb(mut self, mb: usize) -> Self {
        self.limits.max_file_size = mb * 1024 * 1024;
        self
    }

    /// Set maximum image dimension
    pub fn max_image_dimension(mut self, dimension: u32) -> Self {
        self.limits.max_image_dimension = dimension;
        self
    }

    /// Set maximum vertex count
    pub fn max_vertices(mut self, count: usize) -> Self {
        self.limits.max_vertices = count;
        self
    }

    /// Set maximum face count
    pub fn max_faces(mut self, count: usize) -> Self {
        self.limits.max_faces = count;
        self
    }

    /// Build the ResourceLimits
    pub fn build(self) -> ResourceLimits {
        self.limits
    }
}

impl Default for ResourceLimitsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_file_size, DEFAULT_MAX_FILE_SIZE);
        assert_eq!(limits.max_image_dimension, DEFAULT_MAX_IMAGE_DIMENSION);
        assert_eq!(limits.max_vertices, DEFAULT_MAX_VERTICES);
        assert_eq!(limits.max_faces, DEFAULT_MAX_FACES);
    }

    #[test]
    fn test_check_file_size_ok() {
        let limits = ResourceLimits::default();
        assert!(limits.check_file_size(1024).is_ok());
        assert!(limits.check_file_size(50 * 1024 * 1024).is_ok());
        assert!(limits.check_file_size(100 * 1024 * 1024).is_ok()); // exactly at limit
    }

    #[test]
    fn test_check_file_size_exceeded() {
        let limits = ResourceLimits::default();
        let result = limits.check_file_size(200 * 1024 * 1024);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds limit"));
    }

    #[test]
    fn test_check_dimensions_ok() {
        let limits = ResourceLimits::default();
        assert!(limits.check_image_dimensions(1920, 1080).is_ok());
        assert!(limits.check_image_dimensions(4096, 4096).is_ok());
        assert!(limits.check_image_dimensions(65535, 65535).is_ok()); // at limit
    }

    #[test]
    fn test_check_dimensions_width_exceeded() {
        let limits = ResourceLimits::default();
        let result = limits.check_image_dimensions(100_000, 100);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("width"));
    }

    #[test]
    fn test_check_dimensions_height_exceeded() {
        let limits = ResourceLimits::default();
        let result = limits.check_image_dimensions(100, 100_000);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("height"));
    }

    #[test]
    fn test_check_mesh_resources_ok() {
        let limits = ResourceLimits::default();
        assert!(limits.check_mesh_resources(1000, 2000).is_ok());
        assert!(limits.check_mesh_resources(10_000_000, 10_000_000).is_ok()); // at limit
    }

    #[test]
    fn test_check_vertex_count_exceeded() {
        let limits = ResourceLimits::default();
        let result = limits.check_vertex_count(20_000_000);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Vertex count"));
    }

    #[test]
    fn test_check_face_count_exceeded() {
        let limits = ResourceLimits::default();
        let result = limits.check_face_count(20_000_000);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Face count"));
    }

    #[test]
    fn test_permissive_limits() {
        let limits = ResourceLimits::permissive();
        assert!(limits.check_file_size(500 * 1024 * 1024).is_ok());
        assert!(limits.check_image_dimensions(100_000, 100_000).is_ok());
        assert!(limits.check_mesh_resources(50_000_000, 50_000_000).is_ok());
    }

    #[test]
    fn test_builder() {
        let limits = ResourceLimits::builder()
            .max_file_size_mb(50)
            .max_image_dimension(10000)
            .max_vertices(1_000_000)
            .max_faces(2_000_000)
            .build();

        assert_eq!(limits.max_file_size, 50 * 1024 * 1024);
        assert_eq!(limits.max_image_dimension, 10000);
        assert_eq!(limits.max_vertices, 1_000_000);
        assert_eq!(limits.max_faces, 2_000_000);
    }

    #[test]
    fn test_builder_custom_file_size() {
        let limits = ResourceLimits::builder()
            .max_file_size(1024 * 1024) // 1MB
            .build();

        assert!(limits.check_file_size(512 * 1024).is_ok());
        assert!(limits.check_file_size(2 * 1024 * 1024).is_err());
    }
}
