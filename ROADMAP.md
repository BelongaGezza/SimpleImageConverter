# Project Roadmap
## Simple Image Converter

**Last Updated:** January 29, 2025
**Current Version:** v0.1.1
**Next Release:** v0.2.0 (STEP/CAD Support)

---

## 🎯 Current Phase: v0.2.0 - STEP/CAD Support

### Status Overview

**Current Status:** ⚠️ **ARCHITECTURE DECISION REQUIRED**

**Completed:**
- ✅ STEP file parsing (ruststep 0.4.0 with AP203 feature)
- ✅ Entity extraction framework
- ✅ Entity type identification (MANIFOLD_SOLID_BREP, CLOSED_SHELL, etc.)
- ✅ Code structure and error handling
- ✅ Dependencies integrated (ruststep, truck-meshalgo)
- ✅ Research documentation (Sam - comprehensive)
- ✅ **Tables population via `TableInit::from_data_sections()`** (Riley - COMPLETE)
- ✅ **Entity deserialization via `_holders()` methods** (Riley - COMPLETE)
- ✅ **Reference resolution via `IntoOwned` trait** (Riley - COMPLETE)

**Blocked:**
- ❌ **CRITICAL:** truck-stepio input functionality **does not exist** (v0.3.0)
- ❌ AP203 → truck Shell conversion requires custom implementation or alternative approach

**Architecture Decision Required:**
- See "STEP Implementation Options" section below for detailed analysis

**Pending (after architecture decision):**
- ⏳ Implement chosen approach (FACETED_BREP or opencascade-rs)
- ⏳ Testing with real STEP files
- ⏳ Complete documentation updates

---

## 🔴 CRITICAL: STEP Implementation Options

### The Problem

We have successfully parsed STEP files and deserialized AP203 entities using ruststep. However, **truck-stepio does not have input (reading) functionality** - only output is implemented. This blocks conversion from AP203 entities to truck Shell for tessellation.

### Research Finding: STEPToMesh Approach

Analysis of [STEPToMesh](https://github.com/aleutgeb/STEPToMesh) (C++ project) revealed the standard approach:

1. **OpenCASCADE** handles both STEP reading AND tessellation
2. `STEPCAFControl_Reader` reads STEP files
3. `BRepMesh_IncrementalMesh` tessellates curved surfaces (NURBS, cylinders, etc.)
4. No separate "bridge" library needed - OCCT does everything

### Available Options

| Option | Curved Surfaces | Effort | New Dependencies | Recommendation |
|--------|----------------|--------|------------------|----------------|
| **A: FACETED_BREP only** | ❌ No | 1-2 weeks | None | **v0.2.0** |
| **B: opencascade-rs** | ✅ Yes | 2-4 weeks | OCCT C++ library | **v0.3.0** |
| **C: Custom AP203→truck** | ✅ Yes | Months | None | Not recommended |
| **D: Wait for truck-stepio** | ✅ Yes | Unknown | None | Uncertain timeline |

### Option A: FACETED_BREP Only (Recommended for v0.2.0)

**What it does:**
- Supports STEP files with pre-tessellated geometry (FACETED_BREP entities)
- Extracts vertices/faces directly from AP203 structs
- Skips truck Shell entirely - builds our Mesh directly

**Limitations:**
- Only works with STEP files exported with tessellation option
- No support for curved surfaces (NURBS, cylinders, spheres)
- Many CAD tools can export FACETED_BREP format

**Implementation path:**
```
STEP File → ruststep → Tables → FACETED_BREP entities → Extract vertices → Mesh
```

### Option B: opencascade-rs (Recommended for v0.3.0)

**What it does:**
- Uses [opencascade-rs](https://github.com/bschwind/opencascade-rs) Rust bindings
- Full OpenCASCADE kernel for STEP reading AND tessellation
- `BRepMesh_IncrementalMesh` handles all curved surface types

**Available APIs (verified in source):**
```rust
// STEP Reading (from opencascade-sys)
type STEPControl_Reader;
fn read_step(reader: &mut STEPControl_Reader, filename: String) -> IFSelect_ReturnStatus;
fn one_shape_step(reader: &STEPControl_Reader) -> UniquePtr<TopoDS_Shape>;

// Tessellation (from opencascade/src/mesh.rs)
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub normals: Vec<DVec3>,
    pub indices: Vec<usize>,
}
pub struct Mesher; // Wraps BRepMesh_IncrementalMesh
```

**Trade-offs:**
- ✅ Full curved surface support
- ✅ Industry-standard OCCT kernel
- ❌ Adds C++ dependency (OpenCASCADE ~100MB)
- ❌ opencascade-rs is "work in progress" (but functional)

**Implementation path:**
```
STEP File → STEPControl_Reader → TopoDS_Shape → BRepMesh → Mesh
```

### Decision Matrix

| Criteria | FACETED_BREP | opencascade-rs |
|----------|--------------|----------------|
| Ships v0.2.0 on time | ✅ Yes | ❌ Delays release |
| Curved surface support | ❌ No | ✅ Yes |
| Pure Rust | ✅ Yes | ❌ No (C++ dep) |
| Implementation risk | Low | Medium |
| Future maintenance | Simple | More complex |

### Recommended Strategy

1. **v0.2.0:** Implement FACETED_BREP extraction (Option A)
   - Ships working STEP support quickly
   - Document limitation clearly
   - Useful for many CAD exports

2. **v0.3.0:** Add opencascade-rs backend (Option B)
   - Full curved surface support
   - Can coexist with FACETED_BREP path
   - Feature-gated to keep pure-Rust option available

### Team Assignments

**Riley Thompson (Junior Engineer, 3D Formats):**
- **Status:** ✅ Completed Tasks 2.1-2.3 (Tables, Deserialization, References)
- **Current:** Awaiting architecture decision on STEP approach
- **Next Task:** Implement FACETED_BREP extraction (if Option A approved)
- **Progress Document:** `RILEY_IMPLEMENTATION_PROGRESS.md`
- **Grade:** B+ (Excellent work discovering correct APIs)

**Sam Parker (Junior Engineer, 2D Formats):**
- **Status:** Research support complete
- **Current:** Documentation updates
- **Collaboration:** Sam's Tables API research was foundational; Riley discovered correct API

**Senior Engineer (Jordan Rivera):**
- **Current:** Architecture decision required
- **Review Document:** `SENIOR_ENGINEER_CRITICAL_REVIEW_STEP_IMPLEMENTATION.md`

**Coordination:**
- See `TASK_ASSIGNMENTS_V0.2.0.md` for team coordination details
- See `SENIOR_ENGINEER_CRITICAL_REVIEW_STEP_IMPLEMENTATION.md` for latest review

---

## 🔥 High Priority - Immediate Next Steps

### 1. Architecture Decision (BLOCKING)

**Status:** ⏳ Awaiting decision

**Options:**
- **Option A:** FACETED_BREP only (v0.2.0) - Recommended
- **Option B:** opencascade-rs (v0.3.0) - For curved surfaces

**Decision Required By:** Project Owner / System Architect

### 2. Implement FACETED_BREP Extraction (After Decision)

**Objective:** Extract pre-tessellated geometry from STEP files.

**Tasks:**
- [ ] Check if `tables.faceted_brep_holders()` exists
- [ ] Traverse entity tree: FACETED_BREP → CLOSED_SHELL → FACE → EDGE_LOOP → VERTEX_POINT → CARTESIAN_POINT
- [ ] Extract vertex coordinates from CARTESIAN_POINT entities
- [ ] Build Face indices from EDGE_LOOP structure
- [ ] Calculate normals from face vertices
- [ ] Convert to our Mesh format

**Implementation Path:**
```
FACETED_BREP
  └── outer: CLOSED_SHELL
      └── cfs_faces: [FACE, ...]
          └── bounds: [FACE_BOUND]
              └── bound: EDGE_LOOP
                  └── edge_list: [ORIENTED_EDGE, ...]
                      └── edge_element: EDGE
                          └── edge_start/end: VERTEX_POINT
                              └── vertex_geometry: CARTESIAN_POINT (x, y, z)
```

**Estimated Effort:** 1-2 weeks

### 3. Document STEP Limitations

**Objective:** Clear user documentation about STEP support scope.

**Tasks:**
- [ ] Update `docs/FORMATS.md` with STEP limitations
- [ ] Add examples of CAD export settings for FACETED_BREP
- [ ] Document which CAD tools support tessellated STEP export
- [ ] Add troubleshooting guide for unsupported STEP files

**Estimated Effort:** 1-2 days

### 4. Testing & Validation

**Objective:** Validate FACETED_BREP conversion with real-world files.

**Tasks:**
- [ ] Collect test STEP files with FACETED_BREP entities
- [ ] Create test suite for vertex/face extraction
- [ ] Validate conversion correctness
- [ ] Test error handling for non-FACETED_BREP files

**Estimated Effort:** 3-5 days

### 5. (v0.3.0) Prototype opencascade-rs Integration

**Objective:** Proof-of-concept for full curved surface support.

**Tasks:**
- [ ] Add opencascade-rs as optional dependency
- [ ] Create minimal STEP → Mesh test
- [ ] Evaluate build complexity (OCCT dependency)
- [ ] Document integration approach

**Estimated Effort:** 1 week (research/prototype)

---

## 📋 Medium Priority - v0.2.0 Completion

### CAD Format Improvements
- [ ] Enhance DXF support with additional entity types
- [ ] Improve CAD-specific validations
- [ ] Add CAD metadata preservation (if feasible)

### Documentation
- [ ] Update README.md with STEP format support status
- [ ] Add STEP format documentation
- [ ] Update CHANGELOG.md

---

## 🔮 Future Phases

### v0.3.0 - GUI Implementation
- Desktop GUI using egui
- Drag-and-drop file conversion
- Real-time preview
- See `GUI_DESIGN_AND_IMPLEMENTATION.md` for details

### v0.4.0+ - Additional Formats
- Additional 2D image formats
- Additional 3D mesh formats
- Enhanced conversion options

---

## 📁 Reference Documents

### Current Implementation
- `TASKS_SENIOR_ENGINEER_V0.2.0.md` - Detailed v0.2.0 task breakdown
- `STEP_IMPLEMENTATION_CURRENT_STATE.md` - Current STEP implementation status

### Planning & Research
- `V0.2.0_PHASE_PLAN.md` - Full v0.2.0 phase plan
- `V0.2.0_RESEARCH_FINDINGS.md` - Research results and recommendations
- `V0.2.0_STEP_READING_RESEARCH.md` - STEP reading research notes

### Architecture & Design
- `docs/ARCHITECTURE.md` - System architecture
- `docs/FORMATS.md` - Format support details
- `GUI_DESIGN_AND_IMPLEMENTATION.md` - GUI design plan

---

## 🎯 Success Criteria for v0.2.0

- ✅ Can parse STEP files successfully
- ✅ Can extract geometric data from STEP files
- 🚧 **IN PROGRESS:** Can convert STEP entities to truck Shell types (Riley - 20% complete, Tables population blocking)
- ⏳ Can tessellate Shell objects to meshes (pending entity conversion)
- ⏳ Can convert to target mesh formats (STL, OBJ, PLY) (pending conversion)
- ⏳ Comprehensive test coverage (pending implementation)
- 🚧 Documentation updated (Sam - partial, in progress)

**Critical Path:** Tables population → Entity deserialization → Shell conversion → Tessellation

**Current Blocker:** Tables population API needs research (Riley + Sam collaboration needed)

---

## 📝 Notes

- STEP entity conversion is complex and requires deep understanding of:
  - STEP entity semantics (ISO 10303 standard)
  - AP203 structure and types
  - truck geometry construction APIs
  - BREP topology (faces, edges, vertices, curves, surfaces)

- Incremental progress is expected - this is a complex domain.

- The hybrid approach (ruststep for parsing + truck for geometry) has been validated as feasible.

---

*This roadmap is a living document and will be updated as progress is made.*


