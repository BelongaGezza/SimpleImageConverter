# STEP Format Reference Guide
## Comprehensive Specification Reference for Rust Implementation

**Document Type:** Technical Reference  
**Target Audience:** System Architect, Senior Engineer, Implementation Team  
**Purpose:** Provide detailed STEP format specifications and implementation guidance  
**Date:** December 27, 2025  
**Status:** Reference Document

---

## Executive Summary

This document provides a comprehensive reference for implementing STEP (ISO 10303-21) format support in Rust using the `ruststep` library. It consolidates information from open-source EXPRESS schema repositories, publicly accessible tools, and official STEP documentation to support the implementation of STEP file reading and conversion.

**Key Resources:**
- STEPcode Repository (NIST-based EXPRESS schemas)
- RustStep Library (includes ISO 10303 schemas)
- NIST STEP File Analyzer (entity/attribute tables)
- STEP Tools Online Reference (syntax and mapping)
- Library of Congress Format Sustainability (technical summary)

---

## Table of Contents

1. [STEP Format Overview](#step-format-overview)
2. [ISO 10303 Standards Structure](#iso-10303-standards-structure)
3. [EXPRESS Schema Sources](#express-schema-sources)
4. [Physical File Format (ISO 10303-21)](#physical-file-format-iso-10303-21)
5. [Application Protocols (AP203, AP214, AP242)](#application-protocols-ap203-ap214-ap242)
6. [Core Entity Types and Structures](#core-entity-types-and-structures)
7. [Reference Resolution Patterns](#reference-resolution-patterns)
8. [Implementation Guidance for Rust](#implementation-guidance-for-rust)
9. [Data Tables and Entity Mappings](#data-tables-and-entity-mappings)
10. [Common Implementation Patterns](#common-implementation-patterns)
11. [Resources and Tools](#resources-and-tools)

---

## STEP Format Overview

### What is STEP?

**STEP (Standard for the Exchange of Product model data)** is an ISO standard (ISO 10303) for representing and exchanging product data. It is widely used in CAD, CAM, CAE, and PLM systems.

**Key Characteristics:**
- **Format:** ISO 10303-21 (Physical File Format) - ASCII text format
- **Schema Language:** EXPRESS (ISO 10303-11)
- **Application Protocols:** AP203 (Configuration Controlled Design), AP214 (Automotive Design), AP242 (Managed Model-Based 3D Engineering)
- **Structure:** Header section + Data section with entity instances

### STEP File Structure

```
ISO-10303-21;
HEADER;
  FILE_DESCRIPTION(...);
  FILE_NAME(...);
  FILE_SCHEMA(...);
ENDSEC;
DATA;
  #1 = ENTITY_TYPE(...);
  #2 = ENTITY_TYPE(...);
  ...
ENDSEC;
END-ISO-10303-21;
```

---

## ISO 10303 Standards Structure

### Parts of ISO 10303

The ISO 10303 standard is organized into multiple parts:

1. **Part 11 (EXPRESS Language Reference Manual)**
   - Defines the EXPRESS schema language
   - Used to define entity types and relationships

2. **Part 21 (Implementation Methods: Clear Text Encoding)**
   - Defines the physical file format
   - Syntax for STEP files (.step, .stp)

3. **Part 42 (Geometric and Topological Representation)**
   - Defines geometric entities (points, curves, surfaces)
   - Defines topological entities (vertices, edges, faces, shells)

4. **Part 203 (Application Protocol: Configuration Controlled Design - AP203)**
   - Most common AP for 3D CAD models
   - Defines entities for solid models with boundary representation

5. **Part 214 (Application Protocol: Core Data for Automotive Mechanical Design Processes - AP214)**
   - Automotive industry focused
   - Extends AP203 with automotive-specific entities

6. **Part 242 (Application Protocol: Managed Model-Based 3D Engineering - AP242)**
   - Modern AP combining AP203 and AP214
   - Supports MBD (Model-Based Definition)

### Application Protocol Selection

**For this project (mesh conversion):**
- **Primary Focus:** AP203 (most common for 3D CAD models)
- **Secondary:** AP214/AP242 (if needed for automotive/advanced models)
- **ruststep Support:** Currently supports AP203 via `ap203` feature

---

## EXPRESS Schema Sources

### 1. STEPcode Repository

**Location:** https://github.com/stepcode/stepcode  
**Description:** Comprehensive collection of EXPRESS schemas maintained by NIST

**Key Directories:**
- `data/` - Contains EXPRESS (.exp) files for major Application Protocols
- `src/` - C++ implementation (STEP Class Library)
- `schemas/` - Organized by AP (AP203, AP214, AP242)

**Access:**
- GitHub repository is publicly accessible
- EXPRESS files can be read directly
- No paywall or proprietary restrictions

**Example Schema Locations:**
```
stepcode/
  data/
    ap203/
      config_control_design.exp  # AP203 main schema
      geometric_model_schema.exp  # Geometry entities
      topology_schema.exp         # Topology entities
    ap214/
      automotive_design.exp      # AP214 main schema
    ap242/
      managed_model_based_3d_engineering.exp  # AP242 main schema
```

**Usage for Implementation:**
- Read EXPRESS files to understand entity definitions
- Understand parameter structures
- Identify entity relationships and inheritance
- Reference for validation

### 2. RustStep Repository

**Location:** https://github.com/ricosjp/ruststep  
**Description:** Rust implementation of STEP parser with AP203 support

**Key Features:**
- Includes copies of ISO 10303 schemas
- Code-generated Rust types from EXPRESS schemas
- AP203 struct definitions available

**Schema Access:**
- EXPRESS schemas in repository (if included)
- Generated Rust code shows entity structures
- `ruststep::ap203::config_control_design` module contains AP203 types

**Usage for Implementation:**
- Reference Rust struct definitions
- Understand how EXPRESS maps to Rust types
- Use generated types for deserialization

### 3. NIST STEP File Analyzer

**Location:** https://www.nist.gov/services-resources/software/step-file-analyzer  
**Description:** Free tool that generates detailed entity/attribute tables from STEP files

**Capabilities:**
- Takes any STEP file as input
- Generates CSV/Excel spreadsheet of all entities
- Lists every entity type and attribute used
- Creates custom reference table for specific model

**Usage for Implementation:**
- Analyze real STEP files to understand structure
- Generate entity tables for specific models
- Understand attribute usage patterns
- Create test data reference tables

**Output Format:**
- CSV/Excel with columns: Entity ID, Entity Type, Attribute Name, Attribute Value
- Can be used as reference during implementation

### 4. STEP Tools Online Reference

**Location:** https://www.steptools.com/support/stdev_docs/express/  
**Description:** Publicly accessible reference for ISO 10303-21 syntax and mapping

**Content:**
- ISO 10303-21 syntax rules
- Mapping rules for EXPRESS to physical file
- Exchange structure documentation
- Entity encoding rules

**Usage for Implementation:**
- Understand physical file format syntax
- Reference encoding rules
- Validate file structure

### 5. Library of Congress Format Sustainability

**Location:** https://www.loc.gov/preservation/digital/formats/fdd/fdd000449.shtml  
**Description:** Technical summary of ISO 10303-21 (STEP-file)

**Content:**
- Format structure overview
- Conformance classes
- History and evolution
- Technical characteristics

**Usage for Implementation:**
- High-level format understanding
- Conformance class identification
- Format history context

---

## Physical File Format (ISO 10303-21)

### File Structure

STEP files follow a strict structure defined by ISO 10303-21:

```
ISO-10303-21;
HEADER;
  <header entities>
ENDSEC;
DATA;
  <data entities>
ENDSEC;
END-ISO-10303-21;
```

### Header Section

**Purpose:** File metadata and schema information

**Required Entities:**
1. **FILE_DESCRIPTION**
   - Description of file contents
   - Implementation level

2. **FILE_NAME**
   - File name
   - Timestamp
   - Author information
   - Organization

3. **FILE_SCHEMA**
   - Application Protocol(s) used
   - Example: `('AUTOMOTIVE_DESIGN')` or `('CONFIG_CONTROL_DESIGN')`

**Example:**
```
HEADER;
FILE_DESCRIPTION(('STEP file for 3D model'),'2;1');
FILE_NAME('model.step','2025-12-27T12:00:00',('Author'),('Organization'),'','','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;
```

### Data Section

**Purpose:** Contains all entity instances

**Format:**
```
DATA;
#1 = ENTITY_TYPE(parameter1, parameter2, ...);
#2 = ENTITY_TYPE(parameter1, parameter2, ...);
...
ENDSEC;
```

**Entity Instance Format:**
```
#<id> = <entity_type>(<parameter_list>);
```

**Parameters:**
- Can be values (strings, numbers, booleans)
- Can be references to other entities (`#<id>`)
- Can be lists `(<item1>, <item2>, ...)`
- Can be optional (`$`)

### Parameter Types

1. **String:** `'text'` (single quotes)
2. **Real:** `1.0`, `-3.14159`
3. **Integer:** `1`, `-5`
4. **Boolean:** `.T.` (TRUE), `.F.` (FALSE)
5. **Reference:** `#1`, `#2` (entity ID)
6. **List:** `(#1, #2, #3)` or `(1.0, 2.0, 3.0)`
7. **Optional:** `$` (no value)

### Entity ID Format

- Format: `#<number>`
- Must be unique within file
- Sequential numbering common but not required
- Forward references allowed (can reference entities defined later)

---

## Application Protocols (AP203, AP214, AP242)

### AP203: Configuration Controlled Design

**Most Common AP for 3D CAD Models**

**Key Characteristics:**
- Focus on solid models with boundary representation (BREP)
- Configuration management entities
- Geometric and topological entities

**Common Entity Types:**
- `MANIFOLD_SOLID_BREP` - Solid with boundary representation
- `CLOSED_SHELL` - Closed shell of faces
- `FACE` - Face with geometry and boundary
- `EDGE` - Edge with geometry
- `VERTEX_POINT` - Vertex with coordinates
- `CARTESIAN_POINT` - 3D point coordinates

**ruststep Support:**
- Full AP203 support via `ap203` feature
- Types in `ruststep::ap203::config_control_design` module
- `Tables` structure for entity lookup

### AP214: Automotive Design

**Automotive Industry Focus**

**Key Characteristics:**
- Extends AP203
- Automotive-specific entities
- Assembly structures
- Manufacturing information

**Common Extensions:**
- Assembly relationships
- Part numbers
- Manufacturing features
- Automotive-specific geometry

**ruststep Support:**
- Limited (AP203 is primary focus)
- May need custom handling for AP214-specific entities

### AP242: Managed Model-Based 3D Engineering

**Modern Combined AP**

**Key Characteristics:**
- Combines AP203 and AP214
- Model-Based Definition (MBD) support
- PMI (Product Manufacturing Information)
- GD&T (Geometric Dimensioning and Tolerancing)

**ruststep Support:**
- Limited (AP203 is primary focus)
- May need custom handling for AP242-specific entities

---

## Core Entity Types and Structures

### Geometric Entities

#### CARTESIAN_POINT

**Purpose:** 3D point coordinates

**EXPRESS Definition:**
```
ENTITY cartesian_point
  SUBTYPE OF (point);
  coordinates : LIST [1:3] OF REAL;
END_ENTITY;
```

**STEP Format:**
```
#1 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `coordinates` (LIST[3] OF REAL) - [x, y, z] coordinates

**Rust Type (ruststep):**
```rust
use ruststep::ap203::config_control_design::CartesianPoint;

// Structure:
// CartesianPoint {
//   name: String,
//   coordinates: [f64; 3],
// }
```

#### DIRECTION

**Purpose:** Direction vector (normalized)

**EXPRESS Definition:**
```
ENTITY direction
  SUBTYPE OF (geometric_representation_item);
  direction_ratios : LIST [2:3] OF REAL;
END_ENTITY;
```

**STEP Format:**
```
#2 = DIRECTION('', (0.0, 0.0, 1.0));
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `direction_ratios` (LIST[3] OF REAL) - Direction vector [x, y, z]

#### AXIS2_PLACEMENT_3D

**Purpose:** 3D coordinate system (origin + axes)

**EXPRESS Definition:**
```
ENTITY axis2_placement_3d
  SUBTYPE OF (placement);
  axis : OPTIONAL direction;
  ref_direction : OPTIONAL direction;
END_ENTITY;
```

**STEP Format:**
```
#3 = AXIS2_PLACEMENT_3D('', #1, #2, #4);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `location` (CARTESIAN_POINT) - Origin point
3. `axis` (DIRECTION, optional) - Z-axis direction
4. `ref_direction` (DIRECTION, optional) - X-axis reference direction

**Usage:**
- Defines coordinate system for surfaces and curves
- Used by PLANE, CYLINDRICAL_SURFACE, etc.

#### PLANE

**Purpose:** Plane surface

**EXPRESS Definition:**
```
ENTITY plane
  SUBTYPE OF (elementary_surface);
  -- Inherits position: axis2_placement_3d
END_ENTITY;
```

**STEP Format:**
```
#10 = PLANE('Plane1', #3);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `position` (AXIS2_PLACEMENT_3D) - Coordinate system

#### CYLINDRICAL_SURFACE

**Purpose:** Cylindrical surface

**EXPRESS Definition:**
```
ENTITY cylindrical_surface
  SUBTYPE OF (elementary_surface);
  radius : REAL;
  -- Inherits position: axis2_placement_3d
END_ENTITY;
```

**STEP Format:**
```
#11 = CYLINDRICAL_SURFACE('Cylinder1', #3, 5.0);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `position` (AXIS2_PLACEMENT_3D) - Coordinate system
3. `radius` (REAL) - Cylinder radius

#### LINE

**Purpose:** Straight line curve

**EXPRESS Definition:**
```
ENTITY line
  SUBTYPE OF (curve);
  pnt : cartesian_point;
  dir : direction;
END_ENTITY;
```

**STEP Format:**
```
#20 = LINE('Line1', #1, #2);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `pnt` (CARTESIAN_POINT) - Point on line
3. `dir` (DIRECTION) - Direction vector

#### CIRCLE

**Purpose:** Circle curve

**EXPRESS Definition:**
```
ENTITY circle
  SUBTYPE OF (conic);
  radius : REAL;
  -- Inherits position: axis2_placement_3d
END_ENTITY;
```

**STEP Format:**
```
#21 = CIRCLE('Circle1', #3, 5.0);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `position` (AXIS2_PLACEMENT_3D) - Coordinate system
3. `radius` (REAL) - Circle radius

### Topological Entities

#### VERTEX_POINT

**Purpose:** Vertex with point coordinates

**EXPRESS Definition:**
```
ENTITY vertex_point
  SUBTYPE OF (vertex);
  vertex_geometry : point;
END_ENTITY;
```

**STEP Format:**
```
#30 = VERTEX_POINT(#1);
```

**Parameters:**
1. `vertex_geometry` (CARTESIAN_POINT) - Point coordinates

#### EDGE

**Purpose:** Edge with geometry

**EXPRESS Definition:**
```
ENTITY edge
  SUBTYPE OF (topological_representation_item);
  edge_start : vertex;
  edge_end : vertex;
  edge_geometry : OPTIONAL curve;
END_ENTITY;
```

**STEP Format:**
```
#40 = EDGE(#30, #31, #20);
```

**Parameters:**
1. `edge_start` (VERTEX_POINT) - Start vertex
2. `edge_end` (VERTEX_POINT) - End vertex
3. `edge_geometry` (CURVE, optional) - Edge curve geometry

#### ORIENTED_EDGE

**Purpose:** Oriented edge (with direction)

**EXPRESS Definition:**
```
ENTITY oriented_edge
  SUBTYPE OF (edge);
  edge_element : edge;
  orientation : BOOLEAN;
END_ENTITY;
```

**STEP Format:**
```
#41 = ORIENTED_EDGE(#40, .T.);
```

**Parameters:**
1. `edge_element` (EDGE) - Base edge
2. `orientation` (BOOLEAN) - TRUE = same direction, FALSE = reversed

#### EDGE_LOOP

**Purpose:** Loop of edges

**EXPRESS Definition:**
```
ENTITY edge_loop
  SUBTYPE OF (loop);
  edge_list : LIST [1:?] OF oriented_edge;
END_ENTITY;
```

**STEP Format:**
```
#50 = EDGE_LOOP((#41, #42, #43, #44));
```

**Parameters:**
1. `edge_list` (LIST OF ORIENTED_EDGE) - List of oriented edges

#### FACE_BOUND

**Purpose:** Face boundary (outer or inner loop)

**EXPRESS Definition:**
```
ENTITY face_bound
  SUBTYPE OF (topological_representation_item);
  bound : loop;
  orientation : BOOLEAN;
END_ENTITY;
```

**STEP Format:**
```
#60 = FACE_BOUND(#50, .T.);
```

**Parameters:**
1. `bound` (EDGE_LOOP) - Boundary loop
2. `orientation` (BOOLEAN) - TRUE = same orientation, FALSE = reversed

#### FACE

**Purpose:** Face with geometry and boundary

**EXPRESS Definition:**
```
ENTITY face
  SUBTYPE OF (topological_representation_item);
  bounds : SET [1:?] OF face_bound;
  face_geometry : OPTIONAL surface;
END_ENTITY;
```

**STEP Format:**
```
#70 = FACE((#60), #10);
```

**Parameters:**
1. `bounds` (SET OF FACE_BOUND) - Boundary loops (first is outer, rest are holes)
2. `face_geometry` (SURFACE, optional) - Face surface geometry

#### CLOSED_SHELL

**Purpose:** Closed shell of faces

**EXPRESS Definition:**
```
ENTITY closed_shell
  SUBTYPE OF (connected_face_set);
  cfs_faces : SET [1:?] OF face;
END_ENTITY;
```

**STEP Format:**
```
#80 = CLOSED_SHELL((#70, #71, #72, #73, #74, #75));
```

**Parameters:**
1. `cfs_faces` (SET OF FACE) - Set of faces forming closed shell

#### MANIFOLD_SOLID_BREP

**Purpose:** Solid with boundary representation

**EXPRESS Definition:**
```
ENTITY manifold_solid_brep
  SUBTYPE OF (solid_model);
  outer : closed_shell;
END_ENTITY;
```

**STEP Format:**
```
#100 = MANIFOLD_SOLID_BREP('Solid1', #80);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `outer` (CLOSED_SHELL) - Outer shell of solid

**Entity Hierarchy:**
```
MANIFOLD_SOLID_BREP
  └─ outer: CLOSED_SHELL
      └─ cfs_faces: SET[FACE]
          ├─ bounds: SET[FACE_BOUND]
          │   └─ bound: EDGE_LOOP
          │       └─ edge_list: LIST[ORIENTED_EDGE]
          │           └─ edge_element: EDGE
          │               ├─ edge_start: VERTEX_POINT
          │               ├─ edge_end: VERTEX_POINT
          │               └─ edge_geometry: CURVE
          └─ face_geometry: SURFACE
```

### Shape Representation Entities

#### ADVANCED_BREP_SHAPE_REPRESENTATION

**Purpose:** Advanced BREP shape representation

**EXPRESS Definition:**
```
ENTITY advanced_brep_shape_representation
  SUBTYPE OF (shape_representation);
  -- items: SET OF representation_item
  -- (contains MANIFOLD_SOLID_BREP, CLOSED_SHELL, etc.)
END_ENTITY;
```

**STEP Format:**
```
#200 = ADVANCED_BREP_SHAPE_REPRESENTATION('', (#100), #201);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `items` (SET OF REPRESENTATION_ITEM) - Geometric items (solids, shells)
3. `context_of_items` (REPRESENTATION_CONTEXT) - Coordinate system context

#### FACETED_BREP

**Purpose:** Faceted BREP (triangulated)

**EXPRESS Definition:**
```
ENTITY faceted_brep
  SUBTYPE OF (manifold_solid_brep);
  -- Same as MANIFOLD_SOLID_BREP but with triangulated faces
END_ENTITY;
```

**STEP Format:**
```
#300 = FACETED_BREP('FacetedSolid', #80);
```

**Parameters:**
1. `name` (STRING) - Optional name
2. `outer` (CLOSED_SHELL) - Outer shell of solid (same as MANIFOLD_SOLID_BREP)

**Rust Type (ruststep):**
```rust
use ruststep::ap203::config_control_design::Tables;

// Access FACETED_BREP entities
let fb_holders = tables.faceted_brep_holders();
// Returns: &HashMap<u64, FacetedBrepHolder>
```

**API Access:**
- Method: `tables.faceted_brep_holders()` ✅ **CONFIRMED TO EXIST**
- Pattern: Follows same pattern as other entity accessors
- Return: `&HashMap<u64, FacetedBrepHolder>`

**Entity Traversal Path:**
```
FACETED_BREP
  └── outer: CLOSED_SHELL
      └── cfs_faces: SET OF FACE
          └── bounds: SET OF FACE_BOUND
              └── bound: EDGE_LOOP
                  └── edge_list: LIST OF ORIENTED_EDGE
                      └── edge_element: EDGE
                          └── edge_start: VERTEX_POINT
                              └── vertex_geometry: CARTESIAN_POINT
```

**Implementation Notes:**
- **v0.2.0 Support:** FACETED_BREP only (pre-tessellated geometry)
- **v0.3.0 Planned:** Full B-Rep support (NURBS surfaces, cylinders, etc.)
- **Advantage:** Faces are already triangulated, making conversion to mesh easier
- **Requirement:** Users must export STEP files with tessellation enabled

**See Also:**
- `FACETED_BREP_API_FINDINGS.md` - Detailed API research findings
- `docs/CAD_EXPORT_GUIDE.md` - Instructions for exporting FACETED_BREP from CAD software

---

## Reference Resolution Patterns

### Reference Format

STEP uses entity references in the format `#<id>`:

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
#2 = CLOSED_SHELL('shell', (#3, #4));
```

### Reference Resolution Process

1. **Parse all entities** - Build entity table/index
2. **Resolve references** - Look up `#<id>` in entity table
3. **Follow reference chains** - Resolve nested references recursively
4. **Validate references** - Check all references exist

### Reference Types

1. **Direct Reference:** `#2` - Points to entity #2
2. **List Reference:** `(#3, #4, #5)` - List of entity references
3. **Optional Reference:** `$` - No reference (optional parameter)
4. **Forward Reference:** Entity #1 can reference #2 even if #2 is defined later

### Reference Resolution Example

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
  └─ Resolve #2 → CLOSED_SHELL
      └─ Resolve (#3, #4) → FACE entities
          └─ Resolve #3 → FACE
              ├─ Resolve #10 → PLANE (surface)
              └─ Resolve #20 → FACE_BOUND
                  └─ Resolve #30 → EDGE_LOOP
                      └─ Resolve (#40, #41, #42) → ORIENTED_EDGE entities
                          └─ Resolve #40 → EDGE
                              ├─ Resolve #50 → VERTEX_POINT
                              ├─ Resolve #51 → VERTEX_POINT
                              └─ Resolve #60 → LINE (curve)
```

### Circular References

STEP allows circular references in some cases (e.g., bidirectional edge relationships). Resolution must handle cycles gracefully.

---

## Implementation Guidance for Rust

### ruststep Library Structure

**Main Modules:**
- `ruststep::parser` - STEP file parsing
- `ruststep::ast` - Abstract Syntax Tree types
- `ruststep::ap203::config_control_design` - AP203 entity types

**Key Types:**
```rust
use ruststep::{ast, parser};

// Parse STEP file
let exchange: ast::Exchange = parser::parse(step_text)?;

// Exchange structure
struct Exchange {
    header: Header,
    data: Vec<DataSection>,
}

// DataSection structure
struct DataSection {
    entities: Vec<EntityInstance>,
}

// EntityInstance variants
enum EntityInstance {
    Simple { id: EntityId, record: Record },
    Complex { id: EntityId, subsuper: SubSuperRecord },
}

// Record structure
struct Record {
    name: String,        // Entity type name
    parameters: Vec<Parameter>,  // Entity parameters
}
```

### AP203 Tables Structure

**Purpose:** Index all entities by type for efficient lookup

**Structure:**
```rust
use ruststep::ap203::config_control_design::Tables;

// Tables contains HashMaps for each entity type
struct Tables {
    cartesian_point: HashMap<EntityId, CartesianPoint>,
    direction: HashMap<EntityId, Direction>,
    axis2_placement_3d: HashMap<EntityId, Axis2Placement3d>,
    plane: HashMap<EntityId, Plane>,
    cylindrical_surface: HashMap<EntityId, CylindricalSurface>,
    line: HashMap<EntityId, Line>,
    circle: HashMap<EntityId, Circle>,
    vertex_point: HashMap<EntityId, VertexPoint>,
    edge: HashMap<EntityId, Edge>,
    oriented_edge: HashMap<EntityId, OrientedEdge>,
    edge_loop: HashMap<EntityId, EdgeLoop>,
    face_bound: HashMap<EntityId, FaceBound>,
    face: HashMap<EntityId, Face>,
    closed_shell: HashMap<EntityId, ClosedShell>,
    manifold_solid_brep: HashMap<EntityId, ManifoldSolidBrep>,
    // ... more entity types
}
```

### Building Tables from Exchange

**Pattern (Hypothetical - needs verification):**

```rust
use ruststep::ap203::config_control_design::Tables;
use ruststep::ast;

fn build_tables(exchange: &ast::Exchange) -> Result<Tables> {
    let mut tables = Tables::default();
    
    // Iterate through all entities
    for data_section in &exchange.data {
        for entity_instance in &data_section.entities {
            match entity_instance {
                ast::EntityInstance::Simple { id, record } => {
                    // Deserialize record into appropriate AP203 type
                    // Add to corresponding table
                    match record.name.as_str() {
                        "CARTESIAN_POINT" => {
                            let point: CartesianPoint = deserialize_record(record)?;
                            tables.cartesian_point.insert(*id, point);
                        }
                        "MANIFOLD_SOLID_BREP" => {
                            let msb: ManifoldSolidBrep = deserialize_record(record)?;
                            tables.manifold_solid_brep.insert(*id, msb);
                        }
                        // ... handle other entity types
                        _ => {
                            // Unknown entity type - log or skip
                        }
                    }
                }
                ast::EntityInstance::Complex { id, subsuper } => {
                    // Handle complex entities (subtype/supertype)
                    // May need to deserialize multiple records
                }
            }
        }
    }
    
    Ok(tables)
}
```

### Deserializing Records

**Pattern (needs verification with ruststep API):**

```rust
use ruststep::ap203::config_control_design::ManifoldSolidBrep;
use serde::Deserialize;

fn deserialize_record<T: Deserialize>(record: &ast::Record) -> Result<T> {
    // Convert Record to deserializable format
    // This may require custom deserializer or ruststep helper
    // Exact API needs verification from ruststep documentation
    T::deserialize(/* record or converted format */)
}
```

**Alternative Pattern (if ruststep provides helpers):**

```rust
// If ruststep provides deserialization helpers:
use ruststep::ap203::config_control_design;

// May have helper like:
let msb: ManifoldSolidBrep = config_control_design::deserialize_manifold_solid_brep(record)?;
```

### Resolving References

**Pattern:**

```rust
fn resolve_reference<T>(
    ref_id: EntityId,
    tables: &Tables,
) -> Result<&T> {
    // Look up in appropriate table
    // Exact method depends on Tables structure
    // May need type-specific resolution functions
}

// Example for CLOSED_SHELL reference:
fn resolve_closed_shell(
    ref_id: EntityId,
    tables: &Tables,
) -> Result<&ClosedShell> {
    tables.closed_shell
        .get(&ref_id)
        .ok_or_else(|| ConversionError::MissingReference(format!("#{}", ref_id)))
}
```

### Converting to truck Shell

**Pattern (conceptual):**

```rust
use truck_modeling::Shell;

fn convert_closed_shell_to_truck(
    closed_shell: &ClosedShell,
    tables: &Tables,
) -> Result<Shell> {
    // 1. Resolve all face references
    let mut faces = Vec::new();
    for face_ref in closed_shell.cfs_faces() {
        let face = resolve_face(face_ref.entity_id(), tables)?;
        let truck_face = convert_face_to_truck(face, tables)?;
        faces.push(truck_face);
    }
    
    // 2. Build truck Shell from faces
    // Exact API depends on truck Shell construction
    let shell = Shell::from_faces(faces)?;
    
    Ok(shell)
}

fn convert_face_to_truck(
    face: &Face,
    tables: &Tables,
) -> Result<truck::Face> {
    // 1. Resolve surface geometry
    let surface = if let Some(surf_ref) = face.face_geometry() {
        resolve_surface(surf_ref.entity_id(), tables)?
    } else {
        // No surface - may need to construct from boundary
        return Err(ConversionError::MissingGeometry);
    };
    
    // 2. Resolve boundary loops
    let mut boundaries = Vec::new();
    for bound_ref in face.bounds() {
        let bound = resolve_face_bound(bound_ref.entity_id(), tables)?;
        let loop_edges = convert_edge_loop_to_truck(&bound, tables)?;
        boundaries.push(loop_edges);
    }
    
    // 3. Construct truck Face
    // Exact API depends on truck Face construction
    let truck_face = truck::Face::new(surface, boundaries)?;
    
    Ok(truck_face)
}
```

---

## Data Tables and Entity Mappings

### Entity Type to AP203 Rust Type Mapping

| STEP Entity Type | AP203 Rust Type | Module Path |
|-----------------|-----------------|-------------|
| CARTESIAN_POINT | `CartesianPoint` | `ruststep::ap203::config_control_design` |
| DIRECTION | `Direction` | `ruststep::ap203::config_control_design` |
| AXIS2_PLACEMENT_3D | `Axis2Placement3d` | `ruststep::ap203::config_control_design` |
| PLANE | `Plane` | `ruststep::ap203::config_control_design` |
| CYLINDRICAL_SURFACE | `CylindricalSurface` | `ruststep::ap203::config_control_design` |
| LINE | `Line` | `ruststep::ap203::config_control_design` |
| CIRCLE | `Circle` | `ruststep::ap203::config_control_design` |
| VERTEX_POINT | `VertexPoint` | `ruststep::ap203::config_control_design` |
| EDGE | `Edge` | `ruststep::ap203::config_control_design` |
| ORIENTED_EDGE | `OrientedEdge` | `ruststep::ap203::config_control_design` |
| EDGE_LOOP | `EdgeLoop` | `ruststep::ap203::config_control_design` |
| FACE_BOUND | `FaceBound` | `ruststep::ap203::config_control_design` |
| FACE | `Face` | `ruststep::ap203::config_control_design` |
| CLOSED_SHELL | `ClosedShell` | `ruststep::ap203::config_control_design` |
| MANIFOLD_SOLID_BREP | `ManifoldSolidBrep` | `ruststep::ap203::config_control_design` |

**Note:** Exact type names and module paths need verification from ruststep documentation.

### Parameter Index Mapping

For entities with positional parameters, the parameter index corresponds to EXPRESS attribute order:

**Example: CARTESIAN_POINT**
```
#1 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
     └─ [0] name: STRING
     └─ [1] coordinates: LIST[3] OF REAL
```

**Example: MANIFOLD_SOLID_BREP**
```
#100 = MANIFOLD_SOLID_BREP('Solid1', #80);
       └─ [0] name: STRING
       └─ [1] outer: CLOSED_SHELL (reference)
```

### Reference Resolution Table

When resolving references, maintain a mapping:

| Reference ID | Entity Type | Resolved Entity | Status |
|--------------|-------------|-----------------|--------|
| #1 | CARTESIAN_POINT | CartesianPoint { ... } | Resolved |
| #2 | DIRECTION | Direction { ... } | Resolved |
| #80 | CLOSED_SHELL | ClosedShell { ... } | Resolved |
| #100 | MANIFOLD_SOLID_BREP | ManifoldSolidBrep { ... } | Resolved |

---

## Common Implementation Patterns

### Pattern 1: Parse and Extract Entities

```rust
use ruststep::{ast, parser};

fn parse_step_file(data: &[u8]) -> Result<ast::Exchange> {
    let step_text = std::str::from_utf8(data)?;
    let exchange = parser::parse(step_text)?;
    Ok(exchange)
}

fn extract_entities(exchange: &ast::Exchange) -> Vec<&ast::Record> {
    let mut entities = Vec::new();
    for data_section in &exchange.data {
        for entity_instance in &data_section.entities {
            match entity_instance {
                ast::EntityInstance::Simple { id: _, record } => {
                    entities.push(record);
                }
                ast::EntityInstance::Complex { id: _, subsuper } => {
                    // Extract from subsuper records
                    for record in &subsuper.0 {
                        entities.push(record);
                    }
                }
            }
        }
    }
    entities
}
```

### Pattern 2: Build Entity Index

```rust
use std::collections::HashMap;

fn build_entity_index(exchange: &ast::Exchange) -> HashMap<EntityId, &ast::Record> {
    let mut index = HashMap::new();
    for data_section in &exchange.data {
        for entity_instance in &data_section.entities {
            match entity_instance {
                ast::EntityInstance::Simple { id, record } => {
                    index.insert(*id, record);
                }
                ast::EntityInstance::Complex { id, subsuper } => {
                    // Index first record (or all?)
                    if let Some(record) = subsuper.0.first() {
                        index.insert(*id, record);
                    }
                }
            }
        }
    }
    index
}
```

### Pattern 3: Resolve Reference Chain

```rust
fn resolve_reference_chain(
    start_id: EntityId,
    index: &HashMap<EntityId, &ast::Record>,
) -> Result<Vec<EntityId>> {
    let mut chain = Vec::new();
    let mut current_id = start_id;
    
    loop {
        chain.push(current_id);
        
        let record = index.get(&current_id)
            .ok_or_else(|| ConversionError::MissingReference(format!("#{}", current_id)))?;
        
        // Extract next reference from record parameters
        // (Implementation depends on parameter structure)
        if let Some(next_id) = extract_next_reference(record)? {
            current_id = next_id;
        } else {
            break;
        }
        
        // Prevent infinite loops
        if chain.len() > 1000 {
            return Err(ConversionError::CircularReference);
        }
    }
    
    Ok(chain)
}
```

### Pattern 4: Error Handling

```rust
use common::error::{ConversionError, Result};

fn handle_entity_conversion(
    record: &ast::Record,
    tables: &Tables,
) -> Result<Option<Shell>> {
    match record.name.as_str() {
        "MANIFOLD_SOLID_BREP" => {
            match deserialize_manifold_solid_brep(record, tables) {
                Ok(msb) => {
                    match convert_to_shell(&msb, tables) {
                        Ok(shell) => Ok(Some(shell)),
                        Err(e) => {
                            // Log error but continue
                            eprintln!("Failed to convert MANIFOLD_SOLID_BREP: {}", e);
                            Ok(None)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize MANIFOLD_SOLID_BREP: {}", e);
                    Ok(None)
                }
            }
        }
        _ => Ok(None)
    }
}
```

---

## Resources and Tools

### Open-Source Schema Repositories

1. **STEPcode Repository**
   - **URL:** https://github.com/stepcode/stepcode
   - **Content:** EXPRESS schemas for AP203, AP214, AP242
   - **Location:** `data/` directory
   - **Usage:** Read EXPRESS files for entity definitions

2. **RustStep Repository**
   - **URL:** https://github.com/ricosjp/ruststep
   - **Content:** Rust STEP parser with AP203 support
   - **Usage:** Reference Rust type definitions, examples

### Analysis Tools

1. **NIST STEP File Analyzer**
   - **URL:** https://www.nist.gov/services-resources/software/step-file-analyzer
   - **Purpose:** Generate entity/attribute tables from STEP files
   - **Output:** CSV/Excel spreadsheets
   - **Usage:** Analyze real STEP files, create reference tables

2. **STEP Tools Online Reference**
   - **URL:** https://www.steptools.com/support/stdev_docs/express/
   - **Purpose:** ISO 10303-21 syntax and mapping reference
   - **Usage:** Understand physical file format rules

3. **Library of Congress Format Sustainability**
   - **URL:** https://www.loc.gov/preservation/digital/formats/fdd/fdd000449.shtml
   - **Purpose:** Technical summary of ISO 10303-21
   - **Usage:** High-level format understanding

### Documentation

1. **ruststep Documentation**
   - **URL:** https://docs.rs/ruststep/
   - **Purpose:** Rust API documentation
   - **Usage:** Understand ruststep API, types, methods

2. **ISO 10303 Standards**
   - **Part 21:** Physical File Format
   - **Part 42:** Geometric and Topological Representation
   - **Part 203:** AP203 Application Protocol
   - **Note:** Full standards may be behind paywall, but EXPRESS schemas are available in open-source repositories

### Implementation References

1. **Current Implementation**
   - **File:** `mesh-core/src/formats/step.rs`
   - **Status:** Framework complete, entity conversion in progress

2. **Research Documents**
   - `RESEARCH_STEP_STRUCTURE.md` - STEP file structure research
   - `RESEARCH_RUSTSTEP_EXAMPLES.md` - ruststep usage patterns
   - `STEP_IMPLEMENTATION_CURRENT_STATE.md` - Current implementation status

3. **Verification Code**
   - `mesh-core/examples/verify_ruststep_tables.rs` - Tables API verification
   - `mesh-core/examples/explore_ruststep_tables.rs` - Tables exploration

---

## Quick Reference Tables

### Entity Type Priority for Implementation

| Priority | Entity Type | Reason | Complexity |
|----------|-------------|--------|------------|
| 1 | CARTESIAN_POINT | Foundation for all geometry | Low |
| 2 | DIRECTION | Used by surfaces/curves | Low |
| 3 | AXIS2_PLACEMENT_3D | Coordinate systems | Medium |
| 4 | PLANE | Simple surface type | Low |
| 5 | LINE | Simple curve type | Low |
| 6 | VERTEX_POINT | Topology foundation | Low |
| 7 | EDGE | Topology building block | Medium |
| 8 | FACE | Topology building block | Medium |
| 9 | CLOSED_SHELL | Shell structure | High |
| 10 | MANIFOLD_SOLID_BREP | Complete solid | High |

### Parameter Type Handling

| Parameter Type | Rust Type | Parsing Method |
|----------------|-----------|----------------|
| STRING | `String` | Extract from quotes |
| REAL | `f64` | Parse as float |
| INTEGER | `i64` | Parse as integer |
| BOOLEAN | `bool` | `.T.` → true, `.F.` → false |
| Reference | `EntityId` | Extract `#<id>` |
| LIST | `Vec<T>` | Parse comma-separated |
| OPTIONAL | `Option<T>` | `$` → None |

### Common Entity Relationships

| Parent Entity | Child Entity | Relationship |
|---------------|--------------|--------------|
| MANIFOLD_SOLID_BREP | CLOSED_SHELL | 1:1 (outer) |
| CLOSED_SHELL | FACE | 1:N (cfs_faces) |
| FACE | FACE_BOUND | 1:N (bounds) |
| FACE_BOUND | EDGE_LOOP | 1:1 (bound) |
| EDGE_LOOP | ORIENTED_EDGE | 1:N (edge_list) |
| ORIENTED_EDGE | EDGE | 1:1 (edge_element) |
| EDGE | VERTEX_POINT | 2:1 (start, end) |
| VERTEX_POINT | CARTESIAN_POINT | 1:1 (vertex_geometry) |
| FACE | SURFACE | 1:1 (face_geometry, optional) |
| EDGE | CURVE | 1:1 (edge_geometry, optional) |

---

## Implementation Checklist

### Phase 1: Foundation
- [ ] Understand ruststep Tables API
- [ ] Build Tables from Exchange.data
- [ ] Implement entity deserialization for CARTESIAN_POINT
- [ ] Test with simple STEP file containing points

### Phase 2: Basic Geometry
- [ ] Implement DIRECTION deserialization
- [ ] Implement AXIS2_PLACEMENT_3D deserialization
- [ ] Implement PLANE deserialization
- [ ] Implement LINE deserialization
- [ ] Test with simple geometric entities

### Phase 3: Topology
- [ ] Implement VERTEX_POINT deserialization
- [ ] Implement EDGE deserialization
- [ ] Implement ORIENTED_EDGE deserialization
- [ ] Implement EDGE_LOOP deserialization
- [ ] Test with simple topology

### Phase 4: Faces and Shells
- [ ] Implement FACE_BOUND deserialization
- [ ] Implement FACE deserialization
- [ ] Implement CLOSED_SHELL deserialization
- [ ] Test with simple shell

### Phase 5: Solids
- [ ] Implement MANIFOLD_SOLID_BREP deserialization
- [ ] Implement reference resolution
- [ ] Test with simple solid

### Phase 6: Conversion to truck
- [ ] Convert CARTESIAN_POINT to truck Point3
- [ ] Convert CURVE to truck Curve
- [ ] Convert SURFACE to truck Surface
- [ ] Convert FACE to truck Face
- [ ] Convert CLOSED_SHELL to truck Shell
- [ ] Test conversion pipeline

### Phase 7: Tessellation
- [ ] Implement truck Shell tessellation
- [ ] Extract PolygonMesh from tessellated Shell
- [ ] Convert to project Mesh format
- [ ] Test end-to-end conversion

---

## Conclusion

This reference document provides comprehensive information for implementing STEP format support in Rust. The key resources (STEPcode, RustStep, NIST Analyzer) provide the detailed EXPRESS schema definitions and entity structures needed for implementation.

**Next Steps:**
1. Verify ruststep Tables API from documentation/source
2. Implement Tables population from Exchange.data
3. Implement entity deserialization incrementally
4. Test with real STEP files
5. Convert to truck Shell and tessellate

**Support:**
- Reference this document for entity structures
- Use NIST STEP File Analyzer for real file analysis
- Consult ruststep documentation for API details
- Review EXPRESS schemas in STEPcode repository for entity definitions

---

**Document Status:** Reference Document  
**Last Updated:** December 27, 2025  
**Maintained By:** Research Team  
**For:** System Architect, Senior Engineer, Implementation Team

