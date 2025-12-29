# ruststep FACETED_BREP API - Research Findings
## For Riley: Implementation Support

**Researcher:** Sam Parker (Junior Engineer, 2D Formats)  
**Date:** January 29, 2025  
**Status:** ✅ **RESEARCH COMPLETE**  
**Priority:** 🔥 **URGENT** - Blocking Implementation

---

## Quick Summary

✅ **GOOD NEWS:** `faceted_brep_holders()` method **EXISTS** in ruststep AP203 API!

The method follows the same pattern as other entity accessors:
- `tables.faceted_brep_holders()` → `&HashMap<u64, FacetedBrepHolder>`

---

## ✅ Verified Facts

### 1. Method Exists and Compiles

**Confirmed:** `faceted_brep_holders()` method exists in `ruststep::ap203::config_control_design::Tables`

```rust
use ruststep::ap203::config_control_design::Tables;

let tables = Tables::default(); // or from TableInit::from_data_sections()
let fb_holders = tables.faceted_brep_holders();
// Returns: &HashMap<u64, FacetedBrepHolder>
```

**Verification:** Code compiles successfully (tested in `mesh-core/examples/test_faceted_brep_method.rs`)

---

### 2. API Pattern Consistency

The FACETED_BREP API follows the same pattern as other entities:

| Entity Type | Method | Return Type |
|------------|--------|-------------|
| `CartesianPoint` | `cartesian_point_holders()` | `&HashMap<u64, CartesianPointHolder>` |
| `ManifoldSolidBrep` | `manifold_solid_brep_holders()` | `&HashMap<u64, ManifoldSolidBrepHolder>` |
| `ClosedShell` | `closed_shell_holders()` | `&HashMap<u64, ClosedShellHolder>` |
| **`FacetedBrep`** | **`faceted_brep_holders()`** | **`&HashMap<u64, FacetedBrepHolder>`** |

**Pattern:** `[entity_name]_holders()` → `&HashMap<u64, EntityHolder>`

---

### 3. Entity Hierarchy (STEP AP203)

In STEP AP203, FACETED_BREP is a **subtype** of MANIFOLD_SOLID_BREP:

```
MANIFOLD_SOLID_BREP
  └── FACETED_BREP (subtype)
```

**Implication:** 
- FACETED_BREP entities are also accessible via `manifold_solid_brep_holders()`
- But `faceted_brep_holders()` gives direct access to FACETED_BREP entities only
- You may need to check entity type if using `manifold_solid_brep_holders()`

---

## 🔬 Entity Structure (To Be Explored)

### FacetedBrepHolder Fields

**Status:** ⚠️ **NEEDS VERIFICATION** - Structure to be explored with actual STEP file

**Expected Fields (based on STEP AP203 spec):**
- `name`: Optional name/description
- `outer`: Reference to `CLOSED_SHELL` entity (the outer boundary)

**Entity Traversal Path:**
```
FACETED_BREP
  └── outer: CLOSED_SHELL
      └── cfs_faces: LIST OF FACE
          └── bounds: SET OF FACE_BOUND
              └── bound: EDGE_LOOP
                  └── edge_list: LIST OF ORIENTED_EDGE
                      └── edge_element: EDGE
                          └── edge_start: VERTEX_POINT
                              └── vertex_geometry: CARTESIAN_POINT
```

---

## 📋 Code Examples

### Example 1: Basic Access

```rust
use ruststep::ap203::config_control_design::Tables;
use ruststep::tables::TableInit;

// Parse STEP file and create Tables
let exchange = parser::parse(step_text)?;
let tables = Tables::from_data_sections(&exchange.data)?;

// Access FACETED_BREP entities
let fb_holders = tables.faceted_brep_holders();

if fb_holders.is_empty() {
    return Err(ConversionError::ConversionFailed(
        "No FACETED_BREP entities found. \
         Please export STEP file with FACETED_BREP tessellation enabled."
            .to_string(),
    ));
}

// Iterate over FACETED_BREP entities
for (id, holder) in fb_holders.iter() {
    println!("Found FACETED_BREP entity #{}", id);
    // Access holder fields here
}
```

### Example 2: Get Owned Entity (Fully Resolved)

```rust
use ruststep::tables::IntoOwned;

// Get fully resolved FacetedBrep entity
// This resolves all references and gives you the complete entity
if let Some((id, holder)) = fb_holders.iter().next() {
    // Check if IntoOwned is implemented for FacetedBrepHolder
    // let owned: config_control_design::FacetedBrep = holder.get_owned(&tables)?;
    
    // Access fields:
    // owned.outer -> ClosedShell reference
    // Then traverse to CLOSED_SHELL -> FACE -> vertices
}
```

### Example 3: Fallback to ManifoldSolidBrep

```rust
// If no FACETED_BREP found, check MANIFOLD_SOLID_BREP
let fb_holders = tables.faceted_brep_holders();
let msb_holders = tables.manifold_solid_brep_holders();

if fb_holders.is_empty() && msb_holders.is_empty() {
    return Err(ConversionError::ConversionFailed(
        "No supported geometric entities found. \
         For v0.2.0, only FACETED_BREP (pre-tessellated) geometry is supported."
            .to_string(),
    ));
}

// Prefer FACETED_BREP if available
if !fb_holders.is_empty() {
    // Use FACETED_BREP entities
} else {
    // Fallback: Check if MANIFOLD_SOLID_BREP entities are actually FACETED_BREP
    // (This requires checking entity type in raw Exchange data)
}
```

---

## 🔍 Next Steps for Implementation

### For Riley (Implementation):

1. **✅ Use `faceted_brep_holders()` method** - Confirmed to exist
2. **🔬 Explore FacetedBrepHolder structure** - Check fields (likely `outer` for CLOSED_SHELL)
3. **🔬 Test with actual STEP file** - Need FACETED_BREP STEP file to verify structure
4. **🔬 Implement entity traversal** - FACETED_BREP → CLOSED_SHELL → FACE → vertices
5. **🔬 Check IntoOwned trait** - For fully resolved entities

### For Sam (Support):

1. **✅ API research complete** - Method exists, pattern confirmed
2. **📝 Documentation updates** - Update FORMATS.md, README.md, create CAD_EXPORT_GUIDE.md
3. **📁 Test file collection** - Find/create FACETED_BREP STEP files
4. **💬 Error message review** - Improve user-facing error messages

---

## 📚 Resources

- **ruststep docs.rs:** https://docs.rs/ruststep/0.4/
- **ruststep GitHub:** https://github.com/ricosjp/ruststep
- **STEP AP203 Spec:** ISO 10303-203 (Automotive Design)
- **Previous Research:** `TABLES_API_FINDINGS_FOR_RILEY.md`
- **Verification Code:** `mesh-core/examples/test_faceted_brep_method.rs`
- **Exploration Code:** `mesh-core/examples/explore_faceted_brep.rs`

---

## ⚠️ Known Limitations

1. **Structure Exploration:** Need actual FACETED_BREP STEP file to verify holder fields
2. **IntoOwned Trait:** Need to verify if `FacetedBrepHolder` implements `IntoOwned`
3. **Entity Traversal:** Need to implement full traversal path (FACETED_BREP → vertices)
4. **Error Handling:** Need to handle cases where FACETED_BREP is missing

---

## 💡 Implementation Recommendations

### Recommended Approach:

1. **Start with `faceted_brep_holders()`** - Direct access to FACETED_BREP entities
2. **Check if empty** - Provide clear error message if no FACETED_BREP found
3. **Access `outer` field** - Get CLOSED_SHELL reference from FacetedBrepHolder
4. **Traverse to vertices** - Follow entity path to extract geometry
5. **Build Mesh** - Convert extracted geometry to Mesh structure

### Error Messages:

```rust
if fb_holders.is_empty() {
    return Err(ConversionError::ConversionFailed(
        "STEP file contains no FACETED_BREP entities. \
         For v0.2.0, only pre-tessellated (FACETED_BREP) geometry is supported. \
         Please export your STEP file with tessellation enabled. \
         See docs/CAD_EXPORT_GUIDE.md for instructions."
            .to_string(),
    ));
}
```

---

## ✅ Success Criteria

- [x] **Method exists** - `faceted_brep_holders()` confirmed
- [x] **API pattern understood** - Follows same pattern as other entities
- [ ] **Structure verified** - Need actual STEP file to verify holder fields
- [ ] **Traversal implemented** - Riley to implement entity traversal
- [ ] **Test files available** - Sam to collect test STEP files

---

## 🚀 Ready for Implementation

**Status:** ✅ **API RESEARCH COMPLETE**

Riley can now proceed with implementation using `faceted_brep_holders()`. The method exists and follows the expected pattern. Next step is to explore the `FacetedBrepHolder` structure with an actual STEP file.

---

**Last Updated:** January 29, 2025  
**Status:** ✅ **RESEARCH COMPLETE**  
**Next:** Implementation by Riley, Documentation by Sam

---

*Sam Parker (Junior Engineer, 2D Formats)*  
*For: Riley Thompson (Junior Engineer, 3D Formats)*

