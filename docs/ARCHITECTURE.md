# Architecture Overview
## Simple Image Converter

**Version:** 0.3.0  
**Last Updated:** December 30, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Workspace Structure](#workspace-structure)
3. [Design Principles](#design-principles)
4. [Module Architecture](#module-architecture)
5. [Data Flow](#data-flow)
6. [Error Handling](#error-handling)
7. [Testing Strategy](#testing-strategy)

---

## Overview

Simple Image Converter is a high-performance Rust toolkit for converting between image and 3D mesh formats. The project consists of two main tools:

- **img-convert**: 2D image format converter
- **mesh-convert**: 3D mesh and CAD format converter

Both tools share a common architecture based on trait-based format handlers, allowing for extensible and maintainable code.

---

## Workspace Structure

```
SimpleImageConverter/
├── common/              # Shared utilities
│   ├── error.rs        # Common error types
│   ├── limits.rs       # Resource limits (security)
│   ├── progress.rs     # Progress reporting
│   ├── validation.rs   # File validation
│   └── io.rs           # I/O helpers
│
├── img-core/           # 2D image library
│   ├── formats/        # Format implementations
│   ├── convert.rs      # Conversion orchestration
│   └── quality.rs      # Quality settings
│
├── img-convert/         # 2D CLI binary
│   └── main.rs         # CLI entry point
│
├── mesh-core/          # 3D mesh library
│   ├── formats/        # Format implementations
│   ├── mesh/           # Mesh data structures
│   └── convert.rs      # Conversion orchestration
│
└── mesh-convert/        # 3D CLI binary
    └── main.rs          # CLI entry point
```

---

## Design Principles

### 1. Library-First Architecture

The binaries (`img-convert` and `mesh-convert`) are thin wrappers around the core libraries (`img-core` and `mesh-core`). This allows:

- Reuse in other projects
- Testing without CLI overhead
- Future GUI integration
- Programmatic API access

### 2. Trait-Based Format System

All formats implement standard traits:

**For 2D Images:**
- `ImageReader`: Read image data from bytes
- `ImageWriter`: Write image data to bytes

**For 3D Meshes:**
- `MeshReader`: Read mesh data from bytes
- `MeshWriter`: Write mesh data to bytes

This design enables:
- Easy addition of new formats
- Consistent API across formats
- Format-agnostic conversion logic

### 3. Zero-Copy Where Possible

The architecture minimizes unnecessary data copying:

- Direct byte slice operations
- In-place transformations when possible
- Streaming I/O for large files (future)

### 4. Comprehensive Error Handling

All operations return `Result<T, ConversionError>`:

- Clear error messages
- Context preservation
- Chainable error handling

### 5. Extensive Testing

- Unit tests for each module
- Integration tests for conversions
- CLI tests
- Performance benchmarks

### 6. Security-First Design

All external input is treated as untrusted:

- **Resource Limits**: File size, image dimensions, mesh vertices/faces
- **Input Validation**: Magic bytes, format verification, path validation
- **Integer Safety**: Checked arithmetic for all size calculations
- **Error Sanitization**: No sensitive data in user-facing error messages
- **No Unsafe Code**: Pure safe Rust (unless documented justification)

**Key Security Components:**
- `common::limits::ResourceLimits` - Centralized limit configuration
- `common::io::read_file_bytes_checked()` - Size-validated file reading
- `FormatRegistry::verify_format()` - Magic byte validation

---

## Module Architecture

### Common Module

Provides shared functionality:

- **Error Types**: `ConversionError`, `Result<T>`
- **Resource Limits**: `ResourceLimits` - configurable security limits
- **Progress Reporting**: `ProgressReporter` trait
- **Validation**: File path, format, and security validation
- **I/O Helpers**: Size-validated file reading/writing utilities

### Image Core Module

**Formats:**
- Trait definitions (`ImageReader`, `ImageWriter`)
- Format implementations (PNG, JPEG, BMP, GIF, etc.)
- Format registry for auto-detection

**Conversion:**
- `ImageConverter`: Orchestrates format conversion
- Quality settings management
- Color space handling
- Metadata preservation

### Mesh Core Module

**Formats:**
- Trait definitions (`MeshReader`, `MeshWriter`)
- Format implementations (STL, OBJ, PLY, etc.)
- Format registry for auto-detection

**Mesh Data:**
- `Mesh`: Main mesh structure
- `Vertex`, `Face`, `Normal`: Geometric primitives
- Transform utilities

**Conversion:**
- `MeshConverter`: Orchestrates format conversion
- Coordinate system transforms
- Normal recalculation
- Mesh validation

---

## Data Flow

### Image Conversion Flow

```
Input File
    ↓
[File Size Validation] ← ResourceLimits
    ↓
[Format Detection + Magic Bytes]
    ↓
[Dimension Validation] ← ResourceLimits
    ↓
[ImageReader.read()] → ImageData
    ↓
[ImageConverter.convert()]
    ↓
[ImageWriter.write()] → Vec<u8>
    ↓
Output File
```

### Mesh Conversion Flow

```
Input File
    ↓
[File Size Validation] ← ResourceLimits
    ↓
[Format Detection]
    ↓
[MeshReader.read()] → Mesh
    ↓
[Resource Validation] ← ResourceLimits (vertices, faces)
    ↓
[MeshConverter.convert()]
    ↓
[Optional: Transforms/Validation]
    ↓
[MeshWriter.write()] → Vec<u8>
    ↓
Output File
```

### STEP Conversion Flow (v0.2.0 - FACETED_BREP)

```
STEP File (ASCII)
    ↓
[File Size Validation] ← ResourceLimits
    ↓
[Format Detection] → STEP format
    ↓
[ruststep::parser::parse()] → Exchange
    ↓
[Tables::from_data_sections()] → Tables (AP203 entities)
    ↓
[Entity Extraction] → FACETED_BREP entities
    ↓
[Entity Traversal] → Extract vertices/faces from pre-tessellated geometry
    ↓
[Calculate Normals] → Face normals from vertex positions
    ↓
[Mesh { vertices, faces, normals }]
    ↓
[Resource Validation] ← ResourceLimits (vertices, faces)
    ↓
[MeshConverter.convert()] (if needed)
    ↓
[MeshWriter.write()] → Vec<u8>
    ↓
Output File
```

**Note:** For STEP files with curved surfaces, see v1.1.0 opencascade-rs integration plan (deferred from v0.3.0).

---

## Error Handling

### Error Types

```rust
pub enum ConversionError {
    Io(std::io::Error),
    InvalidFormat(String),
    UnsupportedFormat(String),
    ConversionFailed(String),
    ValidationFailed(String),
    InvalidInput(String),
    ResourceLimitExceeded(String),  // Security: resource limits
}
```

### Error Propagation

Errors are propagated using `?` operator and `Result<T>` types. Context is preserved through error messages and error chaining.

### User-Friendly Error Messages (v0.3.0)

The GUI application includes a comprehensive error message system (`converter-gui/src/error_messages.rs`) that converts technical errors into user-friendly, actionable messages:

**Key Features:**
- **User-friendly language** - No technical jargon, clear explanations
- **Actionable guidance** - Each message includes specific suggestions for resolution
- **Context-aware** - Messages adapt based on error type and content
- **Consistent format** - All messages follow the same structure

**Error Message Mapping:**

The `format_user_message()` function maps `ConversionError` variants to user-friendly messages:
- Pattern matching extracts safe information from error messages
- I/O errors are mapped to specific, actionable messages
- Resource limit errors include specific limits and suggestions
- Format errors provide guidance on supported formats

**Example:**
```rust
// Technical error
ConversionError::ResourceLimitExceeded("Image dimension 100000 exceeds limit")

// User-friendly message
"Image dimensions exceed limit (65,535 pixels). Please use a smaller image or resize it before converting."
```

### Error Sanitization

User-facing error messages are sanitized to prevent information disclosure (security-reviewed in Task 2.2):

**Path Sanitization:**
- Full paths are replaced with filenames only using `sanitize_path()`
- User directories and usernames are never exposed
- System paths are never revealed
- Only filenames are shown in error messages

**Information Minimization:**
- Only necessary information is displayed
- No technical system details exposed
- No internal error codes or stack traces shown
- Pattern matching prevents raw error string exposure

**Security Review:**
- ✅ Security review completed (see `SECURITY_REVIEW_TASK_2.2.md`)
- ✅ Approved for production use
- ✅ No critical or high-severity issues found
- ✅ Defense in depth with multiple sanitization layers

**Implementation:**
- `common/src/validation.rs` - `sanitize_path()` function ensures path sanitization
- `converter-gui/src/error_messages.rs` - User-friendly message formatting
- All validation functions use `sanitize_path()` when creating errors

---

## Testing Strategy

### Unit Tests

Each module has unit tests:
- Format readers/writers
- Data structure operations
- Utility functions
- Validation functions

### Integration Tests

End-to-end conversion tests:
- Format-to-format conversions
- Round-trip tests
- Edge case handling

### Security Tests

Security-focused test cases:
- Malformed file headers (reject gracefully)
- Oversized files (reject with clear error)
- Extreme dimensions (reject with clear error)
- Path traversal attempts (reject)
- Integer overflow conditions (handle safely)
- Format spoofing (detect and reject)

### CLI Tests

Test the command-line interface:
- Argument parsing
- Error messages
- Output validation
- Quality parameter validation

### Benchmarks

Performance benchmarks for:
- Conversion speed
- Memory usage
- Large file handling

---

## STEP Format Implementation

### Current Status (v0.2.0)

**Implementation Approach:** Hybrid phased strategy

**v0.2.0: FACETED_BREP Extraction (Pure Rust)** ✅ COMPLETE (Released December 29, 2025)
- ✅ STEP file parsing (ruststep 0.4.0 with AP203 feature)
- ✅ Entity extraction framework (Tables, IntoOwned)
- ✅ Entity type identification (MANIFOLD_SOLID_BREP, CLOSED_SHELL, FACETED_BREP)
- ✅ FACETED_BREP entity traversal and mesh extraction
- ✅ Direct mesh construction from pre-tessellated geometry
- ✅ Integration tests (8 tests, all passing)

**Architecture:**
```
STEP File → ruststep::parser → Exchange → Tables::from_data_sections()
  → FACETED_BREP entities → Entity traversal → Extract vertices/faces
  → Mesh { vertices, faces, normals }
```

**Limitations:**
- Only supports STEP files with pre-tessellated geometry (FACETED_BREP)
- No support for curved surfaces (NURBS, cylinders, spheres)
- Many CAD tools can export FACETED_BREP format

**v1.1.0: opencascade-rs Integration (Planned - Deferred from v0.3.0)**
- Full curved surface support via OpenCASCADE kernel
- Feature-gated optional dependency
- Can coexist with FACETED_BREP path
- See `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md` for details

**Feature Flags:**
- `step`: Pure Rust STEP support (FACETED_BREP) - v0.2.0
- `step-opencascade`: Full STEP support with OCCT - v0.3.0 (planned)

**Architectural Decision:**
See `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md` for complete architecture decision record.

---

## Future Enhancements

### ✅ Completed (Phase 2-3)
- Advanced formats (SVG, WebP, glTF, DXF, OFF) - ✅ Implemented
- Quality control - ✅ Implemented
- STEP format parsing and entity extraction - ✅ Implemented (feature-gated)

### 📅 Future Enhancements (Phase 4+)
- **v0.2.0:** ✅ FACETED_BREP extraction and mesh conversion (COMPLETE - December 29, 2025)
- **v0.3.0:** ✅ Parallel batch processing, 3D viewer, GUI enhancements (COMPLETE - December 30, 2025)
- **v1.0.0:** GUI polish, quality improvements, release preparation (in progress)
- **v1.1.0:** opencascade-rs integration for full STEP support (planned)
- **Future:** Batch processing improvements, plugin system

---

## References

- [Phase3_Architecture.md](../Phase3_Architecture.md) - Detailed technical design
- [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) - Sprint-by-sprint plan
- [rust-resources.md](../rust-resources.md) - Living knowledge base
- [ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md](../ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md) - STEP implementation architecture decision record
- [ROADMAP.md](../ROADMAP.md) - Current project roadmap and status

---

_For detailed implementation details, see Phase3_Architecture.md_  
_For STEP implementation architecture decisions, see ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md_

