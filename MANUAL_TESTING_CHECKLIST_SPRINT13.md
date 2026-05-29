# Manual Testing Checklist — Sprint 13

**Created:** May 29, 2026  
**Author:** Jamie Chen (UI Designer)  
**Application:** Simple Image Converter v0.3.0 (`converter-gui`)  
**Reference:** `MANUAL_TESTING_REPORT_SPRINT12.md` (Sprint 12 baseline + Sprint 13 addendum)

---

## How to Use This Checklist

| Marker | Meaning |
|--------|---------|
| **Code Verified ✅** | Implementation confirmed in source and/or automated tests (May 29, 2026). Human still recommended for UX sign-off. |
| **Manual Required ⏳** | Requires human interaction (keyboard, mouse, native dialogs, visual inspection). Cannot be signed off by automation alone. |
| **Re-test 🔁** | Sprint 12 partial pass or fix applied — must be re-verified on target platform. |

**Build before testing:**

```bash
cargo build --release -p converter-gui
# Binary: target/release/converter-gui
```

**Test fixtures:** PNG/JPEG images, STL/OBJ/PLY meshes, multi-file set for batch queue.

**Record results in:** `MANUAL_TESTING_REPORT_SPRINT12.md` (Sprint 13 addendum section).

---

## Automated Verification Summary (May 29, 2026)

Session: macOS (darwin). Command: `cargo test -p converter-gui --workspace`

| Test suite | Count | Result |
|------------|-------|--------|
| `converter-gui` unit tests (lib) | 69 | ✅ Pass |
| `converter-gui` unit tests (bin) | 69 | ✅ Pass |
| `converter-gui/tests/integration_tests.rs` | 25 | ✅ Pass |
| `converter-gui/tests/security_tests.rs` | 18 | ✅ Pass |
| `converter-gui/tests/sprint11_tests.rs` | 13 | ✅ Pass |
| **converter-gui total** | **194** | **✅ 0 failed** |
| Full workspace (`--workspace`) | All crates | ✅ Pass |

**macOS release build:** `cargo build --release -p converter-gui` — ✅ succeeded (May 29, 2026).  
**Not verified this session:** Windows 11, Linux Ubuntu 24.04+ builds and GUI behaviour.

---

## Code Audit: Keyboard Shortcuts vs Help Panel

Source: `converter-gui/src/app.rs::handle_keyboard_shortcuts()`, `converter-gui/src/ui/help_panel.rs`

| Shortcut | Handler | Help panel | Audit |
|----------|---------|------------|-------|
| Ctrl+O / Cmd+O | ✅ | ✅ | **Code Verified ✅** |
| Ctrl+S / Cmd+S (settings open) | ✅ | ✅ | **Code Verified ✅** |
| Ctrl+R / Cmd+R | ✅ | ✅ | **Code Verified ✅** |
| Ctrl+A / Cmd+A | ✅ (skips text fields) | ✅ | **Code Verified ✅** |
| Ctrl+Shift+D / Cmd+Shift+D | ✅ | ✅ | **Code Verified ✅** |
| Ctrl+Enter / Cmd+Enter | ✅ | ✅ | **Code Verified ✅** |
| Ctrl+P / Cmd+P | ✅ | ✅ | **Code Verified ✅** |
| Space (batch active) | ✅ (skips text fields) | ✅ | **Code Verified ✅** |
| Escape (edit dialog / batch) | ✅ | ✅ | **Code Verified ✅** |
| Enter (single conversion) | ✅ | ✅ | **Code Verified ✅** |
| F1 (open help) | ✅ | ✅ | **Code Verified ✅** |
| Tab / Arrow keys | egui default | ✅ documented | **Manual Required ⏳** |

**Help menu note:** Menu item is **Help → Help & Documentation** (not "Keyboard Shortcuts..." as in Sprint 12 draft). F1 also opens the help panel.

**About dialog:** Version from `CARGO_PKG_VERSION` (currently **0.3.0**).

---

## Priority Re-tests (Sprint 12 Carry-over)

These block Task 3.1 sign-off until human pass on macOS **and** Windows 11.

| ID | Item | Sprint 12 status | Sprint 13 action |
|----|------|------------------|------------------|
| P1 | Test 5.3 — Edit queue dialog + Escape | Fix applied (radio_value); re-test pending | **Re-test 🔁 Manual Required ⏳** |
| P2 | Test 1.2 — Settings persist after restart | Not tested | **Manual Required ⏳** |
| P3 | Test 1.2 — Cmd+S disabled when settings closed | Not tested | **Manual Required ⏳** |
| P4 | High — Mesh files visible in Open dialog (Windows) | Fix applied; Windows re-test pending | **Re-test 🔁 Manual Required ⏳** (Windows) |
| P5 | Tests 6.1–6.2 — Help & About | Not manually tested | **Manual Required ⏳** |
| P6 | Tests 5.4–5.5 — Tab / Arrow navigation | Not tested | **Manual Required ⏳** |
| P7 | Tests 7.1–7.5 — UI consistency visual | Not tested | **Manual Required ⏳** |
| P8 | Cross-platform smoke (Task 3.2) | Incomplete | **Manual Required ⏳** |

---

## Section 1: Keyboard Shortcuts (Tests 1.1–5.5)

Platform modifier: **Cmd** on macOS, **Ctrl** on Windows/Linux. Mark each platform column when done.

### 1.1 Open File — Ctrl+O / Cmd+O

**Code Verified ✅** | **Manual Required ⏳** (native file dialog)

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Launch app | ☐ | ☐ |
| 2 | Press modifier+O | ☐ | ☐ |
| 3 | Native file dialog opens | ☐ | ☐ |
| 4 | Select PNG image; file loads, format options appear | ☐ | ☐ |
| 5 | Repeat with STL mesh; mesh loads | ☐ | ☐ |
| 6 | Verify "Supported Files" / mesh extensions visible in dialog | ☐ | ☐ |

### 1.2 Save Settings — Ctrl+S / Cmd+S

**Code Verified ✅** | **Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Edit → Preferences; change default quality | ☐ | ☐ |
| 2 | With panel open, press modifier+S; success message | ☐ | ☐ |
| 3 | With panel **closed**, press modifier+S; no save / no error | ☐ | ☐ |
| 4 | Quit app, relaunch; setting persisted | ☐ | ☐ |

### 2.1 Start Conversion — Enter

**Code Verified ✅** (macOS partial pass Sprint 12) | **Re-test 🔁**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Load file, select output format | ☐ | ☐ |
| 2 | Press Enter; conversion starts | ☐ | ☐ |
| 3 | No file selected + Enter → no conversion | ☐ | ☐ |
| 4 | During active conversion + Enter → no duplicate start | ☐ | ☐ |

### 3.1–3.4 Batch Shortcuts

**Code Verified ✅** (macOS partial pass Sprint 12) | **Re-test 🔁**

| Test | Shortcut | Steps | macOS | Windows |
|------|----------|-------|-------|---------|
| 3.1 | Ctrl+Enter | Add 3 files (Ctrl+A), start batch | ☐ | ☐ |
| 3.2 | Ctrl+P | Pause mid-batch, resume | ☐ | ☐ |
| 3.3 | Space | Pause/resume while batch active | ☐ | ☐ |
| 3.4 | Escape | Cancel active batch | ☐ | ☐ |
| — | Ctrl+Enter | Empty queue → no start / graceful | ☐ | ☐ |

### 4.1–4.2 Queue Management

**Code Verified ✅** (macOS pass Sprint 12) | **Re-test 🔁** on Windows

| Test | Shortcut | Steps | macOS | Windows |
|------|----------|-------|-------|---------|
| 4.1 | Ctrl+A | Multi-select files → queue populated | ☐ | ☐ |
| 4.2 | Ctrl+Shift+D | Confirm clears queue; cancel preserves; empty queue no-op | ☐ | ☐ |

### 5.1 Preferences Toggle

**Code Verified ✅** | **Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Edit → Preferences opens settings section | ☐ | ☐ |
| 2 | Edit → Preferences again closes | ☐ | ☐ |

### 5.2 Reset — Ctrl+R / Cmd+R

**Code Verified ✅** (macOS pass) | **Re-test 🔁**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Load file + format + options | ☐ | ☐ |
| 2 | Press modifier+R; selection and options reset | ☐ | ☐ |

### 5.3 Escape — Edit Queue Dialog 🔁 BLOCKING

**Code Verified ✅** (Escape clears `editing_queue_item`; format selector uses `radio_value`)  
**Manual Required ⏳** — primary Sprint 13 re-test

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Add image to batch queue | ☐ | ☐ |
| 2 | Click **Edit** on item; "Edit Queue Item" dialog opens | ☐ | ☐ |
| 3 | Click different output format; selection **updates visually** | ☐ | ☐ |
| 4 | Press Escape; dialog closes | ☐ | ☐ |
| 5 | Re-open Edit; format unchanged (no spurious save) | ☐ | ☐ |
| 6 | Change format, click Save; queue item updated | ☐ | ☐ |

### 5.4 Tab Navigation

**Manual Required ⏳** (egui focus; not in shortcut handler)

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Load file; Tab through format/options fields | ☐ | ☐ |
| 2 | Open settings; Tab through inputs | ☐ | ☐ |
| 3 | Focus ring visible | ☐ | ☐ |

### 5.5 Arrow Keys — Format Radio Buttons

**Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Load image; arrow keys change format selection | ☐ | ☐ |
| 2 | Enter starts conversion with selected format | ☐ | ☐ |

### 5.6 F1 — Open Help

**Code Verified ✅** | **Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Press F1; Help & Documentation window opens | ☐ | ☐ |
| 2 | Shortcuts list matches behaviour | ☐ | ☐ |

---

## Section 2: Help System (Tests 6.1–6.2)

### 6.1 Help & Documentation

**Code Verified ✅** (content + menu wiring) | **Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Help → Help & Documentation | ☐ | ☐ |
| 2 | Quick Start, Shortcuts, Features, Troubleshooting visible | ☐ | ☐ |
| 3 | Platform notes (Ctrl vs Cmd) readable | ☐ | ☐ |
| 4 | GitHub link opens in browser | ☐ | ☐ |
| 5 | Window closes via X or Escape | ☐ | ☐ |

### 6.2 About Dialog

**Code Verified ✅** | **Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Help → About | ☐ | ☐ |
| 2 | Version shows **0.3.0** | ☐ | ☐ |
| 3 | License MIT OR Apache-2.0 | ☐ | ☐ |
| 4 | Repository hyperlink works | ☐ | ☐ |
| 5 | Technology credits listed | ☐ | ☐ |

---

## Section 3: UI Consistency (Tests 7.1–7.5)

**Code Verified ✅** (style constants in unit/sprint11 tests) | **Manual Required ⏳** (visual)

| Test | Panel | Verify | macOS | Windows |
|------|-------|--------|-------|---------|
| 7.1 | Drop zone | Spacing, borders, drag/empty/selected states | ☐ | ☐ |
| 7.2 | Batch queue | Status colours, progress, buttons | ☐ | ☐ |
| 7.3 | Settings | Collapsible sections, auto-save indicator | ☐ | ☐ |
| 7.4 | History | Entry styling, timestamps | ☐ | ☐ |
| 7.5 | Preview | Image preview; 3D viewer if feature enabled | ☐ | ☐ |

---

## Section 4: Error Messages (Tests 8.1–8.2)

**Code Verified ✅** (error_messages + security tests) | **Manual Required ⏳**

| Step | Action | macOS | Windows |
|------|--------|-------|---------|
| 1 | Convert unsupported/invalid file; friendly message | ☐ | ☐ |
| 2 | No full path in message (filename only) | ☐ | ☐ |
| 3 | Info / Warning / Error / Success colours distinct | ☐ | ☐ |
| 4 | Messages area scrolls; Clear button works | ☐ | ☐ |

---

## Section 5: Drag-and-Drop (Tests 9.1–9.2)

**Code Verified ✅** (drop_zone implementation) | **Manual Required ⏳**

| Test | Action | macOS | Windows |
|------|--------|-------|---------|
| 9.1 | Single file onto drop zone; blue hover, green selected | ☐ | ☐ |
| 9.2 | Multiple files onto batch area (if supported) | ☐ | ☐ |

---

## Section 6: Batch Processing (Tests 10.1–10.4)

**Code Verified ✅** (integration_tests: queue, pause/resume, parallel) | **Manual Required ⏳**

| Test | Scenario | macOS | Windows |
|------|----------|-------|---------|
| 10.1 | Batch images (PNG/JPEG/BMP) | ☐ | ☐ |
| 10.2 | Batch meshes (STL/OBJ/PLY) | ☐ | ☐ |
| 10.3 | Mixed image + mesh queue | ☐ | ☐ |
| 10.4 | Parallel processing (Settings → max concurrent > 1) | ☐ | ☐ |

---

## Section 7: Exit & Settings Persistence (Task 3.2)

**Code Verified ✅** (`on_exit` + File → Exit save; auto-save debounce in tests) | **Manual Required ⏳**

| Step | Action | macOS | Windows | Linux |
|------|--------|-------|---------|-------|
| 1 | Change setting; wait for auto-save indicator | ☐ | ☐ | ☐ |
| 2 | Close via window X; relaunch; settings restored | ☐ | ☐ | ☐ |
| 3 | File → Exit; settings saved (tooltip claims auto-save) | ☐ | ☐ | ☐ |
| 4 | Window size restored on relaunch | ☐ | ☐ | ☐ |

---

## Section 8: Cross-Platform Smoke Test (Task 3.2)

**Manual Required ⏳** on all three platforms.

| Scenario | macOS | Windows 11 | Ubuntu 24.04+ |
|----------|-------|------------|---------------|
| Single-file PNG → JPEG conversion | ☐ | ☐ | ☐ |
| Single-file STL → OBJ conversion | ☐ | ☐ | ☐ |
| Batch: add 3 files, convert all | ☐ | ☐ | ☐ |
| Batch: pause → resume → complete | ☐ | ☐ | ☐ |
| Batch: cancel mid-run | ☐ | ☐ | ☐ |
| Settings persist across restart | ☐ | ☐ | ☐ |
| Native file dialogs acceptable | ☐ | ☐ | ☐ |
| HiDPI / Retina layout acceptable | ☐ | ☐ | ☐ |

---

## Section 9: 3D Viewer (Tests 11.1–11.2, if enabled)

**Manual Required ⏳** — build with `viewer-3d` feature if testing 3D preview.

| Test | Action | macOS | Windows |
|------|--------|-------|---------|
| 11.1 | Load STL; viewer renders mesh | ☐ | ☐ |
| 11.2 | Orbit, pan (Shift+drag), zoom, wireframe/solid | ☐ | ☐ |

---

## Sign-Off

| Role | Name | Date | Task 3.1 | Task 3.2 |
|------|------|------|----------|----------|
| UI Designer | Jamie Chen | | ☐ In Progress | ☐ In Progress |
| Tester (human) | | | ☐ Complete | ☐ Complete |

**Release gate:** Task 3.1 requires zero open Critical/High issues and all Priority Re-tests (P1–P4) passed on macOS + Windows 11.

---

*Document version: 1.0 — May 29, 2026*
