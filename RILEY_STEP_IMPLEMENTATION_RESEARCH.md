# Riley's STEP Implementation Research Notes
## v0.2.0 STEP Entity Conversion Implementation

**Engineer:** Riley Thompson (Junior Engineer, 3D Formats)  
**Date Started:** January 27, 2025  
**Status:** 🔬 Research Phase 1 - In Progress

---

## Research Phase 1: ruststep Tables API

### Objective
Understand how to build AP203 `Tables` from `Exchange.data` and deserialize entities.

### Resources
- ruststep v0.4.0 documentation: https://docs.rs/ruststep/
- ruststep GitHub: https://github.com/ricosjp/ruststep
- AP203 feature enabled in Cargo.toml

### Current Understanding

**What we have:**
- `ruststep::parser::parse()` returns an `Exchange` struct
- `Exchange` contains `data: Vec<DataSection>`
- Each `DataSection` contains `entities: Vec<EntityInstance>`
- `EntityInstance` can be `Simple { id, record }` or `Complex { id, subsuper }`
- `Record` has a `name: String` and parameters

**What we need:**
- Build AP203 `Tables` structure from `Exchange.data`
- Deserialize `Record`s into AP203 structs (e.g., `ManifoldSolidBrep`, `ClosedShell`)
- Resolve entity references (#1, #2, etc.)

### Questions to Answer

1. How do I create AP203 `Tables` from `Exchange.data`?
2. How do I deserialize a `Record` into an AP203 struct?
3. How do I resolve entity references (like #1, #2) using Tables?
4. What AP203 types are available for the entities we care about?

### Findings

**What I've Learned So Far:**

1. **STEP Parsing Works:**
   - ✅ `ruststep::parser::parse()` successfully parses STEP files
   - ✅ Returns `Exchange` struct containing `data: Vec<DataSection>`
   - ✅ Each `DataSection` has `entities: Vec<EntityInstance>`
   - ✅ Entity instances can be `Simple { id, record }` or `Complex { id, subsuper }`
   - ✅ Records have `name: String` and `parameter` fields
   - ✅ Created experimental code (`explore_ruststep_tables.rs`) to explore the API

2. **AP203 Feature Enabled:**
   - ✅ `ruststep` is configured with `features = ["ap203"]` in Cargo.toml
   - ✅ AP203 types should be available via `ruststep::ap203::config_control_design`
   - ⚠️ Tables API exists but exact usage pattern still needs investigation

3. **Current Code Structure:**
   - ✅ STEP parsing is working in `mesh-core/src/formats/step.rs`
   - ✅ Entity type identification works (match on `record.name`)
   - ❌ Missing: Tables construction, entity deserialization, reference resolution

**Open Questions:**
1. How exactly do I create and populate `Tables` from `Exchange.data`?
2. What's the pattern for deserializing a `Record` into an AP203 struct (e.g., `ManifoldSolidBrep`)?
3. How do I resolve entity references (#1, #2) using the Tables structure?

**Next Steps:**
- [ ] Explore ruststep documentation more directly (docs.rs or GitHub)
- [ ] Check ruststep GitHub repository for examples
- [ ] Consider asking Senior Engineer for guidance on Tables API usage

---

## Research Phase 2: truck Shell Construction APIs

### Objective
Learn how to build `Shell` objects from geometric primitives in truck.

### Resources
- truck-modeling v0.3.0 documentation
- truck-topology documentation
- truck-meshalgo documentation
- truck GitHub: https://github.com/ricosjp/truck

### Current Understanding

**What we have:**
- `truck_modeling::Shell` type imported
- Need to construct `Shell` objects from STEP entity data

**What we need:**
- Understand Shell construction APIs
- Learn face/edge/vertex construction patterns
- Coordinate system handling
- Curve and surface types in truck

### Questions to Answer

1. How do I create a `Shell` from faces?
2. How do I create a `Face` from edges/vertices?
3. How do I handle curves and surfaces?
4. What coordinate transformations are needed?

### Findings

(To be filled as research progresses)

---

## Research Phase 3: Tessellation API

### Objective
Learn how to use truck-meshalgo to tessellate Shell objects.

### Resources
- truck-meshalgo v0.4.0 documentation
- Already imported (commented out) in step.rs

### Current Understanding

**What we have:**
- Commented-out imports show intended approach:
  - `truck_meshalgo::prelude::*`
  - `truck_polymesh::PolygonMesh`

**What we need:**
- Understand `triangulation()` method
- Extract `PolygonMesh` from tessellated Shell
- Convert `PolygonMesh` to our `Mesh` format

### Questions to Answer

1. What does `shell.triangulation(tolerance)` return?
2. How do I extract `PolygonMesh` from the result?
3. How do I convert `PolygonMesh` to our `Mesh` format (vertices, faces, normals)?

### Findings

(To be filled as research progresses)

---

## Implementation Strategy

### Incremental Approach

1. **Start Simple:** Begin with simpler entity types
   - FACETED_BREP (if available - already triangulated)
   - Then CLOSED_SHELL
   - Then MANIFOLD_SOLID_BREP

2. **Test Frequently:** Test each step with real STEP files

3. **Document Progress:** Update this file and STEP_IMPLEMENTATION_CURRENT_STATE.md

---

## Next Steps

1. [ ] Research ruststep Tables API - build experimental code
2. [ ] Research truck Shell API - build experimental code
3. [ ] Review ruststep GitHub repository for examples
4. [ ] Review truck GitHub repository for examples
5. [ ] Create experimental code snippets demonstrating key operations
6. [ ] Share findings with Senior Engineer for review

---

**Last Updated:** January 27, 2025

