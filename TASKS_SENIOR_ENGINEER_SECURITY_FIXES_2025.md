# Senior Engineer Security Fixes - January 2025
**Assigned To:** Jordan Rivera (Senior Engineer)  
**Assigned By:** Casey Morgan (Security Specialist)  
**Date:** January 27, 2025  
**Priority:** 🔴 **CRITICAL** - Block production deployment until complete  
**Reference:** `SECURITY_REVIEW_CRITICAL_2025.md`

---

## Overview

Following the comprehensive security review conducted on January 27, 2025, you are tasked with implementing **critical and high-priority security fixes** identified in the codebase. These issues must be addressed before any production deployment.

**Review Document:** `SECURITY_REVIEW_CRITICAL_2025.md` - Full security audit findings

---

## Task Summary

| Task | Priority | Issue ID | Estimated Effort | Dependencies |
|------|----------|----------|-----------------|--------------|
| T1: Fix Error Message Path Disclosure | 🔴 CRITICAL | ISSUE-002 | 2 hours | None |
| T2: Fix PLY Index Conversion Bounds | 🔴 HIGH | ISSUE-001 | 3 hours | None |
| T3: Replace WebP unwrap() Calls | 🟠 MEDIUM | ISSUE-003 | 1 hour | None |
| T4: Add Path Canonicalization | 🔴 HIGH | Path Traversal | 4 hours | T1 |
| T5: Security Test Updates | 🟠 HIGH | All Issues | 2 hours | T1-T4 |
| T6: Security Review Verification | 🟠 HIGH | Verification | 1 hour | T1-T5 |

**Total Estimated Effort:** ~13 hours (2 days)

---

## Detailed Tasks

### T1: Fix Error Message Path Disclosure 🔴 CRITICAL

**Issue:** Error messages include full file paths, leaking directory structure and system information.

**Priority:** CRITICAL - Fix immediately

**Files to Modify:**
- `common/src/validation.rs` (primary)
- `common/src/io.rs` (secondary)
- Any format parsers that include paths in errors

**Current Problem:**
```rust
// common/src/validation.rs:9-12
return Err(ConversionError::InvalidInput(format!(
    "File does not exist: {}",
    path.display()  // ❌ Leaks full path like /home/user/secret/file.png
)));
```

**Solution:**
Sanitize paths to show only filenames, following the pattern already used in `common/src/security.rs`.

**Implementation:**

1. **Update `common/src/validation.rs`:**

```rust
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::error::{ConversionError, Result};

/// Sanitize a path for error messages (returns filename only)
fn sanitize_path(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Validate that a file path exists and is readable
pub fn validate_file_path(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Err(ConversionError::InvalidInput(format!(
            "File does not exist: {}",
            sanitize_path(path)
        )));
    }

    if !path.is_file() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a file: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}

/// Validate that a directory path exists and is writable
pub fn validate_directory_path(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Err(ConversionError::InvalidInput(format!(
            "Directory does not exist: {}",
            sanitize_path(path)
        )));
    }

    if !path.is_dir() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a directory: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}
```

2. **Update `common/src/io.rs`:**

```rust
// Update the error message in read_file_bytes_checked (line 36-42)
pub fn read_file_bytes_checked(path: &Path, limits: &ResourceLimits) -> Result<Vec<u8>> {
    // Get file metadata to check size before reading
    let metadata = fs::metadata(path).map_err(|e| {
        ConversionError::InvalidInput(format!(
            "Cannot read file metadata for '{}': {}",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown"),
            e
        ))
    })?;
    // ... rest of function unchanged
}
```

3. **Search and update any other error messages that include paths:**

Run this search to find all instances:
```bash
grep -r "path.display()" --include="*.rs"
```

**Acceptance Criteria:**
- [ ] `validate_file_path()` uses sanitized paths
- [ ] `validate_directory_path()` uses sanitized paths
- [ ] `read_file_bytes_checked()` uses sanitized paths
- [ ] All error messages show only filenames, not full paths
- [ ] Existing tests still pass
- [ ] New tests verify path sanitization

**Test Cases:**
```rust
#[test]
fn test_path_sanitization_in_errors() {
    let path = Path::new("/home/user/secret/file.png");
    let result = validate_file_path(path);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    // Should contain "file.png" but NOT "/home/user/secret"
    assert!(error_msg.contains("file.png"));
    assert!(!error_msg.contains("/home"));
    assert!(!error_msg.contains("secret"));
}
```

---

### T2: Fix PLY Index Conversion Bounds Checking 🔴 HIGH

**Issue:** PLY parser converts indices without validating bounds, allowing potential overflow/panic.

**Priority:** HIGH - Fix before next release

**File:** `mesh-core/src/formats/ply.rs`

**Current Problem:**
```rust
// mesh-core/src/formats/ply.rs:139-144
let indices = match vertex_indices_prop {
    ply_rs_bw::ply::Property::ListUInt(v) => {
        v.iter().map(|&i| i as usize).collect::<Vec<_>>()  // ❌ No validation
    }
    ply_rs_bw::ply::Property::ListInt(v) => {
        v.iter().map(|&i| i as usize).collect::<Vec<_>>()  // ❌ No validation
    }
```

**Solution:**
Validate indices during conversion, checking for:
- Negative values (for `ListInt`)
- Values exceeding `usize::MAX` (for both types)
- Values that would be out of bounds for the vertex array

**Implementation:**

Replace the index conversion code (lines 137-150) with:

```rust
// PLY faces can have variable vertex counts, we need to triangulate
if let Some(vertex_indices_prop) = face_data.get("vertex_indices") {
    let indices = match vertex_indices_prop {
        ply_rs_bw::ply::Property::ListUInt(v) => {
            v.iter()
                .map(|&i| {
                    // Validate u32 can fit in usize
                    // On 32-bit platforms, usize is u32, so this is always valid
                    // On 64-bit platforms, usize is u64, so this is always valid
                    // But we check anyway for safety and to catch future issues
                    if i > usize::MAX as u32 {
                        return Err(ConversionError::InvalidInput(format!(
                            "PLY vertex index {} exceeds maximum usize value",
                            i
                        )));
                    }
                    Ok(i as usize)
                })
                .collect::<Result<Vec<_>>>()?
        }
        ply_rs_bw::ply::Property::ListInt(v) => {
            v.iter()
                .map(|&i| {
                    // Validate i32 index
                    if i < 0 {
                        return Err(ConversionError::InvalidInput(format!(
                            "PLY vertex index cannot be negative: {}",
                            i
                        )));
                    }
                    if i > usize::MAX as i32 {
                        return Err(ConversionError::InvalidInput(format!(
                            "PLY vertex index {} exceeds maximum usize value",
                            i
                        )));
                    }
                    Ok(i as usize)
                })
                .collect::<Result<Vec<_>>>()?
        }
        _ => {
            return Err(ConversionError::InvalidInput(
                "PLY face has invalid vertex_indices type".to_string(),
            ));
        }
    };

    // Validate indices are within bounds (we'll know vertex count after parsing)
    // This check happens later in the validation loop, but we've now ensured
    // the indices themselves are valid usize values

    // Triangulate polygon (fan triangulation)
    if indices.len() < 3 {
        return Err(ConversionError::InvalidInput(
            "PLY face has fewer than 3 vertices".to_string(),
        ));
    }

    // Create triangles from polygon using fan triangulation
    for i in 1..(indices.len() - 1) {
        mesh.faces.push(Face {
            indices: [indices[0], indices[i], indices[i + 1]],
        });
    }
} else {
    return Err(ConversionError::InvalidInput(
        "PLY face missing vertex_indices".to_string(),
    ));
}
```

**Note:** The bounds checking against the vertex array length (lines 191-199) remains in place and is still necessary. This fix ensures the index values themselves are valid before they're used.

**Acceptance Criteria:**
- [ ] Negative indices rejected with clear error
- [ ] Indices exceeding `usize::MAX` rejected
- [ ] Error messages are clear and actionable
- [ ] Existing PLY tests still pass
- [ ] New tests for edge cases added

**Test Cases:**
```rust
#[test]
fn test_ply_negative_index_rejected() {
    // Create PLY data with negative index
    // This test would need to craft a PLY file with negative indices
    // or mock the ply_rs_bw parser response
    // Implementation depends on how to inject test data
}

#[test]
fn test_ply_large_index_rejected() {
    // Test with index > usize::MAX (on 64-bit, this would be > u32::MAX)
    // Similar to above, needs test data injection
}
```

**Alternative Approach (if test data injection is difficult):**
Add unit tests that test the conversion logic in isolation by creating mock property values.

---

### T3: Replace WebP unwrap() Calls 🟠 MEDIUM

**Issue:** WebP writer uses `unwrap()` which could panic on encoding failure.

**Priority:** MEDIUM - Fix in next sprint

**File:** `img-core/src/formats/webp.rs`

**Current Problem:**
```rust
// img-core/src/formats/webp.rs:161, 173
.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
    .unwrap();  // ❌ Could panic
```

**Solution:**
Replace `unwrap()` with proper error handling using `?` operator.

**Implementation:**

Update both `write_rgb()` and `write_rgba()` methods:

```rust
// For RGB (around line 158-163)
fn write_rgb(&self, img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    DynamicImage::ImageRgb8(img.clone())
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
        .map_err(|e| ConversionError::ConversionFailed(format!(
            "Failed to encode WebP image: {}",
            e
        )))?;
    Ok(buffer)
}

// For RGBA (around line 170-175)
fn write_rgba(&self, img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    DynamicImage::ImageRgba8(img.clone())
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::WebP)
        .map_err(|e| ConversionError::ConversionFailed(format!(
            "Failed to encode WebP image: {}",
            e
        )))?;
    Ok(buffer)
}
```

**Note:** The function signatures may need to change from returning `Vec<u8>` to returning `Result<Vec<u8>>`. Check the trait definition and update accordingly.

**Acceptance Criteria:**
- [ ] All `unwrap()` calls replaced with error handling
- [ ] Error messages are clear
- [ ] Function signatures updated if needed
- [ ] All tests pass
- [ ] Error cases are testable

**Test Cases:**
```rust
#[test]
fn test_webp_encoding_error_handled() {
    // If possible, create a scenario where encoding fails
    // This might require mocking or using invalid image data
    // Verify that error is returned, not panic
}
```

---

### T4: Add Path Canonicalization 🔴 HIGH

**Issue:** Path traversal protection is incomplete - no canonicalization or directory restrictions.

**Priority:** HIGH - Implement in next sprint

**File:** `common/src/validation.rs` (extend existing functions)

**Current Problem:**
- Path validation only checks existence, not traversal
- No canonicalization to resolve `..` components
- No directory restrictions

**Solution:**
Add canonicalization and optional directory restrictions.

**Implementation:**

Add new function to `common/src/validation.rs`:

```rust
use std::path::{Path, PathBuf};

/// Validate file path with security checks (canonicalization and optional directory restriction)
///
/// This function:
/// 1. Canonicalizes the path to resolve `..` and symlinks
/// 2. Optionally restricts the path to a specific directory
/// 3. Validates the path exists and is a file
///
/// # Arguments
///
/// * `path` - The path to validate
/// * `allowed_dir` - Optional directory to restrict paths to (None = no restriction)
///
/// # Example
///
/// ```no_run
/// use common::validation::validate_file_path_secure;
/// use std::path::Path;
///
/// // Without directory restriction
/// validate_file_path_secure(Path::new("input.png"), None)?;
///
/// // With directory restriction
/// validate_file_path_secure(
///     Path::new("input.png"),
///     Some(Path::new("/safe/directory"))
/// )?;
/// # Ok::<(), common::error::ConversionError>(())
/// ```
pub fn validate_file_path_secure(
    path: &Path,
    allowed_dir: Option<&Path>,
) -> Result<()> {
    // Canonicalize to resolve .. and symlinks
    let canonical = path.canonicalize().map_err(|e| {
        ConversionError::ValidationFailed(format!(
            "Cannot resolve path '{}': {}",
            sanitize_path(path),
            e
        ))
    })?;

    // If allowed_dir is specified, ensure path is within it
    if let Some(allowed) = allowed_dir {
        let allowed_canonical = allowed.canonicalize().map_err(|e| {
            ConversionError::ValidationFailed(format!(
                "Cannot resolve allowed directory: {}",
                e
            ))
        })?;

        if !canonical.starts_with(&allowed_canonical) {
            return Err(ConversionError::ValidationFailed(format!(
                "Path '{}' is outside allowed directory",
                sanitize_path(path)
            )));
        }
    }

    // Validate it's a file (canonicalize might have resolved to a directory)
    if !canonical.is_file() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a file: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}
```

**Update existing `validate_file_path()` to use canonicalization (optional - for backward compatibility, keep both):**

```rust
/// Validate that a file path exists and is readable
///
/// Note: For better security, use `validate_file_path_secure()` which
/// canonicalizes paths and can restrict to specific directories.
pub fn validate_file_path(path: &std::path::Path) -> Result<()> {
    // Use canonicalization for basic security
    let canonical = path.canonicalize().map_err(|e| {
        ConversionError::InvalidInput(format!(
            "Cannot resolve path '{}': {}",
            sanitize_path(path),
            e
        ))
    })?;

    if !canonical.is_file() {
        return Err(ConversionError::InvalidInput(format!(
            "Path is not a file: {}",
            sanitize_path(path)
        )));
    }

    Ok(())
}
```

**Update CLI tools to use secure validation (optional enhancement):**

In `img-convert/src/main.rs` and `mesh-convert/src/main.rs`, consider adding an optional `--restrict-to-dir` flag:

```rust
#[arg(long)]
/// Restrict file access to this directory (security)
restrict_to_dir: Option<String>,
```

**Acceptance Criteria:**
- [ ] `validate_file_path_secure()` implemented
- [ ] Path canonicalization works correctly
- [ ] Directory restriction works correctly
- [ ] Existing `validate_file_path()` updated or documented
- [ ] Tests for path traversal attempts
- [ ] Tests for directory restriction

**Test Cases:**
```rust
#[test]
fn test_path_traversal_blocked() {
    // Create a test directory structure
    let temp_dir = tempfile::tempdir().unwrap();
    let safe_dir = temp_dir.path().join("safe");
    std::fs::create_dir(&safe_dir).unwrap();
    
    // Try to access file outside safe directory using ..
    let malicious_path = safe_dir.join("../../etc/passwd");
    
    // Should fail (either file doesn't exist or is outside allowed dir)
    let result = validate_file_path_secure(&malicious_path, Some(&safe_dir));
    assert!(result.is_err());
}

#[test]
fn test_canonicalization_resolves_dots() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, b"test").unwrap();
    
    // Path with .. should resolve correctly
    let path_with_dots = test_file.parent().unwrap().join("..").join("test.txt");
    assert!(validate_file_path_secure(&path_with_dots, None).is_ok());
}
```

---

### T5: Security Test Updates 🟠 HIGH

**Issue:** Add/update tests to verify security fixes work correctly.

**Priority:** HIGH

**Files:**
- `common/src/validation.rs` (add tests)
- `mesh-core/src/formats/ply.rs` (add tests)
- `img-core/src/formats/webp.rs` (add tests)
- `common/src/limits.rs` (verify existing tests)

**Requirements:**
1. Test path sanitization in error messages
2. Test PLY index conversion edge cases
3. Test WebP error handling
4. Test path canonicalization and traversal blocking
5. Verify all security-related tests pass

**Test Implementation:**

See test cases in T1-T4 above. Create comprehensive test suites for each fix.

**Acceptance Criteria:**
- [ ] All new security tests pass
- [ ] Edge cases covered
- [ ] No panics on malicious input
- [ ] Test coverage for security fixes >80%

---

### T6: Security Review Verification 🟠 HIGH

**Issue:** Verify all fixes are complete and codebase is secure.

**Priority:** HIGH

**Requirements:**
1. Run `cargo test` - all tests pass
2. Run `cargo clippy` - no security-related warnings
3. Run `cargo audit` - no new vulnerabilities
4. Review code changes against security checklist
5. Update `SECURITY_RISK_REGISTER.md` with resolved issues

**Checklist:**
- [ ] All critical issues fixed (T1)
- [ ] All high-priority issues fixed (T2, T4)
- [ ] Medium-priority issues fixed (T3)
- [ ] All tests pass
- [ ] No new `unwrap()` calls in production code
- [ ] No path disclosure in error messages
- [ ] Path traversal protection implemented
- [ ] Security tests comprehensive
- [ ] Documentation updated

**Update Risk Register:**

Update `SECURITY_RISK_REGISTER.md`:

```markdown
### RISK-002: Format Spoofing Attacks
**Status:** ✅ MITIGATED (updated date)

### RISK-005: Path Traversal Attacks  
**Status:** ✅ MITIGATED (updated date)
**Resolution:** Path canonicalization implemented in T4
```

---

## Definition of Done

All tasks are complete when:

1. ✅ All code compiles without warnings
2. ✅ All tests pass (`cargo test --workspace`)
3. ✅ Clippy clean (`cargo clippy --workspace`)
4. ✅ Format applied (`cargo fmt --all`)
5. ✅ Security tests specifically pass
6. ✅ `cargo audit` clean (no new issues)
7. ✅ Code reviewed by Security Specialist
8. ✅ `SECURITY_RISK_REGISTER.md` updated
9. ✅ `SECURITY_REVIEW_CRITICAL_2025.md` updated with resolution status

---

## Timeline

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| Day 1 AM | T1: Error message sanitization | Path sanitization complete |
| Day 1 PM | T2: PLY index bounds checking | PLY parser secure |
| Day 2 AM | T3: WebP unwrap() replacement | WebP error handling |
| Day 2 PM | T4: Path canonicalization | Path traversal protection |
| Day 3 AM | T5: Security test updates | Comprehensive test suite |
| Day 3 PM | T6: Verification & documentation | All fixes verified |

---

## Questions for Security Specialist

If you have questions during implementation:

1. **Path sanitization scope:** Should we sanitize paths in ALL error messages, or only user-facing ones?
2. **Directory restriction:** Should CLI tools have a default restricted directory, or only when flag is set?
3. **PLY index validation:** Should we validate indices against vertex count during conversion, or is later validation sufficient?
4. **Backward compatibility:** Can we change function signatures, or must we maintain compatibility?

---

## Reporting

Please update progress in this document or create `SECURITY_FIXES_STATUS.md` with:
- Tasks completed (with checkboxes)
- Any issues encountered
- Actual vs estimated time
- Code review status
- Test results

---

**Signed:** Casey Morgan, Security Specialist  
**Date:** January 27, 2025

**Priority:** 🔴 **CRITICAL** - These fixes block production deployment. Please prioritize accordingly.

