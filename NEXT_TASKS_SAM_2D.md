# Task Assignment: Sam Parker (Junior Engineer - 2D Formats)
## Sprint 4: Advanced 2D Formats

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Sprint Status:** Sprint 2 ✅ Complete | Sprint 3 ✅ Complete | **Sprint 4 - Ready to Begin**  
**Priority:** 🔴 **HIGH - Sprint 4 Implementation**

---

## 🎉 Congratulations!

**Excellent work on Sprint 2!** Your implementations of PNG, JPEG, BMP, and GIF formats are production-ready and demonstrate excellent code quality. All tests passing, no issues found in code review.

Sprint 3 (Mesh Core) is now complete, so we're ready to move forward with Sprint 4!

---

## Current Status

**Completed Sprints:**
- ✅ **Sprint 2:** PNG, JPEG, BMP, GIF formats (production-ready)
- ✅ **Sprint 3:** STL, OBJ, PLY formats (Riley's work - complete)

**Current Sprint:** **Sprint 4 - Advanced 2D Formats** - **YOUR TASK**

---

## Sprint 4 Overview

**Goal:** Add Tier 2 image formats and advanced features

**Duration:** 2 weeks (14 days)  
**Focus:** TIFF, WebP, SVG (rasterization), and optional Tier 2 formats

---

## Task 1: Implement TIFF Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 3-4 days  
**Difficulty:** Medium (multi-page support adds complexity)

### Requirements

1. **Create the format handler:**
   - File: `img-core/src/formats/tiff.rs`
   - Follow the exact pattern from `png.rs` and `jpg.rs`
   - Implement `TiffFormat` struct
   - Implement `ImageReader` trait
   - Implement `ImageWriter` trait

2. **Use `image` crate:**
   - The `image` crate already supports TIFF
   - Use `image::open()` for reading
   - Use `image::save()` for writing
   - Handle multi-page TIFF files (read first page, write single page)

3. **Handle TIFF-specific features:**
   - Multi-page TIFF (read first page only for now)
   - Compression options (LZW, Deflate) - use image crate defaults
   - Color modes (RGB, RGBA, Grayscale)
   - Bit depth support (8-bit, 16-bit)

4. **Error handling:**
   - Invalid TIFF structure
   - Unsupported compression
   - Corrupted files
   - Empty files

5. **Write implementation:**
   - Write single-page TIFF files
   - Support common color modes
   - Use appropriate compression

### Implementation Pattern

Follow the PNG/JPEG pattern exactly:

```rust
pub struct TiffFormat;

impl TiffFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TiffFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageReader for TiffFormat {
    fn read(&self, path: &Path) -> Result<Image, FormatError> {
        // Validate path
        self.validate_path(path)?;
        
        // Use image crate to read TIFF
        let img = image::open(path)
            .map_err(|e| FormatError::ReadError(e.to_string()))?;
        
        Ok(Image::from_dynamic(img))
    }
}

impl ImageWriter for TiffFormat {
    fn write(&self, image: &Image, path: &Path, options: &WriteOptions) -> Result<(), FormatError> {
        // Validate path
        self.validate_path(path)?;
        
        // Convert to DynamicImage and save
        let dynamic = image.to_dynamic_image()?;
        dynamic.save(path)
            .map_err(|e| FormatError::WriteError(e.to_string()))?;
        
        Ok(())
    }
}
```

### Testing Requirements

Write comprehensive tests (aim for 10-12 tests):

1. **Unit Tests:**
   - `test_tiff_format_new`
   - `test_read_rgb_tiff`
   - `test_read_rgba_tiff`
   - `test_read_grayscale_tiff`
   - `test_read_multi_page_tiff` (first page only)
   - `test_read_invalid_tiff`
   - `test_read_empty_file`
   - `test_write_rgb_tiff`
   - `test_write_rgba_tiff`
   - `test_write_grayscale_tiff`
   - `test_round_trip_rgb`
   - `test_round_trip_rgba`

2. **Integration Tests:**
   - Add to `img-core/tests/integration.rs`
   - `test_tiff_round_trip_conversion`
   - `test_image_converter_tiff_round_trip`

### Success Criteria
- ✅ TIFF format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Registered in format registry
- ✅ Follows PNG/JPEG pattern exactly
- ✅ No linter errors
- ✅ Documentation complete

---

## Task 2: Implement WebP Format Handler

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 2-3 days  
**Difficulty:** Medium (lossy/lossless modes)

### Requirements

1. **Create the format handler:**
   - File: `img-core/src/formats/webp.rs`
   - Follow the exact pattern from other formats
   - Implement `WebPFormat` struct
   - Implement `ImageReader` trait
   - Implement `ImageWriter` trait

2. **Use `image` crate:**
   - The `image` crate supports WebP
   - Use `image::open()` for reading
   - Use `image::save_with_format()` for writing
   - Handle quality settings via `WriteOptions`

3. **Handle WebP-specific features:**
   - Lossy compression (default)
   - Lossless compression (optional)
   - Quality settings (0-100)
   - Transparency support (RGBA)

4. **Error handling:**
   - Invalid WebP structure
   - Corrupted files
   - Quality parameter validation

5. **Write implementation:**
   - Support quality settings from `WriteOptions`
   - Handle transparency correctly
   - Use appropriate compression mode

### Implementation Pattern

Follow the JPEG pattern (similar quality handling):

```rust
pub struct WebPFormat;

impl WebPFormat {
    pub fn new() -> Self {
        Self
    }
}

impl ImageWriter for WebPFormat {
    fn write(&self, image: &Image, path: &Path, options: &WriteOptions) -> Result<(), FormatError> {
        self.validate_path(path)?;
        
        let dynamic = image.to_dynamic_image()?;
        
        // WebP quality is 0-100, similar to JPEG
        let quality = options.quality.unwrap_or(90);
        
        // Save with quality setting
        dynamic.save_with_format(path, image::ImageFormat::WebP)
            .map_err(|e| FormatError::WriteError(e.to_string()))?;
        
        Ok(())
    }
}
```

### Testing Requirements

Write comprehensive tests (aim for 10+ tests):

1. **Unit Tests:**
   - `test_webp_format_new`
   - `test_read_rgb_webp`
   - `test_read_rgba_webp` (transparency)
   - `test_read_lossless_webp`
   - `test_read_invalid_webp`
   - `test_write_rgb_webp`
   - `test_write_rgba_webp`
   - `test_write_with_quality`
   - `test_round_trip_rgb`
   - `test_round_trip_rgba`

2. **Integration Tests:**
   - Add to `img-core/tests/integration.rs`
   - `test_webp_round_trip_conversion`

### Success Criteria
- ✅ WebP format handler implemented
- ✅ 10+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Quality settings functional
- ✅ Registered in format registry
- ✅ Follows established pattern
- ✅ No linter errors

---

## Task 3: Implement SVG Rasterization (Read-Only)

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 3-4 days  
**Difficulty:** Medium-High (new dependency, rasterization logic)

### Requirements

1. **Create the format handler:**
   - File: `img-core/src/formats/svg.rs`
   - Implement `SvgFormat` struct
   - Implement `ImageReader` trait (read-only)
   - **DO NOT** implement `ImageWriter` (SVG is vector, not raster)

2. **Use `resvg` crate:**
   - Add `resvg = "0.40"` to `img-core/Cargo.toml`
   - Use `resvg` to rasterize SVG to bitmap
   - Handle DPI/resolution settings
   - Default to 96 DPI (standard screen resolution)

3. **Handle SVG-specific features:**
   - Rasterize to RGB or RGBA (based on SVG content)
   - Configurable output size (via DPI or explicit dimensions)
   - Handle embedded images
   - Handle text rendering

4. **Error handling:**
   - Invalid SVG syntax
   - Unsupported SVG features
   - File read errors
   - Rasterization failures

5. **Implementation notes:**
   - SVG is **read-only** (we rasterize SVG → bitmap)
   - Cannot write SVG (that would be vector graphics generation)
   - Output size can be controlled via DPI setting

### Implementation Pattern

```rust
pub struct SvgFormat;

impl SvgFormat {
    pub fn new() -> Self {
        Self
    }
    
    /// Rasterize SVG with specified DPI
    fn rasterize(&self, data: &[u8], dpi: f32) -> Result<DynamicImage, FormatError> {
        use resvg::prelude::*;
        
        let opt = resvg::Options::default();
        let tree = resvg::Tree::from_data(data, &opt)
            .map_err(|e| FormatError::ReadError(format!("SVG parse error: {}", e)))?;
        
        let size = tree.size();
        let pixmap_size = size.to_int_size();
        
        let mut pixmap = tiny_skia::Pixmap::new(
            pixmap_size.width(),
            pixmap_size.height()
        ).ok_or_else(|| FormatError::ReadError("Failed to create pixmap".to_string()))?;
        
        resvg::render(&tree, resvg::FitTo::Original, pixmap.as_mut())
            .ok_or_else(|| FormatError::ReadError("Failed to render SVG".to_string()))?;
        
        // Convert pixmap to DynamicImage
        // ... conversion logic ...
    }
}

impl ImageReader for SvgFormat {
    fn read(&self, path: &Path) -> Result<Image, FormatError> {
        self.validate_path(path)?;
        
        let data = std::fs::read(path)
            .map_err(|e| FormatError::ReadError(e.to_string()))?;
        
        // Rasterize at 96 DPI (standard)
        let dynamic = self.rasterize(&data, 96.0)?;
        
        Ok(Image::from_dynamic(dynamic))
    }
}

// NO ImageWriter implementation - SVG is read-only
```

### Dependencies to Add

Add to `img-core/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
resvg = "0.40"  # For SVG rasterization
tiny-skia = "0.11"  # Required by resvg
```

### Testing Requirements

Write comprehensive tests (aim for 8-10 tests):

1. **Unit Tests:**
   - `test_svg_format_new`
   - `test_read_simple_svg`
   - `test_read_svg_with_transparency`
   - `test_read_svg_with_text`
   - `test_read_invalid_svg`
   - `test_read_empty_file`
   - `test_rasterize_at_different_dpi`
   - `test_svg_to_png_conversion` (integration)

2. **Integration Tests:**
   - Add to `img-core/tests/integration.rs`
   - `test_svg_to_raster_conversion`

### Success Criteria
- ✅ SVG format handler implemented (read-only)
- ✅ 8+ unit tests (all passing)
- ✅ Integration tests added
- ✅ Registered in format registry
- ✅ Rasterization working correctly
- ✅ No linter errors
- ✅ Documentation notes SVG is read-only

---

## Task 4: Update Format Registry

**Priority:** 🔴 **HIGH**  
**Estimated Time:** 1 hour  
**Difficulty:** Easy

### Requirements

1. **Update `img-core/src/formats/registry.rs`:**
   - Add TIFF to `ImageFormat` enum
   - Add WebP to `ImageFormat` enum
   - Add SVG to `ImageFormat` enum
   - Add format detection logic (magic bytes)
   - Add to `get_reader()` method
   - Add to `get_writer()` method (TIFF, WebP only - no SVG writer)

2. **Update format detection:**
   - TIFF magic bytes: `49 49 2A 00` (little-endian) or `4D 4D 00 2A` (big-endian)
   - WebP magic bytes: `52 49 46 46 ?? ?? ?? ?? 57 45 42 50` (RIFF...WEBP)
   - SVG detection: Check for `<?xml` or `<svg` at start

3. **Update tests:**
   - Add tests for format detection
   - Add tests for get_reader/get_writer

### Code Changes

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
    Gif,
    Tiff,  // ADD THIS
    WebP,  // ADD THIS
    Svg,   // ADD THIS (read-only)
}

pub fn get_reader(format: ImageFormat) -> Result<Box<dyn ImageReader>> {
    match format {
        ImageFormat::Png => Ok(Box::new(PngFormat::new())),
        ImageFormat::Jpeg => Ok(Box::new(JpegFormat::new())),
        ImageFormat::Bmp => Ok(Box::new(BmpFormat::new())),
        ImageFormat::Gif => Ok(Box::new(GifFormat::new())),
        ImageFormat::Tiff => Ok(Box::new(TiffFormat::new())),  // ADD THIS
        ImageFormat::WebP => Ok(Box::new(WebPFormat::new())),   // ADD THIS
        ImageFormat::Svg => Ok(Box::new(SvgFormat::new())),    // ADD THIS
    }
}

pub fn get_writer(format: ImageFormat) -> Result<Box<dyn ImageWriter>> {
    match format {
        ImageFormat::Png => Ok(Box::new(PngFormat::new())),
        ImageFormat::Jpeg => Ok(Box::new(JpegFormat::new())),
        ImageFormat::Bmp => Ok(Box::new(BmpFormat::new())),
        ImageFormat::Gif => Ok(Box::new(GifFormat::new())),
        ImageFormat::Tiff => Ok(Box::new(TiffFormat::new())),   // ADD THIS
        ImageFormat::WebP => Ok(Box::new(WebPFormat::new())),   // ADD THIS
        // SVG is read-only, no writer
        ImageFormat::Svg => Err(FormatError::UnsupportedOperation(
            "SVG is a vector format and cannot be written as raster".to_string()
        )),
    }
}
```

### Success Criteria
- ✅ Registry updated with TIFF, WebP, SVG
- ✅ Format detection working
- ✅ All registry tests pass
- ✅ No regressions

---

## Task 5: Optional - Tier 2 Formats (If Time Permits)

**Priority:** 🟡 **MEDIUM** (Optional)  
**Estimated Time:** 2-3 days per format  
**Difficulty:** Easy (all use `image` crate)

### Optional Formats

If you complete Tasks 1-4 ahead of schedule, you can implement:

1. **TGA Format** (Targa)
   - File: `img-core/src/formats/tga.rs`
   - Uses `image` crate
   - Follow same pattern

2. **ICO Format** (Windows Icon)
   - File: `img-core/src/formats/ico.rs`
   - Uses `image` crate
   - Handle multiple icon sizes

3. **HDR Format** (Radiance HDR)
   - File: `img-core/src/formats/hdr.rs`
   - Uses `image` crate
   - High dynamic range support

**Note:** These are optional. Focus on TIFF, WebP, and SVG first!

---

## Task 6: Update Documentation

**Priority:** 🟡 **MEDIUM**  
**Estimated Time:** 1 hour  
**Difficulty:** Easy

### Requirements

1. **Update `docs/FORMATS.md`:**
   - Mark TIFF as ✅ implemented
   - Mark WebP as ✅ implemented
   - Mark SVG as ✅ implemented (read-only)
   - Update Sprint 4 status

2. **Update code documentation:**
   - Ensure all public APIs documented
   - Add examples if needed
   - Note SVG is read-only

### Success Criteria
- ✅ FORMATS.md updated
- ✅ All docs accurate
- ✅ Examples work

---

## Dependencies to Add

Add these to `img-core/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
resvg = "0.40"      # For SVG rasterization
tiny-skia = "0.11"  # Required by resvg
```

**Note:** TIFF and WebP are already supported by the `image` crate (no additional dependencies needed).

---

## Implementation Checklist

### TIFF Format
- [ ] Create `tiff.rs` file
- [ ] Implement `TiffFormat` struct
- [ ] Implement `ImageReader` for TIFF
- [ ] Implement `ImageWriter` for TIFF
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### WebP Format
- [ ] Create `webp.rs` file
- [ ] Implement `WebPFormat` struct
- [ ] Implement `ImageReader` for WebP
- [ ] Implement `ImageWriter` for WebP
- [ ] Write 10+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry
- [ ] Update documentation

### SVG Format (Read-Only)
- [ ] Add `resvg` and `tiny-skia` dependencies
- [ ] Create `svg.rs` file
- [ ] Implement `SvgFormat` struct
- [ ] Implement `ImageReader` for SVG (rasterization)
- [ ] Write 8+ unit tests
- [ ] Add integration tests
- [ ] Register in format registry (read-only)
- [ ] Update documentation (note read-only)

### Format Registry
- [ ] Update `ImageFormat` enum
- [ ] Add format detection logic
- [ ] Update `get_reader()` method
- [ ] Update `get_writer()` method
- [ ] Add registry tests

### Documentation
- [ ] Update `docs/FORMATS.md`
- [ ] Update code documentation
- [ ] Verify all examples work

---

## Code Quality Standards

### ✅ Do's
- Follow PNG/JPEG/BMP/GIF pattern exactly
- Write comprehensive tests (10+ per format)
- Include proper error handling
- Document public APIs
- Use descriptive error messages
- Validate inputs thoroughly
- Test edge cases (empty files, invalid data, etc.)
- Note SVG is read-only in documentation

### ❌ Don'ts
- Don't skip tests
- Don't ignore edge cases
- Don't use unsafe code
- Don't copy-paste without understanding
- Don't commit without testing
- Don't forget to register in format registry
- Don't try to implement SVG writer (it's vector, not raster)

---

## Reference Materials

1. **Existing Format Implementations:**
   - `img-core/src/formats/png.rs` - Reference pattern
   - `img-core/src/formats/jpg.rs` - Quality handling example
   - `img-core/src/formats/bmp.rs` - Your excellent work
   - `img-core/src/formats/gif.rs` - Your excellent work

2. **Documentation:**
   - `docs/ARCHITECTURE.md`
   - `docs/FORMATS.md`
   - `Phase3_Architecture.md`

3. **Library Documentation:**
   - `image` crate: https://docs.rs/image/
   - `resvg` crate: https://docs.rs/resvg/

---

## Timeline

| Task | Duration | Start | End |
|------|----------|-------|-----|
| TIFF Format | 3-4 days | Day 1 | Day 4 |
| WebP Format | 2-3 days | Day 5 | Day 7 |
| SVG Format | 3-4 days | Day 8 | Day 11 |
| Registry Update | 1 hour | Day 12 | Day 12 |
| Documentation | 1 hour | Day 13 | Day 13 |
| Testing & Polish | 1 day | Day 14 | Day 14 |

**Total Estimated Time:** 14 days (2 weeks)

---

## Questions & Support

If you have questions:

1. **Check Existing Implementations:**
   - Your PNG/JPEG/BMP/GIF code is excellent reference
   - Follow the same patterns

2. **Check Documentation:**
   - `docs/ARCHITECTURE.md`
   - `docs/FORMATS.md`
   - `Phase3_Architecture.md`

3. **Ask for Help:**
   - Senior Engineer (Jordan) available
   - Code review available
   - Pair programming if needed

---

## Success Metrics

**Sprint 4 Completion:**
- ✅ TIFF format implemented and tested
- ✅ WebP format implemented and tested
- ✅ SVG rasterization implemented and tested
- ✅ All tests passing (target: 30+ new image tests)
- ✅ Documentation updated
- ✅ Code review approved

**Overall:**
- ✅ Sprint 4 marked complete
- ✅ Ready for Sprint 6 (Polish & Testing)
- ✅ Advanced formats foundation solid

---

## Final Notes

**Great work on Sprint 2!** Your implementations are excellent and serve as perfect references for Sprint 4.

**Focus Areas:**
1. Follow the established patterns exactly
2. Write comprehensive tests
3. Handle errors properly
4. Don't skip edge cases
5. Remember SVG is read-only (rasterization only)

**Remember:** Quality over speed. Take time to do things right. Your Sprint 2 work shows you can do this!

---

**Assigned by:** Jordan Rivera (Senior Engineer)  
**Date:** January 27, 2025  
**Status:** Ready to begin  
**Priority:** 🔴 HIGH - Sprint 4 Implementation
