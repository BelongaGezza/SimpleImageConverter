# Sprint 12_A Tasking: v1.0.0 Final Release Preparation

**Sprint Duration:** January 3-10, 2026
**Sprint Goal:** Complete all remaining tasks for v1.0.0 stable release
**Target Release:** v1.0.0 (Mid-January 2026)
**Last Updated:** January 26, 2026

---

## Canonical Plan References (Source of Truth)

To keep tasking consistent and avoid “status drift” across older Sprint 12 documents, this Sprint 12_A tracker is aligned to:

- **Scope**: `V1.0.0_SCOPE.md`
- **Release gates / checklist**: `V1.0.0_RELEASE_CHECKLIST.md`
- **Format claims (must match implementation)**: `docs/FORMATS.md` and `RELEASE_NOTES_v1.0.0.md`

**Important:** Several Sprint 12 documents dated **Dec 30, 2025** reflect the state *at that time* (e.g., before the Jan 3 security audit and before Jan 26 quality-gate drift). Use this document for current blocking status, and treat older Sprint 12 docs as historical context unless explicitly updated.

---

## Sprint Progress Summary

| Phase | Status | Completion |
|-------|--------|------------|
| Phase 1: Critical Fixes (baseline) | ✅ Complete | 100% |
| Addendum: Release Gate Recovery (A.*) | 🔴 Blocking | 0% |
| Phase 2: Manual Testing | 🟡 In Progress | Incomplete |
| Phase 3: Security Review | ✅ Complete | 100% |
| Phase 4: Documentation | ✅ Complete | 100% |
| Phase 5: Approval & Release Execution | ⏳ Pending | 0% |

**Overall Sprint Progress:** ✅ Baseline fixes/security/docs done; 🔴 **blocked** on release gates (A.*) + manual testing sign-off.

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes |
|------|--------------|--------|----------------|------------|-------|
| Senior Engineer (Jordan Rivera) | `.agents/senior-engineer.md` | 🟡 In Progress (claimed) | Cursor session `/agent senior` (Jordan Rivera) | 1.1, 1.2, 2.3, 2.4 (eng), 2.5 (eng), A.1, A.5, 5.2, 5.3, 5.4 | Owns release gates + packaging/tag/release execution. Reviews/accepts 3D addendum implementations. |
| System Architect (Alex Chen) | `.agents/system-architect.md` | 🟡 In Progress | `/agent architect` | 5.1, A.2 (decision), A.3 (policy) | Owns “ship/no-ship” decisions + policy alignment. |
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | 🟡 In Progress | `/agent ui` | 2.1, 2.2, 2.4 (manual re-test), 2.5 (manual verify) | Owns manual testing execution + issue triage/sign-off. |
| Security Specialist (Casey Morgan) | `.agents/security-specialist.md` | ✅ Complete | `/agent security` | 3.1, 3.2 | Done (Grade A). Re-consult only if release-scope changes. |
| Documentation Specialist (Sam Parker) | `.agents/documentation-specialist.md` | ✅ Complete | `/agent docs` | 4.1, 4.2 | Done (formats + release notes draft). |
| Junior Engineer (3D Formats) | `.agents/junior-engineer-3d.md` | 🟡 In Progress | Cursor session (Alex Rivera / junior-3d) | A.2 (implementation), A.3 (implementation), A.4.1 | Owns glTF/mesh format correctness work (with Senior review). |
| Junior Engineer (2D Formats) | `.agents/junior-engineer-2d.md` | 🟡 In Progress (claimed) | Cursor session `/agent junior-2d` | A.4.2 | Owns SVG/resvg upgrade work (with Senior review). |
| Researcher | `.agents/researcher.md` | 🟡 In Progress (claimed) | Cursor session `/agent research` | 2.2.1, A.2 (validator/tooling), A.4 (dependency notes) | Owns ecosystem/tooling investigation and decision support. |

---

## Completed Work (January 3, 2026)

### Commits Pushed
1. `fed6071` - fix(ci): add font license exceptions to cargo-deny config
2. `3e154e6` - docs: add Sprint 12_A tasking for v1.0.0 final release preparation
3. `5a12c7b` - docs: update ROADMAP.md for v1.0.0 release preparation
4. `6ae322b` - docs: update FORMATS.md for v0.3.0 features and v1.0.0 preparation
5. `7ebf0a3` - docs: add v1.0.0 release notes draft
6. `2b72ec0` - deps: update rfd 0.14 -> 0.15 to resolve Rust 2024 compatibility
7. `b345cfe` - docs: add v1.0.0 security audit report

### Key Fixes Applied
- ✅ License configuration fixed (OFL-1.1, UFL-1.0 for egui fonts)
- ✅ Dependency compatibility resolved (rfd 0.14→0.15, ashpd 0.8→0.11)
- ✅ No more Rust 2024 future-incompat warnings

---

## Addendum (January 26, 2026): Reliability & Consistency Review Follow-ups (PRIORITY OVERRIDE)

This addendum captures new release-relevant findings from a codebase reliability/consistency review run on **January 26, 2026**.

### Evidence (automated checks)
- ✅ `cargo test --workspace`: PASS
- 🟡 `cargo clippy --workspace --all-targets`: PASS with **3 warnings** (all in `converter-gui-modern/src/app.rs`)
- 🔴 `cargo fmt --all --check`: **FAIL** (formatting drift present)

### Key Findings (impact)
- 🔴 **Formatting drift**: `cargo fmt --check` failing means the formatting/consistency gate is currently broken.
- 🔴 **glTF writer output likely incomplete**: `mesh-core/src/formats/gltf.rs` writes a placeholder `uri` without embedding buffer data, which can produce invalid `.gltf` output.
- 🟡 **Security/consistency mismatch**: image formats use **two-stage detection**; mesh formats are currently **extension-only**.

### Priority Tasks (assignments)

#### Addendum Task A.1: Restore rustfmt compliance (Quality Gate)
**Assigned Role:** Senior Engineer (Jordan Rivera)  
**Priority:** **BLOCKING**  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete

**Acceptance Criteria:**
- [ ] `cargo fmt --all --check` passes on `main`
- [ ] CI (or equivalent) enforces the formatting gate consistently

---

#### Addendum Task A.2: glTF write correctness decision + remediation
**Assigned Roles:** System Architect (Alex Chen) + Junior Engineer (3D Formats) + Senior Engineer (Jordan Rivera) + Researcher  
**Priority:** **BLOCKING**  
**Status:** [ ] Not Started / [x] In Progress / [ ] Complete

**Architect Decision (required):**
- [x] **Decision (v1.0.0): (A) Support glTF write fully.**
  - Minimum bar: valid `.glb` output; `.gltf` output must be valid as a single-file export (embedded buffer) or a documented, deterministic multi-file export.
  - Rationale: `docs/FORMATS.md` and `RELEASE_NOTES_v1.0.0.md` currently claim glTF/GLB **Write: Yes**; v1.0.0 must not ship with a knowingly-invalid writer.

**Implementation Acceptance Criteria:**
- [ ] If (A): generated output imports in a standard glTF validator/viewer and has automated test coverage
- [ ] If (B): writer returns `UnsupportedFormat` and documentation reflects read-only status

**Junior-3D update:** Implemented valid `.gltf` (single-file, embedded base64 buffer) + `.glb` writer output and added parse-based tests in `mesh-core` (pending external validator run + Senior review).

**Allocation (so work is fully owned):**
- **A.2.1 Architect (decision/contract)**: define v1.0.0 glTF/GLB write contract (single-file `.glb` minimum; `.gltf` either embedded or deterministic multi-file) and document it here.
- **A.2.2 Junior-3D (implementation)**: implement the chosen contract in `mesh-core/src/formats/gltf.rs` (no placeholder `uri`), plus add tests. ✅ **Implemented**
- **A.2.3 Research (tooling/validation)**: pick a validator/viewer for CI/manual validation (e.g., Khronos validator or a widely-used viewer) and document exact validation steps + expected outputs.
- **A.2.3 Research (tooling/validation) - Proposed tooling + steps (Jan 26, 2026):**
  - **Validator (recommended for CI + reproducible local checks):** KhronosGroup **glTF-Validator** CLI (`gltf_validator`) from GitHub Releases.
    - **Validate a single asset (GLB or GLTF):**
      - `gltf_validator -r -o --no-write-timestamp --no-absolute-path path/to/model.glb > model.report.json`
      - `gltf_validator -r -o --no-write-timestamp --no-absolute-path path/to/model.gltf > model.report.json`
    - **Interpretation:** CLI exit code is **non-zero if at least one error** is found; JSON report contains issues + asset stats.
    - **Directory validation:** you can pass a directory to validate all `*.gltf`/`*.glb` recursively (multi-threaded).
  - **Viewer (recommended for quick manual sanity):**
    - Babylon.js Sandbox (drag-drop the exported `.glb` / `.gltf` and confirm geometry/materials render)
    - Don McCurdy’s `gltf-viewer` (drag-drop and confirm model loads; useful for quick regression checks across browsers)
  - **Expected outputs:** no validator **errors**; warnings should be triaged (and either resolved or justified, depending on contract).
- **A.2.4 Senior (review/gate)**: code review + run `cargo test --workspace`/sanity export on sample meshes; confirm docs/claims remain accurate.

---

#### Addendum Task A.3: Mesh format two-stage detection policy + implementation
**Assigned Roles:** System Architect (Alex Chen) + Junior Engineer (3D Formats) + Senior Engineer (Jordan Rivera)  
**Priority:** **HIGH**  
**Status:** [ ] Not Started / [x] In Progress / [ ] Complete

**Acceptance Criteria:**
- [ ] Define a consistent policy aligned with `rust-resources.md`: **extension + signature verification where feasible; otherwise extension + parse-validate**
- [x] Implement signature verification for formats with clear headers (e.g., `OFF`, `ply`, `glTF`/`GLB`)
- [x] Add/adjust tests covering spoofing/mismatch cases

**Allocation (so work is fully owned):**
- **A.3.1 Architect (policy)**: write the normative policy (what we check, when we fail, and how errors are reported) and ensure it aligns with `rust-resources.md`.
- **A.3.2 Junior-3D (implementation)**: implement signature/parse-validate checks for mesh formats with clear headers; add/adjust spoofing tests. ✅ **Implemented**
- **A.3.3 Senior (review/gate)**: ensure behavior matches image “two-stage” expectations and doesn’t regress CLI/GUI UX; sign off on acceptance criteria.

---

#### Addendum Task A.4: Dependency maintenance per rust-resources.md
**Assigned Roles:** Junior Engineer (3D Formats) + Junior Engineer (2D Formats) + Researcher + Senior Engineer (review)  
**Priority:** **MEDIUM** (post-v1.0.0 unless required for compatibility/security)  
**Status:** [ ] Not Started / [x] In Progress / [ ] Complete

**Scope:**
- [x] **A.4.1 (Junior-3D)** Upgrade `stl_io` (**0.7 → 0.10**) with mesh regression testing (done: `cargo test --workspace` PASS)
- [x] **A.4.2 (Junior-2D)** Upgrade `resvg` (**0.40 → 0.45**) with SVG rendering regression testing (done: `resvg` 0.45.1; usvg API update; `cargo test --workspace` PASS)
- [ ] **A.4.3 (Research)** Validate upstream changelogs/advisories; note any breaking changes and update `rust-resources.md` guidance if needed
- [ ] **A.4.4 (Senior)** Review/merge gate: `cargo test --workspace` passes and changes are release-safe

**Acceptance Criteria:**
- [x] `cargo test --workspace` passes after upgrades
- [ ] Any API changes documented (and `rust-resources.md` updated if needed)

---

#### Addendum Task A.5: Address outstanding clippy warnings (converter-gui-modern)
**Assigned Role:** Senior Engineer (Jordan Rivera)  
**Priority:** **MEDIUM**  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete

**Acceptance Criteria:**
- [ ] `cargo clippy --workspace --all-targets` is warning-free (or warnings explicitly justified/allowed)

## Task Breakdown

### Phase 1: Critical Fixes ✅ COMPLETE

#### Task 1.1: Fix License Configuration
**Assigned Role:** Senior Engineer (Jordan Rivera)
**Status:** [x] Complete
**Priority:** HIGH - Blocking CI

**Completed:**
- [x] Add `OFL-1.1` to `deny.toml` allow list
- [x] Add exception for `LicenseRef-UFL-1.0` (non-SPDX font license)
- [x] Verify `cargo deny check` passes
- [x] Commit changes (`fed6071`)

---

#### Task 1.2: Assess Dependency Future-Compatibility
**Assigned Role:** Senior Engineer (Jordan Rivera)
**Status:** [x] Complete
**Priority:** MEDIUM

**Completed:**
- [x] Check if `rfd` has newer version with updated `ashpd` - Yes, 0.15.4
- [x] Update rfd 0.14 → 0.15 and test - All tests passing
- [x] ashpd updated 0.8.1 → 0.11.0 - Rust 2024 compatible
- [x] Commit changes (`2b72ec0`)

**Result:** Issue fully resolved, no longer non-blocking - it's fixed.

---

### Phase 2: Manual Testing 🟡 IN PROGRESS

#### Task 2.1: Execute Manual Testing Checklist
**Assigned Role:** UI Designer (Jamie Chen)
**Status:** [x] In Progress
**Priority:** HIGH - Blocking v1.0.0

**Description:**
Code verification is complete, but **manual testing execution is not signed off** (see `MANUAL_TESTING_REPORT_SPRINT12.md` for partial results and open re-test blockers).

**Acceptance Criteria:**
- [ ] Execute all keyboard shortcut tests (Tests 1.1-5.5)
- [ ] Execute help system tests (Tests 6.1-6.2)
- [ ] Execute UI consistency tests (Tests 7.1-7.5)
- [ ] Execute error message tests (Tests 8.1-8.2)
- [ ] Execute drag-and-drop tests (Tests 9.1-9.2)
- [ ] Execute batch processing tests (Tests 10.1-10.4)
- [ ] Execute 3D viewer tests (Tests 11.1-11.2) if feature enabled
- [ ] Document all findings in test report
- [ ] Report any critical issues immediately

**Platform:**
- Primary: Windows 11
- Secondary: macOS, Linux (if available)

---

#### Task 2.2: Fix Issues Found in Manual Testing
**Assigned Role:** UI Designer (Jamie Chen)
**Status:** [x] In Progress
**Priority:** HIGH - Depends on Task 2.1

**Subtasks (explicit ownership):**
- **2.2.1 Investigate Windows file dialog mesh filters** (possible `rfd`/OS behavior vs app bug)  
  - **Owner:** Researcher (`/agent research`)  
  - **Support:** Senior Engineer (`/agent senior`) if a code workaround is needed; UI Designer for reproduction notes  
  - **Exit criteria:** either (a) confirmed working with repro steps, or (b) root-caused + tracked fix/workaround documented for v1.0.0
  - **Update (Jan 26, 2026):** Implemented a low-risk mitigation: added a first filter named **"Supported Files"** (images + meshes) to the relevant `rfd::FileDialog` call sites so mesh files are visible by default even if the Windows filter dropdown is not shown/used. **Next:** quick Windows 11 manual re-test to confirm behavior.

**Acceptance Criteria:**
- [ ] Triage issues by severity (Critical/High/Medium/Low)
- [ ] Fix all Critical issues (if any)
- [ ] Fix all High priority issues
- [ ] Document Medium/Low issues for post-v1.0.0
- [ ] Re-test fixed issues
- [ ] Update test report with final status

---

#### Task 2.3: Cross-Platform Validation
**Assigned Role:** Senior Engineer (Jordan Rivera)
**Status:** [ ] Not Started
**Priority:** MEDIUM

**Acceptance Criteria:**
- [ ] Build release binaries for Windows x64
- [ ] Build release binaries for macOS x64 (if available)
- [ ] Build release binaries for Linux x64
- [ ] Test basic conversion on each platform
- [ ] Verify GUI launches correctly on each platform
- [ ] Document any platform-specific issues

---

#### Task 2.4: Urgent GUI Remediation (External Consultant + Sprint 12 Manual Testing Blockers)
**Assigned Role:** Senior Engineer (Jordan Rivera) + System Architect (Alex Chen)
**Status:** [x] Code Fixes Applied / [ ] Manual Re-test Complete
**Priority:** BLOCKING (must be resolved before completing Sprint 12 manual testing)

**Context:**
- External consultant report: `ExternalConsultantReportGUI.md` (Critical Issues 1.1–1.3)
- Manual testing blocker: `MANUAL_TESTING_REPORT_SPRINT12.md` Issue #2 (Edit Queue Item output format selection not updating)

**Scope (must-fix before manual testing completion):**
- [x] **Exit-save reliability**: Implement `eframe::App::on_exit()` to force-save settings on OS-level window close
- [x] **Shortcut focus correctness**: Do not override text-field conventions:
  - [x] `Ctrl+A` / `Cmd+A` respects Select All when typing in a text field
  - [x] `Space` does not pause/resume batch while typing in a text field
- [x] **Edit Queue Item dialog**: Make output format selection clearly interactive and persistent (align with main format selector)
- [ ] **Manual verification**: Re-run manual tests impacted by these changes (macOS + Windows 11; Linux if available)

**Allocation (so the remaining work is fully owned):**
- **2.4.1 Senior (engineering)**: complete (code fixes landed); remain available for bugfixes found during re-test.
- **2.4.2 UI (manual re-test/sign-off)**: re-run impacted manual tests on Windows + macOS (Linux if available) and update `MANUAL_TESTING_REPORT_SPRINT12.md`.
- **2.4.3 Architect (release-risk sign-off)**: confirm the fixes satisfy consultant concerns and do not introduce UX regressions that would block v1.0.0.

**Acceptance Criteria (manual):**
- [ ] Settings changes persist when closing via window close (X) immediately after edits
- [ ] `Ctrl+A` / `Cmd+A` selects all text inside output path/name fields (does not open Add Files dialog)
- [ ] While batch processing is active, pressing Space inside a focused text field inserts a space (does not pause)
- [ ] In Edit Queue Item dialog, selecting a different output format visibly changes selection and persists until Save/Cancel
- [ ] Escape closes Edit Queue Item dialog without committing changes (Test 5.3) — verify after output-format selection is functional

**Code touchpoints (for review):**
- `converter-gui/src/app.rs` (keyboard shortcut focus gating; `on_exit`)
- `converter-gui/src/ui/batch_queue.rs` (edit dialog output format selection)

---

#### Task 2.5: Consultant Report Follow-Through (High-Leverage Non-Blockers)
**Assigned Role:** Senior Engineer (Jordan Rivera) + System Architect (Alex Chen)
**Status:** [x] Code Fixes Applied / [ ] Manual Verification Complete
**Priority:** HIGH

**Scope (from `ExternalConsultantReportGUI.md`, beyond Task 2.4):**
- [x] **Window size restore**: Load window size from settings on startup; persist window size changes
- [x] **Batch processing cleanup**: Sync UI state from background worker and clear processing state on completion
- [x] **Recent files integration**: Record recent files on selection and batch add; expose list in Settings
- [x] **Preview cache eviction**: Replace O(n) front-removal with O(1) `VecDeque` pop-front
- [x] **Documentation drift**: Update `rust-resources.md` to reflect `rfd` version in use

**Manual verification (recommended):**
- [ ] Launch app, resize window, close, relaunch → size restored
- [ ] Run a batch, wait for completion → pause/resume controls disabled; UI reflects final statuses
- [ ] Select a file and add batch items → “Recent Files” populates in Settings

**Allocation (so the remaining work is fully owned):**
- **2.5.1 Senior (engineering)**: complete (code fixes landed); remain available for bugfixes found during verification.
- **2.5.2 UI (manual verification)**: run the above checks on Windows + macOS and record results in `MANUAL_TESTING_REPORT_SPRINT12.md` (or a short addendum section).
- **2.5.3 Architect (acceptance)**: confirm any “non-blockers” truly remain non-blocking for v1.0.0 and schedule remaining UX enhancements post-release.

---

#### Task 2.6 (Post-v1.0.0): Cloud-synced paths & batch-processing UX (iCloud/Dropbox/OneDrive)
**Assigned Roles:** System Architect (Alex Chen) + Researcher  
**Status:** [ ] Deferred / [ ] In Progress / [ ] Complete  
**Priority:** LOW (post-v1.0.0 unless elevated by user impact)

**Context (from `MANUAL_TESTING_REPORT_SPRINT12.md` Test 3.2 notes):**
Batch processing may behave unexpectedly when input/output paths are cloud-synced locations (download/upload semantics, availability, permissions).

**Acceptance Criteria:**
- [ ] Document v1.0.0 behavior/limitations (if any) and recommended user workaround (e.g., local output folder)
- [ ] Propose a v1.0.1+ design (local staging, explicit “copy back”, better error messaging) and estimate effort/risk

**Allocation:**
- **2.6.1 Research (ecosystem/OS behavior)**: investigate common patterns/limitations for file access in cloud-synced folders across macOS/Windows.
- **2.6.2 Architect (product/UX decision)**: decide whether we (a) document as limitation, (b) add local-staging option, or (c) implement deeper sync-aware behavior.

### Phase 3: Security Final Review ✅ COMPLETE

#### Task 3.1: Final Security Audit
**Assigned Role:** Security Specialist (Casey Morgan)
**Status:** [x] Complete
**Priority:** HIGH

**Completed:**
- [x] Verify `cargo deny check` passes (all checks) - PASS
- [x] Verify no security advisories - PASS
- [x] Review dependencies updated (rfd 0.15, ashpd 0.11) - PASS
- [x] Verify no unsafe code blocks - PASS (none found)
- [x] Check for any credential/secret exposure - PASS (none found)
- [x] Sign off on security readiness - APPROVED

**Audit Report:** `SECURITY_AUDIT_v1.0.0.md`
**Security Grade:** A

---

#### Task 3.2: Security Documentation Review
**Assigned Role:** Security Specialist (Casey Morgan)
**Status:** [x] Complete (via audit report)
**Priority:** MEDIUM

**Completed:**
- [x] Security documentation reviewed as part of audit
- [x] OWASP analysis included in audit report
- [x] Security features verified and documented

---

### Phase 4: Documentation Updates ✅ COMPLETE

#### Task 4.1: Update Stale Documentation
**Assigned Role:** Documentation Specialist (Sam Parker)
**Status:** [x] Complete
**Priority:** MEDIUM

**Completed:**
- [x] Update ROADMAP.md to reflect v1.0.0 preparation status (`5a12c7b`)
- [x] Update docs/FORMATS.md to reference v0.3.0 features (`6ae322b`)
- [x] Verify all version numbers are consistent
- [x] Clean up outdated content

---

#### Task 4.2: Create v1.0.0 Release Notes Draft
**Assigned Role:** Documentation Specialist (Sam Parker)
**Status:** [x] Complete
**Priority:** HIGH

**Completed:**
- [x] Create RELEASE_NOTES_v1.0.0.md (`7ebf0a3`)
- [x] Document all features (GUI, CLI, formats)
- [x] Document known limitations (STEP FACETED_BREP only, etc.)
- [x] Include upgrade instructions
- [x] Include acknowledgments and credits

---

### Phase 5: Approval & Release Execution ⏳ PENDING

#### Task 5.1: System Architect Final Review
**Assigned Role:** System Architect (Alex Chen)
**Status:** [ ] Not Started
**Priority:** BLOCKING
**Dependencies:** Addendum Tasks A.1–A.3, Tasks 2.1–2.2 (Manual Testing)

**Description:**
Final architecture review and release approval for v1.0.0.

**Acceptance Criteria:**
- [ ] Review all task completion status
- [ ] Verify no release-blocking issues remain (quality gates + scope compliance)
- [ ] Review latest automated results (`cargo test`, `cargo fmt --check`, `cargo clippy`)
- [ ] Review security audit results (Grade A - Approved)
- [ ] Review documentation completeness
- [ ] Approve or reject v1.0.0 release
- [ ] If approved, **review and update** `V1.0.0_RELEASE_CHECKLIST.md` (mark completed items)
- [ ] Sign off document: `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md`

**Blocked By:** Release gates (A.*) and manual testing sign-off

---

#### Task 5.2: Final Release Artifacts (archives + checksums)
**Assigned Role:** Senior Engineer (Jordan Rivera)  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete  
**Priority:** BLOCKING  
**Dependencies:** Tasks 2.3 (cross-platform validation), 3.1 (security audit), A.1 (rustfmt gate)

**Acceptance Criteria:**
- [ ] Windows/macOS/Linux archives produced (portable ZIP/TAR.GZ per `V1.0.0_RELEASE_CHECKLIST.md`)
- [ ] `SHA256SUMS` generated for all archives
- [ ] Archive contents verified (README, LICENSE files, binaries)
- [ ] Documented in (or aligned with) `RELEASE_ARTIFACTS_SPRINT12.md`

---

#### Task 5.3: Version bump + git tag (v1.0.0)
**Assigned Role:** Senior Engineer (Jordan Rivera)  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete  
**Priority:** BLOCKING  
**Dependencies:** Task 5.1 (Architect approval), Phase 4 (docs complete), Task 5.2 (artifacts complete)

**Acceptance Criteria:**
- [ ] Workspace version updated to `1.0.0`
- [ ] Tag created and pushed: `v1.0.0`
- [ ] Version references verified (docs + code)

---

#### Task 5.4: GitHub Release (v1.0.0)
**Assigned Role:** Senior Engineer (Jordan Rivera)  
**Status:** [ ] Not Started / [ ] In Progress / [ ] Complete  
**Priority:** BLOCKING  
**Dependencies:** Task 5.2 (artifacts), Task 5.3 (tag), Phase 4 (release notes draft)

**Acceptance Criteria:**
- [ ] GitHub Release created using tag `v1.0.0`
- [ ] Release artifacts attached (Windows/macOS/Linux + `SHA256SUMS`)
- [ ] Release notes sourced from `RELEASE_NOTES_v1.0.0.md`

---

## Success Criteria for Sprint

- [x] Baseline Phase 1 tasks complete (license fix, dependency update)
- [ ] Release gates green (A.1 rustfmt + A.2 glTF write decision + A.3 mesh two-stage detection)
- [ ] Manual testing 100% complete with sign-off
- [ ] No release-blocking issues remaining (quality gates, scope compliance, critical bugs)
- [x] Security audit passed (Grade A)
- [x] Documentation updated
- [ ] System Architect approval received
- [ ] Ready for v1.0.0 release tagging

**Completed: 3/8 criteria**

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| rustfmt gate failing (`cargo fmt --check`) | Senior Engineer | 🔴 Blocking (Addendum A.1) |
| glTF write correctness (decision + implementation) | System Architect + Junior-3D + Senior Engineer + Researcher | 🔴 Blocking (Addendum A.2) |
| Mesh two-stage detection mismatch (policy + implementation) | System Architect + Junior-3D + Senior Engineer | 🟡 High (Addendum A.3) |
| Manual testing execution + re-test of fixed issues | UI Designer | 🟡 In Progress |
| System Architect approval | System Architect | Blocked by release gates + manual testing |

---

## Timeline

| Date | Milestone | Status |
|------|-----------|--------|
| Jan 3, 2026 | Sprint 12_A begins | ✅ Complete |
| Jan 3, 2026 | Phase 1 (Critical Fixes) complete | ✅ Complete |
| Jan 3, 2026 | Phase 3-4 (Security/Docs) complete | ✅ Complete |
| Jan 5-7, 2026 | Phase 2 (Manual Testing) complete | ⏳ Pending |
| Jan 8-10, 2026 | Phase 5 (Approval & Release Execution) - Sprint 12_A ends | ⏳ Pending |
| Jan 12-15, 2026 | v1.0.0 Release (target window) | ⏳ Pending |

---

## Quality Metrics (Current)

| Metric | Value | Status |
|--------|-------|--------|
| Automated Tests | PASS | ✅ Latest run passing |
| cargo fmt --check | FAIL | 🔴 Formatting drift present |
| Clippy Warnings | 3 | 🟡 All in `converter-gui-modern/src/app.rs` |
| Security Audit | Grade A | ✅ Approved |
| cargo deny | All pass | ✅ Clean |
| Manual Tests | Incomplete | 🟡 In progress (see `MANUAL_TESTING_REPORT_SPRINT12.md`) |

---

## Notes

- Phase 1, 3, and 4 completed ahead of schedule on Day 1
- Addendum (Jan 26) re-opened release gates (rustfmt/clippy + format-policy decisions)
- Manual testing is a critical-path item (and must include re-testing impacted fixes)
- System Architect approval depends on release gates + manual testing completion
- No new features - stability and quality only

---

**Document Version:** 2.0
**Created:** January 3, 2026
**Last Updated:** January 26, 2026
**Author:** Senior Engineer (Jordan Rivera)
**Status:** In Progress - Awaiting Manual Testing
