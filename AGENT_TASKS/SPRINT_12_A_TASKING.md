# Sprint 12_A Tasking: v1.0.0 Final Release Preparation

**Sprint Duration:** January 3-10, 2026
**Sprint Goal:** Complete all remaining tasks for v1.0.0 stable release
**Target Release:** v1.0.0 (Mid-January 2026)

---

## CRITICAL: Role Selection (READ FIRST - STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles

Look at the Role Assignment table below. Find a role where:
- Status = `Available`
- No agent is currently assigned

### Step 2: Claim Your Role

Once you find an available role:
1. **Edit this document** to update that role's row:
   - Change Status from `Available` to `In Progress`
   - Add your session identifier to the "Assigned Agent" column
2. **Read the persona file** listed in the "Persona File" column
3. **Adopt that persona** for all remaining work on this tasking

### Step 3: Become Your Role

After claiming, you ARE that agent. Read the persona file and embody:
- The agent's name and identity
- Their expertise and communication style
- Their decision-making authority
- Their specific responsibilities

**IMPORTANT:** If all roles show "In Progress" or "Complete", STOP. Do not proceed.

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Tasks | Dependencies |
|------|--------------|--------|----------------|-------|--------------|
| Senior Engineer (Jordan Rivera) | `.agents/senior-engineer.md` | Available | - | 1.1, 1.2, 2.3 | None |
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | Available | - | 2.1, 2.2 | None |
| Security Specialist (Casey Morgan) | `.agents/security-specialist.md` | Available | - | 3.1, 3.2 | Task 1.1 |
| Documentation Specialist (Sam Parker) | `.agents/documentation-specialist.md` | Available | - | 4.1, 4.2 | None |
| System Architect (Alex Chen) | `.agents/system-architect.md` | Available | - | 5.1 | Tasks 1-4 |

---

## Sprint Overview

**Context:** Sprint 11 and 12 are complete. v0.3.0 is released. This sprint focuses on final v1.0.0 release preparation.

**Key Findings from Senior Engineer Review (January 3, 2026):**
- 633 tests all passing
- Clippy clean, no warnings
- Manual testing is 0% complete (BLOCKING)
- License check failing (deny.toml needs update)
- ashpd future-incompatibility warning (non-blocking)
- ROADMAP.md needs update

---

## Task Breakdown

### Phase 1: Critical Fixes (Priority: BLOCKING)

#### Task 1.1: Fix License Configuration
**Assigned Role:** Senior Engineer (Jordan Rivera)
**Status:** [ ] Not Started
**Priority:** HIGH - Blocking CI

**Description:**
`cargo deny check licenses` fails because font licenses used by egui are not in the allowlist.

**Acceptance Criteria:**
- [ ] Add `OFL-1.1` to `deny.toml` allow list
- [ ] Add exception for `LicenseRef-UFL-1.0` (non-SPDX font license)
- [ ] Verify `cargo deny check` passes
- [ ] Commit changes

**Technical Details:**
- Font licenses are standard OSI-approved licenses used for fonts embedded in egui
- OFL-1.1 = SIL Open Font License 1.1
- UFL-1.0 = Ubuntu Font License

---

#### Task 1.2: Assess Dependency Future-Compatibility
**Assigned Role:** Senior Engineer (Jordan Rivera)
**Status:** [ ] Not Started
**Priority:** MEDIUM - Non-blocking for v1.0.0

**Description:**
`ashpd v0.8.1` has Rust 2024 edition incompatibility. Transitive dependency from `rfd`.

**Acceptance Criteria:**
- [ ] Check if `rfd` has newer version with updated `ashpd`
- [ ] If update available, update and test
- [ ] If no update available, document in known issues for v1.0.0
- [ ] Create GitHub issue for tracking

**Notes:**
- This is non-blocking for v1.0.0 (Rust 2024 edition not required)
- Will become blocking when project adopts Rust 2024 edition

---

### Phase 2: Manual Testing (Priority: HIGH)

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

**Description:**
Address any critical or high-priority issues found during manual testing.

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

**Description:**
Validate release binaries work correctly on all target platforms.

**Acceptance Criteria:**
- [ ] Build release binaries for Windows x64
- [ ] Build release binaries for macOS x64 (if available)
- [ ] Build release binaries for Linux x64
- [ ] Test basic conversion on each platform
- [ ] Verify GUI launches correctly on each platform
- [ ] Document any platform-specific issues

---

### Phase 3: Security Final Review (Priority: HIGH)

#### Task 3.1: Final Security Audit
**Assigned Role:** Security Specialist (Casey Morgan)
**Status:** [ ] Not Started
**Priority:** HIGH
**Dependencies:** Task 1.1

**Description:**
Final security review before v1.0.0 release.

**Acceptance Criteria:**
- [ ] Verify `cargo deny check` passes (all checks)
- [ ] Verify `cargo audit` passes
- [ ] Review any new dependencies added since v0.3.0
- [ ] Verify no unsafe code blocks added
- [ ] Check for any credential/secret exposure
- [ ] Sign off on security readiness

---

#### Task 3.2: Security Documentation Review
**Assigned Role:** Security Specialist (Casey Morgan)
**Status:** [ ] Not Started
**Priority:** MEDIUM

**Description:**
Review security documentation is up-to-date for v1.0.0.

**Acceptance Criteria:**
- [ ] Review SECURITY.md is current
- [ ] Review docs/THREAT_MODEL.md is current
- [ ] Review docs/SECURE_BY_DESIGN_GUIDANCE.md is current
- [ ] Verify security contact information is correct
- [ ] Document security release process

---

### Phase 4: Documentation Updates (Priority: MEDIUM)

#### Task 4.1: Update Stale Documentation
**Assigned Role:** Documentation Specialist (Sam Parker)
**Status:** [ ] Not Started
**Priority:** MEDIUM

**Description:**
Several documents have stale information referencing old sprint status.

**Acceptance Criteria:**
- [ ] Update ROADMAP.md to reflect v1.0.0 preparation status
- [ ] Update docs/FORMATS.md to reference v0.3.0 features
- [ ] Verify all version numbers are consistent (0.3.0 current, 1.0.0 target)
- [ ] Remove or archive outdated sprint documents
- [ ] Update any broken links

---

#### Task 4.2: Create v1.0.0 Release Notes Draft
**Assigned Role:** Documentation Specialist (Sam Parker)
**Status:** [ ] Not Started
**Priority:** HIGH

**Description:**
Draft release notes for v1.0.0 stable release.

**Acceptance Criteria:**
- [ ] Create RELEASE_NOTES_v1.0.0.md
- [ ] Document all features since v0.3.0
- [ ] Document any breaking changes (none expected)
- [ ] Document known limitations
- [ ] Include upgrade instructions (from v0.3.0)
- [ ] Include acknowledgments

---

### Phase 5: Release Approval (Priority: BLOCKING)

#### Task 5.1: System Architect Final Review
**Assigned Role:** System Architect (Alex Chen)
**Status:** [ ] Not Started
**Priority:** BLOCKING
**Dependencies:** Tasks 1.1, 1.2, 2.1, 2.2, 3.1, 3.2, 4.1, 4.2

**Description:**
Final architecture review and release approval for v1.0.0.

**Acceptance Criteria:**
- [ ] Review all task completion status
- [ ] Verify no critical issues remain
- [ ] Review test coverage and results
- [ ] Review security audit results
- [ ] Review documentation completeness
- [ ] Approve or reject v1.0.0 release
- [ ] If approved, create release checklist
- [ ] Sign off document: SYSTEM_ARCHITECT_V1.0.0_APPROVAL.md

---

## Agent Persona Reference

| Role | Persona File | Key Expertise |
|------|--------------|---------------|
| System Architect | `.agents/system-architect.md` | Architecture, design decisions |
| Senior Engineer | `.agents/senior-engineer.md` | Core implementation, code reviews |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | 2D image formats |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | 3D mesh formats |
| Security Specialist | `.agents/security-specialist.md` | Security reviews |
| Documentation Specialist | `.agents/documentation-specialist.md` | API docs, user guides |
| Researcher | `.agents/researcher.md` | Ecosystem monitoring |
| UI Designer | `.agents/ui-designer.md` | GUI design, egui |

---

## Success Criteria for Sprint

- [ ] All Phase 1 tasks complete (license fix)
- [ ] Manual testing 100% complete with sign-off
- [ ] No critical issues remaining
- [ ] Security audit passed
- [ ] Documentation updated
- [ ] System Architect approval received
- [ ] Ready for v1.0.0 release tagging

---

## Timeline

| Date | Milestone |
|------|-----------|
| Jan 3, 2026 | Sprint 12_A begins |
| Jan 5, 2026 | Phase 1 (Critical Fixes) complete |
| Jan 7, 2026 | Phase 2 (Manual Testing) complete |
| Jan 8, 2026 | Phase 3-4 (Security/Docs) complete |
| Jan 10, 2026 | Phase 5 (Approval) - Sprint 12_A ends |
| Jan 12-15, 2026 | v1.0.0 Release (target window) |

---

## Notes

- This sprint is focused and scoped specifically for v1.0.0 release
- No new features - stability and quality only
- Any feature requests should be documented for v1.1.0
- Quick decisions needed to stay on timeline

---

**Document Version:** 1.0
**Created:** January 3, 2026
**Author:** Senior Engineer (Jordan Rivera)
**Status:** Ready for Role Assignment
