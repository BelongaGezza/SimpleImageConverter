# Senior Engineer Security Implementation Tasks
**Assigned To:** Jordan Rivera (Senior Engineer)  
**Assigned By:** Alex Chen (System Architect)  
**Date:** December 26, 2025  
**Priority:** 🔴 **CRITICAL** - Block release until complete

---

## Overview

Following the Security Specialist review and Architect approval, you are tasked with implementing the security architecture changes. These are critical fixes that must be completed before any production deployment.

**Reference Documents:**
- `SECURITY_REVIEW.md` - Security Specialist findings
- `ARCHITECT_REVIEW_SECURITY.md` - Architect recommendations
- `Phase3_Architecture.md` Section 12 - Security Architecture specification

---

## Task Summary

| Task | Priority | Estimated Effort | Dependencies |
|------|----------|-----------------|--------------|
| T1: Create ResourceLimits module | 🔴 Critical | 2 hours | None |
| T2: Add file size validation | 🔴 Critical | 1 hour | T1 |
| T3: Add dimension validation | 🔴 Critical | 1 hour | T1 |
| T4: Add mesh resource validation | 🔴 Critical | 1 hour | T1 |
| T5: Update CLI with limit flags | 🟠 High | 2 hours | T1-T4 |
| T6: Add magic byte detection | 🟠 High | 2 hours | None |
| T7: Add error message sanitization | 🟠 High | 1 hour | None |
| T8: Add quality parameter validation | 🟡 Medium | 30 min | None |
| T9: Write security tests | 🟠 High | 3 hours | T1-T4 |
| T10: Run dependency audit | 🟠 High | 30 min | None |

**Total Estimated Effort:** ~14 hours (2 days)

---

## Detailed Tasks

### T1: Create ResourceLimits Module 🔴 CRITICAL

**File:** `common/src/limits.rs` (NEW)

**Requirements:**
1. Create `ResourceLimits` struct with configurable limits
2. Implement `Default` with secure defaults
3. Implement `check_*` validation methods
4. Export from `common/src/lib.rs`

**Implementation:**

```rust
// common/src/limits.rs
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};

/// Default maximum file size: 100MB
pub const DEFAULT_MAX_FILE_SIZE: usize = 100 * 1024 * 1024;

/// Default maximum image dimension: 65535 pixels
pub const DEFAULT_MAX_IMAGE_DIMENSION: u32 = 65535;

/// Default maximum vertices: 10 million
pub const DEFAULT_MAX_VERTICES: usize = 10_000_000;

/// Default maximum faces: 10 million
pub const DEFAULT_MAX_FACES: usize = 10_000_000;

/// Centralized resource limits for security
/// 
/// All file operations should validate against these limits
/// before allocating memory or processing data.
/// 
/// # Example
/// 
/// ```
/// use common::limits::ResourceLimits;
/// 
/// let limits = ResourceLimits::default();
/// limits.check_file_size(1024)?;  // OK
/// limits.check_file_size(200 * 1024 * 1024)?;  // Error: too large
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum file size in bytes
    pub max_file_size: usize,
    
    /// Maximum image dimension (width or height) in pixels
    pub max_image_dimension: u32,
    
    /// Maximum number of mesh vertices
    pub max_vertices: usize,
    
    /// Maximum number of mesh faces
    pub max_faces: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_file_size: DEFAULT_MAX_FILE_SIZE,
            max_image_dimension: DEFAULT_MAX_IMAGE_DIMENSION,
            max_vertices: DEFAULT_MAX_VERTICES,
            max_faces: DEFAULT_MAX_FACES,
        }
    }
}

impl ResourceLimits {
    /// Create new ResourceLimits with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create permissive limits for trusted input only
    /// 
    /// WARNING: Only use for trusted input sources!
    pub fn permissive() -> Self {
        Self {
            max_file_size: 1024 * 1024 * 1024,     // 1GB
            max_image_dimension: 131072,            // 128K
            max_vertices: 100_000_000,              // 100M
            max_faces: 100_000_000,                 // 100M
        }
    }
    
    /// Validate file size against limit
    pub fn check_file_size(&self, size: usize) -> Result<()> {
        if size > self.max_file_size {
            return Err(ConversionError::InvalidInput(format!(
                "File size {} bytes exceeds limit of {} bytes",
                size, self.max_file_size
            )));
        }
        Ok(())
    }
    
    /// Validate image dimensions against limit
    pub fn check_image_dimensions(&self, width: u32, height: u32) -> Result<()> {
        if width > self.max_image_dimension {
            return Err(ConversionError::InvalidInput(format!(
                "Image width {} exceeds limit of {}",
                width, self.max_image_dimension
            )));
        }
        if height > self.max_image_dimension {
            return Err(ConversionError::InvalidInput(format!(
                "Image height {} exceeds limit of {}",
                height, self.max_image_dimension
            )));
        }
        Ok(())
    }
    
    /// Validate mesh vertex count against limit
    pub fn check_vertex_count(&self, count: usize) -> Result<()> {
        if count > self.max_vertices {
            return Err(ConversionError::InvalidInput(format!(
                "Vertex count {} exceeds limit of {}",
                count, self.max_vertices
            )));
        }
        Ok(())
    }
    
    /// Validate mesh face count against limit
    pub fn check_face_count(&self, count: usize) -> Result<()> {
        if count > self.max_faces {
            return Err(ConversionError::InvalidInput(format!(
                "Face count {} exceeds limit of {}",
                count, self.max_faces
            )));
        }
        Ok(())
    }
    
    /// Validate all mesh resources at once
    pub fn check_mesh_resources(&self, vertices: usize, faces: usize) -> Result<()> {
        self.check_vertex_count(vertices)?;
        self.check_face_count(faces)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_limits() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_file_size, DEFAULT_MAX_FILE_SIZE);
        assert_eq!(limits.max_image_dimension, DEFAULT_MAX_IMAGE_DIMENSION);
    }
    
    #[test]
    fn test_check_file_size_ok() {
        let limits = ResourceLimits::default();
        assert!(limits.check_file_size(1024).is_ok());
        assert!(limits.check_file_size(50 * 1024 * 1024).is_ok());
    }
    
    #[test]
    fn test_check_file_size_exceeded() {
        let limits = ResourceLimits::default();
        assert!(limits.check_file_size(200 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_check_dimensions_ok() {
        let limits = ResourceLimits::default();
        assert!(limits.check_image_dimensions(1920, 1080).is_ok());
        assert!(limits.check_image_dimensions(65535, 65535).is_ok());
    }
    
    #[test]
    fn test_check_dimensions_exceeded() {
        let limits = ResourceLimits::default();
        assert!(limits.check_image_dimensions(100000, 100).is_err());
        assert!(limits.check_image_dimensions(100, 100000).is_err());
    }
    
    #[test]
    fn test_check_mesh_resources_ok() {
        let limits = ResourceLimits::default();
        assert!(limits.check_mesh_resources(1000, 2000).is_ok());
    }
    
    #[test]
    fn test_check_mesh_resources_exceeded() {
        let limits = ResourceLimits::default();
        assert!(limits.check_mesh_resources(20_000_000, 100).is_err());
        assert!(limits.check_mesh_resources(100, 20_000_000).is_err());
    }
    
    #[test]
    fn test_permissive_limits() {
        let limits = ResourceLimits::permissive();
        assert!(limits.check_file_size(500 * 1024 * 1024).is_ok());
        assert!(limits.check_image_dimensions(100000, 100000).is_ok());
    }
}
```

**Update `common/src/lib.rs`:**
```rust
pub mod error;
pub mod io;
pub mod limits;  // NEW
pub mod progress;
pub mod validation;

pub use error::{ConversionError, Result};
pub use limits::ResourceLimits;  // NEW
```

**Acceptance Criteria:**
- [ ] `ResourceLimits` struct exists with all fields
- [ ] Default implementation has secure values
- [ ] All `check_*` methods work correctly
- [ ] Unit tests pass
- [ ] Exported from `common` crate

---

### T2: Add File Size Validation 🔴 CRITICAL

**File:** `common/src/io.rs`

**Requirements:**
1. Add `read_file_bytes_checked()` function
2. Validate file size before reading
3. Keep original `read_file_bytes()` for backward compatibility (deprecated)

**Implementation:**

```rust
// Add to common/src/io.rs

use crate::limits::ResourceLimits;

/// Read file with size validation
/// 
/// Validates file size against limits before reading to prevent
/// memory exhaustion attacks.
pub fn read_file_bytes_checked(path: &Path, limits: &ResourceLimits) -> Result<Vec<u8>> {
    // Get file metadata to check size
    let metadata = fs::metadata(path).map_err(|e| {
        ConversionError::InvalidInput(format!(
            "Cannot read file metadata: {}",
            path.display()
        ))
    })?;
    
    let size = metadata.len() as usize;
    limits.check_file_size(size)?;
    
    fs::read(path).map_err(ConversionError::Io)
}
```

**Acceptance Criteria:**
- [ ] Function validates size before reading
- [ ] Returns clear error on oversized files
- [ ] Tests with files at and beyond limits

---

### T3: Add Image Dimension Validation 🔴 CRITICAL

**File:** `img-core/src/validation.rs`

**Requirements:**
1. Add `ResourceLimits` parameter to `validate_image_data()`
2. Check dimensions against limits
3. Update all callers

**Implementation:**

```rust
// Update img-core/src/validation.rs

use common::limits::ResourceLimits;

/// Validate image data dimensions and data length
pub fn validate_image_data(image: &ImageData, limits: &ResourceLimits) -> Result<()> {
    // Check dimensions against resource limits
    limits.check_image_dimensions(image.width, image.height)?;
    
    // ... existing validation code ...
}
```

**Update callers:**
- `img-core/src/formats/png.rs`
- `img-core/src/formats/jpg.rs`
- `img-core/src/formats/bmp.rs`
- `img-core/src/formats/gif.rs`

**Acceptance Criteria:**
- [ ] Dimension limits enforced
- [ ] All format writers updated
- [ ] Tests with extreme dimensions

---

### T4: Add Mesh Resource Validation 🔴 CRITICAL

**File:** `mesh-core/src/formats/stl.rs`

**Requirements:**
1. Validate vertex/face counts after parsing
2. Return error if limits exceeded

**Implementation:**

```rust
// Update mesh-core/src/formats/stl.rs

use common::limits::ResourceLimits;

impl MeshReader for StlFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        let limits = ResourceLimits::default();
        
        // ... existing parsing code ...
        
        // Validate resource limits before allocating
        limits.check_mesh_resources(
            stl_mesh.vertices.len(),
            stl_mesh.faces.len()
        )?;
        
        // ... rest of implementation ...
    }
}
```

**Acceptance Criteria:**
- [ ] Vertex count validated
- [ ] Face count validated
- [ ] Tests with extreme mesh sizes

---

### T5: Update CLI with Limit Flags 🟠 HIGH

**Files:** `img-convert/src/main.rs`, `mesh-convert/src/main.rs`

**Requirements:**
1. Add optional CLI flags for limit overrides
2. Pass limits through to converters

**Implementation:**

```rust
// img-convert/src/main.rs

#[derive(Parser)]
struct Args {
    // ... existing args ...
    
    /// Maximum file size in MB (default: 100)
    #[arg(long, default_value_t = 100)]
    max_file_size_mb: usize,
    
    /// Maximum image dimension in pixels (default: 65535)
    #[arg(long, default_value_t = 65535)]
    max_dimension: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Build limits from args
    let limits = ResourceLimits {
        max_file_size: args.max_file_size_mb * 1024 * 1024,
        max_image_dimension: args.max_dimension,
        ..ResourceLimits::default()
    };
    
    // Use limits in file reading
    let input_data = common::io::read_file_bytes_checked(input_path, &limits)?;
    
    // ... rest of implementation ...
}
```

**Acceptance Criteria:**
- [ ] CLI flags documented
- [ ] Limits configurable at runtime
- [ ] Defaults match security requirements

---

### T6: Add Magic Byte Detection 🟠 HIGH

**File:** `img-core/src/formats/registry.rs`

**Requirements:**
1. Add `detect_from_bytes()` method
2. Add `verify_format()` method
3. Use in CLI before processing

**Implementation:**

```rust
// Add to img-core/src/formats/registry.rs

impl FormatRegistry {
    /// Detect format from file magic bytes
    pub fn detect_from_bytes(data: &[u8]) -> Option<ImageFormat> {
        if data.len() < 8 {
            return None;
        }
        
        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Some(ImageFormat::Png);
        }
        
        // JPEG: FF D8 FF
        if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Some(ImageFormat::Jpeg);
        }
        
        // BMP: 42 4D
        if data.starts_with(&[0x42, 0x4D]) {
            return Some(ImageFormat::Bmp);
        }
        
        // GIF: 47 49 46 38
        if data.starts_with(&[0x47, 0x49, 0x46, 0x38]) {
            return Some(ImageFormat::Gif);
        }
        
        None
    }
    
    /// Verify format matches expected (two-stage detection)
    pub fn verify_format(data: &[u8], expected: ImageFormat) -> Result<()> {
        if let Some(detected) = Self::detect_from_bytes(data) {
            if detected != expected {
                return Err(ConversionError::InvalidFormat(format!(
                    "Format mismatch: expected {:?}, detected {:?}",
                    expected, detected
                )));
            }
        }
        Ok(())
    }
}
```

**Acceptance Criteria:**
- [ ] All supported formats detected by magic bytes
- [ ] Format spoofing detected and rejected
- [ ] Tests for each format

---

### T7: Add Error Message Sanitization 🟠 HIGH

**File:** `common/src/error.rs`

**Requirements:**
1. Add `user_message()` method to `ConversionError`
2. Sanitize paths and internal details

**Implementation:**

```rust
// Add to common/src/error.rs

impl ConversionError {
    /// Get user-safe error message (sanitized for display)
    pub fn user_message(&self) -> String {
        match self {
            ConversionError::Io(e) => format!("File error: {}", e.kind()),
            ConversionError::InvalidFormat(msg) => {
                format!("Invalid format: {}", Self::sanitize(msg))
            }
            ConversionError::InvalidInput(msg) => {
                format!("Invalid input: {}", Self::sanitize(msg))
            }
            ConversionError::ConversionFailed(msg) => {
                format!("Conversion failed: {}", Self::sanitize(msg))
            }
            _ => self.to_string(),
        }
    }
    
    fn sanitize(msg: &str) -> String {
        // Limit length
        let truncated: String = msg.chars().take(200).collect();
        
        // Remove full paths (keep only filename)
        // This is a simple implementation; could be enhanced
        truncated
    }
}
```

**Acceptance Criteria:**
- [ ] `user_message()` method implemented
- [ ] Full paths not exposed
- [ ] Message length limited

---

### T8: Add Quality Parameter Validation 🟡 MEDIUM

**File:** `img-convert/src/main.rs`

**Requirements:**
1. Validate quality is 1-100
2. Return clear error if out of range

**Implementation:**

```rust
// Already in main.rs, but ensure it's checked early

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Validate quality range
    if args.quality == 0 || args.quality > 100 {
        return Err(ConversionError::InvalidInput(
            "Quality must be between 1 and 100".to_string()
        ).into());
    }
    
    // ... rest of implementation ...
}
```

**Acceptance Criteria:**
- [ ] Quality 0 rejected
- [ ] Quality > 100 rejected
- [ ] Clear error message

---

### T9: Write Security Tests 🟠 HIGH

**Files:** 
- `common/src/limits.rs` (unit tests)
- `img-core/tests/security_tests.rs` (NEW)
- `mesh-core/tests/security_tests.rs` (NEW)

**Requirements:**
1. Test resource limit enforcement
2. Test format spoofing detection
3. Test extreme dimension rejection
4. Test integer overflow handling

**Test Cases:**

```rust
// img-core/tests/security_tests.rs

use common::limits::ResourceLimits;
use img_core::FormatRegistry;

#[test]
fn test_reject_oversized_file() {
    let limits = ResourceLimits::default();
    let result = limits.check_file_size(200 * 1024 * 1024);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds limit"));
}

#[test]
fn test_reject_extreme_dimensions() {
    let limits = ResourceLimits::default();
    let result = limits.check_image_dimensions(100_000, 100_000);
    assert!(result.is_err());
}

#[test]
fn test_detect_png_magic_bytes() {
    let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00];
    let format = FormatRegistry::detect_from_bytes(&png_data);
    assert_eq!(format, Some(img_core::ImageFormat::Png));
}

#[test]
fn test_detect_format_spoofing() {
    // JPEG data with wrong extension would be detected here
    let jpeg_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x00, 0x00, 0x00];
    let result = FormatRegistry::verify_format(&jpeg_data, img_core::ImageFormat::Png);
    assert!(result.is_err());
}

#[test]
fn test_integer_overflow_protection() {
    // This should use checked arithmetic and not panic
    let limits = ResourceLimits::default();
    // If dimensions could overflow, the check should catch it
    assert!(limits.check_image_dimensions(u32::MAX, u32::MAX).is_err());
}
```

**Acceptance Criteria:**
- [ ] All security test cases pass
- [ ] Edge cases covered
- [ ] No panics on bad input

---

### T10: Run Dependency Audit 🟠 HIGH

**Requirements:**
1. Install cargo-audit
2. Run audit and fix any issues
3. Document results

**Commands:**

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit

# If issues found, update dependencies or add ignores with justification
```

**Acceptance Criteria:**
- [ ] `cargo audit` passes (or known issues documented)
- [ ] No critical vulnerabilities
- [ ] High vulnerabilities addressed or documented

---

## Definition of Done

All tasks are complete when:

1. ✅ All code compiles without warnings
2. ✅ All tests pass (`cargo test`)
3. ✅ Clippy clean (`cargo clippy`)
4. ✅ Format applied (`cargo fmt`)
5. ✅ Security tests specifically pass
6. ✅ `cargo audit` clean or issues documented
7. ✅ Code reviewed by Security Specialist (re-review)
8. ✅ Documentation updated

---

## Timeline

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| Day 1 AM | T1, T2, T3 | ResourceLimits, file/dimension validation |
| Day 1 PM | T4, T8, T10 | Mesh validation, quality validation, audit |
| Day 2 AM | T5, T6 | CLI flags, magic byte detection |
| Day 2 PM | T7, T9 | Error sanitization, security tests |

---

## Questions for Architect

If you have questions during implementation:

1. **Limit values**: Are the default limits (100MB, 65K, 10M) appropriate?
2. **Backward compatibility**: Should `read_file_bytes()` be deprecated or removed?
3. **CLI flags**: Should limits be required flags or optional with defaults?
4. **Magic bytes**: What if format can't be detected (unknown format)?

---

## Reporting

Please update `SECURITY_IMPLEMENTATION_STATUS.md` (create if needed) with:
- Tasks completed
- Any issues encountered
- Actual vs estimated time
- Code review status

---

**Signed:** Alex Chen, System Architect  
**Date:** December 26, 2025

