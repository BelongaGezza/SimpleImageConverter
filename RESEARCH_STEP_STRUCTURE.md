# Research: STEP File Structure
## Task 1.3 - Sam Parker (Junior Engineer, 2D Formats)

**Date:** January 27, 2025  
**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Purpose:** Document STEP file format structure, entity types, and reference resolution patterns

---

## Executive Summary

This document compiles research findings on STEP file format structure, focusing on:
- STEP file organization (header, data sections)
- Common entity types and their parameters
- Reference resolution patterns (#1, #2, etc.)
- Entity relationships and hierarchies
- Simple STEP file examples

**Target Audience:** Riley (3D formats engineer) and Senior Engineer for implementation guidance

---

## STEP File Format Overview

### File Structure

STEP files (ISO 10303-21) have a standard structure:

```
ISO-10303-21;
HEADER;
  ... header entities ...
ENDSEC;
DATA;
  ... data entities ...
ENDSEC;
END-ISO-10303-21;
```

### Sections

1. **Header Section:**
   - File metadata
   - Description, name, timestamp
   - Schema information

2. **Data Section:**
   - Geometric entities
   - Topology entities
   - References between entities

3. **End Marker:**
   - `END-ISO-10303-21;`

---

## Entity Structure

### Entity Format

STEP entities follow this pattern:

```
#ID = ENTITY_TYPE(parameter1, parameter2, ...);
```

**Example:**
```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
#2 = CLOSED_SHELL('shell', (#3, #4, #5));
```

### Entity Components

1. **Entity ID:** `#1`, `#2`, etc. (unique identifier)
2. **Entity Type:** `MANIFOLD_SOLID_BREP`, `CLOSED_SHELL`, etc.
3. **Parameters:** Values or references to other entities

---

## Common Entity Types

### Geometric Entities

#### 1. MANIFOLD_SOLID_BREP

**Purpose:** Represents a solid with boundary representation

**Format:**
```
#ID = MANIFOLD_SOLID_BREP(name, closed_shell_ref);
```

**Parameters:**
- `name`: String identifier
- `closed_shell_ref`: Reference to CLOSED_SHELL entity

**Example:**
```
#1 = MANIFOLD_SOLID_BREP('MySolid', #2);
```

**Entity Hierarchy:**
```
MANIFOLD_SOLID_BREP
  └─ closed_shell: CLOSED_SHELL
```

#### 2. CLOSED_SHELL

**Purpose:** Represents a closed shell (collection of faces)

**Format:**
```
#ID = CLOSED_SHELL(name, face_list);
```

**Parameters:**
- `name`: String identifier
- `face_list`: List of references to FACE entities

**Example:**
```
#2 = CLOSED_SHELL('MyShell', (#3, #4, #5, #6));
```

**Entity Hierarchy:**
```
CLOSED_SHELL
  └─ faces: List[FACE]
```

#### 3. FACE

**Purpose:** Represents a face with geometry and boundary

**Format:**
```
#ID = FACE(face_geometry, face_bound);
```

**Parameters:**
- `face_geometry`: Reference to surface entity
- `face_bound`: Reference to FACE_BOUND entity

**Example:**
```
#3 = FACE(#10, #20);
```

**Entity Hierarchy:**
```
FACE
  ├─ face_geometry: SURFACE
  └─ face_bound: FACE_BOUND
      └─ edges: List[EDGE]
```

#### 4. FACE_BOUND

**Purpose:** Defines the boundary of a face

**Format:**
```
#ID = FACE_BOUND(bound, orientation);
```

**Parameters:**
- `bound`: Reference to EDGE_LOOP or similar
- `orientation`: Boolean (TRUE/FALSE)

**Example:**
```
#20 = FACE_BOUND(#30, .T.);
```

#### 5. EDGE_LOOP

**Purpose:** Defines a loop of edges

**Format:**
```
#ID = EDGE_LOOP(edge_list);
```

**Parameters:**
- `edge_list`: List of references to ORIENTED_EDGE entities

**Example:**
```
#30 = EDGE_LOOP((#40, #41, #42, #43));
```

#### 6. ORIENTED_EDGE

**Purpose:** Oriented edge (with direction)

**Format:**
```
#ID = ORIENTED_EDGE(edge_element, orientation);
```

**Parameters:**
- `edge_element`: Reference to EDGE entity
- `orientation`: Boolean (TRUE/FALSE)

**Example:**
```
#40 = ORIENTED_EDGE(#50, .T.);
```

#### 7. EDGE

**Purpose:** Represents an edge with geometry

**Format:**
```
#ID = EDGE(edge_start, edge_end, edge_geometry);
```

**Parameters:**
- `edge_start`: Reference to VERTEX_POINT
- `edge_end`: Reference to VERTEX_POINT
- `edge_geometry`: Reference to curve entity

**Example:**
```
#50 = EDGE(#60, #61, #70);
```

**Entity Hierarchy:**
```
EDGE
  ├─ edge_start: VERTEX_POINT
  ├─ edge_end: VERTEX_POINT
  └─ edge_geometry: CURVE
```

#### 8. VERTEX_POINT

**Purpose:** Represents a vertex with coordinates

**Format:**
```
#ID = VERTEX_POINT(vertex_geometry);
```

**Parameters:**
- `vertex_geometry`: Reference to CARTESIAN_POINT

**Example:**
```
#60 = VERTEX_POINT(#80);
```

#### 9. CARTESIAN_POINT

**Purpose:** Represents a 3D point

**Format:**
```
#ID = CARTESIAN_POINT(name, coordinates);
```

**Parameters:**
- `name`: String identifier (optional)
- `coordinates`: List of 3 coordinates [x, y, z]

**Example:**
```
#80 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
```

---

## Surface Types

### Common Surface Entities

#### 1. PLANE

**Purpose:** Represents a plane surface

**Format:**
```
#ID = PLANE(name, position);
```

**Parameters:**
- `name`: String identifier
- `position`: Reference to AXIS2_PLACEMENT_3D

**Example:**
```
#10 = PLANE('Plane1', #11);
```

#### 2. CYLINDRICAL_SURFACE

**Purpose:** Represents a cylindrical surface

**Format:**
```
#ID = CYLINDRICAL_SURFACE(name, position, radius);
```

**Parameters:**
- `name`: String identifier
- `position`: Reference to AXIS2_PLACEMENT_3D
- `radius`: Real number

**Example:**
```
#10 = CYLINDRICAL_SURFACE('Cylinder1', #11, 5.0);
```

#### 3. CONICAL_SURFACE

**Purpose:** Represents a conical surface

**Format:**
```
#ID = CONICAL_SURFACE(name, position, radius, semi_angle);
```

**Parameters:**
- `name`: String identifier
- `position`: Reference to AXIS2_PLACEMENT_3D
- `radius`: Real number
- `semi_angle`: Real number (half-angle in radians)

**Example:**
```
#10 = CONICAL_SURFACE('Cone1', #11, 3.0, 0.523599);
```

#### 4. SPHERICAL_SURFACE

**Purpose:** Represents a spherical surface

**Format:**
```
#ID = SPHERICAL_SURFACE(name, position, radius);
```

**Parameters:**
- `name`: String identifier
- `position`: Reference to AXIS2_PLACEMENT_3D
- `radius`: Real number

**Example:**
```
#10 = SPHERICAL_SURFACE('Sphere1', #11, 10.0);
```

#### 5. B_SPLINE_SURFACE

**Purpose:** Represents a B-spline surface (NURBS)

**Format:**
```
#ID = B_SPLINE_SURFACE(name, u_degree, v_degree, control_points, ...);
```

**Parameters:**
- Complex parameters for B-spline definition

**Note:** More complex, may not be fully supported initially

---

## Curve Types

### Common Curve Entities

#### 1. LINE

**Purpose:** Represents a straight line

**Format:**
```
#ID = LINE(name, point, direction);
```

**Parameters:**
- `name`: String identifier
- `point`: Reference to CARTESIAN_POINT (point on line)
- `direction`: Reference to VECTOR (direction vector)

**Example:**
```
#70 = LINE('Line1', #80, #81);
```

#### 2. CIRCLE

**Purpose:** Represents a circle

**Format:**
```
#ID = CIRCLE(name, position, radius);
```

**Parameters:**
- `name`: String identifier
- `position`: Reference to AXIS2_PLACEMENT_3D
- `radius`: Real number

**Example:**
```
#70 = CIRCLE('Circle1', #71, 5.0);
```

#### 3. ELLIPSE

**Purpose:** Represents an ellipse

**Format:**
```
#ID = ELLIPSE(name, position, semi_axis_1, semi_axis_2);
```

**Parameters:**
- `name`: String identifier
- `position`: Reference to AXIS2_PLACEMENT_3D
- `semi_axis_1`: Real number
- `semi_axis_2`: Real number

**Example:**
```
#70 = ELLIPSE('Ellipse1', #71, 5.0, 3.0);
```

#### 4. B_SPLINE_CURVE

**Purpose:** Represents a B-spline curve (NURBS)

**Format:**
```
#ID = B_SPLINE_CURVE(name, degree, control_points, ...);
```

**Parameters:**
- Complex parameters for B-spline definition

**Note:** More complex, may not be fully supported initially

---

## Reference Resolution

### Reference Format

STEP uses entity references in the format `#ID`:

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
#2 = CLOSED_SHELL('shell', (#3, #4));
```

### Reference Resolution Process

1. **Parse all entities** - Build entity table
2. **Resolve references** - Look up `#ID` in entity table
3. **Follow reference chains** - Resolve nested references
4. **Validate references** - Check all references exist

### Reference Types

- **Direct references:** `#2` - Points to entity #2
- **List references:** `(#3, #4, #5)` - List of entity references
- **Optional references:** `$` - No reference (optional parameter)

### Example Reference Chain

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
  └─ #2 = CLOSED_SHELL('shell', (#3, #4));
      ├─ #3 = FACE(#10, #20);
      │   ├─ #10 = PLANE('Plane1', #11);
      │   └─ #20 = FACE_BOUND(#30, .T.);
      │       └─ #30 = EDGE_LOOP((#40, #41, #42, #43));
      │           └─ #40 = ORIENTED_EDGE(#50, .T.);
      │               └─ #50 = EDGE(#60, #61, #70);
      │                   ├─ #60 = VERTEX_POINT(#80);
      │                   │   └─ #80 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
      │                   ├─ #61 = VERTEX_POINT(#81);
      │                   │   └─ #81 = CARTESIAN_POINT('', (1.0, 0.0, 0.0));
      │                   └─ #70 = LINE('Line1', #80, #81);
      └─ #4 = FACE(...);
```

---

## Simple STEP File Examples

### Example 1: Minimal STEP File

```
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Simple test'),'2;1');
FILE_NAME('test.step','2025-01-27T00:00:00',(''),(''),'','','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;
DATA;
#1 = MANIFOLD_SOLID_BREP('Solid1', #2);
#2 = CLOSED_SHELL('Shell1', (#3));
#3 = FACE(#10, #20);
#10 = PLANE('Plane1', #11);
#11 = AXIS2_PLACEMENT_3D('', #80, #81, #82);
#20 = FACE_BOUND(#30, .T.);
#30 = EDGE_LOOP((#40));
#40 = ORIENTED_EDGE(#50, .T.);
#50 = EDGE(#60, #61, #70);
#60 = VERTEX_POINT(#80);
#61 = VERTEX_POINT(#81);
#70 = LINE('Line1', #80, #90);
#80 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
#81 = CARTESIAN_POINT('', (1.0, 0.0, 0.0));
#82 = DIRECTION('', (0.0, 0.0, 1.0));
#90 = DIRECTION('', (1.0, 0.0, 0.0));
ENDSEC;
END-ISO-10303-21;
```

### Example 2: Cube (Conceptual)

```
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Cube'),'2;1');
FILE_NAME('cube.step','2025-01-27T00:00:00',(''),(''),'','','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;
DATA;
#1 = MANIFOLD_SOLID_BREP('Cube', #2);
#2 = CLOSED_SHELL('CubeShell', (#3, #4, #5, #6, #7, #8));
#3 = FACE('Face1', #10, #20);
#4 = FACE('Face2', #11, #21);
#5 = FACE('Face3', #12, #22);
#6 = FACE('Face4', #13, #23);
#7 = FACE('Face5', #14, #24);
#8 = FACE('Face6', #15, #25);
#10 = PLANE('Plane1', #16);
#11 = PLANE('Plane2', #17);
#12 = PLANE('Plane3', #18);
#13 = PLANE('Plane4', #19);
#14 = PLANE('Plane5', #20);
#15 = PLANE('Plane6', #21);
#16 = AXIS2_PLACEMENT_3D('', #80, #81, #82);
#17 = AXIS2_PLACEMENT_3D('', #83, #84, #85);
#18 = AXIS2_PLACEMENT_3D('', #86, #87, #88);
#19 = AXIS2_PLACEMENT_3D('', #89, #90, #91);
#20 = AXIS2_PLACEMENT_3D('', #92, #93, #94);
#21 = AXIS2_PLACEMENT_3D('', #95, #96, #97);
#20 = FACE_BOUND(#30, .T.);
#21 = FACE_BOUND(#31, .T.);
#22 = FACE_BOUND(#32, .T.);
#23 = FACE_BOUND(#33, .T.);
#24 = FACE_BOUND(#34, .T.);
#25 = FACE_BOUND(#35, .T.);
#30 = EDGE_LOOP((#40, #41, #42, #43));
#31 = EDGE_LOOP((#44, #45, #46, #47));
#32 = EDGE_LOOP((#48, #49, #50, #51));
#33 = EDGE_LOOP((#52, #53, #54, #55));
#34 = EDGE_LOOP((#56, #57, #58, #59));
#35 = EDGE_LOOP((#60, #61, #62, #63));
#40 = ORIENTED_EDGE(#70, .T.);
#41 = ORIENTED_EDGE(#71, .T.);
#42 = ORIENTED_EDGE(#72, .T.);
#43 = ORIENTED_EDGE(#73, .T.);
#70 = EDGE(#100, #101, #200);
#71 = EDGE(#101, #102, #201);
#72 = EDGE(#102, #103, #202);
#73 = EDGE(#103, #100, #203);
#100 = VERTEX_POINT(#300);
#101 = VERTEX_POINT(#301);
#102 = VERTEX_POINT(#302);
#103 = VERTEX_POINT(#303);
#300 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
#301 = CARTESIAN_POINT('', (1.0, 0.0, 0.0));
#302 = CARTESIAN_POINT('', (1.0, 1.0, 0.0));
#303 = CARTESIAN_POINT('', (0.0, 1.0, 0.0));
#200 = LINE('Line1', #300, #400);
#400 = DIRECTION('', (1.0, 0.0, 0.0));
ENDSEC;
END-ISO-10303-21;
```

**Note:** This is a simplified conceptual example. Real STEP files are more complex.

---

## Entity Relationships

### Topology Hierarchy

```
MANIFOLD_SOLID_BREP
  └─ CLOSED_SHELL
      └─ FACE (list)
          ├─ Surface (geometry)
          └─ FACE_BOUND
              └─ EDGE_LOOP
                  └─ ORIENTED_EDGE (list)
                      └─ EDGE
                          ├─ VERTEX_POINT (start)
                          ├─ VERTEX_POINT (end)
                          └─ CURVE (geometry)
                              └─ CARTESIAN_POINT (point on curve)
```

### Geometry Hierarchy

```
SURFACE
  └─ AXIS2_PLACEMENT_3D
      ├─ CARTESIAN_POINT (location)
      ├─ DIRECTION (axis)
      └─ DIRECTION (ref_direction)

CURVE
  └─ AXIS2_PLACEMENT_3D (for circles, etc.)
      └─ (similar to surface)

CARTESIAN_POINT
  └─ Coordinates: [x, y, z]
```

---

## Parameter Types

### Common Parameter Types

1. **String:** `'text'` - Enclosed in single quotes
2. **Real:** `1.0`, `-3.14` - Floating point numbers
3. **Integer:** `1`, `-5` - Integer numbers
4. **Boolean:** `.T.` (TRUE), `.F.` (FALSE)
5. **Reference:** `#1`, `#2` - Entity references
6. **List:** `(#1, #2, #3)` - List of values/references
7. **Optional:** `$` - No value (optional parameter)

### Parameter Examples

```
#1 = MANIFOLD_SOLID_BREP('Solid1', #2);
     └─ String: 'Solid1'
     └─ Reference: #2

#2 = CLOSED_SHELL('Shell1', (#3, #4, #5));
     └─ String: 'Shell1'
     └─ List of references: (#3, #4, #5)

#80 = CARTESIAN_POINT('', (0.0, 1.0, 2.0));
     └─ String: '' (empty)
     └─ List of reals: (0.0, 1.0, 2.0)
```

---

## Reference Resolution Patterns

### Pattern 1: Direct Reference

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
```

**Resolution:**
- Entity #1 references entity #2
- Look up #2 in entity table
- Resolve #2 to get CLOSED_SHELL

### Pattern 2: List Reference

```
#2 = CLOSED_SHELL('shell', (#3, #4, #5));
```

**Resolution:**
- Entity #2 references entities #3, #4, #5
- Look up each reference in entity table
- Resolve each to get FACE entities

### Pattern 3: Nested Reference

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
#2 = CLOSED_SHELL('shell', (#3));
#3 = FACE(#10, #20);
```

**Resolution:**
- Start with #1
- Resolve #2 (CLOSED_SHELL)
- Resolve #3 (FACE) from #2's face list
- Resolve #10 and #20 from #3

### Pattern 4: Circular Reference (Avoid)

```
#1 = ENTITY1('', #2);
#2 = ENTITY2('', #1);
```

**Note:** Circular references should be handled carefully. Usually not a problem if resolved correctly.

---

## Common Issues and Solutions

### Issue 1: Missing References

**Problem:** Entity references entity #100, but #100 doesn't exist

**Solution:**
- Validate all references exist
- Report missing references with clear error messages
- Handle gracefully (skip or error)

### Issue 2: Forward References

**Problem:** Entity #1 references #2, but #2 is defined later in file

**Solution:**
- Parse all entities first
- Build entity table
- Then resolve references
- STEP files can have forward references

### Issue 3: Optional Parameters

**Problem:** Entity has optional parameter marked with `$`

**Solution:**
- Check for `$` in parameter list
- Treat as `None` or skip
- Don't try to resolve as entity reference

### Issue 4: Complex Entity Types

**Problem:** Entity type not recognized or not supported

**Solution:**
- Log unsupported entity types
- Skip gracefully
- Document limitations
- Expand support incrementally

---

## Resources and References

### Official Documentation
- ISO 10303-21: STEP Physical File Format
- ISO 10303-203: AP203 (Configuration Controlled Design)
- STEP Format Specification

### Project References
- `STEP_IMPLEMENTATION_CURRENT_STATE.md` - Current implementation
- `mesh-core/src/formats/step.rs` - Current code
- `V0.2.0_STEP_READING_RESEARCH.md` - Previous research

---

## Findings Summary

### ✅ Confirmed
- STEP files have standard structure (header, data, end)
- Entities use `#ID = TYPE(...)` format
- References use `#ID` format
- Common entity types identified
- Reference chains can be followed

### ⚠️ Needs Verification
- Exact parameter formats for all entity types
- Optional parameter handling
- Complex entity types (B-splines, etc.)
- Coordinate system conventions

### ❓ Unknown
- All possible entity types
- Entity type variations
- Advanced features (assemblies, instances)
- Performance implications of reference resolution

---

## Next Steps

### Immediate Actions

1. **Test with Real STEP Files:**
   - Parse simple STEP files
   - Verify entity structure
   - Test reference resolution

2. **Document Entity Types:**
   - Create comprehensive entity type list
   - Document parameter structures
   - Note which types are supported

3. **Create Examples:**
   - Simple geometric shapes
   - Common CAD models
   - Edge cases

---

## Updates Log

| Date | Update | Status |
|------|--------|--------|
| 2025-01-27 | Initial research document created | In Progress |
| | | |

---

**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Next Update:** After testing with real STEP files  
**Target:** Provide comprehensive STEP structure reference for Riley

---

*Researcher: Sam Parker (Junior Engineer, 2D Formats)*  
*For: Riley Thompson (Junior Engineer, 3D Formats) & Senior Engineer*

