# Task Assignment: Riley Thompson (Junior Engineer - 3D Formats)

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** December 26, 2025  
**Priority:** HIGH - Start Sprint 3

---

## Overview

You're tasked with implementing Sprint 3: the core 3D mesh format handlers. This is the foundation for the `mesh-convert` tool. Start with **STL** (simplest), then **OBJ**, then **PLY**.

**Current Status:**
- ✅ Mesh data structures defined (`mesh-core/src/mesh/mod.rs`)
- ✅ Trait system ready (`mesh-core/src/formats/traits.rs`)
- ✅ MeshConverter orchestrator exists (`mesh-core/src/convert.rs`)
- ❌ No format implementations yet (your task)
- ❌ No format registry yet (your task)
- ❌ CLI is skeleton only (needs integration)

---

## Task 1: Create Format Registry

**Priority:** HIGH (do this first)  
**Estimated Time:** 1 day  
**Difficulty:** Easy (copy pattern from image registry)

### Requirements

1. **Create the registry:**
   - File: `mesh-core/src/formats/registry.rs`
   - Copy structure from `img-core/src/formats/registry.rs`
   - Create `MeshFormat` enum (STL, OBJ, PLY, OFF, etc.)
   - Create `FormatRegistry` struct with static methods

2. **Implement registry methods:**
   - `detect_format(extension: &str) -> Result<MeshFormat>`
   - `detect_from_path(path: &Path) -> Result<MeshFormat>`
   - `get_reader(format: MeshFormat) -> Result<Box<dyn MeshReader>>`
   - `get_writer(format: MeshFormat) -> Result<Box<dyn MeshWriter>>`

3. **Update module exports:**
   - Add `pub mod registry;` to `mesh-core/src/formats/mod.rs`
   - Export `FormatRegistry` and `MeshFormat`

4. **Write tests:**
   - Format detection tests
   - Registry lookup tests

### Reference
- `img-core/src/formats/registry.rs` - Perfect reference

### Acceptance Criteria

- ✅ Registry created and exported
- ✅ Can detect formats from extension
- ✅ Can detect formats from path
- ✅ Tests pass
- ✅ Ready for format implementations

---

## Task 2: Implement STL Format Handler

**Priority:** HIGH  
**Estimated Time:** 4-5 days  
**Difficulty:** Medium (good learning format)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/stl.rs`
   - Implement `StlFormat` struct
   - Follow pattern from image formats (structure, not implementation)

2. **Implement MeshReader trait:**
   - Read binary STL files
   - Read ASCII STL files
   - Auto-detect format (binary vs ASCII)
   - Parse vertices and faces
   - Handle normals (STL includes face normals)
   - Return `Mesh` structure

3. **Implement MeshWriter trait:**
   - Write binary STL files
   - Write ASCII STL files
   - Option to choose format (default to binary)
   - Write vertices and faces
   - Write normals

4. **Update module exports:**
   - Add `pub mod stl;` to `mesh-core/src/formats/mod.rs`
   - Add `pub use stl::StlFormat;` to exports

5. **Update FormatRegistry:**
   - Add STL to `MeshFormat` enum
   - Update `get_reader()` and `get_writer()` to return `StlFormat`

6. **Write comprehensive tests:**
   - Minimum 6 unit tests:
     - `test_stl_read_binary()` - Read binary STL
     - `test_stl_read_ascii()` - Read ASCII STL
     - `test_stl_write_binary()` - Write binary STL
     - `test_stl_write_ascii()` - Write ASCII STL
     - `test_stl_round_trip()` - Read → Write → Read
     - `test_stl_read_invalid()` - Error handling

### Reference Libraries

**Option 1: Use `stl_io` crate**
```toml
[dependencies]
stl_io = "0.6"
```

**Option 2: Custom parser**
- STL format is relatively simple
- Binary: 80-byte header + triangles
- ASCII: text-based format

### Reference Code

Study these files:
- `img-core/src/formats/png.rs` - Structure and pattern
- `mesh-core/src/mesh/mod.rs` - Data structures to use
- `mesh-core/src/formats/traits.rs` - Trait definitions

### Implementation Notes

- STL is a good starting format (simpler than OBJ)
- Binary STL is more common than ASCII
- STL includes face normals - preserve them
- Coordinate system: STL often uses Z-up (we'll handle transforms later)

### Questions to Ask

- Should we default to binary or ASCII output?
- How should we handle STL files with invalid normals?
- Should we validate mesh topology?

### Acceptance Criteria

- ✅ STL format handler created
- ✅ Can read binary STL files
- ✅ Can read ASCII STL files
- ✅ Can write binary STL files
- ✅ Can write ASCII STL files
- ✅ Format auto-detection works
- ✅ Registered in FormatRegistry
- ✅ All unit tests pass
- ✅ Integration test: STL → STL conversion works
- ✅ Code follows existing patterns

---

## Task 3: Implement OBJ Format Handler

**Priority:** HIGH  
**Estimated Time:** 5-6 days  
**Difficulty:** Medium-High (more complex)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/obj.rs`
   - Implement `ObjFormat` struct

2. **Implement MeshReader trait:**
   - Parse vertex positions (`v`)
   - Parse normals (`vn`)
   - Parse texture coordinates (`vt`) - store but may not use initially
   - Parse faces (`f`) with vertex/normal/UV indices
   - Handle multiple objects (extract first or combine)
   - Handle material files (.mtl) - basic support (parse but don't require)

3. **Implement MeshWriter trait:**
   - Write vertices (`v`)
   - Write normals (`vn`)
   - Write faces (`f`)
   - Optionally write materials (can defer)

4. **Update module exports:**
   - Add `pub mod obj;` to `mesh-core/src/formats/mod.rs`
   - Add `pub use obj::ObjFormat;` to exports

5. **Update FormatRegistry:**
   - Add OBJ to `MeshFormat` enum (if not already)
   - Update registry methods

6. **Write comprehensive tests:**
   - Minimum 6 unit tests:
     - `test_obj_read()` - Read OBJ file
     - `test_obj_read_with_normals()` - Read OBJ with normals
     - `test_obj_read_with_uvs()` - Read OBJ with UVs
     - `test_obj_write()` - Write OBJ file
     - `test_obj_round_trip()` - Read → Write → Read
     - `test_obj_read_invalid()` - Error handling

### Reference Libraries

**Option 1: Use `tobj` crate**
```toml
[dependencies]
tobj = "4.0"
```

**Option 2: Use `obj-rs` crate**
```toml
[dependencies]
obj-rs = "0.1"
```

Evaluate both and choose the one that fits best.

### Implementation Notes

- OBJ is more complex than STL (materials, UVs, multiple objects)
- Start with basic mesh data (vertices, faces, normals)
- Material support can be basic initially
- UV coordinates: parse and store, but may not use in all conversions
- Multiple objects: extract first object or combine all

### Questions to Ask

- Should we support multiple objects in one OBJ file?
- How should we handle missing normals (recalculate or error)?
- Should we require .mtl files or make them optional?

### Acceptance Criteria

- ✅ OBJ format handler created
- ✅ Can read OBJ files (vertices, faces, normals)
- ✅ Can write OBJ files
- ✅ Basic material support (parse .mtl if present)
- ✅ Registered in FormatRegistry
- ✅ All unit tests pass
- ✅ Integration test: OBJ ↔ STL conversion works
- ✅ Code follows existing patterns

---

## Task 4: Implement PLY Format Handler

**Priority:** HIGH  
**Estimated Time:** 4-5 days  
**Difficulty:** Medium

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/ply.rs`
   - Implement `PlyFormat` struct

2. **Implement MeshReader trait:**
   - Read binary PLY files
   - Read ASCII PLY files
   - Parse vertices and faces
   - Handle custom properties (ignore for now, or store generically)

3. **Implement MeshWriter trait:**
   - Write binary PLY files
   - Write ASCII PLY files
   - Write standard properties (x, y, z, vertex indices)

4. **Update module exports:**
   - Add `pub mod ply;` to `mesh-core/src/formats/mod.rs`
   - Add `pub use ply::PlyFormat;` to exports

5. **Update FormatRegistry:**
   - Add PLY to `MeshFormat` enum
   - Update registry methods

6. **Write comprehensive tests:**
   - Minimum 6 unit tests:
     - `test_ply_read_binary()` - Read binary PLY
     - `test_ply_read_ascii()` - Read ASCII PLY
     - `test_ply_write_binary()` - Write binary PLY
     - `test_ply_write_ascii()` - Write ASCII PLY
     - `test_ply_round_trip()` - Read → Write → Read
     - `test_ply_read_invalid()` - Error handling

### Reference Libraries

**Option 1: Use `ply-rs` crate**
```toml
[dependencies]
ply-rs = "0.1"
```

**Option 2: Custom parser**
- PLY format is text-based (ASCII) or binary
- Relatively straightforward format

### Implementation Notes

- PLY is similar to STL but more flexible
- Supports custom properties (can ignore for now)
- Binary and ASCII variants
- Similar complexity to STL

### Acceptance Criteria

- ✅ PLY format handler created
- ✅ Can read binary PLY files
- ✅ Can read ASCII PLY files
- ✅ Can write binary PLY files
- ✅ Can write ASCII PLY files
- ✅ Registered in FormatRegistry
- ✅ All unit tests pass
- ✅ Integration test: PLY ↔ STL conversion works
- ✅ Code follows existing patterns

---

## Task 5: Complete mesh-convert CLI

**Priority:** MEDIUM (after formats are done)  
**Estimated Time:** 2-3 days  
**Difficulty:** Easy (follow img-convert pattern)

### Requirements

1. **Update CLI implementation:**
   - File: `mesh-convert/src/main.rs`
   - Follow pattern from `img-convert/src/main.rs`
   - Integrate with `mesh-core` library
   - Use `FormatRegistry` for format detection
   - Use `MeshConverter` for conversion

2. **Implement CLI features:**
   - Input file path
   - Output format
   - Output file path (optional)
   - Coordinate transform option (defer implementation, just parse)
   - Normal recalculation option (defer implementation, just parse)
   - Validation option (defer implementation, just parse)

3. **Error handling:**
   - Clear error messages
   - File validation
   - Format validation

4. **Testing:**
   - Test CLI with real mesh files
   - Test error cases

### Reference Code

Study `img-convert/src/main.rs` - it's a perfect template.

### Acceptance Criteria

- ✅ CLI functional for STL, OBJ, PLY conversions
- ✅ Format detection works
- ✅ Error messages are clear
- ✅ Options parsed correctly (transforms can be stubs)

---

## Testing Requirements

### Unit Tests

For each format, create tests:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Your tests here
}
```

### Integration Tests

Create `mesh-core/tests/integration.rs`:
- STL ↔ OBJ conversion
- STL ↔ PLY conversion
- OBJ ↔ PLY conversion
- Round-trip tests

### Test Files

You may need test mesh files. You can:
- Use simple test meshes (cube, sphere)
- Generate programmatically
- Use sample files from online repositories

---

## Code Quality Standards

Follow these patterns:

1. **Error Handling:**
   ```rust
   .map_err(|e| ConversionError::ConversionFailed(format!(
       "Failed to read STL mesh: {}",
       e
   )))?;
   ```

2. **Data Structures:**
   - Use `mesh-core/src/mesh/mod.rs` structures
   - `Mesh`, `Vertex`, `Face`, `Normal`

3. **Documentation:**
   - Add doc comments to public items
   - Follow existing documentation style

---

## Timeline

- **Week 1:**
  - Day 1: Format registry
  - Day 2-5: STL format

- **Week 2:**
  - Day 1-5: OBJ format

- **Week 3:**
  - Day 1-4: PLY format
  - Day 5: Integration testing

- **Week 4:**
  - Day 1-2: CLI completion
  - Day 3-4: Final testing, bug fixes
  - Day 5: Documentation, code review

---

## Getting Help

**When to ask:**
- If you're stuck for more than 2 hours
- If format parsing is unclear
- If you encounter unexpected behavior
- Before marking a task complete

**How to ask:**
- "I'm working on [task] and found [issue]. My options are [A] or [B]. Which approach fits our architecture better?"

**Code Review:**
- Request review after STL is complete
- Request review after OBJ is complete
- Request review after PLY is complete
- Don't wait until all are done

---

## Definition of Done

Each format is complete when:

1. ✅ Format handler file created
2. ✅ `MeshReader` trait implemented
3. ✅ `MeshWriter` trait implemented
4. ✅ Registered in `FormatRegistry`
5. ✅ Exported in `formats/mod.rs`
6. ✅ Unit tests written (minimum 6 tests)
7. ✅ All tests passing
8. ✅ Integration tests added
9. ✅ Code review completed
10. ✅ Documentation updated

---

## Notes

- **Start with STL** - it's the simplest format, good for learning
- **Follow image format patterns** - structure is similar, just different data
- **Coordinate systems** - we'll handle transforms later, focus on reading/writing first
- **Normals** - preserve them when present, we'll add recalculation later
- **Materials** - basic support is fine, full material system is Phase 2

---

**Good luck! You're building the foundation for 3D mesh conversion. Start with STL and work your way up!**

*Jordan Rivera*  
*Senior Engineer*

