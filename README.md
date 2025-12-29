# Simple Image Converter

A high-performance, pure Rust command-line toolkit for converting between image and 3D mesh formats.

## 🎯 Project Overview

Two separate CLI tools written in Rust:
- **img-convert** - 2D image format converter
- **mesh-convert** - 3D mesh and CAD format converter

**Status:** ✅ Active Development (Private Repository)

**Current Version:** 0.1.0 (Ready for Release)

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
- **🚧 Partial Support:** STEP (feature-gated, read-only, **FACETED_BREP only** - pre-tessellated geometry)
- **📅 Planned Features:** Coordinate system transforms, normal recalculation, mesh validation
- **⚠️ STEP Limitations:** Only supports FACETED_BREP entities. Full B-Rep support (NURBS, cylinders, etc.) planned for v0.3.0. See `docs/CAD_EXPORT_GUIDE.md` for export instructions.
- Material preservation (where supported)
- Resource limits and security validation
- Binary and ASCII format variants

## 🚀 Quick Start

### Installation

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

# Binary to ASCII STL
./mesh-convert model.stl stl --format-variant ascii

# Note: Transform, recalculate-normals, and validate features are planned for v0.1.1
# Currently these options show "not yet implemented" warnings
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
└── converter-gui/       # **FUTURE:** GUI (Phase 4, not yet implemented)
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

### 🚧 Current Phase

**Sprint 6: Quality & Testing** 🚧 IN PROGRESS
- [x] Test coverage (365+ tests, excellent coverage)
- [x] Code quality (no clippy warnings)
- [x] Security posture (zero unsafe code)
- [ ] Documentation updates (in progress)
- [ ] CLI integration tests (planned for v0.1.1)
- [ ] mesh-convert advanced features (planned for v0.1.1)

### 📅 Planned Phases

**Sprint 7-8: STEP + CAD** 📅 PLANNED
- [ ] Complete STEP tessellation
- [ ] STEP read/write testing
- [ ] CAD-specific validations

**Sprint 9-12: GUI** 📅 **FUTURE** (Planned for v1.0.0)
- [ ] egui framework setup
- [ ] Drag-and-drop interface
- [ ] Batch processing
- [ ] Settings panel

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
- ✅ 365+ tests total covering all format implementations
- ✅ Unit tests for all format readers/writers
- ✅ Integration tests for format conversions
- ✅ Security tests for format spoofing and malformed input
- ⚠️ CLI integration tests (**FUTURE:** planned for v0.1.1)
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

## 🐛 Known Limitations

- **STEP Format:** Feature-gated (`--features step`), read-only, tessellation in progress (blocked by library limitation - truck-stepio v0.3.0 input API not yet available)
- **mesh-convert Advanced Features:** Transform, recalculate-normals, and validate options show "not yet implemented" warnings (**FUTURE:** planned for v0.1.1)
- **FBX Format:** Not supported (proprietary, no open-source Rust library)
- **DWG Format:** Not supported (proprietary)
- **SVG Format:** Read-only (rasterization to bitmap), no SVG export
- **Large Files:** Files >100MB may require resource limit adjustments (configurable)

## 🔮 Release Roadmap

- [x] **v0.1.0** - Core converters (MVP) ✅ **READY FOR RELEASE**
  - All Tier 1 & Tier 2 image formats (PNG, JPEG, BMP, GIF, TIFF, WebP, SVG read)
  - All core mesh formats (STL, OBJ, PLY, OFF, glTF, DXF)
  - Comprehensive test coverage (365+ tests)
  - Production-ready security posture
- [ ] **v0.1.1** - Feature completion (Planned: 2-3 weeks)
  - mesh-convert transform, recalculate-normals, validate features
  - CLI integration tests
  - Bug fixes and improvements
- [ ] **v0.2.0** - STEP/CAD support (Planned: 4-6 weeks)
  - Complete STEP format support
  - Additional format improvements
- [ ] **v0.3.0** - Performance optimizations
- [ ] **v1.0.0** - GUI release (Sprint 9-12)
- [ ] **v1.1.0** - Batch processing improvements
- [ ] **v1.2.0** - Plugin system for custom formats

## 📄 License

MIT License - See [LICENSE](LICENSE) file for details.

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

**Last Updated:** January 27, 2025  
**Status:** ✅ v0.1.0 Ready for Release - All core features implemented and tested (365+ tests passing)

**Note:** Future features (GUI, additional formats, mesh-convert advanced options) are planned for subsequent releases. See Release Roadmap section above.
