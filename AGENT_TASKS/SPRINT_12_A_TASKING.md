# Sprint 12_A Tasking: v1.0.0 Final Release Preparation

**Sprint Duration:** January 3-10, 2026
**Sprint Goal:** Complete all remaining tasks for v1.0.0 stable release
**Target Release:** v1.0.0 (Mid-January 2026)
**Last Updated:** January 3, 2026

---

## Sprint Progress Summary

| Phase | Status | Completion |
|-------|--------|------------|
| Phase 1: Critical Fixes | ✅ Complete | 100% |
| Phase 2: Manual Testing | ⏳ Pending | 0% |
| Phase 3: Security Review | ✅ Complete | 100% |
| Phase 4: Documentation | ✅ Complete | 100% |
| Phase 5: Release Approval | ⏳ Pending | 0% |

**Overall Sprint Progress: 60%**

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Tasks | Completion |
|------|--------------|--------|----------------|-------|------------|
| Senior Engineer (Jordan Rivera) | `.agents/senior-engineer.md` | Complete | Session Jan 3 | 1.1, 1.2, 2.3 | 2/3 |
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | Available | - | 2.1, 2.2 | 0/2 |
| Security Specialist (Casey Morgan) | `.agents/security-specialist.md` | Complete | Session Jan 3 | 3.1, 3.2 | 2/2 |
| Documentation Specialist (Sam Parker) | `.agents/documentation-specialist.md` | Complete | Session Jan 3 | 4.1, 4.2 | 2/2 |
| System Architect (Alex Chen) | `.agents/system-architect.md` | Available | - | 5.1 | 0/1 |

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

### Phase 2: Manual Testing ⏳ PENDING

#### Task 2.1: Execute Manual Testing Checklist
**Assigned Role:** UI Designer (Jamie Chen)
**Status:** [ ] Not Started
**Priority:** HIGH - Blocking v1.0.0

**Description:**
Code verification is complete, but manual testing is 0% complete per `MANUAL_TESTING_REPORT_SPRINT12.md`.

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
**Status:** [ ] Not Started
**Priority:** HIGH - Depends on Task 2.1

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

---

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

### Phase 5: Release Approval ⏳ PENDING

#### Task 5.1: System Architect Final Review
**Assigned Role:** System Architect (Alex Chen)
**Status:** [ ] Not Started
**Priority:** BLOCKING
**Dependencies:** Tasks 2.1, 2.2 (Manual Testing)

**Description:**
Final architecture review and release approval for v1.0.0.

**Acceptance Criteria:**
- [ ] Review all task completion status
- [ ] Verify no critical issues remain
- [ ] Review test coverage and results (633 tests passing)
- [ ] Review security audit results (Grade A - Approved)
- [ ] Review documentation completeness
- [ ] Approve or reject v1.0.0 release
- [ ] If approved, create release checklist
- [ ] Sign off document: SYSTEM_ARCHITECT_V1.0.0_APPROVAL.md

**Blocked By:** Manual testing (Task 2.1) must complete first

---

## Success Criteria for Sprint

- [x] All Phase 1 tasks complete (license fix, dependency update)
- [ ] Manual testing 100% complete with sign-off
- [x] No critical issues remaining (from automated checks)
- [x] Security audit passed (Grade A)
- [x] Documentation updated
- [ ] System Architect approval received
- [ ] Ready for v1.0.0 release tagging

**Completed: 4/7 criteria**

---

## Current Blockers

| Blocker | Owner | Status |
|---------|-------|--------|
| Manual testing not started | UI Designer | Awaiting assignment |
| System Architect approval | System Architect | Blocked by manual testing |

---

## Timeline

| Date | Milestone | Status |
|------|-----------|--------|
| Jan 3, 2026 | Sprint 12_A begins | ✅ Complete |
| Jan 3, 2026 | Phase 1 (Critical Fixes) complete | ✅ Complete |
| Jan 3, 2026 | Phase 3-4 (Security/Docs) complete | ✅ Complete |
| Jan 5-7, 2026 | Phase 2 (Manual Testing) complete | ⏳ Pending |
| Jan 8-10, 2026 | Phase 5 (Approval) - Sprint 12_A ends | ⏳ Pending |
| Jan 12-15, 2026 | v1.0.0 Release (target window) | ⏳ Pending |

---

## Quality Metrics (Current)

| Metric | Value | Status |
|--------|-------|--------|
| Automated Tests | 633 | ✅ All passing |
| Clippy Warnings | 0 | ✅ Clean |
| Security Audit | Grade A | ✅ Approved |
| cargo deny | All pass | ✅ Clean |
| Manual Tests | 0% | ⏳ Pending |

---

## Notes

- Phase 1, 3, and 4 completed ahead of schedule on Day 1
- Manual testing is the critical path item
- System Architect approval depends on manual testing completion
- No new features - stability and quality only

---

**Document Version:** 2.0
**Created:** January 3, 2026
**Last Updated:** January 3, 2026
**Author:** Senior Engineer (Jordan Rivera)
**Status:** In Progress - Awaiting Manual Testing
