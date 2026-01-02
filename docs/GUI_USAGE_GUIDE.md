# GUI Usage Guide
## Simple Image Converter - Graphical User Interface

**Version:** 0.3.0  
**Last Updated:** December 30, 2025 (Updated for v0.3.0 parallel processing)

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Conversion](#basic-conversion)
3. [Format Selection](#format-selection)
4. [Output Options](#output-options)
5. [Advanced Options](#advanced-options)
6. [Error Messages](#error-messages)
7. [Tips and Tricks](#tips-and-tricks)
8. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Launching the Application

**Windows:**
- Double-click `converter-gui.exe`
- Or run from command line: `converter-gui.exe`

**macOS/Linux:**
- Run from terminal: `./converter-gui`
- Or double-click the executable (if file associations are set)

### First Launch

When you first launch the GUI, you'll see:
- A large drop zone in the center
- Format selection area (empty until file is selected)
- Options panel (collapsed by default)
- Messages area (empty)
- Status bar at the bottom showing "Ready"
- Convert and Clear buttons

---

## Basic Conversion

### Step 1: Select a File

You have two options:

**Option A: Drag and Drop**
1. Open your file manager (Windows Explorer, Finder, etc.)
2. Drag a file into the drop zone
3. Release the mouse button

**Option B: Browse for File**
1. Click the "Browse Files..." button in the drop zone
2. Select a file from the file dialog
3. Click "Open"

**What Happens:**
- The file is validated for security
- File type is detected (Image or Mesh)
- Available output formats are shown
- Output filename is auto-generated
- Output location is set to the source file's directory

### Step 2: Select Output Format

After selecting a file, you'll see radio buttons for available formats:

**For Image Files:**
- PNG
- JPEG
- BMP
- GIF
- TIFF
- WebP

**For Mesh Files:**
- STL
- OBJ
- PLY
- OFF
- glTF
- DXF

**Note:** SVG (images) and STEP (meshes) are read-only and cannot be selected as output formats.

### Step 3: Adjust Options (Optional)

- **Output Filename:** Edit the filename if desired
- **Output Location:** Click "Browse..." to change save location
- **Quality:** Adjust slider for JPEG/WebP (1-100, default: 90)

### Step 4: Convert

1. Click the "Convert" button
2. Status bar shows "Converting..."
3. For long operations (>30 seconds), a progress indicator appears
4. When complete, status bar shows "Conversion complete" with output path
5. Success message appears in the messages area

### Step 5: View Results

- **Status Bar:** Shows current status and output path (sanitized)
- **Messages Area:** Shows success message with file location
- **Output File:** Saved to the specified location

---

## Format Selection

### Automatic Format Filtering

The GUI automatically filters available formats based on your source file:

- **Image files** → Only image output formats shown
- **Mesh files** → Only mesh output formats shown

This prevents errors and makes selection easier.

### Format Descriptions

**Image Formats:**
- **PNG** - Lossless, supports transparency
- **JPEG** - Lossy, good for photos (quality adjustable)
- **BMP** - Uncompressed, large file size
- **GIF** - Supports animation (first frame only)
- **TIFF** - High quality, supports multiple pages
- **WebP** - Modern format, good compression (quality adjustable)

**Mesh Formats:**
- **STL** - Simple, widely supported (binary or ASCII)
- **OBJ** - Text-based, supports materials
- **PLY** - Stanford format, supports colors
- **OFF** - Object File Format, simple structure
- **glTF** - Modern web format, supports materials and animations
- **DXF** - AutoCAD format, supports 3D entities

### Read-Only Formats

Some formats can only be read (converted FROM), not written (converted TO):

- **SVG** (images) - Vector format, cannot be written as raster
- **STEP** (meshes) - CAD format, read-only (feature-gated)

These formats appear in the input list but not in output options.

---

## Output Options

### Output Filename

**Auto-Generation:**
- Default filename is generated from source file + selected format
- Example: `photo.png` → `photo.jpg` (if JPEG selected)

**Customization:**
- Click in the filename field
- Edit the name as desired
- Extension is automatically updated when format changes

**Validation:**
- Invalid characters are rejected: `< > : " | ? *`
- Path traversal attempts are blocked (`../`)
- Filename length is validated (Windows MAX_PATH: 260 chars)

### Output Location

**Default:**
- Output location defaults to the source file's directory

**Change Location:**
1. Click "Browse..." button next to output location
2. Select a directory in the file dialog
3. Click "OK"

**Validation:**
- System directories are blocked (security)
- Write permissions are checked before conversion
- Path is validated for security

### Quality Settings

**When Available:**
- Quality slider appears only for lossy formats: JPEG and WebP
- Hidden for lossless formats (PNG, BMP, GIF, TIFF)

**Usage:**
- **Higher values (90-100):** Better quality, larger file size
- **Lower values (1-50):** Smaller file size, lower quality
- **Default:** 90 (good balance)

**Recommendations:**
- **Photos:** 85-95
- **Web images:** 75-85
- **Thumbnails:** 60-75

---

## Advanced Options

### Accessing Advanced Options

1. Click "Show Advanced Options" button
2. Advanced options panel expands
3. Adjust settings as needed
4. Click again to collapse

### Resource Limits

**Max File Size:**
- Default: 100 MB
- Maximum: 1 GB (with warning)
- Prevents DoS attacks from oversized files

**Max Dimension (Images Only):**
- Default: 65535 pixels
- Maximum width or height
- Prevents memory exhaustion

**Max Vertices/Faces (Meshes Only):**
- Default: 10,000,000 each
- Prevents memory exhaustion
- Adjust for very large meshes

**Warning:**
- Increasing limits beyond defaults may cause:
  - Slower processing
  - Higher memory usage
  - Potential crashes on low-memory systems

---

## Error Messages

### Understanding Error Messages

All error messages are designed to be **user-friendly**, **actionable**, and **secure**. The error message system (implemented in v0.3.0) provides clear guidance without exposing sensitive system information.

**Key Features:**
- ✅ **User-friendly language** - No technical jargon
- ✅ **Actionable guidance** - Each message includes suggestions for resolution
- ✅ **Security-first** - No sensitive information (paths, usernames) exposed
- ✅ **Context-aware** - Messages adapt based on error type and context

### Common Error Messages

**File Format Errors:**

- **"File type not supported. Please use a supported image or mesh format."**
  - Solution: Select a supported format (PNG, JPEG, BMP, GIF, WebP for images; STL, OBJ, PLY, OFF, DXF, glTF for meshes)
  - See Format Selection section for complete list

- **"File extension doesn't match file content. The file may be corrupted or have the wrong extension."**
  - Solution: Verify the file is valid and has the correct extension
  - Try renaming the file with the correct extension

**File Access Errors:**

- **"File not found. Please check that the file exists and the path is correct."**
  - Solution: Verify the file exists at the specified location
  - Check that you have read permissions for the file

- **"Permission denied. Please check file permissions or try running as administrator."**
  - Solution: Check file permissions
  - On Windows: Right-click file → Properties → Security tab
  - On Linux/macOS: Check file permissions with `ls -l`

- **"File already exists at the output location. Please choose a different filename or location."**
  - Solution: Select a different output filename or location
  - Or delete/rename the existing file first

**Size and Resource Limit Errors:**

- **"File size too large. Maximum size is 100 MB. Please use a smaller file or compress it first."**
  - Solution: Compress the file or use a smaller version
  - For images: Resize or reduce quality before conversion

- **"Image dimensions too large. Maximum dimension is 65,535 pixels. Please use a smaller image or resize it before converting."**
  - Solution: Resize the image using an image editor before conversion
  - Or split the image into smaller sections

- **"Mesh has too many vertices. Maximum is 10,000,000. Please use a mesh with fewer vertices or simplify the model."**
  - Solution: Simplify the mesh using a 3D modeling tool
  - Or use mesh decimation to reduce vertex count

- **"Mesh has too many faces. Maximum is 10,000,000. Please use a mesh with fewer faces or simplify the model."**
  - Solution: Simplify the mesh topology
  - Use mesh optimization tools to reduce face count

**Conversion Errors:**

- **"File appears to be corrupted or invalid. Please verify the file is valid and try again."**
  - Solution: Verify the file opens in other applications
  - Try re-saving the file from the original application
  - Check if the file was partially downloaded or transferred

- **"Failed to write output file. Please check that you have write permissions and sufficient disk space."**
  - Solution: Check available disk space
  - Verify write permissions for the output directory
  - Try saving to a different location

- **"Conversion failed. Please check that the file is valid and try again. If the problem persists, the file format may not be fully supported."**
  - Solution: Verify file is not corrupted
  - Try converting to a different format
  - Check if the file uses unsupported features

**Validation Errors:**

- **"Invalid file path. Please check that the path is valid and doesn't contain invalid characters."**
  - Solution: Avoid special characters in file paths
  - Don't use system directories (e.g., `C:\Windows\`)
  - Use standard file naming conventions

- **"Mesh validation failed. The mesh may have invalid geometry. Try enabling mesh validation options or check if the mesh file is valid."**
  - Solution: Enable mesh validation in Advanced Options
  - Try recalculating normals
  - Verify mesh is valid in a 3D modeling application

**Quality Setting Errors:**

- **"Quality setting must be between 1 and 100. Please adjust the quality slider."**
  - Solution: Use the quality slider to set a value between 1 and 100
  - Higher values (85-100) = better quality, larger files
  - Lower values (1-50) = smaller files, lower quality

### Message Types

Messages are color-coded for easy identification:

- **Info (Blue):** General information and status updates
- **Warning (Yellow):** Non-critical issues (e.g., file will be overwritten, format limitations)
- **Error (Red):** Conversion failures and critical issues
- **Success (Green):** Successful conversions and operations

### Security and Privacy

**Path Sanitization:**

For security and privacy, full file paths are **never** displayed in error messages:

- ✅ Only filenames are shown (not full directory paths)
- ✅ User directories and usernames are never exposed
- ✅ System paths are never revealed
- ✅ Example: `C:\Users\JohnDoe\Documents\photo.jpg` → Only "photo.jpg" is shown

**Information Minimization:**

- Error messages contain only the information needed to resolve the issue
- No technical system details are exposed
- No internal error codes or stack traces shown to users
- All messages are sanitized to prevent information disclosure

**Security Review:**

The error message system has been security-reviewed and approved (see `SECURITY_REVIEW_TASK_2.2.md`). All error messages follow security best practices to prevent information disclosure.

---

## Tips and Tricks

### Keyboard Shortcuts

**File Operations:**
- **Ctrl+O** (Cmd+O on macOS): Open file dialog to select a file for conversion
- **Ctrl+S** (Cmd+S on macOS): Save settings (when settings panel is open)

**Conversion:**
- **Enter**: Start conversion (when file and format are selected)

**Batch Processing:**
- **Ctrl+Enter** (Cmd+Enter on macOS): Start batch processing (if queue has pending items)
- **Ctrl+P** (Cmd+P on macOS): Pause/Resume batch processing
- **Space**: Pause/Resume batch processing (when processing is active)
- **Escape**: Cancel batch processing (when processing is active)

**Queue Management:**
- **Ctrl+A** (Cmd+A on macOS): Add files to batch queue (opens multi-file dialog)
- **Ctrl+Shift+D** (Cmd+Shift+D on macOS): Clear batch queue (shows confirmation dialog)

**Navigation:**
- **Ctrl+R** (Cmd+R on macOS): Reset/Clear current file selection and options
- **Escape**: Close dialogs or cancel batch processing
- **Tab**: Navigate between fields
- **Arrow Keys**: Navigate radio buttons in format selection

### Best Practices

1. **Check file before converting:**
   - Verify file is not corrupted
   - Check file size is reasonable

2. **Choose appropriate format:**
   - Photos: JPEG (quality 85-95)
   - Graphics with transparency: PNG
   - Web images: WebP (good compression)

3. **Monitor resource usage:**
   - Large files may take time
   - Progress indicator appears for long operations

4. **Use Clear button:**
   - Reset all settings quickly
   - Start fresh conversion

### Performance Tips

- **Small files (< 10 MB):** Convert in seconds
- **Medium files (10-50 MB):** May take 10-30 seconds
- **Large files (> 50 MB):** May take 1-5 minutes

Progress indicator appears automatically for operations > 30 seconds.

---

## Troubleshooting

### Common Issues

**Issue: "File type not supported"**
- **Cause:** File format not recognized
- **Solution:** Verify file extension matches file content, try different format

**Issue: "Can't read file"**
- **Cause:** File doesn't exist or no read permissions
- **Solution:** Check file path, verify permissions

**Issue: Conversion takes too long**
- **Cause:** Large file or complex format
- **Solution:** Wait for completion (progress indicator shows), or cancel and try smaller file

**Issue: "Invalid file path"**
- **Cause:** Path contains invalid characters or system directory
- **Solution:** Select different output location

**Issue: GUI doesn't respond during conversion**
- **Cause:** Normal behavior for long operations
- **Solution:** Wait for completion, UI remains responsive for other actions

### Getting Help

#### Help Menu

The application includes a Help menu in the menu bar with the following options:

**Keyboard Shortcuts...**
- Opens the help panel showing all available keyboard shortcuts
- Includes shortcuts for file operations, conversion, batch processing, queue management, and navigation
- Access via: Menu bar → Help → Keyboard Shortcuts...

**About**
- Displays application information including:
  - Version number (currently v0.3.0)
  - License information (MIT OR Apache-2.0)
  - Copyright information
  - Link to GitHub repository
  - List of technologies used
- Access via: Menu bar → Help → About

#### Help Panel

The help panel provides comprehensive assistance:

**Keyboard Shortcuts Reference:**
- Complete list of all keyboard shortcuts organized by category
- Includes platform-specific notes (Ctrl vs Cmd)
- Quick reference for common operations

**Feature Overview:**
- List of supported image formats
- List of supported mesh formats
- Key features summary

**Troubleshooting Tips:**
- Common issues and solutions
- Step-by-step resolution guidance
- Performance tips

**Additional Resources:**
- Link to GitHub repository
- Documentation references

#### Additional Help

If you encounter issues not covered here:

1. Check error messages in the messages area
2. Verify file format is supported
3. Check file is not corrupted
4. Try converting to a different format
5. Access the Help menu for keyboard shortcuts and troubleshooting
6. Report issues through the project repository

---

## Technical Details

### Architecture

The GUI uses **direct library integration** with `img-core` and `mesh-core`:
- No subprocess calls to CLI binaries
- Better security and performance
- Type-safe error handling

### Security Features

- **Two-stage format detection:** Extension + magic bytes validation
- **Path validation:** Prevents path traversal attacks
- **Resource limits:** Prevents DoS attacks
- **Error sanitization:** No information leakage

### Thread Safety

- Conversions run in separate threads
- UI remains responsive during conversion
- Thread-safe state sharing using `Arc<Mutex<>>`

---

## v0.2.2 New Features

### Batch Processing

Convert multiple files at once using the batch processing queue:

1. **Add files to queue** - Click "Add Files" or drag multiple files into the batch queue panel
2. **Review queue** - Check files, formats, and settings for each item
3. **Edit queue items (v0.3.0)** - Click "Edit" button on any pending item to change format, output path, or options
4. **Process queue** - Click "Process Queue" to convert all files sequentially
5. **Monitor progress** - Watch real-time progress for each file
6. **Review results** - Check statistics and handle any failed conversions

**For detailed information, see:** [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md)

### Queue Item Editing (v0.3.0)

Edit queue items before processing to fix mistakes or adjust settings:

1. **Open edit dialog** - Click the "Edit" button on any pending queue item
2. **Edit fields** - Change:
   - **Output format** - Select a different format from the dropdown
   - **Output path** - Change the output file location
   - **Quality** - Adjust quality for lossy image formats (JPEG, WebP)
   - **Mesh options** - Adjust mesh conversion options (if applicable)
3. **Validate changes** - The dialog validates your changes:
   - Format compatibility is checked
   - Output path is validated
   - Invalid values are rejected
4. **Save changes** - Click "Save" to update the queue item
5. **Cancel** - Click "Cancel" or close the dialog to discard changes

**Restrictions:**
- **Pending items only** - You can only edit items with "Pending" status
- **Processing items** - Cannot edit items currently being processed
- **Completed items** - Cannot edit items that are already completed or failed

**Benefits:**
- Fix mistakes without removing and re-adding items
- Adjust settings for individual items
- More flexible batch processing workflow

### Preview Functionality

Preview images and meshes before conversion:

1. **Select a file** - Choose an image or mesh file
2. **Preview appears** - Image preview shows in preview panel (meshes show metadata)
3. **Review before converting** - Verify file is correct before conversion
4. **Format change updates preview** - Preview updates when you change output format

**Image Previews:**
- Shows thumbnail for large images
- Automatically resized to fit preview panel
- Maintains aspect ratio

**3D Mesh Viewer (v0.3.0):**
- Interactive 3D preview of mesh files (STL, OBJ, PLY, OFF, glTF, DXF)
- Hardware-accelerated rendering using wgpu
- Automatic mesh loading when mesh file is selected
- See [3D Viewer Controls](#3d-viewer-controls) section below for details

### Settings Persistence

Your preferences are saved automatically:

1. **Open Settings** - Click "Settings" in menu bar
2. **Configure preferences** - Set default output directory, quality, resource limits, etc.
3. **Auto-save (v0.3.0)** - Settings automatically save 500ms after you make a change (no need to click "Save")
4. **Manual save** - You can still click "Save" to save immediately
5. **Settings restored** - Your preferences are restored on next launch

**Auto-Save Status Indicator:**
- Shows current auto-save status (Idle, Pending, Saving, Saved, Error)
- Visual feedback confirms when settings are saved

**For detailed information, see:** [Settings Guide](SETTINGS_GUIDE.md)

### Conversion History

Track your recent conversions:

1. **History enabled** - Conversion history is enabled by default
2. **View history** - Access conversion history from menu or panel
3. **Recent conversions** - See source file, output file, format, and timestamp
4. **Status indicators** - Each entry shows a status icon:
   - **Green ✓** - Conversion successful (hover to see "Conversion successful" tooltip)
   - **Red ✗** - Conversion failed (hover to see "Conversion failed" tooltip)
   - **Note:** These are status indicators only, not interactive checkboxes
5. **Open output** - Click "Open Output" button to open the converted file in your system's default application
   - Only available for successful conversions
   - Opens the file using the system default application (e.g., image viewer, 3D model viewer)
   - Shows success message in messages area when file opens
   - Shows error message if file cannot be opened or doesn't exist
6. **Remove entries** - Click "Remove" button to delete individual history entries
7. **Clear history** - Click "Clear History" button to remove all entries

**Note:** History is stored in settings file and persists across sessions.

---

## v0.3.0 Features (Implemented)

### Parallel Batch Processing

**Status:** ✅ **IMPLEMENTED** - Parallel processing is now available in v0.3.0

**Features:**
- ✅ Concurrent file conversion using thread pool (`rayon` library)
- ✅ Automatic thread management based on CPU cores (default: CPU cores, capped at 8)
- ✅ Configurable maximum concurrent conversions (Settings → Conversion → Max Concurrent Conversions, range: 1-16)
- ✅ Real-time progress tracking for each parallel operation
- ✅ Improved performance for large batch queues (4x speedup on 4-core systems)
- ✅ Thread-safe queue management
- ✅ Error isolation (individual failures don't stop parallel processing)

**How to Use:**
1. **Configure Concurrency (Optional):**
   - Open Settings (menu bar → Settings)
   - Navigate to "Conversion" section
   - Adjust "Max Concurrent Conversions" slider (1-16)
   - Default: Number of CPU cores (capped at 8)
   - Settings auto-save after 500ms

2. **Process Batch Queue:**
   - Add files to batch queue (as before)
   - Click "Process Queue" button
   - Multiple files process simultaneously
   - Watch real-time progress for each file

**Performance:**
- **4-core system:** Up to 4x faster than sequential processing
- **8-core system:** Up to 8x faster than sequential processing
- **Example:** 10 files (2 seconds each):
  - Sequential: 20 seconds
  - Parallel (4 cores): ~5 seconds

**Configuration Tips:**
- **High-end systems (8+ cores, 16GB+ RAM):** Use 8 concurrent conversions
- **Mid-range systems (4 cores, 8GB RAM):** Use 4 concurrent conversions
- **Low-end systems (2 cores, 4GB RAM):** Use 2 concurrent conversions
- **Memory-constrained:** Reduce concurrency if experiencing high memory usage

**For detailed information, see:** [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md#parallel-processing-v030)

### Pause, Resume, and Cancel Controls (v0.3.0)

**Status:** ✅ **COMPLETE** - Fully implemented in v0.3.0 (Sprint 10)

**Pause Processing:**
- Click the **"Pause"** button to temporarily stop batch processing
- Currently processing items will finish, but new items won't start
- Queue state is preserved (no data loss)
- Processing can be resumed at any time

**Resume Processing:**
- Click the **"Resume"** button to continue processing after pausing
- Processing continues from where it left off
- Pending items resume processing in order

**Cancel Processing:**
- Click the **"Cancel"** button to stop batch processing completely
- Currently processing items will finish (cannot interrupt in-progress conversions)
- All pending items are marked as "Cancelled"
- Queue statistics update to reflect cancelled items

**Visual Feedback:**
- Pause button shows when processing is active
- Resume button shows when processing is paused
- Cancel button is always available during processing
- Status indicators show current state (Processing, Paused, Cancelled)

**Use Cases:**
- **Pause:** Temporarily stop to free up system resources
- **Resume:** Continue processing after pause
- **Cancel:** Stop processing entirely (e.g., wrong files in queue)

**Note:** These controls are fully implemented and available in v0.3.0.

**For detailed information, see:** [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md#pause-resume-and-cancel-v030)

### 3D Viewer Controls

**v0.3.0 Feature:** Full 3D preview of mesh files before conversion.

#### Accessing the 3D Viewer

1. **Select a mesh file** - Choose any supported mesh format (STL, OBJ, PLY, OFF, glTF, DXF)
2. **Preview panel expands** - The preview panel automatically shows the 3D viewer
3. **Mesh loads automatically** - The mesh is loaded and displayed in the viewer

#### Camera Controls

**Orbit (Rotate):**
- **Mouse drag** - Click and drag in the viewer to orbit around the mesh
- Rotates the camera around the mesh center
- Smooth, responsive rotation

**Pan (Move):**
- **Shift + Mouse drag** - Hold Shift and drag to pan the view
- Moves the camera without rotating
- Useful for examining different parts of the mesh

**Zoom:**
- **Mouse wheel** - Scroll up to zoom in, scroll down to zoom out
- Smooth zoom with limits to prevent extreme close-ups or far views
- Zoom is centered on the mesh

**Reset Camera:**
- **Reset Camera button** - Click to return to the default viewing angle
- Automatically positions camera to view entire mesh
- Useful when you've moved the camera too far

#### Rendering Modes

**Solid Mode (Default):**
- Shows mesh with solid surfaces and lighting
- Best for viewing mesh structure and surface details
- Uses directional lighting for depth perception

**Wireframe Mode:**
- Shows mesh edges only (no surfaces)
- Best for examining mesh topology and structure
- Useful for debugging mesh issues

**Switching Modes:**
- Use the **"Solid"** and **"Wireframe"** buttons above the viewer
- Current mode is highlighted
- Switch instantly between modes

#### Performance

**Optimized for:**
- Meshes up to 100,000 vertices render smoothly
- Larger meshes may have reduced frame rates
- Performance depends on your graphics hardware

**Requirements:**
- Requires wgpu-compatible graphics hardware
- Automatically falls back gracefully if wgpu unavailable
- Error messages shown if rendering fails

#### Benefits

- **Visual verification** - See exactly what you're converting before processing
- **Better understanding** - Examine mesh structure, topology, and details
- **Catch issues early** - Identify problems before conversion
- **Format preview** - See how mesh looks before converting to different format

#### Troubleshooting

**Viewer not showing:**
- Check that mesh file is valid and supported format
- Verify wgpu is available (most modern systems support it)
- Check error messages in messages area

**Performance issues:**
- Large meshes (>100k vertices) may be slow
- Try wireframe mode for better performance
- Close other applications to free up GPU resources

**Camera stuck:**
- Click "Reset Camera" button to return to default view
- Try zooming out (scroll down) if mesh appears too large

**Rendering errors:**
- Check that mesh file is not corrupted
- Verify mesh has valid geometry (vertices and faces)
- Error messages will explain the issue

---

**For more information, see:**
- [README.md](../README.md) - Project overview
- [CHANGELOG.md](../CHANGELOG.md) - Version history
- [RELEASE_NOTES_v0.2.1.md](../RELEASE_NOTES_v0.2.1.md) - Release details
- [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md) - Batch conversion guide
- [Settings Guide](SETTINGS_GUIDE.md) - Configuration guide

