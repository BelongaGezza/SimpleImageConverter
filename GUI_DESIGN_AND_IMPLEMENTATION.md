# GUI Design and Implementation Plan
## Simple Image Converter - Graphical User Interface

**Designer:** Jamie Chen (UI Designer Agent)  
**Date:** December 2025  
**Status:** Design Complete - Reviewed and Approved  
**Review Status:** Approved with conditions addressed (see `GUI_DESIGN_REVIEWS.md`)

---

## Executive Summary

This document outlines the design and implementation plan for a graphical user interface (GUI) for Simple Image Converter. The GUI will provide an intuitive, drag-and-drop interface that uses the existing proven libraries (`img-core` and `mesh-core`) directly, making file conversion accessible to non-technical users while maintaining the library-first architecture principle.

**Architecture Compliance**: This design follows Phase 3 Architecture principles:
- Library-first design (direct library integration, not subprocess calls)
- Trait-based format system
- Comprehensive error handling using `common::error`
- Resource limits using `common::limits`

---

## Design Goals

1. **Simplicity First**: Most common action (drag file, select format, convert) = fewest clicks
2. **Feedback Always**: Every action has visible feedback, progress indicators for long operations
3. **Forgiveness**: Clear error messages, confirmation for destructive actions
4. **Low-Tech Friendly**: Abbreviated, simple messages that avoid technical jargon

---

## Supported File Types

### 2D Image Formats (via `img-convert`)
- **Input**: PNG, JPEG/JPG, BMP, GIF, TIFF/TIF, WebP, SVG (read-only, rasterized)
- **Output**: PNG, JPEG/JPG, BMP, GIF, TIFF/TIF, WebP
- **Note**: SVG can only be converted FROM (rasterized), not TO

### 3D Mesh Formats (via `mesh-convert`)
- **Input**: STL, OBJ, PLY, OFF, glTF/GLB, DXF, STEP/STP (feature-gated)
- **Output**: STL, OBJ, PLY, OFF, glTF/GLB, DXF
- **Note**: STEP can only be converted FROM (read-only), not TO

---

## GUI Layout Design

### Main Window Structure

```
┌─────────────────────────────────────────────────────────────────┐
│  Simple Image Converter                    [─] [□] [×]          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │                                                           │ │
│  │        📁 Drag & Drop File Here                          │ │
│  │        or click to browse                                │ │
│  │                                                           │ │
│  │        [Browse Files...]                                  │ │
│  │                                                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  Source File: [No file selected]                               │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Output Format:                                            │ │
│  │                                                           │ │
│  │  ○ PNG                                                    │ │
│  │  ○ JPEG                                                   │ │
│  │  ○ BMP                                                    │ │
│  │  ○ GIF                                                    │ │
│  │  ○ TIFF                                                   │ │
│  │  ○ WebP                                                    │ │
│  │                                                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Options:                                                  │ │
│  │                                                           │ │
│  │  Output Filename: [photo.jpg]                            │ │
│  │  Output Location: [C:\Users\...\Documents] [Browse...]  │ │
│  │                                                           │ │
│  │  Quality (1-100): [90] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │ │
│  │                                                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Messages:                                                  │ │
│  │                                                           │ │
│  │  [No messages]                                            │ │
│  │                                                           │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Status: Ready                                             │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│                    [Convert]  [Clear]                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Specifications

### 1. Source File Drop Zone

**Location**: Top of main panel  
**Behavior**:
- Large, clearly visible drop zone (minimum 200px height)
- Visual feedback on drag-over (highlight border, change background)
- Click to open file browser
- Display selected file name and path below drop zone
- **Security Validation** (two-stage format detection):
  - Validate file extension
  - Validate magic bytes (format signature)
  - Check file size before reading (DoS prevention)
  - Reject symbolic links
  - Use `common::validation::validate_file_path()` for path validation
- Show error if file type not supported

**Visual States**:
- **Empty**: Light gray background, dashed border, centered text
- **Drag Over**: Blue border, light blue background
- **File Selected**: Green border, show file name and icon
- **Error**: Red border, show error message

### 2. Output Format Selection

**Location**: Below source file area  
**Behavior**:
- Radio button group showing available output formats
- **Format filtering**: Only show formats compatible with source file type
  - Image files → Show image output formats (exclude SVG if source is SVG)
  - Mesh files → Show mesh output formats (exclude STEP if source is STEP)
- Default selection: First available format (alphabetically)
- Disable radio buttons for formats that can't be written (e.g., SVG, STEP)

**Format Lists**:

**Image Output Formats** (when source is image):
- PNG
- JPEG
- BMP
- GIF
- TIFF
- WebP

**Mesh Output Formats** (when source is mesh):
- STL
- OBJ
- PLY
- OFF
- glTF
- DXF

### 3. Options Panel

**Location**: Below format selection  
**Components**:

#### 3.1 Output Filename
- **Default**: Source filename (without extension) + selected format extension
  - Example: `photo.png` → `photo.jpg` (if JPEG selected)
  - Uses `PathBuf::with_extension()` for safe extension replacement
  - Handles edge cases: no extension, multiple extensions, invalid characters
- **Editable**: Text field allowing user to customize filename
- **Validation**: 
  - Check for invalid characters (Windows reserved: `< > : " | ? *`)
  - Prevent path traversal attacks (`../`)
  - Ensure extension matches selected format
  - Validate path length (Windows MAX_PATH: 260 chars)
  - Warn if file already exists
  - Validate output path is not in system directories (security)

#### 3.2 Output Location
- **Default**: Same directory as source file
- **Display**: Show current directory path (truncate if too long, sanitize for display)
- **Button**: "Browse..." to select different directory
- **Validation**: 
  - Check write permissions before conversion starts
  - Validate path is not in system directories
  - Use `common::validation::validate_file_path()` for path checks

#### 3.3 Quality Slider (Images Only)
- **Visible**: Only when converting images to lossy formats (JPEG, WebP)
- **Range**: 1-100
- **Default**: 90
- **Display**: Show numeric value next to slider
- **Label**: "Quality (1-100): [90]"

#### 3.4 Advanced Options (Collapsible)
- **Default**: Hidden
- **Toggle**: "Show Advanced Options" button
- **Options** (when expanded):
  - Max file size (MB) - Default: 100MB, Max: 1GB (with warning)
  - Max dimension (pixels) - Images only, Default: 65535
  - Max vertices/faces - Meshes only, Default: 10,000,000 each
  - Format variant (ASCII/Binary) - STL only
- **Security Note**: Advanced limits use `common::limits::ResourceLimits` builder
- **Warning**: Display warning if user increases limits beyond safe defaults

### 4. Messages Area

**Location**: Below options panel  
**Purpose**: Display warnings, errors, and informational messages  
**Behavior**:
- Scrollable text area
- **Message Types**:
  - **Info** (blue): General information
  - **Warning** (yellow): Non-critical issues
  - **Error** (red): Conversion failures
  - **Success** (green): Successful conversions

**Message Format** (Low-Tech Friendly):
- **Good**: "File converted successfully"
- **Bad**: "ConversionError::InvalidInput: File validation failed due to..."

**Abbreviation Rules**:
- Use simple language: "File too large" instead of "Resource limit exceeded"
- Avoid technical terms: "Can't read file" instead of "I/O error occurred"
- Keep messages under 80 characters when possible
- Use icons/symbols: ✓ (success), ⚠ (warning), ✗ (error)

**Security - Error Message Sanitization**:
- Never display full file paths (truncate or use relative paths)
- Never display system information or stack traces
- Never display internal error types
- Sanitize paths before display (remove user home directory)
- Example: "Saved to: Documents\photo.jpg" instead of "C:\Users\JohnDoe\Documents\photo.jpg"

### 5. Status Bar

**Location**: Bottom of window  
**Purpose**: Show current operation status  
**States**:
- **Ready**: "Ready" (gray)
- **Converting**: "Converting..." (blue) + progress indicator if > 30 seconds
- **Success**: "Conversion complete" (green) + file path
- **Error**: "Conversion failed" (red) + brief error message

**Progress Indicator** (for conversions > 30 seconds):
- Show progress bar (indeterminate for now, future: integrate with `common::progress`)
- Track elapsed time using `std::time::Instant`
- Show elapsed time: "Converting... (45 seconds)"
- Thread-safe progress updates using `Arc<Mutex<ConversionState>>`

### 6. Action Buttons

**Location**: Bottom right of window  
**Buttons**:
- **Convert**: Start conversion (disabled if no file selected)
- **Clear**: Reset all fields to default state

---

## User Flow

### Standard Conversion Flow

1. **User drags file** into drop zone OR clicks "Browse Files..."
2. **System detects file type** using two-stage detection:
   - Extension-based detection (primary)
   - Magic byte validation (security check)
   - Uses `img_core::FormatRegistry::detect_two_stage()` for images
   - Uses `mesh_core::FormatRegistry::detect_two_stage()` for meshes (extension + signature verification where available)
3. **GUI updates**:
   - Shows file name in "Source File" field
   - Populates format radio buttons (filtered by file type)
   - Sets default output filename
   - Sets default output location (same as source)
   - Shows/hides quality slider based on format
4. **User selects output format** (radio button)
5. **GUI updates**:
   - Updates output filename extension
   - Shows/hides quality slider if needed
6. **User optionally adjusts**:
   - Output filename
   - Output location
   - Quality setting
7. **User clicks "Convert"**
8. **System validates** (security checks):
   - File exists and is readable
   - File size within limits (using `common::limits::ResourceLimits`)
   - Two-stage format validation (extension + magic bytes)
   - Output location is writable
   - Output path validation (not system directories)
   - Filename is valid (no invalid characters, no path traversal)
   - Resource limits enforced (file size, dimensions, vertices/faces)
9. **Conversion starts**:
   - Status bar: "Converting..."
   - Convert button disabled
   - If > 30 seconds: show progress bar
10. **Conversion completes**:
    - Status bar: "Conversion complete: [sanitized output path]"
    - Messages area: "File converted successfully"
    - Convert button re-enabled
    - Output path displayed with sanitization (relative or truncated)

### Error Handling Flow

1. **File validation error**:
   - Show error in messages area: "Can't read file. Check if file exists."
   - Status bar: "Error: Invalid file"
   - Keep file selected (user can fix and retry)

2. **Format detection error**:
   - Show error: "File type not supported"
   - Clear format selection
   - Status bar: "Error: Unsupported format"

3. **Conversion error**:
   - Show error in messages area: "Conversion failed. [Brief reason]"
   - Status bar: "Conversion failed"
   - Keep all settings (user can adjust and retry)

4. **Output file exists**:
   - Show confirmation dialog: "File already exists. Will be overwritten."
   - Validate output path is not in system directories (security)
   - Allow user to proceed or change filename
   - Check write permissions before conversion starts

---

## Technical Implementation

### Technology Stack

- **GUI Framework**: `egui` (immediate mode GUI for Rust)
- **Backend**: Direct library integration with `img-core` and `mesh-core` (library-first architecture)
- **File Handling**: Use `rfd` (Rust File Dialog) for file browser
- **Threading**: Use `std::thread` for long-running conversions (egui is immediate mode, blocking is acceptable)
- **Progress Updates**: Use `Arc<Mutex<>>` for thread-safe state sharing

### Project Structure

**Workspace Integration**: The GUI is integrated as a workspace member, not a separate project.

```
SimpleImageConverter/
├── Cargo.toml              # Workspace manifest (includes converter-gui)
├── common/                 # Shared utilities (used by GUI)
├── img-core/               # Image library (dependency)
├── img-convert/            # CLI binary
├── mesh-core/              # Mesh library (dependency)
├── mesh-convert/           # CLI binary
└── converter-gui/          # GUI application (NEW)
    ├── Cargo.toml
    └── src/
        ├── main.rs              # Application entry point
        ├── app.rs              # Main application state
        ├── ui/
        │   ├── mod.rs
        │   ├── drop_zone.rs    # File drop zone component
        │   ├── format_selector.rs  # Format radio buttons
        │   ├── options_panel.rs    # Options UI
        │   ├── messages.rs      # Messages display
        │   └── status_bar.rs   # Status bar component
        ├── conversion.rs       # Conversion logic (direct library integration)
        ├── error_messages.rs   # Error-to-message mapping
        └── utils.rs            # Helper functions (path sanitization, etc.)
```

**Workspace Cargo.toml Update**:
```toml
[workspace]
members = [
    "common",
    "img-core",
    "img-convert",
    "mesh-core",
    "mesh-convert",
    "converter-gui",  # Add this
]
```

### Key Dependencies

```toml
# converter-gui/Cargo.toml
[dependencies]
eframe = "0.27"           # egui framework
egui = "0.27"
rfd = "0.14"              # File dialogs

# Workspace dependencies (direct library integration)
common = { path = "../common" }
img-core = { path = "../img-core" }
mesh-core = { path = "../mesh-core" }
```

**Note**: `tokio` removed - not needed for egui's immediate mode. Use `std::thread` for long operations.

### Direct Library Integration

**Architecture Compliance**: The GUI uses direct library integration, following the library-first architecture principle. This eliminates security risks, improves error handling, and provides better performance.

#### Image Conversion

```rust
use img_core::{ImageConverter, FormatRegistry, ImageFormat, QualitySettings};
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;

// Build resource limits
let limits = ResourceLimits::builder()
    .max_file_size_mb(100)
    .max_image_dimension(65535)
    .build();

// Validate and read input file
let input_path = Path::new(&source_file);
common::validation::validate_file_path(input_path)?;
let input_data = read_file_bytes_checked(input_path, &limits)?;

// Two-stage format detection (security)
let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;
let output_format = FormatRegistry::detect_format(&selected_format)?;

// Get format handlers
let reader = FormatRegistry::get_reader(input_format)?;
let writer = FormatRegistry::get_writer(output_format)?;

// Convert
let converter = ImageConverter::new();
let quality = QualitySettings::new(quality_value);
let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref(), &quality)?;

// Write output
write_file_bytes(&output_path, &output_data)?;
```

#### Mesh Conversion

```rust
use mesh_core::{MeshConverter, FormatRegistry as MeshFormatRegistry, MeshFormat};
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;

// Build resource limits
let limits = ResourceLimits::builder()
    .max_file_size_mb(100)
    .max_vertices(10_000_000)
    .max_faces(10_000_000)
    .build();

// Validate and read input file
let input_path = Path::new(&source_file);
common::validation::validate_file_path(input_path)?;
let input_data = read_file_bytes_checked(input_path, &limits)?;

// Format detection
let input_format = MeshFormatRegistry::detect_from_path(input_path)?;
let output_format = MeshFormatRegistry::detect_format(&selected_format)?;

// Get format handlers with resource limits
let reader = MeshFormatRegistry::get_reader_with_limits(input_format, limits.clone())?;
let writer = MeshFormatRegistry::get_writer(output_format)?;

// Convert
let converter = MeshConverter::new();
let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref())?;

// Write output
write_file_bytes(&output_path, &output_data)?;
```

**Benefits of Direct Library Integration**:
- ✅ Better error handling (structured errors vs. string parsing)
- ✅ No PATH dependency
- ✅ Faster execution (no process spawn overhead)
- ✅ Better progress reporting (can use callbacks)
- ✅ Type safety
- ✅ Eliminates command injection risks
- ✅ Aligns with Phase 3 Architecture principles

---

## Message Examples (Low-Tech Friendly)

### Success Messages
- ✓ "File converted successfully"
- ✓ "Saved to: Documents\photo.jpg" (sanitized path, not full path)
- ✓ "Conversion complete"

### Warning Messages
- ⚠ "File already exists. Will be overwritten."
- ⚠ "Large file. This may take a while."
- ⚠ "Quality set to 50. File will be smaller but lower quality."

### Error Messages
- ✗ "Can't read file. Check if file exists."
- ✗ "File type not supported."
- ✗ "Conversion failed. File may be corrupted."
- ✗ "Can't save file. Check if you have permission."
- ✗ "File too large. Maximum size is 100 MB."

### Info Messages
- ℹ "Select a file to begin"
- ℹ "Converting... This may take a minute."
- ℹ "Quality: Higher = better quality, larger file"

---

## Error Handling Strategy

### Error-to-Message Mapping

The GUI converts `common::error::ConversionError` to user-friendly messages using a mapping function:

```rust
// error_messages.rs
use common::error::ConversionError;

pub fn format_user_message(error: &ConversionError) -> String {
    match error {
        ConversionError::InvalidInput(msg) => {
            if msg.contains("extension") || msg.contains("format") {
                "File type not supported.".to_string()
            } else if msg.contains("size") || msg.contains("too large") {
                "File too large. Maximum size is 100 MB.".to_string()
            } else if msg.contains("dimension") {
                "Image too large. Maximum dimension is 65535 pixels.".to_string()
            } else {
                "Invalid file. Check if file exists and is readable.".to_string()
            }
        }
        ConversionError::UnsupportedFormat(_) => {
            "File type not supported.".to_string()
        }
        ConversionError::IoError(_) => {
            "Can't read file. Check if file exists.".to_string()
        }
        ConversionError::SecurityError(msg) => {
            // Sanitize security error messages
            if msg.contains("path") {
                "Invalid file path.".to_string()
            } else {
                "Security validation failed.".to_string()
            }
        }
        _ => "Conversion failed. Please try again.".to_string()
    }
}
```

### Path Sanitization for Display

```rust
// utils.rs
use std::path::Path;

pub fn sanitize_path_for_display(path: &Path) -> String {
    // Get relative path or truncate
    if let Ok(relative) = path.strip_prefix(std::env::home_dir().unwrap_or_default()) {
        relative.display().to_string()
    } else {
        // Truncate long paths
        let path_str = path.display().to_string();
        if path_str.len() > 60 {
            format!("...{}", &path_str[path_str.len() - 57..])
        } else {
            path_str
        }
    }
}
```

---

## Threading Model for Long Operations

### Conversion Thread Pattern

For long-running conversions, use a separate thread with thread-safe state sharing:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Clone)]
enum ConversionStatus {
    Ready,
    Converting { start_time: Instant },
    Success { output_path: PathBuf },
    Error { message: String },
}

struct ConversionState {
    status: ConversionStatus,
    progress: f32,  // 0.0 to 1.0
    message: String,
}

impl App {
    fn start_conversion(&mut self) {
        let state = Arc::new(Mutex::new(ConversionState {
            status: ConversionStatus::Converting { 
                start_time: Instant::now() 
            },
            progress: 0.0,
            message: "Converting...".to_string(),
        }));

        // Clone for thread
        let state_clone = Arc::clone(&state);
        let input_path = self.input_path.clone();
        let output_path = self.output_path.clone();
        // ... other conversion parameters

        thread::spawn(move || {
            // Perform conversion
            match perform_conversion(&input_path, &output_path, ...) {
                Ok(path) => {
                    let mut state = state_clone.lock().unwrap();
                    state.status = ConversionStatus::Success { output_path: path };
                    state.progress = 1.0;
                    state.message = "Conversion complete".to_string();
                }
                Err(e) => {
                    let mut state = state_clone.lock().unwrap();
                    state.status = ConversionStatus::Error { 
                        message: format_user_message(&e) 
                    };
                    state.progress = 0.0;
                }
            }
        });

        self.conversion_state = Some(state);
    }

    fn update_ui(&mut self, ctx: &egui::Context) {
        if let Some(state) = &self.conversion_state {
            let state_guard = state.lock().unwrap();
            
            // Update UI based on state
            match &state_guard.status {
                ConversionStatus::Converting { start_time } => {
                    let elapsed = start_time.elapsed();
                    if elapsed.as_secs() > 30 {
                        // Show progress bar
                        ui.add(egui::ProgressBar::new(state_guard.progress));
                    }
                }
                // ... other states
            }
        }
    }
}
```

---

## Default Output Filename Generation

### Implementation with Edge Cases

The default output filename is generated from the source filename with the selected format extension:

```rust
// utils.rs
use std::path::{Path, PathBuf};

pub fn generate_output_filename(input: &Path, output_format: &str) -> Result<PathBuf> {
    let mut output = input.to_path_buf();
    
    // Remove old extension
    output.set_extension("");
    
    // Handle edge case: file with no extension
    // PathBuf::set_extension("") removes extension, which is what we want
    
    // Add new extension
    output.set_extension(output_format);
    
    // Validate generated filename
    validate_output_filename(&output)?;
    
    Ok(output)
}

fn validate_output_filename(path: &Path) -> Result<()> {
    // Check for invalid characters (Windows reserved: < > : " | ? *)
    let filename = path.file_name()
        .ok_or_else(|| ConversionError::InvalidInput("Invalid filename".to_string()))?
        .to_string_lossy();
    
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    if filename.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(ConversionError::InvalidInput(
            "Filename contains invalid characters".to_string()
        ));
    }
    
    // Check path length (Windows MAX_PATH: 260 chars)
    let path_str = path.display().to_string();
    if path_str.len() > 260 {
        return Err(ConversionError::InvalidInput(
            "Path too long (maximum 260 characters)".to_string()
        ));
    }
    
    Ok(())
}
```

**Edge Cases Handled**:
- File with no extension: `document` → `document.jpg`
- File with multiple extensions: `archive.tar.gz` → `archive.tar.jpg` (removes last extension)
- File with invalid characters: Validated and rejected with user-friendly error
- Very long filenames: Validated against Windows MAX_PATH limit
- Path traversal: Prevented by `common::validation::validate_file_path()`

## Format Detection Integration

### Image Format Detection

```rust
use img_core::{FormatRegistry, ImageFormat};

// Two-stage detection (extension + magic bytes for security)
let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;

// Get available output formats (filter read-only formats)
fn get_writable_image_formats() -> Vec<ImageFormat> {
    vec![
        ImageFormat::Png,
        ImageFormat::Jpeg,
        ImageFormat::Bmp,
        ImageFormat::Gif,
        ImageFormat::Tiff,
        ImageFormat::WebP,
        // Note: SVG is read-only, excluded
    ]
}
```

### Mesh Format Detection

```rust
use mesh_core::{FormatRegistry as MeshFormatRegistry, MeshFormat};

// Format detection
let input_format = MeshFormatRegistry::detect_from_path(input_path)?;

// Get available output formats (filter read-only formats)
fn get_writable_mesh_formats() -> Vec<MeshFormat> {
    vec![
        MeshFormat::Stl,
        MeshFormat::Obj,
        MeshFormat::Ply,
        MeshFormat::Off,
        MeshFormat::Gltf,
        MeshFormat::Dxf,
        // Note: STEP is read-only (feature-gated), excluded
    ]
}
```

---

## Security Validation Checklist

All file operations must pass these security validations:

1. **Path Validation**:
   - [ ] Use `common::validation::validate_file_path()` for all paths
   - [ ] Prevent path traversal attacks (`../`)
   - [ ] Validate path length (Windows MAX_PATH: 260 chars)
   - [ ] Check for invalid characters

2. **File Validation**:
   - [ ] Two-stage format detection (extension + magic bytes)
   - [ ] Check file size before reading (DoS prevention)
   - [ ] Reject symbolic links (or resolve safely)
   - [ ] Validate file exists and is readable

3. **Resource Limits**:
   - [ ] Use `common::limits::ResourceLimits` builder
   - [ ] Enforce default limits (100MB file, 65535 pixels, 10M vertices/faces)
   - [ ] Validate user-adjusted limits are within safe bounds (max 1GB)
   - [ ] Warn user if limits are increased

4. **Output Validation**:
   - [ ] Validate output path is not in system directories
   - [ ] Check write permissions before conversion starts
   - [ ] Validate output filename (no invalid characters)
   - [ ] Confirm overwrite for existing files

5. **Error Message Sanitization**:
   - [ ] Never display full file paths
   - [ ] Never display system information or stack traces
   - [ ] Never display internal error types
   - [ ] Use `sanitize_path_for_display()` for all path displays

---

## Accessibility Considerations

1. **Keyboard Navigation**:
   - Tab through all interactive elements
   - Enter/Space to activate buttons
   - Arrow keys for radio buttons

2. **Screen Reader Support**:
   - All UI elements have descriptive labels
   - Status changes are announced
   - Error messages are clearly marked

3. **Visual Accessibility**:
   - High contrast color scheme (WCAG AA minimum)
   - Clear visual hierarchy
   - Large click targets (minimum 44x44px)

4. **UI Scaling**:
   - Support system DPI scaling
   - Allow window resizing (minimum 800x600)

---

## Future Enhancements

1. **Batch Conversion**: Support multiple files at once
2. **Conversion History**: Remember recent conversions
3. **Presets**: Save common conversion settings
4. **Preview**: Show image preview before conversion
5. **Progress Details**: Show more detailed progress for large files
6. **Settings**: Persistent user preferences

---

## Implementation Phases

### Phase 1: Core UI (Week 1)
- [ ] Add converter-gui to workspace Cargo.toml
- [ ] Set up egui project structure
- [ ] Add dependencies (eframe, egui, rfd, workspace crates)
- [ ] Implement main window layout
- [ ] Create file drop zone with drag/drop support
- [ ] Implement file browser integration (rfd)
- [ ] Basic file type detection (format registry integration)
- [ ] Security: Path validation on file selection

### Phase 2: Format Selection (Week 1)
- [ ] Implement format radio buttons
- [ ] Format filtering based on source file type (image vs mesh)
- [ ] Two-stage format detection integration (extension + magic bytes)
- [ ] Default format selection logic
- [ ] Format-specific UI (quality slider, etc.)
- [ ] Filter read-only formats (SVG, STEP) from output options

### Phase 3: Options Panel (Week 2)
- [ ] Output filename field with auto-generation (PathBuf::with_extension)
- [ ] Filename validation (invalid characters, path traversal)
- [ ] Output location browser
- [ ] Output path validation (not system directories, write permissions)
- [ ] Quality slider for images
- [ ] Advanced options (collapsible)
- [ ] Resource limits UI (with warnings for increased limits)
- [ ] Path sanitization for display

### Phase 4: Conversion Integration (Week 2)
- [ ] Direct library integration (img-core, mesh-core)
- [ ] Resource limits enforcement (common::limits)
- [ ] Two-stage format detection (security)
- [ ] Error-to-message mapping function
- [ ] Thread-safe progress reporting (Arc<Mutex>)
- [ ] Progress tracking for long operations (>30 seconds)
- [ ] Error handling and message display
- [ ] Status bar updates
- [ ] Path sanitization for display

### Phase 5: Polish (Week 3)
- [ ] Error message mapping function (error_messages.rs)
- [ ] Message formatting (low-tech friendly)
- [ ] Error message sanitization (no path leaks)
- [ ] Visual feedback improvements
- [ ] Accessibility improvements
- [ ] Unit tests for conversion logic
- [ ] Integration tests for format detection
- [ ] Testing and bug fixes

---

## Testing Checklist

### Functional Tests
- [ ] Drag and drop image file
- [ ] Drag and drop mesh file
- [ ] Browse for file
- [ ] Select different output formats
- [ ] Change output filename
- [ ] Change output location
- [ ] Adjust quality slider
- [ ] Convert image to JPEG
- [ ] Convert image to PNG
- [ ] Convert mesh to OBJ
- [ ] Convert mesh to STL
- [ ] Handle unsupported file type
- [ ] Handle conversion error
- [ ] Handle file already exists
- [ ] Handle large file (> 30 seconds)
- [ ] Keyboard navigation
- [ ] Window resizing

### Security Tests
- [ ] Path traversal attack prevention (`../etc/passwd`)
- [ ] Invalid character validation in filenames
- [ ] File size limit enforcement (DoS prevention)
- [ ] Two-stage format detection (magic bytes validation)
- [ ] Output path validation (not system directories)
- [ ] Resource limits enforcement
- [ ] Error message sanitization (no path leaks)
- [ ] Symbolic link handling

### Unit Tests
- [ ] Error message mapping function
- [ ] Path sanitization function
- [ ] Format detection logic
- [ ] Output filename generation (edge cases)
- [ ] Resource limits validation

### Integration Tests
- [ ] Direct library integration (img-core)
- [ ] Direct library integration (mesh-core)
- [ ] Format registry integration
- [ ] Resource limits integration

## Testing Strategy

### Unit Tests

Test conversion logic without GUI components:

```rust
// conversion.rs - testable without GUI
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_detection_image() {
        let path = Path::new("test.png");
        let format = detect_file_type(path).unwrap();
        assert!(matches!(format, FileType::Image(_)));
    }
    
    #[test]
    fn test_format_detection_mesh() {
        let path = Path::new("test.stl");
        let format = detect_file_type(path).unwrap();
        assert!(matches!(format, FileType::Mesh(_)));
    }
    
    #[test]
    fn test_error_message_formatting() {
        let error = ConversionError::InvalidInput("extension".to_string());
        let message = format_user_message(&error);
        assert_eq!(message, "File type not supported.");
    }
    
    #[test]
    fn test_output_filename_generation() {
        let input = Path::new("photo.png");
        let output = generate_output_filename(input, "jpg").unwrap();
        assert_eq!(output, Path::new("photo.jpg"));
    }
    
    #[test]
    fn test_output_filename_no_extension() {
        let input = Path::new("document");
        let output = generate_output_filename(input, "jpg").unwrap();
        assert_eq!(output, Path::new("document.jpg"));
    }
    
    #[test]
    fn test_path_sanitization() {
        let path = Path::new("C:\\Users\\JohnDoe\\Documents\\photo.jpg");
        let sanitized = sanitize_path_for_display(path);
        assert!(!sanitized.contains("JohnDoe")); // User directory removed
    }
}
```

### Integration Tests

Test format detection and conversion with actual libraries:

```rust
// tests/integration_tests.rs
#[cfg(test)]
mod integration_tests {
    use img_core::{FormatRegistry, ImageFormat};
    use mesh_core::{FormatRegistry as MeshFormatRegistry, MeshFormat};
    
    #[test]
    fn test_image_format_detection() {
        // Test with actual image file
        let format = FormatRegistry::detect_from_path("test.png").unwrap();
        assert_eq!(format, ImageFormat::Png);
    }
    
    #[test]
    fn test_mesh_format_detection() {
        // Test with actual mesh file
        let format = MeshFormatRegistry::detect_from_path("test.stl").unwrap();
        assert_eq!(format, MeshFormat::Stl);
    }
}
```

### Manual Testing

GUI components require manual testing (egui doesn't have comprehensive automated testing support):

- Visual appearance and layout
- Drag and drop functionality
- File browser dialogs
- User interaction flows
- Accessibility features

### Headless Testing (Future)

Consider using `eframe`'s headless mode for some automated UI tests if needed.

---

## Conclusion

This GUI design provides a simple, intuitive interface for the Simple Image Converter tools. By using direct library integration with the proven `img-core` and `mesh-core` libraries, we make file conversion accessible to users of all technical levels while maintaining the reliability, security, and architecture compliance of the underlying tools.

The design emphasizes:
- **Simplicity**: Clear feedback and forgiveness
- **Security**: Comprehensive validation and sanitization
- **Architecture Compliance**: Library-first design, trait-based formats
- **User Experience**: Low-tech friendly messages and intuitive workflow

**Review Status**: This design has been reviewed and approved by:
- System Architect (Alex Chen) - Architecture compliance verified
- Security Specialist (Casey Morgan) - Security concerns addressed
- Senior Engineer (Jordan Rivera) - Implementation feasibility confirmed

See `GUI_DESIGN_REVIEWS.md` for detailed review comments and recommendations.

---

**Next Steps**: 
1. Add `converter-gui` to workspace Cargo.toml
2. Begin Phase 1 implementation with egui project setup and basic window layout
3. Implement direct library integration (not subprocess calls)
4. Add security validations from the start

