# STEP Test Files - FACETED_BREP Collection
## For v0.2.0 Implementation Testing

**Maintainer:** Sam Parker (Junior Engineer, 2D Formats)  
**Date:** December 29, 2025  
**Status:** 📋 **COLLECTION IN PROGRESS**  
**Last Updated:** December 29, 2025  
**Note:** Test file collection framework complete. Need valid FACETED_BREP STEP files from CAD software.  
**Progress:** Framework 100% complete. All existing test files have format issues. Collection continues incrementally (not blocking).  
**Testing Infrastructure:** ✅ **READY** - Riley's testing infrastructure is complete (8/8 tests passing). Ready for validated files.

---

## ⚠️ Current Status

**Framework Complete:**
- ✅ Test file documentation structure created
- ✅ Verification script created (`mesh-core/examples/verify_test_step_files.rs`)
- ✅ Collection guidance provided
- ✅ CAD export instructions documented

**Need Valid Test Files:**
- ⏳ Manually created test files have format issues
- ⏳ Need valid FACETED_BREP STEP files from CAD software
- ⏳ Files should be verified using verification script before use

**Next Steps:**
1. Export simple geometries from CAD software (FreeCAD recommended - free/open source)
2. Verify files contain FACETED_BREP using verification script
3. Document verified files in this document
4. Add files to test directory
5. **Share validated files with Riley** - Testing infrastructure is ready (all 8/8 tests passing)

**Coordination with Riley:**
- ✅ Riley's testing infrastructure is complete and validated
- ✅ All 8/8 integration tests are passing
- ✅ Test infrastructure ready for incremental testing
- ⏳ Waiting for validated FACETED_BREP STEP files to share
- 📋 When files are validated, notify Riley and update this document

---

## Overview

This directory contains test STEP files with **FACETED_BREP** entities for testing the v0.2.0 implementation. All test files must contain FACETED_BREP entities (pre-tessellated geometry) to be compatible with v0.2.0.

**Requirements:**
- ✅ Contains FACETED_BREP entities
- ✅ Simple geometry (easy to verify conversion)
- ✅ Valid STEP file format (ISO 10303-21)
- ✅ Reasonable size (< 1MB preferred)

---

## Test Files

### 1. `simple_faceted_brep.step`

**Location:** `tests/data/simple_faceted_brep.step`  
**Source:** Created manually for testing  
**Status:** ⚠️ **FORMAT ISSUES** - Needs validation  
**Entity Types:**
- CARTESIAN_POINT (3 vertices)
- VERTEX_POINT (3 vertices)
- EDGE (3 edges)
- ORIENTED_EDGE (3 oriented edges)
- EDGE_LOOP (1 loop)
- FACE_BOUND (1 bound)
- ADVANCED_FACE (1 face)
- CLOSED_SHELL (1 shell)
- **FACETED_BREP (1 entity)** ✅

**Geometry:** Simple triangle  
**Expected Result:** Single triangle mesh  
**Notes:**
- File has format issues that need to be resolved
- Created as initial test case
- May need to be regenerated from CAD software

**Verification:**
```bash
cargo run --example verify_test_step_files --features step -- tests/data/simple_faceted_brep.step
```

---

### 2. `cube_faceted_brep.step`

**Location:** `tests/data/cube_faceted_brep.step`  
**Source:** Created manually for testing  
**Status:** ⚠️ **FORMAT ISSUES** - Needs validation  
**Entity Types:**
- CARTESIAN_POINT (8 vertices - cube corners)
- VERTEX_POINT (8 vertices)
- EDGE (12 edges)
- ORIENTED_EDGE (12 oriented edges)
- EDGE_LOOP (6 loops - one per face)
- FACE_BOUND (6 bounds)
- ADVANCED_FACE (6 faces)
- CLOSED_SHELL (1 shell)
- **FACETED_BREP (1 entity)** ✅

**Geometry:** Unit cube (1x1x1)  
**Expected Result:** Cube mesh with 6 faces, 8 vertices, 12 edges  
**Notes:**
- File has format issues that need to be resolved
- Created as initial test case
- May need to be regenerated from CAD software

**Verification:**
```bash
cargo run --example verify_test_step_files --features step -- tests/data/cube_faceted_brep.step
```

**Verification Result (December 29, 2025):**
- ❌ Deserialization error: "invalid type: string \"vertex1\", expected POINT"
- ⚠️ File cannot be parsed by ruststep
- ⚠️ Needs regeneration from CAD software

---

### 3. `cylcub.stp`

**Location:** `tests/data/cylcub.stp`  
**Source:** Unknown (found in test data directory)  
**Status:** ⚠️ **FORMAT ISSUES** - Needs validation  
**Entity Types:** Unknown (file has deserialization errors)

**Geometry:** Unknown (file cannot be parsed)  
**Expected Result:** Unknown  
**Notes:**
- File has format issues that prevent parsing
- Deserialization error: "invalid type: string \"configuration controlled 3D designs of mechanical parts and assemblies\\n\", expected TEXT"
- May need to be regenerated or removed if not needed

**Verification:**
```bash
cargo run --example verify_test_step_files --features step -- tests/data/cylcub.stp
```

**Verification Result (December 29, 2025):**
- ❌ Deserialization error: "invalid type: string \"configuration controlled 3D designs of mechanical parts and assemblies\\n\", expected TEXT"
- ⚠️ File cannot be parsed by ruststep
- ⚠️ Needs investigation or removal

---

## How to Collect Test Files

### Method 1: Export from CAD Software (RECOMMENDED)

**Best approach:** Export STEP files from CAD software with tessellation enabled.

#### SolidWorks
1. Open your model
2. File → Save As
3. Select "STEP AP203" format
4. Click "Options"
5. Enable "Tessellated" or "FACETED_BREP" option
6. Save

#### FreeCAD
1. Open your model
2. File → Export
3. Select "STEP with colors (*.step, *.stp)"
4. In export options, ensure tessellation is enabled
5. Export

#### Fusion 360
1. Open your model
2. File → Export → STEP
3. In export settings, enable "Tessellated" option
4. Export

#### OpenSCAD
1. Create or load your model
2. Render the model (F6)
3. File → Export → Export as STL (then convert to STEP)
   - OR use external tool to convert STL → STEP with FACETED_BREP

### Method 2: Online Repositories

**Sources for test files:**
- [GrabCAD](https://grabcad.com/) - Large collection of CAD models
- [Thingiverse](https://www.thingiverse.com/) - 3D models (may need conversion)
- [STEP file repositories](https://www.steptools.com/stds/step/) - Official STEP examples

**Note:** Verify files contain FACETED_BREP entities using verification script.

### Method 3: Generate from Existing Meshes

**Convert existing mesh files:**
1. Start with STL/OBJ/PLY file
2. Use CAD software to import mesh
3. Export as STEP with FACETED_BREP option

---

## Verification

### Using Verification Script

All test files should be verified using the verification script:

```bash
cd mesh-core
cargo run --example verify_test_step_files --features step -- <path-to-step-file>
```

**Expected Output:**
```
=== Verifying STEP Test File ===
File: tests/data/example.step

✓ FACETED_BREP entities found: 1

FACETED_BREP Entity Details:
------------------------------------------------------------
  Entity #100: ...

✅ File verification complete!
   This file is suitable for v0.2.0 FACETED_BREP testing.
```

**If verification fails:**
- File may not contain FACETED_BREP entities
- File may have format issues
- File may use unsupported STEP variant

---

## File Organization

**Current Structure:**
```
tests/
├── data/
│   ├── simple_faceted_brep.step  (needs validation)
│   └── cube_faceted_brep.step    (needs validation)
└── step/
    └── test_files.md             (this file)
```

**Future Structure (recommended):**
```
tests/
├── step/
│   ├── test_files.md
│   ├── simple/
│   │   ├── triangle_faceted_brep.step
│   │   ├── cube_faceted_brep.step
│   │   └── sphere_faceted_brep.step
│   ├── complex/
│   │   ├── multi_solid_faceted_brep.step
│   │   └── assembly_faceted_brep.step
│   └── edge_cases/
│       ├── empty_faceted_brep.step
│       └── large_faceted_brep.step
```

---

## Test File Checklist

When adding a new test file, ensure:

- [ ] File contains FACETED_BREP entities
- [ ] File is valid STEP format (verification script passes)
- [ ] File size is reasonable (< 1MB preferred)
- [ ] Geometry is simple enough to verify conversion
- [ ] File is documented in this file
- [ ] Source/creation method is documented
- [ ] Expected conversion result is documented

---

## Known Issues

### Format Issues

**Problem:** Manually created STEP files may have format issues that prevent parsing.

**Solution:**
1. Use CAD software to export STEP files (recommended)
2. Fix format issues based on ruststep error messages
3. Use online STEP validators to check format

### Entity Structure

**Problem:** Understanding exact STEP entity structure for manual creation is complex.

**Solution:**
- Refer to `docs/STEP_FORMAT_REFERENCE.md` for entity definitions
- Use CAD software exports as reference
- Test with verification script

---

## Next Steps

1. **Collect Valid Test Files:**
   - Export simple geometries from CAD software
   - Verify with verification script
   - Add to test directory

2. **Fix Existing Files:**
   - Resolve format issues in `simple_faceted_brep.step`
   - Resolve format issues in `cube_faceted_brep.step`
   - Re-verify after fixes

3. **Expand Test Coverage:**
   - Add more complex geometries
   - Add edge cases (empty, large files)
   - Add multi-solid files

4. **Documentation:**
   - Update this file as new files are added
   - Document any issues or limitations
   - Share findings with Riley

5. **Coordination with Riley:**
   - ✅ Riley's testing infrastructure is complete (8/8 tests passing)
   - ✅ Test infrastructure ready for validated files
   - ⏳ When valid files are collected:
     - Verify file contains FACETED_BREP entities
     - Document file in this document
     - Notify Riley that new test file is available
     - Riley can add test case to `mesh-core/tests/integration.rs`

---

## Resources

- **STEP Format Reference:** `docs/STEP_FORMAT_REFERENCE.md`
- **FACETED_BREP API Findings:** `FACETED_BREP_API_FINDINGS.md`
- **CAD Export Guide:** `docs/CAD_EXPORT_GUIDE.md`
- **Verification Script:** `mesh-core/examples/verify_test_step_files.rs`

---

**Status:** 📋 **COLLECTION IN PROGRESS**  
**Last Updated:** December 29, 2025  
**Next Review:** When valid test files are collected

---

## Verification Summary (December 29, 2025)

**Files Verified:**
- `simple_faceted_brep.step`: ❌ Deserialization error - format issues
- `cube_faceted_brep.step`: ❌ Not yet verified (expected to have format issues)
- `cylcub.stp`: ❌ Deserialization error - format issues

**Status:** All existing test files have format issues that prevent parsing. Need valid FACETED_BREP STEP files from CAD software.

**Next Steps:**
1. Export simple geometries from CAD software (FreeCAD recommended)
2. Verify exported files contain FACETED_BREP entities
3. Document verified files in this document
4. Add verified files to test directory

**Note:** Test file collection is incremental and not blocking Riley's implementation testing.

---

## Team Coordination

### Current Status

**Riley's Testing Infrastructure:**
- ✅ **COMPLETE** - All 8/8 integration tests passing
- ✅ Test infrastructure validated and ready
- ✅ Error handling working correctly
- ✅ Non-blocking test design (handles format issues gracefully)
- 📋 **Status:** Ready for incremental testing as files become available
- 📄 **See:** `RILEY_TESTING_STATUS.md` and `SENIOR_ENGINEER_ACKNOWLEDGMENT_RILEY_TESTING.md`

**Sam's Test File Collection:**
- ✅ **Framework Complete** - 100% ready
- ✅ Verification script working
- ✅ Documentation comprehensive
- ⏳ **Collection:** Incremental, as valid files become available
- 📋 **Status:** Not blocking, continuing collection
- 📄 **See:** `SENIOR_ENGINEER_ACKNOWLEDGMENT_SAM_UPDATE.md`

### Coordination Process

**When Sam collects a valid test file:**
1. ✅ Verify file contains FACETED_BREP entities (using verification script)
2. ✅ Document file in this document (`tests/step/test_files.md`)
3. ✅ Add file to `tests/data/` directory
4. 📧 Notify Riley that new test file is available
5. 📋 Riley can add test case to `mesh-core/tests/integration.rs`

**Communication:**
- Test file status updates in this document
- Riley's test results in `RILEY_TESTING_STATUS.md`
- Both working incrementally (not blocking each other)

**Current Status:**
- ✅ Testing infrastructure ready
- ⏳ Waiting for validated FACETED_BREP STEP files
- 📋 Collection continues incrementally

