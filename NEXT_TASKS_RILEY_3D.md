# Task Assignment: Riley Thompson (Junior Engineer - 3D Formats)
## Sprint 3 Completion Tasks

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** December 27, 2025  
**Priority:** 🔴 **HIGH - Complete Sprint 3**

---

## Current Status

**Sprint 3 Progress:** 🚧 **33% Complete**

| Component | Status | Notes |
|-----------|--------|-------|
| **Format Registry** | ✅ Complete | Excellent foundation |
| **STL Format** | ✅ Complete | Production-ready, 12 tests |
| **OBJ Format** | ❌ Not Started | **YOUR TASK** |
| **PLY Format** | ❌ Not Started | **YOUR TASK** |
| **mesh-convert CLI** | ⚠️ Skeleton Only | **YOUR TASK** |

**Excellent work on STL!** Your implementation is production-ready and serves as an excellent reference.

---

## Task 1: Implement OBJ Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 3-4 days  
**Difficulty:** Medium (similar to STL, but with materials)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/obj.rs`
   - Follow the exact pattern from `stl.rs`
   - Implement `ObjFormat` struct
   - Implement `MeshReader` trait
   - Implement `MeshWriter` trait

2. **Use `tobj` crate:**
   - Add `tobj = "4.0"` to `mesh-core/Cargo.toml`
   - Use `tobj::load_obj()` for reading
   - Handle OBJ/MTL file pairs
   - Extract vertices, faces, normals, UVs

3. **Handle OBJ-specific features:**
   - Vertex positions
   - Face indices (can be quads, triangulate if needed)
   - Vertex normals (if present)
   - Texture coordinates (UVs)
   - Materials (basic support - store in mesh metadata)

4. **Error handling:**
   - Invalid OBJ syntax
   - Missing MTL files (warn but continue)
   - Invalid face indices
   - Empty meshes

5. **Write implementation:**
   - Write vertices, faces, normals
   - Basic material support (optional)
   - ASCII format only (OBJ is text-based)

### Implementation Pattern

Follow the STL pattern exactly:

```rust
pub struct ObjFormat;

impl ObjFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ObjFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for ObjFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Use tobj to parse OBJ
        // Convert to our Mesh structure
        // Handle errors properly
    }
}

impl MeshWriter for ObjFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Validate mesh
        // Write OBJ format
        // Handle errors properly
    }
}
```

### Testing Requirements

Write comprehensive tests (aim for 10-12 tests like STL):

1. **Unit Tests:**
   - `test_obj_format_new`
   - `test_read_simple_triangle`
   - `test_read_cube`
   - `test_read_with_normals`
   - `test_read_with_uvs`
   - `test_read_invalid_data`
   - `test_read_empty_data`
   - `test_write_triangle`
   - `test_write_cube`
   - `test_write_mesh_without_normals`
   - `test_write_mesh_invalid_index`
   - `test_round_trip_triangle`
   - `test_round_trip_cube`

2. **Integration Tests:**
   - Add to `mesh-core/tests/integration.rs`
   - `test_obj_round_trip_conversion`
   - `test_mesh_converter_obj_round_trip`

### Success Criteria
- ✅ OBJ format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Registered in format registry
- ✅ Follows STL pattern exactly
- ✅ No linter errors
- ✅ Documentation complete

---

## Task 2: Implement PLY Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 3-4 days  
**Difficulty:** Medium (similar to STL/OBJ)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/ply.rs`
   - Follow the exact pattern from `stl.rs` and `obj.rs`
   - Implement `PlyFormat` struct
   - Implement `MeshReader` trait
   - Implement `MeshWriter` trait

2. **Use `ply-rs` crate:**
   - Add `ply-rs = "0.1"` to `mesh-core/Cargo.toml`
   - Use `ply_rs::read_ply()` for reading
   - Handle both ASCII and binary PLY
   - Extract vertices, faces, normals

3. **Handle PLY-specific features:**
   - Vertex positions (x, y, z)
   - Face indices (polygons - triangulate if needed)
   - Vertex normals (nx, ny, nz) - if present
   - Texture coordinates (s, t) - if present
   - ASCII and binary formats

4. **Error handling:**
   - Invalid PLY syntax
   - Missing required properties
   - Invalid face indices
   - Empty meshes
   - Unsupported PLY features

5. **Write implementation:**
   - Write vertices, faces, normals
   - Support ASCII format (simpler to start)
   - Binary format (optional enhancement)

### Implementation Pattern

Follow the STL/OBJ pattern:

```rust
pub struct PlyFormat;

impl PlyFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PlyFormat {
    fn default() -> Self::new()
    }
}

impl MeshReader for PlyFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Use ply-rs to parse PLY
        // Convert to our Mesh structure
        // Handle ASCII and binary
    }
}

impl MeshWriter for PlyFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Validate mesh
        // Write PLY format (ASCII)
        // Handle errors properly
    }
}
```

### Testing Requirements

Write comprehensive tests (aim for 10-12 tests):

1. **Unit Tests:**
   - `test_ply_format_new`
   - `test_read_simple_triangle_ascii`
   - `test_read_cube_ascii`
   - `test_read_with_normals`
   - `test_read_invalid_data`
   - `test_read_empty_data`
   - `test_write_triangle_ascii`
   - `test_write_cube_ascii`
   - `test_write_mesh_without_normals`
   - `test_write_mesh_invalid_index`
   - `test_round_trip_triangle`
   - `test_round_trip_cube`

2. **Integration Tests:**
   - Add to `mesh-core/tests/integration.rs`
   - `test_ply_round_trip_conversion`
   - `test_mesh_converter_ply_round_trip`

### Success Criteria
- ✅ PLY format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Registered in format registry
- ✅ Follows STL/OBJ pattern exactly
- ✅ No linter errors
- ✅ Documentation complete

---

## Task 3: Update Format Registry

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 1 hour  
**Difficulty:** Easy

### Requirements

1. **Update `mesh-core/src/formats/registry.rs`:**
   - Add OBJ and PLY to `get_reader()` method
   - Add OBJ and PLY to `get_writer()` method
   - Ensure error handling is correct

2. **Update tests:**
   - Add tests for OBJ format detection
   - Add tests for PLY format detection
   - Add tests for get_reader/get_writer for OBJ/PLY

### Code Changes

```rust
pub fn get_reader(format: MeshFormat) -> Result<Box<dyn MeshReader>> {
    match format {
        MeshFormat::Stl => Ok(Box::new(StlFormat::new())),
        MeshFormat::Obj => Ok(Box::new(ObjFormat::new())),  // ADD THIS
        MeshFormat::Ply => Ok(Box::new(PlyFormat::new())),  // ADD THIS
    }
}

pub fn get_writer(format: MeshFormat) -> Result<Box<dyn MeshWriter>> {
    match format {
        MeshFormat::Stl => Ok(Box::new(StlFormat::new())),
        MeshFormat::Obj => Ok(Box::new(ObjFormat::new())),  // ADD THIS
        MeshFormat::Ply => Ok(Box::new(PlyFormat::new())),  // ADD THIS
    }
}
```

### Success Criteria
- ✅ Registry updated with OBJ/PLY
- ✅ All registry tests pass
- ✅ No regressions

---

## Task 4: Integrate mesh-convert CLI

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 1-2 days  
**Difficulty:** Medium (follow img-convert pattern)

### Requirements

1. **Update `mesh-convert/src/main.rs`:**
   - Remove TODO comments
   - Implement actual conversion logic
   - Follow the pattern from `img-convert/src/main.rs`

2. **Use MeshConverter:**
   - Import `MeshConverter` from `mesh-core`
   - Use `FormatRegistry` for format detection
   - Handle command-line arguments properly

3. **Implement features:**
   - Format detection (input and output)
   - File reading/writing
   - Conversion orchestration
   - Error handling and user messages
   - Transform options (future - can be placeholder)
   - Recalculate normals (future - can be placeholder)
   - Validate mesh (future - can be placeholder)

### Implementation Pattern

Follow `img-convert/src/main.rs` exactly:

```rust
fn main() -> Result<()> {
    let args = Args::parse();

    // Validate input file
    let input_path = Path::new(&args.input);
    common::validation::validate_file_path(input_path)?;

    // Detect formats
    let input_format = FormatRegistry::detect_from_path(input_path)?;
    let output_format = FormatRegistry::detect_format(&args.format)?;

    // Determine output path
    let output_path = /* ... */;

    // Read input file
    let input_data = read_file_bytes(input_path)?;

    // Get format handlers
    let reader = FormatRegistry::get_reader(input_format)?;
    let writer = FormatRegistry::get_writer(output_format)?;

    // Convert
    let converter = MeshConverter::new();
    let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref())?;

    // Write output file
    write_file_bytes(&output_path, &output_data)?;

    println!("Successfully converted {} to {}", args.input, output_path.display());

    Ok(())
}
```

### Testing

1. **Manual Testing:**
   - Test STL to OBJ conversion
   - Test OBJ to PLY conversion
   - Test PLY to STL conversion
   - Test error handling (invalid files, etc.)

2. **CLI Tests (optional):**
   - Add basic CLI tests if time permits

### Success Criteria
- ✅ CLI fully functional
- ✅ All format conversions work
- ✅ Error handling proper
- ✅ User-friendly messages
- ✅ Follows img-convert pattern

---

## Task 5: Update Documentation

**Priority:** 🟡 **MEDIUM**  
**Estimated Time:** 1 hour  
**Difficulty:** Easy

### Requirements

1. **Update `docs/FORMATS.md`:**
   - Mark OBJ as ✅ implemented
   - Mark PLY as ✅ implemented
   - Update Sprint 3 status to ✅ complete

2. **Update code documentation:**
   - Ensure all public APIs documented
   - Add examples if needed
   - Update doc comments

### Success Criteria
- ✅ FORMATS.md updated
- ✅ All docs accurate
- ✅ Examples work

---

## Dependencies to Add

Add these to `mesh-core/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
tobj = "4.0"      # For OBJ format
ply-rs = "0.1"    # For PLY format
```

---

## Implementation Checklist

### OBJ Format
- [ ] Add `tobj` dependency
- [ ] Create `obj.rs` file
- [ ] Implement `ObjFormat` struct
- [ ] Implement `MeshReader` for OBJ
- [ ] Implement `MeshWriter` for OBJ
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### PLY Format
- [ ] Add `ply-rs` dependency
- [ ] Create `ply.rs` file
- [ ] Implement `PlyFormat` struct
- [ ] Implement `MeshReader` for PLY
- [ ] Implement `MeshWriter` for PLY
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### CLI Integration
- [ ] Update `mesh-convert/src/main.rs`
- [ ] Implement conversion logic
- [ ] Test all format combinations
- [ ] Verify error handling
- [ ] Update help text if needed

### Documentation
- [ ] Update `docs/FORMATS.md`
- [ ] Update code documentation
- [ ] Verify all examples work

---

## Code Quality Standards

### ✅ Do's
- Follow STL implementation pattern exactly
- Write comprehensive tests (10+ per format)
- Include proper error handling
- Document public APIs
- Use descriptive error messages
- Validate inputs thoroughly
- Test edge cases (empty meshes, invalid data, etc.)

### ❌ Don'ts
- Don't skip tests
- Don't ignore edge cases
- Don't use unsafe code
- Don't copy-paste without understanding
- Don't commit without testing
- Don't forget to register in format registry

---

## Reference Materials

1. **STL Implementation:**
   - `mesh-core/src/formats/stl.rs` - Your excellent reference

2. **Image Format Pattern:**
   - `img-core/src/formats/png.rs` - Similar pattern
   - `img-core/src/formats/bmp.rs` - Similar pattern

3. **CLI Pattern:**
   - `img-convert/src/main.rs` - Follow this exactly

4. **Documentation:**
   - `docs/ARCHITECTURE.md`
   - `docs/FORMATS.md`
   - `Phase3_Architecture.md`

5. **Library Documentation:**
   - `tobj` crate: https://docs.rs/tobj/
   - `ply-rs` crate: https://docs.rs/ply-rs/

---

## Timeline

| Task | Duration | Start | End |
|------|----------|-------|-----|
| OBJ Format | 3-4 days | Day 1 | Day 4 |
| PLY Format | 3-4 days | Day 5 | Day 8 |
| Registry Update | 1 hour | Day 9 | Day 9 |
| CLI Integration | 1-2 days | Day 10 | Day 11 |
| Documentation | 1 hour | Day 12 | Day 12 |
| Testing & Polish | 1 day | Day 13 | Day 13 |

**Total Estimated Time:** 13 days (2.5 weeks)

---

## Questions & Support

If you have questions:

1. **Check STL Implementation:**
   - Your STL code is an excellent reference
   - Follow the same patterns

2. **Check Image Formats:**
   - Similar patterns in img-core
   - Good examples of error handling

3. **Ask for Help:**
   - Senior Engineer (Jordan) available
   - Code review available
   - Pair programming if needed

---

## Success Metrics

**Sprint 3 Completion:**
- ✅ OBJ format implemented and tested
- ✅ PLY format implemented and tested
- ✅ mesh-convert CLI functional
- ✅ All tests passing (target: 50+ mesh tests)
- ✅ Documentation updated
- ✅ Code review approved

**Overall:**
- ✅ Sprint 3 marked complete
- ✅ Ready for Sprint 5 (advanced 3D formats)
- ✅ Foundation solid for future work

---

## Final Notes

**Great work on STL!** Your implementation is excellent and serves as a perfect reference for OBJ and PLY.

**Focus Areas:**
1. Follow the STL pattern exactly
2. Write comprehensive tests
3. Handle errors properly
4. Don't skip edge cases

**Remember:** Quality over speed. Take time to do things right. Your STL implementation shows you can do this!

---

**Assigned by:** Jordan Rivera (Senior Engineer)  
**Date:** December 27, 2025  
**Status:** Ready to begin  
**Priority:** 🔴 HIGH - Complete Sprint 3

