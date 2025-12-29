# API Documentation
## SimpleImageConverter Libraries

**Last Updated:** December 27, 2025  
**Generate Command:** `cargo doc --workspace --open`

---

## Overview

SimpleImageConverter provides two core libraries:

1. **img-core** - 2D image format conversion
2. **mesh-core** - 3D mesh format conversion

Both libraries share common utilities from the **common** crate.

---

## Quick Start

### Image Conversion

```rust
use img_core::{ImageConverter, FormatRegistry, ImageFormat, QualitySettings};

// Get format handlers
let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;

// Create converter
let converter = ImageConverter::new();
let quality = QualitySettings::new(90);

// Convert
let input_data = std::fs::read("input.png")?;
let output_data = converter.convert(
    &input_data,
    reader.as_ref(),
    writer.as_ref(),
    &quality
)?;

std::fs::write("output.jpg", output_data)?;
```

### Mesh Conversion

```rust
use mesh_core::{MeshConverter, FormatRegistry, MeshFormat};
use common::limits::ResourceLimits;

// Get format handlers with resource limits
let limits = ResourceLimits::default();
let reader = FormatRegistry::get_reader_with_limits(MeshFormat::Stl, limits.clone())?;
let writer = FormatRegistry::get_writer(MeshFormat::Obj)?;

// Create converter
let converter = MeshConverter::new();

// Convert
let input_data = std::fs::read("input.stl")?;
let output_data = converter.convert(
    &input_data,
    reader.as_ref(),
    writer.as_ref()
)?;

std::fs::write("output.obj", output_data)?;
```

---

## Core Types

### Image Formats

```rust
use img_core::formats::registry::ImageFormat;

pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
    Gif,
    Tiff,
    WebP,
    Svg,
}
```

### Mesh Formats

```rust
use mesh_core::formats::registry::MeshFormat;

pub enum MeshFormat {
    Stl,
    Obj,
    Ply,
    Off,
    Gltf,
    Dxf,
    Step,  // Requires --features step
}
```

### Format Capabilities

```rust
use img_core::FormatCapabilities;
use img_core::formats::registry::ImageFormat;

// Check format capabilities
if FormatCapabilities::supports_transparency(ImageFormat::Png) {
    // Handle transparency
}

let info = FormatCapabilities::info(ImageFormat::Gif);
assert!(info.supports_animation);
```

---

## Resource Limits

All file operations respect resource limits for security:

```rust
use common::limits::ResourceLimits;

// Default limits (100MB file, 65535px dimension, 10M vertices/faces)
let limits = ResourceLimits::default();

// Custom limits
let limits = ResourceLimits::builder()
    .max_file_size_mb(50)
    .max_image_dimension(10000)
    .max_vertices(1_000_000)
    .max_faces(2_000_000)
    .build();

// Permissive limits (for trusted input only)
let limits = ResourceLimits::permissive();
```

---

## Progress Reporting

Converters support progress reporting:

```rust
use img_core::ImageConverter;
use common::progress::ProgressReporter;

struct MyProgressReporter;
impl ProgressReporter for MyProgressReporter {
    fn report(&self, progress: f32) {
        println!("Progress: {:.0}%", progress * 100.0);
    }
    fn status(&self, message: &str) {
        println!("Status: {}", message);
    }
}

let converter = ImageConverter::new();
let progress = MyProgressReporter;
converter.convert_with_progress(&input, reader, writer, &quality, &progress)?;
```

---

## Error Handling

All operations return `Result<T, ConversionError>`:

```rust
use common::error::{ConversionError, Result};

match converter.convert(&input, reader, writer, &quality) {
    Ok(output) => println!("Success!"),
    Err(ConversionError::InvalidInput(msg)) => eprintln!("Invalid input: {}", msg),
    Err(ConversionError::ResourceLimitExceeded(msg)) => eprintln!("Limit exceeded: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Security Features

### Input Validation

All format readers validate input size before parsing:

```rust
// Automatic validation in all readers
let image = reader.read(&input_data)?; // Validates size internally
```

### Format Verification

Two-stage format detection prevents format spoofing:

```rust
use img_core::FormatRegistry;
use std::path::Path;

let path = Path::new("photo.png");
let data = std::fs::read(path)?;
// Verifies extension matches magic bytes
let format = FormatRegistry::detect_two_stage(path, &data)?;
```

### Security Logging

Security events are automatically logged:

```rust
use common::security::log_security_error;

// Automatic logging on validation failures
// See common/src/security.rs for details
```

---

## Feature Flags

Optional format support via feature flags:

```toml
# Cargo.toml
[dependencies]
mesh-core = { path = "../mesh-core", features = ["step"] }
```

```bash
# Build with STEP support (optional, feature-gated)
cargo build --features step
```

**Note:** STEP format support is currently partial (read-only, tessellation in progress) and blocked by truck-stepio library limitation (input API not available in v0.3.0).

---

## Testing

### Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run security tests
cargo test --workspace security

# Run integration tests
cargo test --workspace integration
```

### Fuzz Testing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run fuzz tests
cd fuzz
cargo fuzz run fuzz_png_reader
```

---

## Documentation Generation

Generate full API documentation:

```bash
# Generate and open in browser
cargo doc --workspace --open

# Generate without opening
cargo doc --workspace --no-deps

# Generate for specific crate
cargo doc -p img-core --open
```

---

## Examples

See `examples/` directory for complete examples.

---

## References

- **Architecture:** `Phase3_Architecture.md`
- **Format Support:** `docs/FORMATS.md`
- **Security:** `docs/THREAT_MODEL.md`
- **Secure by Design:** `docs/SECURE_BY_DESIGN_GUIDANCE.md`

---

*For complete API reference, run `cargo doc --workspace --open`*

