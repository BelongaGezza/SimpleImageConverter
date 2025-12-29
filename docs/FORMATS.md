# Format Support Matrix
## Simple Image Converter

**Last Updated:** January 27, 2025  
**Status:** v0.1.0 - Core formats complete, STEP partial (feature-gated, blocked by library limitation)

---

## 2D Image Formats

| Format | Extension | Read | Write | Status | Notes |
|--------|-----------|------|-------|--------|-------|
| PNG | .png | ✅ | ✅ | Sprint 2 | Full support |
| JPEG | .jpg, .jpeg | ✅ | ✅ | Sprint 2 | Quality control |
| BMP | .bmp | ✅ | ✅ | Sprint 2 ✅ | Windows bitmap |
| GIF | .gif | ✅ | ✅ | Sprint 2 ✅ | First frame only (animated) |
| TIFF | .tiff, .tif | ✅ | ✅ | Sprint 4 ✅ | Multi-page |
| WebP | .webp | ✅ | ✅ | Sprint 4 ✅ | Lossy/lossless |
| SVG | .svg | ✅ | ❌ | Sprint 4 ✅ | Rasterize only (read-only) |
| TGA | .tga | 📅 | 📅 | **FUTURE** | Tier 2 - Planned for future release |
| ICO | .ico | 📅 | 📅 | **FUTURE** | Tier 2 - Planned for future release |
| DDS | .dds | 📅 | 📅 | **FUTURE** | Optional - Planned for future release |
| HDR | .hdr | 📅 | 📅 | **FUTURE** | Optional - Planned for future release |
| OpenEXR | .exr | 📅 | 📅 | **FUTURE** | Tier 2 - Planned for future release |
| AVIF | .avif | 📅 | 📅 | **FUTURE** | Tier 3 - Planned for future release |
| PDF | .pdf | 📅 | ❌ | **FUTURE** | Page to image - Planned for future release |

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
| STEP | .step, .stp | 🚧 | ❌ | Sprint 7-8 🚧 | **v0.2.0 IN PROGRESS:** Read-only, feature-gated (`--features step`). STEP file parsing working (ruststep 0.4.0). Entity extraction framework complete. Entity conversion to truck Shell in progress. Tessellation pending. See `STEP_IMPLEMENTATION_CURRENT_STATE.md` and `docs/STEP_FORMAT_REFERENCE.md` for details. |

**Legend:**
- ✅ Implemented
- 🚧 In Progress
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
| Coordinate Transforms | N/A | **FUTURE:** Planned for v0.1.1 |
| Normal Recalculation | N/A | **FUTURE:** Planned for v0.1.1 |
| Mesh Validation | N/A | **FUTURE:** Planned for v0.1.1 |

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

### Sprint 4 (Advanced 2D Formats) ✅
- ✅ TIFF format
- ✅ WebP format
- ✅ SVG format (rasterization, read-only)

### Sprint 5 (Advanced 3D Formats) ✅
- ✅ OFF format
- ✅ glTF format (binary & text)
- ✅ DXF format (3D entities)

### Sprint 7-8 (STEP Format) 🚧
- ✅ STEP format skeleton implemented
- ✅ STEP file parsing working (ruststep 0.4.0 with AP203 feature)
- ✅ Entity extraction framework complete
- 🚧 STEP entity → truck Shell conversion in progress
- ⏳ Tessellation pending (requires Shell conversion first)
- ✅ Format registry updated with STEP support
- ✅ Feature flag system in place
- 📋 Research documentation complete (see `RESEARCH_*.md` files)

---

## Notes

- **SVG**: Read-only (rasterization), no write support
- **PDF**: Read-only (page extraction), no write support
- **STEP**: **v0.2.0 IN PROGRESS** - Skeleton implemented (Sprint 7-8). STEP file parsing working (ruststep 0.4.0). Entity extraction framework complete. Entity conversion to truck Shell in progress. Tessellation pending. Write support not planned (requires complex CAD modeling). Feature-gated (`--features step`). See `STEP_IMPLEMENTATION_CURRENT_STATE.md`, `docs/STEP_FORMAT_REFERENCE.md` (comprehensive specification reference), and `RESEARCH_*.md` files for details.
- **FBX**: Not supported (proprietary, no open-source Rust library)
- **DWG**: Not supported (proprietary)

---

_For implementation details, see Phase3_Architecture.md_  
_For sprint planning, see IMPLEMENTATION_PLAN.md_

