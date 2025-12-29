# ruststep Tables API Research Findings
## For Riley - Tables Population Methods

**Researcher:** Sam Parker (Junior Engineer, 2D Formats)  
**Date:** January 27, 2025  
**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Priority:** 🔥 **URGENT** - Blocking Riley's Implementation

---

## Executive Summary

Riley needs to populate `Tables` from `Exchange.data` to enable entity deserialization. This document compiles research findings on how to do this.

---

## Current Understanding

### What We Know

1. ✅ `Tables::default()` creates empty Tables
2. ✅ `Exchange.data` contains all entities
3. ✅ Entities are in `EntityInstance::Simple { id, record }` format
4. ❓ **UNKNOWN:** How to populate Tables from Exchange.data

### The Problem

```rust
let mut tables = Tables::default(); // Empty tables
// How do we populate tables from exchange.data?
```

---

## Research Approaches

### Approach 1: Manual Population

**Hypothesis:** Tables has HashMap-like structures for each entity type, and we manually insert entities.

**Pattern (Hypothetical):**
```rust
use ruststep::ap203::config_control_design::Tables;

let mut tables = Tables::default();

for data_section in &exchange.data {
    for entity_instance in &data_section.entities {
        match entity_instance {
            ast::EntityInstance::Simple { id, record } => {
                match record.name.as_str() {
                    "CARTESIAN_POINT" => {
                        // Deserialize and insert
                        let point = /* deserialize record */;
                        tables.cartesian_point.insert(*id, point);
                    }
                    "MANIFOLD_SOLID_BREP" => {
                        let msb = /* deserialize record */;
                        tables.manifold_solid_brep.insert(*id, msb);
                    }
                    // ... other entity types
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
```

**Status:** ⚠️ Needs verification - need to check if Tables has these fields

---

### Approach 2: Builder Pattern

**Hypothesis:** Tables has a builder or helper method to populate from Exchange.

**Pattern (Hypothetical):**
```rust
// Option A: Builder
let tables = Tables::builder()
    .from_exchange(&exchange)
    .build()?;

// Option B: Helper method
let tables = Tables::from_exchange(&exchange)?;

// Option C: Populate method
let mut tables = Tables::default();
tables.populate_from_exchange(&exchange)?;
```

**Status:** ❌ `from_exchange()` doesn't exist (we know this)

---

### Approach 3: Serde Deserialization with Context

**Hypothesis:** Records can be deserialized with Tables as context for reference resolution.

**Pattern (Hypothetical):**
```rust
use ruststep::ap203::config_control_design::{Tables, CartesianPoint};
use serde::Deserialize;

// Deserialize with Tables context
let point: CartesianPoint = CartesianPoint::deserialize_with_tables(record, &tables)?;
```

**Status:** ⚠️ Needs verification - need to check ruststep deserialization API

---

### Approach 4: Two-Pass Population

**Hypothesis:** First pass: collect all entities. Second pass: deserialize with references.

**Pattern (Hypothetical):**
```rust
// Pass 1: Build entity map
let mut entity_map = HashMap::new();
for entity in &exchange.data {
    // Store raw records
    entity_map.insert(id, record);
}

// Pass 2: Deserialize with reference resolution
let mut tables = Tables::default();
for (id, record) in entity_map {
    // Deserialize with ability to resolve references
    let entity = deserialize_with_references(record, &entity_map)?;
    tables.insert(id, entity);
}
```

**Status:** ⚠️ Needs verification

---

## Key Questions to Answer

1. **What fields does Tables have?**
   - Does it have `cartesian_point: HashMap<EntityId, CartesianPoint>`?
   - Does it have `manifold_solid_brep: HashMap<EntityId, ManifoldSolidBrep>`?
   - What's the actual structure?

2. **How to deserialize Records?**
   - Can we use `serde::Deserialize` directly?
   - Do we need Tables context for deserialization?
   - Are there helper methods?

3. **How to handle references?**
   - Are references resolved during deserialization?
   - Do we need to populate Tables first, then deserialize?
   - Or can we deserialize with lazy reference resolution?

---

## Next Steps for Verification

### Immediate Actions

1. **Check ruststep Source Code:**
   - Look at Tables struct definition
   - Find population methods
   - Check for examples

2. **Check ruststep Documentation:**
   - Review docs.rs for Tables API
   - Look for examples
   - Check for builder patterns

3. **Check ruststep GitHub:**
   - Review repository for examples
   - Check test files
   - Look for usage patterns

4. **Create Experimental Code:**
   - Try different approaches
   - Test what compiles
   - Document what works

---

## Resources

- ruststep docs.rs: https://docs.rs/ruststep/
- ruststep GitHub: https://github.com/ricosjp/ruststep
- Current code: `mesh-core/src/formats/step.rs`
- Verification code: `mesh-core/examples/verify_ruststep_tables.rs`

---

## Findings Log

| Date | Finding | Status |
|------|---------|--------|
| 2025-01-27 | `Tables::default()` works | ✅ Verified |
| 2025-01-27 | `Tables::from_exchange()` doesn't exist | ✅ Verified |
| 2025-01-27 | Need to research population method | 🔬 In Progress |

---

**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Next Update:** After checking ruststep source/documentation  
**Target:** Provide working Tables population pattern for Riley

---

*Researcher: Sam Parker (Junior Engineer, 2D Formats)*  
*For: Riley Thompson (Junior Engineer, 3D Formats)*

