# Project Roadmap
## Simple Image Converter

**Last Updated:** July 1, 2026
**Current Version:** v0.3.0
**Next Release:** v1.0.0 (First Stable Release — Mid-June 2026)

---

## 🎯 Current Status: Sprint 13 — v1.0.0 Release Execution

### Status Overview

**Current Phase:** Sprint 13 — v1.0.0 Release Execution
**Sprint Goal:** Complete all remaining gates for v1.0.0 stable release
**Sprint Duration:** May 29 – June 12, 2026
**Target Release:** v1.0.0 (Mid-June 2026)
**Canonical Action List:** [`V1.0.0_RELEASE_CHECKLIST.md`](V1.0.0_RELEASE_CHECKLIST.md)  
**Historical Tasking:** [`AGENT_TASKS/SPRINT_13_TASKING.md`](AGENT_TASKS/SPRINT_13_TASKING.md)  
**Architect Review:** [`SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md`](SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md) — CONDITIONAL NO-SHIP pending manual testing, dependency triage, and release execution

### Completed Releases

| Version | Date | Highlights |
|---------|------|------------|
| v0.1.0 | Dec 27, 2025 | Core converters (PNG, JPG, BMP, GIF, STL, OBJ, PLY) |
| v0.1.1 | Dec 27, 2025 | Mesh transforms, normal recalculation, validation |
| v0.2.0 | Dec 29, 2025 | STEP format support (FACETED_BREP), comprehensive docs |
| v0.2.1 | Dec 30, 2025 | GUI application, drag-and-drop, user-friendly errors |
| v0.2.2 | Dec 30, 2025 | Batch processing, preview, settings, conversion history |
| v0.3.0 | Dec 30, 2025 | Parallel batch processing, 3D viewer, auto-save |

### Completed Sprints

| Sprint | Focus | Status |
|--------|-------|--------|
| Sprint 1-6 | Core implementation | ✅ Complete |
| Sprint 7 | GUI foundation | ✅ Complete |
| Sprint 8 | v0.2.1/v0.2.2 release | ✅ Complete |
| Sprint 9 | v0.3.0 features | ✅ Complete |
| Sprint 10/10_A | 3D viewer, testing | ✅ Complete |
| Sprint 11 | GUI polish, quality | ✅ Complete |
| Sprint 12 | Documentation | ✅ Complete |
| Sprint 12_A | v1.0.0 final prep | ✅ Complete (gates A.1–A.3 impl; Senior review deferred to Sprint 13) |
| **Sprint 13** | **v1.0.0 release execution** | **🟡 In Progress** |

---

## 🎯 v1.0.0 Scope Definition

**Status:** ✅ **SCOPE DEFINED** (December 30, 2025)
**Target Release:** Mid-June 2026 (Sprint 13)

### Included in v1.0.0

- ✅ **Core Conversion Features:** All essential 2D image and 3D mesh formats
- ✅ **GUI Application:** Complete, polished interface with all features
- ✅ **Basic Packaging:** Portable archives (ZIP, TAR.GZ) for all platforms
- ✅ **Documentation:** Comprehensive user and developer documentation
- ✅ **Quality & Performance:** Optimized, tested, production-ready

### Deferred to v1.1.0+

- ⏳ **Full STEP B-Rep Support:** opencascade-rs integration (NURBS, curved surfaces)
- ⏳ **Installer Packages:** MSI (Windows), DMG (macOS), DEB (Linux)
- ⏳ **Advanced Packaging:** Package manager distribution (winget, Homebrew, apt)

---

## 🔥 Active v1.0.0 Release Work

The active release todos are maintained in one place: [`V1.0.0_RELEASE_CHECKLIST.md`](V1.0.0_RELEASE_CHECKLIST.md).

Current release path:
1. Refresh public status documentation from the canonical release checklist.
2. Triage dependency audit warnings before release approval.
3. Complete manual GUI validation, prioritizing Windows 11.
4. Produce and verify release artifacts for all target platforms.
5. Complete release sign-off and publish only after all blocking gates are green.

### Sprint 12_A Completed Gates ✅
- [x] Restore `cargo fmt --check` compliance (Addendum A.1)
- [x] glTF write implementation (Addendum A.2 — Senior review pending)
- [x] Mesh two-stage detection implementation (Addendum A.3 — Senior sign-off pending)
- [x] Final security audit (`SECURITY_AUDIT_v1.0.0.md`, Grade A)
- [x] Release notes draft (`RELEASE_NOTES_v1.0.0.md`)

**Historical Sprint 12_A tasking:** [`AGENT_TASKS/SPRINT_12_A_TASKING.md`](AGENT_TASKS/SPRINT_12_A_TASKING.md)

---

## 🔥 Sprint 12_A Tasks (Previous — Superseded by Sprint 13)

### Phase 1: Critical Fixes ✅
- [x] Fix license configuration in deny.toml (OFL-1.1, UFL-1.0 for egui fonts)
- [x] Assess dependency future-compatibility (ashpd Rust 2024 warning)
- [x] **P0** Restore `cargo fmt --check` compliance (formatting drift) — complete May 29, 2026
- [x] **P0** glTF write correctness decision + remediation — implementation complete; Senior review pending
- [x] **P1** Align mesh format detection with two-stage verification — implementation complete; Senior sign-off pending

### Phase 2: Manual Testing 🟡
- [ ] Execute manual testing checklist (50+ test cases)
- [ ] Fix any critical/high-priority issues found
- [ ] Cross-platform validation (Windows, macOS, Linux)

### Phase 3: Security Review ✅
- [x] Final security audit (`SECURITY_AUDIT_v1.0.0.md`)
- [x] Security documentation review (included in audit)

### Phase 4: Documentation ✅
- [x] Update stale ROADMAP.md
- [x] Update docs/FORMATS.md for v0.3.0 features
- [x] Draft v1.0.0 release notes (`RELEASE_NOTES_v1.0.0.md`)

### Phase 5: Approval & Release Execution ⏳
- [ ] System Architect final review and approval
- [ ] Final release artifacts (archives + SHA256SUMS)
- [ ] Version bump + tag `v1.0.0`
- [ ] GitHub Release published with artifacts

---

## ✅ v0.3.0 Features (Released December 30, 2025)

### Parallel Batch Processing
- Concurrent file conversion using thread pool (rayon)
- Configurable concurrency (1-16, default: CPU cores capped at 8)
- Up to 4x speedup on 4-core systems
- Thread-safe queue management

### 3D Mesh Viewer
- Interactive 3D preview with wgpu-based rendering
- Camera controls: orbit, pan, zoom
- Rendering modes: solid, wireframe
- Feature-gated (`viewer-3d` feature flag)

### Settings Auto-Save
- Automatic save 500ms after changes
- Visual status indicator
- Debouncing prevents excessive writes

### Queue Item Editing
- Edit pending queue items
- Change output format, path, options
- Validation before saving

---

## 🔮 Future Releases

### v1.0.0 - First Stable Release (Target: Mid-June 2026)
- All current features production-ready
- Complete documentation
- Full test coverage
- Security reviewed

### v1.1.0 - Full STEP Support (Planned)
- opencascade-rs integration
- NURBS surface tessellation
- Curved surface support

### v1.2.0 - Installer & Advanced Packaging (Planned)
- MSI installer (Windows)
- DMG package (macOS)
- DEB packages (Linux)
- Package manager distribution

### v1.3.0+ - Additional Features (Future)
- Additional formats (TGA, ICO, DDS, HDR, OpenEXR, AVIF, IGES, 3MF)
- Cloud integration
- Web service API

### Research: 2D to 2.5D Relief Conversion (Future Investigation)
- **Goal:** Convert 2D images to 2.5D relief meshes (STL) for lithophanes, CNC, and 3D printing
- **Approach:** Use depth estimation AI models to generate height maps from images
- **Potential Technologies:**
  - [Depth-Anything-V2](https://github.com/DepthAnything/Depth-Anything-V2) - State-of-the-art monocular depth estimation
  - [mcp_3d_relief](https://github.com/Bigchx/mcp_3d_relief) - Python tool for 2D to 3D relief, specifically for STL output
- **Use Cases:**
  - Lithophane generation (3D printed photo panels)
  - CNC relief carving from photographs
  - Artistic relief sculptures from images
- **Implementation Considerations:**
  - May require Python integration or ONNX model inference
  - GPU acceleration for real-time depth estimation
  - Quality settings for mesh resolution/detail

---

## 📊 Quality Metrics

| Metric | Current Status |
|--------|----------------|
| Test Count | 620+ tests |
| Test Status | ✅ All passing (verified May 29, 2026) |
| rustfmt | ✅ `cargo fmt --all --check` passing |
| Clippy | ✅ `cargo clippy --workspace --all-targets -D warnings` clean |
| Security Audit | ✅ Passing (Grade A) |
| License Check | ✅ Passing |
| CI/CD | 🟡 Re-run after Sprint 13 gates complete |

---

## 📁 Key Reference Documents

### Current Sprint
- `AGENT_TASKS/SPRINT_13_TASKING.md` - Current sprint tasking
- `AGENT_TASKS/SPRINT_12_A_TASKING.md` - Previous sprint (historical)

### Architecture
- `docs/ARCHITECTURE.md` - System architecture
- `Phase3_Architecture.md` - Detailed architecture design
- `ARCHITECT_REVIEW_STEP_IMPLEMENTATION.md` - STEP implementation decision

### Documentation
- `docs/GUI_USAGE_GUIDE.md` - GUI user guide
- `docs/FORMATS.md` - Format support matrix
- `docs/PERFORMANCE.md` - Performance characteristics
- `CHANGELOG.md` - Version history

### Packaging
- `PACKAGING_STRATEGY.md` - Distribution strategy
- `scripts/package-*.sh` - Platform packaging scripts

---

## 📝 Notes

- STEP format currently supports FACETED_BREP only (pre-tessellated geometry)
- Full B-Rep support (opencascade-rs) planned for v1.1.0
- 3D viewer is optional (feature-gated)
- All security reviews passed

---

*This roadmap is a living document updated as progress is made.*
