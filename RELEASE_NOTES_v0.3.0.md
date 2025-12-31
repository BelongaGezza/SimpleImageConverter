# Simple Image Converter v0.3.0 - Performance & 3D Viewer Release

**Release Date:** December 30, 2025  
**Type:** Feature Release  
**Target:** v0.3.0  
**Status:** ✅ **RELEASED**

---

## What's New

### ⚡ Parallel Batch Processing

Experience dramatically faster batch conversions with our new parallel processing engine! Convert multiple files simultaneously using all available CPU cores.

**Key Features:**
- **Concurrent conversion** - Multiple files processed at the same time
- **Automatic optimization** - Defaults to number of CPU cores (capped at 8 for safety)
- **Configurable concurrency** - Adjust from 1-16 concurrent conversions in Settings
- **Up to 4x faster** - On a 4-core system, batch processing is up to 4x faster than sequential
- **Thread-safe** - All operations are thread-safe with proper synchronization
- **Error isolation** - Individual file failures don't stop the entire batch
- **Real-time progress** - See progress for each parallel operation

**Performance Examples:**
- **10 files (2 seconds each):** Sequential = 20 seconds, Parallel (4 cores) = ~5 seconds
- **100 files (1 second each):** Sequential = 100 seconds, Parallel (4 cores) = ~25 seconds

**How to Use:**
1. Add multiple files to the batch queue
2. Processing automatically uses parallel mode (configurable in Settings → Conversion)
3. Watch as multiple files convert simultaneously!

### 🎨 Interactive 3D Mesh Viewer

Preview your 3D meshes in full 3D before conversion! Rotate, zoom, and inspect your models with hardware-accelerated rendering.

**Key Features:**
- **Hardware-accelerated rendering** - Uses WebGPU for smooth performance
- **Camera controls:**
  - **Orbit** - Click and drag to rotate around the mesh
  - **Pan** - Shift + drag to move the view
  - **Zoom** - Mouse wheel to zoom in/out
- **Rendering modes:**
  - **Solid** - Full lighting and shading
  - **Wireframe** - See the mesh structure
- **Automatic loading** - Meshes load automatically when selected
- **Camera reset** - One-click return to default view
- **Performance optimized** - Smooth rendering for meshes up to 100,000 vertices
- **Graceful fallback** - Works even if WebGPU is unavailable

**How to Use:**
1. Select a mesh file (STL, OBJ, PLY, glTF, etc.)
2. The 3D viewer automatically loads in the preview panel
3. Use mouse controls to inspect your mesh
4. Switch between solid and wireframe modes

**Note:** 3D viewer requires the `viewer-3d` feature flag. Build with:
```bash
cargo build --release --features viewer-3d
```

### ⚙️ Settings Auto-Save

No more manual saving! Settings now automatically save 500ms after you make changes.

**Key Features:**
- **Automatic saving** - Changes save automatically after 500ms
- **Visual status indicator** - See save status (Idle, Pending, Saving, Saved, Error)
- **Debouncing** - Prevents excessive file writes during rapid changes
- **Error handling** - User-friendly messages if save fails
- **Manual save** - Manual save button still available for immediate saving

**Status Indicators:**
- **Idle** - No changes pending
- **Pending** - Changes made, waiting to save
- **Saving** - Currently saving to disk
- **Saved** - Successfully saved
- **Error** - Save failed (with error message)

### ✏️ Queue Item Editing

Made a mistake? No problem! Edit queue items before processing without removing and re-adding them.

**Key Features:**
- **Edit button** - Each pending queue item has an edit button
- **Edit output format** - Change format from dropdown menu
- **Edit output path** - Change file location with file browser
- **Edit conversion options** - Adjust quality (for lossy formats) and mesh options
- **Validation** - Ensures edited values are valid before saving
- **Restrictions** - Can only edit pending items (not processing or completed)

**How to Use:**
1. Add files to the batch queue
2. Click the "Edit" button on any pending item
3. Modify format, path, or options
4. Save changes - item updates in place

---

## Performance Improvements

### Batch Processing Speed
- **4-core system:** Up to 4x faster batch processing
- **8-core system:** Up to 8x faster batch processing (with appropriate concurrency setting)
- **Memory usage:** Each concurrent conversion loads a file into memory (~3x file size for images, ~2x for meshes)

### Technical Details
- Uses `rayon` library for efficient thread pool management
- Work-stealing scheduler distributes work evenly
- Thread-safe state sharing using `Arc<Mutex<>>`
- Mutex poisoning handling for graceful error recovery
- Resource limits apply per-file (not per-batch)

---

## Changed

- **Batch processing** now uses parallel processing by default (configurable in Settings)
- **Settings** automatically save after changes (no manual save required)
- **Queue items** can be edited before processing (previously required removal and re-addition)

---

## Technical Details

### Parallel Processing
- Thread pool implementation using `rayon` library
- Configurable concurrency (1-16 range, default: CPU cores capped at 8)
- Thread-safe queue management with `Arc<Mutex<BatchQueue>>`
- Progress tracking works correctly with parallel operations
- Error isolation prevents cascading failures

### 3D Viewer
- wgpu-based rendering (WebGPU)
- Camera controls: orbit, pan, zoom
- Rendering modes: solid (with lighting) and wireframe
- Automatic mesh loading when selected
- Performance optimized for meshes up to 100,000 vertices
- Graceful fallback when wgpu unavailable

### Settings Auto-Save
- Automatic saving 500ms after changes
- Visual status indicator
- Debouncing prevents excessive file writes
- Error handling with user-friendly messages

### Queue Item Editing
- Edit output format, path, and conversion options
- Validation ensures edited values are valid
- Restrictions: pending items only (not processing or completed)

---

## Security

- ✅ All security checks pass (reviewed by Security Specialist)
- ✅ Thread-safe operations prevent race conditions
- ✅ Resource limits enforced per-file to prevent DoS attacks
- ✅ Error isolation prevents cascading failures
- ✅ No unsafe code blocks
- ✅ Comprehensive input validation

---

## Known Limitations

- **3D Viewer:** Requires `viewer-3d` feature flag (optional, not enabled by default)
- **Full STEP B-Rep Support:** Planned for future release (opencascade-rs integration)
- **Pause/Resume/Cancel:** Backend ready, UI controls implemented in Sprint 10_A

---

## Installation

### Option 1: Download Pre-built Binaries (Coming Soon)
- **Windows:** Download from GitHub Releases
- **macOS:** Download from GitHub Releases
- **Linux:** Download from GitHub Releases

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/BelongaGezza/SimpleImageConverter.git
cd SimpleImageConverter

# Checkout v0.3.0
git checkout v0.3.0

# Build all tools
cargo build --release

# Build with 3D viewer support (optional)
cargo build --release --features viewer-3d

# Binaries will be in target/release/
```

---

## Upgrade Notes

### From v0.2.2

**Settings:**
- Settings now auto-save (no manual save required)
- Settings file format unchanged (backward compatible)

**Batch Processing:**
- Batch processing now uses parallel mode by default
- Adjust concurrency in Settings → Conversion → Max Concurrent Conversions
- Performance significantly improved on multi-core systems

**New Features:**
- 3D mesh viewer (requires `viewer-3d` feature flag)
- Queue item editing
- Settings auto-save

**No Breaking Changes:**
- All existing functionality preserved
- Settings file format unchanged
- CLI tools unchanged

---

## Testing

- ✅ All unit tests passing (400+ tests)
- ✅ All integration tests passing
- ✅ 3D viewer integration tests (20 tests) - all passing
- ✅ Parallel processing integration tests - all passing
- ✅ Security review completed and approved
- ✅ Architecture review completed and approved

---

## Credits

**Development Team:**
- System Architect: Alex Chen
- Senior Engineer: Jordan Rivera
- Junior Engineer (2D): Sam Kim
- Junior Engineer (3D): Alex Rivera
- UI Designer: Jamie Chen
- Security Specialist: Casey Morgan
- Documentation Specialist: Morgan Lee
- Researcher: Dr. Taylor Kim

**Special Thanks:**
- All contributors and testers
- Rust community for excellent libraries
- egui/eframe team for the GUI framework
- wgpu team for WebGPU support

---

## Release Information

**Version:** 0.3.0  
**Release Date:** December 30, 2025  
**Git Tag:** `v0.3.0`  
**Commit:** [To be added after tagging]

**Previous Release:** v0.2.2 (December 30, 2025)  
**Next Release:** v0.3.1 (planned)

---

## Support

- **Issues:** [GitHub Issues](https://github.com/BelongaGezza/SimpleImageConverter/issues)
- **Documentation:** See `README.md` and `docs/` directory
- **Questions:** Open a GitHub Discussion

---

**Enjoy the new features! Happy converting! 🎉**

