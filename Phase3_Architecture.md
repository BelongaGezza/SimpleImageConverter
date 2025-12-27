# Phase 3: Detailed Architecture Design
## Rust Image and 3D Mesh Converters

**Date:** December 26, 2025  
**Language:** Rust 1.92.0  
**Target:** x86-64 Windows 11

---

## Table of Contents
1. [Workspace Structure](#1-workspace-structure)
2. [Module Architecture](#2-module-architecture)
3. [Data Structures](#3-data-structures)
4. [Trait Definitions](#4-trait-definitions)
5. [Error Handling](#5-error-handling)
6. [2D Converter Architecture](#6-2d-converter-architecture)
7. [3D Converter Architecture](#7-3d-converter-architecture)
8. [STEP Integration](#8-step-integration)
9. [CLI Interface Design](#9-cli-interface-design)
10. [Build Configuration](#10-build-configuration)
11. [Testing Strategy](#11-testing-strategy)
12. [Security Architecture](#12-security-architecture)

---

## 1. WORKSPACE STRUCTURE

### 1.1 Directory Layout

```
converter-workspace/
├── Cargo.toml                      # Workspace manifest
├── README.md
├── LICENSE
├── .github/
│   └── workflows/
│       ├── ci.yml                  # CI/CD pipeline
│       └── release.yml             # Release builds
│
├── common/                         # Shared utilities
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── error.rs               # Common error types
│   │   ├── progress.rs            # Progress reporting
│   │   ├── validation.rs          # File validation
│   │   └── io.rs                  # I/O utilities
│   └── tests/
│
├── img-core/                       # 2D image library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── formats/
│   │   │   ├── mod.rs
│   │   │   ├── traits.rs          # Format trait definitions
│   │   │   ├── raster.rs          # Raster format handlers
│   │   │   ├── vector.rs          # Vector rasterization
│   │   │   ├── png.rs
│   │   │   ├── jpg.rs
│   │   │   ├── bmp.rs
│   │   │   ├── gif.rs
│   │   │   ├── tiff.rs
│   │   │   ├── webp.rs
│   │   │   ├── svg.rs             # Phase 2
│   │   │   ├── avif.rs            # Phase 2
│   │   │   └── exr.rs             # Phase 2
│   │   ├── convert.rs             # Conversion orchestration
│   │   ├── quality.rs             # Quality settings
│   │   ├── color.rs               # Color space handling
│   │   └── metadata.rs            # EXIF/metadata
│   ├── tests/
│   │   ├── integration.rs
│   │   └── test_data/
│   └── benches/
│       └── conversion_bench.rs
│
├── img-convert/                    # 2D CLI binary
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── cli.rs                 # CLI argument parsing
│   └── tests/
│       └── cli_tests.rs
│
├── mesh-core/                      # 3D mesh library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── formats/
│   │   │   ├── mod.rs
│   │   │   ├── traits.rs          # Format trait definitions
│   │   │   ├── stl.rs
│   │   │   ├── obj.rs
│   │   │   ├── ply.rs
│   │   │   ├── off.rs             # Custom parser
│   │   │   ├── gltf.rs            # Phase 2
│   │   │   ├── dxf.rs             # Phase 2
│   │   │   └── step.rs            # Phase 3 (truck)
│   │   ├── mesh/
│   │   │   ├── mod.rs
│   │   │   ├── data.rs            # Mesh data structures
│   │   │   ├── builder.rs         # Mesh builder pattern
│   │   │   ├── validate.rs        # Mesh validation
│   │   │   └── transform.rs       # Coordinate transforms
│   │   ├── convert.rs             # Conversion orchestration
│   │   └── normal.rs              # Normal calculation
│   ├── tests/
│   │   ├── integration.rs
│   │   └── test_data/
│   └── benches/
│       └── conversion_bench.rs
│
├── mesh-convert/                   # 3D CLI binary
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── cli.rs                 # CLI argument parsing
│   └── tests/
│       └── cli_tests.rs
│
└── converter-gui/                  # Phase 4: GUI
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── app.rs
    │   └── ui/
    │       ├── image_tab.rs
    │       └── mesh_tab.rs
    └── assets/
        └── icons/
```

### 1.2 Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "common",
    "img-core",
    "img-convert",
    "mesh-core",
    "mesh-convert",
    # "converter-gui",  # Phase 4
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/BelongaGezza/SimpleImageConverter"

[workspace.dependencies]
# Common dependencies
anyhow = "1.0"
thiserror = "1.0"
clap = { version = "4.5", features = ["derive"] }
indicatif = "0.17"
log = "0.4"
env_logger = "0.11"

# 2D image processing
image = "0.25"
imageproc = "0.25"
webp = "0.3"
ravif = "0.11"
exr = "1.72"
resvg = "0.44"

# 3D mesh processing
stl_io = "0.7"
tobj = "4.0"
ply-rs = "0.1"
gltf = "1.4"
dxf = "0.6"

# STEP support (Phase 3)
truck-modeling = "0.4"
truck-polymesh = "0.4"
truck-stepio = "0.4"

# Utilities
nalgebra = "0.33"

[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
strip = true           # Strip symbols
panic = "abort"        # Smaller panic handler

[profile.dev]
opt-level = 0
debug = true

[profile.bench]
inherits = "release"
debug = true
```

---

## 2. MODULE ARCHITECTURE

### 2.1 Common Module

```
common/
├── error.rs        # Shared error types
├── limits.rs       # Resource limits configuration (NEW)
├── progress.rs     # Progress reporting trait
├── validation.rs   # File validation utilities
└── io.rs          # I/O helpers (with size validation)
```

**Purpose:** Shared utilities used by both img-core and mesh-core

**Key Types:**
- `ConversionError` - Common error enum
- `ResourceLimits` - Centralized resource limits (NEW)
- `ProgressReporter` - Trait for progress callbacks
- `FileValidator` - File existence/format checks

### 2.2 Image Core Module

```
img-core/
├── formats/
│   ├── traits.rs      # ImageFormat, ImageReader, ImageWriter traits
│   ├── raster.rs      # Common raster format logic
│   └── {format}.rs    # Per-format implementations
├── convert.rs         # ImageConverter struct
├── quality.rs         # QualitySettings, CompressionLevel
├── color.rs           # ColorSpace, ColorConverter
└── metadata.rs        # MetadataHandler
```

**Core Abstractions:**
- `ImageFormat` trait - Unified format interface
- `ImageData` struct - In-memory representation
- `ImageConverter` - Orchestrates conversions

### 2.3 Mesh Core Module

```
mesh-core/
├── formats/
│   ├── traits.rs      # MeshFormat, MeshReader, MeshWriter traits
│   └── {format}.rs    # Per-format implementations
├── mesh/
│   ├── data.rs        # Mesh, Vertex, Face structs
│   ├── builder.rs     # MeshBuilder pattern
│   ├── validate.rs    # Manifold checks, topology validation
│   └── transform.rs   # Coordinate system transforms
├── convert.rs         # MeshConverter struct
└── normal.rs          # Normal recalculation
```

**Core Abstractions:**
- `MeshFormat` trait - Unified format interface
- `Mesh` struct - In-memory representation
- `MeshConverter` - Orchestrates conversions

---

## 3. DATA STRUCTURES

### 3.1 Image Data Structures

```rust
// common/src/lib.rs
pub type Result<T> = std::result::Result<T, ConversionError>;

// img-core/src/lib.rs
use image::{DynamicImage, ImageBuffer, Rgba};

/// In-memory image representation
pub struct ImageData {
    /// Underlying image data from `image` crate
    pub image: DynamicImage,
    
    /// Optional metadata (EXIF, etc.)
    pub metadata: Option<Metadata>,
    
    /// Original format
    pub source_format: ImageFormatType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormatType {
    Png,
    Jpeg,
    Bmp,
    Gif,
    Tiff,
    WebP,
    Tga,
    Ico,
    Dds,
    Hdr,
    OpenExr,
    Avif,
    Svg,
}

pub struct QualitySettings {
    /// JPEG/WebP quality (1-100)
    pub jpeg_quality: u8,
    
    /// PNG compression level (0-9)
    pub png_compression: u8,
    
    /// Strip metadata
    pub strip_metadata: bool,
}

impl Default for QualitySettings {
    fn default() -> Self {
        Self {
            jpeg_quality: 95,
            png_compression: 6,
            strip_metadata: false,
        }
    }
}

pub struct ConversionOptions {
    pub quality: QualitySettings,
    pub output_path: Option<PathBuf>,
    pub dpi: Option<u32>,  // For vector rasterization
}
```

### 3.2 Mesh Data Structures

```rust
// mesh-core/src/mesh/data.rs
use nalgebra::Point3;

/// 3D mesh representation
pub struct Mesh {
    /// Vertex positions
    pub vertices: Vec<Vertex>,
    
    /// Vertex normals (optional)
    pub normals: Vec<Normal>,
    
    /// Texture coordinates (optional)
    pub uvs: Vec<UV>,
    
    /// Triangle faces
    pub faces: Vec<Face>,
    
    /// Materials (for formats that support them)
    pub materials: Vec<Material>,
    
    /// Metadata
    pub metadata: MeshMetadata,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Point3<f32>> for Vertex {
    fn from(p: Point3<f32>) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct UV {
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone)]
pub struct Face {
    /// Vertex indices (always triangle)
    pub vertices: [usize; 3],
    
    /// Normal indices (optional)
    pub normals: Option<[usize; 3]>,
    
    /// UV indices (optional)
    pub uvs: Option<[usize; 3]>,
    
    /// Material index (optional)
    pub material: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub ambient_color: [f32; 3],
    pub texture_map: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshFormatType {
    Stl,
    Obj,
    Ply,
    Off,
    Gltf,
    Dxf,
    Step,
}

pub struct MeshMetadata {
    pub source_format: MeshFormatType,
    pub vertex_count: usize,
    pub face_count: usize,
    pub has_normals: bool,
    pub has_uvs: bool,
    pub has_colors: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSystem {
    YUp,
    ZUp,
}

pub struct ConversionOptions {
    pub output_path: Option<PathBuf>,
    pub format_options: FormatOptions,
    pub transform: Option<CoordinateTransform>,
    pub recalculate_normals: bool,
    pub validate: bool,
}

#[derive(Debug, Clone)]
pub enum FormatOptions {
    Stl { binary: bool },
    Ply { binary: bool },
    Obj { include_mtl: bool },
    Off,
    Gltf { binary: bool },
    Dxf,
    Step,
}

pub struct CoordinateTransform {
    pub from: CoordinateSystem,
    pub to: CoordinateSystem,
}
```

---

## 4. TRAIT DEFINITIONS

### 4.1 Image Format Traits

```rust
// img-core/src/formats/traits.rs
use crate::{ImageData, ImageFormatType, ConversionOptions, Result};
use std::path::Path;
use std::io::{Read, Write};

/// Trait for image format readers
pub trait ImageReader {
    /// Read image from file
    fn read_from_file(&self, path: &Path) -> Result<ImageData>;
    
    /// Read image from bytes
    fn read_from_bytes(&self, data: &[u8]) -> Result<ImageData>;
    
    /// Get supported format
    fn format_type(&self) -> ImageFormatType;
}

/// Trait for image format writers
pub trait ImageWriter {
    /// Write image to file
    fn write_to_file(
        &self,
        image: &ImageData,
        path: &Path,
        options: &ConversionOptions,
    ) -> Result<()>;
    
    /// Write image to bytes
    fn write_to_bytes(
        &self,
        image: &ImageData,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>>;
    
    /// Get supported format
    fn format_type(&self) -> ImageFormatType;
}

/// Combined format trait
pub trait ImageFormat: ImageReader + ImageWriter {
    /// Get format information
    fn info(&self) -> FormatInfo;
}

pub struct FormatInfo {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub supports_transparency: bool,
    pub supports_animation: bool,
    pub is_lossy: bool,
}

/// Format registry
pub struct FormatRegistry {
    readers: HashMap<ImageFormatType, Box<dyn ImageReader>>,
    writers: HashMap<ImageFormatType, Box<dyn ImageWriter>>,
}

impl FormatRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            readers: HashMap::new(),
            writers: HashMap::new(),
        };
        
        // Register built-in formats
        registry.register_format(PngFormat::new());
        registry.register_format(JpegFormat::new());
        // ... etc
        
        registry
    }
    
    pub fn register_format<F: ImageFormat + 'static>(&mut self, format: F) {
        let format_type = format.format_type();
        self.readers.insert(format_type, Box::new(format.clone()));
        self.writers.insert(format_type, Box::new(format));
    }
    
    pub fn get_reader(&self, format: ImageFormatType) -> Option<&dyn ImageReader> {
        self.readers.get(&format).map(|b| b.as_ref())
    }
    
    pub fn get_writer(&self, format: ImageFormatType) -> Option<&dyn ImageWriter> {
        self.writers.get(&format).map(|b| b.as_ref())
    }
}
```

### 4.2 Mesh Format Traits

```rust
// mesh-core/src/formats/traits.rs
use crate::{Mesh, MeshFormatType, ConversionOptions, Result};
use std::path::Path;

/// Trait for mesh format readers
pub trait MeshReader {
    /// Read mesh from file
    fn read_from_file(&self, path: &Path) -> Result<Mesh>;
    
    /// Read mesh from bytes
    fn read_from_bytes(&self, data: &[u8]) -> Result<Mesh>;
    
    /// Get supported format
    fn format_type(&self) -> MeshFormatType;
}

/// Trait for mesh format writers
pub trait MeshWriter {
    /// Write mesh to file
    fn write_to_file(
        &self,
        mesh: &Mesh,
        path: &Path,
        options: &ConversionOptions,
    ) -> Result<()>;
    
    /// Write mesh to bytes
    fn write_to_bytes(
        &self,
        mesh: &Mesh,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>>;
    
    /// Get supported format
    fn format_type(&self) -> MeshFormatType;
}

/// Combined format trait
pub trait MeshFormat: MeshReader + MeshWriter {
    /// Get format information
    fn info(&self) -> FormatInfo;
}

pub struct FormatInfo {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub supports_normals: bool,
    pub supports_uvs: bool,
    pub supports_colors: bool,
    pub supports_materials: bool,
    pub is_binary: bool,
}

/// Format registry
pub struct FormatRegistry {
    readers: HashMap<MeshFormatType, Box<dyn MeshReader>>,
    writers: HashMap<MeshFormatType, Box<dyn MeshWriter>>,
}

impl FormatRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            readers: HashMap::new(),
            writers: HashMap::new(),
        };
        
        // Register built-in formats
        registry.register_format(StlFormat::new());
        registry.register_format(ObjFormat::new());
        // ... etc
        
        registry
    }
    
    pub fn register_format<F: MeshFormat + 'static>(&mut self, format: F) {
        let format_type = format.format_type();
        self.readers.insert(format_type, Box::new(format.clone()));
        self.writers.insert(format_type, Box::new(format));
    }
}
```

---

## 5. ERROR HANDLING

### 5.1 Error Type Hierarchy

```rust
// common/src/error.rs
use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Image processing error: {0}")]
    ImageError(String),
    
    #[error("Mesh processing error: {0}")]
    MeshError(String),
    
    #[error("Invalid mesh: {0}")]
    InvalidMesh(String),
    
    #[error("Format conversion not supported: {from} to {to}")]
    UnsupportedConversion { from: String, to: String },
    
    #[error("Quality setting out of range: {0}")]
    InvalidQuality(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("STEP error: {0}")]
    StepError(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// Conversion from image crate errors
impl From<image::ImageError> for ConversionError {
    fn from(err: image::ImageError) -> Self {
        ConversionError::ImageError(err.to_string())
    }
}

// Helper result type
pub type Result<T> = std::result::Result<T, ConversionError>;
```

### 5.2 Error Handling Patterns

```rust
// Example usage in converter
pub fn convert_image(
    input: &Path,
    output_format: ImageFormatType,
    options: &ConversionOptions,
) -> Result<PathBuf> {
    // Validate input
    if !input.exists() {
        return Err(ConversionError::FileNotFound(input.to_path_buf()));
    }
    
    // Detect format
    let input_format = detect_image_format(input)?;
    
    // Get reader and writer
    let registry = FormatRegistry::new();
    let reader = registry
        .get_reader(input_format)
        .ok_or_else(|| ConversionError::UnsupportedFormat(format!("{:?}", input_format)))?;
    let writer = registry
        .get_writer(output_format)
        .ok_or_else(|| ConversionError::UnsupportedFormat(format!("{:?}", output_format)))?;
    
    // Read
    let image_data = reader.read_from_file(input)?;
    
    // Determine output path
    let output_path = options
        .output_path
        .clone()
        .unwrap_or_else(|| input.with_extension(output_format.extension()));
    
    // Write
    writer.write_to_file(&image_data, &output_path, options)?;
    
    Ok(output_path)
}
```

---

## 6. 2D CONVERTER ARCHITECTURE

### 6.1 Image Converter Core

```rust
// img-core/src/convert.rs
use crate::{
    ImageData, ImageFormatType, ConversionOptions, QualitySettings,
    formats::{FormatRegistry, ImageReader, ImageWriter},
    Result,
};
use std::path::{Path, PathBuf};

pub struct ImageConverter {
    registry: FormatRegistry,
}

impl ImageConverter {
    pub fn new() -> Self {
        Self {
            registry: FormatRegistry::new(),
        }
    }
    
    /// Convert image from one format to another
    pub fn convert(
        &self,
        input: &Path,
        output_format: ImageFormatType,
        options: &ConversionOptions,
    ) -> Result<PathBuf> {
        // Detect input format
        let input_format = self.detect_format(input)?;
        
        // Load image
        let image_data = self.load_image(input, input_format)?;
        
        // Determine output path
        let output_path = options
            .output_path
            .clone()
            .unwrap_or_else(|| self.default_output_path(input, output_format));
        
        // Save image
        self.save_image(&image_data, &output_path, output_format, options)?;
        
        Ok(output_path)
    }
    
    /// Detect format from file extension or magic bytes
    pub fn detect_format(&self, path: &Path) -> Result<ImageFormatType> {
        // First try extension
        if let Some(ext) = path.extension() {
            if let Some(format) = ImageFormatType::from_extension(ext.to_str().unwrap()) {
                return Ok(format);
            }
        }
        
        // Fallback to magic bytes
        self.detect_format_from_bytes(path)
    }
    
    fn detect_format_from_bytes(&self, path: &Path) -> Result<ImageFormatType> {
        use std::fs::File;
        use std::io::Read;
        
        let mut file = File::open(path)?;
        let mut header = [0u8; 16];
        file.read_exact(&mut header)?;
        
        // Check magic bytes
        if header.starts_with(b"\x89PNG") {
            Ok(ImageFormatType::Png)
        } else if header.starts_with(&[0xFF, 0xD8, 0xFF]) {
            Ok(ImageFormatType::Jpeg)
        } else if header.starts_with(b"BM") {
            Ok(ImageFormatType::Bmp)
        } else if header.starts_with(b"GIF8") {
            Ok(ImageFormatType::Gif)
        } else if header.starts_with(b"RIFF") && &header[8..12] == b"WEBP" {
            Ok(ImageFormatType::WebP)
        } else {
            Err(ConversionError::InvalidFormat(
                "Unable to detect image format".into()
            ))
        }
    }
    
    fn load_image(&self, path: &Path, format: ImageFormatType) -> Result<ImageData> {
        let reader = self
            .registry
            .get_reader(format)
            .ok_or_else(|| ConversionError::UnsupportedFormat(format!("{:?}", format)))?;
        
        reader.read_from_file(path)
    }
    
    fn save_image(
        &self,
        image: &ImageData,
        path: &Path,
        format: ImageFormatType,
        options: &ConversionOptions,
    ) -> Result<()> {
        let writer = self
            .registry
            .get_writer(format)
            .ok_or_else(|| ConversionError::UnsupportedFormat(format!("{:?}", format)))?;
        
        writer.write_to_file(image, path, options)
    }
    
    fn default_output_path(&self, input: &Path, output_format: ImageFormatType) -> PathBuf {
        input.with_extension(output_format.extension())
    }
}

impl ImageFormatType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "bmp" => Some(Self::Bmp),
            "gif" => Some(Self::Gif),
            "tiff" | "tif" => Some(Self::Tiff),
            "webp" => Some(Self::WebP),
            "tga" => Some(Self::Tga),
            "ico" => Some(Self::Ico),
            "dds" => Some(Self::Dds),
            "hdr" => Some(Self::Hdr),
            "exr" => Some(Self::OpenExr),
            "avif" => Some(Self::Avif),
            "svg" => Some(Self::Svg),
            _ => None,
        }
    }
    
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Bmp => "bmp",
            Self::Gif => "gif",
            Self::Tiff => "tiff",
            Self::WebP => "webp",
            Self::Tga => "tga",
            Self::Ico => "ico",
            Self::Dds => "dds",
            Self::Hdr => "hdr",
            Self::OpenExr => "exr",
            Self::Avif => "avif",
            Self::Svg => "svg",
        }
    }
}
```

### 6.2 Format Implementation Example (PNG)

```rust
// img-core/src/formats/png.rs
use crate::{
    ImageData, ImageFormatType, ConversionOptions, Result, ConversionError,
    formats::{ImageReader, ImageWriter, ImageFormat, FormatInfo},
};
use image::{DynamicImage, ImageFormat as ImageCrateFormat};
use std::path::Path;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct PngFormat;

impl PngFormat {
    pub fn new() -> Self {
        Self
    }
}

impl ImageReader for PngFormat {
    fn read_from_file(&self, path: &Path) -> Result<ImageData> {
        let image = image::open(path)?;
        Ok(ImageData {
            image,
            metadata: None, // TODO: Extract EXIF
            source_format: ImageFormatType::Png,
        })
    }
    
    fn read_from_bytes(&self, data: &[u8]) -> Result<ImageData> {
        let image = image::load_from_memory_with_format(
            data,
            ImageCrateFormat::Png
        )?;
        Ok(ImageData {
            image,
            metadata: None,
            source_format: ImageFormatType::Png,
        })
    }
    
    fn format_type(&self) -> ImageFormatType {
        ImageFormatType::Png
    }
}

impl ImageWriter for PngFormat {
    fn write_to_file(
        &self,
        image: &ImageData,
        path: &Path,
        options: &ConversionOptions,
    ) -> Result<()> {
        use image::codecs::png::{PngEncoder, CompressionType, FilterType};
        use std::fs::File;
        
        let file = File::create(path)?;
        let encoder = PngEncoder::new_with_quality(
            file,
            CompressionType::Default,
            FilterType::Adaptive,
        );
        
        let (width, height) = image.image.dimensions();
        let color_type = image.image.color();
        
        encoder.write_image(
            image.image.as_bytes(),
            width,
            height,
            color_type,
        )?;
        
        Ok(())
    }
    
    fn write_to_bytes(
        &self,
        image: &ImageData,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        image.image.write_to(&mut buffer, ImageCrateFormat::Png)?;
        Ok(buffer)
    }
    
    fn format_type(&self) -> ImageFormatType {
        ImageFormatType::Png
    }
}

impl ImageFormat for PngFormat {
    fn info(&self) -> FormatInfo {
        FormatInfo {
            name: "PNG",
            extensions: &["png"],
            supports_transparency: true,
            supports_animation: false,
            is_lossy: false,
        }
    }
}
```

---

## 7. 3D CONVERTER ARCHITECTURE

### 7.1 Mesh Converter Core

```rust
// mesh-core/src/convert.rs
use crate::{
    Mesh, MeshFormatType, ConversionOptions, FormatOptions,
    formats::{FormatRegistry, MeshReader, MeshWriter},
    mesh::{validate_mesh, transform_coordinates, recalculate_normals},
    Result, ConversionError,
};
use std::path::{Path, PathBuf};

pub struct MeshConverter {
    registry: FormatRegistry,
}

impl MeshConverter {
    pub fn new() -> Self {
        Self {
            registry: FormatRegistry::new(),
        }
    }
    
    /// Convert mesh from one format to another
    pub fn convert(
        &self,
        input: &Path,
        output_format: MeshFormatType,
        options: &ConversionOptions,
    ) -> Result<PathBuf> {
        // Detect input format
        let input_format = self.detect_format(input)?;
        
        // Load mesh
        let mut mesh = self.load_mesh(input, input_format)?;
        
        // Apply transformations if requested
        if let Some(transform) = &options.transform {
            mesh = transform_coordinates(mesh, transform)?;
        }
        
        // Recalculate normals if requested
        if options.recalculate_normals {
            mesh = recalculate_normals(mesh)?;
        }
        
        // Validate if requested
        if options.validate {
            validate_mesh(&mesh)?;
        }
        
        // Determine output path
        let output_path = options
            .output_path
            .clone()
            .unwrap_or_else(|| self.default_output_path(input, output_format));
        
        // Save mesh
        self.save_mesh(&mesh, &output_path, output_format, options)?;
        
        Ok(output_path)
    }
    
    pub fn detect_format(&self, path: &Path) -> Result<MeshFormatType> {
        if let Some(ext) = path.extension() {
            if let Some(format) = MeshFormatType::from_extension(ext.to_str().unwrap()) {
                return Ok(format);
            }
        }
        
        Err(ConversionError::UnsupportedFormat(
            format!("Unknown format for file: {}", path.display())
        ))
    }
    
    fn load_mesh(&self, path: &Path, format: MeshFormatType) -> Result<Mesh> {
        let reader = self
            .registry
            .get_reader(format)
            .ok_or_else(|| ConversionError::UnsupportedFormat(format!("{:?}", format)))?;
        
        reader.read_from_file(path)
    }
    
    fn save_mesh(
        &self,
        mesh: &Mesh,
        path: &Path,
        format: MeshFormatType,
        options: &ConversionOptions,
    ) -> Result<()> {
        let writer = self
            .registry
            .get_writer(format)
            .ok_or_else(|| ConversionError::UnsupportedFormat(format!("{:?}", format)))?;
        
        writer.write_to_file(mesh, path, options)
    }
    
    fn default_output_path(&self, input: &Path, output_format: MeshFormatType) -> PathBuf {
        input.with_extension(output_format.extension())
    }
}

impl MeshFormatType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "stl" => Some(Self::Stl),
            "obj" => Some(Self::Obj),
            "ply" => Some(Self::Ply),
            "off" => Some(Self::Off),
            "gltf" | "glb" => Some(Self::Gltf),
            "dxf" => Some(Self::Dxf),
            "step" | "stp" => Some(Self::Step),
            _ => None,
        }
    }
    
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Stl => "stl",
            Self::Obj => "obj",
            Self::Ply => "ply",
            Self::Off => "off",
            Self::Gltf => "gltf",
            Self::Dxf => "dxf",
            Self::Step => "step",
        }
    }
}
```

### 7.2 Mesh Builder Pattern

```rust
// mesh-core/src/mesh/builder.rs
use crate::{Mesh, Vertex, Normal, UV, Face, Material, MeshMetadata, MeshFormatType, Result};

pub struct MeshBuilder {
    vertices: Vec<Vertex>,
    normals: Vec<Normal>,
    uvs: Vec<UV>,
    faces: Vec<Face>,
    materials: Vec<Material>,
    source_format: MeshFormatType,
}

impl MeshBuilder {
    pub fn new(source_format: MeshFormatType) -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            faces: Vec::new(),
            materials: Vec::new(),
            source_format,
        }
    }
    
    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }
    
    pub fn add_normal(&mut self, normal: Normal) -> usize {
        let index = self.normals.len();
        self.normals.push(normal);
        index
    }
    
    pub fn add_uv(&mut self, uv: UV) -> usize {
        let index = self.uvs.len();
        self.uvs.push(uv);
        index
    }
    
    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }
    
    pub fn add_material(&mut self, material: Material) -> usize {
        let index = self.materials.len();
        self.materials.push(material);
        index
    }
    
    pub fn build(self) -> Result<Mesh> {
        let metadata = MeshMetadata {
            source_format: self.source_format,
            vertex_count: self.vertices.len(),
            face_count: self.faces.len(),
            has_normals: !self.normals.is_empty(),
            has_uvs: !self.uvs.is_empty(),
            has_colors: false, // TODO: Add color support
        };
        
        Ok(Mesh {
            vertices: self.vertices,
            normals: self.normals,
            uvs: self.uvs,
            faces: self.faces,
            materials: self.materials,
            metadata,
        })
    }
}
```

### 7.3 Format Implementation Example (STL)

```rust
// mesh-core/src/formats/stl.rs
use crate::{
    Mesh, MeshFormatType, ConversionOptions, FormatOptions, Result, ConversionError,
    formats::{MeshReader, MeshWriter, MeshFormat, FormatInfo},
    mesh::{MeshBuilder, Vertex, Normal, Face},
};
use std::path::Path;
use stl_io::{self, Triangle, Vertex as StlVertex};

#[derive(Clone)]
pub struct StlFormat;

impl StlFormat {
    pub fn new() -> Self {
        Self
    }
}

impl MeshReader for StlFormat {
    fn read_from_file(&self, path: &Path) -> Result<Mesh> {
        use std::fs::OpenOptions;
        
        let mut file = OpenOptions::new().read(true).open(path)?;
        let stl_mesh = stl_io::read_stl(&mut file)?;
        
        self.convert_from_stl(stl_mesh)
    }
    
    fn read_from_bytes(&self, data: &[u8]) -> Result<Mesh> {
        use std::io::Cursor;
        
        let mut cursor = Cursor::new(data);
        let stl_mesh = stl_io::read_stl(&mut cursor)?;
        
        self.convert_from_stl(stl_mesh)
    }
    
    fn format_type(&self) -> MeshFormatType {
        MeshFormatType::Stl
    }
}

impl MeshWriter for StlFormat {
    fn write_to_file(
        &self,
        mesh: &Mesh,
        path: &Path,
        options: &ConversionOptions,
    ) -> Result<()> {
        use std::fs::File;
        
        let triangles = self.convert_to_stl(mesh)?;
        
        let mut file = File::create(path)?;
        
        // Check if binary format requested
        let binary = match &options.format_options {
            FormatOptions::Stl { binary } => *binary,
            _ => true, // Default to binary
        };
        
        if binary {
            stl_io::write_stl(&mut file, triangles.iter())?;
        } else {
            // Write ASCII
            stl_io::write_stl(&mut file, triangles.iter())?;
        }
        
        Ok(())
    }
    
    fn write_to_bytes(
        &self,
        mesh: &Mesh,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>> {
        use std::io::Cursor;
        
        let triangles = self.convert_to_stl(mesh)?;
        let mut buffer = Cursor::new(Vec::new());
        
        stl_io::write_stl(&mut buffer, triangles.iter())?;
        
        Ok(buffer.into_inner())
    }
    
    fn format_type(&self) -> MeshFormatType {
        MeshFormatType::Stl
    }
}

impl MeshFormat for StlFormat {
    fn info(&self) -> FormatInfo {
        FormatInfo {
            name: "STL",
            extensions: &["stl"],
            supports_normals: true,
            supports_uvs: false,
            supports_colors: false,
            supports_materials: false,
            is_binary: true,
        }
    }
}

impl StlFormat {
    fn convert_from_stl(&self, stl_mesh: stl_io::IndexedMesh) -> Result<Mesh> {
        let mut builder = MeshBuilder::new(MeshFormatType::Stl);
        
        // Add vertices
        for vertex in stl_mesh.vertices {
            builder.add_vertex(Vertex {
                x: vertex[0],
                y: vertex[1],
                z: vertex[2],
            });
        }
        
        // Add faces (STL is triangle-only)
        for face in stl_mesh.faces {
            builder.add_face(Face {
                vertices: [
                    face.vertices[0],
                    face.vertices[1],
                    face.vertices[2],
                ],
                normals: Some([
                    builder.add_normal(Normal {
                        x: face.normal[0],
                        y: face.normal[1],
                        z: face.normal[2],
                    }),
                    0,
                    0,
                ]),
                uvs: None,
                material: None,
            });
        }
        
        builder.build()
    }
    
    fn convert_to_stl(&self, mesh: &Mesh) -> Result<Vec<Triangle>> {
        let mut triangles = Vec::with_capacity(mesh.faces.len());
        
        for face in &mesh.faces {
            let v0 = mesh.vertices[face.vertices[0]];
            let v1 = mesh.vertices[face.vertices[1]];
            let v2 = mesh.vertices[face.vertices[2]];
            
            // Calculate normal if not provided
            let normal = if let Some(normal_indices) = face.normals {
                let n = mesh.normals[normal_indices[0]];
                [n.x, n.y, n.z]
            } else {
                self.calculate_face_normal(&v0, &v1, &v2)
            };
            
            triangles.push(Triangle {
                normal,
                vertices: [
                    [v0.x, v0.y, v0.z],
                    [v1.x, v1.y, v1.z],
                    [v2.x, v2.y, v2.z],
                ],
            });
        }
        
        Ok(triangles)
    }
    
    fn calculate_face_normal(&self, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> [f32; 3] {
        use nalgebra::Vector3;
        
        let a = Vector3::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let b = Vector3::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        let normal = a.cross(&b).normalize();
        
        [normal.x, normal.y, normal.z]
    }
}

// Implement From for stl_io error
impl From<stl_io::Error> for ConversionError {
    fn from(err: stl_io::Error) -> Self {
        ConversionError::MeshError(err.to_string())
    }
}
```

---

## 8. STEP INTEGRATION

### 8.1 truck-based STEP Support (Option B - Primary)

```rust
// mesh-core/src/formats/step.rs
use crate::{
    Mesh, MeshFormatType, ConversionOptions, Result, ConversionError,
    formats::{MeshReader, MeshWriter, MeshFormat, FormatInfo},
    mesh::MeshBuilder,
};
use std::path::Path;
use truck_modeling::*;
use truck_polymesh::*;
use truck_stepio::*;

#[derive(Clone)]
pub struct StepFormat;

impl StepFormat {
    pub fn new() -> Self {
        Self
    }
}

impl MeshReader for StepFormat {
    fn read_from_file(&self, path: &Path) -> Result<Mesh> {
        // Read STEP file using truck
        let step_data = std::fs::read_to_string(path)?;
        let shapes = r#try!(
            truck_stepio::read(&step_data)
                .map_err(|e| ConversionError::StepError(format!("Failed to parse STEP: {}", e)))
        );
        
        // Convert truck shapes to our Mesh format
        self.convert_truck_to_mesh(shapes)
    }
    
    fn read_from_bytes(&self, data: &[u8]) -> Result<Mesh> {
        let step_data = String::from_utf8(data.to_vec())
            .map_err(|e| ConversionError::ParseError(format!("Invalid UTF-8: {}", e)))?;
        
        let shapes = truck_stepio::read(&step_data)
            .map_err(|e| ConversionError::StepError(format!("Failed to parse STEP: {}", e)))?;
        
        self.convert_truck_to_mesh(shapes)
    }
    
    fn format_type(&self) -> MeshFormatType {
        MeshFormatType::Step
    }
}

impl MeshWriter for StepFormat {
    fn write_to_file(
        &self,
        mesh: &Mesh,
        path: &Path,
        options: &ConversionOptions,
    ) -> Result<()> {
        // Convert our Mesh to truck format
        let shapes = self.convert_mesh_to_truck(mesh)?;
        
        // Write STEP file
        let step_data = truck_stepio::write(&shapes)
            .map_err(|e| ConversionError::StepError(format!("Failed to write STEP: {}", e)))?;
        
        std::fs::write(path, step_data)?;
        
        Ok(())
    }
    
    fn write_to_bytes(
        &self,
        mesh: &Mesh,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>> {
        let shapes = self.convert_mesh_to_truck(mesh)?;
        
        let step_data = truck_stepio::write(&shapes)
            .map_err(|e| ConversionError::StepError(format!("Failed to write STEP: {}", e)))?;
        
        Ok(step_data.into_bytes())
    }
    
    fn format_type(&self) -> MeshFormatType {
        MeshFormatType::Step
    }
}

impl MeshFormat for StepFormat {
    fn info(&self) -> FormatInfo {
        FormatInfo {
            name: "STEP",
            extensions: &["step", "stp"],
            supports_normals: true,
            supports_uvs: false,
            supports_colors: false,
            supports_materials: false,
            is_binary: false,
        }
    }
}

impl StepFormat {
    fn convert_truck_to_mesh(&self, shapes: Vec<truck_modeling::Shell>) -> Result<Mesh> {
        use truck_polymesh::prelude::*;
        
        let mut builder = MeshBuilder::new(MeshFormatType::Step);
        
        // Convert each shell to polygonal mesh
        for shell in shapes {
            // Tessellate the shell
            let mesh = shell.triangulation(0.01); // Tolerance parameter
            
            // Extract vertices and faces
            let positions = mesh.positions();
            let faces = mesh.faces();
            
            // Add vertices
            let vertex_offset = builder.vertices.len();
            for pos in positions.iter() {
                builder.add_vertex(crate::mesh::Vertex {
                    x: pos.x as f32,
                    y: pos.y as f32,
                    z: pos.z as f32,
                });
            }
            
            // Add faces
            for face in faces.iter() {
                builder.add_face(crate::mesh::Face {
                    vertices: [
                        vertex_offset + face[0],
                        vertex_offset + face[1],
                        vertex_offset + face[2],
                    ],
                    normals: None,
                    uvs: None,
                    material: None,
                });
            }
        }
        
        builder.build()
    }
    
    fn convert_mesh_to_truck(&self, mesh: &Mesh) -> Result<Vec<truck_modeling::Shell>> {
        // This is a simplified conversion
        // Full STEP export requires proper B-rep reconstruction
        
        // For now, create a simple shell from the triangle mesh
        // This loses parametric information but maintains geometry
        
        // TODO: Implement proper B-rep reconstruction for parametric surfaces
        
        Err(ConversionError::StepError(
            "STEP export from mesh not yet fully implemented. Use for import only.".into()
        ))
    }
}
```

### 8.2 OCCT FFI Fallback (Option A - Risk Mitigation)

```rust
// mesh-core/src/formats/step_occt.rs
// This is kept as documentation for fallback option

/*
OCCT FFI FALLBACK APPROACH

If `truck` STEP support proves insufficient, we can fall back to Open CASCADE.

Dependencies:
```toml
[dependencies]
opencascade = "0.1"  # Rust bindings to OCCT
opencascade-sys = "0.1"  # Low-level FFI
```

Build Requirements:
- OCCT 7.7+ installed
- CMake 3.18+
- C++17 compiler
- Platform libraries (X11 on Linux, etc.)

Implementation:
```rust
use opencascade::prelude::*;
use opencascade::step::{StepReader, StepWriter};

pub struct StepFormatOCCT;

impl MeshReader for StepFormatOCCT {
    fn read_from_file(&self, path: &Path) -> Result<Mesh> {
        let reader = StepReader::new();
        let shape = reader.read(path)?;
        
        // Convert OCCT TopoDS_Shape to our Mesh
        self.tessellate_shape(shape)
    }
}

impl StepFormatOCCT {
    fn tessellate_shape(&self, shape: TopoDS_Shape) -> Result<Mesh> {
        // Use OCCT tessellation
        let tessellator = BRepMesh_IncrementalMesh::new(shape, 0.1);
        tessellator.perform();
        
        // Extract triangulation
        // ...
    }
}
```

Binary Size Impact: +10-15 MB

Deployment Impact:
- Windows: Requires OCCT DLLs or static linking
- Complex build process
- CI/CD complexity increased

Use Only If:
1. truck STEP support insufficient for real-world files
2. Customer specifically needs full STEP AP214/AP242 support
3. Willing to accept larger binary and complex dependencies

Migration Path:
- Keep truck implementation as default
- Add feature flag for OCCT: `cargo build --features occt-step`
- Document trade-offs clearly
*/
```

### 8.3 STEP Integration Strategy

```toml
# mesh-core/Cargo.toml

[dependencies]
# Primary STEP support (Pure Rust)
truck-modeling = { version = "0.4", optional = true }
truck-polymesh = { version = "0.4", optional = true }
truck-stepio = { version = "0.4", optional = true }

# Fallback STEP support (OCCT FFI) - Disabled by default
# opencascade = { version = "0.1", optional = true }

[features]
default = ["step-truck"]
step-truck = ["truck-modeling", "truck-polymesh", "truck-stepio"]
# step-occt = ["opencascade"]  # Fallback option

# Build with STEP support:
# cargo build --features step-truck

# Build without STEP:
# cargo build --no-default-features
```

**Risk Mitigation Notes:**
1. **Primary:** truck (pure Rust, smaller binary, simpler build)
2. **Fallback:** OCCT FFI (if truck insufficient)
3. **Feature flags:** Allow building without STEP if needed
4. **Documentation:** Clear instructions for both approaches
5. **Testing:** Maintain test files for both implementations

---

## 9. CLI INTERFACE DESIGN

### 9.1 img-convert CLI

```rust
// img-convert/src/cli.rs
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "img-convert",
    version,
    about = "Convert images between different formats",
    long_about = "A high-quality image format converter supporting PNG, JPG, BMP, GIF, TIFF, WebP, and more."
)]
pub struct Cli {
    /// Input image file
    #[arg(value_name = "SOURCE")]
    pub input: PathBuf,
    
    /// Output format (extension)
    #[arg(value_name = "FORMAT")]
    pub format: String,
    
    /// Output file path (default: same name with new extension)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    
    /// JPEG/WebP quality (1-100)
    #[arg(short, long, default_value = "95", value_name = "QUALITY")]
    pub quality: u8,
    
    /// PNG compression level (0-9)
    #[arg(short, long, default_value = "6", value_name = "LEVEL")]
    pub compression: u8,
    
    /// DPI for vector rasterization
    #[arg(short, long, default_value = "300", value_name = "DPI")]
    pub dpi: u32,
    
    /// Strip metadata (EXIF, etc.)
    #[arg(short, long)]
    pub strip_metadata: bool,
    
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

// img-convert/src/main.rs
use img_core::{ImageConverter, ImageFormatType, ConversionOptions, QualitySettings};
use anyhow::Result;

mod cli;
use cli::Cli;

fn main() -> Result<()> {
    env_logger::init();
    
    let args = Cli::parse();
    
    // Validate quality
    if args.quality == 0 || args.quality > 100 {
        anyhow::bail!("Quality must be between 1 and 100");
    }
    
    if args.compression > 9 {
        anyhow::bail!("Compression level must be between 0 and 9");
    }
    
    // Parse output format
    let output_format = ImageFormatType::from_extension(&args.format)
        .ok_or_else(|| anyhow::anyhow!("Unsupported format: {}", args.format))?;
    
    // Create converter
    let converter = ImageConverter::new();
    
    // Setup options
    let options = ConversionOptions {
        quality: QualitySettings {
            jpeg_quality: args.quality,
            png_compression: args.compression,
            strip_metadata: args.strip_metadata,
        },
        output_path: args.output,
        dpi: Some(args.dpi),
    };
    
    // Perform conversion
    if args.verbose {
        println!("Converting {} to {}", args.input.display(), args.format);
    }
    
    let output = converter.convert(&args.input, output_format, &options)?;
    
    println!("✓ Conversion successful: {}", output.display());
    
    Ok(())
}
```

### 9.2 mesh-convert CLI

```rust
// mesh-convert/src/cli.rs
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mesh-convert",
    version,
    about = "Convert 3D meshes between different formats",
    long_about = "A 3D mesh format converter supporting STL, OBJ, PLY, glTF, DXF, STEP, and more."
)]
pub struct Cli {
    /// Input mesh file
    #[arg(value_name = "SOURCE")]
    pub input: PathBuf,
    
    /// Output format (extension)
    #[arg(value_name = "FORMAT")]
    pub format: String,
    
    /// Output file path (default: same name with new extension)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    
    /// Format variant (binary/ascii for STL, PLY)
    #[arg(short = 'f', long, value_name = "VARIANT")]
    pub format_variant: Option<String>,
    
    /// Coordinate system transform
    #[arg(short, long, value_name = "SYSTEM")]
    pub transform: Option<String>,
    
    /// Recalculate vertex normals
    #[arg(short = 'n', long)]
    pub recalculate_normals: bool,
    
    /// Validate mesh (manifold, topology)
    #[arg(short = 'v', long)]
    pub validate: bool,
    
    /// Verbose output
    #[arg(short = 'V', long)]
    pub verbose: bool,
}

// mesh-convert/src/main.rs
use mesh_core::{
    MeshConverter, MeshFormatType, ConversionOptions, FormatOptions,
    CoordinateSystem, CoordinateTransform,
};
use anyhow::Result;

mod cli;
use cli::Cli;

fn main() -> Result<()> {
    env_logger::init();
    
    let args = Cli::parse();
    
    // Parse output format
    let output_format = MeshFormatType::from_extension(&args.format)
        .ok_or_else(|| anyhow::anyhow!("Unsupported format: {}", args.format))?;
    
    // Parse format options
    let format_options = parse_format_options(output_format, &args)?;
    
    // Parse coordinate transform
    let transform = args.transform.as_ref().map(|t| parse_transform(t)).transpose()?;
    
    // Create converter
    let converter = MeshConverter::new();
    
    // Setup options
    let options = ConversionOptions {
        output_path: args.output,
        format_options,
        transform,
        recalculate_normals: args.recalculate_normals,
        validate: args.validate,
    };
    
    // Perform conversion
    if args.verbose {
        println!("Converting {} to {}", args.input.display(), args.format);
    }
    
    let output = converter.convert(&args.input, output_format, &options)?;
    
    println!("✓ Conversion successful: {}", output.display());
    
    Ok(())
}

fn parse_format_options(format: MeshFormatType, args: &Cli) -> Result<FormatOptions> {
    match format {
        MeshFormatType::Stl => {
            let binary = match args.format_variant.as_deref() {
                Some("binary") => true,
                Some("ascii") => false,
                None => true, // Default to binary
                Some(other) => anyhow::bail!("Invalid STL variant: {}", other),
            };
            Ok(FormatOptions::Stl { binary })
        }
        MeshFormatType::Ply => {
            let binary = match args.format_variant.as_deref() {
                Some("binary") => true,
                Some("ascii") => false,
                None => true, // Default to binary
                Some(other) => anyhow::bail!("Invalid PLY variant: {}", other),
            };
            Ok(FormatOptions::Ply { binary })
        }
        MeshFormatType::Obj => Ok(FormatOptions::Obj { include_mtl: true }),
        MeshFormatType::Off => Ok(FormatOptions::Off),
        MeshFormatType::Gltf => {
            let binary = args.format == "glb";
            Ok(FormatOptions::Gltf { binary })
        }
        MeshFormatType::Dxf => Ok(FormatOptions::Dxf),
        MeshFormatType::Step => Ok(FormatOptions::Step),
    }
}

fn parse_transform(s: &str) -> Result<CoordinateTransform> {
    match s.to_lowercase().as_str() {
        "y-up" => Ok(CoordinateTransform {
            from: CoordinateSystem::ZUp,
            to: CoordinateSystem::YUp,
        }),
        "z-up" => Ok(CoordinateTransform {
            from: CoordinateSystem::YUp,
            to: CoordinateSystem::ZUp,
        }),
        other => anyhow::bail!("Invalid coordinate system: {}", other),
    }
}
```

---

## 10. BUILD CONFIGURATION

### 10.1 Cross-Compilation Setup

```bash
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-pc-windows-msvc
          override: true
      
      - name: Build Release
        run: |
          cargo build --release --target x86_64-pc-windows-msvc
      
      - name: Package
        run: |
          mkdir release
          cp target/x86_64-pc-windows-msvc/release/img-convert.exe release/
          cp target/x86_64-pc-windows-msvc/release/mesh-convert.exe release/
          cp README.md release/
          cp LICENSE release/
      
      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: windows-x64
          path: release/
```

### 10.2 Build Scripts

```bash
#!/bin/bash
# build.sh - Local build script

set -e

echo "Building converter workspace..."

# Build for Windows (cross-compile from Linux)
if [[ "$1" == "windows" ]]; then
    echo "Cross-compiling for Windows..."
    cargo build --release --target x86_64-pc-windows-gnu
    
    echo "Binaries created:"
    ls -lh target/x86_64-pc-windows-gnu/release/*.exe
fi

# Native build
if [[ "$1" == "native" ]] || [[ -z "$1" ]]; then
    echo "Building for native platform..."
    cargo build --release
    
    echo "Binaries created:"
    ls -lh target/release/img-convert*
    ls -lh target/release/mesh-convert*
fi

# With STEP support
if [[ "$1" == "step" ]]; then
    echo "Building with STEP support..."
    cargo build --release --features step-truck
fi

# Size-optimized build
if [[ "$1" == "small" ]]; then
    echo "Building size-optimized..."
    RUSTFLAGS="-C link-arg=-s" cargo build --release
    strip target/release/img-convert* 2>/dev/null || true
    strip target/release/mesh-convert* 2>/dev/null || true
fi

echo "Build complete!"
```

---

## 11. TESTING STRATEGY

### 11.1 Test Organization

```
tests/
├── unit/              # Unit tests (in each module)
├── integration/       # Integration tests
│   ├── image_conversion_tests.rs
│   ├── mesh_conversion_tests.rs
│   └── cli_tests.rs
├── test_data/
│   ├── images/
│   │   ├── sample.png
│   │   ├── transparent.png
│   │   ├── gradient.jpg
│   │   └── vector.svg
│   └── meshes/
│       ├── cube.stl
│       ├── sphere.obj
│       ├── bunny.ply
│       └── teapot.step
└── benchmarks/
    ├── image_bench.rs
    └── mesh_bench.rs
```

### 11.2 Integration Test Examples

```rust
// tests/integration/image_conversion_tests.rs
use img_core::{ImageConverter, ImageFormatType, ConversionOptions, QualitySettings};
use std::path::PathBuf;

#[test]
fn test_png_to_jpg() {
    let converter = ImageConverter::new();
    let input = PathBuf::from("tests/test_data/images/sample.png");
    
    let options = ConversionOptions {
        quality: QualitySettings::default(),
        output_path: Some(PathBuf::from("/tmp/output.jpg")),
        dpi: None,
    };
    
    let result = converter.convert(&input, ImageFormatType::Jpeg, &options);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.exists());
}

#[test]
fn test_transparent_png_to_jpg() {
    // Should handle transparency gracefully
    let converter = ImageConverter::new();
    let input = PathBuf::from("tests/test_data/images/transparent.png");
    
    let options = ConversionOptions::default();
    let result = converter.convert(&input, ImageFormatType::Jpeg, &options);
    
    assert!(result.is_ok());
}

#[test]
fn test_invalid_quality() {
    let converter = ImageConverter::new();
    let input = PathBuf::from("tests/test_data/images/sample.png");
    
    let mut options = ConversionOptions::default();
    options.quality.jpeg_quality = 101; // Invalid
    
    let result = converter.convert(&input, ImageFormatType::Jpeg, &options);
    assert!(result.is_err());
}

// tests/integration/mesh_conversion_tests.rs
use mesh_core::{MeshConverter, MeshFormatType, ConversionOptions};
use std::path::PathBuf;

#[test]
fn test_stl_to_obj() {
    let converter = MeshConverter::new();
    let input = PathBuf::from("tests/test_data/meshes/cube.stl");
    
    let options = ConversionOptions::default();
    let result = converter.convert(&input, MeshFormatType::Obj, &options);
    
    assert!(result.is_ok());
}

#[test]
fn test_mesh_validation() {
    let converter = MeshConverter::new();
    let input = PathBuf::from("tests/test_data/meshes/cube.stl");
    
    let mut options = ConversionOptions::default();
    options.validate = true;
    
    let result = converter.convert(&input, MeshFormatType::Obj, &options);
    assert!(result.is_ok());
}
```

### 11.3 Benchmark Examples

```rust
// benches/image_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use img_core::{ImageConverter, ImageFormatType, ConversionOptions};
use std::path::PathBuf;

fn benchmark_png_to_jpg(c: &mut Criterion) {
    let converter = ImageConverter::new();
    let input = PathBuf::from("tests/test_data/images/sample.png");
    let options = ConversionOptions::default();
    
    c.bench_function("png_to_jpg", |b| {
        b.iter(|| {
            converter.convert(
                black_box(&input),
                black_box(ImageFormatType::Jpeg),
                black_box(&options),
            )
        });
    });
}

criterion_group!(benches, benchmark_png_to_jpg);
criterion_main!(benches);
```

---

## 12. SECURITY ARCHITECTURE

**Added:** December 26, 2025 (Post Security Review)

This section defines the security architecture for handling untrusted input safely.

### 12.1 Threat Model

**Core Principle:** All file inputs are untrusted. Every buffer is a potential overflow.

**Attack Surface:**
- Image files (PNG, JPEG, BMP, GIF, etc.)
- Mesh files (STL, OBJ, PLY, etc.)
- CLI-provided file paths
- File-declared sizes and dimensions

**Attack Vectors:**
1. Memory exhaustion (large files, large dimensions)
2. Integer overflow (dimension calculations)
3. Path traversal (malicious paths)
4. Panic-based DoS (malformed input)
5. Information disclosure (error messages)

### 12.2 Resource Limits

A centralized resource limits system prevents denial-of-service attacks.

```rust
// common/src/limits.rs
/// Centralized resource limits for the converter
/// 
/// All limits are configurable at runtime via CLI flags or programmatically.
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum file size in bytes (default: 100MB)
    pub max_file_size: usize,
    
    /// Maximum image dimension in pixels (default: 65535)
    pub max_image_dimension: u32,
    
    /// Maximum number of mesh vertices (default: 10 million)
    pub max_vertices: usize,
    
    /// Maximum number of mesh faces (default: 10 million)
    pub max_faces: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024,      // 100MB
            max_image_dimension: 65535,             // Standard maximum
            max_vertices: 10_000_000,               // 10 million
            max_faces: 10_000_000,                  // 10 million
        }
    }
}

impl ResourceLimits {
    /// Create limits with custom values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create permissive limits (for trusted input only)
    pub fn permissive() -> Self {
        Self {
            max_file_size: 1024 * 1024 * 1024,     // 1GB
            max_image_dimension: 131072,            // 128K
            max_vertices: 100_000_000,              // 100 million
            max_faces: 100_000_000,                 // 100 million
        }
    }
    
    /// Validate file size against limit
    pub fn check_file_size(&self, size: usize) -> Result<()> {
        if size > self.max_file_size {
            return Err(ConversionError::InvalidInput(format!(
                "File too large: {} bytes (max: {} bytes)",
                size, self.max_file_size
            )));
        }
        Ok(())
    }
    
    /// Validate image dimensions against limit
    pub fn check_image_dimensions(&self, width: u32, height: u32) -> Result<()> {
        if width > self.max_image_dimension || height > self.max_image_dimension {
            return Err(ConversionError::InvalidInput(format!(
                "Image dimensions too large: {}x{} (max: {}x{})",
                width, height, self.max_image_dimension, self.max_image_dimension
            )));
        }
        Ok(())
    }
    
    /// Validate mesh resources against limits
    pub fn check_mesh_resources(&self, vertices: usize, faces: usize) -> Result<()> {
        if vertices > self.max_vertices {
            return Err(ConversionError::InvalidInput(format!(
                "Too many vertices: {} (max: {})",
                vertices, self.max_vertices
            )));
        }
        if faces > self.max_faces {
            return Err(ConversionError::InvalidInput(format!(
                "Too many faces: {} (max: {})",
                faces, self.max_faces
            )));
        }
        Ok(())
    }
}
```

### 12.3 Validation Architecture

Validation occurs at multiple layers:

**Layer 1: File Level (I/O)**
```
File → [Size Check] → [Read] → Bytes
```

**Layer 2: Format Level (Parser)**
```
Bytes → [Magic Bytes] → [Dimension Check] → [Parse] → Data
```

**Layer 3: Data Level (Validation)**
```
Data → [Integrity Check] → [Resource Check] → Validated Data
```

```rust
// Enhanced I/O with size validation
// common/src/io.rs

use crate::limits::ResourceLimits;

/// Read file with size validation
pub fn read_file_bytes_checked(
    path: &Path, 
    limits: &ResourceLimits
) -> Result<Vec<u8>> {
    // Check file size before reading
    let metadata = fs::metadata(path)?;
    let size = metadata.len() as usize;
    limits.check_file_size(size)?;
    
    fs::read(path).map_err(ConversionError::Io)
}
```

### 12.4 Format Detection Security

Two-stage format detection prevents format spoofing:

```rust
// img-core/src/formats/registry.rs

/// Magic bytes for format detection
const PNG_MAGIC: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const JPEG_MAGIC: [u8; 3] = [0xFF, 0xD8, 0xFF];
const BMP_MAGIC: [u8; 2] = [0x42, 0x4D];
const GIF_MAGIC: [u8; 4] = [0x47, 0x49, 0x46, 0x38];

impl FormatRegistry {
    /// Detect format from magic bytes
    pub fn detect_from_bytes(data: &[u8]) -> Option<ImageFormat> {
        if data.len() < 8 { return None; }
        
        if data.starts_with(&PNG_MAGIC) {
            Some(ImageFormat::Png)
        } else if data.starts_with(&JPEG_MAGIC) {
            Some(ImageFormat::Jpeg)
        } else if data.starts_with(&BMP_MAGIC) {
            Some(ImageFormat::Bmp)
        } else if data.starts_with(&GIF_MAGIC) {
            Some(ImageFormat::Gif)
        } else {
            None
        }
    }
    
    /// Verify format matches expected (two-stage detection)
    pub fn verify_format(data: &[u8], expected: ImageFormat) -> Result<()> {
        if let Some(detected) = Self::detect_from_bytes(data) {
            if detected != expected {
                return Err(ConversionError::InvalidFormat(format!(
                    "File extension suggests {:?} but content is {:?}",
                    expected, detected
                )));
            }
        }
        Ok(())
    }
}
```

### 12.5 Error Message Sanitization

Error messages must not leak sensitive information:

```rust
// common/src/error.rs

impl ConversionError {
    /// Get user-safe error message (sanitized)
    pub fn user_message(&self) -> String {
        match self {
            ConversionError::Io(_) => 
                "File operation failed".to_string(),
            ConversionError::InvalidInput(msg) => 
                sanitize_error_message(msg),
            ConversionError::ConversionFailed(msg) => 
                sanitize_error_message(msg),
            // ... other variants
        }
    }
}

fn sanitize_error_message(msg: &str) -> String {
    // Remove full paths, keep only filename
    // Remove internal details
    // Limit message length
    msg.chars().take(200).collect()
}
```

### 12.6 Integer Overflow Protection

All dimension calculations use checked arithmetic:

```rust
// Pattern for safe dimension calculation
let total_size = width
    .checked_mul(height)
    .and_then(|v| v.checked_mul(channels))
    .ok_or_else(|| ConversionError::InvalidInput(
        "Image dimensions cause integer overflow".to_string()
    ))?;
```

### 12.7 CLI Security

CLI validates all inputs before processing:

```rust
// CLI argument validation
impl Cli {
    pub fn validate(&self, limits: &ResourceLimits) -> Result<()> {
        // Validate input path exists
        common::validation::validate_file_path(&self.input)?;
        
        // Validate quality range
        if self.quality == 0 || self.quality > 100 {
            return Err(ConversionError::InvalidInput(
                "Quality must be between 1 and 100".to_string()
            ));
        }
        
        // Validate output path (if specified)
        if let Some(ref output) = self.output {
            common::validation::validate_output_path(output)?;
        }
        
        Ok(())
    }
}
```

### 12.8 Dependency Security

**Required CI/CD checks:**
```yaml
# .github/workflows/security.yml
name: Security Audit

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * 0'  # Weekly

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install cargo-audit
        run: cargo install cargo-audit
        
      - name: Security Audit
        run: cargo audit
        
      - name: Check for unsafe code
        run: |
          cargo install cargo-geiger
          cargo geiger --update-readme --output-format GitHubMarkdown
```

### 12.9 Security Testing

**Required test cases:**
- Malformed file headers
- Files with extreme dimensions
- Files with mismatched extension/content
- Path traversal attempts
- Integer overflow conditions

```rust
#[test]
fn test_reject_oversized_dimensions() {
    let limits = ResourceLimits::default();
    let result = limits.check_image_dimensions(100_000, 100_000);
    assert!(result.is_err());
}

#[test]
fn test_reject_oversized_file() {
    let limits = ResourceLimits::default();
    let result = limits.check_file_size(200 * 1024 * 1024);  // 200MB
    assert!(result.is_err());
}
```

### 12.10 Security Checklist

For every PR:
- [ ] No unsafe code (or documented justification)
- [ ] All external input validated
- [ ] Error messages sanitized
- [ ] Resource limits enforced
- [ ] Integer overflow protection used
- [ ] No panics on bad input
- [ ] Dependency audit passed

---

## SUMMARY

This architecture provides:

✓ **Clean separation** - Libraries vs binaries, 2D vs 3D
✓ **Extensibility** - Trait-based format system
✓ **Type safety** - Rust's strong type system
✓ **Error handling** - Comprehensive error types
✓ **Security** - Resource limits, input validation, sanitization
✓ **Testing** - Unit, integration, security, and benchmarks
✓ **STEP support** - Pure Rust (truck) with OCCT fallback
✓ **Build flexibility** - Feature flags, cross-compilation
✓ **CLI usability** - Clear, documented interfaces
✓ **Future GUI** - Architecture ready for GUI integration

**Next Steps:**
1. Review and approve architecture
2. Begin Phase 1 implementation (img-convert)
3. Follow with Phase 1 implementation (mesh-convert)
4. Iterate based on testing

**Ready to proceed with implementation?**
