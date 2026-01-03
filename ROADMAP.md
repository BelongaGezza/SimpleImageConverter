# Project Roadmap
## Simple Image Converter

**Last Updated:** January 3, 2026
**Current Version:** v0.3.0
**Next Release:** v1.0.0 (First Stable Release - Mid-January 2026)

---

## 🎯 Current Status: Sprint 12_A - v1.0.0 Final Release Preparation

### Status Overview

**Current Phase:** Sprint 12_A - Final v1.0.0 Release Preparation
**Sprint Goal:** Complete all remaining tasks for v1.0.0 stable release
**Target Release:** v1.0.0 (January 12-15, 2026)

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
| **Sprint 12_A** | **v1.0.0 final prep** | **🟡 In Progress** |

---

## 🎯 v1.0.0 Scope Definition

**Status:** ✅ **SCOPE DEFINED** (December 30, 2025)
**Target Release:** Mid-January 2026

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

## 🔥 Sprint 12_A Tasks (Current)

### Phase 1: Critical Fixes ✅
- [x] Fix license configuration in deny.toml (OFL-1.1, UFL-1.0 for egui fonts)
- [ ] Assess dependency future-compatibility (ashpd Rust 2024 warning)

### Phase 2: Manual Testing 🟡
- [ ] Execute manual testing checklist (50+ test cases)
- [ ] Fix any critical/high-priority issues found
- [ ] Cross-platform validation (Windows, macOS, Linux)

### Phase 3: Security Review
- [ ] Final security audit
- [ ] Security documentation review

### Phase 4: Documentation
- [x] Update stale ROADMAP.md
- [ ] Update docs/FORMATS.md for v0.3.0 features
- [ ] Draft v1.0.0 release notes

### Phase 5: Release Approval
- [ ] System Architect final review and approval

**Detailed Tasking:** See `AGENT_TASKS/SPRINT_12_A_TASKING.md`

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

### v1.0.0 - First Stable Release (Target: Mid-January 2026)
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

---

## 📊 Quality Metrics

| Metric | Current Status |
|--------|----------------|
| Test Count | 633 tests |
| Test Status | ✅ All passing |
| Clippy | ✅ Clean |
| Security Audit | ✅ Passing |
| License Check | ✅ Passing |
| CI/CD | ✅ All pipelines green |

---

## 📁 Key Reference Documents

### Current Sprint
- `AGENT_TASKS/SPRINT_12_A_TASKING.md` - Current sprint tasking

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
