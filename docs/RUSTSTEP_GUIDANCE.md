# ruststep Library Guidance
## Comprehensive API Reference and Usage Patterns

**Last Updated:** January 29, 2025  
**Library Version:** 0.4.0 (with `ap203` feature)  
**Maintained By:** System Architect (Alex Chen)  
**Purpose:** Practical guidance for using ruststep in STEP file conversion

---

## Quick Reference

**Official Resources:**
- **Documentation:** https://docs.rs/ruststep/latest/ruststep/
- **GitHub Repository:** https://github.com/ricosjp/ruststep
- **License:** Apache-2.0 (fully compatible)
- **Author:** ricosjp (same author as truck library)

**Current Project Usage:**
- **Version:** `0.4.0` with `ap203` feature
- **Location:** `mesh-core/Cargo.toml`
- **Feature Flag:** `step` (enables ruststep and truck crates)

---

## Table of Contents

1. [Library Overview](#library-overview)
2. [Core Concepts](#core-concepts)
3. [API Reference](#api-reference)
4. [Common Patterns](#common-patterns)
5. [Entity Access Patterns](#entity-access-patterns)
6. [Reference Resolution](#reference-resolution)
7. [Error Handling](#error-handling)
8. [Best Practices](#best-practices)
9. [Known Limitations](#known-limitations)
10. [Examples](#examples)

---

## Library Overview

### What is ruststep?

ruststep is a Rust library for reading and writing STEP (ISO 10303-21) files. It provides:
- STEP file parsing into an abstract syntax tree (AST)
- AP203 schema support for 3D CAD models
- Entity deserialization into Rust structs
- Reference resolution via Tables structure
- Exchange structure graph representation

### Key Features

1. **Schema Support:**
   - ISO 10303-201 (Explicit Draughting) - `ap201` feature
   - ISO 10303-203 (Configuration Controlled Design) - `ap203` feature ✅ **We use this**
   - Additional schemas can be enabled via features

2. **Exchange Structure Handling:**
   - Represents STEP file as a graph (records = nodes, references = edges)
   - Manages entity relationships and dependencies
   - Provides Tables structure for entity lookup

3. **Parser Module:**
   - Tokenizes STEP file text into AST
   - Parses HEADER and DATA sections
   - Handles entity instances (Simple and Complex)

4. **Integration with espr:**
   - Uses espr compiler to generate Rust code from EXPRESS schemas
   - AP203 entities are generated Rust structs
   - Type-safe entity access

---

## Core Concepts

### 1. Exchange Structure

The `Exchange` struct represents a parsed STEP file:

```rust
use ruststep::ast::Exchange;

let exchange = parser::parse(step_text)?;

// Exchange contains:
// - header: Header section data
// - data: Vec<DataSection> - All entity instances
```

### 2. Data Sections

Each `DataSection` contains entity instances:

```rust
for data_section in &exchange.data {
    for entity_instance in &data_section.entities {
        match entity_instance {
            ast::EntityInstance::Simple { id, record } => {
                // id: EntityId (u64) - e.g., #1, #2
                // record.name: String - Entity type name
                // record.parameters: Vec<Parameter> - Entity parameters
            }
            ast::EntityInstance::Complex { id, subsuper } => {
                // Multiple records for subtype/supertype relationships
            }
        }
    }
}
```

### 3. Tables Structure (AP203)

The `Tables` structure provides deserialized AP203 entities:

```rust
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::TableInit;

// Build Tables from Exchange.data
let tables = Tables::from_data_sections(&exchange.data)?;
```

**Key Methods:**
- `Tables::from_data_sections()` - Populate tables from parsed data
- `tables.[entity_name]_holders()` - Get entity holders by type
- `holder.into_owned(tables)` - Resolve references to owned entity

### 4. Entity Holders

Entities are stored as "holders" that may contain unresolved references:

```rust
// Get holders for a specific entity type
let msb_holders = tables.manifold_solid_brep_holders();

// Each holder may contain references that need resolution
for (entity_id, holder) in msb_holders.iter() {
    // Resolve references to get owned entity
    let msb = holder.clone().into_owned(tables)?;
    // Now msb is fully resolved with all references resolved
}
```

---

## API Reference

### Parser Module

#### `parser::parse(text: &str) -> Result<Exchange>`

Parse a STEP file string into an Exchange structure.

```rust
use ruststep::parser;

let step_text = std::fs::read_to_string("model.step")?;
let exchange = parser::parse(&step_text)?;
```

**Returns:** `Exchange` containing header and data sections

**Errors:** Returns parse error if STEP file is malformed

### Tables Module

#### `TableInit::from_data_sections(data: &[DataSection]) -> Result<Tables>`

Populate AP203 Tables from parsed Exchange data sections.

```rust
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::TableInit;

let tables = Tables::from_data_sections(&exchange.data)?;
```

**This is the CORRECT method** - discovered by Riley during implementation.

**Returns:** `Tables` with all AP203 entities populated

**Errors:** Returns error if entities don't match AP203 schema

### Entity Access Methods

#### `tables.[entity_name]_holders() -> &HashMap<EntityId, EntityHolder>`

Get all entities of a specific type from Tables.

**Available Methods (AP203):**
- `tables.manifold_solid_brep_holders()` - MANIFOLD_SOLID_BREP entities
- `tables.closed_shell_holders()` - CLOSED_SHELL entities
- `tables.faceted_brep_holders()` - FACETED_BREP entities
- `tables.cartesian_point_holders()` - CARTESIAN_POINT entities
- `tables.face_holders()` - FACE entities
- `tables.edge_holders()` - EDGE entities
- `tables.vertex_point_holders()` - VERTEX_POINT entities
- And many more...

**Pattern:**
```rust
let holders = tables.[entity_name]_holders();
for (id, holder) in holders.iter() {
    // Process each entity
}
```

### Reference Resolution

#### `IntoOwned::into_owned(holder, tables) -> Result<OwnedEntity>`

Resolve all references in an entity holder to get a fully resolved owned entity.

```rust
use ruststep::tables::IntoOwned;

let holder = tables.manifold_solid_brep_holders().get(&entity_id)?;
let msb = holder.clone().into_owned(tables)?;
// msb is now ManifoldSolidBrep with all references resolved
```

**Key Points:**
- Clone the holder before calling `into_owned()` (consumes the holder)
- Returns error if references cannot be resolved
- Resolved entity has all nested references fully resolved

---

## Common Patterns

### Pattern 1: Basic STEP File Parsing

```rust
use ruststep::parser;

fn parse_step_file(path: &Path) -> Result<Exchange> {
    let step_text = std::fs::read_to_string(path)?;
    let exchange = parser::parse(&step_text)?;
    Ok(exchange)
}
```

### Pattern 2: Build Tables from Exchange

```rust
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::TableInit;

fn build_tables(exchange: &ast::Exchange) -> Result<Tables> {
    let tables = Tables::from_data_sections(&exchange.data)?;
    Ok(tables)
}
```

**Error Handling:**
```rust
let tables = match Tables::from_data_sections(&exchange.data) {
    Ok(t) => t,
    Err(e) => {
        eprintln!("Warning: Could not fully deserialize STEP entities: {:?}", e);
        eprintln!("Falling back to default tables (limited functionality)");
        Tables::default()
    }
};
```

### Pattern 3: Access Specific Entity Types

```rust
use ruststep::ap203::config_control_design::Tables;

fn extract_manifold_solids(tables: &Tables) -> Vec<EntityId> {
    let msb_holders = tables.manifold_solid_brep_holders();
    msb_holders.keys().copied().collect()
}
```

### Pattern 4: Resolve Entity References

```rust
use ruststep::tables::IntoOwned;

fn resolve_manifold_solid_brep(
    entity_id: EntityId,
    tables: &Tables,
) -> Result<ManifoldSolidBrep> {
    let holder = tables.manifold_solid_brep_holders()
        .get(&entity_id)
        .ok_or_else(|| ConversionError::MissingEntity(entity_id))?;
    
    let msb = holder.clone().into_owned(tables)?;
    Ok(msb)
}
```

### Pattern 5: Traverse Entity Hierarchy

```rust
use ruststep::tables::IntoOwned;

fn extract_shell_from_manifold_solid_brep(
    msb_id: EntityId,
    tables: &Tables,
) -> Result<ClosedShell> {
    // 1. Get MANIFOLD_SOLID_BREP holder
    let msb_holder = tables.manifold_solid_brep_holders()
        .get(&msb_id)
        .ok_or_else(|| ConversionError::MissingEntity(msb_id))?;
    
    // 2. Resolve to owned entity
    let msb = msb_holder.clone().into_owned(tables)?;
    
    // 3. Get closed_shell reference
    let shell_ref = msb.closed_shell(); // Returns a reference type
    
    // 4. Resolve shell reference
    let shell_holder = tables.closed_shell_holders()
        .get(&shell_ref.entity_id())
        .ok_or_else(|| ConversionError::MissingReference(shell_ref.entity_id()))?;
    
    // 5. Resolve shell to owned entity
    let shell = shell_holder.clone().into_owned(tables)?;
    
    Ok(shell)
}
```

---

## Entity Access Patterns

### Available AP203 Entity Types

Common geometric entities available in `Tables`:

| Entity Type | Method | Purpose |
|------------|--------|---------|
| `MANIFOLD_SOLID_BREP` | `manifold_solid_brep_holders()` | Solid with boundary representation |
| `CLOSED_SHELL` | `closed_shell_holders()` | Closed shell of faces |
| `FACETED_BREP` | `faceted_brep_holders()` | Pre-tessellated BREP (triangulated) |
| `FACE` | `face_holders()` | Individual face |
| `FACE_BOUND` | `face_bound_holders()` | Face boundary (edge loop) |
| `EDGE_LOOP` | `edge_loop_holders()` | Loop of edges |
| `ORIENTED_EDGE` | `oriented_edge_holders()` | Oriented edge |
| `EDGE` | `edge_holders()` | Edge connecting vertices |
| `VERTEX_POINT` | `vertex_point_holders()` | Vertex with point geometry |
| `CARTESIAN_POINT` | `cartesian_point_holders()` | 3D point (x, y, z) |
| `DIRECTION` | `direction_holders()` | Direction vector |
| `VECTOR` | `vector_holders()` | Vector with magnitude |

### Entity Type Detection

```rust
fn identify_entity_type(record: &ast::Record) -> Option<&str> {
    match record.name.as_str() {
        "MANIFOLD_SOLID_BREP" => Some("MANIFOLD_SOLID_BREP"),
        "CLOSED_SHELL" => Some("CLOSED_SHELL"),
        "FACETED_BREP" => Some("FACETED_BREP"),
        "ADVANCED_BREP_SHAPE_REPRESENTATION" => Some("ADVANCED_BREP_SHAPE_REPRESENTATION"),
        _ => None,
    }
}
```

---

## Reference Resolution

### Understanding References

STEP files use entity references (e.g., `#1`, `#2`) to link entities:

```
#1 = MANIFOLD_SOLID_BREP('solid', #2);
#2 = CLOSED_SHELL('shell', (#3, #4));
```

Here, `#1` references `#2`, which references `#3` and `#4`.

### Resolution Process

1. **Get Entity Holder:**
   ```rust
   let holder = tables.manifold_solid_brep_holders().get(&entity_id)?;
   ```

2. **Resolve to Owned Entity:**
   ```rust
   use ruststep::tables::IntoOwned;
   let entity = holder.clone().into_owned(tables)?;
   ```

3. **Access Referenced Entities:**
   ```rust
   let shell_ref = entity.closed_shell(); // Get reference
   let shell_id = shell_ref.entity_id(); // Extract ID
   let shell_holder = tables.closed_shell_holders().get(&shell_id)?;
   let shell = shell_holder.clone().into_owned(tables)?;
   ```

### Reference Types

References in ruststep are represented as types that implement:
- `entity_id()` - Get the referenced entity ID
- `into_owned()` - Resolve to owned entity (requires Tables)

---

## Error Handling

### Common Error Scenarios

1. **Parse Errors:**
   ```rust
   match parser::parse(step_text) {
       Ok(exchange) => { /* success */ }
       Err(e) => {
           return Err(ConversionError::ConversionFailed(
               format!("Failed to parse STEP file: {}", e)
           ));
       }
   }
   ```

2. **Tables Construction Errors:**
   ```rust
   match Tables::from_data_sections(&exchange.data) {
       Ok(tables) => { /* success */ }
       Err(e) => {
           // May happen if entities don't match AP203 schema
           eprintln!("Warning: Could not fully deserialize: {:?}", e);
           Tables::default() // Fallback
       }
   }
   ```

3. **Missing Entity Errors:**
   ```rust
   let holder = tables.manifold_solid_brep_holders()
       .get(&entity_id)
       .ok_or_else(|| ConversionError::ConversionFailed(
           format!("Entity #{} not found", entity_id)
       ))?;
   ```

4. **Reference Resolution Errors:**
   ```rust
   match holder.clone().into_owned(tables) {
       Ok(entity) => { /* success */ }
       Err(e) => {
           return Err(ConversionError::ConversionFailed(
               format!("Failed to resolve references: {:?}", e)
           ));
       }
   }
   ```

### Error Recovery Strategies

1. **Graceful Degradation:**
   ```rust
   let tables = match Tables::from_data_sections(&exchange.data) {
       Ok(t) => t,
       Err(_) => {
           // Continue with limited functionality
           Tables::default()
       }
   };
   ```

2. **Skip Invalid Entities:**
   ```rust
   for (id, holder) in tables.manifold_solid_brep_holders().iter() {
       match holder.clone().into_owned(tables) {
           Ok(msb) => { /* process */ }
           Err(e) => {
               eprintln!("Skipping entity #{}: {:?}", id, e);
               continue;
           }
       }
   }
   ```

---

## Best Practices

### 1. Always Use `TableInit::from_data_sections()`

**✅ CORRECT:**
```rust
let tables = Tables::from_data_sections(&exchange.data)?;
```

**❌ INCORRECT (Manual Population):**
```rust
// Don't manually populate tables - use the provided method
let mut tables = Tables::default();
// Manual insertion is error-prone and unnecessary
```

### 2. Use Holder Methods for Entity Access

**✅ CORRECT:**
```rust
let msb_holders = tables.manifold_solid_brep_holders();
for (id, holder) in msb_holders.iter() {
    // Process holder
}
```

**❌ INCORRECT (Direct Field Access):**
```rust
// Don't access tables.manifold_solid_brep directly
// Use the getter method instead
```

### 3. Clone Before `into_owned()`

**✅ CORRECT:**
```rust
let entity = holder.clone().into_owned(tables)?;
```

**❌ INCORRECT:**
```rust
// into_owned() consumes the holder
let entity = holder.into_owned(tables)?; // holder is moved
// Can't use holder again
```

### 4. Handle Tables Construction Errors Gracefully

```rust
let tables = match Tables::from_data_sections(&exchange.data) {
    Ok(t) => t,
    Err(e) => {
        // Log warning but continue
        eprintln!("Warning: Partial deserialization: {:?}", e);
        Tables::default()
    }
};
```

### 5. Validate Entity Counts

```rust
let msb_count = tables.manifold_solid_brep_holders().len();
if msb_count == 0 {
    return Err(ConversionError::ConversionFailed(
        "No MANIFOLD_SOLID_BREP entities found".to_string()
    ));
}
```

### 6. Use Entity ID for Logging

```rust
for (entity_id, holder) in tables.manifold_solid_brep_holders().iter() {
    eprintln!("Processing entity #{}", entity_id);
    // Process entity
}
```

---

## Known Limitations

### 1. AP203 Schema Only

**Current Support:**
- ✅ AP203 (Configuration Controlled Design) - **We use this**
- ❌ AP201 (Explicit Draughting) - Not enabled
- ❌ AP214 (Automotive Design) - Not enabled
- ❌ AP242 (Managed Model-Based 3D Engineering) - Not enabled

**Impact:** Only STEP files with AP203 schema are fully supported.

### 2. Tables Construction May Fail

**Scenario:** If STEP file contains entities not in AP203 schema, `from_data_sections()` may fail.

**Mitigation:** Use fallback to `Tables::default()` and continue with limited functionality.

### 3. Reference Resolution May Fail

**Scenario:** Circular references or missing entities cause `into_owned()` to fail.

**Mitigation:** Handle errors gracefully, skip invalid entities, log warnings.

### 4. Complex Entity Types

**Scenario:** Some complex entity types may not be fully supported.

**Mitigation:** Focus on common types (MANIFOLD_SOLID_BREP, CLOSED_SHELL, FACETED_BREP).

---

## Examples

### Complete Example: Extract FACETED_BREP Entities

```rust
use ruststep::parser;
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::{TableInit, IntoOwned};

fn extract_faceted_brep_entities(step_text: &str) -> Result<Vec<FacetedBrep>> {
    // 1. Parse STEP file
    let exchange = parser::parse(step_text)?;
    
    // 2. Build Tables
    let tables = Tables::from_data_sections(&exchange.data)?;
    
    // 3. Get FACETED_BREP entities
    let fb_holders = tables.faceted_brep_holders();
    
    // 4. Resolve to owned entities
    let mut entities = Vec::new();
    for (id, holder) in fb_holders.iter() {
        match holder.clone().into_owned(&tables) {
            Ok(fb) => entities.push(fb),
            Err(e) => {
                eprintln!("Warning: Failed to resolve FACETED_BREP #{}: {:?}", id, e);
            }
        }
    }
    
    Ok(entities)
}
```

### Example: Traverse Entity Hierarchy

```rust
fn extract_shell_faces(
    shell_id: EntityId,
    tables: &Tables,
) -> Result<Vec<Face>> {
    // 1. Get CLOSED_SHELL
    let shell_holder = tables.closed_shell_holders()
        .get(&shell_id)
        .ok_or_else(|| ConversionError::MissingEntity(shell_id))?;
    
    let shell = shell_holder.clone().into_owned(tables)?;
    
    // 2. Get face references
    let face_refs = shell.cfs_faces(); // Returns list of references
    
    // 3. Resolve each face
    let mut faces = Vec::new();
    for face_ref in face_refs {
        let face_id = face_ref.entity_id();
        let face_holder = tables.face_holders()
            .get(&face_id)
            .ok_or_else(|| ConversionError::MissingReference(face_id))?;
        
        let face = face_holder.clone().into_owned(tables)?;
        faces.push(face);
    }
    
    Ok(faces)
}
```

### Example: Extract Cartesian Points

```rust
fn extract_cartesian_points(tables: &Tables) -> Vec<(EntityId, (f64, f64, f64))> {
    let cp_holders = tables.cartesian_point_holders();
    let mut points = Vec::new();
    
    for (id, holder) in cp_holders.iter() {
        match holder.clone().into_owned(tables) {
            Ok(cp) => {
                // Access coordinates (structure depends on AP203 definition)
                // This is a simplified example - actual field names may vary
                let coords = cp.coordinates(); // Hypothetical method
                points.push((*id, coords));
            }
            Err(e) => {
                eprintln!("Warning: Failed to resolve CARTESIAN_POINT #{}: {:?}", id, e);
            }
        }
    }
    
    points
}
```

---

## Integration with Project

### Current Implementation

**File:** `mesh-core/src/formats/step.rs`

**Current Usage:**
```rust
use ruststep::parser;
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::{TableInit, IntoOwned};

// Parse STEP file
let exchange = parser::parse(step_text)?;

// Build Tables
let tables = Tables::from_data_sections(&exchange.data)?;

// Access entities
let msb_holders = tables.manifold_solid_brep_holders();
let cs_holders = tables.closed_shell_holders();

// Resolve references
for (id, holder) in msb_holders.iter() {
    match holder.clone().into_owned(&tables) {
        Ok(msb) => { /* process */ }
        Err(e) => { /* handle error */ }
    }
}
```

### Feature Flag

**Cargo.toml:**
```toml
[features]
step = ["ruststep", "truck-modeling", "truck-polymesh", "truck-stepio", "truck-meshalgo"]

[dependencies]
ruststep = { version = "0.4", optional = true, features = ["ap203"] }
```

**Usage:**
```bash
# Build with STEP support
cargo build --features step

# Build without STEP support
cargo build
```

---

## Resources

### Official Documentation
- **docs.rs:** https://docs.rs/ruststep/latest/ruststep/
- **GitHub:** https://github.com/ricosjp/ruststep
- **AP203 Module:** https://docs.rs/ruststep/latest/ruststep/ap203/

### Related Documentation
- **espr (EXPRESS compiler):** https://docs.rs/espr/
- **truck library:** https://docs.rs/truck-modeling/ (same author)
- **STEP Specification:** ISO 10303-203

### Project References
- `mesh-core/src/formats/step.rs` - Current implementation
- `mesh-core/examples/verify_ruststep_tables.rs` - Verification example
- `RESEARCH_RUSTSTEP_EXAMPLES.md` - Research findings
- `docs/STEP_FORMAT_REFERENCE.md` - STEP format specification

---

## Update Log

| Date | Update | Updated By |
|------|--------|------------|
| 2025-01-29 | Initial comprehensive guidance document created | System Architect |

---

**Status:** ✅ **ACTIVE** - Comprehensive guidance for engineering team  
**Next Review:** As needed when ruststep API changes or new patterns discovered

---

*This document provides practical, actionable guidance based on official ruststep documentation and verified implementation patterns from the project.*

