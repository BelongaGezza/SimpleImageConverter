# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Sprint 11 - v1.0.0 Preparation (In Progress)

**Error Message Improvements (Task 2.2):**
- ✅ User-friendly error messages with actionable guidance
- ✅ Context-aware error messages that adapt based on error type
- ✅ Security-first error sanitization (no path or system information leakage)
- ✅ Comprehensive error message mapping for all error types
- ✅ Security review approved (see `SECURITY_REVIEW_TASK_2.2.md`)
- ✅ All error messages follow consistent format and style

**Key Features:**
- Error messages include specific suggestions for resolution
- I/O errors mapped to user-friendly, actionable messages
- Resource limit errors include specific limits and guidance
- Format errors provide clear guidance on supported formats
- Path sanitization ensures no sensitive information exposure

### Planned for Future Releases

**Full STEP B-Rep Support:**
- opencascade-rs integration research complete
- Documentation complete (Sprint 10)
- Full B-Rep support with curved surfaces (planned for v1.1.0)

**Future Enhancements:**
- Full STEP B-Rep support (opencascade-rs integration planned for v1.1.0)

---

## [0.3.0] - 2025-12-30

### Added

#### Parallel Batch Processing
- **Concurrent file conversion** - Multiple files processed simultaneously using thread pool
- **Thread pool implementation** - Uses `rayon` library with work-stealing scheduler
- **Configurable concurrency** - Settings → Conversion → Max Concurrent Conversions (1-16 range)
- **Default concurrency** - Automatically set to number of CPU cores (capped at 8 for memory safety)
- **Performance improvements** - Up to 4x speedup on 4-core systems compared to sequential processing
- **Thread-safe queue management** - All queue operations are thread-safe using `Arc<Mutex<BatchQueue>>`
- **Progress tracking** - Real-time progress updates for each parallel operation
- **Error isolation** - Individual item failures don't stop parallel processing
- **Resource management** - Configurable limits prevent memory exhaustion
- **Automatic load balancing** - Work-stealing scheduler distributes work evenly across threads

**Performance Examples:**
- 10 files (2 seconds each): Sequential = 20 seconds, Parallel (4 cores) = ~5 seconds
- 100 files (1 second each): Sequential = 100 seconds, Parallel (4 cores) = ~25 seconds

#### Settings Auto-Save
- **Automatic saving** - Settings automatically save 500ms after changes are made
- **Visual status indicator** - Shows auto-save state (Idle, Pending, Saving, Saved, Error)
- **Debouncing** - Prevents excessive file writes during rapid changes
- **Error handling** - User-friendly error messages if save fails
- **Manual save option** - Manual save button still available for immediate saving

#### Queue Item Editing
- **Edit queue items** - Edit button for each pending queue item
- **Edit output format** - Change output format from dropdown menu
- **Edit output path** - Change output file location with file browser
- **Edit conversion options** - Adjust quality (for lossy formats) and mesh options
- **Validation** - Ensures edited values are valid before saving
- **Restrictions** - Cannot edit processing or completed items (pending items only)
- **Benefits** - Fix mistakes without removing and re-adding items

#### 3D Mesh Viewer
- **Interactive 3D preview** - View 3D meshes in the preview panel before conversion
- **wgpu-based rendering** - Hardware-accelerated rendering using WebGPU
- **Camera controls** - Orbit (mouse drag), pan (Shift+drag), zoom (mouse wheel)
- **Rendering modes** - Switch between solid (with lighting) and wireframe views
- **Automatic mesh loading** - Meshes load automatically when selected in preview panel
- **Camera reset** - Reset button to return to default viewing angle
- **Performance optimized** - Smooth rendering for meshes up to 100,000 vertices
- **Error handling** - Graceful fallback when wgpu unavailable or rendering fails
- **Feature flag** - Enabled via `viewer-3d` feature flag (optional, not enabled by default)

### Changed
- Batch processing now uses parallel processing by default (configurable)
- Settings automatically save after changes (no manual save required)
- Queue items can be edited before processing (previously required removal and re-addition)

### Technical Details
- Parallel processing uses `rayon` library for efficient thread pool management
- Thread-safe state sharing using `Arc<Mutex<>>` for queue operations
- Mutex poisoning handling for graceful error recovery
- Progress tracking works correctly with parallel operations
- Resource limits apply per-file (not per-batch)

### Performance
- **4-core system:** Up to 4x faster batch processing
- **8-core system:** Up to 8x faster batch processing (with appropriate concurrency setting)
- **Memory usage:** Each concurrent conversion loads a file into memory (~3x file size for images, ~2x for meshes)

### Known Limitations
- Pause/resume/cancel UI controls for batch processing (backend ready, UI complete in Sprint 10_A)
- Full STEP B-Rep support (opencascade-rs) planned for future release (documentation complete)

### Security
- All security checks pass (reviewed by Security Specialist)
- Thread-safe operations prevent race conditions
- Resource limits enforced per-file to prevent DoS attacks
- Error isolation prevents cascading failures

### Notes
- ✅ All v0.3.0 core features fully implemented and tested
- ✅ 3D viewer integration complete (Sprint 10_A)
- ✅ 3D viewer testing and validation complete (Sprint 10_A)
- ✅ Code reviewed and approved by Senior Engineer
- ✅ Architecture reviewed and approved by System Architect
- ✅ Security reviewed and approved by Security Specialist
- ✅ Integration testing completed
- ✅ Performance benchmarks documented
- ✅ **RELEASED** - December 30, 2025

---

## [0.2.2] - 2025-12-30

### Added

#### GUI Enhancements (converter-gui)
- 📦 **Batch Processing** - Convert multiple files at once
  - Batch queue UI component with file list
  - Multi-file selection (drag-and-drop or file dialog)
  - Sequential queue processing (one file at a time)
  - Real-time progress tracking per item
  - Queue statistics (total, completed, failed)
  - Error resilience (failed conversions don't stop queue)
  - Queue management (add, remove, clear items)
- 🖼️ **Preview Functionality** - Preview images and meshes before conversion
  - Image preview panel with thumbnail generation
  - Mesh preview with metadata display (simplified for v0.2.2)
  - Preview updates on format change
  - Lazy loading and caching for performance
- ⚙️ **Settings Persistence** - Save user preferences across sessions
  - Settings panel UI with categories (General, Conversion, History)
  - Platform-specific settings file storage (TOML format)
  - Default output directory configuration
  - Quality defaults and resource limits
  - Advanced options visibility preference
  - Settings validation and corruption handling
- 📜 **Conversion History** - Track recent conversions
  - History tracking (source, output, format, timestamp)
  - History UI panel with recent conversions list
  - Status indicators (✓ for success, ✗ for failure) with tooltips
  - "Open Output" button - Opens converted files in system default application
  - Error handling for file opening (file not found, open failures)
  - History size limits (configurable, default: 50 entries)
  - Clear history functionality
  - Remove individual entries

#### Core Libraries
- `converter-gui/src/batch_queue.rs`: Batch queue data structures and management
- `converter-gui/src/settings.rs`: Settings persistence and configuration
- `converter-gui/src/history.rs`: Conversion history tracking
- `converter-gui/src/ui/batch_queue.rs`: Batch queue UI component
- `converter-gui/src/ui/preview.rs`: Preview panel UI component
- `converter-gui/src/ui/settings_panel.rs`: Settings UI component
- `converter-gui/src/ui/history_panel.rs`: History UI component

#### Documentation
- `docs/BATCH_PROCESSING_GUIDE.md`: Comprehensive batch processing user guide
- `docs/SETTINGS_GUIDE.md`: Complete settings and configuration guide
- Updated `docs/GUI_USAGE_GUIDE.md` with v0.2.2 features

### Changed
- Settings are now persisted across application sessions
- Batch processing enables efficient multi-file conversion
- Preview panel provides visual feedback before conversion
- Conversion history tracks user operations

### Technical Details
- Settings stored in TOML format in platform-specific directories
  - Windows: `%APPDATA%\SimpleImageConverter\config.toml`
  - macOS: `~/Library/Application Support/SimpleImageConverter/config.toml`
  - Linux: `~/.config/simpleimageconverter/config.toml`
- Batch queue uses sequential processing (parallel processing planned for v0.2.3)
- Preview uses thumbnail generation for large images
- History stored in settings file with size limits

### Known Limitations
- ~~Batch processing is sequential only~~ ✅ **COMPLETE in v0.3.0** - Parallel processing now available
- ~~Mesh preview shows metadata only~~ ✅ **COMPLETE in v0.3.0** - Full 3D viewer now available
- ~~Settings require manual save~~ ✅ **COMPLETE in v0.3.0** - Auto-save on change now available
- ~~Queue items cannot be edited after adding~~ ✅ **COMPLETE in v0.3.0** - Queue item editing now available

### Security
- Settings file path validation
- Batch queue path validation
- Preview file size limits
- History path sanitization
- All security checks pass (reviewed by Security Specialist)

### Notes
- ✅ All v0.2.2 features fully implemented and tested
- ✅ Code reviewed and approved by Senior Engineer
- ✅ Architecture reviewed and approved by System Architect (Grade: A)
- ✅ Security reviewed and approved by Security Specialist
- ✅ Documentation complete (Batch Processing Guide, Settings Guide)
- ✅ UI implementation complete (all components functional)
- ✅ Settings persistence working across sessions
- ✅ Batch processing tested and verified
- ✅ Preview functionality operational
- ✅ Conversion history tracking active
- ✅ **RELEASED** - December 30, 2025

---

## [0.2.1] - 2025-12-30

### Added

#### GUI Application (converter-gui)
- 🎨 Graphical User Interface using egui framework
- 📁 Drag-and-drop file support for easy file selection
- 🖼️ Visual format selection with radio buttons
- ⚙️ Quality settings slider for lossy image formats (JPEG, WebP)
- 📊 Status bar and progress indicators
- ✅ User-friendly error messages (no technical jargon)
- 🔄 Thread-safe conversion processing (responsive UI during conversion)
- 📝 Messages area for warnings, errors, and success notifications
- 🎯 Direct library integration (no subprocess calls to CLI binaries)

#### GUI Features
- File type detection (Image vs Mesh) with automatic format filtering
- Output filename auto-generation from source file and selected format
- Output location browser with path validation
- Advanced options panel (collapsible) for resource limits
- Quality slider (1-100) for lossy image formats
- Clear button to reset application state
- Comprehensive security validations (path validation, format detection, resource limits)

#### Core Libraries
- `converter-gui`: New GUI application crate
  - `app.rs`: Application state management
  - `conversion.rs`: Image conversion integration
  - `error_messages.rs`: User-friendly error message mapping
  - `format_helpers.rs`: Format detection and filtering utilities
  - UI components (drop zone, format selector, options panel, messages, status bar)

### Changed
- GUI now available as `converter-gui.exe` binary (Windows) or `converter-gui` (macOS/Linux)
- Error messages are now user-friendly (no technical jargon, no path leaks)
- Direct library integration replaces subprocess calls (better security, performance)

### Technical Details
- Direct library integration with `img-core` and `mesh-core` (architecture compliant)
- Thread-safe conversion processing using `Arc<Mutex<ConversionState>>`
- Two-stage format detection (extension + magic bytes) for security
- Comprehensive security validations (path validation, resource limits, error sanitization)
- Cross-platform ready (Windows tested, macOS/Linux ready)

### Known Limitations
- Batch processing not yet available (planned for v0.2.2)
- Preview functionality not yet available (planned for v0.2.2)
- Settings persistence not yet available (planned for v0.2.2)
- Conversion history not yet available (planned for v0.2.2)

### Security
- All security checks pass (reviewed by Security Specialist)
- Path validation prevents path traversal attacks
- Error message sanitization prevents information leakage
- Resource limits enforced for DoS prevention
- Two-stage format detection prevents format spoofing

### Notes
- All v0.2.1 GUI features fully implemented and tested
- Code reviewed and approved by Senior Engineer
- Architecture reviewed and approved by System Architect
- Security reviewed and approved by Security Specialist
- GUI provides intuitive interface for non-technical users

---

## [0.2.0] - 2025-12-29

### Added

#### Mesh Converter (mesh-convert)
- STEP format support (read-only, feature-gated)
  - FACETED_BREP entity extraction (pre-tessellated geometry)
  - Direct mesh construction from AP203 entities
  - Support for STEP files exported with tessellation enabled
  - Comprehensive error handling with user-friendly messages
  - Resource limits and security validation
- STEP integration tests (8 tests, all passing)
  - File reading tests
  - Conversion tests (STEP → STL, STEP → OBJ)
  - Error handling tests
  - Converter integration tests

#### Core Libraries
- `mesh-core`: STEP format handler
  - `StepFormat` struct with resource limits
  - FACETED_BREP entity traversal and extraction
  - Vertex deduplication with integer-based hashing
  - Face triangulation for polygons
  - Normal calculation for extracted meshes
  - Comprehensive validation and error handling

#### Documentation
- Comprehensive STEP format documentation
  - `docs/STEP_FORMAT_REFERENCE.md` - Technical reference
  - `docs/CAD_EXPORT_GUIDE.md` - User guide for CAD software
  - `docs/RUSTSTEP_GUIDANCE.md` - Developer guide for ruststep API
  - `docs/FORMATS.md` - Updated with STEP limitations
- Test file collection framework
  - Verification scripts
  - Collection guidelines
  - Test file documentation

### Changed
- STEP format support moved from "in progress" to "partial support"
- Updated error messages to be more user-friendly and actionable
- Enhanced resource limit validation for STEP files
- Improved security logging for STEP operations

### Improved
- Error messages now include solutions and documentation references
- Better handling of unsupported STEP file types
- Clearer guidance for users on STEP file requirements

### Limitations
- **FACETED_BREP only:** v0.2.0 supports only pre-tessellated STEP files
  - Files must be exported with tessellation enabled
  - No support for curved surfaces (NURBS, cylinders, spheres)
  - Full B-Rep support planned for v0.3.0
- **Feature-gated:** STEP support requires `--features step` flag
- **Read-only:** STEP writing not supported (requires complex CAD modeling)

### Security
- All security checks pass (reviewed by Security Specialist)
- Zero unsafe code blocks
- Comprehensive input validation (file size, UTF-8, mesh resources)
- Resource limits enforced before parsing and after extraction
- Security logging for all limit violations
- Secure by Design: 10/10 principles met
- Security grade: A (Strong - Production Ready)

### Architecture
- Approved hybrid phased approach (FACETED_BREP → opencascade-rs)
- Pure Rust implementation (no C++ dependencies for v0.2.0)
- Feature-gated implementation
- Direct mesh construction (no intermediate Shell conversion)

### Notes
- All v0.2.0 features fully implemented and tested
- Code reviewed and approved by Senior Engineer
- Architecture reviewed and approved by System Architect
- Security reviewed and approved by Security Specialist
- 8 STEP integration tests passing
- All tests passing (370+ total)

---

## [0.1.1] - 2025-12-27

### Added

#### Mesh Converter (mesh-convert)
- Coordinate system transform functionality (`--transform`)
  - Transform between Y-up and Z-up coordinate systems
  - Support for explicit transforms (`z-up:y-up`) or auto-detect (`y-up`)
  - Automatic normal vector transformation
- Normal recalculation (`--recalculate-normals`)
  - Area-weighted face normal calculation
  - Smooth vertex normal computation
  - Automatic handling of degenerate faces
- Mesh validation (`--validate`)
  - Vertex and face index validation
  - Degenerate face detection
  - Duplicate vertex detection
  - Normal consistency checks
- CLI integration tests for new features

#### Core Libraries
- `mesh-core`: New mesh manipulation utilities
  - `transform_coordinates()` - Coordinate system transformation
  - `recalculate_normals()` - Vertex normal recalculation from geometry
  - `validate_mesh()` - Comprehensive mesh validation
  - `ConversionOptions` struct for advanced conversion settings

### Changed
- `MeshConverter` now supports `ConversionOptions` for advanced operations
- Improved code quality with refactored transform logic
- Enhanced test coverage (14+ new tests)

### Improved
- Transform logic refactored to eliminate code duplication
- Magic numbers replaced with named constants
- Better error messages and documentation

### Fixed
- Improved handling of degenerate faces in normal recalculation
- Enhanced validation test coverage

### Security
- All security checks pass (reviewed by Security Specialist)
- Zero unsafe code blocks
- Comprehensive bounds checking maintained
- Resource limits properly enforced

### Notes
- All v0.1.1 features fully implemented and tested
- Code reviewed and approved by Senior Engineer
- Security reviewed and approved by Security Specialist
- 350+ tests passing

---

## [0.1.0] - 2025-12-27

### Added

#### Image Converter (img-convert)
- PNG format support (read/write) with transparency handling
- JPEG format support (read/write) with quality control (1-100)
- BMP format support (read/write)
- GIF format support (read/write, first frame)
- TIFF format support (read/write) with multi-page handling
- WebP format support (read/write) with lossy/lossless modes
- SVG format support (read-only, rasterization to bitmap)
- Two-stage format detection (extension + magic bytes)
- Resource limits and security validation
- Output file verification
- Comprehensive error handling

#### Mesh Converter (mesh-convert)
- STL format support (binary/ASCII, read/write)
- OBJ format support (read/write) with material (.mtl) handling
- PLY format support (read/write)
- OFF format support (read/write, custom parser)
- glTF/GLB format support (read/write) with material handling
- DXF format support (read/write, 3D entities)
- STEP format support (read-only, feature-gated, tessellation in progress)
- Format detection and validation
- Resource limits and security validation
- Output file verification

#### Core Libraries
- `img-core`: Image conversion library with trait-based format system
- `mesh-core`: Mesh conversion library with trait-based format system
- `common`: Shared utilities (error types, I/O helpers, resource limits, security logging)

#### Security
- Zero unsafe code blocks
- Comprehensive input validation
- Resource limits (file size, dimensions, vertices, faces)
- Two-stage format detection to prevent format spoofing
- Security event logging
- Integer overflow protection

#### Testing
- 365+ tests total covering all format implementations
- Unit tests for all format readers/writers
- Integration tests for format conversions
- Security tests for format spoofing and malformed input
- Edge case handling (empty files, invalid data, oversized files)

### Changed
- Project status updated from "In Development" to "Active Development"
- All Sprints 1-5 marked as complete

### Fixed
- All previously identified critical security issues resolved
- Format registry now returns `Result` instead of panicking
- Comprehensive input validation implemented
- Error handling standardized across all modules

### Security
- All critical security vulnerabilities addressed
- Secure by Design compliance: 10/10 principles met
- No unsafe code in production paths
- Comprehensive validation at all entry points

### Notes
- This is the first production-ready release
- All core formats are implemented and tested
- STEP format is feature-gated (`--features step`) and partial
- mesh-convert transform, recalculate-normals, and validate features are planned for v0.1.1
- CLI integration tests are planned for v0.1.1

---

## Version History

### Planned Releases

- **v0.1.0** (Sprint 3) - MVP: Core converters (PNG, JPG, BMP, GIF, STL, OBJ, PLY)
- **v0.2.0** (Sprint 6) - Extended formats (TIFF, WebP, SVG, glTF, DXF)
- **v0.3.0** (Sprint 8) - STEP/CAD support
- **v1.0.0** (Sprint 12) - GUI release, public repository

---

## Release Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security updates
```

---

**Note:** This changelog will be updated as development progresses through sprints.

[Unreleased]: https://github.com/yourusername/SimpleImageConverter/compare/v0.1.0...HEAD
[0.1.0-dev]: https://github.com/yourusername/SimpleImageConverter/releases/tag/v0.1.0-dev
