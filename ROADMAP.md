# Project Roadmap
## Simple Image Converter

**Last Updated:** January 27, 2025  
**Current Version:** v0.1.1  
**Next Release:** v0.2.0 (STEP/CAD Support)

---

## 🎯 Current Phase: v0.2.0 - STEP/CAD Support

### Status Overview

**Current Status:** 🚧 **Framework Complete, Entity Conversion In Progress**

**Completed:**
- ✅ STEP file parsing (ruststep 0.4.0 with AP203 feature)
- ✅ Entity extraction framework
- ✅ Entity type identification (MANIFOLD_SOLID_BREP, CLOSED_SHELL, etc.)
- ✅ Code structure and error handling
- ✅ Dependencies integrated (ruststep, truck-meshalgo)

**In Progress:**
- 🚧 STEP entity → truck Shell conversion logic
- ⏳ Tessellation implementation (pending entity conversion)

**Pending:**
- ⏳ Testing with real STEP files
- ⏳ Documentation updates

---

## 🔥 High Priority - Immediate Next Steps

### 1. Research ruststep Tables API (Critical Path)

**Objective:** Understand how to build AP203 `Tables` from `Exchange.data` and deserialize entities.

**Tasks:**
- [ ] Explore ruststep's Tables API structure
- [ ] Understand AP203 type deserialization patterns
- [ ] Learn reference resolution mechanisms (#1, #2, etc.)
- [ ] Write experimental code to test API usage

**Resources:**
- `ruststep` v0.4.0 documentation (with `ap203` feature)
- ruststep GitHub repository for examples
- STEP_IMPLEMENTATION_CURRENT_STATE.md (current implementation details)

**Estimated Effort:** 1-2 days

### 2. Research truck Shell Construction APIs

**Objective:** Learn how to build `Shell` objects from geometric primitives in truck.

**Tasks:**
- [ ] Review truck Shell/Solid construction APIs
- [ ] Understand face/edge/vertex construction patterns
- [ ] Learn coordinate system handling in truck
- [ ] Explore curve and surface types in truck

**Resources:**
- `truck-modeling` v0.3.0 documentation
- `truck-topology` documentation
- truck GitHub repository

**Estimated Effort:** 1-2 days

### 3. Implement STEP Entity → truck Shell Conversion

**Objective:** Convert AP203 entities (MANIFOLD_SOLID_BREP, CLOSED_SHELL, etc.) to truck Shell objects.

**Approach:**
1. Build AP203 `Tables` from `Exchange.data`
2. Deserialize STEP `Record`s into AP203 structs using serde
3. Resolve entity references using Tables
4. Convert AP203 geometric types to truck Shell
5. Handle coordinate transformations
6. Reconstruct BREP topology (faces, edges, vertices)

**Strategy:**
- Start with simpler entity types (e.g., `FACETED_BREP` - already triangulated)
- Progress to complex BREP entities
- Incremental implementation with testing

**Estimated Effort:** 1-2 weeks

### 4. Implement Tessellation

**Objective:** Convert truck Shell objects to polygonal meshes using truck-meshalgo.

**Tasks:**
- [ ] Implement `convert_truck_to_mesh()` function
- [ ] Use `truck-meshalgo::MeshableShape::triangulation()` method
- [ ] Extract `PolygonMesh` from tessellated Shell faces
- [ ] Convert to our `Mesh` format with vertices, faces, normals
- [ ] Handle multiple shells (merge into single mesh)

**Estimated Effort:** 2-3 days

### 5. Testing & Validation

**Objective:** Validate STEP conversion with real-world files.

**Tasks:**
- [ ] Collect test STEP files (various complexities)
- [ ] Create comprehensive test suite
- [ ] Validate conversion correctness
- [ ] Performance testing
- [ ] Error handling validation

**Estimated Effort:** 1 week

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
- ✅ Can convert STEP entities to truck Shell types
- ✅ Can tessellate Shell objects to meshes
- ✅ Can convert to target mesh formats (STL, OBJ, PLY)
- ✅ Comprehensive test coverage
- ✅ Documentation updated

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

