# Rust Resources & Lessons Learned
## Living Knowledge Base for Simple Image Converter

**Maintained By:** Researcher (Dr. Taylor Kim)  
**Last Updated:** December 26, 2025  
**Update Frequency:** Weekly + as needed  
**Purpose:** Track Rust ecosystem changes, library updates, and project learnings

**⚠️ IMPORTANT:** All team members must consult this document before implementing features or making decisions.

---

## 📅 Update Log

| Date | Category | Summary | Updated By |
|------|----------|---------|------------|
| 2025-12-26 | Initial | Document created | Researcher |
| TBD | Rust | (Future updates) | Team |

---

## Table of Contents

1. [Rust Language Updates](#rust-language-updates)
2. [Core Dependencies](#core-dependencies)
3. [Best Practices](#best-practices)
4. [Known Issues & Gotchas](#known-issues--gotchas)
5. [Lessons Learned](#lessons-learned)
6. [Performance Tips](#performance-tips)
7. [Security Considerations](#security-considerations)
8. [Breaking Changes](#breaking-changes)

---

## Rust Language Updates

### Current Rust Version
**Project MSRV:** 1.70.0  
**Latest Stable:** 1.92.0 (as of knowledge cutoff)  
**Latest Edition:** 2021

### Relevant Features Since Our MSRV (1.70+)

#### Rust 1.70 (Our MSRV)
- `OnceCell` and `OnceLock` stabilized in std
- Sparse registry protocol default
- Improved error messages

#### Rust 1.75
- `async fn` and return position `impl Trait` in traits
- Pointer byte offset APIs

#### Rust 1.80
- `LazyCell` and `LazyLock` stabilized
- Exclusive range patterns `a..b`

#### Rust 1.85
- `unsafe extern` blocks stabilized
- `gen` blocks for iterators

**Action Items:**
- [ ] Consider updating MSRV to 1.75 for async trait support (if needed)
- [ ] Use `LazyCell` for format registry initialization (optimization)

### Rust 2024 Edition (Future)
**Status:** Not yet released  
**Potential Impact:** Monitor for breaking changes  
**Action:** Wait for announcement, then evaluate migration

---

## Core Dependencies

### Image Processing (img-core)

#### image (v0.25)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable  
**Last Checked:** 2025-12-26

**Key APIs:**
```rust
use image::{DynamicImage, ImageFormat, ImageBuffer};

// Load image
let img = image::open("path.png")?;

// Save with format
img.save_with_format("out.jpg", ImageFormat::Jpeg)?;

// Access pixels
let rgb_img = img.to_rgb8();
```

**Changes Since Training Cutoff:**
- (No updates yet - will track)

**Gotchas:**
- RGBA → JPG requires manual conversion
- Some formats support multiple color types
- Memory usage scales with image dimensions

**Best Practice:**
```rust
// Convert RGBA to RGB for JPEG
if image.color() == ColorType::Rgba8 {
    let rgb = image.to_rgb8();
    // Save as JPEG
}
```

#### resvg (v0.44) - SVG Rasterization
**License:** MPL-2.0  
**Status:** ✅ Active development

**Key APIs:**
```rust
use resvg::usvg::{Tree, Options};
use resvg::tiny_skia::Pixmap;

// Parse SVG
let tree = Tree::from_data(svg_data, &Options::default())?;

// Render to pixmap
let pixmap = Pixmap::new(width, height)?;
resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
```

**Gotchas:**
- Requires font database for text rendering
- DPI affects output size calculations

#### ravif (v0.11) - AVIF Encoding
**License:** BSD-3-Clause  
**Status:** ✅ Stable

**Performance:** Slower than JPEG but better compression

#### exr (v1.72) - OpenEXR
**License:** BSD-3-Clause  
**Status:** ✅ Stable

**Gotchas:**
- HDR data requires special handling
- Large file sizes for high-resolution

### 3D Mesh Processing (mesh-core)

#### stl_io (v0.7)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable

**Key APIs:**
```rust
use stl_io::{read_stl, write_stl, IndexedMesh};

// Read STL (auto-detects binary/ASCII)
let mesh = read_stl(&mut file)?;

// Write binary STL
write_stl(&mut file, mesh.faces.iter())?;
```

**Gotchas:**
- Auto-detection can fail on malformed files
- Binary format is more efficient

**Best Practice:**
```rust
// Always specify binary for output
let binary_format = true;
stl_io::write_stl_binary(&mut file, &triangles)?;
```

#### tobj (v4.0) - OBJ Format
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable

**Key APIs:**
```rust
use tobj::{load_obj, GPU_LOAD_OPTIONS};

let (models, materials) = load_obj("model.obj", &GPU_LOAD_OPTIONS)?;
```

**Gotchas:**
- Material files (.mtl) are optional
- Multiple objects per file possible
- Texture coordinates may be missing

#### ply-rs (v0.1)
**License:** MIT  
**Status:** ⚠️ Low maintenance

**Gotchas:**
- API is older, less ergonomic
- Binary/ASCII detection manual
- Consider alternatives if issues arise

#### gltf (v1.4)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Active

**Key APIs:**
```rust
use gltf::Gltf;

let gltf = Gltf::open("model.gltf")?;
for mesh in gltf.meshes() {
    // Process primitives
}
```

**Gotchas:**
- Separate buffer data handling
- glTF is scene-oriented, not just mesh
- Animation data ignored in conversion

#### truck (v0.4) - STEP Support
**License:** MIT OR Apache-2.0  
**Status:** ✅ Active development

**Components:**
- truck-modeling: CAD kernel
- truck-polymesh: Mesh operations
- truck-stepio: STEP I/O

**Key APIs:**
```rust
use truck_stepio::read;
use truck_modeling::Shell;

let shells = read(step_string)?;
```

**Gotchas:**
- STEP is complex, not all features supported
- Tessellation quality affects output
- May not handle all STEP AP variants

**Status:** To be tested in Sprint 7

### Utilities

#### nalgebra (v0.33)
**License:** Apache-2.0  
**Status:** ✅ Stable

**Use Cases:**
- Coordinate transforms
- Normal calculations
- Matrix operations

**Key APIs:**
```rust
use nalgebra::{Vector3, Matrix4, Point3};

// Cross product for normals
let normal = v1.cross(&v2).normalize();

// Transform matrix
let transform = Matrix4::new_rotation(angle);
```

**Best Practice:**
```rust
// Use Point3 for positions, Vector3 for directions
let pos = Point3::new(x, y, z);
let dir = Vector3::new(dx, dy, dz);
```

#### anyhow (v1.0) & thiserror (v1.0)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable

**Usage Pattern:**
```rust
// In libraries: use thiserror
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// In binaries: use anyhow
fn main() -> anyhow::Result<()> {
    // Easy error handling
}
```

#### clap (v4.5)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable

**Derive API:**
```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,
}
```

**Best Practice:** Use derive API for maintainability

---

## Best Practices

### Error Handling

**Library Code (img-core, mesh-core):**
```rust
// Use thiserror for custom errors
#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ConversionError>;
```

**Binary Code (img-convert, mesh-convert):**
```rust
// Use anyhow for easy error handling
fn main() -> anyhow::Result<()> {
    let result = convert_file()?;
    Ok(())
}
```

**Context Wrapping:**
```rust
use anyhow::Context;

std::fs::read("file.txt")
    .context("Failed to read configuration file")?;
```

### File I/O Patterns

**Reading:**
```rust
use std::fs::File;
use std::io::BufReader;

// Buffered reading for large files
let file = File::open(path)?;
let reader = BufReader::new(file);
```

**Writing:**
```rust
use std::fs::File;
use std::io::BufWriter;

// Buffered writing
let file = File::create(path)?;
let writer = BufWriter::new(file);
```

### Memory Management

**Large Files:**
```rust
// Don't load entire file into memory
// Process in chunks where possible

// Good: Streaming
for chunk in reader.chunks(8192) {
    process(chunk)?;
}

// Bad: Loading everything
let data = std::fs::read(path)?; // Entire file in RAM
```

**Image Buffers:**
```rust
// Drop intermediate buffers explicitly
{
    let temp_buffer = image.to_rgb8();
    process(&temp_buffer)?;
} // temp_buffer dropped here
```

### Testing Patterns

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversion() {
        let input = create_test_data();
        let result = convert(input);
        assert!(result.is_ok());
    }
}
```

**Integration Tests:**
```rust
// tests/integration.rs
use my_crate::*;

#[test]
fn test_real_file_conversion() {
    let input = "tests/test_data/sample.png";
    let result = convert_file(input, "jpg");
    assert!(result.is_ok());
}
```

**Test Data Location:**
```
tests/
├── test_data/
│   ├── images/
│   │   ├── sample.png
│   │   └── transparent.png
│   └── meshes/
│       ├── cube.stl
│       └── sphere.obj
└── integration.rs
```

### Documentation

**Module Level:**
```rust
//! This module handles PNG format conversion.
//!
//! # Examples
//!
//! ```
//! use img_core::formats::png::PngFormat;
//! let format = PngFormat::new();
//! ```
```

**Function Level:**
```rust
/// Converts an image to the specified format.
///
/// # Arguments
///
/// * `input` - Source image path
/// * `format` - Target format
///
/// # Errors
///
/// Returns `ConversionError` if:
/// - File doesn't exist
/// - Format not supported
/// - Conversion fails
///
/// # Examples
///
/// ```
/// let result = convert_image("in.png", "jpg")?;
/// ```
pub fn convert_image(input: &Path, format: &str) -> Result<PathBuf> {
    // Implementation
}
```

### Cargo Features

**Define Features:**
```toml
[features]
default = ["step-truck"]
step-truck = ["truck-modeling", "truck-polymesh", "truck-stepio"]
step-occt = ["opencascade"]  # Fallback
all = ["step-truck"]
```

**Use in Code:**
```rust
#[cfg(feature = "step-truck")]
use truck_stepio::*;

#[cfg(feature = "step-occt")]
use opencascade::*;
```

---

## Known Issues & Gotchas

### Image Processing

#### PNG Transparency
**Issue:** Direct PNG RGBA → JPEG fails (JPEG doesn't support transparency)

**Solution:**
```rust
if img.color() == ColorType::Rgba8 {
    // Convert to RGB with white background
    let rgb = DynamicImage::ImageRgba8(img.to_rgba8())
        .into_rgb8();
    // Now save as JPEG
}
```

#### GIF Animation
**Issue:** image crate loads only first frame

**Solution:** Document limitation or use specialized GIF crate

#### SVG Text Rendering
**Issue:** resvg needs font database

**Solution:**
```rust
use resvg::usvg::fontdb::Database;

let mut fontdb = Database::new();
fontdb.load_system_fonts();
```

### 3D Mesh Processing

#### STL Normal Calculation
**Issue:** STL normals may be incorrect or missing

**Solution:**
```rust
// Always recalculate normals from geometry
fn calculate_normal(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Normal {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    edge1.cross(&edge2).normalize()
}
```

#### OBJ Vertex Indices
**Issue:** OBJ uses 1-based indexing, Rust uses 0-based

**Solution:** tobj crate handles this, but be aware when writing custom parsers

#### Coordinate Systems
**Issue:** Different formats use Y-up vs Z-up

**Solution:**
```rust
// Provide transform option
if options.transform == CoordinateSystem::ZUp {
    // Swap Y and Z coordinates
    let (x, y, z) = (vertex.x, vertex.z, -vertex.y);
}
```

### Platform-Specific

#### Windows Path Handling
**Issue:** Windows uses backslashes

**Solution:** Always use `Path` and `PathBuf`, never string manipulation
```rust
use std::path::Path;

// Good
let path = Path::new("dir").join("file.txt");

// Bad
let path = "dir\\file.txt";  // Breaks on Unix
```

#### Line Endings
**Issue:** Windows CRLF vs Unix LF

**Solution:** Use `std::io::BufReader` which handles both

---

## Lessons Learned

### Sprint 1 Learnings
(To be filled during Sprint 1)

**Template for entries:**
```markdown
#### [Date] [Topic] - [Person]
**Problem:** Description of issue
**Solution:** How it was solved
**Prevention:** How to avoid in future
**References:** Links or docs
```

### General Development

#### Cargo Workspace Tips
- Run `cargo check --workspace` frequently
- Use `--no-default-features` to test minimal builds
- Keep workspace dependencies in sync

#### Testing Tips
- Run tests before pushing: `cargo test --workspace`
- Test on Windows if targeting Windows
- Include edge cases in tests

#### Documentation Tips
- Write docs as you code, not after
- Include examples in doc comments
- Test examples with `cargo test --doc`

---

## Performance Tips

### Image Processing

#### Avoid Unnecessary Copies
```rust
// Good: Reference
fn process(img: &DynamicImage) { }

// Bad: Clone
fn process(img: DynamicImage) { }  // Copies entire image
```

#### Use Parallel Processing (Future)
```rust
// Consider rayon for batch processing
use rayon::prelude::*;

files.par_iter().for_each(|file| {
    convert_file(file).ok();
});
```

### 3D Mesh Processing

#### Preallocate Vectors
```rust
// Good: Preallocate if size known
let mut vertices = Vec::with_capacity(expected_count);

// Bad: Frequent reallocations
let mut vertices = Vec::new();
```

#### Avoid Redundant Calculations
```rust
// Cache normal calculations
let normal_cache: HashMap<FaceId, Normal> = HashMap::new();
```

### General

#### Profile Before Optimizing
```bash
# Use cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bin img-convert -- input.png jpg
```

#### Release Builds for Performance Testing
```bash
# Always test performance in release mode
cargo build --release
cargo test --release
```

---

## Security Considerations

### Input Validation

**All format parsers must validate:**
```rust
// Check file size
if file_size > MAX_SIZE {
    return Err(Error::FileTooLarge);
}

// Validate magic bytes
if !header.starts_with(MAGIC) {
    return Err(Error::InvalidFormat);
}

// Check dimensions
if width > MAX_DIMENSION || height > MAX_DIMENSION {
    return Err(Error::DimensionsTooLarge);
}
```

### Unsafe Code

**Policy:** Avoid unless absolutely necessary

**If needed:**
```rust
// SAFETY: Detailed explanation of why this is safe
unsafe {
    // Minimal unsafe block
}
```

### Dependencies

**Monitor:** Run `cargo audit` regularly
```bash
cargo install cargo-audit
cargo audit
```

---

## Breaking Changes

### Planning for Future Breaks

**Semantic Versioning:**
- Patch (0.1.x): Bug fixes only
- Minor (0.x.0): New features, backward compatible
- Major (x.0.0): Breaking changes

**Deprecation Process:**
```rust
#[deprecated(since = "0.2.0", note = "Use new_function instead")]
pub fn old_function() { }
```

### Monitoring Dependency Changes

**Check before updating:**
```bash
# See what would update
cargo update --dry-run

# Check for breaking changes
cargo tree
```

---

## Resources & References

### Official Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Ecosystem Resources
- [This Week in Rust](https://this-week-in-rust.org/)
- [Rust Blog](https://blog.rust-lang.org/)
- [crates.io](https://crates.io/)
- [docs.rs](https://docs.rs/)

### Security
- [RustSec Advisory Database](https://rustsec.org/)
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)

### Tools
- [cargo-edit](https://github.com/killercup/cargo-edit) - Manage dependencies
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated) - Check for updates
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) - Lint dependencies

---

## Update Process

### Weekly Update (Researcher)

**Checklist:**
- [ ] Check Rust blog for announcements
- [ ] Read This Week in Rust
- [ ] Review dependency changelogs
- [ ] Check RustSec advisories
- [ ] Update this document
- [ ] Notify team of important changes

### Ad-hoc Updates (All Team)

**When to update:**
- Discover a gotcha or best practice
- Solve a tricky problem
- Learn something valuable
- Find a breaking change

**How to update:**
1. Add entry to appropriate section
2. Include date and your name
3. Provide clear explanation
4. Link to references if available
5. Commit with message: `docs: Update rust-resources.md - [topic]`

---

## Template for New Entries

### Dependency Update Template
```markdown
#### [Crate Name] (v[Version])
**License:** [License]  
**Status:** ✅/⚠️/❌ [Status]  
**Updated:** [Date]

**Changes:**
- Change 1
- Change 2

**Impact:** Description

**Action Required:** What team needs to do
```

### Lesson Learned Template
```markdown
#### [Date] [Topic] - [Person]
**Problem:** What went wrong or what was learned

**Solution:** How it was resolved

**Prevention:** How to avoid in future

**Code Example:**
\`\`\`rust
// Example
\`\`\`
```

---

**Status:** 🟢 Active Document  
**Review Frequency:** Weekly  
**Next Review:** TBD (first week of Sprint 1)

---

## Quick Links

**Most Important Sections:**
1. [Core Dependencies](#core-dependencies) - Library APIs
2. [Best Practices](#best-practices) - How to code
3. [Known Issues](#known-issues--gotchas) - Avoid pitfalls
4. [Lessons Learned](#lessons-learned) - Team knowledge

**For Quick Lookup:**
- Error handling: [Best Practices > Error Handling](#error-handling)
- Image APIs: [Core Dependencies > Image Processing](#image-processing-img-core)
- 3D Mesh APIs: [Core Dependencies > 3D Mesh Processing](#3d-mesh-processing-mesh-core)
- Security: [Security Considerations](#security-considerations)

---

_This is a living document. Keep it updated!_  
_Every team member should check this before implementing features._
