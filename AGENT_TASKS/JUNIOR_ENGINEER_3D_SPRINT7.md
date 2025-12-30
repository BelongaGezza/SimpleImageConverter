# Sprint 7 Task Assignment - Junior Engineer 3D (Alex Rivera)
## GUI Implementation for v0.2.1

**Agent:** Junior Engineer - 3D (Alex Rivera)  
**Role:** Supporting - Mesh Conversion Integration  
**Sprint Duration:** 2 weeks (Weeks 13-14)  
**Target Release:** v0.2.1

---

## Your Mission

You are supporting the GUI implementation, focusing on **mesh conversion integration**. Your expertise with the `mesh-core` library and 3D format handling is essential for making mesh conversion work seamlessly through the GUI.

---

## Required Reading (Before Starting)

1. **SPRINT_7_SUMMARY.md** - Executive briefing and sprint overview
2. **SPRINT_7_TASKING.md** - Complete detailed task breakdown (focus on your tasks)
3. **GUI_DESIGN_AND_IMPLEMENTATION.md** - GUI design specification
4. **Phase3_Architecture.md** - Architecture guidelines (mesh format sections)
5. **AI_DEVELOPMENT_GUIDE.md** - Team coordination guidelines
6. **rust-resources.md** - Check for mesh library updates

---

## Your Assigned Tasks

### Phase 2: Core UI Components (Days 4-7)

#### ✅ Task 2.2: Format Selection UI Component (Supporting)
**Priority:** Critical  
**Estimated:** 2 hours (your portion)  
**Status:** [x] Completed  
**Note:** Collaborate with UI Designer (Jamie Chen)

**What to Do:**
- Assist UI Designer with mesh format detection logic
- Provide format lists for mesh output formats
- Help implement format filtering (exclude STEP - read-only)
- Ensure format detection uses `mesh-core::FormatRegistry` correctly

**Reference:** SPRINT_7_TASKING.md lines 262-316

**Your Focus:**
- Mesh format detection integration
- Format filtering logic for meshes
- Format registry usage

**Format Lists:**
- Mesh Output: STL, OBJ, PLY, OFF, glTF, DXF (exclude STEP - read-only)

**Acceptance Criteria:**
- ✅ Mesh formats detected correctly
- ✅ Only writable mesh formats shown (exclude STEP)
- ✅ Format detection works for all mesh formats

---

#### ✅ Task 2.3: Options Panel Component (Supporting)
**Priority:** High  
**Estimated:** 3 hours (your portion)  
**Status:** [x] Completed  
**Note:** Collaborate with UI Designer (Jamie Chen)

**What to Do:**
- Assist with mesh-specific options UI
- Implement conversion options UI:
  - Transform: Radio buttons for "None", "Y-up", "Z-up", "Custom (from:to)"
  - Recalculate Normals: Checkbox
  - Validate: Checkbox
- Help with mesh-specific resource limits UI (max vertices, max faces)
- Validate conversion options

**Reference:** SPRINT_7_TASKING.md lines 319-364, 595-598

**Your Focus:**
- Mesh conversion options UI
- Mesh-specific resource limits (vertices, faces)
- Conversion options validation

**Acceptance Criteria:**
- ✅ Transform options UI functional
- ✅ Recalculate normals checkbox works
- ✅ Validate checkbox works
- ✅ Mesh-specific resource limits UI functional
- ✅ Options validated before conversion

---

### Phase 3: Conversion Integration (Days 8-11)

#### ✅ Task 3.1: Error Message Mapping (Supporting)
**Priority:** High  
**Estimated:** 2 hours (your portion)  
**Status:** [x] Completed  
**Note:** Collaborate with Junior Engineer 2D (Sam Kim)

**What to Do:**
- Map mesh-specific errors to user-friendly messages
- Handle mesh format errors (InvalidInput, UnsupportedFormat)
- Handle mesh validation errors
- Handle mesh resource limit errors (vertices, faces)

**Reference:** SPRINT_7_TASKING.md lines 418-467

**Your Focus:**
- Mesh conversion error mapping
- Mesh-specific error messages

**Error Examples:**
- "File type not supported." (for unsupported mesh formats)
- "Mesh too large. Maximum vertices is 10,000,000."
- "Mesh validation failed. Check if mesh is valid."

**Acceptance Criteria:**
- ✅ All mesh error types mapped to user-friendly messages
- ✅ No technical jargon in error messages
- ✅ Messages are concise and actionable

---

#### ✅ Task 3.3: Mesh Conversion Integration
**Priority:** Critical  
**Estimated:** 8 hours  
**Status:** [x] Completed

**What to Do:**
- Add mesh conversion function to `conversion.rs`
- **Direct library integration** with `mesh-core` (not subprocess calls)
- Format detection using `mesh-core::FormatRegistry`
- Resource limits enforcement (vertices, faces, file size)
- Support for conversion options (transform, validate, recalculate-normals)
- Error handling with user-friendly messages

**Reference:** SPRINT_7_TASKING.md lines 540-609

**Implementation Pattern:**
```rust
use mesh_core::{MeshConverter, FormatRegistry, MeshFormat, ConversionOptions, CoordinateSystem};
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;

pub fn convert_mesh(
    input_path: &Path,
    output_path: &Path,
    output_format: MeshFormat,
    options: ConversionOptions,
    limits: &ResourceLimits,
) -> Result<PathBuf> {
    // Validate input file
    common::validation::validate_file_path(input_path)?;
    
    // Read input file with size validation
    let input_data = read_file_bytes_checked(input_path, limits)?;
    
    // Format detection
    let input_format = FormatRegistry::detect_from_path(input_path)?;
    
    // Get format handlers with resource limits
    let reader = FormatRegistry::get_reader_with_limits(input_format, limits.clone())?;
    let writer = FormatRegistry::get_writer(output_format)?;
    
    // Convert with options
    let converter = MeshConverter::new();
    let output_data = converter.convert_with_options(
        &input_data,
        reader.as_ref(),
        writer.as_ref(),
        &options,
    )?;
    
    // Write output
    write_file_bytes(output_path, &output_data)?;
    
    Ok(output_path.to_path_buf())
}
```

**Conversion Options:**
- Transform: Coordinate system transform (Y-up ↔ Z-up)
- Recalculate Normals: Recalculate vertex normals
- Validate: Validate mesh integrity

**CRITICAL REQUIREMENTS:**
- ✅ **Direct library integration** - Use `mesh-core` directly, NOT subprocess calls
- ✅ Format detection works for all mesh formats
- ✅ Resource limits enforced (vertices, faces, file size)
- ✅ Conversion options (transform, validate, recalculate-normals) work
- ✅ All errors handled with user-friendly messages

**Acceptance Criteria:**
- ✅ Direct library integration (no subprocess calls)
- ✅ Format detection works for all mesh formats
- ✅ Resource limits enforced (vertices, faces, file size)
- ✅ Conversion options (transform, validate, recalculate-normals) work
- ✅ All errors handled with user-friendly messages
- ✅ Conversion works for all supported mesh formats
- ✅ Senior Engineer code review passed

---

## Key Requirements

### Direct Library Integration (CRITICAL)
- ✅ **MUST:** Use `mesh-core` library directly
- ❌ **MUST NOT:** Call `mesh-convert` CLI binary as subprocess
- **Why:** Security, performance, architecture compliance

### Security First
- ✅ Format detection using `mesh-core::FormatRegistry`
- ✅ File size validation before reading
- ✅ Resource limits enforced (vertices, faces, file size)
- ✅ Path validation on all file operations

### User-Friendly Error Messages
- ✅ No technical jargon
- ✅ Clear, actionable messages
- ✅ No path or system information leaked

---

## Collaboration Points

### With UI Designer (Jamie Chen)
- Format selection UI (Task 2.2)
- Options panel UI (Task 2.3) - mesh options
- Integration of your conversion function into GUI (Task 4.1)

### With Junior Engineer - 2D (Sam Kim)
- Error message mapping (Task 3.1) - share patterns
- Conversion function structure (Task 3.2 vs 3.3) - consistency

### With Senior Engineer (Jordan Rivera)
- Code review for all your work
- Questions about `mesh-core` API usage
- Architecture compliance questions
- Conversion options implementation guidance

### With Security Specialist (Casey Morgan)
- Security validation review
- Resource limits verification

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
- [ ] Conversion options tested

---

## Testing Requirements

### Unit Tests
- [ ] Test error message mapping function
- [ ] Test mesh conversion function
- [ ] Test format detection logic
- [ ] Test conversion options (transform, validate, recalculate-normals)

### Integration Tests
- [ ] Test direct library integration (mesh-core)
- [ ] Test format registry integration
- [ ] Test resource limits integration
- [ ] Test all supported mesh formats
- [ ] Test conversion options combinations

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
- ✅ Mesh conversion works through GUI
- ✅ All mesh formats supported
- ✅ Conversion options functional (transform, validate, recalculate-normals)
- ✅ Error messages user-friendly

### Technical
- ✅ Direct library integration (no subprocess calls)
- ✅ Format detection works for all mesh formats
- ✅ Resource limits enforced
- ✅ Code reviewed and approved by Senior Engineer

---

**You've got this! Follow the patterns established in `mesh-core` and ask questions when needed.**

**Document Version:** 1.0  
**Created:** December 2025  
**Status:** Ready for Implementation

