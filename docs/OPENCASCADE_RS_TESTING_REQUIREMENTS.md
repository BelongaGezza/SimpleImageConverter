# opencascade-rs Testing Requirements
## For SimpleImageConverter STEP Support

**Document Type:** Testing Guide  
**Target Audience:** Developers, QA Engineers  
**Purpose:** Document testing requirements and procedures for opencascade-rs integration  
**Date:** December 30, 2025  
**Status:** Active Documentation  
**Maintained By:** Researcher (Taylor Kim), Senior Engineer (Jordan Rivera)

---

## Executive Summary

This document outlines the testing requirements and procedures for the opencascade-rs integration in SimpleImageConverter. Testing requires OpenCASCADE Technology (OCCT) to be installed, which may not be available in all development environments.

**Key Points:**
- ⚠️ **OCCT Required:** All integration tests require OCCT installation
- ✅ **Unit Tests:** Can run without OCCT (error handling, resource limits)
- ⏳ **Integration Tests:** Deferred if OCCT not available
- 📋 **Test Files:** Requires sample STEP files with various geometry types

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Unit Testing](#unit-testing)
3. [Integration Testing](#integration-testing)
4. [Test Files Required](#test-files-required)
5. [Test Cases](#test-cases)
6. [Performance Testing](#performance-testing)
7. [Cross-Platform Testing](#cross-platform-testing)
8. [CI/CD Testing](#cicd-testing)
9. [Testing Without OCCT](#testing-without-occt)

---

## Prerequisites

### OCCT Installation

**Requirement:** OpenCASCADE Technology (OCCT) 7.7+ must be installed.

**Installation:** See `docs/OCCT_INSTALLATION.md` for detailed instructions.

**Verification:**
```bash
# Verify OCCT installation
cargo build --features step-opencascade

# If successful, OCCT is properly installed
```

### Build Configuration

**Feature Flag:** `step-opencascade`

**Build Command:**
```bash
cargo build --features step-opencascade
```

**Test Command:**
```bash
cargo test --features step-opencascade
```

---

## Unit Testing

### Tests That Don't Require OCCT

**Location:** `mesh-core/src/formats/step_opencascade.rs`

**Test Cases:**
1. ✅ **Error Handling:** Test with empty/invalid STEP files
2. ✅ **Resource Limits:** Test resource limit validation
3. ✅ **File Size Validation:** Test file size checks
4. ✅ **Error Messages:** Verify error messages are user-friendly

**Example:**
```rust
#[test]
fn test_extract_mesh_empty_file() {
    let data = b"ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;";
    let limits = ResourceLimits::default();
    let result = extract_mesh(data, &limits, 0.01);
    assert!(result.is_err());
}
```

**Status:** ✅ **COMPLETE** - These tests can run without OCCT

---

## Integration Testing

### Tests That Require OCCT

**Location:** `mesh-core/tests/integration_step_opencascade.rs` (to be created)

**Prerequisites:**
- OCCT installed and accessible
- Sample STEP files available
- Build with `step-opencascade` feature

**Test Cases:**

#### Test 1: Basic STEP File Reading

**Purpose:** Verify OCCT can read STEP files

**Test File:** STEP file with MANIFOLD_SOLID_BREP + curved surfaces

**Expected:**
- STEP file reads successfully
- Shape extracted from OCCT
- No errors during reading

**Status:** ⏳ **DEFERRED** - Requires OCCT installation

#### Test 2: Tessellation

**Purpose:** Verify tessellation works correctly

**Test File:** STEP file with curved surfaces (cylinders, spheres, NURBS)

**Expected:**
- Tessellation completes successfully
- Mesh extracted with vertices and faces
- No degenerate triangles
- Normals calculated correctly

**Status:** ⏳ **DEFERRED** - Requires OCCT installation

#### Test 3: Mesh Extraction

**Purpose:** Verify mesh data extraction

**Test File:** STEP file with complex geometry

**Expected:**
- Vertices extracted correctly
- Faces extracted correctly
- Vertex deduplication works
- No missing or invalid indices

**Status:** ⏳ **DEFERRED** - Requires OCCT installation

#### Test 4: Fallback Mechanism

**Purpose:** Verify fallback to FACETED_BREP works

**Test File:** STEP file with FACETED_BREP (should use ruststep path)

**Expected:**
- FACETED_BREP path used (faster)
- opencascade-rs not called
- Mesh extracted correctly

**Status:** ⏳ **DEFERRED** - Requires OCCT installation

#### Test 5: Error Handling

**Purpose:** Verify error handling for invalid files

**Test Files:**
- Corrupted STEP file
- Empty STEP file
- Invalid geometry

**Expected:**
- Appropriate error messages
- No panics
- Graceful failure

**Status:** ⏳ **DEFERRED** - Requires OCCT installation

---

## Test Files Required

### Test File Categories

**1. FACETED_BREP Files (for ruststep path):**
- Purpose: Test FACETED_BREP extraction
- Geometry: Pre-tessellated (triangulated)
- Expected: Should use ruststep path (faster)

**2. MANIFOLD_SOLID_BREP Files (for OCCT path):**
- Purpose: Test opencascade-rs integration
- Geometry: Curved surfaces (NURBS, cylinders, spheres)
- Expected: Should use OCCT path (tessellation required)

**3. Mixed Entity Files:**
- Purpose: Test fallback mechanism
- Geometry: Mix of FACETED_BREP and MANIFOLD_SOLID_BREP
- Expected: Should try FACETED_BREP first, fall back to OCCT if needed

**4. Invalid/Corrupted Files:**
- Purpose: Test error handling
- Geometry: Invalid or corrupted
- Expected: Appropriate error messages

### Test File Sources

**Options:**
1. **Create Test Files:** Use CAD software to create test files
2. **Public Test Files:** Use publicly available STEP files (ensure licensing)
3. **Generated Test Files:** Generate simple test files programmatically

**Licensing:** Ensure all test files are properly licensed for use in tests.

---

## Test Cases

### Test Case 1: Simple Cylinder

**Purpose:** Test basic curved surface (cylinder)

**Geometry:** Simple cylinder with curved surface

**Expected:**
- STEP file reads successfully
- Tessellation completes
- Mesh has reasonable number of triangles
- Normals calculated correctly

**Status:** ⏳ **DEFERRED** - Requires OCCT and test file

### Test Case 2: Complex Model

**Purpose:** Test complex geometry with multiple surfaces

**Geometry:** Model with multiple curved surfaces

**Expected:**
- All surfaces tessellated
- Mesh extracted correctly
- Performance acceptable (<10 seconds)

**Status:** ⏳ **DEFERRED** - Requires OCCT and test file

### Test Case 3: Large Model

**Purpose:** Test with large/complex model

**Geometry:** Large model with many surfaces

**Expected:**
- Tessellation completes (may take longer)
- Memory usage acceptable
- Resource limits enforced

**Status:** ⏳ **DEFERRED** - Requires OCCT and test file

### Test Case 4: Deflection Parameter

**Purpose:** Test different tessellation quality settings

**Parameters:** Different deflection values (0.001, 0.01, 0.1)

**Expected:**
- Smaller deflection = more triangles = higher quality
- Larger deflection = fewer triangles = lower quality
- All produce valid meshes

**Status:** ⏳ **DEFERRED** - Requires OCCT and test file

---

## Performance Testing

### Performance Metrics

**Measure:**
- Tessellation time
- Memory usage
- Mesh size (vertices, faces)
- Binary size impact

### Performance Targets

**Tessellation Time:**
- Simple models: <1 second
- Medium models: <5 seconds
- Complex models: <30 seconds

**Memory Usage:**
- Simple models: <50 MB
- Medium models: <200 MB
- Complex models: <500 MB

**Mesh Size:**
- Simple models: <10K vertices
- Medium models: <100K vertices
- Complex models: <1M vertices

**Status:** ⏳ **DEFERRED** - Requires OCCT and test files

---

## Cross-Platform Testing

### Platforms to Test

**Windows:**
- OCCT installation via installer
- MSVC toolchain
- Dynamic linking

**macOS:**
- OCCT installation via Homebrew
- Clang toolchain
- Dynamic linking

**Linux:**
- OCCT installation via package manager
- GCC toolchain
- Dynamic linking

### Test Matrix

| Platform | OCCT Installation | Build | Runtime | Status |
|----------|------------------|-------|---------|--------|
| Windows | ⏳ | ⏳ | ⏳ | Deferred |
| macOS | ⏳ | ⏳ | ⏳ | Deferred |
| Linux | ⏳ | ⏳ | ⏳ | Deferred |

**Status:** ⏳ **DEFERRED** - Requires OCCT installation on each platform

---

## CI/CD Testing

### CI/CD Requirements

**Prerequisites:**
- OCCT installation in CI environment
- Platform-specific installation scripts
- Test files available in CI

### CI/CD Setup

**GitHub Actions Example:**
```yaml
- name: Install OCCT (Linux)
  run: |
    sudo apt-get update
    sudo apt-get install -y libocct-*-dev

- name: Build with step-opencascade
  run: cargo build --features step-opencascade

- name: Run tests
  run: cargo test --features step-opencascade
```

**Status:** ⏳ **DEFERRED** - Requires CI/CD configuration

---

## Testing Without OCCT

### What Can Be Tested

**Without OCCT:**
- ✅ Unit tests (error handling, resource limits)
- ✅ Build system (feature flags, dependencies)
- ✅ Code compilation
- ✅ Error messages

**Cannot Be Tested:**
- ❌ Actual STEP file reading
- ❌ Tessellation
- ❌ Mesh extraction
- ❌ Performance
- ❌ Integration tests

### Testing Strategy

**Phase 1: Without OCCT (Current)**
- ✅ Unit tests for error handling
- ✅ Resource limits validation
- ✅ Build system verification
- ✅ Code review

**Phase 2: With OCCT (When Available)**
- ⏳ Integration tests
- ⏳ Performance testing
- ⏳ Cross-platform testing
- ⏳ CI/CD setup

---

## Test Implementation Status

### Completed Tests

- ✅ Unit test: Empty file handling
- ✅ Unit test: Error handling
- ✅ Unit test: Resource limits validation
- ✅ Build system: Feature flag verification

### Deferred Tests (Require OCCT)

- ⏳ Integration test: STEP file reading
- ⏳ Integration test: Tessellation
- ⏳ Integration test: Mesh extraction
- ⏳ Integration test: Fallback mechanism
- ⏳ Performance test: Tessellation speed
- ⏳ Performance test: Memory usage
- ⏳ Cross-platform test: Windows
- ⏳ Cross-platform test: macOS
- ⏳ Cross-platform test: Linux
- ⏳ CI/CD test: Automated testing

---

## Next Steps

### Immediate (When OCCT Available)

1. **Install OCCT** on development system
2. **Verify Build** with `cargo build --features step-opencascade`
3. **Create Test Files** or obtain sample STEP files
4. **Implement Integration Tests** in `mesh-core/tests/integration_step_opencascade.rs`
5. **Run Tests** and verify functionality

### Future

1. **Performance Testing:** Measure tessellation time and memory usage
2. **Cross-Platform Testing:** Test on Windows, macOS, Linux
3. **CI/CD Setup:** Configure automated testing in CI
4. **Test File Library:** Create collection of test files
5. **Documentation:** Update with test results

---

## Summary

**Current Status:**
- ✅ Unit tests complete (can run without OCCT)
- ⏳ Integration tests deferred (require OCCT)
- ⏳ Performance tests deferred (require OCCT)
- ⏳ Cross-platform tests deferred (require OCCT)

**Requirements for Full Testing:**
- OCCT installation
- Sample STEP files
- Test implementation
- CI/CD configuration

**Recommendation:**
- Proceed with unit tests (already complete)
- Defer integration tests until OCCT available
- Document testing requirements clearly
- Plan for OCCT installation in development environment

---

**Document Status:** Active  
**Last Updated:** December 30, 2025  
**Maintained By:** Researcher (Taylor Kim), Senior Engineer (Jordan Rivera)  
**For:** Developers, QA Engineers

