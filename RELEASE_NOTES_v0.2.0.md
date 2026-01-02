# Release Notes - v0.2.0
## Simple Image Converter

**Release Date:** December 29, 2025  
**Version:** 0.2.0  
**Status:** ✅ Production Release

---

## Overview

v0.2.0 adds **STEP format support** to `mesh-convert`, enabling conversion of CAD files with pre-tessellated geometry. This release implements FACETED_BREP extraction, providing immediate value while establishing the foundation for full curved surface support in v0.3.0.

---

## 🎯 New Features

### STEP Format Support (Read-Only)

Convert STEP files to other mesh formats:

```bash
# Convert STEP to STL (requires --features step)
cargo run --features step --bin mesh-convert -- model.step output.stl

# Convert STEP to OBJ
cargo run --features step --bin mesh-convert -- model.step output.obj

# Convert STEP to PLY
cargo run --features step --bin mesh-convert -- model.step output.ply
```

**Features:**
- ✅ FACETED_BREP entity extraction (pre-tessellated geometry)
- ✅ Direct mesh construction from AP203 entities
- ✅ Vertex deduplication
- ✅ Face triangulation
- ✅ Normal calculation
- ✅ Comprehensive error handling
- ✅ Resource limits and security validation

**Requirements:**
- STEP files must be exported with **tessellation enabled**
- Files must contain **FACETED_BREP** entities
- Feature flag required: `--features step`

---

## 📋 Limitations

### FACETED_BREP Only (v0.2.0)

**What's Supported:**
- ✅ Pre-tessellated STEP files (FACETED_BREP entities)
- ✅ Files exported with tessellation enabled
- ✅ Simple and complex geometries (as long as tessellated)

**What's Not Supported:**
- ❌ Curved surfaces (NURBS, cylinders, spheres)
- ❌ Files with MANIFOLD_SOLID_BREP but no FACETED_BREP
- ❌ STEP writing (read-only support)

**Solution:**
Export your STEP files with tessellation enabled. See `docs/CAD_EXPORT_GUIDE.md` for CAD software-specific instructions.

### Feature-Gated

STEP support requires the `step` feature flag:

```bash
# Build with STEP support
cargo build --features step

# Build without STEP support (default)
cargo build
```

---

## 🔧 Usage Examples

### Basic Conversion

```bash
# Convert STEP to STL
cargo run --features step --bin mesh-convert -- model.step output.stl

# Convert STEP to OBJ
cargo run --features step --bin mesh-convert -- model.step output.obj
```

### Combined with Other Features

```bash
# Convert STEP to OBJ with coordinate transform
cargo run --features step --bin mesh-convert -- \
  model.step output.obj \
  --transform y-up

# Convert STEP to STL with validation
cargo run --features step --bin mesh-convert -- \
  model.step output.stl \
  --validate
```

### Error Handling

If your STEP file doesn't contain FACETED_BREP entities, you'll get a helpful error message:

```
STEP file contains MANIFOLD_SOLID_BREP or CLOSED_SHELL entities, but no FACETED_BREP entities.
For v0.2.0, only FACETED_BREP (pre-tessellated) geometry is supported.

Your file likely contains curved surfaces (NURBS, cylinders, spheres, etc.) which require
full B-Rep support (planned for v0.3.0).

SOLUTION: Please export your STEP file with tessellation enabled to create FACETED_BREP entities.
See docs/CAD_EXPORT_GUIDE.md for CAD software-specific instructions.
```

---

## 📚 Documentation

### User Documentation

- **`docs/CAD_EXPORT_GUIDE.md`** - How to export STEP files with tessellation
  - SolidWorks instructions
  - FreeCAD instructions
  - Fusion 360 instructions
  - Other CAD software

- **`docs/FORMATS.md`** - Format support details and limitations

### Developer Documentation

- **`docs/STEP_FORMAT_REFERENCE.md`** - Technical reference for STEP format
- **`docs/RUSTSTEP_GUIDANCE.md`** - Developer guide for ruststep API
- **`ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md`** - Architecture decisions

---

## 🔒 Security

### Security Measures

- ✅ File size validation before parsing (100MB default limit)
- ✅ Mesh resource validation after extraction (10M vertices/faces default)
- ✅ UTF-8 encoding validation
- ✅ Security event logging for limit violations
- ✅ Comprehensive input validation
- ✅ Zero unsafe code blocks

### Security Review

- **Reviewed by:** Casey Morgan (Security Specialist)
- **Security Grade:** A (Strong - Production Ready)
- **Secure by Design:** 10/10 principles met
- **Status:** ✅ Approved for release

---

## 🏗️ Architecture

### Implementation Approach

v0.2.0 implements a **hybrid phased approach**:

1. **v0.2.0:** FACETED_BREP extraction (Pure Rust) ✅
   - Direct extraction from AP203 entities
   - No C++ dependencies
   - Fast and lightweight

2. **v0.3.0:** opencascade-rs integration (Planned)
   - Full curved surface support
   - Optional feature flag
   - Can coexist with FACETED_BREP path

### Architecture Review

- **Reviewed by:** Alex Chen (System Architect)
- **Status:** ✅ Approved for release
- **Compliance:** Fully compliant with approved architecture

---

## 🧪 Testing

### Test Coverage

- ✅ 8 STEP integration tests (all passing)
- ✅ Error handling tests
- ✅ Conversion tests (STEP → STL, STEP → OBJ)
- ✅ Converter integration tests
- ✅ 370+ total tests passing

### Test Infrastructure

- Test file collection framework
- Verification scripts
- Comprehensive test documentation

---

## 📦 Dependencies

### New Dependencies

- **ruststep** (0.4.0) - STEP file parsing with AP203 feature
  - Pure Rust implementation
  - No C++ dependencies
  - Feature-gated

### Optional Dependencies

- **truck-*** crates - Kept for potential future use (v0.3.0)
  - Currently not used in v0.2.0 implementation
  - May be used for opencascade-rs integration

---

## 🔄 Migration Guide

### From v0.1.1

**No Breaking Changes:**
- All existing functionality remains unchanged
- STEP support is additive (feature-gated)
- Default builds work exactly as before

**To Enable STEP Support:**
```bash
# Build with STEP support
cargo build --features step

# Or use in your code
cargo build --features step --bin mesh-convert
```

---

## 🐛 Known Issues

### STEP Format Limitations

1. **FACETED_BREP Only:**
   - Files without FACETED_BREP entities will fail with helpful error messages
   - Solution: Export with tessellation enabled

2. **Test Files:**
   - Test file collection is ongoing
   - Some manually created test files have format issues
   - Real CAD-exported files work correctly

### Future Enhancements

- Full curved surface support (v0.3.0)
- STEP writing support (future)
- Additional STEP entity types (future)

---

## 🙏 Acknowledgments

### Team Contributions

- **Riley Thompson** (Junior Engineer, 3D Formats)
  - FACETED_BREP extraction implementation
  - Integration tests
  - Comprehensive error handling

- **Sam Parker** (Junior Engineer, 2D Formats)
  - API research and documentation
  - CAD export guide
  - Test file collection framework

- **Jordan Rivera** (Senior Engineer)
  - Implementation review and coordination
  - Release preparation

- **Alex Chen** (System Architect)
  - Architecture review and approval

- **Casey Morgan** (Security Specialist)
  - Security review and approval

---

## 📝 Changelog

See `CHANGELOG.md` for detailed changes.

---

## 🔗 Links

- **Documentation:** `docs/`
- **CAD Export Guide:** `docs/CAD_EXPORT_GUIDE.md`
- **Format Reference:** `docs/FORMATS.md`
- **Architecture Review:** `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md`
- **Security Review:** `SECURITY_REVIEW_V0.2.0.md`

---

## 🚀 Next Steps

### For Users

1. Try converting STEP files with tessellation enabled
2. Report any issues or limitations
3. Provide feedback on error messages

### For Developers

1. Review `docs/RUSTSTEP_GUIDANCE.md` for API details
2. Contribute test files with FACETED_BREP entities
3. Help with v0.3.0 opencascade-rs integration research

---

**Release Prepared By:** Jordan Rivera (Senior Engineer)  
**Release Date:** December 29, 2025  
**Status:** ✅ Production Release

---

*Thank you for using Simple Image Converter!*

