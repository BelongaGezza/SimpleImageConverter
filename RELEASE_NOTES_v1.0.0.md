# Simple Image Converter v1.0.0 - First Stable Release

**Release Date:** Mid-June 2026 (Sprint 13 target)
**Type:** Major Stable Release
**Version:** 1.0.0
**Status:** Draft — pending manual testing and release execution gates

---

## Introducing Simple Image Converter v1.0.0

We are thrilled to announce **Simple Image Converter v1.0.0** - our first stable release! After months of development across 13 sprints, we're delivering a production-ready, high-performance image and 3D mesh conversion toolkit.

Simple Image Converter is a pure Rust application providing both command-line tools and a modern GUI for converting between image and 3D mesh formats.

---

## Highlights

### Production-Ready Quality
- **657 automated tests** - Comprehensive unit, integration, security, and CLI coverage
- **Security audit Grade A** - Re-verified May 2026; resource limits enforced in both image and mesh pipelines
- **Performance optimized** - Parallel processing, efficient memory usage
- **Cross-platform** - Windows 11, macOS, Linux Ubuntu 24.04+

### Complete Feature Set
- **2D Image Conversion** - PNG, JPEG, BMP, GIF, TIFF, WebP, SVG (read-only)
- **3D Mesh Conversion** - STL, OBJ, PLY, OFF, glTF/GLB, DXF, STEP (FACETED_BREP)
- **Modern GUI** - Drag-and-drop, batch processing, 3D preview
- **CLI Tools** - `img-convert` and `mesh-convert` for automation

---

## Features

### GUI Application (converter-gui)

#### Intuitive Interface
- **Drag-and-drop** - Drop files directly onto the application
- **Format selection** - Choose output format from filtered options
- **Quality control** - Adjustable quality slider for lossy formats
- **Real-time feedback** - Status bar and message area for progress

#### Batch Processing
- **Multi-file queue** - Add multiple files for batch conversion
- **Parallel processing** - Convert multiple files simultaneously
- **Up to 4x faster** - On 4-core systems vs sequential processing
- **Configurable concurrency** - 1-16 concurrent conversions
- **Pause/Resume/Cancel** - Full control over batch operations
- **Error isolation** - Failed files don't stop the queue

#### 3D Mesh Viewer
- **Interactive preview** - Rotate, pan, zoom your 3D models
- **Rendering modes** - Solid with lighting or wireframe view
- **Hardware accelerated** - WebGPU-based rendering
- **Performance optimized** - Smooth rendering up to 100K vertices

#### Settings & Persistence
- **Auto-save settings** - Changes save automatically
- **Conversion history** - Track past conversions
- **Customizable defaults** - Set preferred quality and output location
- **Queue item editing** - Modify pending items without re-adding

#### Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open file |
| `Ctrl+A` | Add files to batch queue |
| `Ctrl+Enter` | Start batch processing |
| `Ctrl+P` / `Space` | Pause/resume batch |
| `Escape` | Cancel/close dialogs |
| `Enter` | Start conversion |
| `F1` | Help |

#### Help System
- **Keyboard shortcuts reference** - Quick access via Help menu
- **About dialog** - Version, license, and credits
- **Quick Start guide** - Get started fast

### CLI Tools

#### img-convert (Image Converter)
```bash
# Convert PNG to JPEG with quality setting
img-convert input.png jpg --quality 85 --output output.jpg

# Convert with auto-detected output format
img-convert photo.bmp webp --output photo.webp

# Enforce resource limits (propagated to format readers via get_reader_with_limits)
img-convert large.png jpg --max-file-size-mb 50 --max-dimension 8192

# Batch convert all PNGs in directory
for f in *.png; do img-convert "$f" jpg --output "${f%.png}.jpg"; done
```

#### mesh-convert (Mesh Converter)
```bash
# Convert STL to OBJ
mesh-convert model.stl obj --output model.obj

# Convert with coordinate transform
mesh-convert model.ply gltf --transform y-up --output model.gltf

# Enforce mesh resource limits
mesh-convert model.stl obj --max-file-size-mb 100 --max-vertices 5000000

# Convert STEP file (FACETED_BREP only; requires --features step build)
mesh-convert assembly.step stl --output output.stl
```

---

## Sprint 13 Release Hardening

Final validation work completed in Sprint 13 (May 2026):

- **Image resource limits** — `img-core::FormatRegistry::get_reader_with_limits` mirrors the mesh-core pattern; `img-convert` `--max-dimension` and `--max-file-size-mb` are enforced in format readers (closes RISK-006)
- **Integration test coverage** — Round-trip tests added for glTF (embedded), GLB, and DXF in `mesh-core/tests/integration.rs`
- **CLI integration tests** — Orphaned root tests wired into `img-convert/tests/cli_integration.rs` and `mesh-convert/tests/cli_integration.rs`; all run via `cargo test --workspace`
- **ADR-003 published** — Tiered two-stage format detection policy documented in `Phase3_Architecture.md` §12 (extension + magic bytes where available; parse-time validation for STL/OBJ/DXF)

---

## Supported Formats

### Image Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| PNG | Yes | Yes | Lossless, transparency support |
| JPEG | Yes | Yes | Quality 1-100, no transparency |
| BMP | Yes | Yes | Windows bitmap |
| GIF | Yes | Yes | Animation not supported |
| TIFF | Yes | Yes | Multi-page not supported |
| WebP | Yes | Yes | Lossy and lossless |
| SVG | Yes | No | Rasterization only |

### Mesh Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| STL | Yes | Yes | Binary and ASCII |
| OBJ | Yes | Yes | Wavefront, materials partial |
| PLY | Yes | Yes | Stanford polygon |
| OFF | Yes | Yes | Object file format |
| glTF/GLB | Yes | Yes | 2.0, materials supported |
| DXF | Yes | Yes | AutoCAD, 3DFACE entities |
| STEP | Yes | No | FACETED_BREP only (pre-tessellated) |

---

## Performance

### Batch Processing
- **Parallel execution** - Uses all available CPU cores
- **4-core system** - Up to 4x faster than sequential
- **8-core system** - Up to 8x faster with appropriate settings
- **Thread-safe** - No race conditions or data corruption

### Memory Usage
- **Images** - ~3x file size (read + decode + encode)
- **Meshes** - ~2x file size (read + parse + write)
- **Resource limits** - Configurable max file size and dimensions

### Benchmarks
| Operation | Time (typical) |
|-----------|----------------|
| PNG to JPEG (10MB) | < 1 second |
| STL to OBJ (50K vertices) | < 1 second |
| Batch 100 images (4 cores) | ~25 seconds |

---

## Security

- **Input validation** - All file inputs are validated and sanitized
- **Two-stage format detection** - Extension plus magic-byte verification (ADR-003 tiered policy for mesh formats without reliable signatures)
- **Resource limits** - Configurable file size, image dimensions, and mesh vertex/face limits; enforced via `get_reader_with_limits` in both `img-core` and `mesh-core`
- **Path traversal protection** - Secure file path handling
- **No unsafe code** - No `unsafe` blocks in production code paths
- **Dependency auditing** - Security audit Grade A (`SECURITY_AUDIT_v1.0.0.md`)
- **Error message sanitization** - No sensitive path information leaked

---

## Installation

### GUI Application (Recommended)

**Windows 11:**
1. Download `simpleimageconverter-gui-v1.0.0-windows-x64.zip`
2. Extract to a location of your choice
3. Run `converter-gui.exe`

**macOS:**
1. Download `simpleimageconverter-gui-v1.0.0-macos-x64.tar.gz`
2. Extract: `tar -xzf simpleimageconverter-gui-v1.0.0-macos-x64.tar.gz`
3. Run `converter-gui`

**Linux (Ubuntu 24.04+):**
1. Download `simpleimageconverter-gui-v1.0.0-linux-x64.tar.gz`
2. Extract: `tar -xzf simpleimageconverter-gui-v1.0.0-linux-x64.tar.gz`
3. Run `converter-gui`

### CLI Tools

Download the appropriate archive for your platform and extract the binaries (`img-convert`, `mesh-convert`) to your PATH.

### Build from Source

```bash
# Clone repository
git clone https://github.com/BelongaGezza/SimpleImageConverter.git
cd SimpleImageConverter

# Checkout v1.0.0
git checkout v1.0.0

# Build release binaries
cargo build --release

# Build with 3D viewer (optional)
cargo build --release --features viewer-3d

# Build with STEP read support (FACETED_BREP only)
cargo build --release --features step

# Binaries in target/release/
```

---

## Upgrade Notes

### From v0.3.0
- No breaking changes
- Settings file format unchanged
- All existing functionality preserved

### From v0.2.x
- Settings auto-save is now default (no manual save needed)
- Batch processing uses parallel mode by default
- 3D viewer requires `viewer-3d` feature flag

### From v0.1.x
- GUI application is new (CLI tools unchanged)
- STEP format support added (FACETED_BREP only)
- Resource limits now enforced by default

---

## Known Limitations

1. **STEP Format** - Only FACETED_BREP entities supported (pre-tessellated geometry). Read-only. Requires `--features step` at build time. Full B-Rep support (NURBS, curved surfaces) planned for v1.1.0.

2. **3D Viewer** - Requires `viewer-3d` feature flag. Not enabled in default builds.

3. **SVG** - Read-only (rasterization). Cannot write SVG output.

4. **GIF Animation** - Animated GIFs converted as single frame.

5. **DXF Round-Trip** - 3DFACE entities are stored as quads; round-trip conversion may expand vertex and face counts (geometry preserved, counts may differ).

6. **Mesh Format Detection** - STL, OBJ, and DXF rely on extension-based detection with parse-time validation rather than magic-byte verification (documented in ADR-003).

---

## What's Next

### v1.1.0 - Full STEP Support (Planned)
- opencascade-rs integration
- NURBS surface tessellation
- Curved surface support (cylinders, spheres, etc.)

### v1.2.0 - Installer Packages (Planned)
- MSI installer (Windows)
- DMG package (macOS)
- DEB packages (Linux)
- Package manager distribution (winget, Homebrew, apt)

---

## Version History

| Version | Date | Highlights |
|---------|------|------------|
| v0.1.0 | Dec 27, 2025 | Core converters |
| v0.1.1 | Dec 27, 2025 | Mesh transforms |
| v0.2.0 | Dec 29, 2025 | STEP format, docs |
| v0.2.1 | Dec 30, 2025 | GUI application |
| v0.2.2 | Dec 30, 2025 | Batch processing |
| v0.3.0 | Dec 30, 2025 | Parallel processing, 3D viewer |
| **v1.0.0** | **Jun 2026** | **First stable release** |

---

## Credits

**Development Team:**
- System Architect: Alex Chen
- Senior Engineer: Jordan Rivera
- Junior Engineer (2D): Sam Kim
- Junior Engineer (3D): Alex Rivera
- UI Designer: Jamie Chen
- Security Specialist: Casey Morgan
- Documentation Specialist: Sam Parker
- Researcher: Dr. Taylor Kim

**Built With:**
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [egui/eframe](https://github.com/emilk/egui) - Immediate mode GUI
- [image](https://github.com/image-rs/image) - Image processing
- [wgpu](https://wgpu.rs/) - WebGPU rendering
- [rayon](https://github.com/rayon-rs/rayon) - Parallel processing

**Special Thanks:**
- Rust community for excellent crates
- All contributors and testers
- Users who provided feedback during development

---

## Support

- **Issues:** [GitHub Issues](https://github.com/BelongaGezza/SimpleImageConverter/issues)
- **Documentation:** See `docs/` directory
- **License:** MIT OR Apache-2.0

---

## Release Information

**Version:** 1.0.0
**Release Date:** Mid-June 2026 (Sprint 13)
**Git Tag:** `v1.0.0`
**Previous Release:** v0.3.0 (December 30, 2025)

---

**Thank you for using Simple Image Converter!**

*This is our first stable release. We're committed to maintaining quality and adding new features in future versions.*
