# Format Support Matrix
## Simple Image Converter

**Last Updated:** January 3, 2026
**Current Version:** v0.3.0 (v1.0.0 in preparation)
**Status:** All core formats implemented, STEP FACETED_BREP support, 3D viewer, parallel batch processing

---

## 2D Image Formats

| Format | Extension | Read | Write | Notes |
|--------|-----------|------|-------|-------|
| PNG | .png | Yes | Yes | Lossless, transparency support |
| JPEG | .jpg, .jpeg | Yes | Yes | Quality 1-100, no transparency |
| BMP | .bmp | Yes | Yes | Windows bitmap |
| GIF | .gif | Yes | Yes | First frame only (animation not supported) |
| TIFF | .tiff, .tif | Yes | Yes | Single page (multi-page not supported) |
| WebP | .webp | Yes | Yes | Lossy and lossless modes |
| SVG | .svg | Yes | No | Rasterization only (read-only) |

### Planned Image Formats (Future Releases)

| Format | Extension | Priority | Notes |
|--------|-----------|----------|-------|
| TGA | .tga | Medium | Targa format |
| ICO | .ico | Medium | Windows icon |
| DDS | .dds | Low | DirectDraw Surface |
| HDR | .hdr | Low | High dynamic range |
| OpenEXR | .exr | Medium | Professional HDR |
| AVIF | .avif | Low | Modern compression |
| PDF | .pdf | Low | Page to image (read-only) |

---

## 3D Mesh Formats

| Format | Extension | Read | Write | Notes |
|--------|-----------|------|-------|-------|
| STL | .stl | Yes | Yes | Binary and ASCII variants |
| OBJ | .obj | Yes | Yes | Wavefront, partial material support |
| PLY | .ply | Yes | Yes | Stanford polygon format |
| OFF | .off | Yes | Yes | Object file format |
| glTF/GLB | .gltf, .glb | Yes | Yes | glTF 2.0, materials supported |
| DXF | .dxf | Yes | Yes | AutoCAD, 3DFACE entities |
| STEP | .step, .stp | Yes | No | FACETED_BREP only (feature-gated) |

### STEP Format Details

**Status:** Implemented in v0.2.0 (feature-gated)

**Capabilities:**
- Parses STEP AP203 files using ruststep 0.4.0
- Extracts FACETED_BREP entities (pre-tessellated geometry)
- Converts to mesh formats (STL, OBJ, PLY, etc.)

**Limitations:**
- **FACETED_BREP only** - Requires pre-tessellated geometry in STEP file
- **No B-Rep support** - NURBS surfaces, cylinders, spheres not supported
- **Read-only** - Cannot write STEP files
- **Feature-gated** - Requires `--features step` flag

**Future:** Full B-Rep support via opencascade-rs planned for v1.1.0

**Documentation:**
- `docs/CAD_EXPORT_GUIDE.md` - How to export FACETED_BREP from CAD software
- `docs/STEP_FORMAT_REFERENCE.md` - Technical reference

### Planned Mesh Formats (Future Releases)

| Format | Extension | Priority | Notes |
|--------|-----------|----------|-------|
| IGES | .igs, .iges | Low | Legacy CAD format |
| 3MF | .3mf | Medium | 3D manufacturing |

---

## Format Detection

Formats are detected using a two-stage approach:

### Stage 1: File Extension
Primary detection method based on file extension.

### Stage 2: Magic Bytes
Fallback for unknown or missing extensions.

| Format | Magic Bytes | Offset |
|--------|-------------|--------|
| PNG | `89 50 4E 47 0D 0A 1A 0A` | 0 |
| JPEG | `FF D8 FF` | 0 |
| GIF | `47 49 46 38` ("GIF8") | 0 |
| BMP | `42 4D` ("BM") | 0 |
| TIFF (LE) | `49 49 2A 00` | 0 |
| TIFF (BE) | `4D 4D 00 2A` | 0 |
| WebP | `52 49 46 46` + `57 45 42 50` | 0, 8 |
| STL (ASCII) | `solid` | 0 |
| glTF (JSON) | `7B` ("{") | 0 |
| GLB (Binary) | `67 6C 54 46` ("glTF") | 0 |

---

## Quality and Compression Settings

### Image Quality (Lossy Formats)

| Format | Range | Default | Notes |
|--------|-------|---------|-------|
| JPEG | 1-100 | 90 | Higher = better quality, larger file |
| WebP | 1-100 | 90 | Lossy mode only |

### Compression Level (Lossless Formats)

| Format | Range | Default | Notes |
|--------|-------|---------|-------|
| PNG | 0-9 | 6 | Higher = smaller file, slower |

---

## Feature Support Matrix

### Image Features

| Feature | PNG | JPEG | BMP | GIF | TIFF | WebP | SVG |
|---------|-----|------|-----|-----|------|------|-----|
| Transparency | Yes | No | No | Yes | Yes | Yes | Yes |
| Quality Control | No | Yes | No | No | No | Yes | N/A |
| Lossless | Yes | No | Yes | Yes | Yes | Yes | N/A |
| Animation | No | No | No | No* | No | No | No |

*GIF animation not supported - first frame only

### Mesh Features

| Feature | STL | OBJ | PLY | OFF | glTF | DXF | STEP |
|---------|-----|-----|-----|-----|------|-----|------|
| Vertex Colors | No | No | Yes | Yes | Yes | No | No |
| Materials | No | Yes | No | No | Yes | No | No |
| Textures | No | Yes | No | No | Yes | No | No |
| Binary Format | Yes | No | Yes | No | Yes | No | No |
| Normal Data | Yes | Yes | Yes | Yes | Yes | No | Yes |

### Mesh Processing Features (v0.1.1+)

| Feature | Status | Notes |
|---------|--------|-------|
| Coordinate Transforms | Yes | Y-up to Z-up, etc. |
| Normal Recalculation | Yes | Automatic when missing |
| Mesh Validation | Yes | Vertex/face validation |
| Vertex Deduplication | Yes | Remove duplicate vertices |

---

## v0.3.0 Features

### Parallel Batch Processing
- Convert multiple files simultaneously
- Configurable concurrency (1-16 threads)
- Default: Number of CPU cores (capped at 8)
- Up to 4x speedup on 4-core systems

### 3D Mesh Viewer
- Interactive preview in GUI
- Camera controls: orbit, pan, zoom
- Rendering modes: solid, wireframe
- Hardware accelerated (WebGPU)
- Feature-gated: `--features viewer-3d`

### Settings Auto-Save
- Automatic save 500ms after changes
- Visual status indicator
- No manual save required

### Queue Item Editing
- Edit pending batch items
- Change format, path, options
- No need to remove and re-add

---

## Resource Limits

Default limits to prevent resource exhaustion:

| Resource | Default Limit | Configurable |
|----------|---------------|--------------|
| Max File Size | 100 MB | Yes |
| Max Image Dimension | 65,535 px | Yes |
| Max Vertices | 10,000,000 | Yes |
| Max Faces | 10,000,000 | Yes |
| Max Concurrent Conversions | 8 | Yes (1-16) |

---

## Performance Characteristics

### Typical Conversion Times

| Operation | File Size | Time |
|-----------|-----------|------|
| PNG to JPEG | 10 MB | < 1 sec |
| JPEG to PNG | 5 MB | < 1 sec |
| STL to OBJ | 50K vertices | < 1 sec |
| OBJ to glTF | 100K vertices | 1-2 sec |
| STEP to STL | Varies | 1-5 sec |

### Memory Usage

| Type | Memory | Notes |
|------|--------|-------|
| Images | ~3x file size | Read + decode + encode |
| Meshes | ~2x file size | Read + parse + write |

---

## Version History

| Version | Formats Added | Features Added |
|---------|---------------|----------------|
| v0.1.0 | PNG, JPEG, BMP, GIF, STL, OBJ, PLY | Core converters |
| v0.1.1 | - | Mesh transforms, validation |
| v0.2.0 | STEP (FACETED_BREP) | Feature gates |
| v0.2.1 | - | GUI application |
| v0.2.2 | - | Batch processing, preview |
| v0.3.0 | - | Parallel processing, 3D viewer |
| v1.0.0 | - | First stable release |

---

## Notes

- **SVG**: Read-only via rasterization. Cannot write SVG output.
- **STEP**: FACETED_BREP only. Full B-Rep support planned for v1.1.0.
- **FBX**: Not supported (proprietary format, no open-source Rust library).
- **DWG**: Not supported (proprietary format).

---

## Related Documentation

- `docs/CAD_EXPORT_GUIDE.md` - Exporting FACETED_BREP from CAD software
- `docs/STEP_FORMAT_REFERENCE.md` - STEP format technical reference
- `docs/PERFORMANCE.md` - Performance benchmarks and optimization
- `docs/GUI_USAGE_GUIDE.md` - GUI application usage guide

---

*For architecture details, see `Phase3_Architecture.md`*
*For implementation plan, see `IMPLEMENTATION_PLAN.md`*
