# Phase 2.1: Specification Update - Decisions Incorporated

## Decisions Made

1. ✓ **CAD Formats:** STEP support required (via FFI to Open CASCADE)
2. ✓ **FBX:** Not required, skip FBX support
3. ✓ **Timeline:** Full-featured implementation (12+ weeks)
4. ✓ **GUI:** CLI for MVP, GUI on roadmap with drag-and-drop
5. ✓ **Custom Parsers:** Use FFI for STEP, don't write from scratch

---

## Updated 3D Format Support Matrix

### Tier 1: Core Mesh Formats (Pure Rust)
| Format | Extension | Status | Library |
|--------|-----------|--------|---------|
| **STL** | .stl | ✓ Native | `stl_io` |
| **OBJ** | .obj | ✓ Native | `tobj` |
| **PLY** | .ply | ✓ Native | `ply-rs` |
| **OFF** | .off | ✓ Custom | Write parser |

### Tier 2: Scene Formats (Pure Rust)
| Format | Extension | Status | Library |
|--------|-----------|--------|---------|
| **glTF** | .gltf/.glb | ✓ Native | `gltf` |

### Tier 3: CAD Formats (FFI Required)
| Format | Extension | Status | Library |
|--------|-----------|--------|---------|
| **DXF** | .dxf | ✓ Native | `dxf` |
| **STEP** | .step/.stp | ✓ FFI | Open CASCADE via `opencascade-sys` |
| **IGES** | .iges/.igs | ⚠️ FFI (optional) | Open CASCADE via `opencascade-sys` |

### Explicitly Not Supported
| Format | Reason |
|--------|--------|
| **FBX** | Proprietary, decision: skip |
| **DWG** | Proprietary, no open-source option |

---

## STEP Support Strategy

### Option A: opencascade-sys (Rust FFI bindings)

**Repository:** https://github.com/bschwind/opencascade-rs

```toml
[dependencies]
opencascade = "0.1"  # High-level wrapper
opencascade-sys = "0.1"  # Low-level FFI bindings
```

**Pros:**
- Rust-idiomatic interface
- Community-maintained
- Access to full OCCT functionality

**Cons:**
- Requires OCCT C++ library installation
- Complex build process
- Larger binary size (~15-20 MB)
- Platform-specific compilation

**Build Requirements:**
- Open CASCADE Technology (OCCT) 7.7+
- CMake
- C++17 compiler
- Platform-specific dependencies

### Option B: truck (Pure Rust CAD kernel)

**Repository:** https://github.com/ricosjp/truck

```toml
[dependencies]
truck-modeling = "0.4"
truck-polymesh = "0.4"
truck-shapeops = "0.4"
truck-stepio = "0.4"  # STEP I/O
```

**Pros:**
- Pure Rust (no C++ dependencies)
- Modern, clean API
- Active development
- Smaller binary

**Cons:**
- Less mature than OCCT
- STEP support still evolving
- May not support all STEP features
- Smaller community

### Recommended: Hybrid Approach

**Phase 1 (MVP):**
- Skip STEP initially
- Focus on STL, OBJ, PLY, glTF, DXF
- Deliver working CLI tool quickly

**Phase 2 (STEP Integration):**
- Evaluate `truck` first (pure Rust preference)
- If insufficient, add OCCT FFI
- Test with real STEP files
- Optimize build process

**Reasoning:**
- STEP is complex, needs careful integration
- Better to deliver working tool, then add STEP
- Allows time to evaluate best approach
- Reduces initial complexity

---

## Updated Architecture Decision

### Project Structure

```
workspace/
├── Cargo.toml                 # Workspace definition
├── img-convert/               # 2D converter binary
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── cli.rs
│   └── tests/
├── mesh-convert/              # 3D converter binary
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── cli.rs
│   └── tests/
├── img-core/                  # 2D conversion library
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   └── tests/
├── mesh-core/                 # 3D conversion library
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   └── tests/
├── converter-common/          # Shared utilities
│   ├── Cargo.toml
│   └── src/
│       ├── error.rs
│       ├── progress.rs
│       └── validation.rs
└── converter-gui/             # Future GUI (Phase 3)
    ├── Cargo.toml
    └── src/
```

**Benefits:**
- Separate libraries from binaries
- GUI can import both libraries
- Shared utilities across projects
- Clear separation of concerns

---

## Updated Implementation Phases

### Phase 1: Core Converters (4-6 weeks)
**Week 1-2: img-convert (2D)**
- ✓ PNG, JPG, BMP, GIF, TIFF, WebP
- ✓ Quality controls
- ✓ CLI interface
- ✓ Library + binary structure

**Week 3-4: mesh-convert (3D - Tier 1)**
- ✓ STL (binary/ASCII), OBJ, PLY
- ✓ OFF parser (custom)
- ✓ CLI interface
- ✓ Library + binary structure

**Week 5-6: Testing & Polish**
- ✓ Integration tests
- ✓ Error handling
- ✓ Documentation
- ✓ Windows build
- 🚀 Release v0.1.0 (MVP)

### Phase 2: Extended Formats (4-5 weeks)
**Week 7-8: Advanced 2D**
- ✓ SVG rasterization (resvg)
- ✓ AVIF, OpenEXR
- ✓ Advanced quality presets
- ✓ Metadata handling

**Week 9-10: Advanced 3D**
- ✓ glTF support
- ✓ DXF support
- ✓ Coordinate transforms
- ✓ Normal recalculation

**Week 11: STEP Evaluation**
- Evaluate `truck` STEP support
- Test with real files
- Decide on approach

**Week 12: Testing**
- Extended test suite
- Performance optimization
- 🚀 Release v0.2.0

### Phase 3: STEP + CAD (3-4 weeks)
**Week 13-14: STEP Integration**
- Integrate chosen library (truck or OCCT FFI)
- STEP read/write
- Conversion testing

**Week 15-16: CAD Polish**
- IGES support (if OCCT route)
- DXF improvements
- CAD-specific validations
- 🚀 Release v0.3.0 (Full CAD support)

### Phase 4: GUI (4-6 weeks)
**Week 17-19: GUI Core**
- egui framework setup
- Tabbed interface (2D/3D)
- File selection
- Format dropdowns

**Week 20-22: GUI Features**
- Drag-and-drop files
- Batch queue
- Progress bars
- Settings panel
- Preview (optional)

**Week 23: Polish & Release**
- Icons and branding
- Installer (NSIS)
- User documentation
- 🚀 Release v1.0.0 (GUI)

**Total Timeline: 23 weeks (~5-6 months)**

---

## Binary Size Projections

### MVP (Phase 1)
- `img-convert.exe`: ~3-5 MB
- `mesh-convert.exe`: ~2-3 MB
- **Total: ~5-8 MB**

### With Extended Formats (Phase 2)
- `img-convert.exe`: ~4-6 MB
- `mesh-convert.exe`: ~3-4 MB
- **Total: ~7-10 MB**

### With STEP via truck (Phase 3a)
- `mesh-convert.exe`: ~4-6 MB
- **Total: ~8-12 MB**

### With STEP via OCCT FFI (Phase 3b)
- `mesh-convert.exe`: ~15-20 MB (includes OCCT)
- **Total: ~19-26 MB**

### With GUI (Phase 4)
- `converter-gui.exe`: ~8-12 MB (includes both libraries)
- CLI tools remain separate
- **Total distribution: ~25-40 MB** (depending on STEP approach)

---

## Risk Mitigation

### Risk 1: STEP Integration Complexity
**Mitigation:**
- Defer to Phase 3
- Evaluate pure Rust first
- Allow time for testing
- Have fallback (skip STEP if too complex)

### Risk 2: Cross-Compilation Issues
**Mitigation:**
- Test Windows builds early
- Use GitHub Actions CI/CD
- Provide build instructions
- Consider building on Windows directly

### Risk 3: Library API Changes
**Mitigation:**
- Pin dependency versions
- Regular security updates
- Maintain compatibility layer
- Good test coverage

### Risk 4: Performance with Large Files
**Mitigation:**
- Streaming I/O where possible
- Memory-mapped files
- Progress indicators
- Chunked processing

---

## Success Criteria

### MVP (v0.1.0)
- ✓ All Tier 1 formats working (2D: 6 formats, 3D: 4 formats)
- ✓ Clean CLI interface
- ✓ Error handling
- ✓ Windows executable
- ✓ Basic documentation

### Full-Featured (v0.3.0)
- ✓ All planned formats including STEP
- ✓ Advanced options (quality, transforms, etc.)
- ✓ Comprehensive test suite
- ✓ Performance optimized
- ✓ Complete documentation

### GUI Release (v1.0.0)
- ✓ Intuitive drag-and-drop interface
- ✓ Batch processing
- ✓ Settings persistence
- ✓ Professional appearance
- ✓ Installer
- ✓ User guide

---

## Next: Phase 3 Architecture

Ready to proceed to detailed architecture design:

1. **Module diagrams** for both converters
2. **Interface definitions** (traits, structs)
3. **Error handling strategy** (custom error types)
4. **Data flow** diagrams
5. **Testing approach** (unit, integration, benchmarks)
6. **Build configuration** (Cargo.toml templates)

Should I proceed with Phase 3 architecture design now?
