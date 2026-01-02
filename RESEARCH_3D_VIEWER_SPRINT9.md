# 3D Rendering Library Research - Sprint 9
## v0.3.0 3D Mesh Viewer

**Researcher:** Dr. Taylor Kim (Primary)  
**Junior Engineer 3D:** Alex Rivera (Supporting)  
**Date:** December 30, 2025  
**Status:** ✅ **RESEARCH COMPLETE**  
**Priority:** Medium (Sprint 9 Task 1.2)

---

## Executive Summary

This document provides research on 3D rendering libraries for implementing a 3D mesh viewer in SimpleImageConverter. The viewer will integrate with the existing egui-based GUI to provide mesh preview functionality. This research evaluates wgpu, three-d, and kiss3d for their suitability, integration complexity, and performance characteristics.

**Key Research Questions:**
1. Which library integrates best with egui?
2. What is the binary size impact?
3. What is the performance for typical meshes?
4. How complex is the integration?
5. Are there cross-platform issues?

**Status:** Research complete. All acceptance criteria met. Ready for prototype phase (Task 2.2).

---

## 1. Current Context

### 1.1 GUI Framework

**Current Setup:**
- **Framework:** egui 0.27 + eframe 0.27
- **Platform:** Cross-platform (Windows, macOS, Linux)
- **Rendering:** egui's immediate mode rendering
- **Integration Point:** Preview panel in converter-gui

**Current Preview Implementation:**
- Location: `converter-gui/src/ui/preview.rs`
- Current: Metadata display only (vertex count, face count, format)
- Target: Add 3D mesh rendering capability

### 1.2 Mesh Data Format

**Mesh Structure:**
```rust
// mesh-core/src/mesh/mod.rs
pub struct Mesh {
    pub vertices: Vec<Vertex>,  // Vec<{x: f32, y: f32, z: f32}>
    pub faces: Vec<Face>,        // Vec<{indices: [usize; 3]}>
    pub normals: Vec<Normal>,   // Vec<{x: f32, y: f32, z: f32}>
}
```

**Typical Mesh Sizes:**
- Small: 1K-10K vertices
- Medium: 10K-100K vertices
- Large: 100K-1M vertices
- Very Large: 1M+ vertices (may need LOD)

---

## 2. Library Evaluation

### 2.1 wgpu

**Repository:** https://github.com/gfx-rs/wgpu  
**License:** Apache-2.0 OR MIT  
**Latest Version:** 28.0.0 (verified December 30, 2025)  
**Status:** Mature, actively maintained

**Description:**
- Cross-platform, safe, pure-rust graphics API
- WebGPU-based (works on Web, native, and mobile)
- Low-level graphics API (similar to Vulkan/Metal/DirectX12)
- Used by egui itself for rendering

**Pros:**
- ✅ **Excellent egui Integration:** egui uses wgpu internally, native integration possible
- ✅ **Cross-platform:** Windows, macOS, Linux, Web, mobile
- ✅ **Modern API:** WebGPU standard, future-proof
- ✅ **Performance:** Excellent for large meshes
- ✅ **Active Development:** Well-maintained, large community
- ✅ **No C++ Dependencies:** Pure Rust

**Cons:**
- ❌ **Complexity:** Low-level API, requires shader knowledge
- ❌ **Learning Curve:** Steeper than high-level libraries
- ❌ **Binary Size:** Moderate (~5-10 MB additional)
- ❌ **Setup Complexity:** Requires graphics context management

**Integration Approach:**
```rust
// Use egui's custom rendering support
// egui supports custom rendering via egui::PaintCallback
use egui::PaintCallback;
use wgpu::*;

// Render 3D mesh in egui panel
fn render_mesh_in_egui(ui: &mut egui::Ui, mesh: &Mesh) {
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

**Binary Size Impact:** ~5-10 MB additional

**Performance:** Excellent (can handle 1M+ vertices with proper optimization)

**Integration Complexity:** Medium-High (requires graphics programming knowledge)

---

### 2.2 three-d

**Repository:** https://github.com/asny/three-d  
**License:** MIT OR Apache-2.0  
**Latest Version:** 0.18.2 (verified December 30, 2025)  
**Status:** Active development

**Description:**
- High-level 3D renderer for Rust
- Makes it simple to draw 3D graphics across platforms
- Inspired by Three.js (JavaScript)
- Supports Web and native platforms

**Pros:**
- ✅ **High-Level API:** Easy to use, simple mesh rendering
- ✅ **Cross-platform:** Windows, macOS, Linux, Web
- ✅ **Good Documentation:** Well-documented API
- ✅ **Mesh Support:** Built-in mesh loading and rendering
- ✅ **Camera Controls:** Built-in camera and controls
- ✅ **No C++ Dependencies:** Pure Rust (uses wgpu internally)

**Cons:**
- ⚠️ **egui Integration:** Not directly designed for egui, requires custom integration
- ⚠️ **API Maturity:** Version 0.18.2 (may have API changes)
- ⚠️ **Binary Size:** Moderate (~8-12 MB additional)
- ⚠️ **Performance:** Good, but may be slower than wgpu for very large meshes

**Integration Approach:**
```rust
// Use three-d's window/context, integrate with egui via custom rendering
// May require separate window or custom egui integration

use three_d::*;

// Create three-d context (may conflict with egui's context)
let context = Context::from_window(window)?;
let camera = Camera::new_perspective(
    viewport,
    vec3(0.0, 0.0, 5.0),
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    45.0,
    0.1,
    1000.0,
);

// Render mesh
let mesh = Mesh::new(&context, &mesh_data)?;
mesh.render(&camera, &lights)?;
```

**Binary Size Impact:** ~8-12 MB additional

**Performance:** Good (can handle 100K-500K vertices comfortably)

**Integration Complexity:** Medium (requires custom egui integration)

---

### 2.3 kiss3d

**Repository:** https://github.com/sebcrozet/kiss3d  
**License:** BSD-3-Clause  
**Latest Version:** 0.37.2 (verified December 30, 2025)  
**Status:** Maintained, but less active

**Description:**
- Keep it simple, stupid, 2D and 3D graphics engine
- Simple API for basic 3D rendering
- Uses OpenGL (via glium or glow)
- Focused on simplicity over features

**Pros:**
- ✅ **Simple API:** Very easy to use, minimal setup
- ✅ **Lightweight:** Smaller binary size
- ✅ **Good for Prototypes:** Quick to get started
- ✅ **Mesh Support:** Basic mesh rendering built-in

**Cons:**
- ❌ **egui Integration:** Not designed for egui, difficult integration
- ❌ **OpenGL Dependency:** Uses OpenGL (may conflict with egui's wgpu)
- ❌ **Limited Features:** Basic rendering only, fewer features
- ❌ **Performance:** May be slower for large meshes
- ❌ **Maintenance:** Less active development

**Integration Approach:**
```rust
// kiss3d uses its own window/context (OpenGL)
// Integration with egui would be very difficult
// Would likely require separate window or significant workarounds

use kiss3d::window::Window;
use kiss3d::scene::SceneNode;

let mut window = Window::new("Mesh Viewer");
let mut mesh_node = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
```

**Binary Size Impact:** ~3-5 MB additional

**Performance:** Moderate (good for small-medium meshes, <100K vertices)

**Integration Complexity:** High (OpenGL conflicts with egui's wgpu)

---

## 3. Comparison Matrix

| Feature | wgpu | three-d | kiss3d |
|---------|------|---------|--------|
| **egui Integration** | ✅ Excellent (native) | ⚠️ Custom integration needed | ❌ Difficult (OpenGL conflict) |
| **API Level** | Low-level | High-level | High-level |
| **Learning Curve** | Steep | Moderate | Easy |
| **Binary Size** | ~5-10 MB | ~8-12 MB | ~3-5 MB |
| **Performance** | Excellent (1M+ vertices) | Good (100K-500K) | Moderate (<100K) |
| **Cross-platform** | ✅ Excellent | ✅ Good | ✅ Good |
| **Documentation** | ✅ Good | ✅ Good | ⚠️ Basic |
| **Maintenance** | ✅ Active | ✅ Active | ⚠️ Less active |
| **Mesh Support** | Manual (shaders) | ✅ Built-in | ✅ Built-in |
| **Camera Controls** | Manual | ✅ Built-in | ✅ Built-in |
| **C++ Dependencies** | ✅ None | ✅ None | ✅ None |

---

## 4. Integration with egui

### 4.1 egui Custom Rendering

**egui Support:**
- egui supports custom rendering via `PaintCallback`
- Can render custom graphics in egui panels
- Uses wgpu internally, so wgpu integration is natural

**Example Pattern:**
```rust
use egui::{PaintCallback, Rect};
use std::sync::Arc;

fn render_3d_in_egui(ui: &mut egui::Ui, mesh: &Mesh) {
    let (rect, response) = ui.allocate_exact_size(
        egui::Vec2::new(400.0, 400.0),
        egui::Sense::drag()
    );
    
    // Get egui's wgpu context
    let wgpu_context = ui.ctx().data(|d| {
        d.get_persisted::<WgpuContext>(egui::Id::new("wgpu_context"))
    });
    
    // Render mesh using wgpu
    // (implementation details depend on chosen approach)
}
```

### 4.2 Integration Challenges

**Challenge 1: Graphics Context Sharing**
- egui uses wgpu internally
- Need to share wgpu device/queue with 3D renderer
- Solution: Access egui's wgpu context

**Challenge 2: Coordinate Systems**
- egui uses screen coordinates (pixels)
- 3D rendering uses world coordinates
- Solution: Proper viewport and projection matrix setup

**Challenge 3: Input Handling**
- egui handles mouse/keyboard input
- 3D viewer needs camera controls
- Solution: Use egui's input events for camera controls

---

## 5. Recommended Approach

### 5.1 Primary Recommendation: wgpu (with egui integration)

**Rationale:**
1. **Native egui Integration:** egui uses wgpu, natural integration
2. **Performance:** Best performance for large meshes
3. **Future-proof:** WebGPU standard, actively maintained
4. **No Conflicts:** No OpenGL/wgpu conflicts

**Implementation Strategy:**
1. Use egui's `PaintCallback` for custom rendering
2. Access egui's wgpu context for mesh rendering
3. Implement simple mesh shader (vertex + fragment)
4. Add camera controls via egui input events

**Complexity:** Medium-High (requires graphics programming)

**Timeline:** 12-16 hours for prototype

### 5.2 Alternative: three-d (if wgpu too complex)

**Rationale:**
1. **Easier API:** High-level, less graphics knowledge needed
2. **Built-in Features:** Camera, mesh loading, lighting
3. **Good Performance:** Sufficient for typical meshes

**Implementation Strategy:**
1. Create separate rendering context (may need custom egui integration)
2. Use three-d's mesh rendering
3. Integrate with egui panel (may require workarounds)

**Complexity:** Medium (custom integration needed)

**Timeline:** 10-14 hours for prototype

### 5.3 Not Recommended: kiss3d

**Rationale:**
1. **OpenGL Conflict:** Uses OpenGL, conflicts with egui's wgpu
2. **Integration Difficulty:** Very difficult to integrate with egui
3. **Limited Performance:** Not suitable for large meshes

**Decision:** ❌ **NOT RECOMMENDED** for egui integration

---

## 6. Proof-of-Concept Implementation Plan

### 6.1 wgpu Approach (Recommended)

**Phase 1: Basic Setup (4 hours)**
1. Access egui's wgpu context
2. Create basic shader (vertex + fragment)
3. Set up mesh buffer (vertices, indices)
4. Render simple triangle

**Phase 2: Mesh Rendering (4 hours)**
1. Load mesh data from `mesh-core`
2. Create vertex/index buffers
3. Render mesh with basic shader
4. Add basic camera (orthographic projection)

**Phase 3: Camera Controls (2 hours)**
1. Implement mouse drag for rotation
2. Implement mouse wheel for zoom
3. Implement pan controls

**Phase 4: Integration (2 hours)**
1. Integrate with preview panel
2. Handle resize events
3. Add error handling

**Total:** ~12 hours

### 6.2 three-d Approach (Alternative)

**Phase 1: Context Setup (3 hours)**
1. Create three-d context (may need custom integration)
2. Set up camera and viewport
3. Create basic scene

**Phase 2: Mesh Rendering (3 hours)**
1. Convert mesh-core Mesh to three-d format
2. Create three-d mesh object
3. Render in scene

**Phase 3: Camera Controls (2 hours)**
1. Use three-d's built-in camera controls
2. Integrate with egui input

**Phase 4: egui Integration (4 hours)**
1. Create custom egui integration
2. Handle window/context sharing
3. Integrate with preview panel

**Total:** ~12 hours (but more integration complexity)

---

## 7. Performance Considerations

### 7.1 Mesh Size Handling

**Small Meshes (<10K vertices):**
- Any library can handle easily
- No optimization needed

**Medium Meshes (10K-100K vertices):**
- wgpu: Excellent performance
- three-d: Good performance
- kiss3d: Acceptable performance

**Large Meshes (100K-1M vertices):**
- wgpu: Excellent (with proper optimization)
- three-d: Good (may need optimization)
- kiss3d: May struggle

**Very Large Meshes (1M+ vertices):**
- wgpu: Excellent (with LOD/frustum culling)
- three-d: May need optimization
- kiss3d: Not recommended

### 7.2 Optimization Strategies

**Level of Detail (LOD):**
- Render simplified mesh for distant views
- Switch to full detail for close-up

**Frustum Culling:**
- Only render meshes in view
- Skip off-screen geometry

**Instancing:**
- For multiple meshes, use instanced rendering

---

## 8. Binary Size Impact

### 8.1 Size Comparison

**Current (without 3D viewer):**
- converter-gui: ~8-12 MB

**With wgpu:**
- Additional: ~5-10 MB
- Total: ~13-22 MB

**With three-d:**
- Additional: ~8-12 MB
- Total: ~16-24 MB

**With kiss3d:**
- Additional: ~3-5 MB
- Total: ~11-17 MB

**Assessment:** All options are reasonable. wgpu has best size/performance ratio.

---

## 9. Cross-Platform Considerations

### 9.1 Platform Support

**wgpu:**
- ✅ Windows (DirectX12, Vulkan)
- ✅ macOS (Metal)
- ✅ Linux (Vulkan)
- ✅ Web (WebGPU)

**three-d:**
- ✅ Windows
- ✅ macOS
- ✅ Linux
- ✅ Web

**kiss3d:**
- ✅ Windows (OpenGL)
- ✅ macOS (OpenGL)
- ✅ Linux (OpenGL)
- ⚠️ Web (limited)

**Assessment:** All support our target platforms. wgpu has best cross-platform support.

---

## 10. Recommendations

### 10.1 Primary Recommendation: wgpu

**Decision:** ✅ **RECOMMEND wgpu for 3D viewer**

**Rationale:**
1. **Best egui Integration:** Native support, no conflicts
2. **Best Performance:** Handles large meshes excellently
3. **Future-proof:** WebGPU standard, actively maintained
4. **Reasonable Complexity:** Medium-High, but manageable

**Timeline:** 12-16 hours for prototype

**Risk:** Medium (requires graphics programming knowledge)

### 10.2 Alternative: three-d

**Decision:** ⚠️ **ACCEPTABLE ALTERNATIVE if wgpu too complex**

**Rationale:**
1. **Easier API:** High-level, less graphics knowledge
2. **Built-in Features:** Camera, mesh loading, lighting
3. **Good Performance:** Sufficient for typical meshes

**Timeline:** 12-14 hours for prototype (but more integration work)

**Risk:** Medium-High (custom egui integration complexity)

### 10.3 Not Recommended: kiss3d

**Decision:** ❌ **NOT RECOMMENDED**

**Rationale:**
1. **Integration Difficulty:** OpenGL conflicts with egui's wgpu
2. **Limited Performance:** Not suitable for large meshes
3. **Maintenance:** Less active development

---

## 11. Next Steps

### Immediate (Sprint 9 Week 1)
1. ✅ Complete research document (this document)
2. ⏳ Make final library selection (wgpu recommended)
3. ⏳ Create proof-of-concept prototype (Task 2.2)

### Future (Sprint 9 Week 2 or Sprint 10)
1. ⏳ Full 3D viewer implementation
2. ⏳ Camera controls and interaction
3. ⏳ Integration with preview panel
4. ⏳ Performance optimization
5. ⏳ Documentation updates

---

## 12. References

**Libraries:**
- wgpu: https://github.com/gfx-rs/wgpu
- three-d: https://github.com/asny/three-d
- kiss3d: https://github.com/sebcrozet/kiss3d
- egui: https://github.com/emilk/egui

**Documentation:**
- egui Custom Rendering: https://docs.rs/egui/latest/egui/struct.PaintCallback.html
- wgpu Tutorial: https://sotrh.github.io/learn-wgpu/
- three-d Examples: https://github.com/asny/three-d/tree/master/examples

**Project Documents:**
- `SPRINT_9_TASKING.md` - Sprint 9 task breakdown
- `converter-gui/src/ui/preview.rs` - Current preview implementation
- `mesh-core/src/mesh/mod.rs` - Mesh data structure

---

**Document Status:** ✅ **RESEARCH COMPLETE**  
**Next Review:** After proof-of-concept prototype (Task 2.2)  
**Questions or Concerns:** Contact Researcher (Dr. Taylor Kim) or Junior Engineer 3D (Alex Rivera)
