# OpenCASCADE Technology (OCCT) Installation Guide
## For SimpleImageConverter opencascade-rs Integration

**Document Type:** Installation Guide  
**Target Audience:** Developers and Users  
**Purpose:** Guide for installing OpenCASCADE Technology (OCCT) required for full STEP B-Rep support  
**Date:** December 30, 2025  
**Status:** Active Documentation

---

## Executive Summary

This guide provides step-by-step instructions for installing OpenCASCADE Technology (OCCT) 7.7+ on Windows, macOS, and Linux. OCCT is required to build and use the `step-opencascade` feature, which enables full STEP B-Rep support with curved surfaces (NURBS, cylinders, spheres, etc.).

**Quick Start:**
- **Windows:** Download installer from https://dev.opencascade.org/release
- **macOS:** `brew install opencascade`
- **Linux (Ubuntu/Debian):** `sudo apt-get install libocct-*-dev`

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Windows Installation](#windows-installation)
3. [macOS Installation](#macos-installation)
4. [Linux Installation](#linux-installation)
5. [Verification](#verification)
6. [Troubleshooting](#troubleshooting)
7. [Build Configuration](#build-configuration)
8. [Testing the Installation](#testing-the-installation)

---

## Prerequisites

### System Requirements

**OCCT Version:** 7.7.0 or later (recommended: latest stable release)

**Build Tools Required:**
- **CMake:** 3.18 or later
- **C++ Compiler:** C++17 support required
  - **Windows:** Visual Studio 2019+ (MSVC) or MinGW-w64
  - **macOS:** Xcode Command Line Tools (Clang)
  - **Linux:** GCC 7+ or Clang 5+

**Platform Libraries:**
- **Linux:** X11, OpenGL development libraries
- **macOS:** OpenGL (included with Xcode)
- **Windows:** OpenGL (included with Visual Studio)

### Checking Prerequisites

**CMake:**
```bash
cmake --version
# Should show 3.18 or later
```

**C++ Compiler:**
```bash
# Windows (MSVC)
cl
# macOS/Linux
g++ --version
# or
clang++ --version
```

---

## Windows Installation

### Option 1: Pre-built Installer (Recommended)

**Steps:**
1. Download OCCT installer from https://dev.opencascade.org/release
2. Run the installer (typically installs to `C:\OpenCASCADE-7.7.0`)
3. Add OCCT to system PATH:
   - Open "System Properties" → "Environment Variables"
   - Add `C:\OpenCASCADE-7.7.0\bin` to PATH
   - Add `C:\OpenCASCADE-7.7.0\lib` to PATH (if needed)
4. Set environment variable (optional, for build scripts):
   - `OCCT_DIR=C:\OpenCASCADE-7.7.0`

**Verification:**
```powershell
# Check if OCCT libraries are accessible
dir C:\OpenCASCADE-7.7.0\lib
```

### Option 2: Build from Source

**Steps:**
1. Download OCCT source from https://dev.opencascade.org/download
2. Extract to a directory (e.g., `C:\occt-source`)
3. Create build directory:
   ```powershell
   mkdir C:\occt-build
   cd C:\occt-build
   ```
4. Configure with CMake:
   ```powershell
   cmake -G "Visual Studio 16 2019" -A x64 `
     -DINSTALL_DIR=C:\OpenCASCADE-7.7.0 `
     C:\occt-source
   ```
5. Build:
   ```powershell
   cmake --build . --config Release
   ```
6. Install:
   ```powershell
   cmake --install . --config Release
   ```

**Build Time:** 30-60 minutes (depending on CPU)

---

## macOS Installation

### Option 1: Homebrew (Recommended)

**Steps:**
```bash
# Install OCCT via Homebrew
brew install opencascade

# Verify installation
brew list opencascade
```

**Installation Location:**
- Libraries: `/opt/homebrew/lib` (Apple Silicon) or `/usr/local/lib` (Intel)
- Headers: `/opt/homebrew/include/opencascade` (Apple Silicon) or `/usr/local/include/opencascade` (Intel)

### Option 2: Build from Source

**Steps:**
1. Download OCCT source from https://dev.opencascade.org/download
2. Extract to a directory (e.g., `~/occt-source`)
3. Create build directory:
   ```bash
   mkdir ~/occt-build
   cd ~/occt-build
   ```
4. Configure with CMake:
   ```bash
   cmake -DINSTALL_DIR=/usr/local/occt ~/occt-source
   ```
5. Build:
   ```bash
   make -j$(sysctl -n hw.ncpu)
   ```
6. Install:
   ```bash
   sudo make install
   ```

**Build Time:** 30-60 minutes (depending on CPU)

---

## Linux Installation

### Option 1: Package Manager (Recommended)

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libocct-*-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install opencascade-devel
```

**Arch Linux:**
```bash
sudo pacman -S opencascade
```

**Installation Location:**
- Libraries: `/usr/lib` or `/usr/lib/x86_64-linux-gnu`
- Headers: `/usr/include/opencascade`

### Option 2: Build from Source

**Steps:**
1. Download OCCT source from https://dev.opencascade.org/download
2. Extract to a directory (e.g., `~/occt-source`)
3. Install dependencies:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install build-essential cmake libx11-dev libgl1-mesa-dev
   ```
4. Create build directory:
   ```bash
   mkdir ~/occt-build
   cd ~/occt-build
   ```
5. Configure with CMake:
   ```bash
   cmake -DINSTALL_DIR=/usr/local/occt ~/occt-source
   ```
6. Build:
   ```bash
   make -j$(nproc)
   ```
7. Install:
   ```bash
   sudo make install
   ```

**Build Time:** 30-60 minutes (depending on CPU)

---

## Verification

### Check OCCT Installation

**Windows:**
```powershell
# Check if OCCT libraries exist
dir C:\OpenCASCADE-7.7.0\lib\*.lib
```

**macOS/Linux:**
```bash
# Check if OCCT libraries exist
ls /usr/local/lib/libTK*.dylib  # macOS
ls /usr/lib/libTK*.so            # Linux

# Check pkg-config (if available)
pkg-config --modversion opencascade
```

### Verify opencascade-rs Can Find OCCT

**Test Build:**
```bash
# Navigate to project root
cd SimpleImageConverter

# Try building with step-opencascade feature
cargo build --features step-opencascade

# If successful, OCCT is properly installed and configured
```

**Expected Output:**
- Build should complete without errors
- If OCCT not found, you'll see linker errors or `opencascade-sys` build failures

---

## Troubleshooting

### Common Issues

#### Issue 1: opencascade-sys Cannot Find OCCT

**Symptoms:**
- Build fails with "OCCT not found" errors
- Linker errors about missing OCCT libraries

**Solutions:**

**Windows:**
1. Verify OCCT is installed: `dir C:\OpenCASCADE-7.7.0\lib`
2. Set environment variable: `set OCCT_DIR=C:\OpenCASCADE-7.7.0`
3. Add to PATH: `set PATH=%PATH%;C:\OpenCASCADE-7.7.0\bin`

**macOS:**
1. Verify Homebrew installation: `brew list opencascade`
2. Set environment variable: `export OCCT_DIR=/opt/homebrew` (Apple Silicon) or `/usr/local` (Intel)
3. Set library path: `export DYLD_LIBRARY_PATH=/opt/homebrew/lib` (Apple Silicon)

**Linux:**
1. Verify package installation: `dpkg -l | grep occt`
2. Set environment variable: `export OCCT_DIR=/usr`
3. Set library path: `export LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu`

#### Issue 2: Build Fails with C++ Compiler Errors

**Symptoms:**
- C++ compilation errors in opencascade-sys
- "C++17 required" errors

**Solutions:**
1. Verify C++ compiler version:
   ```bash
   g++ --version  # Should be GCC 7+ or Clang 5+
   ```
2. Set C++ standard explicitly:
   ```bash
   export CXXFLAGS="-std=c++17"
   cargo build --features step-opencascade
   ```

#### Issue 3: Runtime Library Not Found

**Symptoms:**
- Build succeeds but program fails at runtime
- "Cannot find libTK*.so" or "Cannot find libTK*.dylib" errors

**Solutions:**

**Windows:**
- Ensure `C:\OpenCASCADE-7.7.0\bin` is in PATH

**macOS:**
```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/lib:$DYLD_LIBRARY_PATH
```

**Linux:**
```bash
export LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH
# Or add to /etc/ld.so.conf.d/opencascade.conf and run sudo ldconfig
```

#### Issue 4: CMake Not Found

**Symptoms:**
- Build fails with "CMake not found" errors

**Solutions:**
1. Install CMake:
   - **Windows:** Download from https://cmake.org/download/
   - **macOS:** `brew install cmake`
   - **Linux:** `sudo apt-get install cmake`
2. Verify installation: `cmake --version`

---

## Build Configuration

### Environment Variables

**OCCT_DIR (Optional):**
- **Purpose:** Tell opencascade-sys where to find OCCT
- **Windows:** `set OCCT_DIR=C:\OpenCASCADE-7.7.0`
- **macOS:** `export OCCT_DIR=/opt/homebrew` (Apple Silicon) or `/usr/local` (Intel)
- **Linux:** `export OCCT_DIR=/usr`

**PKG_CONFIG_PATH (Linux/macOS):**
- **Purpose:** Help pkg-config find OCCT
- **macOS:** `export PKG_CONFIG_PATH=/opt/homebrew/lib/pkgconfig:$PKG_CONFIG_PATH`
- **Linux:** Usually not needed if installed via package manager

### Cargo Build Configuration

**Build with step-opencascade feature:**
```bash
cargo build --features step-opencascade
```

**Build for release:**
```bash
cargo build --release --features step-opencascade
```

**Test with step-opencascade:**
```bash
cargo test --features step-opencascade
```

---

## Testing the Installation

### Test 1: Build opencascade-sys

```bash
# This will build opencascade-sys and verify OCCT is accessible
cargo build --features step-opencascade
```

**Expected:** Build completes successfully

### Test 2: Run Unit Tests

```bash
# Run tests for step_opencascade module
cargo test --features step-opencascade step_opencascade
```

**Expected:** Tests pass (may require sample STEP files)

### Test 3: Integration Test

```bash
# Run integration tests (if available)
cargo test --features step-opencascade --test integration_step_opencascade
```

**Expected:** Integration tests pass with sample STEP files

---

## Additional Resources

### Official Documentation

- **OCCT Documentation:** https://dev.opencascade.org/doc/refman/html/
- **OCCT User Guide:** https://dev.opencascade.org/doc/overview/html/
- **opencascade-rs Repository:** https://github.com/bschwind/opencascade-rs

### Project Documentation

- **Research Document:** `RESEARCH_OPENCASCADE_RS_SPRINT9.md`
- **Architecture Decision:** `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md`
- **STEP Format Reference:** `docs/STEP_FORMAT_REFERENCE.md`

### Support

If you encounter issues not covered in this guide:
1. Check the troubleshooting section above
2. Review `RESEARCH_OPENCASCADE_RS_SPRINT9.md` for technical details
3. Check opencascade-rs repository for known issues
4. Verify OCCT installation with official OCCT documentation

---

## Summary

**Quick Installation Commands:**

**Windows:**
1. Download installer from https://dev.opencascade.org/release
2. Run installer
3. Add `C:\OpenCASCADE-7.7.0\bin` to PATH

**macOS:**
```bash
brew install opencascade
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install libocct-*-dev
```

**Verify Installation:**
```bash
cargo build --features step-opencascade
```

---

**Document Status:** Active  
**Last Updated:** December 30, 2025  
**Maintained By:** Senior Engineer (Jordan Rivera)  
**For:** Developers and Users

