# Format Support Matrix
## Simple Image Converter

**Last Updated:** January 27, 2025 (Sprint 5 Complete)

---

## 2D Image Formats

| Format | Extension | Read | Write | Status | Notes |
|--------|-----------|------|-------|--------|-------|
| PNG | .png | ✅ | ✅ | Sprint 2 | Full support |
| JPEG | .jpg, .jpeg | ✅ | ✅ | Sprint 2 | Quality control |
| BMP | .bmp | ✅ | ✅ | Sprint 2 ✅ | Windows bitmap |
| GIF | .gif | ✅ | ✅ | Sprint 2 ✅ | First frame only (animated) |
| TIFF | .tiff, .tif | 📅 | 📅 | Sprint 4 | Multi-page |
| WebP | .webp | 📅 | 📅 | Sprint 4 | Lossy/lossless |
| SVG | .svg | 📅 | ❌ | Sprint 4 | Rasterize only |
| TGA | .tga | 📅 | 📅 | Sprint 4 | Tier 2 |
| ICO | .ico | 📅 | 📅 | Sprint 4 | Tier 2 |
| DDS | .dds | 📅 | 📅 | Sprint 4 | Optional |
| HDR | .hdr | 📅 | 📅 | Sprint 4 | Optional |
| OpenEXR | .exr | 📅 | 📅 | Sprint 4 | Tier 2 |
| AVIF | .avif | 📅 | 📅 | Sprint 4 | Tier 3 |
| PDF | .pdf | 📅 | ❌ | Sprint 4 | Page to image |

**Legend:**
- ✅ Implemented
- 📅 Planned
- ❌ Not supported

---

## 3D Mesh Formats

| Format | Extension | Read | Write | Status | Notes |
|--------|-----------|------|-------|--------|-------|
| STL | .stl | ✅ | ✅ | Sprint 3 ✅ | Binary/ASCII |
| OBJ | .obj | ✅ | ✅ | Sprint 3 ✅ | With materials |
| PLY | .ply | ✅ | ✅ | Sprint 3 ✅ | ASCII format |
| OFF | .off | ✅ | ✅ | Sprint 5 ✅ | Custom parser |
| glTF | .gltf, .glb | ✅ | ✅ | Sprint 5 ✅ | Binary/text |
| DXF | .dxf | ✅ | ✅ | Sprint 5 ✅ | 3D entities |
| STEP | .step, .stp | 📅 | 📅 | Sprint 7-8 | Via truck |

**Legend:**
- ✅ Implemented
- 📅 Planned
- ❌ Not supported

---

## Format Detection

Formats are detected by:

1. **File Extension** (primary method)
2. **Magic Bytes** (fallback for unknown extensions)

### Magic Byte Signatures

| Format | Magic Bytes |
|--------|-------------|
| PNG | `89 50 4E 47 0D 0A 1A 0A` |
| JPEG | `FF D8 FF` |
| GIF | `47 49 46 38` |
| BMP | `42 4D` |
| STL (ASCII) | `solid` (first 5 bytes) |
| STL (Binary) | 80-byte header |

---

## Quality Settings

### Image Quality

- **Range**: 0-100
- **Default**: 90
- **Formats**: JPEG, WebP, AVIF

### Compression

- **Range**: 0-9 (format-dependent)
- **Default**: 6
- **Formats**: PNG, TIFF

---

## Feature Matrix

| Feature | 2D Formats | 3D Formats |
|---------|------------|------------|
| Transparency | PNG, GIF, WebP | N/A |
| Animation | GIF | N/A |
| Multi-page | TIFF, PDF | N/A |
| Materials | N/A | OBJ, glTF |
| Textures | N/A | OBJ, glTF |
| Coordinate Transforms | N/A | All formats |
| Normal Recalculation | N/A | All formats |
| Mesh Validation | N/A | All formats |

---

## Implementation Status

### Sprint 1 (Foundation) ✅
- Workspace structure
- Trait definitions
- Basic CLI skeletons

### Sprint 2 (Image Core) ✅
- ✅ PNG format
- ✅ JPEG format
- ✅ BMP format
- ✅ GIF format

### Sprint 3 (Mesh Core) ✅
- ✅ STL format
- ✅ OBJ format
- ✅ PLY format
- ✅ mesh-convert CLI integrated

### Sprint 5 (Advanced 3D Formats) ✅
- ✅ OFF format
- ✅ glTF format (binary & text)
- ✅ DXF format (3D entities)

### Sprint 4+ (Advanced) 📅
- See IMPLEMENTATION_PLAN.md

---

## Notes

- **SVG**: Read-only (rasterization), no write support
- **PDF**: Read-only (page extraction), no write support
- **STEP**: Read-only initially, write support evaluated in Sprint 8
- **FBX**: Not supported (proprietary, no open-source Rust library)
- **DWG**: Not supported (proprietary)

---

_For implementation details, see Phase3_Architecture.md_  
_For sprint planning, see IMPLEMENTATION_PLAN.md_

