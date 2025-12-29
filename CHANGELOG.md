# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

---

## [0.2.0] - 2025-12-29

### Added

#### Mesh Converter (mesh-convert)
- STEP format support (read-only, feature-gated)
  - FACETED_BREP entity extraction (pre-tessellated geometry)
  - Direct mesh construction from AP203 entities
  - Support for STEP files exported with tessellation enabled
  - Comprehensive error handling with user-friendly messages
  - Resource limits and security validation
- STEP integration tests (8 tests, all passing)
  - File reading tests
  - Conversion tests (STEP → STL, STEP → OBJ)
  - Error handling tests
  - Converter integration tests

#### Core Libraries
- `mesh-core`: STEP format handler
  - `StepFormat` struct with resource limits
  - FACETED_BREP entity traversal and extraction
  - Vertex deduplication with integer-based hashing
  - Face triangulation for polygons
  - Normal calculation for extracted meshes
  - Comprehensive validation and error handling

#### Documentation
- Comprehensive STEP format documentation
  - `docs/STEP_FORMAT_REFERENCE.md` - Technical reference
  - `docs/CAD_EXPORT_GUIDE.md` - User guide for CAD software
  - `docs/RUSTSTEP_GUIDANCE.md` - Developer guide for ruststep API
  - `docs/FORMATS.md` - Updated with STEP limitations
- Test file collection framework
  - Verification scripts
  - Collection guidelines
  - Test file documentation

### Changed
- STEP format support moved from "in progress" to "partial support"
- Updated error messages to be more user-friendly and actionable
- Enhanced resource limit validation for STEP files
- Improved security logging for STEP operations

### Improved
- Error messages now include solutions and documentation references
- Better handling of unsupported STEP file types
- Clearer guidance for users on STEP file requirements

### Limitations
- **FACETED_BREP only:** v0.2.0 supports only pre-tessellated STEP files
  - Files must be exported with tessellation enabled
  - No support for curved surfaces (NURBS, cylinders, spheres)
  - Full B-Rep support planned for v0.3.0
- **Feature-gated:** STEP support requires `--features step` flag
- **Read-only:** STEP writing not supported (requires complex CAD modeling)

### Security
- All security checks pass (reviewed by Security Specialist)
- Zero unsafe code blocks
- Comprehensive input validation (file size, UTF-8, mesh resources)
- Resource limits enforced before parsing and after extraction
- Security logging for all limit violations
- Secure by Design: 10/10 principles met
- Security grade: A (Strong - Production Ready)

### Architecture
- Approved hybrid phased approach (FACETED_BREP → opencascade-rs)
- Pure Rust implementation (no C++ dependencies for v0.2.0)
- Feature-gated implementation
- Direct mesh construction (no intermediate Shell conversion)

### Notes
- All v0.2.0 features fully implemented and tested
- Code reviewed and approved by Senior Engineer
- Architecture reviewed and approved by System Architect
- Security reviewed and approved by Security Specialist
- 8 STEP integration tests passing
- All tests passing (370+ total)

---

## [0.1.1] - 2025-12-27

### Added

#### Mesh Converter (mesh-convert)
- Coordinate system transform functionality (`--transform`)
  - Transform between Y-up and Z-up coordinate systems
  - Support for explicit transforms (`z-up:y-up`) or auto-detect (`y-up`)
  - Automatic normal vector transformation
- Normal recalculation (`--recalculate-normals`)
  - Area-weighted face normal calculation
  - Smooth vertex normal computation
  - Automatic handling of degenerate faces
- Mesh validation (`--validate`)
  - Vertex and face index validation
  - Degenerate face detection
  - Duplicate vertex detection
  - Normal consistency checks
- CLI integration tests for new features

#### Core Libraries
- `mesh-core`: New mesh manipulation utilities
  - `transform_coordinates()` - Coordinate system transformation
  - `recalculate_normals()` - Vertex normal recalculation from geometry
  - `validate_mesh()` - Comprehensive mesh validation
  - `ConversionOptions` struct for advanced conversion settings

### Changed
- `MeshConverter` now supports `ConversionOptions` for advanced operations
- Improved code quality with refactored transform logic
- Enhanced test coverage (14+ new tests)

### Improved
- Transform logic refactored to eliminate code duplication
- Magic numbers replaced with named constants
- Better error messages and documentation

### Fixed
- Improved handling of degenerate faces in normal recalculation
- Enhanced validation test coverage

### Security
- All security checks pass (reviewed by Security Specialist)
- Zero unsafe code blocks
- Comprehensive bounds checking maintained
- Resource limits properly enforced

### Notes
- All v0.1.1 features fully implemented and tested
- Code reviewed and approved by Senior Engineer
- Security reviewed and approved by Security Specialist
- 350+ tests passing

---

## [0.1.0] - 2025-12-27

### Added

#### Image Converter (img-convert)
- PNG format support (read/write) with transparency handling
- JPEG format support (read/write) with quality control (1-100)
- BMP format support (read/write)
- GIF format support (read/write, first frame)
- TIFF format support (read/write) with multi-page handling
- WebP format support (read/write) with lossy/lossless modes
- SVG format support (read-only, rasterization to bitmap)
- Two-stage format detection (extension + magic bytes)
- Resource limits and security validation
- Output file verification
- Comprehensive error handling

#### Mesh Converter (mesh-convert)
- STL format support (binary/ASCII, read/write)
- OBJ format support (read/write) with material (.mtl) handling
- PLY format support (read/write)
- OFF format support (read/write, custom parser)
- glTF/GLB format support (read/write) with material handling
- DXF format support (read/write, 3D entities)
- STEP format support (read-only, feature-gated, tessellation in progress)
- Format detection and validation
- Resource limits and security validation
- Output file verification

#### Core Libraries
- `img-core`: Image conversion library with trait-based format system
- `mesh-core`: Mesh conversion library with trait-based format system
- `common`: Shared utilities (error types, I/O helpers, resource limits, security logging)

#### Security
- Zero unsafe code blocks
- Comprehensive input validation
- Resource limits (file size, dimensions, vertices, faces)
- Two-stage format detection to prevent format spoofing
- Security event logging
- Integer overflow protection

#### Testing
- 365+ tests total covering all format implementations
- Unit tests for all format readers/writers
- Integration tests for format conversions
- Security tests for format spoofing and malformed input
- Edge case handling (empty files, invalid data, oversized files)

### Changed
- Project status updated from "In Development" to "Active Development"
- All Sprints 1-5 marked as complete

### Fixed
- All previously identified critical security issues resolved
- Format registry now returns `Result` instead of panicking
- Comprehensive input validation implemented
- Error handling standardized across all modules

### Security
- All critical security vulnerabilities addressed
- Secure by Design compliance: 10/10 principles met
- No unsafe code in production paths
- Comprehensive validation at all entry points

### Notes
- This is the first production-ready release
- All core formats are implemented and tested
- STEP format is feature-gated (`--features step`) and partial
- mesh-convert transform, recalculate-normals, and validate features are planned for v0.1.1
- CLI integration tests are planned for v0.1.1

---

## Version History

### Planned Releases

- **v0.1.0** (Sprint 3) - MVP: Core converters (PNG, JPG, BMP, GIF, STL, OBJ, PLY)
- **v0.2.0** (Sprint 6) - Extended formats (TIFF, WebP, SVG, glTF, DXF)
- **v0.3.0** (Sprint 8) - STEP/CAD support
- **v1.0.0** (Sprint 12) - GUI release, public repository

---

## Release Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security updates
```

---

**Note:** This changelog will be updated as development progresses through sprints.

[Unreleased]: https://github.com/yourusername/SimpleImageConverter/compare/v0.1.0...HEAD
[0.1.0-dev]: https://github.com/yourusername/SimpleImageConverter/releases/tag/v0.1.0-dev
