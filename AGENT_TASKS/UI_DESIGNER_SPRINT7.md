# Sprint 7 Task Assignment - UI Designer (Jamie Chen)
## GUI Implementation for v0.2.1

**Agent:** UI Designer (Jamie Chen)  
**Role:** Primary Lead for Sprint 7 GUI Implementation  
**Sprint Duration:** 2 weeks (Weeks 13-14)  
**Target Release:** v0.2.1

---

## Your Mission

You are the **primary lead** for Sprint 7 GUI implementation. Your expertise in egui framework and user experience design is critical to delivering a functional, intuitive GUI application for v0.2.1.

---

## Required Reading (Before Starting)

1. **SPRINT_7_SUMMARY.md** - Executive briefing and sprint overview
2. **SPRINT_7_TASKING.md** - Complete detailed task breakdown
3. **GUI_DESIGN_AND_IMPLEMENTATION.md** - Complete GUI design specification
4. **Phase3_Architecture.md** - Architecture guidelines (GUI section)
5. **AI_DEVELOPMENT_GUIDE.md** - Team coordination guidelines
6. **rust-resources.md** - Check for egui/eframe updates and best practices

---

## Your Assigned Tasks

### Phase 1: Project Setup & Foundation (Days 1-3)

#### ✅ Task 1.1: Create converter-gui Crate
**Priority:** Critical  
**Estimated:** 4 hours  
**Status:** [ ] Not Started

**What to Do:**
- Create `converter-gui/` directory in workspace root
- Create `converter-gui/Cargo.toml` with dependencies (eframe 0.27, egui 0.27, rfd 0.14)
- Add `converter-gui` to workspace `Cargo.toml` members
- Initialize `converter-gui/src/main.rs` with eframe entry point
- Verify workspace builds: `cargo build --workspace`

**Reference:** SPRINT_7_TASKING.md lines 53-84

**Acceptance Criteria:**
- ✅ Workspace compiles without errors
- ✅ `converter-gui` crate visible in workspace
- ✅ Can run `cargo run --bin converter-gui` (even if window is empty)

---

#### ✅ Task 1.2: Basic egui Window Setup
**Priority:** Critical  
**Estimated:** 6 hours  
**Status:** [ ] Not Started

**What to Do:**
- Implement `eframe::App` trait for main application struct
- Create basic window with title "Simple Image Converter"
- Set minimum window size (800x600)
- Implement window resize handling
- Add basic menu bar (File, Edit, Help - stubs for now)
- Verify window launches and displays correctly

**Reference:** SPRINT_7_TASKING.md lines 88-140

**Code Structure:** See SPRINT_7_TASKING.md for complete code examples

**Acceptance Criteria:**
- ✅ Window launches with correct title
- ✅ Window is resizable (minimum 800x600 enforced)
- ✅ Menu bar displays (File, Edit, Help)
- ✅ No crashes or errors

---

#### ✅ Task 1.3: Application State Structure
**Priority:** Critical  
**Estimated:** 4 hours  
**Status:** [ ] Not Started  
**Note:** Requires Senior Engineer review

**What to Do:**
- Design application state structure
- Implement state for file selection, format selection, conversion options, UI feedback
- Use `Arc<Mutex<>>` for thread-safe state sharing

**Reference:** SPRINT_7_TASKING.md lines 144-200

**Acceptance Criteria:**
- ✅ State structure defined and documented
- ✅ Default implementations for all state fields
- ✅ Thread-safe patterns used where needed
- ✅ State persists across UI updates
- ✅ Senior Engineer review completed

---

### Phase 2: Core UI Components (Days 4-7)

#### ✅ Task 2.1: File Drop Zone Component
**Priority:** Critical  
**Estimated:** 8 hours  
**Status:** [ ] Not Started

**What to Do:**
- Create `drop_zone.rs` UI component module
- Implement large drop zone area (minimum 200px height)
- Implement drag-and-drop file handling using `egui::DragAndDrop` API
- Implement click-to-browse using `rfd::FileDialog`
- Visual feedback for drag-over state
- Display selected file name and path
- Handle file type detection (image vs mesh)
- Security: Validate file path using `common::validation::validate_file_path()`

**Reference:** SPRINT_7_TASKING.md lines 206-258

**Visual States:**
- Empty: Light gray background, dashed border
- Drag Over: Blue border, light blue background
- File Selected: Green border, show file name
- Error: Red border, show error message

**Acceptance Criteria:**
- ✅ Drop zone visually distinct and large enough
- ✅ Drag-and-drop accepts files
- ✅ Click opens file browser
- ✅ Selected file name displays correctly
- ✅ File type detection works for images and meshes
- ✅ Error states display user-friendly messages
- ✅ Path validation prevents security issues

---

#### ✅ Task 2.2: Format Selection UI Component
**Priority:** Critical  
**Estimated:** 6 hours  
**Status:** [ ] Not Started  
**Note:** Collaborate with Junior Engineers

**What to Do:**
- Create `format_selector.rs` UI component module
- Implement radio button group for format selection
- Filter formats based on detected file type (image vs mesh)
- Exclude read-only formats from output options (SVG, STEP)
- Default to first available format alphabetically
- Update output filename extension when format changes

**Reference:** SPRINT_7_TASKING.md lines 262-316

**Format Lists:**
- Image Output: PNG, JPEG, BMP, GIF, TIFF, WebP (exclude SVG)
- Mesh Output: STL, OBJ, PLY, OFF, glTF, DXF (exclude STEP)

**Acceptance Criteria:**
- ✅ Radio buttons display correctly
- ✅ Only compatible formats shown
- ✅ Read-only formats excluded
- ✅ Format change updates output filename extension
- ✅ Default format selected automatically

---

#### ✅ Task 2.3: Options Panel Component
**Priority:** High  
**Estimated:** 8 hours  
**Status:** [ ] Not Started  
**Note:** Collaborate with Junior Engineers

**What to Do:**
- Create `options_panel.rs` UI component module
- Output filename field with auto-generation
- Output location browser (using `rfd::FileDialog`)
- Quality slider (1-100) - visible only for lossy image formats
- Advanced options (collapsible section)
- Resource limits UI
- Validation for output paths and filenames

**Reference:** SPRINT_7_TASKING.md lines 319-364

**Acceptance Criteria:**
- ✅ Output filename auto-generates from source + format
- ✅ Output filename editable
- ✅ Output location browse button works
- ✅ Quality slider shows/hides based on format
- ✅ Advanced options collapse/expand
- ✅ Resource limits validated and enforced
- ✅ Path validation prevents security issues

---

#### ✅ Task 2.4: Messages & Status Bar Components
**Priority:** High  
**Estimated:** 4 hours  
**Status:** [ ] Not Started

**What to Do:**
- Create `messages.rs` and `status_bar.rs` UI component modules
- Messages area: scrollable text area for warnings, errors, info
- Status bar: bottom bar showing current operation status
- Message types: Info (blue), Warning (yellow), Error (red), Success (green)
- Message formatting: Low-tech friendly, no technical jargon
- Path sanitization: Never display full paths

**Reference:** SPRINT_7_TASKING.md lines 368-412

**Message Examples:**
- ✓ "File converted successfully"
- ⚠ "File already exists. Will be overwritten."
- ✗ "Can't read file. Check if file exists."

**Acceptance Criteria:**
- ✅ Messages display with appropriate colors
- ✅ Messages are user-friendly (no technical jargon)
- ✅ Status bar updates correctly for each state
- ✅ Paths are sanitized before display
- ✅ Progress indicator shows for long operations

---

### Phase 3: Conversion Integration (Days 8-11)

#### ✅ Task 3.4: Conversion Thread Integration
**Priority:** High  
**Estimated:** 6 hours  
**Status:** [ ] Not Started  
**Note:** Requires Senior Engineer review

**What to Do:**
- Implement thread-safe conversion state using `Arc<Mutex<ConversionState>>`
- Spawn conversion in separate thread (prevents UI blocking)
- Update UI status during conversion
- Show progress indicator for conversions > 30 seconds
- Handle conversion completion (success/error)
- Update messages area with conversion results

**Reference:** SPRINT_7_TASKING.md lines 613-693

**Acceptance Criteria:**
- ✅ UI remains responsive during conversion
- ✅ Status bar updates during conversion
- ✅ Progress indicator shows for long operations (>30 seconds)
- ✅ Success/error messages display correctly
- ✅ Thread synchronization works correctly (no race conditions)
- ✅ Senior Engineer review completed

---

### Phase 4: Integration & Testing (Days 12-14)

#### ✅ Task 4.1: Complete UI Integration
**Priority:** Critical  
**Estimated:** 6 hours  
**Status:** [ ] Not Started

**What to Do:**
- Wire up all UI components in main app update loop
- Connect file drop zone to file selection state
- Connect format selector to format state
- Connect options panel to conversion options
- Connect Convert button to conversion function
- Connect Clear button to reset state
- Ensure all components render in correct order
- Verify layout matches GUI design document

**Reference:** SPRINT_7_TASKING.md lines 699-748

**Main UI Layout:** See SPRINT_7_TASKING.md for ASCII layout diagram

**Acceptance Criteria:**
- ✅ All UI components integrated
- ✅ Layout matches design document
- ✅ All buttons functional
- ✅ State updates correctly across components
- ✅ UI is responsive and intuitive

---

#### ✅ Task 4.4: Documentation & Polish
**Priority:** High  
**Estimated:** 4 hours  
**Status:** [ ] Not Started  
**Note:** Collaborate with Documentation Specialist

**What to Do:**
- Add inline code documentation
- Update README.md with GUI usage instructions
- Update CHANGELOG.md for v0.2.1
- Create GUI screenshot/demo (optional but recommended)
- Code cleanup (format with `cargo fmt`, fix clippy warnings)

**Reference:** SPRINT_7_TASKING.md lines 856-886

**Acceptance Criteria:**
- ✅ All code documented
- ✅ README updated with GUI information
- ✅ CHANGELOG updated
- ✅ No clippy warnings
- ✅ Code formatted with `cargo fmt`

---

## Design Principles to Follow

### 1. Simplicity First
- Most common action (drag file, select format, convert) = fewest clicks
- Progressive disclosure - advanced options hidden until needed
- Clear visual hierarchy

### 2. Feedback Always
- Every action has visible feedback
- Progress indicators for long operations
- Clear success/error states

### 3. Forgiveness
- Clear error messages
- Confirmation for destructive actions
- Preview before conversion (future)

### 4. Cross-Platform Consistency
- Windows 11: Native window decorations, High DPI scaling
- macOS 26: Retina display support, system appearance
- Ubuntu LTS 24.04+: GTK-compatible styling, Wayland/X11 support

---

## Key Dependencies

### External
- `egui` 0.27 - GUI framework
- `eframe` 0.27 - Application framework
- `rfd` 0.14 - File dialogs

### Internal (Direct Library Integration)
- `common` crate - Validation, limits, error handling
- `img-core` crate - Image conversion (direct integration, not subprocess)
- `mesh-core` crate - Mesh conversion (direct integration, not subprocess)

---

## Architecture Compliance

**CRITICAL:** All conversions must use **direct library integration**:
- ✅ Use `img-core` and `mesh-core` libraries directly
- ❌ **DO NOT** call CLI binaries as subprocesses
- **Why:** Security, performance, architecture compliance

---

## Security Requirements

All file operations must:
- ✅ Use `common::validation::validate_file_path()` for all paths
- ✅ Two-stage format detection (extension + magic bytes)
- ✅ Check file size before reading (DoS prevention)
- ✅ Validate output paths (not system directories)
- ✅ Sanitize error messages (no path leaks)

---

## Collaboration Points

### With Junior Engineer - 2D (Sam Kim)
- Format detection UI integration (Task 2.2)
- Quality settings UI (Task 2.3)
- Image conversion wiring (Task 3.2 - Sam implements, you integrate)

### With Junior Engineer - 3D (Alex Rivera)
- Mesh format detection UI (Task 2.2)
- Mesh options UI (Task 2.3)
- Mesh conversion wiring (Task 3.3 - Alex implements, you integrate)

### With Senior Engineer (Jordan Rivera)
- Application state structure review (Task 1.3)
- Thread-safe conversion review (Task 3.4)
- Security validation review (Task 4.2)
- Code reviews for all your work

### With Security Specialist (Casey Morgan)
- Security validation review (Task 4.2)
- Path validation implementation
- Error message sanitization

### With Documentation Specialist (Morgan Lee)
- Documentation updates (Task 4.4)
- README GUI section
- CHANGELOG updates

---

## Daily Standup Questions

Be prepared to answer:
1. What did I complete yesterday?
2. What am I working on today?
3. Any blockers?
4. Any questions for Senior Engineer or other team members?

---

## Success Criteria

### Functional
- ✅ GUI application launches and displays correctly
- ✅ File drag-and-drop works for images and meshes
- ✅ Format selection works (image and mesh formats)
- ✅ Image and mesh conversion functional through GUI
- ✅ Error handling displays user-friendly messages

### Technical
- ✅ Direct library integration (no subprocess calls)
- ✅ Thread-safe conversion processing
- ✅ All security validations implemented
- ✅ Code compiles without warnings
- ✅ All tests pass

### Quality
- ✅ User interface is intuitive and responsive
- ✅ Error messages are user-friendly (no technical jargon)
- ✅ No information leakage in error messages
- ✅ Cross-platform ready (Windows tested)

---

## Questions or Blockers?

**Contact:**
- Senior Engineer (Jordan Rivera) - Technical questions, code reviews
- System Architect (Alex Chen) - Architecture questions
- Security Specialist (Casey Morgan) - Security validation questions

**Reference Documents:**
- Detailed tasking: `SPRINT_7_TASKING.md`
- GUI design: `GUI_DESIGN_AND_IMPLEMENTATION.md`
- Architecture: `Phase3_Architecture.md`

---

**Good luck! The team is counting on your expertise to deliver an excellent GUI experience.**

**Document Version:** 1.0  
**Created:** January 2026  
**Status:** Ready for Implementation

