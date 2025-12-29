# Task Assignment - Riley Thompson (Junior Engineer, 3D Formats)
## v0.2.0 STEP Implementation - Phase 2 (URGENT)

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Engineer:** Riley Thompson  
**Priority:** 🔥 **CRITICAL - IMMEDIATE ACTION REQUIRED**  
**Status:** ⚠️ **RESEARCH COMPLETE, IMPLEMENTATION MUST START NOW**

---

## Critical Review Summary

**Previous Assessment:**
- ✅ Research notes created
- ❌ **ZERO implementation progress**
- ❌ No code changes to `step.rs`
- ❌ All TODOs still present

**Action Required:** **START IMPLEMENTATION IMMEDIATELY**

---

## Phase 2: Implementation (URGENT)

### Objective

**You must start actual implementation NOW.** Research is good, but working code is the deliverable.

### Immediate Goals (This Week)

**Target:** Get at least ONE entity type working end-to-end (STEP → Mesh)

---

## Task 2.1: Build AP203 Tables (Days 1-2) 🔥 CRITICAL

**Objective:** Create working code to build AP203 Tables from Exchange.data

**Deliverable:** Working code that builds Tables structure

**Steps:**
1. [ ] Study ruststep AP203 module structure
2. [ ] Create experimental code to build Tables
3. [ ] Test with simple STEP file
4. [ ] Verify Tables structure is correct
5. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - `parse_step()` method

**Expected Pattern:**
```rust
use ruststep::ap203::config_control_design::Tables;

// In parse_step(), after parsing:
let mut tables = Tables::default();
// Build tables from exchange.data
// Populate with entity instances
```

**Success Criteria:**
- ✅ Code compiles
- ✅ Tables structure created successfully
- ✅ Can access entity instances from Tables
- ✅ Test with simple STEP file passes

**Estimated Effort:** 1-2 days

---

## Task 2.2: Deserialize ONE Entity Type (Days 2-3) 🔥 CRITICAL

**Objective:** Deserialize at least ONE STEP entity type into AP203 struct

**Strategy:** Start with simplest entity type (e.g., FACETED_BREP or CARTESIAN_POINT)

**Deliverable:** Working code that deserializes one entity type

**Steps:**
1. [ ] Choose simplest entity type to start with
2. [ ] Study AP203 struct for that entity type
3. [ ] Create deserialization code
4. [ ] Test with simple STEP file containing that entity
5. [ ] Verify deserialization works
6. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - `try_extract_shell()` method

**Expected Pattern:**
```rust
use ruststep::ap203::config_control_design::SomeEntityType;

// In try_extract_shell():
if record.name == "SOME_ENTITY_TYPE" {
    // Deserialize record into AP203 struct
    let entity: SomeEntityType = /* deserialize */;
    // Use entity data
}
```

**Success Criteria:**
- ✅ Can deserialize one entity type successfully
- ✅ Can access entity fields
- ✅ Test passes with simple STEP file
- ✅ Code committed

**Estimated Effort:** 1 day

---

## Task 2.3: Resolve Entity References (Days 3-4) 🔥 CRITICAL

**Objective:** Implement reference resolution (#1, #2, etc.) using Tables

**Deliverable:** Working code that resolves entity references

**Steps:**
1. [ ] Understand how references work in ruststep
2. [ ] Implement reference resolution function
3. [ ] Test with STEP file containing references
4. [ ] Verify references resolve correctly
5. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - Helper function

**Expected Pattern:**
```rust
fn resolve_reference<T>(
    ref_id: EntityId,
    tables: &Tables,
) -> Result<&T> {
    // Look up entity in tables
    // Return reference
}
```

**Success Criteria:**
- ✅ Can resolve entity references
- ✅ Handles missing references gracefully
- ✅ Test passes
- ✅ Code committed

**Estimated Effort:** 1 day

---

## Task 2.4: Convert ONE Entity to truck Shell (Days 4-5) 🔥 CRITICAL

**Objective:** Convert at least ONE STEP entity type to truck Shell

**Strategy:** Start with simplest entity (e.g., FACETED_BREP if available, or simple CLOSED_SHELL)

**Deliverable:** Working code that converts one entity type to truck Shell

**Steps:**
1. [ ] Study truck Shell construction API
2. [ ] Choose entity type to convert
3. [ ] Implement conversion logic
4. [ ] Test with simple STEP file
5. [ ] Verify Shell is created correctly
6. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - `try_extract_shell()` method

**Expected Pattern:**
```rust
fn convert_entity_to_shell(
    entity: &SomeEntityType,
    tables: &Tables,
) -> Result<Shell> {
    // Extract geometry from entity
    // Build truck Shell
    // Return Shell
}
```

**Success Criteria:**
- ✅ Can convert one entity type to truck Shell
- ✅ Shell is valid (can be tessellated)
- ✅ Test passes
- ✅ Code committed

**Estimated Effort:** 1-2 days

---

## Task 2.5: Basic Tessellation (Days 5-6) 🔥 CRITICAL

**Objective:** Implement basic tessellation for truck Shell

**Deliverable:** Working code that tessellates Shell to Mesh

**Steps:**
1. [ ] Study truck-meshalgo tessellation API
2. [ ] Implement `convert_truck_to_mesh()` function
3. [ ] Test with simple Shell
4. [ ] Verify Mesh is created correctly
5. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - `convert_truck_to_mesh()` method

**Expected Pattern:**
```rust
fn convert_truck_to_mesh(&self, shells: Vec<Shell>) -> Result<Mesh> {
    // For each shell:
    //   - Tessellate using truck-meshalgo
    //   - Extract PolygonMesh
    //   - Convert to our Mesh format
    // Return combined Mesh
}
```

**Success Criteria:**
- ✅ Can tessellate Shell to Mesh
- ✅ Mesh has vertices and faces
- ✅ Test passes
- ✅ Code committed

**Estimated Effort:** 1 day

---

## Task 2.6: End-to-End Test (Day 6) ✅ VALIDATION

**Objective:** Test complete pipeline with simple STEP file

**Deliverable:** Working end-to-end conversion (STEP → Mesh)

**Steps:**
1. [ ] Create or find simple STEP file
2. [ ] Test complete conversion pipeline
3. [ ] Verify output Mesh is correct
4. [ ] Fix any issues
5. [ ] Commit working code

**Success Criteria:**
- ✅ Simple STEP file converts successfully
- ✅ Output Mesh is valid
- ✅ Can convert to STL/OBJ/PLY
- ✅ Test passes

**Estimated Effort:** 1 day

---

## Weekly Milestones

### End of Week 1 (Critical)

**Must Have:**
- ✅ AP203 Tables construction working
- ✅ At least ONE entity type deserialized
- ✅ Reference resolution working
- ✅ At least ONE entity type converted to truck Shell
- ✅ Basic tessellation working
- ✅ Simple STEP file converts end-to-end

**Success Criteria:**
- ✅ Code compiles and runs
- ✅ At least one simple STEP file converts successfully
- ✅ All code committed to repository

---

## Implementation Strategy

### Incremental Approach

1. **Start Simple:**
   - Begin with simplest entity types
   - Get ONE thing working end-to-end
   - Then expand to more entity types

2. **Test Frequently:**
   - Test after each step
   - Use simple STEP files
   - Verify each component works

3. **Commit Often:**
   - Commit working code frequently
   - Don't wait until everything is done
   - Make progress visible

### Getting Help

**When to Ask:**
- If blocked for more than 2 hours
- If API doesn't work as expected
- If you need clarification on approach

**Resources:**
- Sam's research documents (starting point)
- ruststep documentation
- truck documentation
- Senior Engineer (for questions)

---

## Code Quality Requirements

### Must Follow

1. **Error Handling:**
   - All functions return `Result`
   - Clear error messages
   - Handle edge cases

2. **Testing:**
   - Write tests for each component
   - Test with simple STEP files
   - Verify correctness

3. **Documentation:**
   - Comment complex logic
   - Document assumptions
   - Update implementation status

4. **Code Style:**
   - Follow existing patterns
   - Use consistent error types
   - Respect resource limits

---

## Critical Success Factors

### What Success Looks Like

**By End of Week 1:**
- ✅ Simple STEP file (e.g., cube) converts to mesh
- ✅ Can output to STL/OBJ/PLY
- ✅ Code is working and committed
- ✅ Basic functionality demonstrated

**This is the MINIMUM viable deliverable.**

---

## Questions & Support

### Immediate Support Available

- **Senior Engineer:** Available for questions and code reviews
- **Sam:** Can help verify research patterns
- **Resources:** Research documents, documentation

### Communication

- **Daily Updates:** Share progress daily
- **Blockers:** Report immediately
- **Questions:** Ask early, don't struggle alone

---

## Timeline

**Week 1 (Current):**
- Days 1-2: Tables construction
- Days 2-3: Entity deserialization
- Days 3-4: Reference resolution
- Days 4-5: Entity to Shell conversion
- Days 5-6: Tessellation
- Day 6: End-to-end test

**Target:** Working end-to-end conversion by end of week

---

## Final Notes

**CRITICAL:** Implementation must start **NOW**. Research is good, but working code is the deliverable.

**PRIORITY:** Get ONE thing working end-to-end. Don't try to do everything at once.

**SUPPORT:** Senior Engineer is available for questions and guidance.

**GOAL:** By end of week, have at least one simple STEP file converting successfully.

---

**Status:** 🔥 **URGENT - START IMPLEMENTATION NOW**  
**Priority:** **CRITICAL**  
**Support:** Available immediately

**Good luck, Riley! Let's get this working.**

---

*Assigned By: Jordan Rivera (Senior Engineer)*  
*Date: January 27, 2025*

