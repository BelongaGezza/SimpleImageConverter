# Research: ruststep Examples and Patterns
## Task 1.1 - Sam Parker (Junior Engineer, 2D Formats)

**Date:** January 27, 2025  
**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Purpose:** Document ruststep usage examples, patterns, and API patterns for STEP entity conversion

---

## Executive Summary

This document compiles research findings on ruststep library usage patterns, focusing on:
- Building AP203 Tables from Exchange.data
- Deserializing STEP Records into AP203 structs
- Resolving entity references
- Common patterns for STEP entity handling

**Target Audience:** Riley (3D formats engineer) and Senior Engineer for implementation guidance

---

## ruststep Library Overview

### Basic Information
- **Crate:** `ruststep = "0.4.0"`
- **License:** Apache-2.0 (fully compatible)
- **Author:** ricosjp (same as truck library)
- **Repository:** https://github.com/ricosjp/ruststep
- **Documentation:** https://docs.rs/ruststep/
- **Features:** Requires `ap203` feature for AP203 support

### Current Usage in Project

From `mesh-core/src/formats/step.rs`:

```rust
use ruststep::{ast, parser};

// Parse STEP file
let exchange = parser::parse(step_text)?;

// Access data sections
for data_section in &exchange.data {
    for entity_instance in &data_section.entities {
        match entity_instance {
            ast::EntityInstance::Simple { id: _, record } => {
                // record.name - entity type name
                // record.parameters - entity parameters
            }
            ast::EntityInstance::Complex { id: _, subsuper } => {
                // Multiple records for subtype/supertype
            }
        }
    }
}
```

---

## Key Research Questions

### 1. How to Build AP203 Tables from Exchange.data?

**Current Understanding:**
- `Exchange` contains `data: Vec<DataSection>`
- Each `DataSection` contains `entities: Vec<EntityInstance>`
- Need to build `Tables` structure for AP203 deserialization

**Research Needed:**
- [ ] How to construct `ruststep::ap203::config_control_design::Tables`
- [ ] How to populate tables from `Exchange.data`
- [ ] Table structure and organization

**Hypothesis:**
```rust
use ruststep::ap203::config_control_design::Tables;

// Build tables from exchange
let mut tables = Tables::default();
// ... populate from exchange.data ...
```

### 2. How to Deserialize Records into AP203 Structs?

**Current Understanding:**
- `Record` has `name: String` and `parameters`
- AP203 provides structs like `ManifoldSolidBrep`, `ClosedShell`, etc.
- Need serde deserialization

**Research Needed:**
- [ ] How to deserialize `Record` into AP203 structs
- [ ] What AP203 types are available
- [ ] Deserialization patterns and error handling

**Hypothesis:**
```rust
use ruststep::ap203::config_control_design::ManifoldSolidBrep;
use serde::Deserialize;

// Deserialize record into AP203 struct
let msb: ManifoldSolidBrep = ManifoldSolidBrep::deserialize(&record)?;
```

### 3. How to Resolve Entity References?

**Current Understanding:**
- STEP uses references like `#1`, `#2` in parameters
- References point to other entities
- Need to resolve using Tables

**Research Needed:**
- [ ] How references are represented in ruststep
- [ ] How to resolve references using Tables
- [ ] Reference resolution patterns

**Hypothesis:**
```rust
// Resolve reference
let shell_ref = msb.closed_shell(); // Get reference
let closed_shell = tables.closed_shell.get(&shell_ref.id)?;
```

---

## Code Examples and Patterns

### Pattern 1: Basic STEP File Parsing

```rust
use ruststep::parser;

// Parse STEP file
let step_text = std::fs::read_to_string("model.step")?;
let exchange = parser::parse(&step_text)?;

// Access header
println!("Header: {:?}", exchange.header);

// Access data sections
for data_section in &exchange.data {
    println!("Data section with {} entities", data_section.entities.len());
}
```

### Pattern 2: Entity Type Identification

```rust
use ruststep::ast;

for data_section in &exchange.data {
    for entity_instance in &data_section.entities {
        match entity_instance {
            ast::EntityInstance::Simple { id, record } => {
                println!("Entity #{}: {}", id, record.name);
                
                // Match on entity type
                match record.name.as_str() {
                    "MANIFOLD_SOLID_BREP" => {
                        // Handle solid
                    }
                    "CLOSED_SHELL" => {
                        // Handle shell
                    }
                    _ => {
                        // Unknown entity
                    }
                }
            }
            ast::EntityInstance::Complex { id, subsuper } => {
                println!("Complex entity #{} with {} subtypes", id, subsuper.len());
            }
        }
    }
}
```

### Pattern 3: Building Tables (Hypothetical - Needs Verification)

```rust
use ruststep::ap203::config_control_design::Tables;

// Build tables from exchange
let mut tables = Tables::default();

// Populate tables from exchange.data
for data_section in &exchange.data {
    for entity_instance in &data_section.entities {
        match entity_instance {
            ast::EntityInstance::Simple { id, record } => {
                // Add to appropriate table based on entity type
                // This pattern needs verification from ruststep docs/examples
            }
            _ => {}
        }
    }
}
```

### Pattern 4: Deserializing AP203 Entities (Hypothetical - Needs Verification)

```rust
use ruststep::ap203::config_control_design::{ManifoldSolidBrep, ClosedShell};
use serde::Deserialize;

// Deserialize MANIFOLD_SOLID_BREP
if record.name == "MANIFOLD_SOLID_BREP" {
    // This pattern needs verification
    let msb: ManifoldSolidBrep = ManifoldSolidBrep::deserialize(record)?;
    
    // Access fields
    let shell_ref = msb.closed_shell();
    // Resolve reference using tables
}
```

### Pattern 5: Reference Resolution (Hypothetical - Needs Verification)

```rust
// Resolve closed_shell reference from MANIFOLD_SOLID_BREP
let shell_ref = msb.closed_shell(); // Returns some reference type
let closed_shell_id = shell_ref.id(); // Get entity ID

// Look up in tables
let closed_shell = tables.closed_shell.get(&closed_shell_id)
    .ok_or_else(|| ConversionError::MissingReference(closed_shell_id))?;

// Now we have the ClosedShell entity
```

---

## GitHub Repository Research

### Repository Structure
- **URL:** https://github.com/ricosjp/ruststep
- **Language:** Rust
- **License:** Apache-2.0

### Key Files to Review
1. **Examples directory** - Look for usage examples
2. **Tests directory** - Test files show API usage patterns
3. **Documentation** - README and inline docs
4. **AP203 module** - AP203-specific code

### Search Strategy
1. Search for "Tables" usage in examples/tests
2. Search for "deserialize" patterns
3. Search for "AP203" usage examples
4. Look for reference resolution patterns

---

## AP203 Entity Types

### Common Geometric Entities

Based on STEP specification and ruststep structure:

1. **MANIFOLD_SOLID_BREP**
   - Represents a solid with boundary representation
   - Parameters: [name, closed_shell_ref]
   - Contains reference to CLOSED_SHELL

2. **CLOSED_SHELL**
   - Represents a closed shell
   - Parameters: [name, face_list]
   - Contains list of face references

3. **FACE**
   - Represents a face
   - Parameters: [face_geometry, face_bound]
   - Has surface and boundary information

4. **ADVANCED_BREP_SHAPE_REPRESENTATION**
   - Advanced BREP representation
   - Contains items (shells/solids)

5. **FACETED_BREP**
   - Triangulated BREP
   - May be easier to convert (already triangulated)

### Entity Hierarchy

```
MANIFOLD_SOLID_BREP
  └─ closed_shell: CLOSED_SHELL
      └─ faces: List[FACE]
          └─ face_geometry: SURFACE
          └─ face_bound: FACE_BOUND
              └─ edges: List[EDGE]
                  └─ vertices: List[VERTEX]
```

---

## Common Patterns for AP203 Tables Construction

### Pattern: Building Tables from Exchange

**Hypothesis (needs verification):**

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
                    // Add to appropriate table based on entity type
                    match record.name.as_str() {
                        "MANIFOLD_SOLID_BREP" => {
                            // Add to manifold_solid_brep table
                            // tables.manifold_solid_brep.insert(*id, ...);
                        }
                        "CLOSED_SHELL" => {
                            // Add to closed_shell table
                            // tables.closed_shell.insert(*id, ...);
                        }
                        // ... other entity types
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    
    Ok(tables)
}
```

**Note:** This is a hypothesis. Actual API needs to be verified from ruststep documentation or source code.

---

## Deserialization Patterns

### Pattern: Deserializing with Tables Context

**Hypothesis:**

```rust
use ruststep::ap203::config_control_design::ManifoldSolidBrep;
use serde::Deserialize;

fn deserialize_entity(record: &ast::Record, tables: &Tables) -> Result<ManifoldSolidBrep> {
    // Deserialize record into AP203 struct
    // This may require tables for reference resolution
    let msb = ManifoldSolidBrep::deserialize_with_tables(record, tables)?;
    Ok(msb)
}
```

**Alternative Pattern (if serde Deserialize works directly):**

```rust
use ruststep::ap203::config_control_design::ManifoldSolidBrep;
use serde::Deserialize;

// If Record implements Deserialize or can be converted
let msb: ManifoldSolidBrep = serde::Deserialize::deserialize(record)?;
```

---

## Reference Resolution Patterns

### Pattern: Resolving Entity References

**Hypothesis:**

```rust
// Step 1: Get reference from entity
let shell_ref = msb.closed_shell(); // Returns some reference type

// Step 2: Extract entity ID
let shell_id = shell_ref.entity_id(); // e.g., 2 for #2

// Step 3: Look up in tables
let closed_shell = tables.closed_shell.get(&shell_id)
    .ok_or_else(|| ConversionError::MissingReference(format!("#{}", shell_id)))?;

// Step 4: Now we have the actual ClosedShell entity
```

### Pattern: Resolving List References

**For CLOSED_SHELL with face list:**

```rust
let closed_shell: ClosedShell = /* deserialized */;
let face_refs = closed_shell.faces(); // Returns list of references

let mut faces = Vec::new();
for face_ref in face_refs {
    let face_id = face_ref.entity_id();
    let face = tables.face.get(&face_id)
        .ok_or_else(|| ConversionError::MissingReference(format!("#{}", face_id)))?;
    faces.push(face);
}
```

---

## Error Handling Patterns

### Pattern: Handling Deserialization Errors

```rust
use ruststep::ap203::config_control_design::ManifoldSolidBrep;

fn try_deserialize_manifold_solid_brep(
    record: &ast::Record,
    tables: &Tables,
) -> Result<Option<ManifoldSolidBrep>> {
    if record.name != "MANIFOLD_SOLID_BREP" {
        return Ok(None);
    }
    
    match ManifoldSolidBrep::deserialize(record) {
        Ok(msb) => Ok(Some(msb)),
        Err(e) => {
            // Log error but don't fail entire conversion
            eprintln!("Failed to deserialize MANIFOLD_SOLID_BREP: {}", e);
            Ok(None)
        }
    }
}
```

### Pattern: Handling Missing References

```rust
fn resolve_reference<T>(
    ref_id: EntityId,
    table: &HashMap<EntityId, T>,
) -> Result<&T> {
    table.get(&ref_id)
        .ok_or_else(|| ConversionError::MissingReference(
            format!("Entity #{} not found in tables", ref_id)
        ))
}
```

---

## Testing Patterns

### Pattern: Testing with Simple STEP Files

```rust
#[test]
fn test_parse_simple_step() {
    let step_text = r#"
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Simple test'),'2;1');
FILE_NAME('test.step','2025-01-27T00:00:00',(''),(''),'','','');
ENDSEC;
DATA;
#1 = MANIFOLD_SOLID_BREP('solid', #2);
#2 = CLOSED_SHELL('shell', (#3));
#3 = FACE('face', #4, #5);
ENDSEC;
END-ISO-10303-21;
"#;
    
    let exchange = parser::parse(step_text).unwrap();
    assert_eq!(exchange.data.len(), 1);
    // ... test entity extraction
}
```

---

## Next Steps for Verification

### Immediate Actions

1. **Check ruststep Documentation:**
   - Review https://docs.rs/ruststep/ for Tables API
   - Look for AP203 examples
   - Check deserialization patterns

2. **Examine ruststep Source Code:**
   - Review GitHub repository
   - Check examples directory
   - Review test files for usage patterns

3. **Create Experimental Code:**
   - Build minimal test program
   - Try building Tables
   - Try deserializing entities
   - Document what works

4. **Share Findings:**
   - Update this document with verified patterns
   - Share with Riley and Senior Engineer
   - Create working code snippets

---

## Resources and References

### Official Documentation
- ruststep docs.rs: https://docs.rs/ruststep/
- ruststep GitHub: https://github.com/ricosjp/ruststep

### Related Documentation
- STEP Format Specification: ISO 10303
- AP203 Application Protocol documentation
- truck library documentation (same author)

### Project References
- `STEP_IMPLEMENTATION_CURRENT_STATE.md` - Current implementation status
- `mesh-core/src/formats/step.rs` - Current code
- `V0.2.0_STEP_READING_RESEARCH.md` - Previous research

---

## Findings Summary

### ✅ Confirmed
- ruststep can parse STEP files successfully
- Exchange structure is accessible
- Entity instances can be extracted
- Entity types can be identified by name

### ⚠️ Needs Verification
- Tables construction API
- AP203 deserialization patterns
- Reference resolution mechanisms
- Actual AP203 struct types available

### ❓ Unknown
- Exact Tables API structure
- How to populate tables from Exchange
- Deserialization error handling patterns
- Reference type representation

---

## Code Snippets for Reference

### Snippet 1: Current Implementation Pattern

```rust
// From mesh-core/src/formats/step.rs
fn try_extract_shell(&self, record: &ast::Record) -> Result<Option<Shell>> {
    match record.name.as_str() {
        "MANIFOLD_SOLID_BREP" => {
            // TODO: Extract closed_shell reference and convert to Shell
            Ok(None)
        }
        "CLOSED_SHELL" => {
            // TODO: Extract faces and convert to truck Shell
            Ok(None)
        }
        _ => Ok(None)
    }
}
```

### Snippet 2: Desired Pattern (Hypothetical)

```rust
fn try_extract_shell(&self, record: &ast::Record, tables: &Tables) -> Result<Option<Shell>> {
    match record.name.as_str() {
        "MANIFOLD_SOLID_BREP" => {
            // Deserialize
            let msb: ManifoldSolidBrep = ManifoldSolidBrep::deserialize(record)?;
            
            // Resolve reference
            let shell_ref = msb.closed_shell();
            let closed_shell = tables.closed_shell.get(&shell_ref.id())?;
            
            // Convert to truck Shell
            let shell = self.convert_closed_shell_to_truck(closed_shell, tables)?;
            Ok(Some(shell))
        }
        "CLOSED_SHELL" => {
            let cs: ClosedShell = ClosedShell::deserialize(record)?;
            let shell = self.convert_closed_shell_to_truck(&cs, tables)?;
            Ok(Some(shell))
        }
        _ => Ok(None)
    }
}
```

---

## Questions for Further Research

1. **Tables API:**
   - What is the exact structure of `Tables`?
   - How do we populate tables from `Exchange.data`?
   - Are tables automatically built or manually constructed?

2. **Deserialization:**
   - Does ruststep provide deserialization helpers?
   - Do we need to manually parse Record parameters?
   - How do we handle serde deserialization with Tables context?

3. **Reference Resolution:**
   - How are references represented in ruststep?
   - Do references include entity IDs directly?
   - How do we handle circular references?

4. **AP203 Types:**
   - What AP203 struct types are available?
   - What fields do they have?
   - How do we access nested entities?

---

## Updates Log

| Date | Update | Status |
|------|--------|--------|
| 2025-01-27 | Initial research document created | In Progress |
| | | |

---

**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Next Update:** After reviewing ruststep documentation and source code  
**Target:** Provide verified code examples and patterns for Riley

---

*Researcher: Sam Parker (Junior Engineer, 2D Formats)*  
*For: Riley Thompson (Junior Engineer, 3D Formats) & Senior Engineer*

