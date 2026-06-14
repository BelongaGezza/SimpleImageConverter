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

/// Default maximum decoded image data: 512 MiB
pub const DEFAULT_MAX_DECODED_IMAGE_BYTES: usize = 512 * 1024 * 1024;

/// Default maximum vertices allowed in one source polygon before triangulation
pub const DEFAULT_MAX_VERTICES_PER_POLYGON: usize = 64;

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

    /// Maximum decoded image data in bytes
    pub max_decoded_image_bytes: usize,

    /// Maximum number of vertices in a single polygon before triangulation
    pub max_vertices_per_polygon: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_file_size: DEFAULT_MAX_FILE_SIZE,
            max_image_dimension: DEFAULT_MAX_IMAGE_DIMENSION,
            max_vertices: DEFAULT_MAX_VERTICES,
            max_faces: DEFAULT_MAX_FACES,
            max_decoded_image_bytes: DEFAULT_MAX_DECODED_IMAGE_BYTES,
            max_vertices_per_polygon: DEFAULT_MAX_VERTICES_PER_POLYGON,
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
    #[cfg(any(test, feature = "trusted-input"))]
    pub fn permissive() -> Self {
        Self {
            max_file_size: 1024 * 1024 * 1024,               // 1GB
            max_image_dimension: 131072,                     // 128K
            max_vertices: 100_000_000,                       // 100M
            max_faces: 100_000_000,                          // 100M
            max_decoded_image_bytes: 2 * 1024 * 1024 * 1024, // 2GB
            max_vertices_per_polygon: 1024,
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
        if width == 0 || height == 0 {
            return Err(ConversionError::InvalidInput(
                "Image dimensions must be greater than zero".to_string(),
            ));
        }
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

    /// Validate decoded image byte length against the configured limit
    pub fn check_decoded_bytes(&self, bytes: usize) -> Result<()> {
        if bytes > self.max_decoded_image_bytes {
            return Err(ConversionError::ResourceLimitExceeded(format!(
                "Decoded image size exceeds configured limit ({} > {} bytes)",
                bytes, self.max_decoded_image_bytes
            )));
        }
        Ok(())
    }

    /// Calculate and validate decoded image size using checked arithmetic
    pub fn check_decoded_image_size(
        &self,
        width: u32,
        height: u32,
        bytes_per_pixel: usize,
    ) -> Result<usize> {
        self.check_image_dimensions(width, height)?;

        let bytes = (width as usize)
            .checked_mul(height as usize)
            .and_then(|pixels| pixels.checked_mul(bytes_per_pixel))
            .ok_or_else(|| {
                ConversionError::ResourceLimitExceeded(
                    "Decoded image size calculation overflowed".to_string(),
                )
            })?;

        self.check_decoded_bytes(bytes)?;
        Ok(bytes)
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

    /// Validate source polygon vertex count before fan triangulation
    pub fn check_polygon_vertices(&self, count: usize) -> Result<()> {
        if count > self.max_vertices_per_polygon {
            return Err(ConversionError::ResourceLimitExceeded(format!(
                "Polygon vertex count exceeds configured limit ({} > {})",
                count, self.max_vertices_per_polygon
            )));
        }
        Ok(())
    }

    /// Validate whether adding triangulated faces would exceed the face budget
    pub fn check_triangulated_face_budget(
        &self,
        current_faces: usize,
        additional_faces: usize,
    ) -> Result<()> {
        let total = current_faces.checked_add(additional_faces).ok_or_else(|| {
            ConversionError::ResourceLimitExceeded(
                "Triangulated face count calculation overflowed".to_string(),
            )
        })?;
        self.check_face_count(total)
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
    ///
    /// Prefer [`ResourceLimitsBuilder::try_max_file_size_mb`] for untrusted CLI
    /// input so overflow can be reported as a validation error.
    pub fn max_file_size_mb(mut self, mb: usize) -> Self {
        self.limits.max_file_size = mb
            .checked_mul(1024)
            .and_then(|v| v.checked_mul(1024))
            .expect("max_file_size_mb overflowed; use try_max_file_size_mb for user input");
        self
    }

    /// Set maximum file size in megabytes using checked arithmetic
    pub fn try_max_file_size_mb(mut self, mb: usize) -> Result<Self> {
        self.limits.max_file_size = mb
            .checked_mul(1024)
            .and_then(|v| v.checked_mul(1024))
            .ok_or_else(|| {
                ConversionError::InvalidInput(
                    "Maximum file size is too large to represent safely".to_string(),
                )
            })?;
        Ok(self)
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

    /// Set maximum decoded image bytes
    pub fn max_decoded_image_bytes(mut self, bytes: usize) -> Self {
        self.limits.max_decoded_image_bytes = bytes;
        self
    }

    /// Set maximum decoded image size in MiB using checked arithmetic
    pub fn try_max_decoded_image_mb(mut self, mb: usize) -> Result<Self> {
        self.limits.max_decoded_image_bytes = mb
            .checked_mul(1024)
            .and_then(|v| v.checked_mul(1024))
            .ok_or_else(|| {
                ConversionError::InvalidInput(
                    "Maximum decoded image size is too large to represent safely".to_string(),
                )
            })?;
        Ok(self)
    }

    /// Set maximum vertices allowed in one source polygon
    pub fn max_vertices_per_polygon(mut self, count: usize) -> Self {
        self.limits.max_vertices_per_polygon = count;
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
        assert_eq!(
            limits.max_decoded_image_bytes,
            DEFAULT_MAX_DECODED_IMAGE_BYTES
        );
        assert_eq!(
            limits.max_vertices_per_polygon,
            DEFAULT_MAX_VERTICES_PER_POLYGON
        );
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
        assert!(limits.check_polygon_vertices(1024).is_ok());
    }

    #[test]
    fn test_builder() {
        let limits = ResourceLimits::builder()
            .max_file_size_mb(50)
            .max_image_dimension(10000)
            .max_vertices(1_000_000)
            .max_faces(2_000_000)
            .max_decoded_image_bytes(10 * 1024 * 1024)
            .max_vertices_per_polygon(32)
            .build();

        assert_eq!(limits.max_file_size, 50 * 1024 * 1024);
        assert_eq!(limits.max_image_dimension, 10000);
        assert_eq!(limits.max_vertices, 1_000_000);
        assert_eq!(limits.max_faces, 2_000_000);
        assert_eq!(limits.max_decoded_image_bytes, 10 * 1024 * 1024);
        assert_eq!(limits.max_vertices_per_polygon, 32);
    }

    #[test]
    fn test_builder_custom_file_size() {
        let limits = ResourceLimits::builder()
            .max_file_size(1024 * 1024) // 1MB
            .build();

        assert!(limits.check_file_size(512 * 1024).is_ok());
        assert!(limits.check_file_size(2 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_decoded_image_limit() {
        let limits = ResourceLimits::builder()
            .max_decoded_image_bytes(100)
            .build();
        assert_eq!(limits.check_decoded_image_size(5, 5, 4).unwrap(), 100);
        assert!(limits.check_decoded_image_size(6, 5, 4).is_err());
    }

    #[test]
    fn test_decoded_image_overflow() {
        let limits = ResourceLimits::builder()
            .max_decoded_image_bytes(usize::MAX)
            .build();
        let result = limits.check_decoded_image_size(u32::MAX, u32::MAX, usize::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn test_polygon_vertex_limit() {
        let limits = ResourceLimits::builder()
            .max_vertices_per_polygon(4)
            .build();
        assert!(limits.check_polygon_vertices(4).is_ok());
        assert!(limits.check_polygon_vertices(5).is_err());
    }

    #[test]
    fn test_triangulated_face_budget() {
        let limits = ResourceLimits::builder().max_faces(5).build();
        assert!(limits.check_triangulated_face_budget(3, 2).is_ok());
        assert!(limits.check_triangulated_face_budget(3, 3).is_err());
    }

    #[test]
    fn test_try_max_file_size_mb_overflow() {
        let result = ResourceLimits::builder().try_max_file_size_mb(usize::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_max_decoded_image_mb() {
        let limits = ResourceLimits::builder()
            .try_max_decoded_image_mb(1)
            .unwrap()
            .build();
        assert_eq!(limits.max_decoded_image_bytes, 1024 * 1024);
    }
}
