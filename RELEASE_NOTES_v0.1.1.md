# Release Notes - v0.1.1
## Simple Image Converter

**Release Date:** December 27, 2025  
**Version:** 0.1.1  
**Status:** ✅ Production Release

---

## Overview

v0.1.1 adds advanced mesh manipulation features to `mesh-convert`, including coordinate system transformation, normal recalculation, and mesh validation. This release completes the planned mesh-convert enhancements and includes comprehensive testing and security review.

---

## New Features

### 🎯 Coordinate System Transformation

Transform meshes between different coordinate systems:

```bash
# Auto-detect source (assumes Z-up) and transform to Y-up
mesh-convert model.stl obj --transform y-up

# Explicit transform from Z-up to Y-up
mesh-convert model.stl obj --transform z-up:y-up
```

**Features:**
- Transform between Y-up (OpenGL, glTF) and Z-up (CAD, STL) coordinate systems
- Automatic normal vector transformation and normalization
- Preserves mesh geometry integrity

### 📐 Normal Recalculation

Recalculate vertex normals from face geometry:

```bash
mesh-convert model.stl obj --recalculate-normals
```

**Features:**
- Area-weighted face normal calculation
- Smooth vertex normal computation
- Automatic handling of degenerate faces
- Ensures one normal per vertex

### ✅ Mesh Validation

Validate meshes for common issues:

```bash
mesh-convert model.stl obj --validate
```

**Checks:**
- Vertex and face index validation
- Degenerate face detection
- Duplicate vertex detection
- Normal consistency checks

**Output:**
- Errors prevent conversion
- Warnings are logged but allow conversion to proceed

### 🔧 Combined Operations

All features can be combined:

```bash
mesh-convert model.stl obj \
  --transform y-up \
  --recalculate-normals \
  --validate
```

---

## API Changes

### New Public API

**mesh-core Library:**

```rust
// Coordinate system transformation
pub fn transform_coordinates(
    mesh: Mesh,
    from: CoordinateSystem,
    to: CoordinateSystem,
) -> Result<Mesh>;

pub fn parse_coordinate_system(s: &str) -> Result<CoordinateSystem>;

// Normal recalculation
pub fn recalculate_normals(mesh: Mesh) -> Result<Mesh>;

// Mesh validation
pub fn validate_mesh(mesh: &Mesh) -> Result<()>;

// Conversion options
pub struct ConversionOptions {
    pub transform: Option<(CoordinateSystem, CoordinateSystem)>,
    pub recalculate_normals: bool,
    pub validate: bool,
}
```

### Backward Compatibility

✅ **Fully backward compatible** - All existing code continues to work. New features are opt-in via CLI flags or `ConversionOptions`.

---

## Improvements

### Code Quality
- Refactored transform logic to eliminate duplication
- Replaced magic numbers with named constants
- Improved code maintainability

### Testing
- Added 14+ new unit tests
- CLI integration tests for all new features
- Improved edge case coverage

### Documentation
- Enhanced API documentation
- Usage examples for all new features
- Improved inline comments

---

## Security

✅ **Security Review Complete**

All v0.1.1 code has been reviewed by Security Specialist:
- Zero unsafe code blocks
- Comprehensive input validation
- All array access bounds-checked
- Resource limits properly enforced
- Threat model compliance verified

**Security Grade:** A (Strong - Production Ready)

---

## Testing

### Test Results
- ✅ 350+ tests passing
- ✅ All new features tested
- ✅ Edge cases covered
- ✅ No regressions

### Test Coverage
- Transform: 4 tests
- Normal recalculation: 5 tests (includes degenerate face handling)
- Validation: 5 tests (includes warning scenarios)
- CLI integration: 6 tests

---

## Upgrade Guide

### From v0.1.0

No breaking changes. Simply update your version:

```bash
cargo update
```

Or if building from source:

```bash
git pull
cargo build --release
```

### New CLI Options

The new features are opt-in via CLI flags:
- `--transform <system>` - Transform coordinate system
- `--recalculate-normals` - Recalculate vertex normals
- `--validate` - Validate mesh before conversion

All existing workflows continue to work unchanged.

---

## Bug Fixes

- Improved handling of degenerate faces in normal recalculation
- Enhanced validation error messages
- Fixed test assertion issues in validation tests

---

## Contributors

This release includes code reviewed and approved by:
- **Jordan Rivera** (Senior Engineer) - Code review and improvements
- **Casey Morgan** (Security Specialist) - Security review

---

## Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete details.

---

## Download

- **Source:** [GitHub Repository](https://github.com/BelongaGezza/SimpleImageConverter)
- **Version:** 0.1.1
- **Tag:** `v0.1.1`

---

## Next Steps

### Planned for v0.1.2
- Performance optimizations
- Additional mesh format support
- Enhanced validation reporting

### Planned for v0.2.0
- Extended image format support
- Advanced mesh operations
- GUI development (Sprint 9+)

---

**Release Status:** ✅ Production Ready  
**Compatibility:** Fully backward compatible with v0.1.0  
**Support:** See [CONTRIBUTING.md](CONTRIBUTING.md) for support information

---

*Released: December 27, 2025*

