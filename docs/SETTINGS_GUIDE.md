# Settings Guide
## Simple Image Converter - Configuration and Preferences

**Version:** 0.2.2  
**Last Updated:** December 30, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Accessing Settings](#accessing-settings)
3. [Settings Categories](#settings-categories)
4. [Default Settings](#default-settings)
5. [Customizing Settings](#customizing-settings)
6. [Settings File Location](#settings-file-location)
7. [Settings Persistence](#settings-persistence)
8. [Troubleshooting](#troubleshooting)

---

## Overview

Settings allow you to customize the application behavior and save your preferences across sessions. Your settings are automatically saved and restored when you launch the application.

### Key Features

- **Persistent preferences** - Settings saved automatically
- **Platform-specific storage** - Settings stored in appropriate system directories
- **Easy customization** - Simple UI for changing settings
- **Default fallback** - Defaults used if settings file missing or corrupted
- **Reset option** - Restore defaults at any time

### What Settings Control

- **Window state** - Window size and position (future)
- **Default options** - Output directory, quality settings
- **UI preferences** - Advanced options visibility, theme (future)
- **Recent files** - Track recently opened files
- **Conversion history** - Enable/disable history tracking

---

## Accessing Settings

### Opening the Settings Panel

1. Launch the GUI application
2. Click **"Settings"** in the menu bar (or use keyboard shortcut)
3. Settings panel opens in a dialog window
4. Make your changes
5. Click **"Save"** to apply and close, or **"Cancel"** to discard

### Settings Panel Layout

The settings panel is organized into categories:
- **General** - Default options and basic preferences
- **Conversion** - Quality defaults and resource limits
- **UI** - Interface preferences (future)
- **History** - Conversion history settings

---

## Settings Categories

### General Settings

**Default Output Directory:**
- Sets the default location for converted files
- Can be overridden per conversion
- Click "Browse..." to select a directory
- Default: Source file's directory

**Show Advanced Options:**
- Controls visibility of advanced options panel
- When enabled: Advanced options always visible
- When disabled: Advanced options collapsed by default
- Default: Disabled (collapsed)

**Recent Files:**
- Maximum number of recent files to track
- Range: 0-20 files
- Default: 10 files
- Recent files appear in File menu (future)

### Conversion Settings

**Default Quality:**
- Default quality setting for lossy formats (JPEG, WebP)
- Range: 1-100
- Default: 90
- Can be overridden per conversion

**Resource Limits:**
- **Max File Size:** Maximum file size to process (default: 100 MB)
- **Max Image Dimension:** Maximum width/height for images (default: 65535 pixels)
- **Max Vertices:** Maximum vertices for meshes (default: 10,000,000)
- **Max Faces:** Maximum faces for meshes (default: 10,000,000)

**Warning:** Increasing limits beyond defaults may cause performance issues or crashes on low-memory systems.

### UI Settings (Future)

**Theme:**
- Light/Dark mode selection (planned for future version)
- Default: System default

**Window State:**
- Remember window size and position (planned for future version)
- Default: Enabled

### History Settings

**Conversion History Enabled:**
- Enable or disable conversion history tracking
- When enabled: Conversions are tracked in history
- When disabled: No history is saved
- Default: Enabled

**Max History Entries:**
- Maximum number of history entries to keep
- Range: 0-1000 entries
- Default: 50 entries
- Older entries are automatically removed when limit reached

---

## Default Settings

### Default Values Summary

| Setting | Default Value | Notes |
|---------|--------------|-------|
| Default Output Directory | Source file directory | Changes per file |
| Show Advanced Options | Disabled (collapsed) | Advanced options hidden by default |
| Default Quality | 90 | For JPEG/WebP |
| Max File Size | 100 MB | DoS prevention |
| Max Image Dimension | 65535 pixels | Memory protection |
| Max Vertices | 10,000,000 | Mesh processing limit |
| Max Faces | 10,000,000 | Mesh processing limit |
| Recent Files | 10 | Maximum recent files tracked |
| History Enabled | Enabled | Conversion history active |
| Max History Entries | 50 | History size limit |

### When Defaults Are Used

Defaults are used when:
- First launch (no settings file exists)
- Settings file is corrupted or invalid
- Settings are reset to defaults
- Settings file is deleted

---

## Customizing Settings

### Changing Settings

1. **Open Settings** - Click "Settings" in menu bar
2. **Navigate to category** - Click category tab or scroll
3. **Modify setting** - Change value using controls (slider, text field, checkbox, etc.)
4. **Save changes** - Click "Save" button
5. **Verify** - Settings are applied immediately

### Settings Validation

Settings are validated when you save:
- **Invalid values** - Rejected with error message
- **Out of range** - Adjusted to valid range
- **Invalid paths** - Rejected, must select valid directory
- **Corrupted data** - Defaults used, settings file recreated

### Auto-Save

**Note:** Auto-save on change is planned for a future version. Currently, you must click "Save" to persist changes.

### Reset to Defaults

1. Open Settings panel
2. Click **"Reset to Defaults"** button
3. Confirm the action
4. All settings revert to default values
5. Click **"Save"** to apply defaults

**Warning:** Resetting to defaults cannot be undone. Your custom settings will be lost.

---

## Settings File Location

### Platform-Specific Paths

Settings are stored in platform-specific application data directories:

**Windows 11:**
```
%APPDATA%\SimpleImageConverter\config.toml
```
Example: `C:\Users\YourName\AppData\Roaming\SimpleImageConverter\config.toml`

**macOS:**
```
~/Library/Application Support/SimpleImageConverter/config.toml
```

**Linux (Ubuntu 24.04+):**
```
~/.config/simpleimageconverter/config.toml
```

### File Format

Settings are stored in **TOML format** (Tom's Obvious Minimal Language):
- Human-readable text format
- Can be edited manually (advanced users)
- Automatically created on first save

### File Permissions

Settings file permissions:
- **Owner:** Read/write (you)
- **Others:** Read-only (security)
- **Location:** User-specific directory (not system-wide)

### Manual Editing

**Advanced users only:** You can edit the settings file directly:
1. Close the application
2. Open `config.toml` in a text editor
3. Make changes (follow TOML syntax)
4. Save the file
5. Launch application (settings validated on load)

**Warning:** Invalid TOML syntax will cause settings to reset to defaults.

---

## Settings Persistence

### Automatic Saving

Settings are saved:
- When you click "Save" in Settings panel
- On application exit (if changes were made)
- When settings are reset to defaults

### Automatic Loading

Settings are loaded:
- On application startup
- Before showing main window
- If file missing: Defaults used, file created on first save

### Settings Migration

**Note:** Settings migration for version upgrades is planned for future versions. Currently, settings are reset when upgrading (this will be improved).

### Backup and Restore

**Manual backup:**
1. Close the application
2. Copy `config.toml` to a backup location
3. To restore: Replace `config.toml` with backup copy

**Automatic backup (future):**
- Settings file automatically backed up before major changes
- Backup files stored in same directory with `.bak` extension

---

## Troubleshooting

### Common Issues

**Issue: "Settings not saving"**
- **Cause:** No write permission to settings directory
- **Solution:** Check directory permissions, run as administrator if needed

**Issue: "Settings reset to defaults"**
- **Cause:** Settings file corrupted or invalid
- **Solution:** Settings automatically reset, file recreated on next save

**Issue: "Can't find settings file"**
- **Cause:** Settings file not created yet (first launch)
- **Solution:** Save settings once to create file, or check platform-specific path

**Issue: "Invalid settings values"**
- **Cause:** Manual editing introduced syntax errors
- **Solution:** Fix TOML syntax, or delete file to reset to defaults

**Issue: "Settings file location unknown"**
- **Cause:** Platform detection failed
- **Solution:** Check application logs, verify platform support

### Settings File Corruption

If settings file becomes corrupted:
1. Application detects corruption on load
2. Defaults are used automatically
3. Corrupted file is renamed (`.corrupted` extension)
4. New settings file created on next save
5. You can manually delete corrupted file if desired

### Resetting Settings

**Method 1: Reset in UI**
1. Open Settings panel
2. Click "Reset to Defaults"
3. Click "Save"

**Method 2: Delete Settings File**
1. Close the application
2. Delete `config.toml` from settings directory
3. Launch application (defaults used, new file created on save)

**Method 3: Manual Edit**
1. Close the application
2. Edit `config.toml` manually
3. Fix invalid values or syntax
4. Launch application

### Getting Help

If you encounter issues not covered here:

1. Check settings file location (platform-specific)
2. Verify file permissions (read/write)
3. Check for syntax errors (if manually edited)
4. Try resetting to defaults
5. Delete settings file and start fresh
6. Report issues through the project repository

---

## Technical Details

### Settings Structure

Settings are stored as a structured TOML file:

```toml
[general]
default_output_directory = "/path/to/directory"
show_advanced_options = false
recent_files_max = 10

[conversion]
default_quality = 90
max_file_size_mb = 100
max_image_dimension = 65535
max_vertices = 10000000
max_faces = 10000000

[history]
enabled = true
max_entries = 50
```

### Settings Validation

All settings are validated:
- **Type checking** - Values must match expected types
- **Range validation** - Numeric values within valid ranges
- **Path validation** - Paths must exist and be accessible
- **Security checks** - Paths validated for security (no traversal)

### Settings Loading

Settings loading process:
1. Check if settings file exists
2. If exists: Read and parse TOML file
3. Validate all settings values
4. If invalid: Use defaults, mark file as corrupted
5. If missing: Use defaults, create file on first save

### Settings Saving

Settings saving process:
1. Validate all current settings values
2. Convert to TOML format
3. Write to settings file (atomic write if possible)
4. Set appropriate file permissions
5. Handle errors gracefully (show message to user)

---

**For more information, see:**
- [GUI Usage Guide](GUI_USAGE_GUIDE.md) - General GUI documentation
- [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md) - Batch conversion guide
- [README.md](../README.md) - Project overview

