# ruststep Tables API - Research Findings for Riley
## URGENT: Tables Population Methods

**Researcher:** Sam Parker (Junior Engineer, 2D Formats)  
**Date:** January 27, 2025  
**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Priority:** 🔥 **URGENT** - Blocking Your Implementation

---

## Quick Summary

I've been researching how to populate `Tables` from `Exchange.data`. Here's what I've found so far and what needs to be tested.

---

## ✅ Verified Facts

1. **Tables Creation Works:**
   ```rust
   use ruststep::ap203::config_control_design;
   let tables = config_control_design::Tables::default();
   ```
   ✅ This compiles and runs

2. **Exchange Structure is Accessible:**
   ```rust
   for data_section in &exchange.data {
       for entity_instance in &data_section.entities {
           match entity_instance {
               ast::EntityInstance::Simple { id, record } => {
                   // id: EntityId
                   // record.name: String (entity type)
                   // record.parameter: Vec<Parameter>
               }
           }
       }
   }
   ```
   ✅ This works - I've verified it

3. **Tables::from_exchange() Doesn't Exist:**
   ❌ This method doesn't exist (you already know this)

---

## 🔬 Research Approaches to Test

### Approach 1: Check Tables Structure

**What to Test:**
```rust
use ruststep::ap203::config_control_design::Tables;

let tables = Tables::default();

// Try to inspect Tables structure
// Does it have fields like:
// - tables.cartesian_point: HashMap<EntityId, CartesianPoint>?
// - tables.manifold_solid_brep: HashMap<EntityId, ManifoldSolidBrep>?

// If yes, we can manually populate:
for entity in &exchange.data {
    match entity {
        ast::EntityInstance::Simple { id, record } => {
            if record.name == "CARTESIAN_POINT" {
                // Deserialize and insert
                let point = /* deserialize record */;
                tables.cartesian_point.insert(*id, point);
            }
        }
    }
}
```

**Action:** Check ruststep source code or documentation for Tables struct definition

---

### Approach 2: Serde Deserialization

**What to Test:**
```rust
use ruststep::ap203::config_control_design::{Tables, CartesianPoint};
use serde::Deserialize;

// Try deserializing a Record directly
let record = /* get record from Exchange */;
let point: CartesianPoint = CartesianPoint::deserialize(record)?;

// Then insert into tables
tables.cartesian_point.insert(id, point);
```

**Action:** Test if AP203 types implement `Deserialize` and can deserialize from `Record`

---

### Approach 3: Tables as Deserialization Context

**What to Test:**
```rust
// Some serde deserializers support context
// Tables might be used as context for reference resolution
let point = CartesianPoint::deserialize_with_context(record, &tables)?;
```

**Action:** Check if ruststep provides custom deserializers that use Tables

---

## 📋 Immediate Next Steps

### For You (Riley):

1. **Check ruststep Source Code:**
   - Look at `ruststep::ap203::config_control_design::Tables` struct definition
   - See what fields it has
   - Check for any helper methods

2. **Check ruststep Documentation:**
   - Review https://docs.rs/ruststep/0.4/
   - Look for Tables API documentation
   - Check for examples

3. **Test Deserialization:**
   - Try deserializing a simple entity (CARTESIAN_POINT)
   - See if it works without Tables
   - Then try with Tables context

4. **Share Findings:**
   - Let me know what you discover
   - I'll update research documents
   - We can collaborate on solution

### For Me (Sam):

1. Continue researching ruststep GitHub repository
2. Look for examples in ruststep tests
3. Create experimental code to test different approaches
4. Update research documents as we learn

---

## 🔗 Resources

- **ruststep docs.rs:** https://docs.rs/ruststep/0.4/
- **ruststep GitHub:** https://github.com/ricosjp/ruststep
- **Verification Code:** `mesh-core/examples/verify_ruststep_tables.rs`
- **Research Document:** `TABLES_API_RESEARCH_FINDINGS.md`

---

## 💡 Hypothesis

Based on typical serde patterns, I suspect:

1. **Tables has HashMap fields** for each entity type
2. **AP203 types implement Deserialize** from Record
3. **We manually populate** by deserializing and inserting
4. **References are resolved** during deserialization using Tables context

But this needs verification!

---

## ⚠️ What We Need

**Critical:** We need to understand:
1. What fields does `Tables` have?
2. How to deserialize `Record` into AP203 types?
3. How to insert deserialized entities into Tables?

Once we know this, population should be straightforward.

---

## 📝 Code to Test

Here's experimental code you can try:

```rust
use ruststep::ap203::config_control_design::{Tables, CartesianPoint};
use ruststep::{ast, parser};
use serde::Deserialize;

// Parse STEP file
let exchange = parser::parse(step_text)?;

// Create empty Tables
let mut tables = Tables::default();

// Try to populate
for data_section in &exchange.data {
    for entity_instance in &data_section.entities {
        match entity_instance {
            ast::EntityInstance::Simple { id, record } => {
                if record.name == "CARTESIAN_POINT" {
                    // Try to deserialize
                    // This is what we need to figure out:
                    let point: CartesianPoint = /* ??? */;
                    
                    // Try to insert
                    // This is also what we need to figure out:
                    // tables.cartesian_point.insert(*id, point);
                }
            }
            _ => {}
        }
    }
}
```

---

## 🚀 Let's Collaborate

**I'm here to help!** If you find something, share it immediately and I'll:
- Update research documents
- Create working code examples
- Document verified patterns

**Let's solve this together!**

---

**Last Updated:** January 27, 2025  
**Status:** 🔬 **RESEARCH IN PROGRESS**  
**Next:** Check ruststep source/documentation for Tables structure

---

*Sam Parker (Junior Engineer, 2D Formats)*  
*For: Riley Thompson (Junior Engineer, 3D Formats)*

