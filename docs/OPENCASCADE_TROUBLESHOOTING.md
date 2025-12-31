# opencascade-rs Troubleshooting Guide
## Common Issues and Solutions

**Document Type:** Troubleshooting Guide  
**Target Audience:** Developers and Users  
**Purpose:** Help resolve common issues with opencascade-rs integration  
**Date:** December 30, 2025  
**Status:** Active Documentation

---

## Table of Contents

1. [Build Issues](#build-issues)
2. [Runtime Issues](#runtime-issues)
3. [OCCT Installation Issues](#occt-installation-issues)
4. [Performance Issues](#performance-issues)
5. [Testing Issues](#testing-issues)
6. [Platform-Specific Issues](#platform-specific-issues)

---

## Build Issues

### Issue: opencascade-sys Cannot Find OCCT

**Symptoms:**
```
error: failed to run custom build command for `opencascade-sys`
error: OCCT not found
```

**Solutions:**

**Windows:**
1. Verify OCCT installation:
   ```powershell
   dir C:\OpenCASCADE-7.7.0\lib
   ```
2. Set environment variable:
   ```powershell
   set OCCT_DIR=C:\OpenCASCADE-7.7.0
   ```
3. Add to PATH:
   ```powershell
   set PATH=%PATH%;C:\OpenCASCADE-7.7.0\bin
   ```
4. Rebuild:
   ```bash
   cargo clean
   cargo build --features step-opencascade
   ```

**macOS:**
1. Verify Homebrew installation:
   ```bash
   brew list opencascade
   ```
2. Set environment variable:
   ```bash
   export OCCT_DIR=/opt/homebrew  # Apple Silicon
   # or
   export OCCT_DIR=/usr/local    # Intel
   ```
3. Rebuild:
   ```bash
   cargo clean
   cargo build --features step-opencascade
   ```

**Linux:**
1. Verify package installation:
   ```bash
   dpkg -l | grep occt
   ```
2. Set environment variable:
   ```bash
   export OCCT_DIR=/usr
   ```
3. Rebuild:
   ```bash
   cargo clean
   cargo build --features step-opencascade
   ```

---

### Issue: C++ Compiler Not Found

**Symptoms:**
```
error: failed to run custom build command for `opencascade-sys`
error: C++ compiler not found
```

**Solutions:**

**Windows:**
1. Install Visual Studio 2019+ with C++ workload
2. Or install MinGW-w64:
   ```powershell
   choco install mingw
   ```
3. Verify compiler:
   ```powershell
   cl  # MSVC
   # or
   g++ --version  # MinGW
   ```

**macOS:**
1. Install Xcode Command Line Tools:
   ```bash
   xcode-select --install
   ```
2. Verify compiler:
   ```bash
   clang++ --version
   ```

**Linux:**
1. Install build tools:
   ```bash
   sudo apt-get install build-essential
   ```
2. Verify compiler:
   ```bash
   g++ --version
   ```

---

### Issue: CMake Not Found

**Symptoms:**
```
error: failed to run custom build command for `opencascade-sys`
error: CMake not found
```

**Solutions:**

**Windows:**
1. Download CMake from https://cmake.org/download/
2. Install and add to PATH
3. Verify:
   ```powershell
   cmake --version
   ```

**macOS:**
```bash
brew install cmake
cmake --version
```

**Linux:**
```bash
sudo apt-get install cmake
cmake --version
```

---

### Issue: C++17 Standard Not Supported

**Symptoms:**
```
error: C++17 standard required
error: C++ compiler does not support C++17
```

**Solutions:**

1. **Update Compiler:**
   - **GCC:** Version 7+ required
   - **Clang:** Version 5+ required
   - **MSVC:** Visual Studio 2019+ required

2. **Set C++ Standard Explicitly:**
   ```bash
   export CXXFLAGS="-std=c++17"
   cargo build --features step-opencascade
   ```

3. **Verify Compiler Version:**
   ```bash
   g++ --version  # Should be 7+
   clang++ --version  # Should be 5+
   ```

---

### Issue: Build Takes Too Long

**Symptoms:**
- opencascade-sys compilation takes 10-30 minutes
- Incremental builds are slow

**Solutions:**

1. **Use Incremental Builds:**
   - Only rebuilds changed files
   - First build is always slow

2. **Use Release Build:**
   ```bash
   cargo build --release --features step-opencascade
   ```
   - Release builds are optimized but take longer
   - Debug builds are faster but larger

3. **Parallel Builds:**
   - Cargo automatically uses parallel builds
   - Ensure sufficient CPU cores available

4. **Cache Build Artifacts:**
   - Use `sccache` for faster rebuilds:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

---

## Runtime Issues

### Issue: Cannot Find OCCT Libraries at Runtime

**Symptoms:**
```
error: cannot find libTK*.so
error: cannot find libTK*.dylib
error: DLL not found
```

**Solutions:**

**Windows:**
1. Ensure OCCT DLLs are in PATH:
   ```powershell
   set PATH=%PATH%;C:\OpenCASCADE-7.7.0\bin
   ```
2. Or copy DLLs to executable directory

**macOS:**
```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/lib:$DYLD_LIBRARY_PATH
```

**Linux:**
```bash
export LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH
# Or add to /etc/ld.so.conf.d/opencascade.conf and run:
sudo ldconfig
```

---

### Issue: STEP File Reading Fails

**Symptoms:**
```
error: Failed to read STEP file with OpenCASCADE
error: STEP file contains no root entities
```

**Solutions:**

1. **Verify STEP File:**
   - Check file is valid STEP format
   - Try opening in other CAD software

2. **Check File Permissions:**
   - Ensure file is readable
   - Check temporary file creation permissions

3. **Try FACETED_BREP Path:**
   - Some files may work with pure Rust path
   - Build without `step-opencascade` feature

4. **Check Error Messages:**
   - Error messages should indicate specific issue
   - Review OCCT documentation for error codes

---

### Issue: Tessellation Fails

**Symptoms:**
```
error: No geometry could be extracted from STEP file
error: Tessellation failed
```

**Solutions:**

1. **Adjust Deflection Parameter:**
   - Smaller deflection = higher quality (more triangles)
   - Larger deflection = lower quality (fewer triangles)
   - Default: 0.01 (1% of bounding box)

2. **Check Geometry Type:**
   - Some geometry types may not tessellate
   - Verify STEP file contains valid B-Rep geometry

3. **Try Different Deflection:**
   ```rust
   // In code, adjust deflection parameter
   extract_mesh(data, limits, 0.1)  // Larger deflection
   ```

---

## OCCT Installation Issues

### Issue: OCCT Installation Fails

**Symptoms:**
- Installer fails
- Package manager cannot find OCCT
- Build from source fails

**Solutions:**

**Windows:**
1. Try different installer version
2. Check system requirements (Windows version, architecture)
3. Run installer as administrator

**macOS:**
1. Update Homebrew:
   ```bash
   brew update
   brew upgrade
   ```
2. Try installing specific version:
   ```bash
   brew install opencascade@7.7
   ```

**Linux:**
1. Update package manager:
   ```bash
   sudo apt-get update
   ```
2. Try alternative package:
   ```bash
   sudo apt-get install libopencascade-dev
   ```

---

### Issue: OCCT Version Mismatch

**Symptoms:**
```
error: OCCT version 7.7+ required
error: Found version 7.6
```

**Solutions:**

1. **Upgrade OCCT:**
   - Install OCCT 7.7 or later
   - See `docs/OCCT_INSTALLATION.md`

2. **Check Version:**
   ```bash
   # macOS
   brew list --versions opencascade
   
   # Linux
   dpkg -l | grep occt
   ```

---

## Performance Issues

### Issue: Tessellation Is Slow

**Symptoms:**
- STEP file conversion takes several seconds
- Complex models are very slow

**Solutions:**

1. **Adjust Deflection:**
   - Larger deflection = faster tessellation
   - Trade-off: Lower quality mesh

2. **Optimize Resource Limits:**
   - Reduce max vertices/faces if not needed
   - Prevents processing very large models

3. **Use FACETED_BREP When Possible:**
   - Pre-tessellated geometry is faster
   - Only use opencascade-rs for curved surfaces

---

### Issue: High Memory Usage

**Symptoms:**
- Memory usage spikes during conversion
- Out of memory errors

**Solutions:**

1. **Adjust Resource Limits:**
   - Reduce max file size
   - Reduce max vertices/faces

2. **Process Files Individually:**
   - Don't process multiple large files simultaneously
   - Use batch processing with concurrency limits

3. **Monitor Memory:**
   - Use system tools to monitor memory usage
   - Identify memory-intensive operations

---

## Testing Issues

### Issue: Tests Fail Without OCCT

**Symptoms:**
- Integration tests fail
- Tests require OCCT but it's not installed

**Solutions:**

1. **Skip Tests Without OCCT:**
   ```bash
   # Tests are feature-gated
   cargo test --features step-opencascade
   ```

2. **Install OCCT:**
   - See `docs/OCCT_INSTALLATION.md`
   - Install OCCT before running tests

3. **Use CI/CD:**
   - Tests run in CI/CD with OCCT installed
   - Local testing may require OCCT installation

---

### Issue: No Test STEP Files

**Symptoms:**
- Tests require sample STEP files
- Test files not available

**Solutions:**

1. **Create Test Files:**
   - Export simple geometry from CAD software
   - Use FACETED_BREP export option

2. **Use Public Test Files:**
   - Search for public STEP test files
   - Use NIST STEP File Analyzer sample files

3. **Skip Integration Tests:**
   - Unit tests don't require STEP files
   - Integration tests require OCCT and test files

---

## Platform-Specific Issues

### Windows-Specific Issues

**Issue: MSVC vs MinGW**
- opencascade-sys may prefer MSVC
- Try both toolchains if one fails

**Issue: Path Length**
- Windows has path length limits
- Use shorter paths for OCCT installation

**Issue: DLL Dependencies**
- OCCT DLLs must be in PATH
- Or copy to executable directory

---

### macOS-Specific Issues

**Issue: Apple Silicon vs Intel**
- OCCT paths differ:
  - Apple Silicon: `/opt/homebrew`
  - Intel: `/usr/local`

**Issue: Code Signing**
- May need to disable code signing for development
- Production builds require proper signing

**Issue: DYLD_LIBRARY_PATH**
- macOS restricts DYLD_LIBRARY_PATH in some contexts
- May need to use `install_name_tool` for production

---

### Linux-Specific Issues

**Issue: Package Manager Differences**
- Ubuntu/Debian: `libocct-*-dev`
- Fedora/RHEL: `opencascade-devel`
- Arch: `opencascade`

**Issue: Library Path**
- Libraries may be in `/usr/lib` or `/usr/lib/x86_64-linux-gnu`
- Check with `ldconfig -p | grep occt`

**Issue: X11 Dependencies**
- OCCT may require X11 libraries
- Install: `sudo apt-get install libx11-dev`

---

## Getting Help

### Resources

1. **Documentation:**
   - `docs/OCCT_INSTALLATION.md` - Installation guide
   - `RESEARCH_OPENCASCADE_RS_SPRINT9.md` - Research findings
   - `docs/STEP_FORMAT_REFERENCE.md` - STEP format reference

2. **Official Resources:**
   - OCCT Documentation: https://dev.opencascade.org/doc/refman/html/
   - opencascade-rs Repository: https://github.com/bschwind/opencascade-rs

3. **Project Resources:**
   - Check project issue tracker
   - Review architecture documents
   - Check sprint completion reports

### Reporting Issues

When reporting issues, include:
1. Platform (Windows/macOS/Linux)
2. OCCT version
3. Rust version
4. opencascade-rs version
5. Error messages (full output)
6. Steps to reproduce

---

**Document Status:** Active  
**Last Updated:** December 30, 2025  
**Maintained By:** Senior Engineer (Jordan Rivera)  
**For:** Developers and Users

