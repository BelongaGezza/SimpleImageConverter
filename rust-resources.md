# Rust Resources & Lessons Learned
## Living Knowledge Base for Simple Image Converter

**Maintained By:** Researcher (Dr. Taylor Kim)  
**Last Updated:** December 2025  
**Update Frequency:** Weekly + as needed  
**Purpose:** Track Rust ecosystem changes, library updates, and project learnings

**⚠️ IMPORTANT:** All team members must consult this document before implementing features or making decisions.

### Quick Status Summary (as of Dec 27, 2025)

**Current Project Phase:** Sprint 6 complete (Polish & Testing)

**Technology Audit Completed:** December 27, 2025
- See `TECHNOLOGY_AUDIT_REPORT.md` for full details

**Active Dependencies:**
- ✅ `anyhow` v1.0 - Error handling (latest: 1.0.100)
- ✅ `thiserror` v1.0 - Error types (⚠️ v2.0.17 available - defer upgrade)
- ✅ `clap` v4.5 - CLI (latest: 4.5.53)
- ✅ `serde` v1.0 - Serialization (latest: 1.0.217+)
- ✅ `serde_json` v1.0 - JSON support
- ✅ `log` v0.4 - Logging
- ✅ `image` v0.25 - Image processing (latest: 0.25.8)
- ⚠️ `resvg` v0.40 - SVG rendering (latest: **0.45.1** - update recommended)
- ✅ `tiny-skia` v0.11 - 2D rendering (latest: 0.11.4)

**GUI Dependencies (Sprint 7):**
- 📋 `egui` v0.27 - GUI framework (latest: **0.33.3** - stick with 0.27 for Sprint 7)
- 📋 `eframe` v0.27 - Application framework (latest: **0.33.3** - stick with 0.27 for Sprint 7)
- 📋 `rfd` v0.14 - File dialogs (latest: **0.16.0** - stick with 0.14 for Sprint 7)

**3D/Mesh Dependencies:**
- ❌ `stl_io` v0.7 - STL format (latest: **0.10.0** - update required)
- ✅ `nalgebra` v0.33 - Linear algebra
- ✅ `tobj` v4.0 - OBJ format (latest: 4.0.3)
- ✅ `ply-rs-bw` v0.1.3 - PLY format (patched fork, CVE-2020-25573 fixed)
- ✅ `gltf` v1.4 - glTF format (latest: 1.4.1)
- ✅ `dxf` v0.6 - DXF format (current)
- ✅ `ahash` v0.8 - HashMap (latest: 0.8.12)

**Optional CAD Dependencies:**
- ✅ `ruststep` v0.4.0 - STEP file parsing (feature-gated, ap203 feature)
- ✅ `truck-*` v0.3.0-0.4.0 - STEP geometry processing (feature-gated)

---

## 📅 Update Log

| Date | Category | Summary | Updated By |
|------|----------|---------|------------|
| 2025-12-26 | Initial | Document created | Researcher |
| 2025-12-27 | Project Status | Documented current vs planned dependencies, project early phase status | Researcher |
| 2025-12-27 | **AUDIT** | Comprehensive technology audit completed - see TECHNOLOGY_AUDIT_REPORT.md | Researcher |
| 2025-12-27 | **SECURITY** | **FIXED** CVE-2020-25573 - Replaced ply-rs with ply-rs-bw | Researcher |
| 2025-12-27 | Updates | stl_io 0.7→0.10, resvg 0.40→0.45, thiserror 2.0 available | Researcher |
| 2025-12-29 | ruststep | Added comprehensive ruststep guidance (docs/RUSTSTEP_GUIDANCE.md) | System Architect |
| 2025-12 | **GUI** | Added egui/eframe/rfd framework section for Sprint 7 GUI implementation | Researcher |

---

## Table of Contents

1. [Rust Language Updates](#rust-language-updates)
2. [Core Dependencies](#core-dependencies)
3. [GUI Framework: egui/eframe](#gui-framework-egui-eframe)
4. [Best Practices](#best-practices)
5. [Known Issues & Gotchas](#known-issues--gotchas)
6. [Lessons Learned](#lessons-learned)
7. [Performance Tips](#performance-tips)
8. [Security Considerations](#security-considerations)
9. [Breaking Changes](#breaking-changes)

---

## Rust Language Updates

### Current Rust Version
**Project MSRV:** 1.92.0 (confirmed in workspace Cargo.toml)  
**Latest Stable:** Check `rustc --version` locally or [rust-lang.org](https://www.rust-lang.org/)  
**Latest Edition:** 2021  
**Workspace Edition:** 2021 (confirmed)

### Relevant Features Since Our MSRV (1.92+)

#### Rust 1.92 (Our MSRV)
- Latest stable features and improvements
- Enhanced compiler diagnostics
- Improved dependency resolution

#### Previously Available Features (1.70-1.91)
- `OnceCell` and `OnceLock` stabilized in std
- Sparse registry protocol default
- `async fn` and return position `impl Trait` in traits
- Pointer byte offset APIs
- `LazyCell` and `LazyLock` stabilized
- Exclusive range patterns `a..b`
- `unsafe extern` blocks stabilized
- `gen` blocks for iterators

**Action Items:**
- [x] Updated MSRV to 1.92 for latest features and improvements
- [ ] Use `LazyCell` for format registry initialization (optimization)

### Rust 2024 Edition (Future)
**Status:** Not yet released  
**Potential Impact:** Monitor for breaking changes  
**Action:** Wait for announcement, then evaluate migration

---

## Core Dependencies

**Project Status Note:** As of December 27, 2025, the project is in early development phase (Sprint 1-2 complete). Currently, only minimal core dependencies are in use. The sections below document both:
- ✅ **Currently in use** - Dependencies actively in Cargo.toml
- 📋 **Planned** - Dependencies documented for future implementation phases

### Image Processing (img-core)

#### image (v0.25)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable (Currently in use)  
**Last Checked:** 2025-12-27  
**Current Usage:** Active in workspace.dependencies

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
- (No breaking changes identified as of 2025-12-27 - will monitor for updates)

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

#### resvg (v0.40) - SVG Rasterization - ⚠️ UPDATE AVAILABLE
**License:** MPL-2.0
**Status:** ⚠️ **5 versions behind** - Latest is 0.45.1
**Current Usage:** Active in img-core

**Update Recommended:**
```toml
# In workspace Cargo.toml
# Update: resvg = "0.40"
# To:
resvg = "0.45"
```

**Note:** Test SVG rendering thoroughly after upgrade - 5 minor versions may include API changes.

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
**Status:** ✅ Stable (Planned for future phase)  
**Current Usage:** Not yet added to workspace

**Performance:** Slower than JPEG but better compression

#### exr (v1.72) - OpenEXR
**License:** BSD-3-Clause  
**Status:** ✅ Stable (Planned for future phase)  
**Current Usage:** Not yet added to workspace

**Gotchas:**
- HDR data requires special handling
- Large file sizes for high-resolution

### 3D Mesh Processing (mesh-core)

**Current Status:** mesh-core is in early development. Mesh format dependencies will be added during Sprint 3+.

#### stl_io (v0.7) - ⚠️ OUTDATED
**License:** MIT OR Apache-2.0
**Status:** ⚠️ **OUTDATED** - Current version is 0.10.0
**Current Usage:** Active in mesh-core

**Update Required:**
```toml
# In workspace Cargo.toml
# Update: stl_io = "0.7"
# To:
stl_io = "0.10"
```

**Note:** Review [stl_io changelog](https://github.com/hmeyer/stl_io) for breaking changes between 0.7 and 0.10.

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
**Status:** ✅ Stable (Planned for Sprint 3)  
**Current Usage:** Not yet added to workspace

**Key APIs:**
```rust
use tobj::{load_obj, GPU_LOAD_OPTIONS};

let (models, materials) = load_obj("model.obj", &GPU_LOAD_OPTIONS)?;
```

**Gotchas:**
- Material files (.mtl) are optional
- Multiple objects per file possible
- Texture coordinates may be missing

#### ply-rs-bw (v0.1.3) - ✅ SECURITY PATCHED
**License:** MIT
**Status:** ✅ **FIXED** - Security-patched fork of ply-rs
**Current Usage:** Active in mesh-core

**Security Fix Applied (Dec 27, 2025):**
- **CVE:** CVE-2020-25573 (CVSS 9.8 CRITICAL) - **RESOLVED**
- **Solution:** Migrated from `ply-rs` to `ply-rs-bw` fork
- **Verification:** All 26 PLY tests passing

**Usage:**
```toml
# In mesh-core/Cargo.toml
ply-rs-bw = "0.1.3"
```

```rust
// In code, use alias for compatibility:
use ply_rs_bw as ply_rs;
```

**Notes:**
- API compatible with original ply-rs
- Fixed linked-hash-map vulnerability
- Rust 2021 edition compatible

#### gltf (v1.4)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Active (Planned for Phase 2)  
**Current Usage:** Not yet added to workspace

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

#### ruststep (v0.4) - STEP File Parsing ✅ ACTIVE
**License:** Apache-2.0  
**Status:** ✅ Active (Currently in use for STEP parsing)  
**Current Usage:** Active in mesh-core with `step` feature  
**Documentation:** See `docs/RUSTSTEP_GUIDANCE.md` for comprehensive guide

**Key Features:**
- STEP file parsing (ISO 10303-21)
- AP203 schema support (Configuration Controlled Design)
- Entity deserialization into Rust structs
- Reference resolution via Tables structure

**Key APIs:**
```rust
use ruststep::parser;
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::{TableInit, IntoOwned};

// Parse STEP file
let exchange = parser::parse(step_text)?;

// Build Tables from parsed data
let tables = Tables::from_data_sections(&exchange.data)?;

// Access entities
let msb_holders = tables.manifold_solid_brep_holders();

// Resolve references
let msb = holder.clone().into_owned(tables)?;
```

**Critical API Discovery (Riley, Dec 2025):**
- ✅ **CORRECT:** `Tables::from_data_sections(&exchange.data)` - Proper method to populate tables
- ✅ **CORRECT:** `tables.[entity_name]_holders()` - Getter methods for entity access
- ✅ **CORRECT:** `holder.clone().into_owned(tables)` - Reference resolution pattern

**Gotchas:**
- Must enable `ap203` feature for AP203 schema support
- Tables construction may fail if entities don't match AP203 schema (use fallback)
- `into_owned()` consumes holder - clone first
- Only AP203 schema currently supported (not AP214, AP242)

**Best Practice:**
```rust
// Always use TableInit::from_data_sections() - don't manually populate
let tables = match Tables::from_data_sections(&exchange.data) {
    Ok(t) => t,
    Err(e) => {
        eprintln!("Warning: Partial deserialization: {:?}", e);
        Tables::default() // Fallback
    }
};
```

**Resources:**
- Official docs: https://docs.rs/ruststep/latest/ruststep/
- GitHub: https://github.com/ricosjp/ruststep
- Project guide: `docs/RUSTSTEP_GUIDANCE.md`

#### truck (v0.3-0.4) - STEP Geometry Processing
**License:** MIT OR Apache-2.0  
**Status:** ✅ Active (Planned for Phase 3/Sprint 7)  
**Current Usage:** Active in mesh-core with `step` feature

**Components:**
- truck-modeling (v0.3.0): CAD kernel
- truck-polymesh (v0.3.0): Mesh operations
- truck-stepio (v0.3.0): STEP I/O (⚠️ **OUTPUT ONLY** - input not implemented)
- truck-meshalgo (v0.4.0): Tessellation algorithms

**Key APIs:**
```rust
use truck_modeling::Shell;
use truck_meshalgo::prelude::*;

// Tessellation (when Shell is available)
let tessellated = shell.triangulation(tolerance)?;
```

**⚠️ CRITICAL LIMITATION:**
- **truck-stepio input functionality does not exist** (v0.3.0)
- Only OUTPUT (writing) is supported
- INPUT (reading) is roadmap item
- **This blocks direct STEP → truck Shell conversion**

**Current Workaround:**
- Use ruststep for parsing and entity extraction
- Extract geometry directly from AP203 entities (FACETED_BREP approach)
- Skip truck Shell conversion, build Mesh directly

**Gotchas:**
- STEP is complex, not all features supported
- Tessellation quality affects output
- May not handle all STEP AP variants

**Decision Note:** See `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md` for architecture decision on FACETED_BREP approach

### Serialization

#### serde (v1.0) & serde_json (v1.0)
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable (Currently in workspace)  
**Current Usage:** Active in workspace.dependencies  
**Purpose:** Future use for configuration, metadata, and format-specific data structures

**Key APIs:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    // Configuration structs
}
```

**Note:** Currently in workspace but not yet actively used in code. Will be useful for format metadata and configuration.

### Utilities

#### nalgebra (v0.33)
**License:** Apache-2.0  
**Status:** ✅ Stable (Planned for Sprint 3 - mesh processing)  
**Current Usage:** Not yet added to workspace

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
**Status:** ✅ Stable (Currently in use)  
**Current Usage:** Active in workspace.dependencies

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
**Status:** ✅ Stable (Currently in workspace, for future CLI binaries)  
**Current Usage:** Active in workspace.dependencies

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

### GUI Framework: egui/eframe

**Last Updated:** December 2025  
**Researcher:** Dr. Taylor Kim  
**Status:** ✅ Planned for Sprint 7 (v0.2.1)

#### egui (v0.27) - Immediate Mode GUI
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable (Planned for Sprint 7)  
**Latest Available:** 0.33.3 (as of Jan 2026)  
**Current Usage:** Planned for converter-gui crate

**Key Features:**
- Immediate mode GUI framework (no retained state management)
- Cross-platform (Windows, macOS, Linux)
- Lightweight (~2MB overhead)
- Good for utility apps and tools
- Web and native support (via eframe)

**Best Practices:**

**1. Thread-Safe State Management:**
```rust
use std::sync::{Arc, Mutex};

struct AppState {
    // Use Arc<Mutex<>> for thread-safe state sharing
    conversion_state: Arc<Mutex<ConversionState>>,
}

// Spawn long operations in separate threads
thread::spawn(move || {
    // Long-running conversion
    let mut state = conversion_state.lock().unwrap();
    state.status = ConversionStatus::Complete;
});
```

**2. Immediate Mode Pattern:**
```rust
impl eframe::App for ConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // UI is rebuilt every frame - state must be managed carefully
        egui::CentralPanel::default().show(ctx, |ui| {
            // UI code here
        });
    }
}
```

**3. Drag and Drop:**
```rust
// Use egui::DragAndDrop API for file drops
let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
for file in dropped_files {
    if let Some(path) = &file.path {
        // Handle dropped file
    }
}
```

**Gotchas:**
- egui is immediate mode - state must be managed carefully
- Thread synchronization requires `Arc<Mutex<>>` for shared state
- File dialogs are blocking - use in separate thread if needed
- UI rebuilds every frame - avoid expensive operations in update loop
- Memory usage can grow if state isn't cleaned up properly

**Performance Tips:**
- Avoid expensive operations in `update()` method
- Use `ctx.request_repaint()` sparingly (only when needed)
- Cache expensive computations outside the update loop
- Use `egui::ScrollArea` for large content lists

#### eframe (v0.27) - Application Framework
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable (Planned for Sprint 7)  
**Latest Available:** 0.33.3 (as of Jan 2026)  
**Current Usage:** Planned for converter-gui crate

**Key Features:**
- Application framework for egui
- Handles windowing and event loop
- Cross-platform native windows
- Web support (via wasm)

**Application Structure:**
```rust
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Simple Image Converter",
        options,
        Box::new(|_cc| Box::new(ConverterApp::default())),
    )
}
```

**Window Configuration:**
```rust
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])  // Initial size
        .with_min_inner_size([800.0, 600.0])  // Minimum size
        .with_title("Simple Image Converter"),
    ..Default::default()
};
```

**Gotchas:**
- Window configuration must be set before `run_native()`
- High DPI scaling handled automatically
- System appearance (light/dark mode) supported on macOS

#### rfd (v0.14) - File Dialogs
**License:** MIT OR Apache-2.0  
**Status:** ✅ Stable (Planned for Sprint 7)  
**Latest Available:** 0.16.0 (as of Jan 2026)  
**Current Usage:** Planned for converter-gui crate

**Key Features:**
- Cross-platform file dialogs
- Native file picker on each platform
- File filter support
- Async and sync APIs

**Usage Pattern:**
```rust
use rfd::FileDialog;

// Open file dialog
let file = FileDialog::new()
    .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif"])
    .add_filter("Meshes", &["stl", "obj", "ply", "off"])
    .pick_file();

if let Some(path) = file {
    // Handle selected file
}

// Save file dialog
let file = FileDialog::new()
    .add_filter("PNG", &["png"])
    .set_file_name("output.png")
    .save_file();
```

**Gotchas:**
- File dialogs are blocking - consider using in separate thread for better UX
- Filters are case-sensitive on some platforms
- Default directory behavior varies by platform
- Paths returned are platform-specific (use `PathBuf`)

**Best Practice:**
```rust
// Use in separate thread to avoid blocking UI
let ctx = ctx.clone();
thread::spawn(move || {
    let file = FileDialog::new().pick_file();
    ctx.request_repaint();  // Notify UI of change
});
```

#### GUI Security Patterns

**1. Path Validation:**
```rust
// Always validate paths from user input
use common::validation::validate_file_path;

if let Some(path) = dropped_file.path {
    // Validate before use
    validate_file_path(&path)?;
    // Process file
}
```

**2. Error Message Sanitization:**
```rust
// Never display full paths in error messages
fn sanitize_path_for_display(path: &Path) -> String {
    // Remove user home directory if present
    // Truncate if > 60 characters
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
```

**3. Resource Limits:**
```rust
// Enforce resource limits in GUI
use common::limits::ResourceLimits;

let limits = ResourceLimits::builder()
    .max_file_size_mb(100)
    .max_image_dimension(65535)
    .build();

// Validate before processing
read_file_bytes_checked(&path, &limits)?;
```

**4. Two-Stage Format Detection:**
```rust
// Security: Always use two-stage detection (extension + magic bytes)
let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;
```

**Security Checklist for GUI:**
- [ ] All file paths validated using `common::validation::validate_file_path()`
- [ ] Two-stage format detection (extension + magic bytes)
- [ ] File size checked before reading (DoS prevention)
- [ ] Output paths validated (not in system directories)
- [ ] Filenames validated (invalid characters, path traversal prevented)
- [ ] Resource limits enforced via `ResourceLimits` builder
- [ ] Error messages sanitized (no full paths, no system info)
- [ ] All user input validated before use

#### Cross-Platform Considerations

**Windows 11:**
- Native window decorations
- High DPI scaling supported
- Windows UX patterns followed

**macOS 26:**
- Retina display support
- System appearance (light/dark mode)
- Native file dialogs

**Ubuntu LTS 24.04+:**
- GTK-compatible styling
- Wayland and X11 support
- Native file dialogs via rfd

#### Version Notes

**Current Plan:** Use egui 0.27, eframe 0.27, rfd 0.14 (as specified in GUI design)

**Latest Available (Jan 2026):**
- egui: 0.33.3
- eframe: 0.33.3
- rfd: 0.16.0

**Recommendation:** Stick with 0.27 for Sprint 7 to match design document. Consider upgrading to latest versions in future sprint after testing compatibility.

**Breaking Changes to Watch:**
- Monitor egui/eframe changelogs for API changes
- Test thoroughly if upgrading versions
- Check for deprecation warnings

#### Examples and Resources

**Official Documentation:**
- egui: https://docs.rs/egui/
- eframe: https://docs.rs/eframe/
- rfd: https://docs.rs/rfd/

**Community Resources:**
- egui GitHub: https://github.com/emilk/egui
- eframe template: https://github.com/emilk/eframe_template
- egui examples: https://github.com/emilk/egui/tree/master/examples

**Project-Specific:**
- See `GUI_DESIGN_AND_IMPLEMENTATION.md` for complete GUI design
- See `SPRINT_7_TASKING.md` for implementation tasks

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
**Next Review:** As needed (or before Sprint 3 begins)

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
