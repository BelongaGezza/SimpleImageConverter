# GUI Testing Checklist - Phase 2 Complete

## Quick Start

```bash
# Run the GUI application
cargo run --bin converter-gui

# Or run release build (optimized)
cargo run --release --bin converter-gui
```

## What to Test

### ✅ Phase 1: Basic Window Setup

- [ ] Window opens with title "Simple Image Converter"
- [ ] Window is resizable (minimum 800x600 enforced)
- [ ] Window can be minimized/maximized/closed
- [ ] Menu bar displays (File, Edit, Help)
- [ ] Menu items are clickable

### ✅ Phase 2.1: File Drop Zone

**Visual Appearance:**
- [ ] Drop zone is visible (large box, ~200px height)
- [ ] Shows "📁 Drag & Drop File Here" when empty
- [ ] "Browse Files..." button is visible and clickable
- [ ] Drop zone spans full width

**Click-to-Browse:**
- [ ] Click "Browse Files..." opens file dialog
- [ ] File dialog shows filters:
  - Image Files (png, jpg, jpeg, bmp, gif, tiff, tif, webp, svg)
  - Mesh Files (stl, obj, ply, off, gltf, glb, dxf, step, stp)
  - All Files
- [ ] Selecting a file updates drop zone:
  - Background turns light green
  - Border turns green (2px)
  - Shows "📁 File Selected" + filename

**Drag-and-Drop:**
- [ ] Drag file over drop zone (border turns blue)
- [ ] Drop file onto drop zone
- [ ] File is accepted and displayed correctly
- [ ] File type is detected (image vs mesh)

**File Type Detection:**
- [ ] Test with image files: PNG, JPEG, BMP, GIF, TIFF, WebP
- [ ] Test with mesh files: STL, OBJ, PLY, OFF, glTF, DXF
- [ ] Verify correct file type detected
- [ ] Verify unsupported files show error message

### ✅ Phase 2.2: Format Selection

**Image Formats:**
- [ ] Radio buttons appear when image file selected
- [ ] Shows: BMP, GIF, JPEG, PNG, TIFF, WebP
- [ ] SVG is NOT shown (read-only)
- [ ] Default format selected (first alphabetically: BMP)
- [ ] Changing format updates output filename extension

**Mesh Formats:**
- [ ] Radio buttons appear when mesh file selected
- [ ] Shows: DXF, glTF, OBJ, OFF, PLY, STL
- [ ] STEP is NOT shown (read-only)
- [ ] Default format selected (first alphabetically: DXF)
- [ ] Changing format updates output filename extension

**Format Change:**
- [ ] Select different format
- [ ] Output filename extension updates automatically
- [ ] Filename stem (without extension) is preserved

### ✅ Phase 2.3: Options Panel

**Output Filename:**
- [ ] Filename field is editable
- [ ] Auto-generates from source filename + selected format
- [ ] Updates when format changes
- [ ] Can be manually edited

**Output Location:**
- [ ] Shows current directory (truncated if > 50 chars)
- [ ] "Browse..." button opens folder dialog
- [ ] Selecting folder updates output location
- [ ] Path validation works (invalid paths rejected)

**Quality Slider:**
- [ ] Appears only for JPEG and WebP formats
- [ ] Hidden for PNG, BMP, GIF, TIFF
- [ ] Range: 1-100
- [ ] Default: 90
- [ ] Value updates when slider moved
- [ ] Label shows current value: "Quality (1-100): 90"

**Advanced Options:**
- [ ] "Advanced Options" is collapsible
- [ ] When expanded, shows:
  - Max File Size (MB): 1-1024, default 100
  - Max Dimension (pixels): Images only, 1000-65535, default 65535
  - Max Vertices: Meshes only, 1000-10,000,000, default 10,000,000
  - Max Faces: Meshes only, 1000-10,000,000, default 10,000,000
- [ ] Warning appears if file size > 100 MB

### ✅ Phase 2.4: Messages & Status Bar

**Messages Area:**
- [ ] Scrollable area (max height 150px)
- [ ] Shows "No messages" when empty
- [ ] Messages display with icons:
  - ℹ Info (blue)
  - ⚠ Warning (yellow)
  - ✗ Error (red)
  - ✓ Success (green)
- [ ] Messages are user-friendly (no technical jargon)
- [ ] Paths are sanitized (no full paths shown)

**Status Bar:**
- [ ] Appears at bottom of window
- [ ] Shows "Ready" when no operation
- [ ] Updates when file selected
- [ ] Shows conversion status (when implemented)
- [ ] Progress spinner for long operations (>30 seconds)

**Action Buttons:**
- [ ] "Convert" button appears
- [ ] "Convert" disabled when no file selected
- [ ] "Convert" disabled when format not selected
- [ ] "Clear" button resets all fields
- [ ] Buttons are right-aligned

### Integration Testing

**Complete Workflow:**
1. [ ] Select image file → Format selector appears → Select format → Options update
2. [ ] Select mesh file → Format selector appears → Select format → Options update
3. [ ] Change format → Filename extension updates
4. [ ] Edit filename → Changes persist
5. [ ] Change output location → Updates correctly
6. [ ] Adjust quality (JPEG/WebP) → Value updates
7. [ ] Expand advanced options → Settings visible
8. [ ] Click "Clear" → All fields reset to default

**Error Handling:**
- [ ] Invalid file path → Error message displayed
- [ ] Unsupported file type → Error message displayed
- [ ] Invalid output location → Error message displayed
- [ ] Error messages are user-friendly (no technical jargon)

## Known Limitations

- **Conversion functionality** not yet implemented (Phase 3)
- "Convert" button shows placeholder message
- No actual file conversion happens yet

## Expected Visual Layout

```
┌─────────────────────────────────────────────┐
│ Simple Image Converter          [─][□][×]   │
├─────────────────────────────────────────────┤
│ File  Edit  Help                             │
├─────────────────────────────────────────────┤
│                                             │
│  Simple Image Converter                     │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │  📁 File Selected                     │ │
│  │  photo.png                            │ │
│  └───────────────────────────────────────┘ │
│                                             │
│  Source File: photo.png                     │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │ Output Format:                        │ │
│  │  ○ BMP  ○ GIF  ○ JPEG  ● PNG          │ │
│  │  ○ TIFF  ○ WebP                       │ │
│  └───────────────────────────────────────┘ │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │ Options:                              │ │
│  │  Output Filename: [photo.png]         │ │
│  │  Output Location: [C:\Users\...]     │ │
│  │  [Browse...]                          │ │
│  │  Quality (1-100): 90 ━━━━━━━━━━━━━━ │ │
│  │  [Advanced Options ▼]                 │ │
│  └───────────────────────────────────────┘ │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │ Messages:                             │ │
│  │  ℹ Image file detected: Png           │ │
│  └───────────────────────────────────────┘ │
│                                             │
│                    [Convert]  [Clear]       │
├─────────────────────────────────────────────┤
│ Status: Ready                               │
└─────────────────────────────────────────────┘
```

## Troubleshooting

**Window doesn't open:**
- Check graphics drivers
- Try debug mode: `cargo run --bin converter-gui`

**File dialog doesn't open:**
- Check `rfd` dependency
- Verify Windows file permissions

**Drag-and-drop doesn't work:**
- Make sure you're dragging files (not folders)
- Try clicking "Browse Files..." as alternative

**Format selector doesn't appear:**
- Make sure a file is selected first
- Verify file type is detected correctly

**Quality slider doesn't appear:**
- Only shows for JPEG and WebP formats
- Select JPEG or WebP as output format

## Next Steps

After testing, we'll implement:
- **Phase 3**: Conversion Thread Integration
  - Actual file conversion functionality
  - Background processing
  - Progress tracking
  - Error handling

