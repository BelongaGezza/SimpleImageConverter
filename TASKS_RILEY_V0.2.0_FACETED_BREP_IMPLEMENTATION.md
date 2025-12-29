# Task Assignment - Riley Thompson (Junior Engineer, 3D Formats)
## v0.2.0 FACETED_BREP Implementation - Post-Architect Approval

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 29, 2025  
**Engineer:** Riley Thompson  
**Priority:** 🔥 **CRITICAL - ARCHITECT APPROVED**  
**Status:** ✅ **READY TO PROCEED**

---

## Executive Summary

The System Architect has **approved** the FACETED_BREP approach for v0.2.0. You've done **excellent work** on the foundational pieces (Tables population, entity deserialization). Now we need to complete the implementation by extracting geometry directly from FACETED_BREP entities.

**Architect Decision:** ✅ Direct FACETED_BREP extraction (skip truck Shell conversion)

**Current Status:**
- ✅ STEP file parsing working
- ✅ Tables population working (`Tables::from_data_sections()`)
- ✅ Entity deserialization working (`IntoOwned` trait)
- ✅ Code cleanup completed (debug statements removed)
- 🚧 **NEXT:** Implement FACETED_BREP entity traversal and mesh extraction

---

## Task 1: Code Cleanup ✅ COMPLETE

**Status:** ✅ **DONE** (Senior Engineer completed)

The following cleanup has been completed:
- ✅ Removed `eprintln!` debug statements
- ✅ Removed unused `truck_modeling::Shell` import
- ✅ Refactored `extract_entities_from_tables()` → `extract_faceted_brep()`
- ✅ Removed `convert_truck_to_mesh()` placeholder
- ✅ Updated error messages

**No action needed from you on this task.**

---

## Task 2: FACETED_BREP Entity Detection (2-4 hours)

**Objective:** Verify and implement FACETED_BREP entity detection

**Steps:**
1. [ ] Research ruststep AP203 API for FACETED_BREP entities
   - Check if `tables.faceted_brep_holders()` method exists
   - If not, check ruststep documentation/source code
   - Verify entity type name (may be `FacetedBrep` or similar)

2. [ ] Implement entity detection
   ```rust
   // Try FACETED_BREP first
   if let Some(fb_holders) = tables.faceted_brep_holders() {
       // Found FACETED_BREP entities
   } else {
       // Fall back to MANIFOLD_SOLID_BREP with planar faces
   }
   ```

3. [ ] Add proper error handling
   - Clear error if no FACETED_BREP found
   - Suggest CAD export settings
   - Reference documentation

4. [ ] Test with simple STEP file
   - Verify detection works
   - Log entity counts for debugging

**Success Criteria:**
- ✅ Can detect FACETED_BREP entities (or fallback to MANIFOLD_SOLID_BREP)
- ✅ Clear error messages for unsupported geometry
- ✅ Test passes with simple STEP file

**Resources:**
- ruststep documentation: https://docs.rs/ruststep/0.4/
- ruststep GitHub: https://github.com/ricosjp/ruststep
- Sam's research: `TABLES_API_FINDINGS_FOR_RILEY.md`

---

## Task 3: Entity Traversal Implementation (4-8 hours)

**Objective:** Implement traversal from FACETED_BREP to vertices

**Entity Traversal Path:**
```
FACETED_BREP
  └── outer: CLOSED_SHELL
      └── cfs_faces: SET[FACE]
          └── bounds: SET[FACE_BOUND]
              └── bound: EDGE_LOOP
                  └── edge_list: LIST[ORIENTED_EDGE]
                      └── edge_element: EDGE
                          └── edge_start/end: VERTEX_POINT
                              └── vertex_geometry: CARTESIAN_POINT (x, y, z)
```

**Steps:**
1. [ ] Implement FACETED_BREP → CLOSED_SHELL traversal
   - Resolve `outer` reference
   - Handle reference resolution errors

2. [ ] Implement CLOSED_SHELL → FACE traversal
   - Iterate through `cfs_faces` (connected face set)
   - Handle SET structure

3. [ ] Implement FACE → FACE_BOUND → EDGE_LOOP traversal
   - Extract `bounds` from FACE
   - Find `EDGE_LOOP` in bounds
   - Handle multiple bounds (outer + holes)

4. [ ] Implement EDGE_LOOP → EDGE → VERTEX_POINT traversal
   - Iterate through `edge_list`
   - Resolve `edge_element` (EDGE)
   - Extract `edge_start` and `edge_end` (VERTEX_POINT)

5. [ ] Implement VERTEX_POINT → CARTESIAN_POINT traversal
   - Resolve `vertex_geometry` reference
   - Extract coordinates (x, y, z)

**Code Structure:**
```rust
fn extract_faceted_brep(&self, tables: &Tables) -> Result<Mesh> {
    // 1. Get FACETED_BREP entities
    let fb_holders = /* get FACETED_BREP holders */;
    
    if fb_holders.is_empty() {
        return Err(/* error */);
    }
    
    let mut all_vertices = Vec::new();
    let mut all_faces = Vec::new();
    
    for (id, holder) in fb_holders.iter() {
        // 2. Resolve FACETED_BREP
        let fb = holder.clone().into_owned(tables)?;
        
        // 3. Get CLOSED_SHELL
        let shell = /* resolve outer shell */;
        
        // 4. Iterate through faces
        for face in shell.faces() {
            // 5. Extract vertices from face bounds
            let vertices = /* traverse EDGE_LOOP */;
            all_vertices.extend(vertices);
        }
    }
    
    // 6. Build mesh
    Ok(Mesh { vertices, faces, normals })
}
```

**Success Criteria:**
- ✅ Can traverse from FACETED_BREP to CARTESIAN_POINT
- ✅ Handles reference resolution errors gracefully
- ✅ Test passes with simple FACETED_BREP STEP file

**Resources:**
- STEP_FORMAT_REFERENCE.md (entity structure documentation)
- ruststep AP203 types documentation

---

## Task 4: Vertex and Face Extraction (4-8 hours)

**Objective:** Extract vertices and build face indices from entity traversal

**Steps:**
1. [ ] Extract vertex coordinates
   - Collect CARTESIAN_POINT coordinates
   - Convert to `Vertex` type (f32 x, y, z)
   - Handle coordinate system (STEP uses right-handed)

2. [ ] Implement vertex deduplication
   - Same coordinates = same vertex
   - Use HashMap or similar for O(1) lookup
   - Map original vertex ID → deduplicated index

3. [ ] Build face indices from EDGE_LOOP
   - EDGE_LOOP defines face boundary
   - Need to triangulate polygon (if not already triangles)
   - Handle vertex winding order (STEP may be CCW or CW)

4. [ ] Calculate normals
   - Compute face normals from vertex positions
   - Use cross product: (v1-v0) × (v2-v0)
   - Normalize vectors

5. [ ] Handle edge cases
   - Degenerate faces (collinear vertices)
   - Non-planar faces (shouldn't happen in FACETED_BREP, but check)
   - Empty faces

**Code Structure:**
```rust
fn extract_vertices_from_edge_loop(
    edge_loop: &EdgeLoop,
    tables: &Tables,
) -> Result<Vec<Vertex>> {
    let mut vertices = Vec::new();
    let mut vertex_map = HashMap::new(); // For deduplication
    
    for oriented_edge in edge_loop.edge_list() {
        let edge = /* resolve edge_element */;
        let start_vertex = /* resolve edge_start */;
        let end_vertex = /* resolve edge_end */;
        
        // Extract coordinates from CARTESIAN_POINT
        let start_point = /* resolve vertex_geometry */;
        let end_point = /* resolve vertex_geometry */;
        
        // Add vertices (with deduplication)
        // ...
    }
    
    Ok(vertices)
}

fn build_face_indices(
    edge_loop: &EdgeLoop,
    vertex_map: &HashMap<EntityId, usize>,
) -> Result<Vec<Face>> {
    // Extract vertex indices from edge loop
    // Triangulate if needed
    // Build Face structs
}
```

**Success Criteria:**
- ✅ Vertices extracted correctly
- ✅ Face indices built correctly
- ✅ Normals calculated correctly
- ✅ Vertex deduplication working
- ✅ Test passes with simple FACETED_BREP STEP file

---

## Task 5: Mesh Construction and Testing (2-4 hours)

**Objective:** Build final Mesh and test end-to-end

**Steps:**
1. [ ] Build Mesh from extracted data
   ```rust
   let mesh = Mesh {
       vertices: all_vertices,
       faces: all_faces,
       normals: calculated_normals,
   };
   ```

2. [ ] Validate mesh
   - Non-empty vertices and faces
   - Valid face indices (all < vertices.len())
   - Check for degenerate faces

3. [ ] Test with simple FACETED_BREP STEP file
   - Create or find test file
   - Verify conversion works
   - Check output mesh is valid

4. [ ] End-to-end test
   - STEP → Mesh → STL/OBJ/PLY
   - Verify output can be read by other tools
   - Compare with expected result

5. [ ] Error handling tests
   - Test with non-FACETED_BREP file (should error gracefully)
   - Test with empty file
   - Test with invalid STEP file

**Success Criteria:**
- ✅ Simple FACETED_BREP STEP file converts successfully
- ✅ Output mesh is valid
- ✅ Can convert to other formats (STL/OBJ/PLY)
- ✅ Error handling works correctly
- ✅ All tests pass

---

## Implementation Strategy

### Incremental Approach

1. **Start Simple:**
   - Begin with one FACETED_BREP entity
   - Get ONE face working
   - Then expand to multiple faces

2. **Test Frequently:**
   - Test after each traversal step
   - Use simple STEP files
   - Verify each component works

3. **Commit Often:**
   - Commit working code frequently
   - Don't wait until everything is done
   - Make progress visible

4. **Ask for Help:**
   - If blocked for more than 2 hours, ask for help
   - Don't struggle alone
   - Collaborate with Sam on API research

---

## Getting Help

### When to Ask

**Ask immediately if:**
- Blocked for more than 2 hours
- API doesn't work as expected
- Need clarification on approach
- Entity structure unclear

### Who to Ask

1. **Sam:** For ruststep API research and documentation
2. **Senior Engineer:** For implementation guidance, architecture questions
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

## Timeline

**Week 1 (Days 1-3):**
- Day 1: Task 2 (FACETED_BREP detection) + Task 3 start
- Day 2: Task 3 (Entity traversal) completion
- Day 3: Task 4 (Vertex/face extraction) + Task 5 start

**Week 2 (Days 4-5):**
- Day 4: Task 5 (Testing) + bug fixes
- Day 5: Final testing + documentation

**Target:** Working FACETED_BREP conversion by end of Week 1

---

## Success Criteria

### End of Week 1

- ✅ FACETED_BREP detection working
- ✅ Entity traversal implemented
- ✅ At least one simple FACETED_BREP STEP file converts successfully
- ✅ Code committed and tested

### End of Week 2

- ✅ Full FACETED_BREP extraction working
- ✅ Multiple test files convert successfully
- ✅ Error handling for unsupported geometry
- ✅ Code complete and ready for review

---

## Key Messages

**EXCELLENT WORK:** You've done great foundational work. The architect has approved the approach, so we can proceed with confidence.

**PRIORITY:** Focus on getting FACETED_BREP extraction working end-to-end. Start simple, test frequently, commit often.

**SUPPORT:** Ask for help if blocked. Sam can help with API research. Senior Engineer available for guidance.

**GOAL:** By end of Week 1, have at least one simple FACETED_BREP STEP file converting successfully.

---

**Status:** 🔥 **CRITICAL - READY TO PROCEED**  
**Priority:** **HIGHEST**  
**Support:** Available immediately

**Good luck, Riley! Let's get this working.**

---

*Assigned By: Jordan Rivera (Senior Engineer)*  
*Date: January 29, 2025*  
*Architect Approval: Alex Chen (System Architect) - January 29, 2025*

