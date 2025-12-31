# opencascade-rs Build Complexity and Binary Size Impact
## Technical Assessment for v0.3.0

**Document Type:** Technical Assessment  
**Target Audience:** System Architect, Senior Engineer, Release Manager  
**Purpose:** Document build complexity and binary size impact of opencascade-rs integration  
**Date:** December 30, 2025  
**Status:** Active Documentation

---

## Executive Summary

This document provides a comprehensive assessment of the build complexity and binary size impact of integrating `opencascade-rs` for full STEP B-Rep support in SimpleImageConverter v0.3.0.

**Key Findings:**
- **Build Complexity:** ⚠️ **HIGH** - Requires OCCT installation, C++ toolchain, and longer build times
- **Binary Size Impact:** ❌ **EXCEEDS TARGET** - +10-15 MB (dynamic) or +90-140 MB (static)
- **Mitigation:** Feature-gated, optional dependency, clear documentation

---

## Build Complexity Assessment

### System Dependencies Required

**OCCT (OpenCASCADE Technology):**
- **Version:** 7.7.0 or later
- **Installation Methods:**
  - Pre-built installer (Windows)
  - Package manager (macOS/Linux)
  - Build from source (all platforms)
- **Installation Time:** 5-60 minutes (depending on method)
- **See:** `docs/OCCT_INSTALLATION.md` for detailed instructions

**Build Tools:**
- **CMake:** 3.18 or later
- **C++ Compiler:** C++17 support required
  - **Windows:** Visual Studio 2019+ (MSVC) or MinGW-w64
  - **macOS:** Xcode Command Line Tools (Clang)
  - **Linux:** GCC 7+ or Clang 5+

**Platform Libraries:**
- **Linux:** X11, OpenGL development libraries
- **macOS:** OpenGL (included with Xcode)
- **Windows:** OpenGL (included with Visual Studio)

### Build Time Impact

**First Build (opencascade-sys compilation):**
- **Time:** 10-30 minutes
- **Factors:**
  - CPU cores (parallel compilation)
  - OCCT library size
  - Linker optimization level
  - System performance

**Incremental Builds:**
- **Time:** 1-5 minutes
- **Factors:**
  - Changes to opencascade-rs code
  - Changes to OCCT installation
  - Cargo cache state

**CI/CD Impact:**
- **Setup Time:** 5-10 minutes (OCCT installation)
- **Build Time:** 10-30 minutes (first build)
- **Total:** 15-40 minutes per CI run
- **Mitigation:** Cache OCCT installation and build artifacts

### Build Process Complexity

**Steps Required:**
1. Install OCCT (platform-specific)
2. Configure environment variables (optional)
3. Build opencascade-sys (automatic via Cargo)
4. Link OCCT libraries (automatic via opencascade-sys)
5. Build application with step-opencascade feature

**Complexity Factors:**
- **Platform-Specific:** Different installation methods per platform
- **Environment Variables:** May need OCCT_DIR, PATH, LD_LIBRARY_PATH
- **Toolchain Requirements:** C++ compiler, CMake
- **Library Linking:** Dynamic vs static linking decisions

**Assessment:** ⚠️ **HIGH COMPLEXITY**
- Requires C++ dependency installation
- Platform-specific configuration
- Longer build times
- CI/CD setup complexity

---

## Binary Size Impact

### Current Binary Sizes (Without opencascade-rs)

**Base Binary (No STEP support):**
- `mesh-convert`: ~5-10 MB

**With STEP (FACETED_BREP only):**
- `mesh-convert`: ~8-12 MB (+3-7 MB)
- **Dependencies:** ruststep, truck crates (pure Rust)

### With opencascade-rs (Dynamic Linking - Recommended)

**Binary Size:**
- `mesh-convert`: ~15-25 MB (+10-15 MB from base)
- **Additional Dependencies:** opencascade, opencascade-sys

**OCCT Runtime Libraries:**
- **Size:** ~100 MB (separate installation, not in binary)
- **Location:** System installation directory
- **Distribution:** User must install OCCT separately

**Total Disk Space:**
- **Binary:** ~15-25 MB
- **OCCT Runtime:** ~100 MB (if installed)
- **Total:** ~115-125 MB (if OCCT installed)

**Advantages:**
- ✅ Smaller binary size
- ✅ OCCT can be shared by multiple applications
- ✅ Easier to update OCCT independently

**Disadvantages:**
- ❌ Requires OCCT installation on target system
- ❌ Runtime dependency management
- ❌ Distribution complexity

### With opencascade-rs (Static Linking)

**Binary Size:**
- `mesh-convert`: ~100-150 MB (+90-140 MB from base)
- **All Dependencies:** Statically linked into binary

**OCCT Runtime Libraries:**
- **Size:** Included in binary
- **Distribution:** Single executable

**Total Disk Space:**
- **Binary:** ~100-150 MB
- **OCCT Runtime:** Included
- **Total:** ~100-150 MB

**Advantages:**
- ✅ No runtime dependencies
- ✅ Simpler distribution (single executable)
- ✅ No OCCT installation required

**Disadvantages:**
- ❌ Very large binary size
- ❌ Exceeds <50MB target significantly
- ❌ Slower download/transfer times
- ❌ Higher memory usage

### Size Comparison

| Configuration | Binary Size | OCCT Runtime | Total | Target Met? |
|--------------|-------------|--------------|-------|-------------|
| Base (no STEP) | 5-10 MB | - | 5-10 MB | ✅ |
| FACETED_BREP | 8-12 MB | - | 8-12 MB | ✅ |
| opencascade (dynamic) | 15-25 MB | 100 MB | 115-125 MB | ❌ |
| opencascade (static) | 100-150 MB | - | 100-150 MB | ❌ |

**Target:** <50MB additional  
**Assessment:** ❌ **EXCEEDS TARGET**
- Static linking: +90-140 MB (exceeds target significantly)
- Dynamic linking: +10-15 MB binary, but requires ~100 MB OCCT runtime

---

## Mitigation Strategies

### 1. Feature Gating

**Implementation:**
- `step-opencascade` feature is optional
- Users can build without opencascade-rs
- Falls back to FACETED_BREP if opencascade-rs unavailable

**Benefits:**
- ✅ Users can choose based on needs
- ✅ Smaller binaries for users who don't need full B-Rep support
- ✅ Reduces distribution complexity

### 2. Dynamic Linking (Recommended)

**Implementation:**
- Use dynamic linking by default
- Document OCCT installation requirement
- Provide clear installation instructions

**Benefits:**
- ✅ Smaller binary size (+10-15 MB vs +90-140 MB)
- ✅ OCCT can be shared by multiple applications
- ✅ Easier to update OCCT independently

**Trade-offs:**
- ❌ Requires OCCT installation on target system
- ❌ Runtime dependency management
- ❌ Distribution complexity

### 3. Clear Documentation

**Implementation:**
- `docs/OCCT_INSTALLATION.md` - Installation guide
- `docs/OPENCASCADE_TROUBLESHOOTING.md` - Troubleshooting guide
- `docs/STEP_FORMAT_REFERENCE.md` - Technical reference
- `docs/OPENCASCADE_BUILD_COMPLEXITY.md` - This document

**Benefits:**
- ✅ Users understand requirements upfront
- ✅ Reduces support burden
- ✅ Clear expectations

### 4. User Choice

**Implementation:**
- Feature flags allow users to choose
- Default build excludes opencascade-rs
- Users opt-in for full B-Rep support

**Benefits:**
- ✅ Users control binary size
- ✅ Reduces complexity for most users
- ✅ Power users can enable full support

---

## CI/CD Considerations

### Build Configuration

**Requirements:**
- OCCT installation in CI environment
- C++ toolchain available
- CMake installed
- Platform-specific setup

**Setup Time:**
- **OCCT Installation:** 5-10 minutes
- **First Build:** 10-30 minutes
- **Total:** 15-40 minutes per CI run

**Optimization:**
- Cache OCCT installation
- Cache build artifacts
- Use parallel builds
- Use release builds for production

### Platform Support

**Windows:**
- OCCT installer or build from source
- MSVC or MinGW toolchain
- PATH configuration for DLLs

**macOS:**
- Homebrew installation
- Xcode Command Line Tools
- DYLD_LIBRARY_PATH configuration

**Linux:**
- Package manager installation
- GCC or Clang toolchain
- LD_LIBRARY_PATH configuration

---

## Recommendations

### For Developers

1. **Use Dynamic Linking:**
   - Smaller binary size
   - Easier to manage
   - Recommended for most use cases

2. **Document Requirements:**
   - Clear installation instructions
   - Troubleshooting guide
   - Build complexity documentation

3. **Feature Gate:**
   - Make opencascade-rs optional
   - Default to FACETED_BREP only
   - Users opt-in for full support

### For Users

1. **Evaluate Needs:**
   - Do you need full B-Rep support?
   - Can you use FACETED_BREP instead?
   - Is binary size a concern?

2. **Choose Configuration:**
   - **Small binary:** Build without `step-opencascade`
   - **Full support:** Build with `step-opencascade` (requires OCCT)

3. **Install OCCT:**
   - Follow `docs/OCCT_INSTALLATION.md`
   - Verify installation before building
   - Check troubleshooting guide if issues

---

## Conclusion

**Build Complexity:** ⚠️ **HIGH**
- Requires OCCT installation
- Platform-specific configuration
- Longer build times
- CI/CD setup complexity

**Binary Size Impact:** ❌ **EXCEEDS TARGET**
- Static linking: +90-140 MB (exceeds <50MB target significantly)
- Dynamic linking: +10-15 MB binary, but requires ~100 MB OCCT runtime

**Mitigation:** ✅ **ADEQUATE**
- Feature-gated (optional dependency)
- Clear documentation
- User choice via feature flags
- Dynamic linking recommended

**Recommendation:** ✅ **PROCEED WITH CAUTION**
- Acceptable as optional feature
- Clear documentation required
- Users can choose based on needs
- Falls back to FACETED_BREP if unavailable

---

**Document Status:** Active  
**Last Updated:** December 30, 2025  
**Maintained By:** Senior Engineer (Jordan Rivera)  
**For:** System Architect, Senior Engineer, Release Manager

