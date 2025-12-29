# Task Assignment - Riley Thompson (Junior Engineer, 3D Formats)
## v0.2.0 STEP Implementation - Phase 2 URGENT Tasks

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Engineer:** Riley Thompson  
**Priority:** 🔥 **CRITICAL - ACCELERATE IMPLEMENTATION**  
**Status:** ⚠️ **PROGRESS MADE, BUT CRITICAL GAPS REMAIN**

---

## Critical Review Summary

**Previous Assessment:**
- ✅ Started implementation (Tables structure added)
- ✅ Code compiles successfully
- ❌ **Tables not populated from Exchange.data**
- ❌ **No entity deserialization yet**
- ❌ **No conversion to truck Shell yet**
- ❌ **No end-to-end functionality**

**Action Required:** **ACCELERATE IMPLEMENTATION - Complete Tables population and entity deserialization URGENTLY**

---

## URGENT Tasks (This Week - Days 1-3)

### Task 2.1.1: Research Tables Population API 🔥 CRITICAL

**Objective:** Understand how to populate AP203 Tables from Exchange.data

**Status:** ❌ **BLOCKING** - This is preventing all further progress

**Steps:**
1. [ ] Research ruststep documentation for Tables population methods
2. [ ] Check ruststep GitHub repository for examples
3. [ ] Review ruststep source code if needed
4. [ ] Ask Sam for help (he's doing verification)
5. [ ] Ask Senior Engineer if still blocked after 2 hours

**Resources:**
- ruststep documentation: https://docs.rs/ruststep/
- ruststep GitHub: https://github.com/ricosjp/ruststep
- Sam's verification code: `mesh-core/examples/verify_ruststep_tables.rs`
- Sam's research: `RESEARCH_RUSTSTEP_EXAMPLES.md`

**Expected Outcome:**
- Understand how to populate Tables from Exchange.data
- Have working code that populates Tables
- Tables structure ready for entity deserialization

**Time Limit:** **2-4 hours maximum** - If blocked, ask for help immediately

---

### Task 2.1.2: Implement Tables Population 🔥 CRITICAL

**Objective:** Implement code that populates Tables from Exchange.data

**Status:** ⏳ **PENDING** (blocked by Task 2.1.1)

**Steps:**
1. [ ] Implement Tables population function
2. [ ] Test with simple STEP file
3. [ ] Verify Tables structure is correct
4. [ ] Replace `Tables::default()` with populated Tables
5. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - `parse_step()` method

**Expected Pattern:**
```rust
// In parse_step(), after parsing:
let tables = populate_tables_from_exchange(&exchange)?;
// Use tables for entity deserialization
```

**Success Criteria:**
- ✅ Tables populated from Exchange.data
- ✅ Can access entities from Tables
- ✅ Test passes with simple STEP file
- ✅ Code committed

**Estimated Effort:** 2-4 hours (after understanding API)

---

### Task 2.2: Deserialize ONE Entity Type 🔥 CRITICAL

**Objective:** Deserialize at least ONE STEP entity type into AP203 struct

**Status:** ⏳ **PENDING** (blocked by Task 2.1.2)

**Strategy:** Start with simplest entity type (e.g., CARTESIAN_POINT)

**Steps:**
1. [ ] Choose simplest entity type to start with
2. [ ] Study AP203 struct for that entity type
3. [ ] Implement deserialization code
4. [ ] Test with simple STEP file containing that entity
5. [ ] Verify deserialization works
6. [ ] Commit working code

**Code Location:** `mesh-core/src/formats/step.rs` - `try_extract_shell()` method

**Expected Pattern:**
```rust
use ruststep::ap203::config_control_design::CartesianPoint;

// In try_extract_shell():
if record.name == "CARTESIAN_POINT" {
    // Deserialize record into AP203 struct
    let point: CartesianPoint = /* deserialize using tables */;
    // Use point data
}
```

**Success Criteria:**
- ✅ Can deserialize one entity type successfully
- ✅ Can access entity fields
- ✅ Test passes with simple STEP file
- ✅ Code committed

**Estimated Effort:** 2-4 hours

---

### Task 2.3: Resolve Entity References 🔥 CRITICAL

**Objective:** Implement reference resolution (#1, #2, etc.) using Tables

**Status:** ⏳ **PENDING** (blocked by Task 2.2)

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

**Estimated Effort:** 2-4 hours

---

### Task 2.4: Convert ONE Entity to truck Shell 🔥 CRITICAL

**Objective:** Convert at least ONE STEP entity type to truck Shell

**Status:** ⏳ **PENDING** (blocked by Task 2.3)

**Strategy:** Start with simplest entity (e.g., FACETED_BREP if available, or simple CLOSED_SHELL)

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

**Estimated Effort:** 4-8 hours

---

### Task 2.5: Basic Tessellation 🔥 CRITICAL

**Objective:** Implement basic tessellation for truck Shell

**Status:** ⏳ **PENDING** (blocked by Task 2.4)

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

**Estimated Effort:** 4-6 hours

---

### Task 2.6: End-to-End Test 🔥 CRITICAL

**Objective:** Test complete pipeline with simple STEP file

**Status:** ⏳ **PENDING** (blocked by Task 2.5)

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

**Estimated Effort:** 2-4 hours

---

## Weekly Milestones

### End of Week 1 (Critical)

**Must Have:**
- ✅ Tables populated from Exchange.data (working)
- ✅ At least ONE entity type deserialized
- ✅ Reference resolution working (basic)
- ✅ At least ONE entity type converted to truck Shell
- ✅ Basic tessellation working
- ✅ Simple STEP file converts end-to-end

**Success Criteria:**
- ✅ Code compiles and runs
- ✅ At least one simple STEP file converts successfully
- ✅ All code committed to repository

**This is the MINIMUM viable deliverable.**

---

## Implementation Strategy

### Incremental Approach

1. **Start Simple:**
   - Begin with simplest entity types (CARTESIAN_POINT, then MANIFOLD_SOLID_BREP)
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

4. **Ask for Help:**
   - If blocked for more than 2 hours, ask for help
   - Don't struggle alone
   - Collaborate with Sam on Tables API research

---

## Getting Help

### When to Ask

**Ask immediately if:**
- Blocked for more than 2 hours
- API doesn't work as expected
- Need clarification on approach
- Tables API research not yielding results

### Who to Ask

1. **Sam:** For Tables API research collaboration
2. **Senior Engineer:** For implementation guidance, API questions
3. **Resources:** ruststep documentation, GitHub examples

### How to Ask

- Be specific about what you've tried
- Share code snippets
- Explain what's not working
- Ask for specific help

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

## Timeline

**Week 1 (Remaining - Days 1-3):**
- Day 1: Tables population research + implementation
- Day 2: Entity deserialization + reference resolution
- Day 3: Entity to Shell conversion + tessellation + end-to-end test

**Target:** Working end-to-end conversion by end of Day 3

---

## Final Notes

**CRITICAL:** You've started implementation, which is good, but you need to **accelerate**. The Tables population is blocking everything else.

**PRIORITY:** Focus on getting Tables populated and ONE entity type working end-to-end. Don't try to do everything at once.

**SUPPORT:** Ask for help if blocked. Don't struggle alone on Tables API. Sam can help with research.

**GOAL:** By end of Week 1, have at least one simple STEP file converting successfully.

---

**Status:** 🔥 **URGENT - ACCELERATE IMPLEMENTATION**  
**Priority:** **CRITICAL**  
**Support:** Available immediately

**Good luck, Riley! Let's get this working.**

---

*Assigned By: Jordan Rivera (Senior Engineer)*  
*Date: January 27, 2025*

