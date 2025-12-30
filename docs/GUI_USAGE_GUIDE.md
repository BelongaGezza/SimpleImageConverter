# GUI Usage Guide
## Simple Image Converter - Graphical User Interface

**Version:** 0.2.2  
**Last Updated:** December 30, 2025

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

All error messages are designed to be **user-friendly** and **actionable**:

**Common Messages:**

- **"File type not supported."**
  - Solution: Select a supported file format (see Format Selection section)

- **"Can't read file. Check if file exists."**
  - Solution: Verify file exists and you have read permissions

- **"File too large. Maximum size is 100 MB."**
  - Solution: Use Advanced Options to increase limit (if appropriate)

- **"Image too large. Maximum dimension is 65535 pixels."**
  - Solution: Resize image before conversion, or increase limit in Advanced Options

- **"Invalid file path."**
  - Solution: Select a valid file path (avoid system directories)

- **"Conversion failed. Please try again."**
  - Solution: Check file is not corrupted, try different format

### Message Types

Messages are color-coded for easy identification:

- **Info (Blue):** General information
- **Warning (Yellow):** Non-critical issues (e.g., file will be overwritten)
- **Error (Red):** Conversion failures
- **Success (Green):** Successful conversions

### Path Sanitization

For security, full file paths are never displayed:
- User directories are removed
- Long paths are truncated
- Example: `C:\Users\JohnDoe\Documents\photo.jpg` → `Documents\photo.jpg`

---

## Tips and Tricks

### Keyboard Shortcuts

- **Tab:** Navigate between fields
- **Enter/Space:** Activate buttons
- **Arrow Keys:** Navigate radio buttons

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

If you encounter issues not covered here:

1. Check error messages in the messages area
2. Verify file format is supported
3. Check file is not corrupted
4. Try converting to a different format
5. Report issues through the project repository

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

**Note:** Image previews show thumbnails for large images. Mesh previews show metadata (full 3D viewer planned for future version).

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

## v0.3.0 Planned Features (In Development)

### Parallel Batch Processing

**Status:** ⏳ Planned for future version - Currently batch processing is sequential

**Planned Features:**
- Concurrent file conversion using thread pool
- Automatic thread management based on CPU cores
- Configurable maximum concurrent conversions
- Real-time progress tracking for each parallel operation
- Improved performance for large batch queues

**Current Implementation:**
- Batch processing is currently sequential (one file at a time)
- Files are processed in order
- UI remains responsive during processing
- Progress tracking works for sequential processing

**Note:** Parallel batch processing is being researched and designed. Sequential processing is fully functional and suitable for most use cases.

### 3D Mesh Viewer

**Coming in v0.3.0:** Full 3D preview of mesh files before conversion.

**Features:**
- Interactive 3D mesh rendering
- Camera controls (rotate, zoom, pan)
- Integration with preview panel
- Performance optimized for large meshes

**Benefits:**
- Visual verification before conversion
- Better understanding of mesh structure
- Catch issues before processing

**Note:** These features are currently in research and development. Check the [CHANGELOG.md](../CHANGELOG.md) for the latest status.

---

**For more information, see:**
- [README.md](../README.md) - Project overview
- [CHANGELOG.md](../CHANGELOG.md) - Version history
- [RELEASE_NOTES_v0.2.1.md](../RELEASE_NOTES_v0.2.1.md) - Release details
- [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md) - Batch conversion guide
- [Settings Guide](SETTINGS_GUIDE.md) - Configuration guide

