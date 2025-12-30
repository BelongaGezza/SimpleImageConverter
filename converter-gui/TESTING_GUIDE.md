# GUI Testing Guide - Sprint 7 Phase 1 & 2.1

## Quick Start

### Run the GUI Application

```bash
# Debug build (faster compilation)
cargo run --bin converter-gui

# Release build (optimized, slower compilation)
cargo run --release --bin converter-gui
```

## What to Test

### ✅ Phase 1: Basic Window Setup

1. **Window Launch**
   - [ ] Window opens with title "Simple Image Converter"
   - [ ] Window is resizable (minimum size 800x600 enforced)
   - [ ] Window can be minimized/maximized/closed

2. **Menu Bar**
   - [ ] File menu appears and opens
   - [ ] Edit menu appears and opens
   - [ ] Help menu appears and opens
   - [ ] Menu items are clickable (stubs for now)

### ✅ Phase 2.1: File Drop Zone

1. **Visual Appearance**
   - [ ] Drop zone is visible (large gray box, ~200px height)
   - [ ] Drop zone shows "📁 Drag & Drop File Here" text
   - [ ] "Browse Files..." button is visible and clickable
   - [ ] Drop zone spans full width of window

2. **Click-to-Browse**
   - [ ] Click "Browse Files..." button opens file dialog
   - [ ] File dialog shows image and mesh file filters
   - [ ] Selecting a file updates the drop zone display
   - [ ] Selected file name appears in drop zone
   - [ ] Drop zone border changes to green when file selected

3. **Drag-and-Drop**
   - [ ] Drag a file over the drop zone (border turns blue)
   - [ ] Drop a file onto the drop zone
   - [ ] File is accepted and displayed
   - [ ] File type is detected (image vs mesh)

4. **File Type Detection**
   - [ ] Test with image files: PNG, JPEG, BMP, GIF, TIFF, WebP
   - [ ] Test with mesh files: STL, OBJ, PLY, OFF, glTF, DXF
   - [ ] Verify correct file type is detected
   - [ ] Verify unsupported files show error message

5. **Security Validation**
   - [ ] Invalid file paths are rejected
   - [ ] Error messages are user-friendly (no technical jargon)
   - [ ] No path traversal attacks possible

6. **State Management**
   - [ ] Selected file persists when window is resized
   - [ ] File selection clears when "Clear" is clicked (File menu)
   - [ ] Output filename auto-generates from source filename

## Expected Behavior

### File Selection Flow

1. User drags file or clicks "Browse Files..."
2. File dialog opens (if clicked)
3. User selects a file
4. Drop zone updates:
   - Background: Light green (240, 255, 240)
   - Border: Green (0, 200, 0), 2px width
   - Text: "📁 File Selected" + filename
5. File type is detected (Image or Mesh)
6. Input format is detected (PNG, JPEG, STL, OBJ, etc.)
7. Output filename auto-generates
8. Output directory set to source file's directory

### Visual States

- **Empty**: Light gray background (245, 245, 245), gray border (200, 200, 200), 1px
- **Drag Over**: Light blue background (240, 248, 255), blue border (0, 100, 255), 2px
- **File Selected**: Light green background (240, 255, 240), green border (0, 200, 0), 2px

## Known Limitations (To Be Implemented)

- Format selection UI (Task 2.2) - Not yet implemented
- Options panel (Task 2.3) - Not yet implemented
- Messages display (Task 2.4) - Not yet implemented
- Status bar (Task 2.4) - Not yet implemented
- Conversion functionality (Phase 3) - Not yet implemented

## Troubleshooting

### Window doesn't open
- Check if graphics drivers are up to date
- Try running in debug mode: `cargo run --bin converter-gui`

### File dialog doesn't open
- Check if `rfd` dependency is correctly installed
- Verify Windows file permissions

### Drag-and-drop doesn't work
- Make sure you're dragging files (not folders)
- Try clicking "Browse Files..." as alternative

### File type not detected
- Verify file has correct extension
- Check that file format is supported (see FORMATS.md)

## Next Steps After Testing

Once testing is complete, we'll continue with:
- Task 2.2: Format Selection UI Component
- Task 2.3: Options Panel Component
- Task 2.4: Messages & Status Bar Components

