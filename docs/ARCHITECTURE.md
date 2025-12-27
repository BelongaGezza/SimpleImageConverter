# Architecture Overview
## Simple Image Converter

**Version:** 0.1.0  
**Last Updated:** December 26, 2025

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

### Error Sanitization

User-facing error messages are sanitized to prevent information disclosure:
- Full paths are replaced with filenames only
- Internal details are omitted
- File sizes and dimensions are shown but not exploitable details

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

## Future Enhancements

### Phase 2
- Advanced formats (SVG, WebP, glTF)
- Quality presets
- Metadata preservation

### Phase 3
- STEP/CAD format support
- Coordinate transforms
- Mesh validation

### Phase 4
- GUI application
- Batch processing
- Progress indicators

---

## References

- [Phase3_Architecture.md](../Phase3_Architecture.md) - Detailed technical design
- [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) - Sprint-by-sprint plan
- [rust-resources.md](../rust-resources.md) - Living knowledge base

---

_For detailed implementation details, see Phase3_Architecture.md_

