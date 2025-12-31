# Simple Image Converter

A high-performance, pure Rust command-line toolkit for converting between image and 3D mesh formats.

## 🎯 Project Overview

Two separate CLI tools written in Rust:
- **img-convert** - 2D image format converter
- **mesh-convert** - 3D mesh and CAD format converter

**Status:** ✅ Active Development (Private Repository)

**Current Version:** 0.3.0 (Released - December 30, 2025)  
**Next Version:** 0.3.1 (Planned - UI Enhancements)

## ✨ Features

### Image Converter (img-convert)
- **✅ Implemented Formats:** PNG, JPEG, BMP, GIF, TIFF, WebP, SVG (read-only, rasterization)
- **📅 Planned Formats:** TGA, ICO, DDS, HDR, OpenEXR, AVIF, PDF
- High-quality conversion with configurable compression (1-100 quality scale)
- Transparency handling (RGBA support)
- Two-stage format detection (extension + magic bytes)
- Resource limits and security validation
- Fast processing with minimal memory footprint

### 3D Mesh Converter (mesh-convert)
- **✅ Implemented Formats:** STL (binary/ASCII), OBJ, PLY, OFF, glTF/GLB, DXF
- **✅ STEP Format Support (v0.2.0):** Read-only, feature-gated, **FACETED_BREP only** (pre-tessellated geometry)
- **✅ Advanced Features:** Coordinate system transforms, normal recalculation, mesh validation
- **⚠️ STEP Limitations:** Only supports FACETED_BREP entities. Full B-Rep support (NURBS, cylinders, etc.) planned for v0.3.0. See `docs/CAD_EXPORT_GUIDE.md` for export instructions.
- Material preservation (where supported)
- Resource limits and security validation
- Binary and ASCII format variants

## 🚀 Quick Start

### Installation

#### GUI Application (Recommended for Most Users)

**Windows 11:**
1. Download `simpleimageconverter-gui-v0.2.2-windows-x64.zip` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Extract to a location of your choice (e.g., `C:\Tools\SimpleImageConverter`)
3. Run `converter-gui.exe`

**macOS:**
1. Download `simpleimageconverter-gui-v0.2.2-macos-x64.tar.gz` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Extract: `tar -xzf simpleimageconverter-gui-*.tar.gz`
3. Run `converter-gui`

**Linux (Ubuntu 24.04+):**
1. Download `simpleimageconverter-gui-v0.2.2-linux-x64.tar.gz` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Extract: `tar -xzf simpleimageconverter-gui-*.tar.gz`
3. Run `converter-gui`

#### Option 1: Pre-built CLI Binaries

**Windows 11:**
1. Download `simpleimageconverter-{version}-windows-x64.zip` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Extract to a location of your choice (e.g., `C:\Tools\SimpleImageConverter`)
3. (Optional) Add the directory to your PATH environment variable
4. Open Command Prompt or PowerShell and run: `img-convert --help`

**macOS:**
1. Download `simpleimageconverter-{version}-macos-x64.tar.gz` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Extract: `tar -xzf simpleimageconverter-*.tar.gz`
3. (Optional) Install system-wide: `sudo cp img-convert mesh-convert /usr/local/bin/`
4. Run: `img-convert --help`

**Linux (Ubuntu 24.04+):**
1. Download `simpleimageconverter-{version}-linux-x64.tar.gz` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Extract: `tar -xzf simpleimageconverter-*.tar.gz`
3. (Optional) Install system-wide: `sudo cp img-convert mesh-convert /usr/local/bin/`
4. Run: `img-convert --help`

**Or install via package manager:**
- **Windows:** `winget install BelongaGezza.SimpleImageConverter` (coming soon)
- **macOS:** `brew install --cask simpleimageconverter` (coming soon)
- **Linux:** `sudo apt install simpleimageconverter` (coming soon)

### GUI Usage

The GUI provides an intuitive drag-and-drop interface for file conversion:

1. **Launch** `converter-gui` (or `converter-gui.exe` on Windows)
2. **Drag and drop** a file into the drop zone, or click "Browse Files..." to select a file
3. **Select output format** from the radio buttons (formats are automatically filtered based on file type)
4. **Adjust options** (optional):
   - Change output filename
   - Select output location
   - Adjust quality slider (for JPEG/WebP images)
5. **Click "Convert"** to start the conversion
6. **View results** in the status bar and messages area

**Supported Formats:**
- **Images:** PNG, JPEG, BMP, GIF, TIFF, WebP (SVG read-only)
- **Meshes:** STL, OBJ, PLY, OFF, glTF, DXF (STEP read-only, feature-gated)

**Features:**
- Drag-and-drop file support
- Visual format selection
- Quality settings for lossy image formats
- User-friendly error messages
- Progress indicators for long operations
- Thread-safe conversion processing
- **v0.2.2:** Batch processing, preview functionality, settings persistence, conversion history
- **v0.3.0:** Parallel batch processing (4x speedup on 4-core systems), settings auto-save, queue item editing
- **v0.3.0:** Pause/resume/cancel controls for batch processing
- **v0.3.0:** Interactive 3D mesh viewer with camera controls and rendering modes

#### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/BelongaGezza/SimpleImageConverter.git
cd SimpleImageConverter

# Build all tools
cargo build --release

# Binaries will be in target/release/
```

### Usage Examples

**Image Conversion:**
```bash
# Basic conversion
./img-convert input.png jpg

# With quality control
./img-convert photo.png jpg --quality 95

# Custom output path
./img-convert image.bmp png --output result.png

# Rasterize SVG at 300 DPI
./img-convert logo.svg png --dpi 300
```

**Mesh Conversion:**
```bash
# Basic conversion
./mesh-convert model.stl obj

# With coordinate transform (Z-up to Y-up)
./mesh-convert model.stl obj --transform y-up

# With explicit coordinate system transform
./mesh-convert model.stl obj --transform z-up:y-up

# Recalculate vertex normals
./mesh-convert model.stl obj --recalculate-normals

# Validate mesh integrity
./mesh-convert model.stl obj --validate

# Combined options
./mesh-convert model.stl obj --transform y-up --recalculate-normals --validate --output result.obj
```

## 📋 Requirements

- **Rust:** 1.92+ (MSRV)
- **Target:** x86-64 Windows 11 (primary), cross-platform capable
- **Memory:** 100MB+ recommended for large files

## 🏗️ Architecture

```
workspace/
├── common/              # Shared utilities
├── img-core/            # 2D image conversion library
├── img-convert/         # 2D CLI binary
├── mesh-core/           # 3D mesh conversion library
├── mesh-convert/        # 3D CLI binary
└── converter-gui/       # ✅ GUI application (v0.2.1)
```

**Design Principles:**
- Library-first architecture (binaries are thin wrappers)
- Trait-based format system for extensibility
- Zero-copy where possible
- Comprehensive error handling
- Extensive testing

## 🛠️ Development

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized for size)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code
cargo clippy

# Format code
cargo fmt
```

### Feature Flags

```bash
# Build with STEP support (optional)
cargo build --features step

# Build with 3D viewer support (optional, requires wgpu)
cargo build --features viewer-3d

# Build with both STEP and 3D viewer support
cargo build --features step,viewer-3d

# Build without STEP support (default)
cargo build --no-default-features
```

### Cross-Compilation (Linux → Windows)

```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW
sudo apt-get install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## 📊 Project Status

### ✅ Completed Phases

**Sprint 1: Project Foundation** ✅ COMPLETE
- [x] Project setup and workspace structure
- [x] CI/CD pipeline
- [x] Architecture design and documentation

**Sprint 2: img-convert Core** ✅ COMPLETE
- [x] PNG, JPEG, BMP, GIF formats
- [x] CLI implementation
- [x] Comprehensive testing (164 tests)

**Sprint 3: mesh-convert Core** ✅ COMPLETE
- [x] STL, OBJ, PLY formats
- [x] CLI implementation
- [x] Comprehensive testing (155 tests)

**Sprint 4: Advanced 2D Formats** ✅ COMPLETE
- [x] TIFF, WebP, SVG (read-only) formats
- [x] Quality control and compression
- [x] Security validation

**Sprint 5: Advanced 3D Formats** ✅ COMPLETE
- [x] glTF, DXF, OFF formats
- [x] Material preservation
- [x] Format detection and validation

### ✅ Current Phase

**Sprint 6: Quality & Testing** ✅ COMPLETE
- [x] Test coverage (192 tests, excellent coverage)
- [x] Code quality (no clippy warnings)
- [x] Security posture (zero unsafe code)
- [x] Documentation updates (complete)
- [x] STEP format support (v0.2.0 released)
- [x] Advanced mesh features (transform, recalculate-normals, validate)

**v0.2.0 Release Highlights:**
- ✅ STEP format support (FACETED_BREP extraction)
- ✅ 192 tests passing (all test suites)
- ✅ Zero compilation errors or warnings
- ✅ Production-ready security posture

### 📅 Planned Phases

**Sprint 7: GUI Implementation** ✅ **COMPLETE** (v0.2.1 released)
- [x] GUI application with egui framework
- [x] Drag-and-drop file support
- [x] Visual format selection
- [x] Direct library integration
- [x] User-friendly interface

**Sprint 8: GUI Enhancements** ✅ **COMPLETE** (v0.2.2 released)
- [x] v0.2.1 release preparation and packaging
- [x] Batch processing UI
- [x] Settings panel and persistence
- [x] Preview functionality
- [x] Conversion history

**Sprint 9: v0.3.0 Feature Development** ✅ **COMPLETE**
- [x] Parallel batch processing implementation ✅ **COMPLETE**
- [x] Settings auto-save implementation ✅ **COMPLETE**
- [x] Queue item editing implementation ✅ **COMPLETE**
- [x] Integration testing ✅ **COMPLETE**
- [x] Security review ✅ **COMPLETE**

**Sprint 10: v0.3.0 Feature Completion** 🟡 **IN PROGRESS**
- [x] opencascade-rs testing and documentation ✅ **COMPLETE**
- [x] 3D mesh viewer full implementation ✅ **COMPLETE** (Sprint 10_A)
- [x] Parallel processing UI controls (pause/resume/cancel) ✅ **COMPLETE**
- [ ] Integration testing (Sprint 10_A - in progress)

**Sprint 7-8: STEP + CAD** ✅ COMPLETE (v0.2.0)
- [x] STEP FACETED_BREP extraction (v0.2.0)
- [x] STEP read support (feature-gated)
- [x] CAD export documentation
- [ ] Full STEP B-Rep support (v0.3.0 - opencascade-rs integration) - **IN RESEARCH**

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_png_to_jpg

# Run with output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration

# Benchmarks
cargo bench
```

Test coverage includes:
- ✅ 192 tests total covering all format implementations
- ✅ Unit tests for all format readers/writers
- ✅ Integration tests for format conversions (including STEP)
- ✅ Security tests for format spoofing and malformed input
- ✅ Edge case handling (empty files, invalid data, oversized files)

## 📦 Binary Sizes

| Tool | Size (Release) | With STEP |
|------|---------------|-----------|
| img-convert | ~3-5 MB | N/A |
| mesh-convert | ~2-4 MB | ~4-6 MB |
| Combined | ~5-8 MB | ~7-10 MB |

## 🤝 Contributing

This repository is currently **private** during initial development. Once mature:

1. Issues and PRs will be enabled
2. Contribution guidelines will be published
3. Code of conduct will be established

**Development Team:**
- Maintained via Claude AI, Claude Code, and Cursor 2.2
- Following agile sprint methodology

## 📚 Documentation

- [Architecture Overview](docs/ARCHITECTURE.md)
- [API Documentation](docs/API.md)
- [Format Support Matrix](docs/FORMATS.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Secure by Design Guidance](docs/SECURE_BY_DESIGN_GUIDANCE.md)
- [GUI Usage Guide](docs/GUI_USAGE_GUIDE.md) - Complete GUI documentation
- [Batch Processing Guide](docs/BATCH_PROCESSING_GUIDE.md) - Convert multiple files (v0.2.2)
- [Settings Guide](docs/SETTINGS_GUIDE.md) - Configuration and preferences (v0.2.2)

## 🐛 Known Limitations

- **STEP Format (v0.2.0):** Feature-gated (`--features step`), read-only, **FACETED_BREP only** (pre-tessellated geometry). Full B-Rep support with curved surfaces (NURBS, cylinders, spheres) planned for v0.3.0. See `docs/CAD_EXPORT_GUIDE.md` for export instructions.
- **FBX Format:** Not supported (proprietary, no open-source Rust library)
- **DWG Format:** Not supported (proprietary)
- **SVG Format:** Read-only (rasterization to bitmap), no SVG export
- **Large Files:** Files >100MB may require resource limit adjustments (configurable)

## 🔮 Release Roadmap

- [x] **v0.1.0** - Core converters (MVP) ✅ **RELEASED**
  - All Tier 1 & Tier 2 image formats (PNG, JPEG, BMP, GIF, TIFF, WebP, SVG read)
  - All core mesh formats (STL, OBJ, PLY, OFF, glTF, DXF)
  - Comprehensive test coverage
  - Production-ready security posture
- [x] **v0.1.1** - Feature completion ✅ **RELEASED**
  - mesh-convert transform, recalculate-normals, validate features
  - Enhanced mesh manipulation utilities
- [x] **v0.2.0** - STEP/CAD support ✅ **RELEASED** (December 29, 2025)
  - STEP format support (FACETED_BREP extraction)
  - Feature-gated STEP support (`--features step`)
  - Comprehensive STEP documentation
  - 192 tests passing (all test suites)
- [x] **v0.3.0** - Advanced Features ✅ **RELEASED** (December 30, 2025)
  - ✅ Parallel batch processing (concurrent file conversion) - **IMPLEMENTED**
    - Thread pool using `rayon` library
    - 4x speedup on 4-core systems
    - Configurable concurrency limits (1-16)
    - Thread-safe queue management
  - ✅ Settings auto-save on change - **IMPLEMENTED**
  - ✅ Queue item editing - **IMPLEMENTED**
  - ✅ Pause/resume/cancel controls (Sprint 10 - complete)
  - 🟡 Full STEP B-Rep support (opencascade-rs integration - documentation complete, implementation planned)
  - ✅ 3D mesh viewer (full implementation - Sprint 10_A - complete)
- [x] **v0.2.1** - GUI release ✅ **RELEASED** (December 30, 2025)
  - Graphical user interface with egui framework
  - Drag-and-drop file support
  - Visual format selection
  - Direct library integration
  - User-friendly error messages
- [x] **v0.2.2** - GUI enhancements ✅ **RELEASED** (December 30, 2025)
  - Batch processing (convert multiple files at once)
  - Preview functionality (image/mesh preview before conversion)
  - Settings persistence (save user preferences)
  - Conversion history (track recent conversions)
- [ ] **v1.0.0** - Public release (Sprint 9-12)
- [ ] **v1.1.0** - Batch processing improvements
- [ ] **v1.2.0** - Plugin system for custom formats

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

**Libraries Used:**
- [image](https://github.com/image-rs/image) - Image processing
- [stl_io](https://github.com/hmeyer/stl_io) - STL file handling
- [tobj](https://github.com/Twinklebear/tobj) - OBJ file handling
- [truck](https://github.com/ricosjp/truck) - STEP support
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [nalgebra](https://github.com/dimforge/nalgebra) - Linear algebra

**Inspired by:**
- ImageMagick (comprehensive image conversion)
- FreeCAD (open-source CAD)
- Assimp (asset import library)

## 📞 Contact

For questions or issues during private development phase, contact the repository owner.

---

**Last Updated:** December 30, 2025  
**Status:** ✅ v0.2.1 Released (December 30, 2025) - GUI application available! Drag-and-drop interface for easy file conversion.

**Note:** v0.2.0 includes STEP format support (FACETED_BREP only). Full B-Rep support with curved surfaces planned for v0.3.0. See Release Roadmap section above.
