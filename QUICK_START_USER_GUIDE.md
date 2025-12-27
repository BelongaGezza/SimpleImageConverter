# Quick Start User Guide

**Simple Image Converter** - A fast, secure command-line tool for converting between image and 3D mesh formats.

---

## 📦 Installation

### Prerequisites

- **Rust 1.92+** installed ([rustup.rs](https://rustup.rs/))
- **Windows 11** (primary platform) or compatible OS
- **100MB+ free disk space** for builds

### Build from Source

```bash
# Clone the repository
git clone https://github.com/BelongaGezza/SimpleImageConverter.git
cd SimpleImageConverter

# Build optimized release binaries
cargo build --release

# Binaries will be in target/release/
# - img-convert.exe (or img-convert on Linux/Mac)
# - mesh-convert.exe (or mesh-convert on Linux/Mac)
```

### Add to PATH (Optional)

**Windows:**
```powershell
# Add to your PATH environment variable
$env:PATH += ";C:\path\to\SimpleImageConverter\target\release"
```

**Linux/Mac:**
```bash
# Create symlinks or add to PATH
sudo ln -s /path/to/SimpleImageConverter/target/release/img-convert /usr/local/bin/
sudo ln -s /path/to/SimpleImageConverter/target/release/mesh-convert /usr/local/bin/
```

---

## 🖼️ Image Converter (img-convert)

### Basic Usage

```bash
# Convert PNG to JPEG
img-convert photo.png jpg

# Convert with custom output filename
img-convert image.bmp png --output result.png

# Convert with quality control (1-100, default: 90)
img-convert photo.jpg jpg --quality 95
```

### Supported Formats

**✅ Fully Supported:**
- **PNG** - Portable Network Graphics
- **JPEG/JPG** - Joint Photographic Experts Group
- **BMP** - Windows Bitmap
- **GIF** - Graphics Interchange Format
- **TIFF/TIF** - Tagged Image File Format
- **WebP** - Google WebP format
- **SVG** - Scalable Vector Graphics (read-only, rasterized to bitmap)

### Common Examples

```bash
# Convert photo to high-quality JPEG
img-convert vacation.png jpg --quality 95

# Convert GIF to PNG (preserves transparency)
img-convert animation.gif png

# Rasterize SVG at 300 DPI
img-convert logo.svg png --dpi 300

# Convert TIFF to WebP with compression
img-convert scan.tiff webp --quality 85

# Batch conversion (using shell loops)
for file in *.bmp; do
    img-convert "$file" png
done
```

### Advanced Options

```bash
# Increase maximum file size limit (default: 100MB)
img-convert large_image.tiff jpg --max-file-size-mb 500

# Increase maximum image dimension (default: 65535 pixels)
img-convert huge_image.png jpg --max-dimension 100000
```

---

## 🎲 3D Mesh Converter (mesh-convert)

### Basic Usage

```bash
# Convert STL to OBJ
mesh-convert model.stl obj

# Convert OBJ to PLY
mesh-convert model.obj ply

# Convert with custom output filename
mesh-convert model.stl obj --output result.obj
```

### Supported Formats

**✅ Fully Supported:**
- **STL** - Stereolithography (binary and ASCII)
- **OBJ** - Wavefront OBJ
- **PLY** - Polygon File Format
- **OFF** - Object File Format
- **glTF/GLB** - GL Transmission Format
- **DXF** - Drawing Exchange Format

**🚧 Partial Support:**
- **STEP** - Standard for Exchange of Product Data (read-only, requires `--features step`, tessellation in progress - blocked by library limitation)

### Format Variants

```bash
# Convert STL to ASCII STL
mesh-convert model.stl stl --format-variant ascii

# Convert STL to binary STL
mesh-convert model.stl stl --format-variant binary
```

### Common Examples

```bash
# Convert 3D scan to OBJ
mesh-convert scan.stl obj

# Convert CAD model to PLY
mesh-convert model.dxf ply

# Convert glTF to STL
mesh-convert model.gltf stl

# Convert OBJ to binary STL
mesh-convert model.obj stl --format-variant binary
```

### Advanced Options

```bash
# Increase resource limits for large models
mesh-convert large_model.stl obj \
    --max-file-size-mb 500 \
    --max-vertices 50000000 \
    --max-faces 50000000

# Note: Transform, recalculate-normals, and validate options
# are FUTURE features planned for v0.1.1 (currently show warnings)
```

---

## 🔧 Troubleshooting

### Common Issues

#### "File not found"
```bash
# Use absolute paths or ensure you're in the correct directory
img-convert C:\Users\YourName\Pictures\photo.png jpg
```

#### "Format not supported"
- Check the supported formats list above
- Ensure the file extension matches the actual format
- Some formats may require feature flags (e.g., STEP)

#### "File too large"
```bash
# Increase the file size limit
img-convert large.png jpg --max-file-size-mb 500
```

#### "Quality must be between 1 and 100"
- Quality values must be 1-100 (default: 90)
- Lower values = smaller files, lower quality
- Higher values = larger files, better quality

#### "Output file validation failed"
- This warning indicates the output file may be corrupted
- Try converting again or use a different output format
- Check that you have write permissions in the output directory

### Getting Help

```bash
# View help for img-convert
img-convert --help

# View help for mesh-convert
mesh-convert --help
```

---

## 📋 Quick Reference

### Image Converter Command Structure

```bash
img-convert <input> <format> [OPTIONS]

Options:
  -o, --output <OUTPUT>        Output file path
  -q, --quality <QUALITY>       Quality (1-100, default: 90)
  --max-file-size-mb <SIZE>     Max file size in MB (default: 100)
  --max-dimension <DIM>         Max image dimension in pixels (default: 65535)
  -h, --help                    Print help
```

### Mesh Converter Command Structure

```bash
mesh-convert <input> <format> [OPTIONS]

Options:
  -o, --output <OUTPUT>         Output file path
  --format-variant <VARIANT>    Format variant (ascii/binary)
  --max-file-size-mb <SIZE>     Max file size in MB (default: 100)
  --max-vertices <COUNT>        Max vertices (default: 10,000,000)
  --max-faces <COUNT>           Max faces (default: 10,000,000)
  -h, --help                    Print help
```

---

## 💡 Tips & Best Practices

### Image Conversion

1. **Quality Settings:**
   - **95-100**: Best quality, large files (photos, prints)
   - **85-94**: High quality, good balance (web images)
   - **70-84**: Medium quality, smaller files (thumbnails)
   - **50-69**: Lower quality, very small files (previews)

2. **Format Selection:**
   - **PNG**: Best for images with transparency or sharp edges
   - **JPEG**: Best for photos (smaller file sizes)
   - **WebP**: Modern format, excellent compression
   - **TIFF**: Best for archival or professional use

3. **SVG Rasterization:**
   - SVG files are converted to bitmap images
   - Use `--dpi` to control output resolution
   - Higher DPI = larger files, better quality

### Mesh Conversion

1. **Format Selection:**
   - **STL**: Simple, widely supported (3D printing)
   - **OBJ**: Supports materials and textures
   - **PLY**: Good for 3D scans
   - **glTF**: Modern web format, supports animations

2. **Binary vs ASCII:**
   - **Binary**: Smaller files, faster processing
   - **ASCII**: Human-readable, easier to debug

3. **Large Models:**
   - Increase resource limits for very large files
   - Consider splitting models if conversion fails
   - Binary formats are more efficient for large models

---

## 🚀 Next Steps

- Read the full [README.md](README.md) for detailed information
- Check [docs/FORMATS.md](docs/FORMATS.md) for format-specific details
- Review [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for technical details
- See [CONTRIBUTING.md](CONTRIBUTING.md) if you want to contribute

---

## 📞 Support

For issues or questions:
- Check the troubleshooting section above
- Review the project documentation
- Contact the repository maintainer

---

**Version:** 0.1.0  
**Last Updated:** January 2025  
**Status:** ✅ Ready for Use

