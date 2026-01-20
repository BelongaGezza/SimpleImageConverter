# After-Action Review (AAR) — SimpleImageConverter

**Purpose:** Capture learnings from this development so we can run the next Rust project more effectively. Use this document to gather decisions, lessons learned, reusable artifacts, and recommended working practices.

**Scope:** Post-mortem / continuous AAR for the SimpleImageConverter project. Items marked with ⬜ are action items to complete.

**Maintainers:** System Engineer; Project Manager (Gerry Gillies)

---

## 📌 Snapshot
- **Repository:** SimpleImageConverter
- **Date created:** January 5, 2026
- **Author:** Gerry Gillies

---

## ✅ Files to reuse (suitable for new Rust repo — *scrub project-specific references before reuse*)
- `README.md` — general repo overview and usage; remove product-specific examples and replace with template commands.
- `QUICK_START.md` — setup steps and environment; keep bootstrapping steps (rustup, cargo) and CI hints.
- `IMPLEMENTATION_PLAN.md` — sprint template, ceremonies, and backlog structure.
- `Architecture.md` — architecture patterns, component responsibilities, interfaces.
- `Full_Specification.md` — spec organization and format support examples (strip format-specific content as needed).
- `AI_DEVELOPMENT_GUIDE.md` — AI coordination patterns and human-in-the-loop rules (generalize agent names and tooling).
- `PROJECT_SUMMARY.md` — high-level project brief and scope template.
- `research_outputs.md` — a token efficient record of relevant research conducted.
- `rust-resources.md` — an updating summary of deprecated practices, current versions, good practices and gotchas.
- `CONTRIBUTING.md` — contribution workflow, CLA, coding standards, PR review guidelines.
- `Cargo.toml` — dependency manifest to seed new project (scrub project name, version, authors).
- `LICENSE-MIT` / `LICENSE-APACHE` — license choices and distribution notes.
- `THIRD_PARTY_LICENSES.md` — third-party inventory and maintenance note (maintainer: System Engineer).
- `CHANGELOG.md` & `RELEASE_NOTES*.md` — release note templates and changelog format.
- `.gitignore`, `MANIFEST.md`, `INSTALL.txt` — packaging and release artifacts.
- `docs/` or `docs/RUSTSTEP_GUIDANCE.md` — useful docs examples.
- `tests/` & `examples/` — testing patterns and example usage.
- `PACKAGING_SCRIPT_*`, `BUILD_LINUX_RELEASES.md` — packaging and release automation guides.
- Security & compliance docs: `SECURITY.md`, `SECURITY_AUDIT_v0.0.0.md`, `deny.toml`.

> Tip: When reusing, replace product names, project timelines, and specific AI agent references with neutral placeholders.

---

## 🧭 Working Practices — AI Development & Agents
- Agent Roles: define roles (Researcher, Code Agent, Reviewer, CI Agent) with clear responsibilities and an owner for each.
- Human-in-the-loop: every substantive change suggested by an AI agent must be reviewed and signed off by a human owner.
- Task generation: tasks created by agents must include: goal, acceptance criteria, required files, and reviewer.
- Tasking process: use the sprint board for assignment, add labels (`agent-suggested`, `needs-review`, `security`) and estimate time.
- Artifacts: every AI-driven change must include a minimal reproducible test or validation step.
- Transparency: log agent prompts or summary notes in PR descriptions or dedicated `AI-Audit/` folder.

---

## 🔁 Tasking & Status Tracking
- Use sprints (2-week cadence) with explicit Definition of Done (DoD) per ticket.
- Status fields: `not-started`, `in-progress`, `in-review`, `blocked`, `done`.
- Daily / tri-weekly sync notes: short status updates with blockers and decisions.
- Verification Checklist pattern: include per-release verification items (licenses, tests, packaging, security checks).

---

## ✅ Review & Approval Gates
- Sprint review: Demo, QA sign-off, Security & PM acknowledgment.
- Phase gate: Architecture review, Security audit, Release readiness checklist.
- Release approval: System Engineer sign-off on license and packaging, PM sign-off on scope, Security approval.

**Approval template:**
- Gate: (Sprint/Phase/Release)
- Date: 
- Approved by: 
- Items verified: [ ] Docs, [ ] Licenses, [ ] Tests, [ ] Packaging, [ ] Security

---

## 📋 Lessons Learned — Template & Entries
Use the template below to capture lessons as they are discovered.

### Lesson Template
- **ID:** AAR-XXX
- **Date:** YYYY-MM-DD
- **Title:** Short descriptive title
- **Area:** (process / tooling / architecture / security / other)
- **Summary:** One-sentence summary
- **Details:** Detailed explanation, evidence, and impact
- **Decision / Action:** What was changed or will be changed
- **Owner:** Person responsible
- **Status:** `open` / `in-progress` / `resolved`

### Lessons Log
| ID | Date | Title | Area | Owner | Status |
|----|------|-------|------|-------|--------|
| AAR-001 | 2026-01-05 | License files maintained centrally | process | System Engineer | resolved |
| AAR-002 | 2026-01-20 | egui keyboard shortcuts: use key_pressed() not keys_down.contains() | tooling | Dr. Taylor Kim | resolved |

> Append new lessons below with full template details.

#### AAR-002: egui keyboard shortcuts: use key_pressed() not keys_down.contains()
- **ID:** AAR-002
- **Date:** 2026-01-20
- **Title:** egui keyboard shortcuts: use key_pressed() not keys_down.contains()
- **Area:** tooling
- **Summary:** Using `keys_down.contains()` for keyboard shortcuts with modifiers causes false triggers when modifier keys are held down alone. Must use `key_pressed()` instead.
- **Details:** When implementing cross-platform keyboard shortcuts in egui (Cmd on macOS, Ctrl on Windows/Linux), the initial implementation used `keys_down.contains()` which checks if a key is currently in the "down" state. This caused shortcuts to trigger incorrectly when only the modifier key (Command/Ctrl) was pressed, because `keys_down` can include modifier keys or other keys that happen to be down. The correct pattern is to use `ctx.input(|i| i.key_pressed(egui::Key::X))` which only returns true when the key was pressed THIS frame, not when it's held down.
- **Decision / Action:** Fixed all keyboard shortcuts to use `key_pressed()` pattern. Updated rust-resources.md with correct pattern and gotcha warning.
- **Owner:** Dr. Taylor Kim
- **Status:** resolved

**Correct Pattern:**
```rust
let modifiers = ctx.input(|i| i.modifiers);
let cmd_or_ctrl = modifiers.command || modifiers.ctrl;

// CORRECT - Use key_pressed()
if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::O)) {
    // Handle shortcut
}

// WRONG - Causes false triggers
// if cmd_or_ctrl && pressed_keys.contains(&egui::Key::O) { ... }
```

---

## 🔍 Decisions & Rationale
- Capture all architectural and significant process decisions here, include links to PRs, issues, or meeting notes.
- Decision entry format: **Decision ID**, **Date**, **Summary**, **Rationale**, **Impacted components**, **Status**.

---

## 🗂️ Notes & Running Log
Use this section to append short notes during the remainder of the project.

- [2026-01-05] File created and seeded with candidate reusable files and templates. (Author: _add name_)

_Add new notes with date and short summary._

---

## ⬜ Action Items (initial)
- [ ] Review this AAR with PM and System Engineer and assign owners. (PM)
- [ ] Create repository template using the selected reusable files and scrubbed content. (System Engineer)
- [ ] Seed issues for top 5 lessons to track improvements. (Team)

---

## 🔧 Appendix — Quick repo bootstrap checklist for a new Rust project
1. `cargo init --vcs git` and add `LICENSE-*`, `README.md`, `CONTRIBUTING.md`, `.gitignore`.
2. Copy reusable docs listed above; scrub names and product-specific content.
3. Add `THIRD_PARTY_LICENSES.txt` template and assign maintainer.
4. Add CI templates and release packaging scripts.
5. Seed `IMPLEMENTATION_PLAN.md` with sprint 0 (setup) and sprint 1 (core features).

---

_Last updated: January 5, 2026_




