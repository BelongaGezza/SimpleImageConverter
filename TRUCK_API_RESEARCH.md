# truck API Research Document
## STEP Format Implementation Research

**Researcher:** Dr. Taylor Kim  
**Senior Engineer:** Jordan Rivera  
**Date:** January 27, 2025  
**Status:** ✅ Research Complete

---

## Executive Summary

This document provides comprehensive research on the truck library ecosystem for STEP file format support. The research covers API usage, tessellation approaches, and implementation patterns.

---

## truck Library Ecosystem

### Components

1. **truck-modeling (v0.3.0)**
   - CAD kernel for geometric modeling
   - Provides Shell, Solid, and other geometric primitives
   - Core modeling operations

2. **truck-polymesh (v0.3.0)**
   - Polygonal mesh operations
   - Tessellation capabilities
   - Mesh manipulation utilities

3. **truck-stepio (v0.3.0)**
   - STEP file I/O operations
   - Parses STEP files to truck modeling structures
   - Handles STEP format reading

### Version Information

**Current Version in Project:** 0.3.0  
**Latest Available:** 0.6.0 (as of research date)

**Note:** Architecture documents reference v0.4, but project uses v0.3.0. API may differ slightly.

---

## API Research Findings

### STEP File Parsing

Based on architecture documentation and crate analysis:

```rust
use truck_stepio::read;
use truck_modeling::Shell;

// Parse STEP file
let step_text = std::str::from_utf8(data)?;
let shells: Vec<Shell> = read(&step_text)?;
```

**Key Points:**
- `truck_stepio::read()` takes `&str` (STEP files are ASCII text)
- Returns `Vec<Shell>` - multiple shells for complex models
- Each Shell represents a closed 3D surface

### Tessellation Process

Based on architecture documentation:

```rust
use truck_polymesh::prelude::*;

// Tessellate a shell to polygonal mesh
let tolerance = 0.01; // Tessellation quality parameter
let mesh = shell.triangulation(tolerance);

// Extract geometry
let positions = mesh.positions();  // Vec<Point3>
let faces = mesh.faces();          // Vec<[usize; 3]> (triangle indices)
```

**Key Points:**
- `triangulation(tolerance)` converts Shell to polygonal mesh
- Tolerance controls tessellation quality (smaller = higher quality)
- Returns mesh with positions and face indices
- Faces are triangles (indices into positions array)

### Data Flow

```
STEP File (ASCII text)
    ↓
truck_stepio::read()
    ↓
Vec<Shell> (geometric surfaces)
    ↓
shell.triangulation(tolerance)
    ↓
Polygonal Mesh (positions + faces)
    ↓
Convert to our Mesh format
```

---

## Implementation Approach

### Step 1: Parse STEP File

```rust
// Convert bytes to string
let step_text = std::str::from_utf8(data)
    .map_err(|e| ConversionError::ConversionFailed(
        format!("STEP file is not valid UTF-8: {}", e)
    ))?;

// Parse using truck-stepio
let shells = truck_stepio::read(&step_text)
    .map_err(|e| ConversionError::ConversionFailed(
        format!("Failed to parse STEP file: {}", e)
    ))?;
```

### Step 2: Tessellate Shells

```rust
use truck_polymesh::prelude::*;

let mut all_vertices = Vec::new();
let mut all_faces = Vec::new();
let mut vertex_offset = 0;

for shell in shells {
    // Tessellate shell
    let tolerance = 0.01; // Configurable tessellation quality
    let mesh = shell.triangulation(tolerance);
    
    // Extract positions and faces
    let positions = mesh.positions();
    let faces = mesh.faces();
    
    // Convert positions to our Vertex format
    for pos in positions.iter() {
        all_vertices.push(Vertex {
            x: pos.x as f32,
            y: pos.y as f32,
            z: pos.z as f32,
        });
    }
    
    // Convert faces (adjust indices for vertex offset)
    for face in faces.iter() {
        all_faces.push(Face {
            indices: [
                vertex_offset + face[0],
                vertex_offset + face[1],
                vertex_offset + face[2],
            ],
        });
    }
    
    vertex_offset += positions.len();
}
```

### Step 3: Build Mesh

```rust
let mut mesh = Mesh::new();
mesh.vertices = all_vertices;
mesh.faces = all_faces;

// Calculate normals if needed
// (truck-polymesh may provide normals, or we calculate them)
```

---

## API Details

### truck_stepio::read()

**Signature (inferred):**
```rust
pub fn read(step_text: &str) -> Result<Vec<Shell>, Error>
```

**Parameters:**
- `step_text`: STEP file content as string (ASCII)

**Returns:**
- `Vec<Shell>`: Vector of Shell objects representing 3D surfaces

**Errors:**
- STEP parsing errors
- Invalid STEP structure
- Unsupported STEP features

### Shell::triangulation()

**Signature (inferred):**
```rust
impl Shell {
    pub fn triangulation(&self, tolerance: f64) -> PolygonalMesh
}
```

**Parameters:**
- `tolerance`: Tessellation quality (smaller = higher quality, more triangles)
  - Typical range: 0.001 to 0.1
  - Default recommendation: 0.01

**Returns:**
- `PolygonalMesh`: Tessellated mesh with positions and faces

### PolygonalMesh API

**Inferred methods:**
```rust
impl PolygonalMesh {
    pub fn positions(&self) -> &[Point3]  // Vertex positions
    pub fn faces(&self) -> &[[usize; 3]]   // Triangle face indices
}
```

**Note:** Actual API may differ. These are inferred from architecture documentation.

---

## Limitations and Gotchas

### Known Limitations

1. **STEP Feature Support:**
   - Not all STEP features may be supported
   - Complex assemblies may have issues
   - Some STEP AP variants may not work

2. **Tessellation Quality:**
   - Quality depends on tolerance parameter
   - Smaller tolerance = more triangles = slower
   - Balance between quality and performance

3. **Version Differences:**
   - API may differ between 0.3.0 and 0.6.0
   - Need to verify actual API in 0.3.0

4. **Error Handling:**
   - STEP parsing errors may be generic
   - Need robust error handling
   - May need to handle unsupported features gracefully

### Gotchas

1. **Multiple Shells:**
   - STEP files may contain multiple shells
   - Need to combine all shells into single mesh
   - Vertex indices need offset adjustment

2. **Coordinate Systems:**
   - STEP may use different coordinate systems
   - May need coordinate transformation
   - Verify coordinate system handling

3. **Normal Calculation:**
   - Tessellated mesh may not include normals
   - May need to calculate normals from faces
   - Consider face normal vs vertex normal

4. **Performance:**
   - Tessellation can be slow for complex models
   - Large STEP files may take significant time
   - Consider progress reporting

---

## Testing Strategy

### Test Files Needed

1. **Simple STEP Files:**
   - Single solid
   - Basic geometry (cube, sphere)
   - Minimal complexity

2. **Complex STEP Files:**
   - Multiple solids
   - Complex surfaces
   - Real-world CAD models

3. **Edge Cases:**
   - Empty STEP files
   - Invalid STEP files
   - Unsupported features

### Test Approach

1. **Unit Tests:**
   - Test parsing with sample STEP files
   - Test tessellation with known geometries
   - Test error handling

2. **Integration Tests:**
   - Test STEP → STL conversion
   - Test STEP → OBJ conversion
   - Test round-trip (if possible)

3. **Performance Tests:**
   - Benchmark tessellation time
   - Test with large files
   - Monitor memory usage

---

## Implementation Recommendations

### Phase 1: Basic Implementation

1. **Start Simple:**
   - Implement basic STEP parsing
   - Handle single shell case first
   - Use default tessellation tolerance

2. **Error Handling:**
   - Comprehensive error messages
   - Handle parsing errors gracefully
   - Log unsupported features

3. **Testing:**
   - Test with simple STEP files
   - Verify geometry preservation
   - Test error paths

### Phase 2: Enhancements

1. **Multiple Shells:**
   - Handle multiple shells properly
   - Combine into single mesh
   - Preserve shell boundaries (if needed)

2. **Tessellation Quality:**
   - Make tolerance configurable
   - Optimize for performance
   - Balance quality vs speed

3. **Normal Calculation:**
   - Calculate face normals
   - Optionally calculate vertex normals
   - Preserve normal information

### Phase 3: Optimization

1. **Performance:**
   - Profile tessellation
   - Optimize hot paths
   - Cache computations

2. **Memory:**
   - Stream large files if possible
   - Optimize allocations
   - Monitor memory usage

---

## Code Examples

### Complete Implementation Pattern

```rust
use truck_stepio::read;
use truck_modeling::Shell;
use truck_polymesh::prelude::*;
use crate::mesh::{Mesh, Vertex, Face};

fn parse_step_to_mesh(data: &[u8]) -> Result<Mesh> {
    // 1. Convert to string
    let step_text = std::str::from_utf8(data)?;
    
    // 2. Parse STEP file
    let shells: Vec<Shell> = read(&step_text)?;
    
    // 3. Tessellate and convert
    let mut mesh = Mesh::new();
    let mut vertex_offset = 0;
    
    for shell in shells {
        // Tessellate
        let tolerance = 0.01;
        let poly_mesh = shell.triangulation(tolerance);
        
        // Extract geometry
        let positions = poly_mesh.positions();
        let faces = poly_mesh.faces();
        
        // Add vertices
        for pos in positions.iter() {
            mesh.vertices.push(Vertex {
                x: pos.x as f32,
                y: pos.y as f32,
                z: pos.z as f32,
            });
        }
        
        // Add faces (with offset)
        for face in faces.iter() {
            mesh.faces.push(Face {
                indices: [
                    vertex_offset + face[0],
                    vertex_offset + face[1],
                    vertex_offset + face[2],
                ],
            });
        }
        
        vertex_offset += positions.len();
    }
    
    Ok(mesh)
}
```

---

## Version Compatibility Notes

### Version 0.3.0 (Current)

- Basic STEP reading support
- Tessellation available
- API may be slightly different from 0.4.0+

### Version 0.6.0 (Latest)

- More features
- Better performance
- API improvements
- **Note:** Not yet tested in project

### Migration Considerations

- Current implementation targets 0.3.0
- May need updates for 0.6.0 if upgrading
- Test thoroughly after version changes

---

## References

1. **truck GitHub Repository:**
   - https://github.com/ricosjp/truck

2. **Documentation:**
   - https://docs.rs/truck-stepio/
   - https://docs.rs/truck-polymesh/
   - https://docs.rs/truck-modeling/

3. **Project Documentation:**
   - `Phase3_Architecture.md` - Architecture examples
   - `rust-resources.md` - Library information
   - `Phase2.1_Decisions.md` - Technology decisions

---

## Next Steps

1. ✅ **Research Complete** - API patterns understood
2. ⏳ **Implementation** - Implement based on research
3. ⏳ **Testing** - Test with sample STEP files
4. ⏳ **Documentation** - Update project docs
5. ⏳ **Knowledge Sharing** - Update rust-resources.md

---

**Research Status:** ⚠️ Partial - API Verification Needed  
**Ready for Implementation:** ⚠️ Requires API Verification  
**Confidence Level:** Medium (architecture docs may reference different version)

**Update (2025-01-27):** Compilation errors fixed. Code structure is in place with placeholder error messages. The implementation will return informative errors until the actual truck-stepio v0.3.0 API is verified. Next step: Check https://docs.rs/truck-stepio/0.3.0/ for actual API documentation.

---

## API Verification Required

**Critical Finding:** The actual API in truck crates v0.3.0 differs from architecture documentation.

### Issues Found During Implementation:

1. **truck-stepio::read() not found**
   - Architecture docs suggest `truck_stepio::read(&str) -> Result<Vec<Shell>>`
   - Actual crate may use different function name or module structure
   - Need to verify actual API

2. **truck-polymesh::prelude not found**
   - Architecture docs suggest `use truck_polymesh::prelude::*;`
   - Actual crate structure may differ
   - Need to verify module structure

3. **Shell::triangulation() type inference issues**
   - Method exists but return type unclear
   - Need to verify return type and API

### Next Steps for API Verification:

1. **Check Actual Crate Documentation:**
   ```bash
   cargo doc -p truck-stepio --open
   cargo doc -p truck-polymesh --open
   cargo doc -p truck-modeling --open
   ```

2. **Examine Crate Source:**
   - Check GitHub repository for examples
   - Review crate documentation on docs.rs
   - Look for test files in crate source

3. **Test with Simple Example:**
   - Create minimal test program
   - Try different API patterns
   - Document what works

### Recommended Approach:

1. **Create Minimal Test:**
   ```rust
   // test_truck_api.rs
   use truck_stepio::*;
   use truck_modeling::*;
   use truck_polymesh::*;
   
   fn test_step_parsing() {
       let step_text = "ISO-10303-21;\n...";
       // Try different API patterns
   }
   ```

2. **Document Findings:**
   - Update this document with actual API
   - Provide working code examples
   - Note version differences

3. **Implement Based on Verified API:**
   - Use verified API patterns
   - Handle version differences
   - Add comprehensive error handling

---

**Researcher:** Dr. Taylor Kim  
**Reviewer:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025

