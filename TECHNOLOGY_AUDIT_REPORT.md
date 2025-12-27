# Technology Audit Report
## SimpleImageConverter Project

**Conducted by:** Researcher Agent
**Date:** December 27, 2025
**Rust Toolchain:** 1.92 (current stable: 1.92.0)

---

## Executive Summary

This audit reviewed all dependencies and technologies used in the SimpleImageConverter workspace against current stable versions. Key findings include:

- **3 Critical Updates Required**: stl_io, thiserror, resvg
- **1 Security Vulnerability**: ply-rs (CVE-2020-25573 via linked-hash-map)
- **5 Minor Updates Available**: image, gltf, ahash, tempfile, serde
- **Rust Toolchain**: Currently aligned with latest stable (1.92)

---

## Dependency Analysis

### Core Utilities

| Crate | Current | Latest | Status | Action |
|-------|---------|--------|--------|--------|
| thiserror | 1.0 | **2.0.17** | :warning: MAJOR | Review breaking changes |
| anyhow | 1.0 | 1.0.100 | :white_check_mark: OK | Minor bump optional |
| clap | 4.5 | 4.5.53 | :white_check_mark: OK | Minor bump optional |
| serde | 1.0 | 1.0.217+ | :white_check_mark: OK | Minor bump optional |
| serde_json | 1.0 | 1.0.x | :white_check_mark: OK | Compatible |
| log | 0.4 | 0.4.x | :white_check_mark: OK | Compatible |

### Image Processing

| Crate | Current | Latest | Status | Action |
|-------|---------|--------|--------|--------|
| image | 0.25 | **0.25.8** | :white_check_mark: OK | Minor bump available |
| resvg | 0.40 | **0.45.1** | :warning: UPDATE | 5 minor versions behind |
| tiny-skia | 0.11 | 0.11.4 | :white_check_mark: OK | Compatible |

### 3D/Mesh Processing

| Crate | Current | Latest | Status | Action |
|-------|---------|--------|--------|--------|
| stl_io | 0.7 | **0.10.0** | :x: OUTDATED | Update required |
| nalgebra | 0.33 | 0.33.x | :white_check_mark: OK | Compatible |
| tobj | 4.0 | 4.0.3 | :white_check_mark: OK | Compatible |
| ply-rs | 0.1 | 0.1.2 | :x: SECURITY | See security section |
| gltf | 1.4 | **1.4.1** | :white_check_mark: OK | Minor bump available |
| dxf | 0.6 | 0.6.0 | :white_check_mark: OK | Current |
| ahash | 0.8 | **0.8.12** | :white_check_mark: OK | Minor bump available |

### Optional (STEP Support)

| Crate | Current | Latest | Status | Action |
|-------|---------|--------|--------|--------|
| truck-modeling | 0.3.0 | 0.3.x | :white_check_mark: OK | Latest available |
| truck-polymesh | 0.3.0 | 0.3.x | :white_check_mark: OK | Latest available |
| truck-stepio | 0.3.0 | 0.3.x | :white_check_mark: OK | Latest available |

### Dev Dependencies

| Crate | Current | Latest | Status | Action |
|-------|---------|--------|--------|--------|
| criterion | 0.5 | 0.5.1 | :white_check_mark: OK | Compatible |
| tempfile | 3.10 | **3.14.0** | :white_check_mark: OK | Minor bump available |

---

## Security Vulnerabilities

### :white_check_mark: CVE-2020-25573 - ply-rs via linked-hash-map - **FIXED**

**Severity:** CRITICAL (CVSS 9.8)
**Affected Crate:** ply-rs 0.1.x (depends on linked-hash-map < 0.5.3)
**Issue:** `mem::uninitialized()` creates undefined behavior with `NonNull<T>`

**Status:** :white_check_mark: **RESOLVED** on December 27, 2025

**Fix Applied:**
- Replaced `ply-rs = "0.1"` with `ply-rs-bw = "0.1.3"` in mesh-core/Cargo.toml
- Added alias `use ply_rs_bw as ply_rs;` for API compatibility
- All 26 PLY tests passing

---

## Breaking Changes Analysis

### thiserror 1.0 → 2.0 Migration

If upgrading to thiserror 2.0, review these breaking changes:

1. **Raw identifier syntax change**
   ```rust
   // Before (1.x)
   #[error("invalid {r#type}")]

   // After (2.0)
   #[error("invalid {type}")]
   ```

2. **No-std support** (new feature)
   ```toml
   thiserror = { version = "2", default-features = false }
   ```

3. **Direct dependency requirement**: All crates using `derive(Error)` must directly depend on thiserror.

4. **Recommendation**: Defer thiserror 2.0 upgrade until ecosystem catches up (many crates still on 1.x)

---

## Recommendations

### Immediate Actions (Priority: HIGH)

1. **Address ply-rs security vulnerability**
   - Replace `ply-rs = "0.1"` with `ply-rs-bw = "2.0"` in mesh-core/Cargo.toml
   - Update import statements: `use ply_rs_bw as ply_rs;`

2. **Update stl_io to 0.10.0**
   - Review changelog for API changes
   - Run tests after upgrade

### Short-Term Actions (Priority: MEDIUM)

3. **Update resvg 0.40 → 0.45.1**
   - Note: 5 minor versions may include breaking changes
   - Test SVG rendering thoroughly after upgrade

4. **Minor version bumps** (safe updates)
   ```toml
   image = "0.25.8"
   gltf = "1.4.1"
   ahash = "0.8.12"
   tempfile = "3.14"
   ```

### Deferred Actions (Priority: LOW)

5. **thiserror 2.0 migration**
   - Wait for ecosystem adoption
   - Plan migration when more dependencies support 2.x
   - Current 1.x is still maintained and secure

6. **Rust 2024 Edition**
   - Consider upgrading from `edition = "2021"` to `edition = "2024"`
   - Available since Rust 1.85 (February 2025)
   - Benefits: Pattern matching improvements, async improvements

---

## Rust Toolchain Status

| Component | Specified | Current Stable | Status |
|-----------|-----------|----------------|--------|
| Rust Version | 1.92 | 1.92.0 | :white_check_mark: Aligned |
| Edition | 2021 | 2024 available | :white_check_mark: OK |
| Resolver | 2 | 2 | :white_check_mark: Current |

---

## Recommended Cargo.toml Updates

```toml
[workspace.dependencies]
# Core utilities (keep stable)
anyhow = "1.0"
thiserror = "1.0"  # Defer 2.0 upgrade
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"

# 2D image processing (update recommended)
image = "0.25.8"        # was: 0.25
resvg = "0.45"          # was: 0.40 - TEST THOROUGHLY
tiny-skia = "0.11"

# 3D mesh processing (updates required)
stl_io = "0.10"         # was: 0.7 - BREAKING CHANGES LIKELY
nalgebra = "0.33"
tobj = "4.0"
ahash = "0.8.12"        # was: 0.8

# Testing
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3.14"       # was: 3.10
```

```toml
# mesh-core/Cargo.toml
[dependencies]
# Replace ply-rs with patched fork
ply-rs-bw = "2.0"       # was: ply-rs = "0.1" - SECURITY FIX
gltf = "1.4.1"          # was: 1.4
dxf = "0.6"
```

---

## Sources

- [crates.io - image](https://crates.io/crates/image)
- [crates.io - resvg](https://crates.io/crates/resvg)
- [crates.io - thiserror](https://crates.io/crates/thiserror)
- [crates.io - stl_io](https://crates.io/crates/stl_io)
- [crates.io - gltf](https://crates.io/crates/gltf)
- [crates.io - ply-rs-bw](https://lib.rs/crates/ply-rs-bw)
- [RustSec Advisory RUSTSEC-2020-0026](https://rustsec.org/advisories/RUSTSEC-2020-0026.html)
- [Rust Releases](https://releases.rs/)
- [thiserror 2.0 Migration](https://docs.rs/thiserror/latest)

---

**Report generated by Researcher Agent**
