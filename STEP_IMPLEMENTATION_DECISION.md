# STEP Implementation Decision & Status
**Date:** January 27, 2025  
**Status:** Blocked - Library Limitation

## Investigation Summary

### Key Finding: truck-stepio Input Not Available

**Issue:** `truck-stepio 0.3.0` does **not support STEP file input/reading**.  
**Status:** Primarily designed for STEP **output** (writing) only.  
**Source:** Documentation indicates "Input will come further down the road"

### Tessellation API Discovery

**Working API (from GitHub issue #68):**
```rust
use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};

// For shapes that implement MeshableShape trait
let polygon_mesh = shape.triangulation(tolerance).to_polygon();
```

**Requirements:**
- Need `truck-meshalgo` dependency (not currently in Cargo.toml)
- `triangulation()` is a trait method on `MeshableShape`
- Call `.to_polygon()` to get `PolygonMesh`

### Current Format Support

**✅ Working Formats (6):**
- STL (binary/ASCII)
- OBJ (with materials)
- PLY
- OFF
- glTF/GLB
- DXF

**🚧 STEP Format:**
- Code structure ready
- Blocked by library limitation
- Placeholder returns informative error

## Decision: Defer STEP Support

### Rationale

1. **Library Limitation:** STEP input not available in truck-stepio yet
2. **Alternative Complexity:** Using `ruststep` directly would require:
   - Manual conversion from ruststep data structures to truck `Shell`/`Solid` types
   - Significant implementation effort (non-trivial conversion)
   - Maintenance burden

3. **Current Status:** 6 formats already working - good coverage
4. **Future Path:** Can add STEP when truck-stepio input support is available

### Recommended Action

**Keep current placeholder implementation** with clear documentation:

- ✅ Code structure ready (can be uncommented when API available)
- ✅ Error messages inform users about limitation
- ✅ Tests pass (validate placeholder behavior)
- ✅ Feature flag system in place

**Documentation updates:**
- Update README to clarify STEP limitation
- Keep STEP marked as "partial/in progress"
- Note dependency on truck-stepio future input support

## Future Path (When STEP Input Becomes Available)

1. **Add truck-meshalgo dependency:**
   ```toml
   truck-meshalgo = "0.4.0"
   ```

2. **Uncomment implementation code** in `step.rs`

3. **Use correct API:**
   ```rust
   // STEP reading (when available)
   let shells = truck_stepio::in::read(step_text)?;
   
   // Tessellation
   use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};
   let mesh = shell.triangulation(0.01).to_polygon();
   ```

4. **Test with real STEP files**

## Alternative Considered

**Option:** Use `ruststep` directly + manual conversion
- **Status:** Rejected (too complex)
- **Reason:** Would require implementing non-trivial conversion from ruststep data to truck types
- **Maintenance:** High burden, fragile

## Status Going Forward

- **Current:** STEP format handler exists but returns informative error
- **Users:** Will see clear error message explaining limitation
- **Future:** Can be completed when truck-stepio adds input support
- **Priority:** Low (6 other formats working, STEP can wait)

