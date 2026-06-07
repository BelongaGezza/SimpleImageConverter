# Sprint 13 Tasking: v1.0.0 Release Execution

**Sprint Duration:** May 29 – June 12, 2026  
**Sprint Goal:** Complete all remaining gates for v1.0.0 stable release  
**Target Release:** v1.0.0 (Mid-June 2026)  
**Last Updated:** May 29, 2026  
**Architect Review:** `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md`

---

## CRITICAL: Role Selection (READ FIRST)

**You are an unassigned agent. Claim a role before proceeding.**

1. Find a role in the table below with Status = `Available`
2. Edit this document: set Status to `In Progress`, add your session ID to Assigned Agent
3. Read the persona file and adopt that role for all work

If all roles are In Progress or Complete, STOP — no unclaimed work remains.

---

## Sprint Progress Summary

| Phase | Status | Completion |
|-------|--------|------------|
| Phase 1: Code Hardening | ✅ Complete | 100% |
| Phase 2: Reviews & Sign-off | ✅ Complete | 100% (2.1–2.3 done) |
| Phase 3: Manual Testing | 🟡 In Progress | ~50% (macOS: images ✅, shortcuts ✅; meshes ⏳) |
| Phase 4: Release Execution | 🟡 In Progress | ~40% (macOS 4.1–4.2 done) |
| Phase 5: Documentation Refresh | ✅ Complete | 100% (5.2, 5.3 done) |

**Ship Decision:** CONDITIONAL NO-SHIP (see architect review)

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Owned Tasks |
|------|--------------|--------|----------------|-------------|
| System Architect (Alex Chen) | `.agents/system-architect.md` | Complete | Alex Chen (Task 2.1) | 2.1, 5.1 |
| Senior Engineer (Jordan Rivera) | `.agents/senior-engineer.md` | In Progress | Sprint 13 dispatch | 2.2, 2.3, 4.1, 4.2, 4.3 |
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | In Progress | Sprint 13 dispatch | 3.1, 3.2 |
| Junior Engineer 3D (Alex Rivera) | `.agents/junior-engineer-3d.md` | Complete | Alex Rivera (Task 1.1) | 1.1, 2.2 (support) |
| Junior Engineer 2D (Sam Kim) | `.agents/junior-engineer-2d.md` | Complete | Sprint 13 subagent | 1.2 |
| Documentation Specialist (Sam Parker) | `.agents/documentation-specialist.md` | Complete | Sam Parker (Task 5.3) | 5.2, 5.3 |
| Security Specialist (Casey Morgan) | `.agents/security-specialist.md` | Complete | Sprint 12 | Re-consult if scope changes |

---

## Phase 1: Code Hardening (Parallel — No Dependencies)

### Task 1.1: Add glTF/DXF Integration Tests
**Assigned Role:** Junior Engineer 3D  
**Priority:** HIGH  
**Status:** [ ] Not Started / [ ] In Progress / [x] Complete

**Description:**
Add round-trip integration tests for glTF/GLB and DXF in `mesh-core/tests/integration.rs`. Follow existing STL/OBJ/PLY test patterns.

**Acceptance Criteria:**
- [x] glTF embedded write → read round-trip test passes
- [x] GLB write → read round-trip test passes
- [x] DXF write → read round-trip test passes (documented limitation: 3DFACE quad triangulation expands 1 triangle → 4 vertices / 2 faces)
- [x] `cargo test --workspace` passes
- [x] `cargo clippy --workspace -- -D warnings` passes

**Reference:** `mesh-core/src/formats/gltf.rs`, ADR-002 in architect review

---

### Task 1.2: Implement `get_reader_with_limits` in img-core
**Assigned Role:** Junior Engineer 2D  
**Priority:** HIGH  
**Status:** [ ] Not Started / [ ] In Progress / [x] Complete

**Description:**
Mirror the `mesh-core` pattern: add `FormatRegistry::get_reader_with_limits` and propagate CLI `--max-dimension` / `--max-file-size-mb` from `img-convert` into format readers.

**Acceptance Criteria:**
- [x] `get_reader_with_limits` added to `img-core/src/formats/registry.rs`
- [x] All format readers accept injected `ResourceLimits`
- [x] `img-convert` CLI passes configured limits to readers
- [x] Existing security tests pass; add test for custom max-dimension enforcement
- [x] `cargo test --workspace` and clippy pass

**Reference:** `mesh-core/src/formats/registry.rs` (pattern to follow)

---

### Task 1.3: Wire or Remove Orphaned CLI Tests
**Assigned Role:** Senior Engineer  
**Priority:** MEDIUM  
**Status:** [ ] Not Started / [ ] In Progress / [x] Complete

**Description:**
Root `tests/cli_tests.rs` is not attached to any crate and never runs with `cargo test --workspace`. Wire into `img-convert/tests/` and `mesh-convert/tests/`, or delete if redundant.

**Acceptance Criteria:**
- [x] CLI tests run via `cargo test --workspace` OR file removed with justification
- [x] No duplicate coverage without reason

**Resolution (May 29, 2026):** Wired — split orphaned tests into `img-convert/tests/cli_integration.rs` (2 tests) and `mesh-convert/tests/cli_integration.rs` (8 tests). Removed `#[ignore]` and use `CARGO_BIN_EXE_*` for reliable binary discovery. Deleted root `tests/cli_tests.rs`.

---

## Phase 2: Reviews & Sign-off

### Task 2.1: Publish ADR-003 to Architecture Docs
**Assigned Role:** System Architect  
**Priority:** HIGH  
**Status:** [x] Complete (May 29, 2026)

**Acceptance Criteria:**
- [x] Mesh two-stage detection policy added to `Phase3_Architecture.md` §12
- [x] `rust-resources.md` mesh detection section updated
- [x] Cross-platform scope and 7-crate workspace noted in Phase3 header

---

### Task 2.2: glTF Senior Review + Validator Run
**Assigned Role:** Senior Engineer (+ Junior 3D support)  
**Priority:** **BLOCKING**  
**Status:** [x] Complete (May 29, 2026)

**Acceptance Criteria:**
- [x] Code review of `mesh-core/src/formats/gltf.rs` complete
- [x] Khronos `gltf_validator` run on exported `.glb` and embedded `.gltf` fixtures — **not installed**; parse-based validation via `gltf::import_slice` round-trip tests used instead (see `GLTF_SENIOR_REVIEW_SPRINT13.md`)
- [x] Zero validator errors; warnings triaged — N/A (validator unavailable); 16 gltf + 3 glb unit tests + 4 integration round-trips pass
- [x] Sign-off recorded in this document

**Resolution (May 29, 2026):** Senior review approved ADR-002 contract. GLB self-contained and glTF embedded-base64 paths verified. Full sign-off in `GLTF_SENIOR_REVIEW_SPRINT13.md`.

**Validator steps:** See `AGENT_TASKS/SPRINT_12_A_TASKING.md` Task A.2.3

---

### Task 2.3: Mesh Detection Sign-off (ADR-003)
**Assigned Role:** Senior Engineer  
**Priority:** HIGH  
**Status:** [x] Complete (May 29, 2026)

**Acceptance Criteria:**
- [x] Implementation matches ADR-003 tiered policy
- [x] Spoofing/mismatch tests verified (`cargo test -p mesh-core registry` — 26 passed; mismatch tests for GLB/glTF, PLY/OFF)
- [x] Sign-off recorded in this document

**Resolution (May 29, 2026):** `mesh-core/src/formats/registry.rs` matches ADR-003 tiered policy. Full sign-off in `GLTF_SENIOR_REVIEW_SPRINT13.md` § Task 2.3.

---

## Phase 3: Manual Testing (BLOCKING — Human Required)

### Task 3.1: Execute Manual Testing Re-test
**Assigned Role:** UI Designer  
**Priority:** **BLOCKING**  
**Status:** [ ] Not Started / [x] In Progress / [ ] Complete

**Description:**
Complete re-test of Tasks 2.4/2.5 and remaining Sprint 12 manual checklist items on Windows 11 and macOS.

**Reference:** `MANUAL_TESTING_REPORT_SPRINT12.md`, `MANUAL_TESTING_CHECKLIST_SPRINT13.md`

**Progress (May 29, 2026 — Jamie Chen):**

| Area | Automated (this session) | Human required |
|------|--------------------------|----------------|
| `cargo test -p converter-gui --workspace` | ✅ 194 tests, 0 failed | — |
| macOS release build | ✅ `cargo build --release -p converter-gui` | — |
| Shortcut/help code audit | ✅ Handler ↔ help_panel match | Key press on real OS |
| Sprint 11 + integration tests | ✅ Batch queue edit, settings save | Visual UX |
| Checklist & report addendum | ✅ Created/updated | Execute checklist |
| Test 5.3 edit dialog re-test | ✅ Pass macOS (shortcuts pass) | ⏳ Windows |
| Image conversions (single + batch) | ✅ Pass macOS | — |
| Mesh conversions | ✅ Automated GUI tests (STL→OBJ/PLY) | ⏳ Human manual sign-off pending |
| Keyboard shortcuts §1 | ✅ Pass macOS | ⏳ Windows |
| Windows 11 full pass | — | ⏳ Not run this session |
| Linux GUI | — | ⏳ Out of scope (macOS session) |

**Acceptance Criteria:**
- [x] All keyboard shortcut tests (1.1–5.5) Pass — **macOS** (May 29, 2026); Windows pending
- [ ] Help system tests (6.1–6.2) Pass
- [ ] UI consistency tests (7.1–7.5) Pass
- [x] Edit queue re-test (5.3) Pass — **macOS** (included in shortcut pass)
- [x] Image format conversions Pass — **macOS** (single + batch)
- [ ] Mesh format conversions Pass — **pending**
- [ ] Exit-save / settings persist re-tests Pass
- [ ] Zero Critical/High open issues

**Handoff:** Human tester runs `MANUAL_TESTING_CHECKLIST_SPRINT13.md` on macOS and Windows 11; record results in Sprint 12 report Sprint 13 addendum. Do **not** mark Complete until P1–P4 re-tests pass.

---

### Task 3.2: Cross-Platform GUI Smoke Test
**Assigned Role:** UI Designer (+ Senior Engineer for builds)  
**Priority:** HIGH  
**Status:** [ ] Not Started / [x] In Progress / [ ] Complete

**Progress (May 29, 2026):** Smoke test matrix added to `MANUAL_TESTING_CHECKLIST_SPRINT13.md` §8–§7. macOS release build verified; GUI smoke not executed (requires human). Windows/Linux builds need separate sessions.

**Acceptance Criteria:**
- [ ] Single-file conversion works on Win/macOS/Linux
- [ ] Batch conversion with pause/resume/cancel works
- [ ] Settings persist across restart

---

## Phase 4: Release Execution (After Phase 2 + 3)

### Task 4.1: Cross-Platform Release Builds
**Assigned Role:** Senior Engineer  
**Priority:** **BLOCKING**  
**Dependencies:** Tasks 2.2, 2.3, 3.1 (3.1 does not block macOS build prep)

**Status:** 🟡 Partial — **macOS ARM64 complete** (May 29, 2026); Windows/Linux pending

**macOS (complete):**
- [x] `cargo test --workspace`, clippy, fmt — see `RELEASE_BUILD_REPORT_SPRINT13.md`
- [x] Release build `aarch64-apple-darwin`: `converter-gui`, `img-convert`, `mesh-convert`
- [x] CLI smoke tests (help + PNG→BMP, STL→OBJ)

**Other platforms:** ⏳ Not run this session

---

### Task 4.2: Packaging + SHA256 Checksums
**Assigned Role:** Senior Engineer  
**Scripts:** `scripts/package-windows.ps1`, `package-macos.sh`, `package-linux.sh`

**Status:** 🟡 Partial — **macOS complete** (May 29, 2026); Windows/Linux pending

**macOS (complete):**
- [x] `bash scripts/package-macos.sh 0.3.0 aarch64-apple-darwin`
- [x] `bash scripts/package-gui-macos.sh 0.3.0 aarch64-apple-darwin`
- [x] Archive contents verified; SHA256 in `release/SHA256SUMS-macos-0.3.0.txt`

**Other platforms:** ⏳ Pending respective OS sessions

---

### Task 4.3: Version Bump, Tag, GitHub Release
**Assigned Role:** Senior Engineer  
**Dependencies:** Task 5.1 (Architect approval), Tasks 3.1–3.2, full cross-platform 4.1–4.2

**Status:** ⏳ **Blocked** — do not tag `v1.0.0` or publish GitHub Release until gates green

---

## Phase 5: Documentation

### Task 5.1: Architect Formal Sign-off
**Assigned Role:** System Architect  
**Dependencies:** All blocking gates green

Update `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md` status to ✅ APPROVED.

---

### Task 5.2: Refresh Stale Status Docs
**Assigned Role:** Documentation Specialist  
**Priority:** HIGH  
**Status:** [x] Complete (May 29, 2026)

**Acceptance Criteria:**
- [x] `AGENT_TASKS/SPRINT_12_A_TASKING.md` gate statuses updated (A.1 complete, A.2/A.3 impl complete pending review)
- [x] `ROADMAP.md` dates and sprint status current
- [x] `V1.0.0_RELEASE_CHECKLIST.md` checkboxes reflect May 2026 reality
- [x] `SECURITY_RISK_REGISTER.md` Last Updated → May 29, 2026; RISK-006–009 added

---

### Task 5.3: Release Notes Final Review
**Assigned Role:** Documentation Specialist  
**Priority:** HIGH  
**Status:** [x] Complete (May 29, 2026)  
**Reference:** `RELEASE_NOTES_v1.0.0.md`

**Description:**
Verify release notes against `docs/FORMATS.md`, Sprint 13 hardening work, and `V1.0.0_SCOPE.md`; update checklist documentation gates.

**Acceptance Criteria:**
- [x] Format matrix verified against `docs/FORMATS.md`
- [x] Sprint 13 items documented (img-core `get_reader_with_limits`, glTF/GLB/DXF integration tests, CLI tests wired, ADR-003)
- [x] Incorrect claims corrected (test count, release timeline, CLI examples, credits, STEP feature gate, DXF limitation)
- [x] `V1.0.0_RELEASE_CHECKLIST.md` documentation section updated
- [x] Discrepancies recorded in task completion notes

**Resolution (May 29, 2026):** Updated `RELEASE_NOTES_v1.0.0.md` — test count 633→657, release date January→Mid-June 2026, sprint count 12→13, CLI syntax corrected, Sprint 13 hardening section added, security/known-limitations aligned with ADR-003 and scope doc; Junior 3D credit corrected to Alex Rivera.

---

## Immediate Parallel Dispatch (May 29, 2026 — Round 2)

| Task | Agent | Result |
|------|-------|--------|
| 2.2 glTF Senior review | Senior Engineer | ✅ Complete — `GLTF_SENIOR_REVIEW_SPRINT13.md` |
| 2.3 Mesh detection sign-off | Senior Engineer | ✅ Complete — ADR-003 verified |
| 5.3 Release notes review | Documentation Specialist | ✅ Complete — `aaad2a4` |
| 3.1–3.2 Manual testing prep | UI Designer | 🟡 In Progress — `MANUAL_TESTING_CHECKLIST_SPRINT13.md`; human re-test required |

**Next blocking item:** Task 3.1 human manual testing, Task 5.1 architect approval, then Windows/Linux 4.1–4.2 and Task 4.3.

## Immediate Parallel Dispatch (May 29, 2026 — Round 1)

The following tasks were dispatched to subagents immediately after architect review commit:

| Task | Agent | Dispatch |
|------|-------|----------|
| 1.1 glTF/DXF integration tests | Junior 3D | Subagent |
| 1.2 img-core get_reader_with_limits | Junior 2D | Subagent |
| 1.3 Wire CLI tests | Senior Engineer | Subagent |
| 5.2 Documentation refresh | Documentation Specialist | Subagent |

Tasks 3.1–3.2 require human GUI interaction and cannot be automated by subagents.

---

*Canonical release scope: `V1.0.0_SCOPE.md`. Checklist: `V1.0.0_RELEASE_CHECKLIST.md`.*
