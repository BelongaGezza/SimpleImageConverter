// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Image and mesh preview functionality for the GUI
//!
//! This module provides preview rendering capabilities including:
//! - Image preview with thumbnail generation
//! - Mesh metadata extraction (for v0.2.2, simplified preview)
//! - Preview caching for performance
//! - Error handling for preview loading

#![allow(dead_code)] // Many items reserved for future use

use common::limits::ResourceLimits;
use common::validation::validate_file_path;
use egui::ColorImage;
use image::GenericImageView;
use mesh_core::{FormatRegistry as MeshFormatRegistry, MeshFormat};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Preview cache for storing loaded image previews
///
/// This cache stores preview thumbnails in memory to avoid reloading
/// images when switching between files or changing formats.
pub struct PreviewCache {
    /// Map from file path to cached preview data
    cache: HashMap<PathBuf, Arc<PreviewData>>,
    /// Maximum number of cached entries (to prevent memory bloat)
    max_entries: usize,
}

/// Cached preview data for an image
pub struct PreviewData {
    /// The preview image as egui::ColorImage
    pub image: ColorImage,
    /// Original image dimensions
    pub original_width: u32,
    pub original_height: u32,
    /// Preview thumbnail dimensions
    #[allow(dead_code)] // Reserved for future display size tracking
    pub preview_width: u32,
    #[allow(dead_code)] // Reserved for future display size tracking
    pub preview_height: u32,
}

impl PreviewCache {
    /// Create a new preview cache with the default maximum entries
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_entries: 50, // Cache up to 50 previews
        }
    }

    /// Create a new preview cache with a custom maximum entries limit
    #[allow(dead_code)] // Reserved for future configuration
    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
        }
    }

    /// Get a preview from the cache, or None if not cached
    pub fn get(&self, path: &Path) -> Option<Arc<PreviewData>> {
        self.cache.get(path).cloned()
    }

    /// Store a preview in the cache
    pub fn insert(&mut self, path: PathBuf, preview: Arc<PreviewData>) {
        // Remove oldest entries if cache is full
        while self.cache.len() >= self.max_entries && !self.cache.is_empty() {
            // Remove first entry (simple FIFO - could be improved with LRU)
            if let Some(key) = self.cache.keys().next().cloned() {
                self.cache.remove(&key);
            }
        }
        self.cache.insert(path, preview);
    }

    /// Clear the cache
    #[allow(dead_code)] // Reserved for future cache management
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for PreviewCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for preview operations
#[derive(Debug)]
pub enum PreviewError {
    InvalidPath,
    FileReadError(String),
    #[allow(dead_code)] // Reserved for future error handling
    ImageDecodeError(String),
    ImageTooLarge,
    #[allow(dead_code)] // Reserved for future error handling
    GenerationError(String),
}

impl std::fmt::Display for PreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreviewError::InvalidPath => write!(f, "Invalid file path"),
            PreviewError::FileReadError(msg) => write!(f, "Failed to read file: {}", msg),
            PreviewError::ImageDecodeError(msg) => write!(f, "Failed to decode image: {}", msg),
            PreviewError::ImageTooLarge => write!(f, "Image dimensions exceed limits"),
            PreviewError::GenerationError(msg) => write!(f, "Preview generation failed: {}", msg),
        }
    }
}

impl std::error::Error for PreviewError {}

/// Load and generate a preview thumbnail for an image file
///
/// This function:
/// - Validates the file path (security check)
/// - Loads the image using the `image` crate
/// - Generates a thumbnail if the image is larger than max_width/max_height
/// - Converts to egui::ColorImage for display
/// - Respects resource limits for security
///
/// # Arguments
///
/// * `image_path` - Path to the image file to preview
/// * `max_width` - Maximum preview width in pixels
/// * `max_height` - Maximum preview height in pixels
/// * `limits` - Resource limits for validation
///
/// # Returns
///
/// `Ok(PreviewData)` with the preview image and metadata, or an error if loading fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The file path is invalid
/// - The file cannot be read
/// - The image cannot be decoded
/// - The image dimensions exceed resource limits
/// - Preview generation fails
pub fn generate_image_preview(
    image_path: &Path,
    max_width: u32,
    max_height: u32,
    limits: &ResourceLimits,
) -> std::result::Result<PreviewData, PreviewError> {
    // Validate file path (security check)
    validate_file_path(image_path)
        .map_err(|_| PreviewError::InvalidPath)?;

    // Load image using image crate
    let dynamic_image = image::open(image_path)
        .map_err(|e| PreviewError::FileReadError(format!("Failed to open image: {}", e)))?;

    let (original_width, original_height) = dynamic_image.dimensions();

    // Check image dimensions against resource limits
    if original_width > limits.max_image_dimension
        || original_height > limits.max_image_dimension
    {
        return Err(PreviewError::ImageTooLarge);
    }

    // Generate thumbnail if image is larger than max dimensions
    let (preview_width, preview_height, thumbnail_image) = if original_width > max_width
        || original_height > max_height
    {
        // Calculate thumbnail dimensions maintaining aspect ratio
        let width_ratio = max_width as f32 / original_width as f32;
        let height_ratio = max_height as f32 / original_height as f32;
        let ratio = width_ratio.min(height_ratio);

        let thumb_width = (original_width as f32 * ratio) as u32;
        let thumb_height = (original_height as f32 * ratio) as u32;

        // Resize image
        let resized = dynamic_image.thumbnail(thumb_width, thumb_height);
        (thumb_width, thumb_height, resized)
    } else {
        // Use original image
        (original_width, original_height, dynamic_image)
    };

    // Convert to RGBA8 for egui::ColorImage
    let rgba8_image = thumbnail_image.to_rgba8();
    let (width, height) = rgba8_image.dimensions();
    let pixels = rgba8_image.into_raw();

    // Create egui::ColorImage from RGBA bytes
    // ColorImage expects pixels in row-major order, which matches image crate format
    // Size is [width, height] as an array
    let color_image = ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &pixels);

    Ok(PreviewData {
        image: color_image,
        original_width,
        original_height,
        preview_width,
        preview_height,
    })
}

/// Get or generate a cached preview for an image
///
/// This function checks the cache first, and if not found, generates
/// a new preview and caches it.
///
/// # Arguments
///
/// * `image_path` - Path to the image file
/// * `max_width` - Maximum preview width in pixels
/// * `max_height` - Maximum preview height in pixels
/// * `limits` - Resource limits for validation
/// * `cache` - Preview cache (wrapped in Arc<Mutex<>> for thread safety)
///
/// # Returns
///
/// `Ok(Arc<PreviewData>)` with the cached or newly generated preview.
pub fn get_or_generate_preview(
    image_path: &Path,
    max_width: u32,
    max_height: u32,
    limits: &ResourceLimits,
    cache: &Arc<Mutex<PreviewCache>>,
) -> std::result::Result<Arc<PreviewData>, PreviewError> {
    // Check cache first
    {
        let cache_guard = cache.lock().unwrap();
        if let Some(cached) = cache_guard.get(image_path) {
            return Ok(cached);
        }
    }

    // Generate new preview
    let preview = generate_image_preview(image_path, max_width, max_height, limits)?;

    // Store in cache and return
    let path_buf = image_path.to_path_buf();
    let preview_arc = Arc::new(preview);
    {
        let mut cache_guard = cache.lock().unwrap();
        // Insert the Arc (multiple references are fine)
        cache_guard.insert(path_buf, Arc::clone(&preview_arc));
    }
    Ok(preview_arc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_cache_creation() {
        let cache = PreviewCache::new();
        assert_eq!(cache.cache.len(), 0);
        assert_eq!(cache.max_entries, 50);
    }

    #[test]
    fn test_preview_cache_with_max_entries() {
        let cache = PreviewCache::with_max_entries(100);
        assert_eq!(cache.max_entries, 100);
    }

    // Note: Full integration tests would require actual image files
    // These are better suited for integration tests in converter-gui/tests/
}

/// Metadata extracted from a mesh file for preview display
///
/// For v0.2.2, mesh preview is simplified to metadata display only.
/// Full 3D preview viewer is deferred to v0.2.3.
#[derive(Debug, Clone)]
pub struct MeshMetadata {
    /// Number of vertices in the mesh
    pub vertex_count: usize,
    /// Number of faces in the mesh
    pub face_count: usize,
    /// Detected mesh format
    pub format: MeshFormat,
    /// Whether the mesh has normals
    pub has_normals: bool,
    /// Whether the mesh has UV coordinates (texture coordinates)
    pub has_uvs: bool,
}

/// Extract metadata from a mesh file for preview
///
/// This function loads the mesh file and extracts metadata without performing
/// a full conversion. It's designed to be fast for preview purposes.
///
/// # Arguments
///
/// * `mesh_path` - Path to the mesh file
/// * `limits` - Resource limits for security validation
///
/// # Returns
///
/// `Ok(MeshMetadata)` with mesh information, or an error if loading fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The file path is invalid or inaccessible
/// - The file size exceeds resource limits
/// - Format detection fails
/// - The mesh cannot be loaded
///
/// # Example
///
/// ```no_run
/// use converter_gui::ui::preview::get_mesh_metadata;
/// use common::limits::ResourceLimits;
/// use std::path::Path;
///
/// let limits = ResourceLimits::default();
/// let metadata = get_mesh_metadata(Path::new("model.stl"), &limits)?;
/// println!("Vertices: {}, Faces: {}", metadata.vertex_count, metadata.face_count);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn get_mesh_metadata(
    mesh_path: &Path,
    limits: &ResourceLimits,
) -> std::result::Result<MeshMetadata, PreviewError> {
    // Validate input file path (security check)
    validate_file_path(mesh_path)
        .map_err(|_| PreviewError::InvalidPath)?;

    // Read input file with size validation (DoS prevention)
    let input_data = std::fs::read(mesh_path)
        .map_err(|e| PreviewError::FileReadError(format!("Failed to read mesh file: {}", e)))?;

    // Check file size against limits
    if input_data.len() > limits.max_file_size {
        return Err(PreviewError::ImageTooLarge); // Reuse for "too large" error
    }

    // Detect mesh format
    let format = MeshFormatRegistry::detect_from_path(mesh_path)
        .map_err(|e| PreviewError::FileReadError(format!("Failed to detect mesh format: {}", e)))?;

    // Get reader with resource limits
    let mesh_limits = ResourceLimits::builder()
        .max_file_size(limits.max_file_size)
        .max_vertices(limits.max_vertices)
        .max_faces(limits.max_faces)
        .build();

    let reader = MeshFormatRegistry::get_reader_with_limits(format, mesh_limits)
        .map_err(|e| PreviewError::FileReadError(format!("Failed to get mesh reader: {}", e)))?;

    // Read mesh to extract metadata
    let mesh = reader.read(&input_data)
        .map_err(|e| PreviewError::FileReadError(format!("Failed to read mesh: {}", e)))?;

    // Extract metadata
    let vertex_count = mesh.vertices.len();
    let face_count = mesh.faces.len();
    let has_normals = !mesh.normals.is_empty();
    // Note: UV detection would require checking mesh format-specific data
    // For v0.2.2, we'll assume false (can be enhanced later)
    let has_uvs = false;

    Ok(MeshMetadata {
        vertex_count,
        face_count,
        format,
        has_normals,
        has_uvs,
    })
}

#[cfg(test)]
mod mesh_metadata_tests {
    use super::*;

    #[test]
    fn test_get_mesh_metadata_invalid_path() {
        let limits = ResourceLimits::default();
        let invalid_path = Path::new("../etc/passwd"); // Path traversal attempt

        let result = get_mesh_metadata(invalid_path, &limits);
        assert!(result.is_err());
        // Should be caught by validate_file_path
    }

    // Note: Full integration tests would require actual mesh files
    // These are handled in the mesh-core crate's integration tests
}

