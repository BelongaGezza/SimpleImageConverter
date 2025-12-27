# Security Review - SimpleImageConverter
**Reviewer:** Casey Morgan (Security Specialist)  
**Date:** December 26, 2025  
**Status:** CRITICAL ISSUES FOUND

---

## Executive Summary

This security review identified **3 CRITICAL** and **4 HIGH** severity vulnerabilities that must be addressed before production deployment. The codebase demonstrates good security practices in some areas (integer overflow protection, no unsafe code), but lacks critical resource limits and has several input validation gaps.

**Overall Security Posture:** ⚠️ **NEEDS IMPROVEMENT**

---

## Threat Model

**Attack Surface:**
- All file inputs are untrusted (image and mesh files)
- CLI accepts user-provided file paths
- No network exposure (local tool only)

**Attack Vectors:**
1. Memory exhaustion via maliciously crafted files
2. Path traversal via file path manipulation
3. Integer overflow in dimension calculations
4. Panic-based DoS via malformed input
5. Information disclosure via error messages

---

## Critical Issues (MUST FIX)

### 🔴 CRITICAL-1: Missing File Size Limits

**Location:** `common/src/io.rs`, `img-convert/src/main.rs`, all format readers

**Vulnerability:**
The codebase reads entire files into memory without size validation. A malicious file could cause memory exhaustion.

**Attack Scenario:**
```rust
// Current code (VULNERABLE):
let input_data = read_file_bytes(input_path)?;  // No size check!
let img = image::load_from_memory_with_format(data, ImageFormat::Png)?;
```

An attacker could create a 10GB file with a `.png` extension, causing the application to allocate 10GB of memory and potentially crash the system.

**Evidence:**
- `common/src/io.rs:9-11`: `read_file_bytes()` reads entire file without limits
- `img-convert/src/main.rs:53`: No validation before reading
- All format readers (`png.rs`, `jpg.rs`, `bmp.rs`, `gif.rs`) accept arbitrary-sized byte slices

**Fix Required:**
```rust
// Add to common/src/io.rs:
const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB

pub fn read_file_bytes(path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len() as usize;
    
    if size > MAX_FILE_SIZE {
        return Err(ConversionError::InvalidInput(format!(
            "File too large: {} bytes (max: {} bytes)",
            size, MAX_FILE_SIZE
        )));
    }
    
    fs::read(path).map_err(ConversionError::Io)
}
```

**Also add validation in format readers:**
```rust
// In each format reader's read() method:
if data.len() > MAX_FILE_SIZE {
    return Err(ConversionError::InvalidInput(format!(
        "Input data too large: {} bytes (max: {} bytes)",
        data.len(), MAX_FILE_SIZE
    )));
}
```

**Priority:** 🔴 **CRITICAL** - Fix immediately

---

### 🔴 CRITICAL-2: Missing Dimension Limits

**Location:** `img-core/src/validation.rs`, `img-core/src/formats/*.rs`

**Vulnerability:**
While integer overflow is protected, there are no maximum dimension limits. An attacker could create a file with dimensions like 100,000 x 100,000 pixels, causing massive memory allocation even if the file size is reasonable.

**Attack Scenario:**
A 1MB file could declare dimensions of 100,000 x 100,000 pixels. After validation passes (no overflow), the code would attempt to allocate:
- RGB: 100,000 × 100,000 × 3 = 30GB of memory
- RGBA: 100,000 × 100,000 × 4 = 40GB of memory

**Evidence:**
- `img-core/src/validation.rs:8-59`: Validates overflow but not maximum dimensions
- Format readers trust dimensions from parsed files without limits

**Fix Required:**
```rust
// Add to img-core/src/validation.rs:
const MAX_DIMENSION: u32 = 65535; // Reasonable limit (matches many format specs)

pub fn validate_image_data(image: &ImageData) -> Result<()> {
    // Check maximum dimensions
    if image.width > MAX_DIMENSION || image.height > MAX_DIMENSION {
        return Err(ConversionError::InvalidInput(format!(
            "Image dimensions too large: {}x{} (max: {}x{})",
            image.width, image.height, MAX_DIMENSION, MAX_DIMENSION
        )));
    }
    
    // ... existing validation ...
}
```

**Also validate in format readers before processing:**
```rust
// In each format reader, after parsing dimensions:
if width > MAX_DIMENSION || height > MAX_DIMENSION {
    return Err(ConversionError::InvalidInput(format!(
        "Image dimensions too large: {}x{}", width, height
    )));
}
```

**Priority:** 🔴 **CRITICAL** - Fix immediately

---

### 🔴 CRITICAL-3: Missing Mesh Resource Limits

**Location:** `mesh-core/src/formats/stl.rs`

**Vulnerability:**
STL reader has no limits on number of vertices or faces. A malicious STL file could declare millions of triangles, causing memory exhaustion.

**Attack Scenario:**
```rust
// Current code (VULNERABLE):
let stl_mesh = stl_io::read_stl(&mut cursor)?;
mesh.vertices = stl_mesh.vertices.iter().map(...).collect(); // No limit!
```

A binary STL file could declare 2^32 triangles in the header, causing allocation of massive vectors.

**Evidence:**
- `mesh-core/src/formats/stl.rs:26-73`: No validation of vertex/face counts
- `stl_io` library may not enforce limits

**Fix Required:**
```rust
// Add constants:
const MAX_VERTICES: usize = 10_000_000; // 10 million vertices
const MAX_FACES: usize = 10_000_000;    // 10 million faces

impl MeshReader for StlFormat {
    fn read(&self, data: &[u8]) -> Result<Mesh> {
        // ... existing read code ...
        
        // Validate counts before allocation
        if stl_mesh.vertices.len() > MAX_VERTICES {
            return Err(ConversionError::InvalidInput(format!(
                "Too many vertices: {} (max: {})",
                stl_mesh.vertices.len(), MAX_VERTICES
            )));
        }
        
        if stl_mesh.faces.len() > MAX_FACES {
            return Err(ConversionError::InvalidInput(format!(
                "Too many faces: {} (max: {})",
                stl_mesh.faces.len(), MAX_FACES
            )));
        }
        
        // ... rest of code ...
    }
}
```

**Priority:** 🔴 **CRITICAL** - Fix immediately

---

## High Severity Issues

### 🟠 HIGH-1: Path Traversal Risk

**Location:** `img-convert/src/main.rs:43-50`, `mesh-convert/src/main.rs`

**Vulnerability:**
Output path generation doesn't validate against path traversal. While not directly exploitable in a CLI tool, this could be an issue if the tool is ever used programmatically.

**Evidence:**
```rust
// img-convert/src/main.rs:43-50
let output_path = if let Some(output) = args.output {
    Path::new(&output).to_path_buf()  // No validation!
} else {
    let mut output = input_path.to_path_buf();
    output.set_extension(&args.format);
    output
};
```

**Fix Required:**
```rust
// Add to common/src/validation.rs:
pub fn validate_output_path(path: &Path) -> Result<()> {
    // Canonicalize to prevent path traversal
    let canonical = path.canonicalize()
        .map_err(|_| ConversionError::InvalidInput(
            "Cannot resolve output path".to_string()
        ))?;
    
    // Optionally: Check path is within allowed directory
    // For CLI tool, this may not be necessary, but good practice
    
    Ok(())
}
```

**Priority:** 🟠 **HIGH** - Fix before production

---

### 🟠 HIGH-2: Error Messages May Leak Information

**Location:** Multiple format readers

**Vulnerability:**
Error messages include file sizes and paths which could leak information about the system structure.

**Evidence:**
```rust
// img-core/src/formats/png.rs:27-33
ConversionError::ConversionFailed(format!(
    "Failed to read PNG image ({} bytes): {}",
    data.len(),  // Leaks file size
    e
))
```

**Fix Required:**
Sanitize error messages for external display:
- Remove full paths (use filename only)
- Limit file size in error messages
- Don't expose internal error details

**Priority:** 🟠 **HIGH** - Fix before production

---

### 🟠 HIGH-3: No Input Validation in Format Detection

**Location:** `img-core/src/formats/registry.rs:98-102`

**Vulnerability:**
Format detection relies solely on file extension, which can be spoofed. A malicious file could have a `.png` extension but contain JPEG data, potentially causing parsing issues.

**Evidence:**
```rust
// img-core/src/formats/registry.rs:98-102
pub fn detect_from_path(path: &Path) -> Result<ImageFormat> {
    let ext = get_extension(path)?;
    Self::detect_format(&ext)  // Only checks extension!
}
```

**Fix Required:**
Add magic byte validation:
```rust
// Add magic byte checking before format detection
pub fn detect_format_from_bytes(data: &[u8]) -> Option<ImageFormat> {
    if data.len() < 8 { return None; }
    
    match &data[0..8] {
        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] => Some(ImageFormat::Png),
        [0xFF, 0xD8, 0xFF, ..] => Some(ImageFormat::Jpeg),
        [0x42, 0x4D, ..] => Some(ImageFormat::Bmp),
        [0x47, 0x49, 0x46, 0x38, ..] => Some(ImageFormat::Gif),
        _ => None,
    }
}
```

**Priority:** 🟠 **HIGH** - Fix before production

---

### 🟠 HIGH-4: Dependency Security Audit Needed

**Location:** `Cargo.toml`, `Cargo.lock`

**Vulnerability:**
No evidence of dependency security auditing. Dependencies may contain known vulnerabilities.

**Required Actions:**
1. Run `cargo audit` to check for known vulnerabilities
2. Set up automated dependency scanning in CI/CD
3. Consider using `cargo deny` for policy enforcement

**Priority:** 🟠 **HIGH** - Audit immediately

---

## Medium Severity Issues

### 🟡 MEDIUM-1: GIF Frame Limit Not Enforced

**Location:** `img-core/src/formats/gif.rs:28-78`

**Vulnerability:**
GIF reader extracts only the first frame, but doesn't validate the number of frames. An animated GIF with thousands of frames could cause excessive processing.

**Fix:** Add frame count validation (if possible with `image` crate API).

**Priority:** 🟡 **MEDIUM**

---

### 🟡 MEDIUM-2: Quality Parameter Not Validated

**Location:** `img-convert/src/main.rs:25-26`

**Vulnerability:**
Quality parameter accepts any `u8` value (0-255), but JPEG quality should be 0-100.

**Evidence:**
```rust
#[arg(short, long, default_value_t = 90)]
quality: u8,  // No validation!
```

**Fix:** Add validation or use a validated type.

**Priority:** 🟡 **MEDIUM**

---

## Low Severity Issues

### 🔵 LOW-1: Error Messages Could Be More User-Friendly

Some error messages are technical and could be improved for end users.

**Priority:** 🔵 **LOW**

---

## Positive Security Findings ✅

1. **No Unsafe Code:** ✅ No `unsafe` blocks found in the codebase
2. **Integer Overflow Protection:** ✅ Proper use of `checked_mul()` in `img-core/src/validation.rs`
3. **Panic Safety:** ✅ Most operations return `Result` instead of panicking
4. **Input Validation:** ✅ Image data validation exists (though incomplete)
5. **Bounds Checking:** ✅ Rust's type system prevents buffer overflows
6. **Error Handling:** ✅ Comprehensive error types and propagation

---

## Recommendations

### Immediate Actions (Before Any Release)
1. ✅ Add file size limits (100MB default, configurable)
2. ✅ Add maximum dimension limits (65,535 pixels)
3. ✅ Add mesh resource limits (10M vertices/faces)
4. ✅ Run `cargo audit` and fix any vulnerabilities
5. ✅ Add magic byte validation for format detection

### Short-Term (Next Sprint)
1. Sanitize error messages
2. Add path validation
3. Add quality parameter validation
4. Set up automated security scanning in CI/CD

### Long-Term
1. Consider fuzz testing for format parsers
2. Add security documentation
3. Implement rate limiting if tool becomes network-accessible
4. Add security testing to test suite

---

## Testing Recommendations

1. **Fuzz Testing:** Use `cargo fuzz` to test format parsers with random input
2. **Malicious File Tests:** Create test files with:
   - Extremely large dimensions
   - Extremely large file sizes
   - Malformed headers
   - Path traversal attempts
3. **Resource Exhaustion Tests:** Test with files at the limits

---

## Compliance Notes

This review follows the security checklist from `.cursor/rules/security.mdc`:
- ✅ Unsafe code blocks: None found
- ⚠️ Input validation: Partial (needs file size and dimension limits)
- ⚠️ Error messages: Need sanitization
- ✅ Buffer handling: Safe (Rust type system)
- ✅ Integer overflow: Protected
- ✅ Panic safety: Good
- ❌ Denial of service vectors: **MISSING RESOURCE LIMITS**

---

## Conclusion

The codebase demonstrates good security practices in several areas, but **critical resource limits are missing**. These must be addressed before any production deployment. The vulnerabilities identified are primarily denial-of-service vectors that could be exploited by malicious files.

**Recommendation:** 🔴 **DO NOT DEPLOY** until CRITICAL issues are resolved.

---

**Next Steps:**
1. Architect review of this security assessment
2. Prioritize fixes based on severity
3. Implement fixes
4. Re-review after fixes
5. Set up ongoing security monitoring

