# Release Announcement - v0.2.0
## Simple Image Converter - STEP Format Support

**Release Date:** January 29, 2025  
**Version:** 0.2.0  
**Status:** ✅ **RELEASED**

---

## 🎉 v0.2.0 Released!

We're excited to announce the release of **v0.2.0** with **STEP format support**!

---

## What's New

### STEP Format Support (Read-Only)

Convert STEP files (CAD format) to other mesh formats:

```bash
# Convert STEP to STL (requires --features step)
cargo run --features step --bin mesh-convert -- model.step output.stl

# Convert STEP to OBJ
cargo run --features step --bin mesh-convert -- model.step output.obj
```

**Features:**
- ✅ FACETED_BREP entity extraction (pre-tessellated geometry)
- ✅ Direct mesh construction from AP203 entities
- ✅ Vertex deduplication
- ✅ Face triangulation
- ✅ Normal calculation
- ✅ Comprehensive error handling
- ✅ Resource limits and security validation

---

## Important Notes

### Requirements

- **Feature Flag:** STEP support requires `--features step` flag
- **File Format:** Files must contain FACETED_BREP entities
- **Export Settings:** Files must be exported with tessellation enabled

### Limitations

- **FACETED_BREP Only:** v0.2.0 supports only pre-tessellated STEP files
  - No support for curved surfaces (NURBS, cylinders, spheres)
  - Full B-Rep support planned for v0.3.0
- **Read-Only:** STEP writing not supported
- **Feature-Gated:** Requires `--features step` to build

### Solution

Export your STEP files with tessellation enabled. See `docs/CAD_EXPORT_GUIDE.md` for CAD software-specific instructions.

---

## Documentation

### User Guides
- **`docs/CAD_EXPORT_GUIDE.md`** - How to export STEP files with tessellation
- **`docs/FORMATS.md`** - Format support details and limitations
- **`RELEASE_NOTES_v0.2.0.md`** - Complete release notes

### Developer Guides
- **`docs/STEP_FORMAT_REFERENCE.md`** - Technical reference
- **`docs/RUSTSTEP_GUIDANCE.md`** - Developer guide for ruststep API

---

## Security

- **Security Grade:** A (Strong - Production Ready)
- **Secure by Design:** 10/10 principles met
- **Zero unsafe code blocks**
- **Comprehensive input validation**
- **Resource limits enforced**

---

## Testing

- **370+ tests passing**
- **8 STEP integration tests (all passing)**
- **Comprehensive error handling validated**
- **Conversion tests implemented**

---

## Reviews

All required reviews completed and approved:
- ✅ **System Architect:** Architecture review approved
- ✅ **Security Specialist:** Security review approved (Grade: A)
- ✅ **Senior Engineer:** Implementation review approved

---

## Team Acknowledgments

**Implementation:**
- **Riley Thompson** - FACETED_BREP extraction implementation
- **Sam Parker** - Documentation and research

**Reviews:**
- **Alex Chen** - Architecture review
- **Casey Morgan** - Security review
- **Jordan Rivera** - Release coordination

---

## What's Next

### v0.3.0 (Planned)
- Full curved surface support via opencascade-rs
- Support for NURBS, cylinders, spheres
- Optional feature flag for OCCT integration

### Ongoing
- Test file collection (incremental)
- User feedback collection
- Performance optimization

---

## Download & Installation

**Build with STEP support:**
```bash
cargo build --release --features step
```

**Binaries:**
- `target/release/img-convert.exe` - Image converter
- `target/release/mesh-convert.exe` - Mesh converter (with STEP support)

---

## Links

- **Release Notes:** `RELEASE_NOTES_v0.2.0.md`
- **Changelog:** `CHANGELOG.md`
- **CAD Export Guide:** `docs/CAD_EXPORT_GUIDE.md`
- **Format Reference:** `docs/FORMATS.md`

---

## Support

For issues, questions, or feedback:
- Check `docs/CAD_EXPORT_GUIDE.md` for STEP file export instructions
- Review error messages (they include solutions)
- See `docs/FORMATS.md` for format limitations

---

**Release Date:** January 29, 2025  
**Version:** 0.2.0  
**Status:** ✅ **RELEASED**

---

*Thank you for using Simple Image Converter!*

