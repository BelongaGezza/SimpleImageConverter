# GUI Usage Guide
## Simple Image Converter - Graphical User Interface

**Version:** 0.2.1  
**Last Updated:** December 2025

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

**For more information, see:**
- [README.md](../README.md) - Project overview
- [CHANGELOG.md](../CHANGELOG.md) - Version history
- [RELEASE_NOTES_v0.2.1.md](../RELEASE_NOTES_v0.2.1.md) - Release details

