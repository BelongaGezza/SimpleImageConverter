# Sprint 7 Task Assignment - Junior Engineer 2D (Sam Kim)
## GUI Implementation for v0.2.1

**Agent:** Junior Engineer - 2D (Sam Kim)  
**Role:** Supporting - Image Conversion Integration  
**Sprint Duration:** 2 weeks (Weeks 13-14)  
**Target Release:** v0.2.1

---

## Your Mission

You are supporting the GUI implementation, focusing on **image conversion integration**. Your expertise with the `img-core` library and image format handling is essential for making image conversion work seamlessly through the GUI.

---

## Required Reading (Before Starting)

1. **SPRINT_7_SUMMARY.md** - Executive briefing and sprint overview
2. **SPRINT_7_TASKING.md** - Complete detailed task breakdown (focus on your tasks)
3. **GUI_DESIGN_AND_IMPLEMENTATION.md** - GUI design specification
4. **Phase3_Architecture.md** - Architecture guidelines (image format sections)
5. **AI_DEVELOPMENT_GUIDE.md** - Team coordination guidelines
6. **rust-resources.md** - Check for image crate updates

---

## Your Assigned Tasks

### Phase 2: Core UI Components (Days 4-7)

#### ✅ Task 2.2: Format Selection UI Component (Supporting)
**Priority:** Critical  
**Estimated:** 2 hours (your portion)  
**Status:** [x] Completed  
**Note:** Collaborate with UI Designer (Jamie Chen)

**What to Do:**
- Assist UI Designer with image format detection logic
- Provide format lists for image output formats
- Help implement format filtering (exclude SVG - read-only)
- Ensure format detection uses `img-core::FormatRegistry` correctly

**Reference:** SPRINT_7_TASKING.md lines 262-316

**Your Focus:**
- Image format detection integration
- Format filtering logic for images
- Format registry usage

**Acceptance Criteria:**
- ✅ Image formats detected correctly
- ✅ Only writable image formats shown (exclude SVG)
- ✅ Format detection uses two-stage validation (extension + magic bytes)

---

#### ✅ Task 2.3: Options Panel Component (Supporting)
**Priority:** High  
**Estimated:** 3 hours (your portion)  
**Status:** [x] Completed  
**Note:** Collaborate with UI Designer (Jamie Chen)

**What to Do:**
- Assist with quality slider implementation
- Ensure quality slider shows/hides based on format (JPEG, WebP only)
- Help with image-specific resource limits UI (max dimension)
- Validate quality value range (1-100)

**Reference:** SPRINT_7_TASKING.md lines 319-364

**Your Focus:**
- Quality settings UI for images
- Image-specific options (max dimension)
- Quality validation logic

**Acceptance Criteria:**
- ✅ Quality slider visible only for lossy formats (JPEG, WebP)
- ✅ Quality value validated (1-100)
- ✅ Image-specific resource limits UI functional

---

### Phase 3: Conversion Integration (Days 8-11)

#### ✅ Task 3.1: Error Message Mapping (Supporting)
**Priority:** High  
**Estimated:** 2 hours (your portion)  
**Status:** [x] Completed  
**Note:** Collaborate with Junior Engineer 3D (Alex Rivera)

**What to Do:**
- Map image-specific errors to user-friendly messages
- Handle image format errors (InvalidInput, UnsupportedFormat)
- Handle image dimension errors
- Handle image quality errors

**Reference:** SPRINT_7_TASKING.md lines 418-467

**Your Focus:**
- Image conversion error mapping
- Image-specific error messages

**Error Examples:**
- "File type not supported." (for unsupported image formats)
- "Image too large. Maximum dimension is 65535 pixels."
- "Quality must be between 1 and 100."

**Acceptance Criteria:**
- ✅ All image error types mapped to user-friendly messages
- ✅ No technical jargon in error messages
- ✅ Messages are concise and actionable

---

#### ✅ Task 3.2: Image Conversion Integration
**Priority:** Critical  
**Estimated:** 8 hours  
**Status:** [x] Completed

**What to Do:**
- Create `conversion.rs` module with image conversion function
- **Direct library integration** with `img-core` (not subprocess calls)
- Two-stage format detection (extension + magic bytes)
- Resource limits enforcement using `common::limits::ResourceLimits`
- Error handling with user-friendly messages
- Thread-safe conversion state for progress tracking

**Reference:** SPRINT_7_TASKING.md lines 471-536

**Implementation Pattern:**
```rust
use img_core::{ImageConverter, FormatRegistry, ImageFormat, QualitySettings};
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;

pub fn convert_image(
    input_path: &Path,
    output_path: &Path,
    output_format: ImageFormat,
    quality: u8,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // Validate input file
    common::validation::validate_file_path(input_path)?;
    
    // Read input file with size validation
    let input_data = read_file_bytes_checked(input_path, limits)?;
    
    // Two-stage format detection (security)
    let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;
    
    // Get format handlers
    let reader = FormatRegistry::get_reader(input_format)?;
    let writer = FormatRegistry::get_writer(output_format)?;
    
    // Convert
    let converter = ImageConverter::new();
    let quality_settings = QualitySettings::new(quality);
    let output_data = converter.convert(
        &input_data,
        reader.as_ref(),
        writer.as_ref(),
        &quality_settings,
    )?;
    
    // Write output
    write_file_bytes(output_path, &output_data)?;
    
    Ok(output_path.to_path_buf())
}
```

**CRITICAL REQUIREMENTS:**
- ✅ **Direct library integration** - Use `img-core` directly, NOT subprocess calls
- ✅ Two-stage format detection (extension + magic bytes)
- ✅ Resource limits enforced
- ✅ All errors handled with user-friendly messages

**Acceptance Criteria:**
- ✅ Direct library integration (no subprocess calls)
- ✅ Two-stage format detection implemented
- ✅ Resource limits enforced
- ✅ All errors handled with user-friendly messages
- ✅ Conversion works for all supported image formats
- ✅ Quality settings applied correctly
- ✅ Senior Engineer code review passed

---

## Key Requirements

### Direct Library Integration (CRITICAL)
- ✅ **MUST:** Use `img-core` library directly
- ❌ **MUST NOT:** Call `img-convert` CLI binary as subprocess
- **Why:** Security, performance, architecture compliance

### Security First
- ✅ Two-stage format detection (extension + magic bytes)
- ✅ File size validation before reading
- ✅ Resource limits enforced
- ✅ Path validation on all file operations

### User-Friendly Error Messages
- ✅ No technical jargon
- ✅ Clear, actionable messages
- ✅ No path or system information leaked

---

## Collaboration Points

### With UI Designer (Jamie Chen)
- Format selection UI (Task 2.2)
- Options panel UI (Task 2.3)
- Integration of your conversion function into GUI (Task 4.1)

### With Junior Engineer - 3D (Alex Rivera)
- Error message mapping (Task 3.1) - share patterns
- Conversion function structure (Task 3.2 vs 3.3) - consistency

### With Senior Engineer (Jordan Rivera)
- Code review for all your work
- Questions about `img-core` API usage
- Architecture compliance questions

### With Security Specialist (Casey Morgan)
- Security validation review
- Two-stage format detection verification

---

## Code Review Checklist

Before submitting code for review:
- [ ] Follows architecture design
- [ ] Passes `cargo test`
- [ ] Passes `cargo clippy`
- [ ] Formatted with `cargo fmt`
- [ ] Documentation comments added
- [ ] Error handling implemented
- [ ] Tests written and passing
- [ ] No compilation warnings
- [ ] Direct library integration (no subprocess calls)
- [ ] Security validations implemented

---

## Testing Requirements

### Unit Tests
- [x] Test error message mapping function
- [x] Test image conversion function
- [x] Test format detection logic
- [x] Test quality validation

### Integration Tests
- [x] Test direct library integration (img-core)
- [x] Test format registry integration
- [x] Test resource limits integration
- [x] Test all supported image formats (covered by img-core integration tests)

---

## Daily Standup Questions

Be prepared to answer:
1. What did I complete yesterday?
2. What am I working on today?
3. Any blockers?
4. Any questions for Senior Engineer?

---

## Questions or Help Needed?

**Contact:**
- Senior Engineer (Jordan Rivera) - Technical questions, code reviews, mentoring
- UI Designer (Jamie Chen) - UI integration questions
- Security Specialist (Casey Morgan) - Security validation questions

**Reference Documents:**
- Detailed tasking: `SPRINT_7_TASKING.md`
- GUI design: `GUI_DESIGN_AND_IMPLEMENTATION.md`
- Architecture: `Phase3_Architecture.md`

---

## Success Criteria

### Functional
- ✅ Image conversion works through GUI
- ✅ All image formats supported
- ✅ Quality settings functional
- ✅ Error messages user-friendly

### Technical
- ✅ Direct library integration (no subprocess calls)
- ✅ Two-stage format detection
- ✅ Resource limits enforced
- ✅ Code reviewed and approved by Senior Engineer

---

**You've got this! Follow the patterns established in `img-core` and ask questions when needed.**

**Document Version:** 1.0  
**Created:** December 2025  
**Status:** Ready for Implementation

