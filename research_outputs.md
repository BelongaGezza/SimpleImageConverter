# Research Outputs
## SimpleImageConverter Research Findings

**Maintained By:** Researcher (Dr. Taylor Kim)
**Last Updated:** January 2026
**Purpose:** Consolidated research findings on libraries, integration approaches, and technical evaluations

**Note:** For Rust language and best practices, see `rust-resources.md`

---

## Table of Contents

1. [opencascade-rs Integration Research](#opencascade-rs-integration-research)
2. [3D Rendering Libraries Research](#3d-rendering-libraries-research)

---

## opencascade-rs Integration Research

**Research Date:** December 2025
**Status:** Complete
**Related Sprint:** Sprint 9, Sprint 10

### Executive Summary

opencascade-rs provides Rust bindings to OpenCASCADE Technology (OCCT) for full STEP B-Rep support with curved surface tessellation. This complements the existing v0.2.0 FACETED_BREP extraction (pure Rust via ruststep).

**Key Findings:**
- ✅ Technically feasible - opencascade-rs 0.2.0 provides necessary APIs
- ✅ API compatible with current MeshReader trait interface
- ⚠️ Build complexity HIGH - Requires OCCT installation, CMake, C++17
- ⚠️ Binary size impact EXCEEDS TARGET - +10-15 MB (dynamic) or +90-140 MB (static)
- ✅ Hybrid approach recommended - FACETED_BREP first, opencascade-rs fallback

**Recommendation:** Proceed with feature-gated integration for v0.3.0 as optional enhancement.

### Library Status

**Repository:** https://github.com/bschwind/opencascade-rs
**License:** MIT OR Apache-2.0
**Version:** 0.2.0
**Status:** Active development

**OCCT Requirements:**
- OpenCASCADE Technology 7.7+
- CMake 3.18+
- C++17 compiler
- Platform-specific libraries

### Integration Architecture

**Hybrid Strategy:**
```rust
impl MeshReader for StepFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Strategy 1: Try FACETED_BREP first (pure Rust, fast)
        if let Ok(mesh) = self.extract_faceted_brep(data) {
            return Ok(mesh);
        }

        // Strategy 2: Fall back to opencascade-rs (if enabled)
        #[cfg(feature = "step-opencascade")]
        {
            if let Ok(mesh) = self.extract_with_opencascade(data) {
                return Ok(mesh);
            }
        }

        Err(ConversionError::UnsupportedGeometry)
    }
}
```

**Benefits:**
- Maintains pure Rust option (FACETED_BREP)
- Adds full support when opencascade-rs enabled
- Graceful degradation
- User choice via feature flags

### Build Complexity

**System Dependencies:**
- OCCT installation (package manager or from source)
- CMake build system
- C++ toolchain

**Binary Size Impact:**
- Dynamic linking: +10-15 MB binary, +100 MB OCCT runtime
- Static linking: +90-140 MB binary (not recommended)

**Build Time:**
- First build: 10-30 minutes
- Incremental: 1-5 minutes

**Mitigation:**
- Feature-gated via `step-opencascade` feature
- Clear documentation of requirements
- Dynamic linking recommended

### Feature Flag Strategy

```toml
[features]
default = []
step = ["ruststep"]  # Pure Rust STEP support (v0.2.0)
step-opencascade = ["opencascade", "opencascade-sys", "step"]  # Full STEP support (v0.3.0)
```

**Build Options:**
- `cargo build --features step` - FACETED_BREP only
- `cargo build --features step-opencascade` - Full support (requires OCCT)

### Documentation Created

1. `docs/OCCT_INSTALLATION.md` - Complete installation guide for all platforms
2. `docs/OPENCASCADE_RS_LIMITATIONS.md` - Comprehensive limitations and known issues
3. `docs/STEP_FORMAT_REFERENCE.md` - Updated with opencascade-rs integration details
4. `docs/OPENCASCADE_RS_TESTING_REQUIREMENTS.md` - Testing procedures and requirements

### Performance Characteristics

**Tessellation:**
- Typical files: <1 second
- Complex models: 100ms-5s
- Memory scales with mesh complexity

**Comparison:**
- FACETED_BREP: Very fast (<100ms, no tessellation)
- OCCT tessellation: Moderate (100ms-5s)

### Integration Challenges

1. **Build System Complexity**
   - Mitigation: Clear documentation, build scripts, helpful error messages

2. **Platform-Specific Configuration**
   - Mitigation: pkg-config support, documented installation paths

3. **Binary Size**
   - Mitigation: Dynamic linking, feature-gated, well-documented

4. **API Maturity**
   - Mitigation: Version pinning, FACETED_BREP fallback

### References

- opencascade-rs: https://github.com/bschwind/opencascade-rs
- OpenCASCADE Technology: https://dev.opencascade.org/
- OCCT Documentation: https://dev.opencascade.org/doc/refman/html/

---

## 3D Rendering Libraries Research

**Research Date:** December 2025
**Status:** Complete
**Related Sprint:** Sprint 9

### Executive Summary

Research on 3D rendering libraries for implementing a mesh viewer in the egui-based GUI. Evaluated wgpu, three-d, and kiss3d for egui integration, performance, and binary size impact.

**Key Findings:**
- ✅ **wgpu (Recommended):** Best egui integration, excellent performance
- ✅ **three-d (Alternative):** Easier API, good performance
- ❌ **kiss3d (Not Recommended):** OpenGL conflicts with egui's wgpu

**Recommendation:** wgpu for primary implementation, three-d as fallback option.

### Library Comparison

#### wgpu

**Repository:** https://github.com/gfx-rs/wgpu
**License:** Apache-2.0 OR MIT
**Version:** 28.0.0
**Status:** Mature, actively maintained

**Pros:**
- ✅ Excellent egui integration (egui uses wgpu internally)
- ✅ Modern WebGPU standard, future-proof
- ✅ Excellent performance (handles 1M+ vertices)
- ✅ Cross-platform (Windows, macOS, Linux, Web)
- ✅ Pure Rust (no C++ dependencies)

**Cons:**
- ❌ Complexity: Low-level API, requires shader knowledge
- ❌ Learning curve: Steeper than high-level libraries
- ❌ Binary size: ~5-10 MB additional

**Integration:**
- Use egui's `PaintCallback` for custom rendering
- Access egui's wgpu context directly
- Native integration, no conflicts

**Performance:** Excellent (1M+ vertices with optimization)
**Binary Size:** ~5-10 MB additional
**Integration Complexity:** Medium-High

#### three-d

**Repository:** https://github.com/asny/three-d
**License:** MIT OR Apache-2.0
**Version:** 0.18.2
**Status:** Active development

**Pros:**
- ✅ High-level API (easy to use)
- ✅ Built-in mesh loading and rendering
- ✅ Built-in camera controls
- ✅ Good documentation
- ✅ Pure Rust (uses wgpu internally)

**Cons:**
- ⚠️ egui integration not built-in (requires custom work)
- ⚠️ API maturity (version 0.18.2, may have changes)
- ⚠️ Binary size: ~8-12 MB additional

**Integration:**
- Requires custom egui integration
- May need offscreen rendering to texture

**Performance:** Good (100K-500K vertices comfortably)
**Binary Size:** ~8-12 MB additional
**Integration Complexity:** Medium

#### kiss3d

**Repository:** https://github.com/sebcrozet/kiss3d
**License:** BSD-3-Clause
**Version:** 0.37.2
**Status:** Maintained, less active

**Pros:**
- ✅ Simple API
- ✅ Lightweight (~3-5 MB)

**Cons:**
- ❌ Uses OpenGL (conflicts with egui's wgpu)
- ❌ Difficult egui integration
- ❌ Limited performance for large meshes
- ❌ Less active development

**Recommendation:** ❌ NOT RECOMMENDED for egui integration

### Comparison Matrix

| Feature | wgpu | three-d | kiss3d |
|---------|------|---------|--------|
| **egui Integration** | ✅ Excellent (native) | ⚠️ Custom needed | ❌ Difficult (OpenGL conflict) |
| **API Level** | Low-level | High-level | High-level |
| **Learning Curve** | Steep | Moderate | Easy |
| **Binary Size** | ~5-10 MB | ~8-12 MB | ~3-5 MB |
| **Performance** | Excellent (1M+ vertices) | Good (100K-500K) | Moderate (<100K) |
| **Cross-platform** | ✅ Excellent | ✅ Good | ✅ Good |
| **Maintenance** | ✅ Active | ✅ Active | ⚠️ Less active |

### Integration with egui

**egui Custom Rendering:**
- egui supports custom rendering via `PaintCallback`
- wgpu integrates natively (egui uses wgpu internally)
- Access egui's wgpu context for mesh rendering

**Example Pattern:**
```rust
use egui::{PaintCallback, Rect};

fn render_3d_in_egui(ui: &mut egui::Ui, mesh: &Mesh) {
    let (rect, response) = ui.allocate_exact_size(
        egui::Vec2::new(400.0, 400.0),
        egui::Sense::drag()
    );

    // Create custom paint callback for 3D rendering
    let callback = PaintCallback {
        rect,
        callback: Arc::new(/* wgpu rendering code */),
    };

    ui.painter().add(callback);
}
```

### Recommended Approach

**Primary: wgpu**
- Use egui's `PaintCallback` for custom rendering
- Access egui's wgpu context for mesh rendering
- Implement simple mesh shader (vertex + fragment)
- Add camera controls via egui input events
- Estimated effort: 12-16 hours for prototype

**Alternative: three-d**
- Create three-d rendering context
- Use three-d's mesh rendering
- Custom egui integration via offscreen rendering
- Estimated effort: 12-14 hours (but more integration complexity)

### Performance Considerations

**Mesh Size Handling:**
- Small (<10K vertices): Any library works
- Medium (10K-100K): wgpu excellent, three-d good
- Large (100K-1M): wgpu excellent (with optimization), three-d good
- Very Large (1M+): wgpu recommended (with LOD/culling)

**Optimization Strategies:**
- Level of Detail (LOD) for large meshes
- Frustum culling for off-screen geometry
- Instancing for multiple meshes

### Binary Size Impact

| Configuration | Binary Size | Notes |
|--------------|-------------|-------|
| Current (without 3D viewer) | ~8-12 MB | Base GUI |
| With wgpu | ~13-22 MB | +5-10 MB |
| With three-d | ~16-24 MB | +8-12 MB |
| With kiss3d | ~11-17 MB | +3-5 MB (not recommended) |

**Assessment:** All reasonable. wgpu has best size/performance ratio.

### Cross-Platform Support

All evaluated libraries support:
- ✅ Windows (DirectX12, Vulkan)
- ✅ macOS (Metal)
- ✅ Linux (Vulkan)
- ✅ Web (WebGPU)

### References

- wgpu: https://github.com/gfx-rs/wgpu
- three-d: https://github.com/asny/three-d
- kiss3d: https://github.com/sebcrozet/kiss3d
- egui Custom Rendering: https://docs.rs/egui/latest/egui/struct.PaintCallback.html

---

## Change Log

| Date | Research Area | Summary |
|------|---------------|---------|
| 2025-12 | opencascade-rs | Initial research for v0.3.0 STEP support |
| 2025-12 | opencascade-rs | Sprint 9 detailed research completed |
| 2025-12 | opencascade-rs | Sprint 10 documentation verified |
| 2025-12 | 3D Rendering | Sprint 9 library evaluation completed |
| 2026-01 | Consolidation | Created consolidated research outputs |

---

**For Rust-specific knowledge, best practices, and ecosystem updates, see `rust-resources.md`**
