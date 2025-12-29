# Task Assignment - Riley Thompson (Junior Engineer, 3D Formats)
## v0.2.0 STEP Implementation

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Engineer:** Riley Thompson  
**Specialization:** 3D Mesh Formats  
**Priority:** 🔥 **HIGH** - Critical Path for v0.2.0 Release

---

## Overview

You are assigned to complete the STEP format implementation for v0.2.0. The framework is already in place, but the core entity conversion logic needs to be implemented. This is a complex task requiring research, experimentation, and careful implementation.

**Current Status:**
- ✅ STEP file parsing working (ruststep 0.4.0)
- ✅ Entity extraction framework complete
- ✅ Entity type identification working
- 🚧 **STEP entity → truck Shell conversion** - **YOUR PRIMARY TASK**
- ⏳ Tessellation - **YOUR SECONDARY TASK**

---

## Your Responsibilities

### Primary Task: STEP Entity → truck Shell Conversion

**Location:** `mesh-core/src/formats/step.rs`  
**Method:** `try_extract_shell()` (currently returns `None` for all entities)

**Objective:** Convert STEP entities (MANIFOLD_SOLID_BREP, CLOSED_SHELL, etc.) to truck `Shell` objects.

**Current Code State:**
```rust
fn try_extract_shell(&self, record: &ast::Record) -> Result<Option<Shell>> {
    let entity_name = &record.name;
    match entity_name.as_str() {
        "MANIFOLD_SOLID_BREP" => {
            // TODO: Extract closed_shell reference and convert to Shell
            Ok(None)  // ← YOU NEED TO IMPLEMENT THIS
        }
        "CLOSED_SHELL" => {
            // TODO: Extract faces and convert to truck Shell
            Ok(None)  // ← YOU NEED TO IMPLEMENT THIS
        }
        // ... other entity types
    }
}
```

### Secondary Task: Tessellation Implementation

**Location:** `mesh-core/src/formats/step.rs`  
**Method:** `convert_truck_to_mesh()` (currently returns error)

**Objective:** Convert truck `Shell` objects to our `Mesh` format using truck-meshalgo.

**Current Code State:**
```rust
fn convert_truck_to_mesh(&self, _shells: Vec<Shell>) -> Result<Mesh> {
    // TODO: Implement tessellation using truck-meshalgo
    Err(ConversionError::ConversionFailed(...))  // ← YOU NEED TO IMPLEMENT THIS
}
```

---

## Task Breakdown

### Phase 1: Research (Days 1-2) 🔍

**Task 1.1: Research ruststep Tables API**

**Objective:** Understand how to build AP203 `Tables` from `Exchange.data` and deserialize entities.

**What to Research:**
- [ ] Explore ruststep's `Tables` API structure
- [ ] Understand AP203 type deserialization patterns
- [ ] Learn reference resolution mechanisms (#1, #2, etc.)
- [ ] Review ruststep documentation and examples
- [ ] Check ruststep GitHub repository for usage examples

**Resources:**
- `ruststep` v0.4.0 documentation (with `ap203` feature enabled)
- ruststep GitHub: https://github.com/ricosjp/ruststep
- `STEP_IMPLEMENTATION_CURRENT_STATE.md` (implementation notes)
- `V0.2.0_STEP_READING_RESEARCH.md` (research notes)

**Deliverable:** 
- Document your findings in a research notes file
- Create a small experimental code snippet demonstrating Tables API usage
- Share findings with Senior Engineer for review

**Task 1.2: Research truck Shell Construction APIs**

**Objective:** Learn how to build `Shell` objects from geometric primitives in truck.

**What to Research:**
- [ ] Review truck Shell/Solid construction APIs
- [ ] Understand face/edge/vertex construction patterns
- [ ] Learn coordinate system handling in truck
- [ ] Explore curve and surface types in truck
- [ ] Review truck-meshalgo tessellation API

**Resources:**
- `truck-modeling` v0.3.0 documentation
- `truck-topology` documentation
- `truck-meshalgo` documentation
- truck GitHub: https://github.com/ricosjp/truck

**Deliverable:**
- Document your findings in research notes
- Create experimental code snippets showing Shell construction
- Share findings with Senior Engineer for review

**Estimated Effort:** 1-2 days total

---

### Phase 2: Implementation - Entity Conversion (Days 3-10) 💻

**Task 2.1: Build AP203 Tables from Exchange.data**

**Objective:** Create the Tables structure needed for entity deserialization.

**Implementation Steps:**
1. [ ] Build AP203 `Tables` from `Exchange.data`
2. [ ] Populate tables with entity instances
3. [ ] Test with simple STEP files
4. [ ] Verify reference resolution works

**Code Location:** `mesh-core/src/formats/step.rs` - `parse_step()` method

**Task 2.2: Deserialize STEP Entities**

**Objective:** Convert STEP `Record`s into AP203 structs using serde.

**Implementation Steps:**
1. [ ] Deserialize MANIFOLD_SOLID_BREP entities
2. [ ] Deserialize CLOSED_SHELL entities
3. [ ] Deserialize FACE entities
4. [ ] Handle other entity types as needed
5. [ ] Add error handling for deserialization failures

**Code Location:** `mesh-core/src/formats/step.rs` - `try_extract_shell()` method

**Task 2.3: Resolve Entity References**

**Objective:** Resolve STEP entity references (#1, #2, etc.) using Tables.

**Implementation Steps:**
1. [ ] Implement reference resolution for closed_shell references
2. [ ] Implement reference resolution for face references
3. [ ] Implement reference resolution for edge/vertex references
4. [ ] Handle circular references and validation
5. [ ] Add error handling for missing references

**Code Location:** `mesh-core/src/formats/step.rs` - `try_extract_shell()` method

**Task 2.4: Convert AP203 Types to truck Shell**

**Objective:** Convert AP203 geometric types to truck Shell objects.

**Implementation Strategy:**
- Start with simpler entity types (e.g., `FACETED_BREP` - already triangulated)
- Progress to complex BREP entities
- Incremental implementation with testing

**Implementation Steps:**
1. [ ] Convert CLOSED_SHELL to truck Shell
   - Extract faces from ClosedShell
   - Convert each face to truck Face
   - Build truck Shell from faces
2. [ ] Handle coordinate transformations
3. [ ] Reconstruct BREP topology (faces, edges, vertices)
4. [ ] Handle curves and surfaces
5. [ ] Test with various STEP files

**Code Location:** `mesh-core/src/formats/step.rs` - `try_extract_shell()` method

**Estimated Effort:** 1-2 weeks

---

### Phase 3: Implementation - Tessellation (Days 11-13) 🔺

**Task 3.1: Implement Tessellation**

**Objective:** Convert truck Shell objects to polygonal meshes using truck-meshalgo.

**Implementation Steps:**
1. [ ] Implement `convert_truck_to_mesh()` function
2. [ ] Use `truck-meshalgo::MeshableShape::triangulation()` method
3. [ ] Extract `PolygonMesh` from tessellated Shell faces
4. [ ] Convert to our `Mesh` format with vertices, faces, normals
5. [ ] Handle multiple shells (merge into single mesh)
6. [ ] Configure appropriate tolerance settings
7. [ ] Handle tessellation errors gracefully

**Code Location:** `mesh-core/src/formats/step.rs` - `convert_truck_to_mesh()` method

**Implementation Outline:**
```rust
// 1. For each shell: shell.triangulation(tolerance) -> Shell<Point3, PolylineCurve, Option<PolygonMesh>>
// 2. Iterate through shell faces, extract Option<PolygonMesh> from each surface
// 3. Collect all PolygonMeshes and merge them into a single mesh
// 4. Convert to our Mesh format with vertices, faces, and normals
```

**Estimated Effort:** 2-3 days

---

### Phase 4: Testing & Validation (Days 14-18) ✅

**Task 4.1: Unit Tests**

**Objective:** Create comprehensive unit tests for your implementation.

**Test Coverage:**
- [ ] Test AP203 Tables construction
- [ ] Test entity deserialization
- [ ] Test reference resolution
- [ ] Test entity conversion to truck Shell
- [ ] Test tessellation
- [ ] Test error handling

**Task 4.2: Integration Tests**

**Objective:** Test with real STEP files.

**Test Files Needed:**
- [ ] Collect test STEP files (various complexities)
- [ ] Small files (<10MB)
- [ ] Medium files (10-100MB)
- [ ] Files with different entity types

**Test Scenarios:**
- [ ] STEP → STL conversion
- [ ] STEP → OBJ conversion
- [ ] STEP → PLY conversion
- [ ] Validate conversion correctness
- [ ] Performance testing
- [ ] Error handling validation

**Task 4.3: Edge Case Testing**

**Objective:** Validate error handling and edge cases.

**Test Cases:**
- [ ] Invalid STEP files
- [ ] Unsupported entity types
- [ ] Missing references
- [ ] Resource limits enforcement
- [ ] Memory usage validation

**Estimated Effort:** 1 week

---

## Code Quality Standards

Follow these standards (as per Senior Engineer guidelines):

### Error Handling
```rust
// Always use Result for fallible operations
pub fn try_extract_shell(&self, record: &ast::Record) -> Result<Option<Shell>> {
    // Validate first
    // Implementation with proper error mapping
}
```

### Testing
- Write tests alongside implementations
- Test edge cases
- Test error conditions
- Aim for ≥80% code coverage

### Documentation
- Document public APIs
- Add inline comments for complex logic
- Document assumptions and limitations
- Update `STEP_IMPLEMENTATION_CURRENT_STATE.md` as you progress

### Code Style
- Follow existing code patterns in `mesh-core/src/formats/`
- Use consistent error types from `common::error`
- Respect resource limits from `common::limits`
- Follow Rust idioms and best practices

---

## Questions & Review Process

### When to Ask Questions

**Ask immediately if:**
- You encounter blockers that prevent progress
- You find API documentation doesn't match actual behavior
- You need clarification on architecture decisions
- You're unsure about error handling approach
- You need help understanding STEP entity semantics

**Ask before implementing if:**
- You're considering a significant architectural change
- You need to add new dependencies
- You're unsure about the best approach for a complex problem

### Review Process

1. **Research Phase Review:**
   - Share research findings with Senior Engineer
   - Get feedback on approach before implementation
   - Validate experimental code snippets

2. **Implementation Reviews:**
   - Submit code for review after each major milestone:
     - After Task 2.1 (Tables construction)
     - After Task 2.4 (Entity conversion)
     - After Task 3.1 (Tessellation)
   - Request review when you have questions
   - Don't wait until everything is done

3. **Final Review:**
   - Submit complete implementation for final review
   - Include all tests
   - Include documentation updates

### Communication Style

- **Be proactive:** Share progress and challenges early
- **Be specific:** When asking questions, provide context and code examples
- **Document learnings:** Update research notes as you discover things
- **Ask for help:** Don't struggle in silence - this is complex work!

---

## Reference Documents

### Implementation Context
- `STEP_IMPLEMENTATION_CURRENT_STATE.md` - Current implementation status
- `mesh-core/src/formats/step.rs` - Current code (your starting point)
- `TASKS_SENIOR_ENGINEER_V0.2.0.md` - Overall phase plan

### Planning & Research
- `V0.2.0_PHASE_PLAN.md` - Full v0.2.0 phase plan
- `V0.2.0_RESEARCH_FINDINGS.md` - Research results
- `V0.2.0_STEP_READING_RESEARCH.md` - STEP reading research notes

### Architecture
- `docs/ARCHITECTURE.md` - System architecture
- `docs/FORMATS.md` - Format support details
- `Phase3_Architecture.md` - Implementation patterns

### External Resources
- [STEP Format Specification](https://www.iso.org/standard/72658.html)
- [ruststep Documentation](https://docs.rs/ruststep/)
- [ruststep GitHub](https://github.com/ricosjp/ruststep)
- [truck Library Documentation](https://github.com/ricosjp/truck)
- [truck-meshalgo API](https://docs.rs/truck-meshalgo/)

---

## Success Criteria

### Must Have (v0.2.0 MVP)

- ✅ Can parse STEP files successfully (already done)
- ✅ Can extract geometric data from STEP files (framework done)
- 🎯 **Can convert STEP entities to truck Shell types** ← **YOUR PRIMARY GOAL**
- 🎯 **Can tessellate Shell objects to meshes** ← **YOUR SECONDARY GOAL**
- 🎯 Can convert to target mesh formats (STL, OBJ, PLY) ← **YOUR TERTIARY GOAL**
- 🎯 Comprehensive test coverage (≥80%) ← **YOUR RESPONSIBILITY**
- 🎯 Documentation updated ← **YOUR RESPONSIBILITY**

---

## Timeline

**Estimated Duration:** 2-3 weeks

- **Week 1:** Research + Entity Conversion (Phases 1-2)
- **Week 2:** Tessellation + Initial Testing (Phase 3)
- **Week 3:** Comprehensive Testing + Documentation (Phase 4)

**Milestones:**
- **End of Week 1:** Entity conversion working for at least one entity type
- **End of Week 2:** Full conversion pipeline working (STEP → Mesh)
- **End of Week 3:** Tests passing, documentation complete, ready for review

---

## Notes

### Complexity Acknowledgment

This is a **complex task** requiring:
- Deep understanding of STEP entity semantics (ISO 10303 standard)
- Understanding of AP203 structure and types
- Understanding of truck geometry construction APIs
- BREP topology knowledge (faces, edges, vertices, curves, surfaces)

**It's okay to:**
- Take time to understand the concepts
- Ask questions frequently
- Start with simpler cases and expand
- Make incremental progress
- Document challenges and learnings

### Incremental Approach Recommended

1. **Start Simple:** Begin with FACETED_BREP (already triangulated)
2. **Expand Gradually:** Add CLOSED_SHELL, then MANIFOLD_SOLID_BREP
3. **Test Frequently:** Test each step with real STEP files
4. **Document Progress:** Update status as you go

---

## Getting Started

1. **Read the Context:**
   - Review `STEP_IMPLEMENTATION_CURRENT_STATE.md`
   - Review `mesh-core/src/formats/step.rs`
   - Understand the current code structure

2. **Start Research:**
   - Begin with Task 1.1 (ruststep Tables API)
   - Create experimental code snippets
   - Document your findings

3. **Begin Implementation:**
   - Start with Task 2.1 (Tables construction)
   - Work incrementally
   - Test frequently

4. **Communicate:**
   - Share progress regularly
   - Ask questions early
   - Request reviews at milestones

---

**Status:** 🚀 **READY TO BEGIN**  
**Priority:** 🔥 **HIGH** - Critical Path for v0.2.0  
**Support:** Senior Engineer available for questions and reviews

**Good luck, Riley! This is challenging but important work. Don't hesitate to ask questions.**

---

*Last Updated: January 27, 2025*  
*Assigned By: Jordan Rivera (Senior Engineer)*

