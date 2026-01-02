# opencascade-rs Integration - Limitations and Known Issues
## For SimpleImageConverter STEP Support

**Document Type:** Technical Reference  
**Target Audience:** Developers, Users, System Administrators  
**Purpose:** Document limitations, known issues, and workarounds for opencascade-rs integration  
**Date:** December 30, 2025  
**Status:** Active Documentation  
**Maintained By:** Researcher (Taylor Kim), Senior Engineer (Jordan Rivera)

---

## Executive Summary

This document provides a comprehensive list of limitations, known issues, and workarounds for the opencascade-rs integration in SimpleImageConverter. This integration enables full STEP B-Rep support with curved surfaces (NURBS, cylinders, spheres, etc.) but comes with several constraints and requirements.

**Key Points:**
- ⚠️ **OCCT Installation Required:** Cannot use without OpenCASCADE Technology installed
- ⚠️ **Build Complexity:** Requires C++ toolchain and OCCT installation
- ⚠️ **Binary Size Impact:** Significant size increase (especially with static linking)
- ✅ **Feature-Gated:** Can build without opencascade-rs if not needed
- ✅ **Graceful Fallback:** Falls back to FACETED_BREP if opencascade-rs unavailable

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Build Limitations](#build-limitations)
3. [Runtime Limitations](#runtime-limitations)
4. [Performance Limitations](#performance-limitations)
5. [Platform-Specific Limitations](#platform-specific-limitations)
6. [Known Issues](#known-issues)
7. [Workarounds](#workarounds)
8. [Testing Limitations](#testing-limitations)
9. [Future Improvements](#future-improvements)

---

## System Requirements

### OCCT Installation

**Requirement:** OpenCASCADE Technology (OCCT) 7.7+ must be installed on the system.

**Limitations:**
- ❌ **No Bundled OCCT:** OCCT is not bundled with the application
- ❌ **No Auto-Installation:** Users must install OCCT manually
- ❌ **Platform-Specific:** Installation process varies by platform
- ⚠️ **Version Dependency:** Requires specific OCCT version (7.7+)

**Impact:**
- Users must follow installation guide (`docs/OCCT_INSTALLATION.md`)
- Build will fail if OCCT not found
- Runtime will fail if OCCT libraries not accessible

### Build Toolchain

**Requirements:**
- CMake 3.18+
- C++17 compiler (GCC 7+, Clang 5+, or MSVC 2019+)
- Platform-specific libraries (X11, OpenGL on Linux)

**Limitations:**
- ❌ **No Cross-Compilation Support:** Difficult to cross-compile with OCCT
- ❌ **CI/CD Complexity:** Requires OCCT installation in CI environment
- ⚠️ **Build Time:** First build takes 10-30 minutes (opencascade-sys compilation)

---

## Build Limitations

### Binary Size Impact

**Current Measurements:**

**Without opencascade-rs:**
- Base binary: ~5-10 MB
- With STEP (FACETED_BREP only): ~8-12 MB

**With opencascade-rs (Dynamic Linking - Recommended):**
- Binary: ~15-25 MB (+10-15 MB from base)
- OCCT runtime: ~100 MB (separate installation, not in binary)
- **Total disk space:** ~115-125 MB (if OCCT installed)

**With opencascade-rs (Static Linking):**
- Binary: ~100-150 MB (+90-140 MB from base)
- No runtime dependencies
- **Total disk space:** ~100-150 MB

**Assessment:** ❌ **EXCEEDS TARGET** (<50MB additional)
- Static linking: +90-140 MB (exceeds target significantly)
- Dynamic linking: +10-15 MB binary, but requires ~100 MB OCCT runtime

**Mitigation:**
- ✅ Feature-gated (optional dependency)
- ✅ Clear documentation of size impact
- ✅ User choice via feature flags
- ✅ Dynamic linking recommended (smaller binary)

### Build Time Impact

**Expected Build Times:**
- **opencascade-sys compilation:** 10-30 minutes (first build)
- **Incremental builds:** 1-5 minutes (depends on changes)
- **CI/CD impact:** Requires OCCT installation in CI environment

**Limitations:**
- ❌ **Long Initial Build:** First build takes significantly longer
- ❌ **CI/CD Setup:** Requires OCCT installation scripts
- ⚠️ **Platform-Specific:** Build times vary by platform

### Build Configuration

**Limitations:**
- ❌ **No Auto-Detection:** Build system cannot auto-detect OCCT installation
- ❌ **Manual Configuration:** Users must set environment variables or paths
- ⚠️ **Platform-Specific Paths:** OCCT installation paths vary by platform

**Common Issues:**
- Build fails with "OCCT not found" errors
- Linker errors about missing OCCT libraries
- Runtime library not found errors

**See Also:** `docs/OCCT_INSTALLATION.md` troubleshooting section

---

## Runtime Limitations

### OCCT Runtime Dependencies

**Requirement:** OCCT runtime libraries must be accessible at runtime.

**Limitations:**
- ❌ **Dynamic Linking Required:** OCCT libraries must be in library path
- ❌ **Platform-Specific Paths:** Library paths vary by platform
- ⚠️ **Version Mismatch:** Runtime OCCT version must match build-time version

**Common Issues:**
- "Cannot find libTK*.so" (Linux)
- "Cannot find libTK*.dylib" (macOS)
- "Cannot find TK*.dll" (Windows)

**Workarounds:**
- Set `LD_LIBRARY_PATH` (Linux)
- Set `DYLD_LIBRARY_PATH` (macOS)
- Add OCCT bin directory to PATH (Windows)

### Temporary File Handling

**Current Implementation:**
- OCCT expects file paths, not in-memory data
- Implementation writes STEP data to temporary file
- Temporary file is cleaned up after processing

**Limitations:**
- ⚠️ **File System Required:** Cannot process from memory directly
- ⚠️ **Temporary File Creation:** Requires write permissions
- ⚠️ **Cleanup on Panic:** Relies on `tempfile` crate for cleanup

**Future Improvement:**
- Consider in-memory approach if opencascade-rs supports it

### Tessellation Quality

**Current Implementation:**
- Deflection parameter: 0.01 (1% of bounding box size)
- Configurable via `ConversionOptions` (future)

**Limitations:**
- ⚠️ **Fixed Quality:** Default deflection may not be optimal for all models
- ⚠️ **No Adaptive Quality:** Quality is uniform across entire model
- ⚠️ **Performance vs Quality:** Higher quality = more triangles = slower processing

**Workarounds:**
- Adjust deflection parameter in code (future: via ConversionOptions)
- Pre-tessellate STEP files in CAD software (use FACETED_BREP path)

---

## Performance Limitations

### Tessellation Speed

**Expected Performance:**
- Simple models: <1 second
- Complex models: Several seconds
- Very complex models: 10+ seconds

**Limitations:**
- ⚠️ **Variable Performance:** Depends on model complexity
- ⚠️ **No Progress Reporting:** No way to report tessellation progress
- ⚠️ **Blocking Operation:** Tessellation blocks thread (future: async support)

### Memory Usage

**Expected Memory:**
- Scales with mesh complexity
- Typical: 10-100 MB for medium models
- Large models: 100+ MB

**Limitations:**
- ⚠️ **No Memory Limits:** No built-in memory limits (relies on ResourceLimits)
- ⚠️ **OCCT Memory:** OCCT may use additional memory internally

---

## Platform-Specific Limitations

### Windows

**Limitations:**
- ⚠️ **OCCT Installation:** Requires manual installer or build from source
- ⚠️ **PATH Configuration:** Must add OCCT bin directory to PATH
- ⚠️ **Visual Studio Required:** MSVC toolchain recommended

**Common Issues:**
- OCCT not found in PATH
- Missing Visual C++ runtime libraries
- DLL loading errors

### macOS

**Limitations:**
- ⚠️ **Homebrew Installation:** Recommended but not required
- ⚠️ **Apple Silicon:** Different paths for Apple Silicon vs Intel
- ⚠️ **Code Signing:** May require code signing for distribution

**Common Issues:**
- OCCT not found in Homebrew paths
- Library path issues on Apple Silicon
- Code signing errors

### Linux

**Limitations:**
- ⚠️ **Package Manager:** Installation varies by distribution
- ⚠️ **Library Paths:** May need to configure LD_LIBRARY_PATH
- ⚠️ **X11 Dependencies:** Requires X11 development libraries

**Common Issues:**
- OCCT package not available in all distributions
- Library path configuration required
- Missing X11 dependencies

---

## Known Issues

### Issue 1: OCCT Version Mismatch

**Symptoms:**
- Build succeeds but runtime fails
- Linker errors about missing symbols
- "OCCT version mismatch" errors

**Cause:**
- Build-time OCCT version differs from runtime OCCT version
- opencascade-rs compiled against different OCCT version

**Workaround:**
- Ensure OCCT version matches at build-time and runtime
- Rebuild opencascade-rs if OCCT version changes

**Status:** ⚠️ Known limitation, no fix planned (requires user attention)

### Issue 2: Build Fails with "OCCT not found"

**Symptoms:**
- Build fails with "OCCT not found" errors
- opencascade-sys build fails

**Cause:**
- OCCT not installed
- OCCT not in expected location
- Environment variables not set

**Workaround:**
- Install OCCT (see `docs/OCCT_INSTALLATION.md`)
- Set `OCCT_DIR` environment variable
- Configure pkg-config (Linux/macOS)

**Status:** ✅ Documented in installation guide

### Issue 3: Runtime Library Not Found

**Symptoms:**
- Build succeeds but program fails at runtime
- "Cannot find libTK*.so" or similar errors

**Cause:**
- OCCT libraries not in library path
- Dynamic linking cannot find OCCT libraries

**Workaround:**
- Set `LD_LIBRARY_PATH` (Linux)
- Set `DYLD_LIBRARY_PATH` (macOS)
- Add OCCT bin directory to PATH (Windows)

**Status:** ✅ Documented in installation guide

### Issue 4: Tessellation Fails for Complex Models

**Symptoms:**
- STEP file reads successfully but tessellation fails
- Empty mesh returned
- Error: "No geometry could be extracted"

**Cause:**
- Model too complex for default tessellation settings
- Invalid or corrupted geometry in STEP file
- OCCT tessellation algorithm limitations

**Workaround:**
- Try adjusting deflection parameter
- Verify STEP file is valid
- Use FACETED_BREP path if available

**Status:** ⚠️ Under investigation

### Issue 5: Memory Usage High for Large Models

**Symptoms:**
- High memory usage during tessellation
- Out of memory errors for very large models

**Cause:**
- OCCT tessellation uses significant memory
- No built-in memory limits in OCCT

**Workaround:**
- Use ResourceLimits to limit mesh size
- Process smaller models
- Increase system memory

**Status:** ⚠️ Known limitation, ResourceLimits provide partial mitigation

---

## Workarounds

### Workaround 1: Use FACETED_BREP Path

**When to Use:**
- OCCT not installed
- Want smaller binary size
- STEP file contains FACETED_BREP (pre-tessellated)

**How:**
- Build without `step-opencascade` feature
- Export STEP files with tessellation enabled in CAD software
- Use `step` feature only (FACETED_BREP support)

**Limitations:**
- Cannot handle curved surfaces (NURBS, cylinders, spheres)
- Requires CAD software to tessellate before export

### Workaround 2: Dynamic Linking

**When to Use:**
- Want smaller binary size
- OCCT can be installed separately

**How:**
- Use dynamic linking (default)
- Install OCCT separately
- Ensure OCCT libraries are in library path

**Limitations:**
- Requires OCCT installation on target system
- Library path configuration required

### Workaround 3: Feature-Gated Build

**When to Use:**
- Don't need full STEP support
- Want to avoid OCCT dependency

**How:**
- Build without `step-opencascade` feature
- Use `step` feature only (FACETED_BREP support)
- Falls back gracefully if opencascade-rs unavailable

**Limitations:**
- Cannot handle curved surfaces
- Limited STEP support

---

## Testing Limitations

### Integration Testing

**Current Status:**
- ✅ Unit tests for error handling
- ✅ Resource limits validation tests
- ⏳ Integration tests deferred (requires OCCT)

**Limitations:**
- ❌ **OCCT Required:** Integration tests require OCCT installation
- ❌ **CI/CD Setup:** Requires OCCT installation in CI environment
- ⚠️ **Platform-Specific:** Testing varies by platform

**Deferred Tests:**
- Actual STEP file reading with OCCT
- Tessellation testing
- Mesh extraction testing
- Performance testing
- Cross-platform build testing

### Test Files

**Requirements:**
- STEP files with FACETED_BREP (for ruststep path)
- STEP files with MANIFOLD_SOLID_BREP + curved surfaces (for OCCT path)
- STEP files with mixed entities (for fallback testing)

**Limitations:**
- ⚠️ **Test Data:** Requires sample STEP files
- ⚠️ **File Size:** Large test files may be problematic
- ⚠️ **Licensing:** Test files must be properly licensed

---

## Future Improvements

### Planned Improvements

1. **In-Memory Processing:**
   - Support processing from memory (if opencascade-rs supports it)
   - Eliminate temporary file requirement

2. **Configurable Tessellation:**
   - Expose deflection parameter via ConversionOptions
   - Support adaptive quality settings

3. **Progress Reporting:**
   - Report tessellation progress
   - Support cancellation of long operations

4. **Better Error Messages:**
   - More specific error messages for common issues
   - Guidance on how to resolve issues

5. **CI/CD Support:**
   - Automated OCCT installation scripts
   - CI/CD configuration examples

6. **Cross-Platform Testing:**
   - Automated testing on multiple platforms
   - Platform-specific test suites

### Research Areas

1. **Alternative Libraries:**
   - Evaluate other STEP reading libraries
   - Consider pure Rust alternatives

2. **Performance Optimization:**
   - Optimize tessellation performance
   - Reduce memory usage

3. **Binary Size Reduction:**
   - Investigate static linking optimization
   - Consider feature flags for OCCT components

---

## Summary

**Key Limitations:**
- ⚠️ OCCT installation required
- ⚠️ Build complexity high
- ⚠️ Binary size impact significant
- ⚠️ Platform-specific configuration required

**Mitigations:**
- ✅ Feature-gated (optional)
- ✅ Graceful fallback to FACETED_BREP
- ✅ Clear documentation
- ✅ User choice via feature flags

**Recommendation:**
- Use opencascade-rs for full STEP support when needed
- Use FACETED_BREP path for simpler use cases
- Document limitations clearly for users

---

**Document Status:** Active  
**Last Updated:** December 30, 2025  
**Maintained By:** Researcher (Taylor Kim), Senior Engineer (Jordan Rivera)  
**For:** Developers, Users, System Administrators

