# Simple Image Converter

A high-performance, pure Rust command-line toolkit for converting between image and 3D mesh formats.

## 🎯 Project Overview

Two separate CLI tools written in Rust:
- **img-convert** - 2D image format converter
- **mesh-convert** - 3D mesh and CAD format converter

**Status:** 🚧 In Development (Private Repository)

**Current Version:** 0.1.0-dev

## ✨ Features

### Image Converter (img-convert)
- **Tier 1 Formats:** PNG, JPEG, BMP, GIF, TIFF, WebP
- **Tier 2 Formats:** TGA, ICO, DDS, HDR, OpenEXR, AVIF
- **Tier 3 Formats:** SVG (rasterize to bitmap), PDF (page to image)
- High-quality conversion with configurable compression
- Transparency handling
- Metadata preservation options
- Fast processing with minimal memory footprint

### 3D Mesh Converter (mesh-convert)
- **Core Formats:** STL (binary/ASCII), OBJ, PLY, OFF
- **Scene Formats:** glTF/GLB
- **CAD Formats:** DXF, STEP (via truck)
- Coordinate system transforms (Y-up ↔ Z-up)
- Normal recalculation
- Mesh validation (manifold checking)
- Material preservation (where supported)

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

# With coordinate transform and normal recalculation
./mesh-convert model.obj stl --transform z-up --recalculate-normals

# Validate mesh during conversion
./mesh-convert mesh.ply obj --validate
```

## 📋 Requirements

- **Rust:** 1.70+ (MSRV)
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
└── converter-gui/       # Future GUI (Phase 4)
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
# Build with STEP support (default)
cargo build --features step-truck

# Build without STEP support
cargo build --no-default-features

# Future: OCCT fallback
# cargo build --features step-occt
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

### Phase 1: Core Converters (Weeks 1-6) - 🚧 In Progress
- [x] Project setup
- [x] Architecture design
- [ ] img-convert implementation
- [ ] mesh-convert implementation
- [ ] Basic testing

### Phase 2: Extended Formats (Weeks 7-12) - 📅 Planned
- [ ] Advanced 2D formats (SVG, AVIF, OpenEXR)
- [ ] Advanced 3D formats (glTF, DXF)
- [ ] Coordinate transforms
- [ ] Quality presets

### Phase 3: STEP + CAD (Weeks 13-16) - 📅 Planned
- [ ] truck STEP integration
- [ ] STEP read/write testing
- [ ] CAD-specific validations

### Phase 4: GUI (Weeks 17-23) - 📅 Future
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
- Unit tests for each module
- Integration tests for conversions
- CLI tests
- Performance benchmarks
- Real-world file testing

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
- [Implementation Plan](docs/IMPLEMENTATION_PLAN.md)
- [API Documentation](docs/API.md) (Coming Soon)
- [Format Support Matrix](docs/FORMATS.md) (Coming Soon)

## 🐛 Known Issues

- STEP export not fully implemented (import only in Phase 3)
- FBX format not supported (proprietary, no open-source Rust library)
- DWG format not supported (proprietary)
- Large file handling (>1GB) may require streaming optimizations

## 🔮 Future Roadmap

- [ ] **v0.1.0** - Core converters (MVP)
- [ ] **v0.2.0** - Extended formats
- [ ] **v0.3.0** - STEP/CAD support
- [ ] **v0.4.0** - Performance optimizations
- [ ] **v1.0.0** - GUI release
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

**Note:** This is a work in progress. Features and documentation will be updated as development progresses through the planned sprints.

**Last Updated:** December 26, 2025
