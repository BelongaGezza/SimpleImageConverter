# Task Assignment: Sam Parker (Junior Engineer - 2D Formats)

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** December 26, 2025  
**Priority:** HIGH - Complete Sprint 2

---

## Overview

You're tasked with completing Sprint 2 by implementing the remaining Tier 1 image formats: **BMP** and **GIF**. These are the final two formats needed to complete the core image converter functionality.

**Current Status:**
- ✅ PNG - Complete (excellent implementation)
- ✅ JPEG - Complete (excellent implementation)
- ❌ BMP - Not implemented (your task)
- ❌ GIF - Not implemented (your task)

---

## Task 1: Implement BMP Format Handler

**Priority:** HIGH  
**Estimated Time:** 2-3 days  
**Difficulty:** Medium (similar to PNG)

### Requirements

1. **Create the format handler:**
   - File: `img-core/src/formats/bmp.rs`
   - Follow the exact pattern from `png.rs` and `jpg.rs`
   - Implement `BmpFormat` struct with `new()` and `Default`

2. **Implement ImageReader trait:**
   - Read BMP files from bytes
   - Support RGB and RGBA color modes
   - Handle different bit depths (24-bit, 32-bit)
   - Use `image` crate's `ImageFormat::Bmp`
   - Map errors to `ConversionError::ConversionFailed`

3. **Implement ImageWriter trait:**
   - Write BMP files to bytes
   - Support RGB and RGBA output
   - Use `image` crate's `ImageFormat::Bmp`
   - Validate image data before writing

4. **Update module exports:**
   - Add `pub mod bmp;` to `img-core/src/formats/mod.rs`
   - Add `pub use bmp::BmpFormat;` to exports

5. **Update FormatRegistry:**
   - In `img-core/src/formats/registry.rs`
   - Update `get_reader()` to return `BmpFormat` for `ImageFormat::Bmp`
   - Update `get_writer()` to return `BmpFormat` for `ImageFormat::Bmp`

6. **Write comprehensive tests:**
   - Minimum 5 unit tests:
     - `test_bmp_read_rgb()` - Read RGB BMP
     - `test_bmp_read_rgba()` - Read RGBA BMP (if supported)
     - `test_bmp_write_rgb()` - Write RGB BMP
     - `test_bmp_write_rgba()` - Write RGBA BMP
     - `test_bmp_round_trip()` - Read → Write → Read
     - `test_bmp_read_invalid()` - Error handling
   - Follow the test pattern from `png.rs` tests

### Reference Code

Study these files:
- `img-core/src/formats/png.rs` - Excellent reference for structure
- `img-core/src/formats/jpg.rs` - Good reference for color conversion
- `img-core/src/formats/registry.rs` - See how PNG/JPEG are registered

### Implementation Notes

- BMP format is straightforward - similar complexity to PNG
- The `image` crate handles most of the complexity
- Focus on proper error handling and color type conversion
- Test with various BMP files (different bit depths)

### Acceptance Criteria

- ✅ BMP format handler created
- ✅ Can read BMP files (RGB, RGBA)
- ✅ Can write BMP files (RGB, RGBA)
- ✅ Registered in FormatRegistry
- ✅ All unit tests pass
- ✅ Integration test: BMP ↔ PNG conversion works
- ✅ Code follows existing patterns
- ✅ Error messages are clear

---

## Task 2: Implement GIF Format Handler

**Priority:** HIGH  
**Estimated Time:** 3-4 days  
**Difficulty:** Medium-High (transparency handling)

### Requirements

1. **Create the format handler:**
   - File: `img-core/src/formats/gif.rs`
   - Follow the exact pattern from `png.rs` and `jpg.rs`
   - Implement `GifFormat` struct with `new()` and `Default`

2. **Implement ImageReader trait:**
   - Read GIF files from bytes
   - Handle palette-based transparency
   - Extract first frame (animated GIF support can be Phase 2)
   - Convert palette-based transparency to RGBA
   - Use `image` crate's `ImageFormat::Gif`

3. **Implement ImageWriter trait:**
   - Write GIF files to bytes
   - Handle RGBA → palette conversion (if needed)
   - Support transparency
   - Use `image` crate's `ImageFormat::Gif`

4. **Update module exports:**
   - Add `pub mod gif;` to `img-core/src/formats/mod.rs`
   - Add `pub use gif::GifFormat;` to exports

5. **Update FormatRegistry:**
   - In `img-core/src/formats/registry.rs`
   - Update `get_reader()` to return `GifFormat` for `ImageFormat::Gif`
   - Update `get_writer()` to return `GifFormat` for `ImageFormat::Gif`

6. **Write comprehensive tests:**
   - Minimum 5 unit tests:
     - `test_gif_read()` - Read GIF file
     - `test_gif_read_transparent()` - Read GIF with transparency
     - `test_gif_write()` - Write GIF file
     - `test_gif_write_transparent()` - Write GIF with transparency
     - `test_gif_round_trip()` - Read → Write → Read
     - `test_gif_read_invalid()` - Error handling

### Reference Code

Study these files:
- `img-core/src/formats/png.rs` - Transparency handling reference
- `img-core/src/formats/jpg.rs` - Color conversion patterns
- `img-core/src/color.rs` - Color conversion utilities

### Implementation Notes

- GIF uses palette-based transparency (different from PNG's alpha channel)
- The `image` crate should handle most of this, but verify transparency is preserved
- For animated GIFs, extract first frame only (full animation support is Phase 2)
- Test with transparent GIFs to ensure alpha channel is handled correctly

### Questions to Ask

If you encounter issues:
- How should palette-based transparency be converted to RGBA?
- Should we support animated GIFs in Sprint 2, or defer to Phase 2?
- How should we handle GIF color palettes?

### Acceptance Criteria

- ✅ GIF format handler created
- ✅ Can read GIF files (with transparency)
- ✅ Can write GIF files (with transparency)
- ✅ Transparency preserved in conversions
- ✅ Registered in FormatRegistry
- ✅ All unit tests pass
- ✅ Integration test: GIF ↔ PNG conversion works
- ✅ Code follows existing patterns
- ✅ Error messages are clear

---

## Testing Requirements

### Unit Tests

For each format, create tests in the format file:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Your tests here
}
```

### Integration Tests

Add to `img-core/tests/integration.rs`:
- BMP ↔ PNG conversion
- BMP ↔ JPEG conversion
- GIF ↔ PNG conversion
- GIF ↔ JPEG conversion

### Test Files

You may need to create test BMP and GIF files, or use the `image` crate to generate them programmatically (like PNG/JPEG tests do).

---

## Code Quality Standards

Follow these patterns from existing code:

1. **Error Handling:**
   ```rust
   .map_err(|e| ConversionError::ConversionFailed(format!(
       "Failed to read BMP image ({} bytes): {}",
       data.len(),
       e
   )))?;
   ```

2. **Validation:**
   ```rust
   crate::validation::validate_image_data(image)?;
   ```

3. **Color Type Handling:**
   - Match on `image.color_type` and handle each case
   - Convert to appropriate format for the target

4. **Documentation:**
   - Add doc comments to public items
   - Follow existing documentation style

---

## Timeline

- **Day 1-2:** Implement BMP format
- **Day 3:** Test BMP, get code review
- **Day 4-6:** Implement GIF format
- **Day 7:** Test GIF, get code review
- **Day 8:** Integration testing, documentation updates

---

## Getting Help

**When to ask:**
- If you're stuck for more than 2 hours
- If you encounter unexpected behavior
- If you're unsure about the approach
- Before marking a task complete

**How to ask:**
- "I'm working on [task] and encountered [issue]. I've tried [approach]. Should I [option A] or [option B]?"

**Code Review:**
- Request review when BMP is complete
- Request review when GIF is complete
- Don't wait until both are done

---

## Definition of Done

Each format is complete when:

1. ✅ Format handler file created
2. ✅ `ImageReader` trait implemented
3. ✅ `ImageWriter` trait implemented
4. ✅ Registered in `FormatRegistry`
5. ✅ Exported in `formats/mod.rs`
6. ✅ Unit tests written (minimum 5 tests)
7. ✅ All tests passing
8. ✅ Integration tests added
9. ✅ Code review completed
10. ✅ Documentation updated

---

**Good luck! You've got excellent reference implementations to follow. Let's complete Sprint 2!**

*Jordan Rivera*  
*Senior Engineer*

