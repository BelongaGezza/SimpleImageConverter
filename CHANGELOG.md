# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned for v0.1.1
- mesh-convert transform, recalculate-normals, and validate features
- CLI integration tests
- Additional bug fixes and improvements

---

## [0.1.0] - 2025-01-27

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
