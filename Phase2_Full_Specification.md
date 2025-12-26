# Phase 2: Full Specification - Rust Implementation
## Image and 3D Mesh Converters

**Date:** December 26, 2025  
**Language:** Rust 1.92.0  
**Target:** x86-64 Windows 11  
**License:** Open Source Only

---

## Executive Summary

Two separate command-line tools written in Rust:
1. **img-convert.exe** - 2D image format converter
2. **mesh-convert.exe** - 3D mesh and CAD format converter

Both tools share common architecture patterns but use domain-specific libraries.

---

## 1. IMAGE CONVERTER (img-convert)

### 1.1 Supported Format Matrix

#### Tier 1: Core Formats (PoC Validated)
| From/To | PNG | JPG | BMP | GIF | TIFF | WebP |
|---------|-----|-----|-----|-----|------|------|
| **PNG** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **JPG** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **BMP** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **GIF** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **TIFF** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **WebP** | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

**Library:** `image` crate v0.25+
- All formats fully supported
- High-quality encoders/decoders
- Production-ready

#### Tier 2: Advanced Raster Formats
| Format | Read | Write | Notes | Library |
|--------|------|-------|-------|---------|
| **TGA** | ✓ | ✓ | Targa | `image` crate |
| **ICO** | ✓ | ✓ | Windows Icon | `image` crate |
| **DDS** | ✓ | ✓ | DirectDraw Surface | `image` crate |
| **HDR** | ✓ | ✓ | Radiance HDR | `image` crate |
| **OpenEXR** | ✓ | ✓ | High dynamic range | `exr` crate |
| **AVIF** | ✓ | ✓ | Next-gen format | `ravif` crate |
| **JPEG XL** | ⚠️ | ⚠️ | Experimental | `jxl-oxide` (if needed) |

#### Tier 3: Vector Formats (Rasterization Only)
| Format | Input | Output | Notes | Library |
|--------|-------|--------|-------|---------|
| **SVG** | ✓ (rasterize) | ✗ | Convert to raster | `resvg` crate |
| **PDF** | ✓ (rasterize) | ✗ | Page → image | `pdfium-render` or `pdf` crate |

**Limitations:**
- Vector formats can only be *read* (rasterized to bitmap)
- Cannot write SVG/PDF (would require separate vector library)
- Resolution must be specified for rasterization (default: 300 DPI)

#### Format Compatibility Matrix

**Transparency Support:**
- ✓ Full: PNG, WebP, GIF, TIFF, TGA, ICO, AVIF
- ✗ None: JPG, BMP

**Color Depth:**
- 8-bit: All formats
- 16-bit: PNG, TIFF, OpenEXR
- 32-bit float: OpenEXR, HDR

**Color Modes:**
- RGB/RGBA: All formats
- Grayscale: All formats
- CMYK: TIFF (limited)
- Indexed/Palette: GIF, PNG, BMP

### 1.2 Rust Dependencies (2D)

```toml
[dependencies]
# Core image processing
image = "0.25"              # Main image library (PNG, JPG, BMP, GIF, TIFF, WebP, etc.)
imageproc = "0.25"          # Image processing utilities

# Advanced formats
webp = "0.3"                # WebP codec (already in `image`)
ravif = "0.11"              # AVIF encoding
libavif = "0.14"            # AVIF decoding
exr = "1.72"                # OpenEXR support

# Vector rasterization (Tier 3)
resvg = "0.44"              # SVG rasterization
usvg = "0.44"               # SVG parsing (dependency of resvg)

# Utilities
anyhow = "1.0"              # Error handling
clap = { version = "4.5", features = ["derive"] }  # CLI parsing
indicatif = "0.17"          # Progress bars (optional)
```

**Total Dependency Count:** ~8-10 crates + transitive dependencies
**Estimated Binary Size:** 3-5 MB (release build, stripped)

### 1.3 Architecture (2D)

```
img-convert/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point, argument parsing
│   ├── lib.rs               # Public library interface
│   ├── formats/
│   │   ├── mod.rs           # Format detection and routing
│   │   ├── raster.rs        # Raster format conversions (PNG, JPG, etc.)
│   │   ├── vector.rs        # Vector rasterization (SVG, PDF)
│   │   └── metadata.rs      # Format metadata and validation
│   ├── convert.rs           # Core conversion logic
│   ├── quality.rs           # Quality/compression settings
│   ├── color.rs             # Color space conversions
│   └── error.rs             # Error types
└── tests/
    ├── integration_tests.rs
    └── test_data/           # Test images
```

### 1.4 Command-Line Interface (2D)

```bash
# Basic usage
img-convert source.png jpg

# With quality control
img-convert source.png jpg --quality 95

# Specify output filename
img-convert source.png jpg --output result.jpg

# Rasterize SVG with DPI
img-convert logo.svg png --dpi 300

# Batch mode (future)
img-convert *.png jpg --quality 90
```

**Arguments:**
- `<SOURCE>`: Input file path
- `<FORMAT>`: Target format extension (png, jpg, bmp, etc.)
- `--output, -o`: Custom output path (optional, default: same name with new extension)
- `--quality, -q`: Quality level 1-100 for lossy formats (default: 95)
- `--dpi, -d`: DPI for vector rasterization (default: 300)
- `--compression, -c`: Compression level for PNG (0-9, default: 6)
- `--strip-metadata`: Remove EXIF/metadata (default: preserve)
- `--help, -h`: Show help
- `--version, -V`: Show version

### 1.5 Quality Settings (2D)

**JPEG:**
- Quality: 1-100 (default: 95 for high quality)
- Optimize: true (Huffman optimization)

**PNG:**
- Compression: 0-9 (default: 6 = balanced)
- Filter: Auto-select optimal

**WebP:**
- Lossy quality: 1-100 (default: 95)
- Lossless: flag option

**GIF:**
- No quality setting (lossless compression)

**TIFF:**
- Compression: None, LZW, Deflate (default: LZW)

---

## 2. 3D MESH CONVERTER (mesh-convert)

### 2.1 Supported Format Matrix

#### Tier 1: Mesh Formats (High Priority)
| Format | Extension | Read | Write | Notes | Library |
|--------|-----------|------|-------|-------|---------|
| **STL** | .stl | ✓ | ✓ | Binary & ASCII | `stl_io` |
| **OBJ** | .obj | ✓ | ✓ | Wavefront | `tobj` or `obj-rs` |
| **PLY** | .ply | ✓ | ✓ | Stanford | `ply-rs` |
| **OFF** | .off | ✓ | ✓ | Object File Format | Custom parser |
| **3MF** | .3mf | ⚠️ | ⚠️ | 3D Manufacturing | `threemf` (if available) |

#### Tier 2: Scene/Animation Formats
| Format | Extension | Read | Write | Notes | Library |
|--------|-----------|------|-------|-------|---------|
| **glTF** | .gltf/.glb | ✓ | ✓ | GL Transmission | `gltf` crate |
| **FBX** | .fbx | ✗ | ✗ | Proprietary (Autodesk) | **NOT AVAILABLE** |
| **COLLADA** | .dae | ⚠️ | ⚠️ | XML-based | Limited support |

**FBX Reality Check:**
- No open-source Rust FBX library exists
- FBX is proprietary Autodesk format
- Options:
  1. Skip FBX support
  2. Use Assimp via FFI (see below)
  3. Write custom parser (significant effort)

#### Tier 3: CAD/Engineering Formats
| Format | Extension | Read | Write | Notes | Library |
|--------|-----------|------|-------|-------|---------|
| **DXF** | .dxf | ✓ | ✓ | AutoCAD Drawing Exchange | `dxf` crate |
| **STEP** | .step/.stp | ⚠️ | ⚠️ | ISO 10303 | **NEEDS EVALUATION** |
| **IGES** | .iges/.igs | ⚠️ | ⚠️ | Initial Graphics Exchange | **NEEDS EVALUATION** |
| **DWG** | .dwg | ✗ | ✗ | AutoCAD Native | **NOT AVAILABLE** |

**CAD Format Reality:**
- **DXF**: `dxf` crate (ixmilia port) - Good support
- **STEP/IGES**: Limited/experimental Rust support
- **DWG**: Proprietary, no open-source Rust library

**Options for CAD:**
1. **DXF only** (well-supported in Rust)
2. **FFI to Open CASCADE** (C++ library, complex integration)
3. **External tool wrapper** (call FreeCAD/OpenSCAD CLI)
4. **Phase 2 implementation** (write parsers if needed)

### 2.2 Conversion Matrix (Realistic)

#### Phase 1 (Immediate)
| From/To | STL | OBJ | PLY | OFF |
|---------|-----|-----|-----|-----|
| **STL** | ✓ | ✓ | ✓ | ✓ |
| **OBJ** | ✓ | ✓ | ✓ | ✓ |
| **PLY** | ✓ | ✓ | ✓ | ✓ |
| **OFF** | ✓ | ✓ | ✓ | ✓ |

**Libraries:** `stl_io`, `tobj`, `ply-rs`, custom OFF parser

#### Phase 2 (With Additional Work)
| Format | Status | Approach |
|--------|--------|----------|
| **glTF** | Add `gltf` crate | Straightforward |
| **DXF** | Add `dxf` crate | Straightforward |
| **3MF** | Evaluate `threemf` | May need contribution |
| **STEP** | FFI or skip | Complex |
| **IGES** | FFI or skip | Complex |
| **FBX** | FFI to Assimp | Complex |

### 2.3 Rust Dependencies (3D)

```toml
[dependencies]
# Mesh formats
stl_io = "0.7"              # STL (binary & ASCII)
tobj = "4.0"                # OBJ/MTL
ply-rs = "0.1"              # PLY
gltf = "1.4"                # glTF (optional, Phase 2)

# CAD formats
dxf = "0.6"                 # DXF (ixmilia port)

# Mesh processing
nalgebra = "0.33"           # Linear algebra (transforms, normals)
# or nalgebra-glm = "0.18"

# Utilities
anyhow = "1.0"              # Error handling
clap = { version = "4.5", features = ["derive"] }  # CLI
indicatif = "0.17"          # Progress bars

# Optional: FFI for advanced formats
# assimp-sys = "0.3"        # If we add Assimp FFI
```

**Total Dependency Count:** ~8-12 crates
**Estimated Binary Size:** 2-4 MB (release build, stripped)

### 2.4 Architecture (3D)

```
mesh-convert/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Public library interface
│   ├── formats/
│   │   ├── mod.rs           # Format detection and routing
│   │   ├── stl.rs           # STL reader/writer
│   │   ├── obj.rs           # OBJ reader/writer
│   │   ├── ply.rs           # PLY reader/writer
│   │   ├── off.rs           # OFF reader/writer (custom)
│   │   ├── gltf.rs          # glTF (Phase 2)
│   │   └── dxf.rs           # DXF (Phase 2)
│   ├── mesh/
│   │   ├── mod.rs           # Mesh data structures
│   │   ├── vertex.rs        # Vertex, normal, UV data
│   │   ├── triangle.rs      # Face/triangle data
│   │   └── transform.rs     # Coordinate transforms
│   ├── convert.rs           # Core conversion logic
│   ├── validate.rs          # Mesh validation (manifold, normals)
│   └── error.rs             # Error types
└── tests/
    ├── integration_tests.rs
    └── test_data/           # Test meshes
```

### 2.5 Command-Line Interface (3D)

```bash
# Basic usage
mesh-convert model.stl obj

# Specify output
mesh-convert model.stl obj --output result.obj

# Binary vs ASCII STL
mesh-convert model.stl stl --format binary
mesh-convert model.stl stl --format ascii

# Transform coordinates (Y-up to Z-up)
mesh-convert model.obj stl --transform z-up

# Validate mesh
mesh-convert model.stl obj --validate
```

**Arguments:**
- `<SOURCE>`: Input file path
- `<FORMAT>`: Target format (stl, obj, ply, off, gltf, dxf)
- `--output, -o`: Custom output path
- `--format, -f`: Binary/ASCII for formats that support both (STL, PLY)
- `--transform, -t`: Coordinate system transform (y-up, z-up, etc.)
- `--validate, -v`: Validate mesh integrity
- `--recalculate-normals, -n`: Recalculate vertex normals
- `--help, -h`: Show help
- `--version, -V`: Show version

### 2.6 Mesh Data Structure

```rust
pub struct Mesh {
    pub vertices: Vec<Vertex>,      // Vertex positions
    pub normals: Vec<Normal>,       // Vertex normals (optional)
    pub uvs: Vec<UV>,               // Texture coordinates (optional)
    pub faces: Vec<Face>,           // Triangle indices
    pub materials: Vec<Material>,   // Materials (OBJ, glTF)
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Face {
    pub v0: usize,  // Vertex index
    pub v1: usize,
    pub v2: usize,
    pub normal_indices: Option<(usize, usize, usize)>,
    pub uv_indices: Option<(usize, usize, usize)>,
    pub material_index: Option<usize>,
}
```

### 2.7 Format-Specific Considerations

**STL:**
- Binary vs ASCII detection (auto or manual)
- Normal recalculation (STL includes face normals)
- No color/texture support

**OBJ:**
- Material (.mtl) file handling
- Texture coordinates
- Multiple objects per file

**PLY:**
- ASCII vs binary
- Custom properties
- Color per-vertex support

**glTF:**
- Embedded vs external resources
- Animation data (ignored in conversion)
- PBR materials

**DXF:**
- 2D entities (ignore or error)
- Layers and blocks
- Units handling

---

## 3. MISSING LIBRARY SOLUTIONS

### 3.1 Assimp FFI Approach

If we need FBX, STEP, IGES, or other formats not available in Rust:

**Option A: Direct FFI to Assimp**

```toml
[dependencies]
assimp-sys = "0.3"  # Low-level bindings
# or
assimp = "0.1"      # Higher-level wrapper (if exists)
```

**Pros:**
- Access to 40+ formats
- Battle-tested library
- Community support

**Cons:**
- Requires C++ runtime
- Larger binary size (~10-15MB)
- Platform-specific builds
- More complex deployment

**Implementation Effort:** ~1-2 weeks for integration

### 3.2 Custom Parser Approach

For formats with clear specifications but no Rust library:

**Candidates:**
- OFF (simple, can write in a day)
- 3MF (ZIP-based XML, moderate complexity)
- STEP/IGES (complex, 2-4 weeks each)

**Strategy:**
- Start with OFF (easy win)
- Evaluate 3MF need
- Defer STEP/IGES unless critical

### 3.3 External Tool Wrapper

For ultra-complex formats:

```bash
# Example: Use FreeCAD for STEP conversion
mesh-convert model.step stl --engine freecad
```

Call external tools via `std::process::Command`:
- FreeCAD (STEP, IGES → STL)
- OpenSCAD (various → STL)
- Meshlab (via meshlabserver)

**Pros:**
- Leverage existing tools
- No FFI complexity
- Quick implementation

**Cons:**
- Requires external dependencies
- Slower (process spawn overhead)
- Less integrated

---

## 4. BUILD AND DEPLOYMENT

### 4.1 Cross-Compilation to Windows

From Linux development environment to Windows x86-64:

```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW cross-compiler
sudo apt-get install mingw-w64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

**Alternative: Build on Windows**
- Install Rust via rustup.rs
- Use MSVC toolchain (recommended for Windows)
- Native compilation

### 4.2 Binary Optimization

**Cargo.toml:**
```toml
[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
strip = true           # Strip symbols
panic = "abort"        # Smaller panic handler
```

**Expected sizes:**
- img-convert.exe: ~3-5 MB
- mesh-convert.exe: ~2-4 MB
- With Assimp FFI: ~10-15 MB

### 4.3 Distribution

**Single executable per tool:**
- No runtime dependencies (statically linked)
- No DLLs needed (except Windows system DLLs)
- Copy-and-run deployment

**Optional: Installer**
- NSIS/WiX installer (future)
- Add to PATH option
- File association registration

---

## 5. GUI ROADMAP

### 5.1 Technology Options for Rust GUI

**Option A: egui (Recommended)**
```toml
[dependencies]
eframe = "0.29"   # egui framework
```
- Immediate mode GUI
- Cross-platform
- Lightweight (~2MB overhead)
- Good for utility apps

**Option B: iced**
- Elm-inspired architecture
- Prettier, more complex
- ~5MB overhead

**Option C: Tauri**
- Web tech (HTML/CSS/JS) + Rust backend
- Professional look
- Larger binary (~15-20MB)
- Best for complex UIs

**Option D: Slint**
- Declarative UI
- Native look and feel
- Still maturing

### 5.2 GUI Architecture

```
converter-gui/
├── Cargo.toml
├── src/
│   ├── main.rs           # GUI entry point
│   ├── app.rs            # Application state
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── image_tab.rs  # Image converter UI
│   │   └── mesh_tab.rs   # 3D converter UI
│   └── backend/
│       ├── img_convert.rs   # Calls img-convert lib
│       └── mesh_convert.rs  # Calls mesh-convert lib
```

**Strategy:**
1. Build CLI tools as libraries (`lib.rs`)
2. CLI binaries are thin wrappers
3. GUI imports the libraries directly
4. Shared code, different frontends

### 5.3 GUI Features

**Core:**
- Drag & drop file input
- Format dropdown selection
- Quality/options sliders
- Progress bar
- Batch conversion queue

**Advanced:**
- Preview (before/after)
- Settings persistence
- History/recent files
- Format information tooltips

**Phase 1 GUI:** ~2-3 weeks development
**Phase 2 GUI:** +2-4 weeks for polish

---

## 6. TESTING STRATEGY

### 6.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_to_jpg() {
        let input = "test_data/sample.png";
        let result = convert(input, "jpg").unwrap();
        assert!(result.exists());
    }

    #[test]
    fn test_stl_binary_to_ascii() {
        let input = "test_data/cube.stl";
        let result = convert_stl(input, StlFormat::Ascii).unwrap();
        // Verify ASCII format
    }
}
```

### 6.2 Integration Tests

- Test each format pair conversion
- Validate output file integrity
- Check error handling
- Performance benchmarks

### 6.3 Test Data

**2D:**
- PNG: transparent, opaque, indexed, grayscale
- JPG: various qualities
- SVG: simple shapes, complex paths
- Large files (stress test)

**3D:**
- STL: binary, ASCII, manifold, non-manifold
- OBJ: with/without materials, textured
- PLY: binary, ASCII, colored vertices
- Large meshes (1M+ triangles)

---

## 7. DEVELOPMENT TIMELINE

### Phase 1: Core Converters (4-6 weeks)

**Week 1-2: img-convert**
- Setup project structure
- Implement Tier 1 formats (PNG, JPG, BMP, GIF, TIFF, WebP)
- CLI interface
- Error handling
- Unit tests

**Week 3-4: mesh-convert**
- Setup project structure
- Implement STL, OBJ, PLY
- CLI interface
- Mesh data structures
- Unit tests

**Week 5-6: Polish & Testing**
- Integration tests
- Performance optimization
- Documentation
- Windows cross-compilation
- Release v0.1.0

### Phase 2: Extended Formats (3-4 weeks)

**Week 7-8: Advanced 2D**
- SVG rasterization (resvg)
- AVIF support
- OpenEXR
- Quality presets

**Week 9-10: Advanced 3D**
- glTF support
- DXF support
- OFF format
- Coordinate transforms

**Week 11: Testing & Release**
- Extended test suite
- Bug fixes
- Release v0.2.0

### Phase 3: GUI (3-6 weeks)

**Week 12-14: GUI Core**
- Choose framework (egui recommended)
- Basic UI layout
- Integration with libs
- File dialogs

**Week 15-17: GUI Features**
- Drag & drop
- Batch processing
- Progress indicators
- Settings panel

**Week 18: Polish**
- Icons and branding
- Installer
- Release v1.0.0

**Total Timeline: 11-18 weeks** (depends on scope)

---

## 8. PRIORITY RECOMMENDATIONS

### Must-Have (Phase 1)
**2D:** PNG, JPG, BMP, GIF, TIFF, WebP
**3D:** STL, OBJ, PLY

### Should-Have (Phase 2)
**2D:** SVG (rasterize), AVIF, OpenEXR
**3D:** glTF, DXF

### Nice-to-Have (Future)
**2D:** PDF rasterization, JPEG XL
**3D:** 3MF, STEP (via FFI), FBX (via FFI)

### Skip (Not Worth Effort)
**3D:** DWG (proprietary), IGES (obsolete)

---

## 9. OPEN QUESTIONS

Before proceeding to Phase 3 (Architecture Design), please clarify:

1. **CAD Format Priority:**
   - Is DXF support essential? (easy)
   - Is STEP/IGES needed? (hard, may need FFI)
   - Can we skip FBX? (no good Rust option)

2. **Assimp FFI:**
   - Willing to add C++ dependency for broader format support?
   - Or stick to pure Rust (more limited formats)?

3. **Timeline:**
   - Need MVP quickly (4-6 weeks for core)?
   - Or comprehensive tool (12+ weeks for everything)?

4. **GUI Priority:**
   - GUI essential for v1.0?
   - Or CLI-first, GUI later?

5. **Custom Parsers:**
   - Open to writing STEP/IGES parsers if needed? (significant effort)
   - Or external tool wrappers acceptable?

---

## 10. NEXT STEPS

Once you approve this specification:

1. **Phase 3A: Detailed Architecture**
   - Class/module diagrams
   - Interface definitions
   - Error handling strategy
   - Configuration approach

2. **Phase 3B: Implementation**
   - Start with img-convert (easier)
   - Then mesh-convert
   - Iterative development

3. **Deliverables:**
   - Two separate executables
   - Comprehensive test suites
   - Documentation
   - Build scripts for Windows

**Ready to proceed? Please provide guidance on the Open Questions above.**
