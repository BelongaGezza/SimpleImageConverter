# Task Assignment: Riley Thompson (Junior Engineer - 3D Formats)
## Sprint 5: Advanced 3D Formats

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Sprint Status:** Sprint 3 ✅ Complete | **Sprint 5 - Ready to Begin**  
**Priority:** 🔴 **HIGH - Sprint 5 Implementation**

---

## 🎉 Congratulations!

**Excellent work on Sprint 3!** Your implementations of STL, OBJ, and PLY formats are production-ready and demonstrate excellent code quality. All tests passing, no issues found in code review.

Sprint 4 (Advanced 2D) is being handled by Sam, so we're ready to move forward with Sprint 5!

---

## Current Status

**Completed Sprints:**
- ✅ **Sprint 2:** PNG, JPEG, BMP, GIF formats (Sam's work - complete)
- ✅ **Sprint 3:** STL, OBJ, PLY formats (your work - complete)

**Current Sprint:** **Sprint 5 - Advanced 3D Formats** - **YOUR TASK**

---

## Sprint 5 Overview

**Goal:** Add glTF, DXF, and custom OFF format support

**Duration:** 2 weeks (14 days)  
**Focus:** glTF (modern 3D), DXF (CAD exchange), OFF (simple geometry)

---

## Task 1: Implement OFF Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 2-3 days  
**Difficulty:** Medium (custom parser required)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/off.rs`
   - Follow the exact pattern from `stl.rs`, `obj.rs`, and `ply.rs`
   - Implement `OffFormat` struct
   - Implement `MeshReader` trait
   - Implement `MeshWriter` trait

2. **Write custom parser:**
   - OFF (Object File Format) is simple ASCII format
   - No external crate needed - write custom parser
   - Format specification:
     ```
     OFF
     [num_vertices] [num_faces] [num_edges]
     x1 y1 z1
     x2 y2 z2
     ...
     n v1 v2 v3 ... (face with n vertices)
     ```

3. **Handle OFF-specific features:**
   - Vertex positions (x, y, z)
   - Face indices (polygons - triangulate if needed)
   - Optional colors (can ignore for now)
   - ASCII format only

4. **Error handling:**
   - Invalid OFF syntax
   - Missing header
   - Invalid vertex/face counts
   - Out-of-bounds indices
   - Empty meshes

5. **Write implementation:**
   - Write ASCII OFF format
   - Validate mesh before writing
   - Handle triangulation if needed

### Implementation Pattern

Follow the STL/OBJ/PLY pattern:

```rust
pub struct OffFormat;

impl OffFormat {
    pub fn new() -> Self {
        Self
    }
    
    fn parse_off(&self, data: &[u8]) -> Result<Mesh> {
        let text = std::str::from_utf8(data)
            .map_err(|e| FormatError::ReadError(format!("Invalid UTF-8: {}", e)))?;
        
        let lines: Vec<&str> = text.lines().collect();
        
        // Parse header
        if lines.is_empty() || !lines[0].trim().starts_with("OFF") {
            return Err(FormatError::ReadError("Invalid OFF header".to_string()));
        }
        
        // Parse counts
        let counts_line = lines[1].trim();
        let counts: Vec<&str> = counts_line.split_whitespace().collect();
        if counts.len() < 2 {
            return Err(FormatError::ReadError("Invalid OFF counts".to_string()));
        }
        
        let num_vertices: usize = counts[0].parse()
            .map_err(|e| FormatError::ReadError(format!("Invalid vertex count: {}", e)))?;
        let num_faces: usize = counts[1].parse()
            .map_err(|e| FormatError::ReadError(format!("Invalid face count: {}", e)))?;
        
        // Parse vertices
        let mut vertices = Vec::new();
        for i in 2..(2 + num_vertices) {
            // Parse vertex line
            // ...
        }
        
        // Parse faces
        let mut faces = Vec::new();
        for i in (2 + num_vertices)..(2 + num_vertices + num_faces) {
            // Parse face line
            // ...
        }
        
        // Build mesh
        // ...
    }
}

impl Default for OffFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshReader for OffFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        self.parse_off(data)
    }
}

impl MeshWriter for OffFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Validate mesh
        // Write OFF format
        // ...
    }
}
```

### Testing Requirements

Write comprehensive tests (aim for 10-12 tests):

1. **Unit Tests:**
   - `test_off_format_new`
   - `test_read_simple_triangle`
   - `test_read_cube`
   - `test_read_with_colors` (ignore colors)
   - `test_read_polygon_face` (triangulate)
   - `test_read_invalid_header`
   - `test_read_invalid_counts`
   - `test_read_empty_data`
   - `test_write_triangle`
   - `test_write_cube`
   - `test_write_mesh_invalid_index`
   - `test_round_trip_triangle`
   - `test_round_trip_cube`

2. **Integration Tests:**
   - Add to `mesh-core/tests/integration.rs`
   - `test_off_round_trip_conversion`
   - `test_mesh_converter_off_round_trip`

### Success Criteria
- ✅ OFF format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Registered in format registry
- ✅ Follows STL/OBJ/PLY pattern exactly
- ✅ No linter errors
- ✅ Documentation complete

---

## Task 2: Implement glTF Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 4-5 days  
**Difficulty:** Medium-High (complex format, materials, textures)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/gltf.rs`
   - Follow the pattern from other formats
   - Implement `GltfFormat` struct
   - Implement `MeshReader` trait
   - Implement `MeshWriter` trait

2. **Use `gltf` crate:**
   - Add `gltf = "1.4"` to `mesh-core/Cargo.toml`
   - Use `gltf::Document::from_slice()` for reading
   - Handle both binary (.glb) and text (.gltf) formats
   - Extract mesh data from glTF scenes

3. **Handle glTF-specific features:**
   - Vertex positions
   - Face indices
   - Vertex normals (if present)
   - Texture coordinates (UVs)
   - Materials (basic support - store in metadata)
   - Multiple meshes (combine or handle separately)
   - Binary (.glb) and text (.gltf) formats

4. **Error handling:**
   - Invalid glTF structure
   - Missing required data
   - Unsupported glTF features
   - Binary/text format detection

5. **Write implementation:**
   - Write text (.gltf) format (simpler)
   - Binary (.glb) format (optional enhancement)
   - Basic material support

### Implementation Pattern

```rust
pub struct GltfFormat;

impl GltfFormat {
    pub fn new() -> Self {
        Self
    }
    
    fn parse_gltf(&self, data: &[u8]) -> Result<Mesh> {
        use gltf::Document;
        
        // Try to parse as glTF document
        let gltf = Document::from_slice(data)
            .map_err(|e| FormatError::ReadError(format!("glTF parse error: {}", e)))?;
        
        // Extract mesh data from first mesh in first scene
        // ...
        
        // Build mesh
        // ...
    }
}

impl MeshReader for GltfFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // Detect binary vs text format
        if data.starts_with(b"glTF") {
            // Binary .glb format
            self.parse_glb(data)
        } else {
            // Text .gltf format
            self.parse_gltf(data)
        }
    }
}

impl MeshWriter for GltfFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Write text .gltf format
        // ...
    }
}
```

### Dependencies to Add

Add to `mesh-core/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
gltf = "1.4"  # For glTF format support
```

### Testing Requirements

Write comprehensive tests (aim for 10-12 tests):

1. **Unit Tests:**
   - `test_gltf_format_new`
   - `test_read_simple_gltf`
   - `test_read_gltf_with_normals`
   - `test_read_gltf_with_uvs`
   - `test_read_binary_glb`
   - `test_read_invalid_gltf`
   - `test_read_empty_data`
   - `test_write_simple_gltf`
   - `test_write_gltf_with_normals`
   - `test_round_trip_simple`
   - `test_round_trip_with_normals`

2. **Integration Tests:**
   - Add to `mesh-core/tests/integration.rs`
   - `test_gltf_round_trip_conversion`
   - `test_mesh_converter_gltf_round_trip`

### Success Criteria
- ✅ glTF format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Binary and text formats supported
- ✅ Registered in format registry
- ✅ Follows established pattern
- ✅ No linter errors

---

## Task 3: Implement DXF Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 3-4 days  
**Difficulty:** Medium (CAD format, focus on 3D entities)

### Requirements

1. **Create the format handler:**
   - File: `mesh-core/src/formats/dxf.rs`
   - Follow the pattern from other formats
   - Implement `DxfFormat` struct
   - Implement `MeshReader` trait
   - Implement `MeshWriter` trait

2. **Use `dxf` crate:**
   - Add `dxf = "0.7"` to `mesh-core/Cargo.toml`
   - Use `dxf::Drawing::load()` for reading
   - Focus on 3D entities (ignore 2D)
   - Extract mesh data from 3D entities

3. **Handle DXF-specific features:**
   - 3D entities (3DFACE, POLYLINE, etc.)
   - Vertex positions
   - Face indices
   - Coordinate system handling
   - Layers (can ignore for now)

4. **Error handling:**
   - Invalid DXF structure
   - Missing 3D entities
   - Unsupported entity types
   - Empty drawings

5. **Write implementation:**
   - Write basic DXF format
   - Focus on 3DFACE entities
   - Handle coordinate system

### Implementation Pattern

```rust
pub struct DxfFormat;

impl DxfFormat {
    pub fn new() -> Self {
        Self
    }
    
    fn parse_dxf(&self, data: &[u8]) -> Result<Mesh> {
        use dxf::Drawing;
        
        let drawing = Drawing::load(data)
            .map_err(|e| FormatError::ReadError(format!("DXF parse error: {}", e)))?;
        
        // Extract 3D entities
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        
        for entity in drawing.entities() {
            match entity {
                dxf::entities::EntityType::Face3d(face) => {
                    // Extract vertices and face
                    // ...
                }
                // Handle other 3D entity types
                _ => {}
            }
        }
        
        // Build mesh
        // ...
    }
}

impl MeshReader for DxfFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        self.parse_dxf(data)
    }
}

impl MeshWriter for DxfFormat {
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>> {
        // Write DXF format
        // ...
    }
}
```

### Dependencies to Add

Add to `mesh-core/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
dxf = "0.7"  # For DXF format support
```

### Testing Requirements

Write comprehensive tests (aim for 10+ tests):

1. **Unit Tests:**
   - `test_dxf_format_new`
   - `test_read_simple_dxf`
   - `test_read_dxf_with_3dface`
   - `test_read_dxf_with_polyline`
   - `test_read_dxf_2d_only` (should handle gracefully)
   - `test_read_invalid_dxf`
   - `test_read_empty_data`
   - `test_write_simple_dxf`
   - `test_write_dxf_3dface`
   - `test_round_trip_simple`

2. **Integration Tests:**
   - Add to `mesh-core/tests/integration.rs`
   - `test_dxf_round_trip_conversion`

### Success Criteria
- ✅ DXF format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ 3D entities supported
- ✅ Registered in format registry
- ✅ Follows established pattern
- ✅ No linter errors

---

## Task 4: Update Format Registry

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 1 hour  
**Difficulty:** Easy

### Requirements

1. **Update `mesh-core/src/formats/registry.rs`:**
   - Add OFF to `MeshFormat` enum
   - Add Gltf to `MeshFormat` enum
   - Add Dxf to `MeshFormat` enum
   - Add format detection logic
   - Add to `get_reader()` method
   - Add to `get_writer()` method

2. **Update format detection:**
   - OFF detection: Check for "OFF" header
   - glTF detection: Check for "glTF" magic bytes (binary) or JSON structure (text)
   - DXF detection: Check for DXF section markers

3. **Update tests:**
   - Add tests for format detection
   - Add tests for get_reader/get_writer

### Code Changes

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshFormat {
    Stl,
    Obj,
    Ply,
    Off,   // ADD THIS
    Gltf,  // ADD THIS
    Dxf,   // ADD THIS
}

pub fn get_reader(format: MeshFormat) -> Result<Box<dyn MeshReader>> {
    match format {
        MeshFormat::Stl => Ok(Box::new(StlFormat::new())),
        MeshFormat::Obj => Ok(Box::new(ObjFormat::new())),
        MeshFormat::Ply => Ok(Box::new(PlyFormat::new())),
        MeshFormat::Off => Ok(Box::new(OffFormat::new())),   // ADD THIS
        MeshFormat::Gltf => Ok(Box::new(GltfFormat::new())), // ADD THIS
        MeshFormat::Dxf => Ok(Box::new(DxfFormat::new())),   // ADD THIS
    }
}

pub fn get_writer(format: MeshFormat) -> Result<Box<dyn MeshWriter>> {
    match format {
        MeshFormat::Stl => Ok(Box::new(StlFormat::new())),
        MeshFormat::Obj => Ok(Box::new(ObjFormat::new())),
        MeshFormat::Ply => Ok(Box::new(PlyFormat::new())),
        MeshFormat::Off => Ok(Box::new(OffFormat::new())),   // ADD THIS
        MeshFormat::Gltf => Ok(Box::new(GltfFormat::new())), // ADD THIS
        MeshFormat::Dxf => Ok(Box::new(DxfFormat::new())),   // ADD THIS
    }
}
```

### Success Criteria
- ✅ Registry updated with OFF, glTF, DXF
- ✅ Format detection working
- ✅ All registry tests pass
- ✅ No regressions

---

## Task 5: Update Documentation

**Priority:** 🟡 **MEDIUM**  
**Estimated Time:** 1 hour  
**Difficulty:** Easy

### Requirements

1. **Update `docs/FORMATS.md`:**
   - Mark OFF as ✅ implemented
   - Mark glTF as ✅ implemented
   - Mark DXF as ✅ implemented
   - Update Sprint 5 status

2. **Update code documentation:**
   - Ensure all public APIs documented
   - Add examples if needed
   - Document format-specific features

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
gltf = "1.4"  # For glTF format support
dxf = "0.7"   # For DXF format support
# OFF format uses custom parser (no external dependency)
```

---

## Implementation Checklist

### OFF Format
- [ ] Create `off.rs` file
- [ ] Implement `OffFormat` struct
- [ ] Implement custom OFF parser
- [ ] Implement `MeshReader` for OFF
- [ ] Implement `MeshWriter` for OFF
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### glTF Format
- [ ] Add `gltf` dependency
- [ ] Create `gltf.rs` file
- [ ] Implement `GltfFormat` struct
- [ ] Implement `MeshReader` for glTF (binary & text)
- [ ] Implement `MeshWriter` for glTF
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### DXF Format
- [ ] Add `dxf` dependency
- [ ] Create `dxf.rs` file
- [ ] Implement `DxfFormat` struct
- [ ] Implement `MeshReader` for DXF (3D entities)
- [ ] Implement `MeshWriter` for DXF
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### Format Registry
- [ ] Update `MeshFormat` enum
- [ ] Add format detection logic
- [ ] Update `get_reader()` method
- [ ] Update `get_writer()` method
- [ ] Add registry tests

### Documentation
- [ ] Update `docs/FORMATS.md`
- [ ] Update code documentation
- [ ] Verify all examples work

---

## Code Quality Standards

### ✅ Do's
- Follow STL/OBJ/PLY pattern exactly
- Write comprehensive tests (10+ per format)
- Include proper error handling
- Document public APIs
- Use descriptive error messages
- Validate inputs thoroughly
- Test edge cases (empty meshes, invalid data, etc.)
- Handle coordinate system differences

### ❌ Don'ts
- Don't skip tests
- Don't ignore edge cases
- Don't use unsafe code
- Don't copy-paste without understanding
- Don't commit without testing
- Don't forget to register in format registry
- Don't try to handle 2D DXF entities (focus on 3D)

---

## Reference Materials

1. **Existing Format Implementations:**
   - `mesh-core/src/formats/stl.rs` - Your excellent reference
   - `mesh-core/src/formats/obj.rs` - Your excellent reference
   - `mesh-core/src/formats/ply.rs` - Your excellent reference

2. **Documentation:**
   - `docs/ARCHITECTURE.md`
   - `docs/FORMATS.md`
   - `Phase3_Architecture.md`

3. **Library Documentation:**
   - `gltf` crate: https://docs.rs/gltf/
   - `dxf` crate: https://docs.rs/dxf/
   - OFF format spec: https://en.wikipedia.org/wiki/OFF_(file_format)

---

## Timeline

| Task | Duration | Start | End |
|------|----------|-------|-----|
| OFF Format | 2-3 days | Day 1 | Day 3 |
| glTF Format | 4-5 days | Day 4 | Day 8 |
| DXF Format | 3-4 days | Day 9 | Day 12 |
| Registry Update | 1 hour | Day 13 | Day 13 |
| Documentation | 1 hour | Day 13 | Day 13 |
| Testing & Polish | 1 day | Day 14 | Day 14 |

**Total Estimated Time:** 14 days (2 weeks)

---

## Questions & Support

If you have questions:

1. **Check Existing Implementations:**
   - Your STL/OBJ/PLY code is excellent reference
   - Follow the same patterns

2. **Check Documentation:**
   - `docs/ARCHITECTURE.md`
   - `docs/FORMATS.md`
   - `Phase3_Architecture.md`

3. **Ask for Help:**
   - Senior Engineer (Jordan) available
   - Code review available
   - Pair programming if needed

---

## Success Metrics

**Sprint 5 Completion:**
- ✅ OFF format implemented and tested
- ✅ glTF format implemented and tested
- ✅ DXF format implemented and tested
- ✅ All tests passing (target: 30+ new mesh tests)
- ✅ Documentation updated
- ✅ Code review approved

**Overall:**
- ✅ Sprint 5 marked complete
- ✅ Ready for Sprint 6 (Polish & Testing)
- ✅ Advanced formats foundation solid

---

## Final Notes

**Great work on Sprint 3!** Your implementations are excellent and serve as perfect references for Sprint 5.

**Focus Areas:**
1. Follow the established patterns exactly
2. Write comprehensive tests
3. Handle errors properly
4. Don't skip edge cases
5. For DXF, focus on 3D entities only

**Remember:** Quality over speed. Take time to do things right. Your Sprint 3 work shows you can do this!

---

**Assigned by:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Status:** Ready to begin  
**Priority:** 🔴 HIGH - Sprint 5 Implementation
