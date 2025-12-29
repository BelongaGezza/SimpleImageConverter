# Research: truck Shell Construction Examples
## Task 1.2 - Sam Parker (Junior Engineer, 2D Formats)

**Date:** January 27, 2025  
**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Purpose:** Document truck library Shell construction patterns and examples for STEP entity conversion

---

## Executive Summary

This document compiles research findings on truck library Shell construction APIs, focusing on:
- Building Shell objects from faces, edges, and vertices
- Handling curves and surfaces
- Coordinate system transformations
- Tessellation usage patterns

**Target Audience:** Riley (3D formats engineer) and Senior Engineer for implementation guidance

---

## truck Library Ecosystem

### Components Used

From `mesh-core/Cargo.toml`:

```toml
truck-modeling = { version = "0.3.0", optional = true }
truck-polymesh = { version = "0.3.0", optional = true }
truck-stepio = { version = "0.3.0", optional = true }
truck-meshalgo = { version = "0.4.0", optional = true }
```

### Library Information
- **Author:** ricosjp (same as ruststep)
- **License:** MIT OR Apache-2.0 (fully compatible)
- **Repository:** https://github.com/ricosjp/truck
- **Documentation:** https://docs.rs/truck-modeling/

---

## Key Research Questions

### 1. How to Construct Shell from Faces?

**Current Understanding:**
- `truck_modeling::Shell` is the target type
- Need to build Shell from STEP entity faces
- Shell requires proper topology (faces, edges, vertices)

**Research Needed:**
- [ ] Shell construction API
- [ ] Face construction from edges/vertices
- [ ] Topology building patterns

### 2. How to Construct Faces from Edges/Vertices?

**Current Understanding:**
- STEP entities have explicit topology
- Faces reference edges
- Edges reference vertices
- Need to reconstruct this in truck

**Research Needed:**
- [ ] Face construction API
- [ ] Edge construction API
- [ ] Vertex/Point construction API

### 3. How to Handle Curves and Surfaces?

**Current Understanding:**
- STEP entities have geometric information (curves, surfaces)
- truck needs geometric primitives
- May need curve/surface conversion

**Research Needed:**
- [ ] Curve types in truck
- [ ] Surface types in truck
- [ ] Conversion from STEP geometry

### 4. How to Use Tessellation?

**Current Understanding:**
- `truck-meshalgo` provides tessellation
- `triangulation()` method exists
- Returns `PolygonMesh` or similar

**Research Needed:**
- [ ] Exact tessellation API
- [ ] Tolerance parameter usage
- [ ] PolygonMesh extraction

---

## Code Examples and Patterns

### Pattern 1: Basic Shell Type Import

From current code (`mesh-core/src/formats/step.rs`):

```rust
use truck_modeling::Shell;
```

### Pattern 2: Tessellation Pattern (From Architecture Docs)

Based on `TRUCK_API_RESEARCH.md`:

```rust
use truck_polymesh::prelude::*;

// Tessellate a shell to polygonal mesh
let tolerance = 0.01; // Tessellation quality parameter
let mesh = shell.triangulation(tolerance);

// Extract geometry
let positions = mesh.positions();  // Vec<Point3>
let faces = mesh.faces();          // Vec<[usize; 3]> (triangle indices)
```

**Note:** This pattern is from architecture documentation and needs verification with actual API.

### Pattern 3: Shell Construction (Hypothetical)

**Hypothesis for building Shell from faces:**

```rust
use truck_modeling::{Shell, Face, Edge, Vertex};

// Build Shell from list of faces
fn build_shell_from_faces(faces: Vec<Face>) -> Shell {
    // This pattern needs verification
    // Shell::new(faces) or similar
}
```

### Pattern 4: Face Construction (Hypothetical)

**Hypothesis for building Face from edges:**

```rust
use truck_modeling::Face;

// Build Face from edges
fn build_face_from_edges(edges: Vec<Edge>, surface: Surface) -> Face {
    // Face needs:
    // - Boundary edges (forming a loop)
    // - Surface geometry
    // This pattern needs verification
}
```

### Pattern 5: Edge Construction (Hypothetical)

**Hypothesis for building Edge from vertices:**

```rust
use truck_modeling::Edge;

// Build Edge from vertices and curve
fn build_edge_from_vertices(
    start_vertex: Vertex,
    end_vertex: Vertex,
    curve: Curve,
) -> Edge {
    // Edge needs:
    // - Start and end vertices
    // - Curve geometry
    // This pattern needs verification
}
```

### Pattern 6: Vertex/Point Construction (Hypothetical)

**Hypothesis for creating vertices:**

```rust
use truck_modeling::Point3; // or similar type
use nalgebra::Point3;

// Create vertex from coordinates
fn create_vertex(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
```

---

## Shell Construction Workflow

### Workflow: STEP Entity → truck Shell

**Hypothetical workflow:**

```
STEP CLOSED_SHELL
  ↓
Extract face references
  ↓
For each face reference:
  Resolve FACE entity
  Extract face_bound (edge references)
  Extract face_geometry (surface)
  ↓
For each edge reference:
  Resolve EDGE entity
  Extract edge_start/edge_end (vertex references)
  Extract edge_geometry (curve)
  ↓
For each vertex reference:
  Resolve VERTEX_POINT entity
  Extract coordinates
  ↓
Build truck types:
  Point3 from coordinates
  Edge from vertices + curve
  Face from edges + surface
  Shell from faces
```

### Pattern: Complete Conversion (Hypothetical)

```rust
use truck_modeling::{Shell, Face, Edge, Point3};

fn convert_closed_shell_to_truck(
    closed_shell: &ClosedShell,
    tables: &Tables,
) -> Result<Shell> {
    let mut faces = Vec::new();
    
    // Convert each face
    for face_ref in closed_shell.faces() {
        let face_entity = resolve_face(face_ref, tables)?;
        let truck_face = convert_face_to_truck(face_entity, tables)?;
        faces.push(truck_face);
    }
    
    // Build Shell from faces
    let shell = Shell::from_faces(faces)?; // Hypothetical API
    Ok(shell)
}

fn convert_face_to_truck(face: &Face, tables: &Tables) -> Result<truck_modeling::Face> {
    // Extract face boundary (edges)
    let face_bound = face.face_bound();
    let edge_refs = face_bound.edges();
    
    let mut edges = Vec::new();
    for edge_ref in edge_refs {
        let edge_entity = resolve_edge(edge_ref, tables)?;
        let truck_edge = convert_edge_to_truck(edge_entity, tables)?;
        edges.push(truck_edge);
    }
    
    // Extract surface
    let surface = face.face_geometry();
    let truck_surface = convert_surface_to_truck(surface)?;
    
    // Build Face
    let truck_face = truck_modeling::Face::new(edges, truck_surface)?;
    Ok(truck_face)
}

fn convert_edge_to_truck(edge: &Edge, tables: &Tables) -> Result<truck_modeling::Edge> {
    // Extract vertices
    let start_vertex = resolve_vertex(edge.edge_start(), tables)?;
    let end_vertex = resolve_vertex(edge.edge_end(), tables)?;
    
    let start_point = Point3::new(
        start_vertex.coordinates().x(),
        start_vertex.coordinates().y(),
        start_vertex.coordinates().z(),
    );
    let end_point = Point3::new(
        end_vertex.coordinates().x(),
        end_vertex.coordinates().y(),
        end_vertex.coordinates().z(),
    );
    
    // Extract curve
    let curve = edge.edge_geometry();
    let truck_curve = convert_curve_to_truck(curve)?;
    
    // Build Edge
    let truck_edge = truck_modeling::Edge::new(start_point, end_point, truck_curve)?;
    Ok(truck_edge)
}
```

**Note:** This is a hypothetical pattern. Actual API needs verification.

---

## Tessellation Patterns

### Pattern 1: Basic Tessellation

From `TRUCK_API_RESEARCH.md` and architecture docs:

```rust
use truck_meshalgo::prelude::*;
use truck_polymesh::PolygonMesh;

// Tessellate shell
let tolerance = 0.01; // Smaller = higher quality
let tessellated = shell.triangulation(tolerance);

// Extract mesh data
let positions = tessellated.positions();
let faces = tessellated.faces();
```

### Pattern 2: Multiple Shells Tessellation

From current code comments:

```rust
// For each shell: shell.triangulation(tolerance) 
// -> Shell<Point3, PolylineCurve, Option<PolygonMesh>>
// Iterate through shell faces, extract Option<PolygonMesh> from each surface
// Collect all PolygonMeshes and merge them into a single mesh
```

**Implementation outline from `step.rs`:**

```rust
fn convert_truck_to_mesh(&self, shells: Vec<Shell>) -> Result<Mesh> {
    // 1. For each shell: shell.triangulation(tolerance) 
    //    -> Shell<Point3, PolylineCurve, Option<PolygonMesh>>
    // 2. Iterate through shell faces, extract Option<PolygonMesh> from each surface
    // 3. Collect all PolygonMeshes and merge them into a single mesh
    // 4. Convert to our Mesh format with vertices, faces, and normals
}
```

### Pattern 3: PolygonMesh Extraction (Hypothetical)

```rust
use truck_polymesh::PolygonMesh;

fn extract_polygon_meshes(shell: &Shell) -> Vec<PolygonMesh> {
    let mut meshes = Vec::new();
    
    // Tessellate shell
    let tessellated = shell.triangulation(0.01);
    
    // Extract PolygonMesh from each face
    for face in tessellated.faces() {
        if let Some(polygon_mesh) = face.surface_polygon_mesh() {
            meshes.push(polygon_mesh);
        }
    }
    
    meshes
}
```

---

## Coordinate System Handling

### Pattern: Coordinate Transformation

**Hypothetical pattern for coordinate system conversion:**

```rust
use nalgebra::Point3;

// STEP may use different coordinate system
// May need to transform coordinates
fn transform_coordinates(
    point: Point3<f64>,
    transform: &CoordinateTransform,
) -> Point3<f64> {
    // Apply transformation matrix
    transform.apply(point)
}

// Common transformations:
// - Y-up to Z-up
// - Unit conversion (mm to m, etc.)
// - Rotation/translation
```

---

## Curve and Surface Types

### Pattern: Curve Conversion (Hypothetical)

**STEP curve types that may need conversion:**

```rust
// STEP curve types:
// - LINE
// - CIRCLE
// - ELLIPSE
// - B_SPLINE_CURVE
// - etc.

fn convert_curve_to_truck(step_curve: &StepCurve) -> Result<truck_modeling::Curve> {
    match step_curve.curve_type() {
        "LINE" => {
            // Convert to truck Line
        }
        "CIRCLE" => {
            // Convert to truck Circle
        }
        "B_SPLINE_CURVE" => {
            // Convert to truck BSplineCurve
        }
        _ => {
            // Unsupported curve type
        }
    }
}
```

### Pattern: Surface Conversion (Hypothetical)

**STEP surface types that may need conversion:**

```rust
// STEP surface types:
// - PLANE
// - CYLINDRICAL_SURFACE
// - CONICAL_SURFACE
// - SPHERICAL_SURFACE
// - B_SPLINE_SURFACE
// - etc.

fn convert_surface_to_truck(step_surface: &StepSurface) -> Result<truck_modeling::Surface> {
    match step_surface.surface_type() {
        "PLANE" => {
            // Convert to truck Plane
        }
        "CYLINDRICAL_SURFACE" => {
            // Convert to truck CylindricalSurface
        }
        "B_SPLINE_SURFACE" => {
            // Convert to truck BSplineSurface
        }
        _ => {
            // Unsupported surface type
        }
    }
}
```

---

## Topology Reconstruction

### Pattern: Building Topology from STEP

**STEP has explicit topology, need to reconstruct in truck:**

```rust
// STEP topology structure:
// Shell → Faces → Edges → Vertices
// 
// truck needs:
// Shell with faces
// Faces with edges (boundary loops)
// Edges with vertices and curves
// Vertices with coordinates

fn build_topology(
    closed_shell: &ClosedShell,
    tables: &Tables,
) -> Result<Shell> {
    // 1. Collect all vertices first
    let mut vertices = HashMap::new();
    // ... collect from all edges ...
    
    // 2. Build edges from vertices
    let mut edges = HashMap::new();
    // ... build from edge entities ...
    
    // 3. Build faces from edges
    let mut faces = Vec::new();
    // ... build from face entities ...
    
    // 4. Build shell from faces
    let shell = Shell::from_faces(faces)?;
    Ok(shell)
}
```

---

## Error Handling Patterns

### Pattern: Handling Construction Errors

```rust
fn build_shell_safely(faces: Vec<Face>) -> Result<Shell> {
    if faces.is_empty() {
        return Err(ConversionError::InvalidGeometry(
            "Cannot create shell from empty face list".to_string()
        ));
    }
    
    // Validate faces form closed shell
    // Check edge connectivity
    // Verify topology
    
    match Shell::from_faces(faces) {
        Ok(shell) => Ok(shell),
        Err(e) => Err(ConversionError::ConversionFailed(
            format!("Failed to build shell: {}", e)
        )),
    }
}
```

---

## Testing Patterns

### Pattern: Testing Shell Construction

```rust
#[test]
fn test_shell_construction() {
    // Create simple shell (cube)
    let faces = create_cube_faces();
    let shell = Shell::from_faces(faces).unwrap();
    
    // Verify shell properties
    assert_eq!(shell.face_count(), 6);
    assert!(shell.is_closed());
}
```

---

## Resources and References

### Official Documentation
- truck-modeling docs.rs: https://docs.rs/truck-modeling/
- truck-polymesh docs.rs: https://docs.rs/truck-polymesh/
- truck-meshalgo docs.rs: https://docs.rs/truck-meshalgo/
- truck GitHub: https://github.com/ricosjp/truck

### Project References
- `TRUCK_API_RESEARCH.md` - Previous truck API research
- `mesh-core/src/formats/step.rs` - Current implementation
- `STEP_IMPLEMENTATION_CURRENT_STATE.md` - Implementation status

---

## Findings Summary

### ✅ Confirmed
- `truck_modeling::Shell` is the target type
- `truck-meshalgo` provides tessellation
- Shell tessellation returns some mesh type
- Architecture docs show basic patterns

### ⚠️ Needs Verification
- Exact Shell construction API
- Face/Edge/Vertex construction APIs
- Curve and surface type conversion
- PolygonMesh extraction from tessellated shell
- Coordinate system handling

### ❓ Unknown
- Exact API signatures
- Required parameters for construction
- Error types and handling
- Performance characteristics

---

## Next Steps for Verification

### Immediate Actions

1. **Check truck Documentation:**
   - Review https://docs.rs/truck-modeling/ for Shell API
   - Review https://docs.rs/truck-meshalgo/ for tessellation API
   - Look for construction examples

2. **Examine truck Source Code:**
   - Review GitHub repository
   - Check examples directory
   - Review test files for usage patterns

3. **Create Experimental Code:**
   - Build minimal test program
   - Try constructing simple Shell
   - Try tessellation
   - Document what works

4. **Share Findings:**
   - Update this document with verified patterns
   - Share with Riley and Senior Engineer
   - Create working code snippets

---

## Code Snippets for Reference

### Snippet 1: Current Tessellation Placeholder

From `mesh-core/src/formats/step.rs`:

```rust
fn convert_truck_to_mesh(&self, _shells: Vec<Shell>) -> Result<Mesh> {
    // TODO: Implement tessellation using truck-meshalgo
    // The triangulation() method returns Shell<Point3, PolylineCurve, Option<PolygonMesh>>
    // We need to extract PolygonMesh from each face's surface Option<PolygonMesh>
    Err(ConversionError::ConversionFailed(...))
}
```

### Snippet 2: Desired Pattern (Hypothetical)

```rust
use truck_meshalgo::prelude::*;
use truck_polymesh::PolygonMesh;

fn convert_truck_to_mesh(&self, shells: Vec<Shell>) -> Result<Mesh> {
    let mut all_vertices = Vec::new();
    let mut all_faces = Vec::new();
    let mut vertex_offset = 0;
    
    for shell in shells {
        // Tessellate
        let tolerance = 0.01;
        let tessellated = shell.triangulation(tolerance);
        
        // Extract PolygonMesh from faces
        for face in tessellated.faces() {
            if let Some(polygon_mesh) = face.surface_polygon_mesh() {
                // Extract vertices and faces
                let positions = polygon_mesh.positions();
                let faces = polygon_mesh.faces();
                
                // Add to combined mesh
                // ... conversion logic ...
            }
        }
    }
    
    // Build final Mesh
    Ok(Mesh { vertices: all_vertices, faces: all_faces })
}
```

---

## Questions for Further Research

1. **Shell Construction:**
   - What is the exact API for `Shell::new()` or `Shell::from_faces()`?
   - What parameters are required?
   - How do we handle edge connectivity?

2. **Face Construction:**
   - How do we create a `Face` from edges?
   - What surface types are required?
   - How do we handle face boundaries (loops)?

3. **Edge Construction:**
   - How do we create an `Edge` from vertices?
   - What curve types are supported?
   - How do we handle edge orientation?

4. **Tessellation:**
   - What does `triangulation()` actually return?
   - How do we extract `PolygonMesh` from the result?
   - What tolerance values are appropriate?

5. **Geometry Conversion:**
   - What curve types does truck support?
   - What surface types does truck support?
   - How do we convert STEP geometry to truck geometry?

---

## Updates Log

| Date | Update | Status |
|------|--------|--------|
| 2025-01-27 | Initial research document created | In Progress |
| | | |

---

**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Next Update:** After reviewing truck documentation and source code  
**Target:** Provide verified code examples and patterns for Riley

---

*Researcher: Sam Parker (Junior Engineer, 2D Formats)*  
*For: Riley Thompson (Junior Engineer, 3D Formats) & Senior Engineer*

