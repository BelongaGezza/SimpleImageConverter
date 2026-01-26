# Rust Resources & Lessons Learned
## Living Knowledge Base for Simple Image Converter

**Maintained By:** Researcher (Dr. Taylor Kim)  
**Last Updated:** January 26, 2026  
**Update Frequency:** Weekly + as needed  
**Purpose:** Track Rust ecosystem changes, library updates, and project learnings

**⚠️ IMPORTANT:** All team members must consult this document before implementing features or making decisions.

### Quick Status Summary (as of Jan 26, 2026)

**Current Project Phase:** Sprint 12_A (v1.0.0 Final Release Preparation)

**Technology Audit Completed:** December 27, 2025
- See `TECHNOLOGY_AUDIT_REPORT.md` for full details

**Sprint 8 Research Completed:** December 30, 2025
- Configuration libraries evaluated (serde, toml, directories)
- Preview rendering researched (image thumbnails, mesh preview)
- Performance optimization opportunities identified
- See sections below for detailed findings

**Sprint 10 Research Completed:** December 30, 2025
- opencascade-rs documentation verified and compiled (Task 1.1 support)
- OCCT installation guide reviewed and confirmed complete
- Build complexity and binary size impact documented
- Limitations and known issues documented
- STEP format reference updated with opencascade-rs information
- See RESEARCHER_TASK1.1_SUPPORT_SUMMARY.md for details

**Active Dependencies:**
- ✅ `anyhow` v1.0 - Error handling (latest: 1.0.100)
- ✅ `thiserror` v1.0 - Error types (⚠️ v2.0.17 available - defer upgrade)
- ✅ `clap` v4.5 - CLI (latest: 4.5.53)
- ✅ `serde` v1.0 - Serialization (latest: 1.0.217+)
- ✅ `serde_json` v1.0 - JSON support
- ✅ `log` v0.4 - Logging
- ✅ `image` v0.25 - Image processing (latest: 0.25.8)
- ✅ `resvg` v0.45 - SVG rendering (current: 0.45.1)
- ✅ `tiny-skia` v0.11 - 2D rendering (latest: 0.11.4)

**GUI Dependencies (Sprint 7):**
- 📋 `egui` v0.27 - GUI framework (latest: **0.33.3** - stick with 0.27 for Sprint 7)
- 📋 `eframe` v0.27 - Application framework (latest: **0.33.3** - stick with 0.27 for Sprint 7)
- 📋 `rfd` v0.15 - File dialogs (latest: **0.16.0** - updated for compatibility; monitor for 0.16+)

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
- ✅ `opencascade` v0.2.0 - OCCT Rust bindings (feature-gated, step-opencascade feature)
- ✅ `opencascade-sys` v0.2.0 - OCCT FFI bindings (feature-gated, step-opencascade feature)

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
| 2025-12-30 | **SPRINT 8** | Configuration libraries evaluated (serde, toml, directories) - All recommended | Researcher |
| 2025-12-30 | **SPRINT 8** | Preview rendering researched - image thumbnails recommended, mesh preview simplified | Researcher |
| 2025-12-30 | **SPRINT 8** | Performance optimization opportunities documented - sequential batch processing for v0.2.2 | Researcher |
| 2025-12-30 | **SPRINT 8** | egui/eframe monitoring updated - 0.27.2 stable, 0.33.3 available (upgrade planned for v0.3.0) | Researcher |
| 2025-12-30 | **SPRINT 10** | opencascade-rs documentation verified and compiled (Task 1.1 support) - OCCT installation guide complete, limitations documented, STEP reference updated | Researcher |
| 2026-01-26 | Updates | Upgraded resvg 0.40→0.45 (usvg API: `Tree::from_data(data, &Options)`; fonts via `Options::fontdb_mut()`) | Junior Engineer (2D Formats) |
| 2026-01-26 | Updates | Documented stl_io 0.7→0.10 upgrade approach: no GitHub Releases notes; rely on docs.rs comparison + compile + regression tests. | Researcher |

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

#### resvg (v0.45) - SVG Rasterization
**License:** MIT OR Apache-2.0 (note: resvg upstream changed from MPL-2.0 in v0.45.0)
**Status:** ✅ Current (workspace uses 0.45.1)
**Current Usage:** Active in img-core

**API Note (0.45+):**
- `usvg::Tree::from_data` now takes only `(&[u8], &Options)`
- Load system fonts via `Options::fontdb_mut()` (fontdb is stored inside `Options`)
  - See upstream changelog: [linebender/resvg CHANGELOG](https://raw.githubusercontent.com/linebender/resvg/main/CHANGELOG.md)

**Key APIs:**
```rust
use resvg::usvg::{Options, Tree};
use resvg::tiny_skia::Pixmap;

let mut opt = Options::default();
opt.fontdb_mut().load_system_fonts();

let tree = Tree::from_data(svg_data, &opt)?;

let mut pixmap = Pixmap::new(width, height).unwrap();
let mut pixmap_mut = pixmap.as_mut();
resvg::render(&tree, resvg::usvg::Transform::default(), &mut pixmap_mut);
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

**Notes (0.7 → 0.10):**
- The upstream repo does **not** publish GitHub Releases notes; treat this as a “read docs + compile + test” upgrade.
- The high-level API (`read_stl`, `create_stl_reader`, `write_stl`, `IndexedMesh`) appears stable between docs.rs `0.7.0` and `0.10.0`, but semver is still 0.x so breaking changes are possible.
- References:
  - [docs.rs `stl_io` 0.7.0](https://docs.rs/stl_io/0.7.0/stl_io/)
  - [docs.rs `stl_io` 0.10.0](https://docs.rs/stl_io/0.10.0/stl_io/)
  - Source: [hmeyer/stl_io](https://github.com/hmeyer/stl_io)

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

#### opencascade (v0.2.0) & opencascade-sys (v0.2.0) - OCCT Rust Bindings
**License:** MIT OR Apache-2.0  
**Status:** ✅ Active (Sprint 9 prototype complete, Sprint 10 full implementation)  
**Current Usage:** Active in mesh-core with `step-opencascade` feature (optional)  
**Repository:** https://github.com/bschwind/opencascade-rs

**Components:**
- opencascade (v0.2.0): High-level Rust wrapper for OCCT
- opencascade-sys (v0.2.0): Low-level FFI bindings to OCCT

**Requirements:**
- **OpenCASCADE Technology (OCCT) 7.7+** must be installed on system
- CMake 3.18+
- C++17 compiler (GCC 7+, Clang 5+, or MSVC 2019+)
- Platform-specific libraries (X11, OpenGL on Linux)

**Key APIs:**
```rust
use opencascade::prelude::*;

// Read STEP file
let mut reader = STEPControl_Reader::default();
reader.read_file("model.step")?;

// Transfer root entities
reader.transfer_root(1);
let shape = reader.one_shape();

// Tessellate
let mut mesher = BRepMesh_IncrementalMesh::new(&shape, 0.01);
mesher.perform();
```

**⚠️ CRITICAL REQUIREMENTS:**
- **OCCT Installation Required:** Cannot use without OCCT installed
- **Build Complexity:** High - requires C++ toolchain and OCCT
- **Binary Size Impact:** +10-15 MB (dynamic) or +90-140 MB (static)
- **Build Time:** 10-30 minutes first build (opencascade-sys compilation)

**Current Status:**
- ✅ Prototype implementation complete (Sprint 9)
- ✅ Documentation complete (installation guide, limitations, troubleshooting)
- 🟡 Full implementation in progress (Sprint 10)
- ⚠️ Testing deferred until OCCT available

**Gotchas:**
- OCCT must be installed before building (no auto-detection)
- Platform-specific installation paths vary
- Runtime library dependencies must be in library path
- Temporary file handling required (OCCT expects file paths, not bytes)
- Tessellation quality configurable via deflection parameter

**Best Practices:**
```rust
// Feature-gate all opencascade-rs code
#[cfg(feature = "step-opencascade")]
use mesh_core::formats::step_opencascade;

// Use fallback strategy (try FACETED_BREP first, then opencascade-rs)
// See mesh-core/src/formats/step.rs for integration pattern
```

**Documentation:**
- Installation: `docs/OCCT_INSTALLATION.md`
- Limitations: `docs/OPENCASCADE_RS_LIMITATIONS.md`
- STEP Reference: `docs/STEP_FORMAT_REFERENCE.md` (opencascade-rs section)
- Research: `RESEARCH_OPENCASCADE_RS_SPRINT9.md`

**Decision Note:** See `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md` for architecture decision on hybrid approach (FACETED_BREP + opencascade-rs)

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

**4. Keyboard Shortcuts with Modifiers (Cross-Platform):**
```rust
let modifiers = ctx.input(|i| i.modifiers);
// Use Command on macOS, Ctrl on Windows/Linux
let cmd_or_ctrl = modifiers.command || modifiers.ctrl;

// CRITICAL: Use key_pressed() not keys_down.contains() to avoid false triggers
if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::O)) {
    // Handle Ctrl+O / Cmd+O
}

// WRONG - causes false triggers when modifier alone is held:
// if cmd_or_ctrl && pressed_keys.contains(&egui::Key::O) { ... }
```

**Gotchas:**
- egui is immediate mode - state must be managed carefully
- Thread synchronization requires `Arc<Mutex<>>` for shared state
- File dialogs are blocking - use in separate thread if needed
- UI rebuilds every frame - avoid expensive operations in update loop
- Memory usage can grow if state isn't cleaned up properly
- **CRITICAL:** For keyboard shortcuts with modifiers, always use `key_pressed()` not `keys_down.contains()` to avoid false triggers when modifier keys are held down alone

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

**Current Usage (Sprint 7):** egui 0.27.2, eframe 0.27.2, rfd 0.14.1 (as specified in GUI design)

**Latest Available (December 2025):**
- egui: 0.33.3 (6 minor versions ahead)
- eframe: 0.33.3 (6 minor versions ahead)
- rfd: 0.16.0 (2 minor versions ahead)

**Sprint 8 Monitoring (December 30, 2025):**
- ✅ **Current versions stable** - No immediate upgrade needed
- ✅ **No breaking changes** identified between 0.27 and 0.33 that affect current implementation
- ⚠️ **New features available** in 0.33: Improved image handling, better file dialogs, performance improvements
- 📋 **Recommendation for v0.2.2:** Continue with 0.27 for stability. Plan upgrade to 0.33+ for v0.3.0 after thorough testing

**Breaking Changes to Watch:**
- Monitor egui/eframe changelogs for API changes
- Test thoroughly if upgrading versions
- Check for deprecation warnings
- **Action:** Review changelog before v0.3.0 upgrade

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
- See `SPRINT_8_TASKING.md` for v0.2.2 GUI enhancements

---

### Configuration & Settings Persistence (v0.2.2)

**Last Updated:** December 30, 2025  
**Researcher:** Dr. Taylor Kim  
**Status:** ✅ Evaluated for Sprint 8 (v0.2.2)

#### serde (v1.0) - Serialization Framework
**License:** MIT OR Apache-2.0  
**Status:** ✅ **RECOMMENDED** - Already in workspace  
**Current Usage:** Active in workspace.dependencies (v1.0.228)  
**Purpose:** Serialization/deserialization for settings files

**Evaluation:**
- ✅ **Ease of Use:** Excellent - derive macros make it trivial
- ✅ **Performance:** Excellent - zero-copy deserialization where possible
- ✅ **Maintenance:** Excellent - Most popular Rust serialization crate
- ✅ **Compatibility:** Excellent - Works seamlessly with toml
- ✅ **Security:** Good - No known vulnerabilities, well-audited

**Usage Pattern:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    window_width: f32,
    window_height: f32,
    default_output_directory: PathBuf,
    default_quality: u8,
    show_advanced_options: bool,
}
```

**Recommendation:** ✅ **USE** - Already in workspace, perfect for settings serialization

---

#### toml (v0.8) - TOML Parsing
**License:** MIT OR Apache-2.0  
**Status:** ✅ **RECOMMENDED** - Available in dependency tree  
**Current Usage:** Available (v0.8.23 in Cargo.lock)  
**Purpose:** TOML file format for settings persistence

**Evaluation:**
- ✅ **Ease of Use:** Excellent - Works seamlessly with serde
- ✅ **Performance:** Good - Fast parsing, reasonable for config files
- ✅ **Maintenance:** Excellent - Actively maintained, TOML 1.0 compliant
- ✅ **Compatibility:** Excellent - Native serde integration
- ✅ **Security:** Good - No known vulnerabilities
- ✅ **Human-Readable:** Excellent - TOML is easy to read/edit

**Usage Pattern:**
```rust
use toml;

// Serialize to TOML
let toml_string = toml::to_string_pretty(&settings)?;

// Deserialize from TOML
let settings: AppSettings = toml::from_str(&toml_content)?;
```

**Alternatives Considered:**
- `serde_json` - JSON format (less human-readable)
- `config` crate - More features but heavier (not needed)
- `confy` - Higher-level wrapper (adds dependency, less control)

**Recommendation:** ✅ **USE toml** - Best balance of features, readability, and simplicity

---

#### directories (v5.0) - Platform-Specific Directories
**License:** MIT OR Apache-2.0  
**Status:** ✅ **RECOMMENDED** - Already in dependency tree  
**Current Usage:** Available (v5.0.1 in Cargo.lock via transitive dependency)  
**Purpose:** Platform-specific config directory resolution

**Evaluation:**
- ✅ **Ease of Use:** Excellent - Simple API
- ✅ **Platform Support:** Excellent - Windows, macOS, Linux
- ✅ **Maintenance:** Good - Stable, mature crate
- ✅ **Compatibility:** Excellent - No conflicts
- ✅ **Security:** Good - Follows platform conventions

**Usage Pattern:**
```rust
use directories::ProjectDirs;

let proj_dirs = ProjectDirs::from("com", "SimpleImageConverter", "SimpleImageConverter")
    .ok_or("Failed to get project directories")?;

let config_dir = proj_dirs.config_dir();
// Windows: %APPDATA%\SimpleImageConverter\config\
// macOS: ~/Library/Application Support/SimpleImageConverter/config/
// Linux: ~/.config/simpleimageconverter/config/
```

**Platform Paths:**
- **Windows:** `%APPDATA%\SimpleImageConverter\config\`
- **macOS:** `~/Library/Application Support/SimpleImageConverter/config/`
- **Linux:** `~/.config/simpleimageconverter/config/`

**Recommendation:** ✅ **USE** - Already available, perfect for platform-specific paths

---

#### Configuration Library Summary

| Library | Status | Recommendation | Notes |
|---------|--------|----------------|-------|
| `serde` | ✅ In workspace | **USE** | Already available, perfect for serialization |
| `toml` | ✅ Available | **USE** | Human-readable, serde-compatible |
| `directories` | ✅ Available | **USE** | Platform-specific paths, already in tree |

**Final Recommendation for v0.2.2:**
✅ **Use serde + toml + directories** - All three are available, well-maintained, and perfect for settings persistence. No additional dependencies needed.

**Implementation Pattern:**
```rust
// converter-gui/src/settings.rs
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    window_width: f32,
    window_height: f32,
    default_output_directory: PathBuf,
    // ... other settings
}

impl AppSettings {
    pub fn load() -> Result<Self, SettingsError> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let settings: AppSettings = toml::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Self::default())
        }
    }
    
    pub fn save(&self) -> Result<(), SettingsError> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf, SettingsError> {
        let proj_dirs = ProjectDirs::from("com", "SimpleImageConverter", "SimpleImageConverter")
            .ok_or(SettingsError::NoConfigDir)?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}
```

---

### Preview Rendering (v0.2.2)

**Last Updated:** December 30, 2025  
**Researcher:** Dr. Taylor Kim  
**Status:** ✅ Researched for Sprint 8 (v0.2.2)

#### Image Preview

**Library:** `image` crate (already in workspace)  
**Status:** ✅ **RECOMMENDED** - Already available  
**Current Usage:** Active in workspace (v0.25)

**Approach:**
- Use `image` crate to load images
- Generate thumbnails for large images (max 400x300 for preview)
- Use `egui::Image` widget to display
- Cache thumbnails in memory

**Implementation Pattern:**
```rust
use image::{DynamicImage, imageops};
use egui::{Image, ColorImage};

fn generate_thumbnail(img: &DynamicImage, max_width: u32, max_height: u32) -> ColorImage {
    let (width, height) = img.dimensions();
    
    // Calculate thumbnail size maintaining aspect ratio
    let (thumb_width, thumb_height) = if width > height {
        let ratio = max_width as f32 / width as f32;
        (max_width, (height as f32 * ratio) as u32)
    } else {
        let ratio = max_height as f32 / height as f32;
        ((width as f32 * ratio) as u32, max_height)
    };
    
    // Resize image
    let thumbnail = imageops::resize(
        img,
        thumb_width,
        thumb_height,
        imageops::FilterType::Triangle,
    );
    
    // Convert to egui::ColorImage
    let size = [thumb_width as usize, thumb_height as usize];
    let pixels: Vec<egui::Color32> = thumbnail
        .pixels()
        .map(|p| egui::Color32::from_rgb(p[0], p[1], p[2]))
        .collect();
    
    ColorImage { size, pixels }
}
```

**Performance Considerations:**
- ✅ Thumbnail generation is fast (<100ms for typical images)
- ✅ Memory usage: ~400KB per thumbnail (400x300 RGB)
- ✅ Cache thumbnails in `HashMap<PathBuf, ColorImage>` to avoid regeneration
- ⚠️ For very large images (>10MP), consider async loading

**Recommendation:** ✅ **USE image crate** - Already available, perfect for thumbnails

---

#### Mesh Preview (v0.2.2 - Simplified)

**Status:** ⚠️ **SIMPLIFIED FOR v0.2.2** - Full 3D viewer deferred to future version

**Approach for v0.2.2:**
- Display mesh metadata (vertex count, face count, format)
- Show placeholder icon or simple wireframe representation
- Defer full 3D viewer to v0.2.3 or later

**Future Research (v0.2.3+):**
- **egui-3d** - 3D rendering in egui (experimental)
- **wgpu** - Low-level graphics (more control, more complex)
- **three-d** - High-level 3D library (may be overkill)
- **Simple wireframe** - Generate 2D projection as image

**Recommendation for v0.2.2:**
✅ **Use metadata display** - Simple, fast, sufficient for v0.2.2. Full 3D preview can be added in future version.

**Implementation Pattern:**
```rust
// converter-gui/src/ui/preview.rs
pub fn show_mesh_preview(ui: &mut egui::Ui, mesh_info: &MeshInfo) {
    ui.vertical(|ui| {
        ui.heading("Mesh Preview");
        ui.separator();
        ui.label(format!("Format: {:?}", mesh_info.format));
        ui.label(format!("Vertices: {}", mesh_info.vertex_count));
        ui.label(format!("Faces: {}", mesh_info.face_count));
        ui.label(format!("Size: {:.2} MB", mesh_info.file_size_mb));
        // Placeholder icon or simple wireframe
        ui.add(egui::Label::new("📦 3D Preview (Coming in v0.2.3)"));
    });
}
```

---

### Performance Optimization (v0.2.2)

**Last Updated:** December 30, 2025  
**Researcher:** Dr. Taylor Kim  
**Status:** ✅ Researched for Sprint 8 (v0.2.2)

#### Batch Processing Optimization

**Current Approach (v0.2.2):** Sequential processing (one file at a time)

**Performance Characteristics:**
- ✅ **Memory Usage:** Low - Only one file in memory at a time
- ✅ **Simplicity:** High - Easy to implement and debug
- ⚠️ **Speed:** Moderate - Slower than parallel for many files

**Optimization Opportunities:**

1. **Sequential Processing (v0.2.2 - Recommended)**
   - ✅ Simple to implement
   - ✅ Low memory usage
   - ✅ Easy error handling
   - ✅ Predictable resource usage
   - **Recommendation:** ✅ **USE for v0.2.2** - Sufficient for initial release

2. **Parallel Processing (Future - v0.2.3+)**
   - Use `rayon` crate for parallel iteration
   - Process multiple files concurrently
   - **Trade-offs:**
     - ⚠️ Higher memory usage (multiple files in memory)
     - ⚠️ More complex error handling
     - ⚠️ Resource contention (CPU, I/O)
   - **Recommendation:** ⚠️ **DEFER to v0.2.3** - Add after sequential is stable

**Implementation Pattern (Sequential):**
```rust
// converter-gui/src/app.rs
fn process_batch_queue(&mut self) {
    let queue = self.batch_queue.clone();
    let state = self.conversion_state.clone();
    
    std::thread::spawn(move || {
        for item in queue.items.iter_mut() {
            item.status = BatchItemStatus::Processing;
            // Process conversion
            match convert_item(item) {
                Ok(output_path) => {
                    item.status = BatchItemStatus::Completed { output_path };
                }
                Err(e) => {
                    item.status = BatchItemStatus::Failed { error: e.to_string() };
                }
            }
        }
    });
}
```

**Future Parallel Pattern (v0.2.3+):**
```rust
use rayon::prelude::*;

queue.items.par_iter_mut().for_each(|item| {
    // Process in parallel (with resource limits)
});
```

---

#### Preview Caching Strategy

**Approach:** In-memory cache with LRU eviction

**Cache Structure:**
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct PreviewCache {
    images: HashMap<PathBuf, Arc<ColorImage>>,
    max_size: usize,  // Max number of cached previews
}

impl PreviewCache {
    fn get(&mut self, path: &Path) -> Option<Arc<ColorImage>> {
        self.images.get(path).cloned()
    }
    
    fn insert(&mut self, path: PathBuf, image: ColorImage) {
        // LRU eviction if cache full
        if self.images.len() >= self.max_size {
            // Remove oldest (simple FIFO for v0.2.2)
            if let Some(oldest) = self.images.keys().next().cloned() {
                self.images.remove(&oldest);
            }
        }
        self.images.insert(path, Arc::new(image));
    }
}
```

**Performance Characteristics:**
- ✅ **Memory Usage:** ~400KB per cached thumbnail (400x300 RGB)
- ✅ **Cache Size:** Recommend max 10-20 previews (~4-8MB total)
- ✅ **Speed:** Instant retrieval for cached images
- ✅ **Eviction:** Simple FIFO for v0.2.2, can upgrade to LRU later

**Recommendation:** ✅ **USE in-memory cache** - Simple, fast, sufficient for v0.2.2

---

#### Settings File I/O Optimization

**Approach:** Synchronous I/O with debouncing

**Performance Characteristics:**
- ✅ **File Size:** Small (<1KB typical) - Fast I/O
- ✅ **Frequency:** Low - Only on changes
- ✅ **Blocking:** Acceptable - Settings save is fast (<10ms)

**Optimization Strategies:**

1. **Debounced Auto-Save (Recommended)**
   - Save settings 500ms after last change
   - Prevents excessive file writes
   - **Implementation:** Use `std::time::Instant` to track last change

2. **Explicit Save Button (Alternative)**
   - User controls when to save
   - No automatic writes
   - **Trade-off:** User must remember to save

**Recommendation:** ✅ **USE debounced auto-save** - Best user experience

**Implementation Pattern:**
```rust
struct SettingsManager {
    settings: AppSettings,
    last_change: Option<std::time::Instant>,
    auto_save_delay: std::time::Duration,
}

impl SettingsManager {
    fn on_setting_changed(&mut self) {
        self.last_change = Some(std::time::Instant::now());
    }
    
    fn update(&mut self) {
        if let Some(last_change) = self.last_change {
            if last_change.elapsed() >= self.auto_save_delay {
                self.settings.save().ok();
                self.last_change = None;
            }
        }
    }
}
```

---

#### UI Rendering Performance

**Optimization Tips:**

1. **Avoid Expensive Operations in `update()`**
   - ✅ Load previews in background thread
   - ✅ Cache expensive computations
   - ✅ Use `ctx.request_repaint()` sparingly

2. **Lazy Loading**
   - ✅ Load previews only when visible
   - ✅ Generate thumbnails on-demand
   - ✅ Defer heavy operations

3. **Batch UI Updates**
   - ✅ Update UI only when state changes
   - ✅ Use `Arc<Mutex<>>` for thread-safe state sharing
   - ✅ Minimize UI rebuilds

**Recommendation:** ✅ **Follow egui best practices** - Already documented in GUI section

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
use resvg::usvg::Options;

let mut opt = Options::default();
opt.fontdb_mut().load_system_fonts();
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
