# Batch Processing Guide
## Simple Image Converter - Convert Multiple Files at Once

**Version:** 0.3.0  
**Last Updated:** December 30, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Getting Started](#getting-started)
3. [Adding Files to Queue](#adding-files-to-queue)
4. [Processing the Queue](#processing-the-queue)
5. [Queue Management](#queue-management)
6. [Error Handling](#error-handling)
7. [Tips and Best Practices](#tips-and-best-practices)
8. [Troubleshooting](#troubleshooting)

---

## Overview

Batch processing allows you to convert multiple files at once, saving time and effort. Instead of converting files one by one, you can add multiple files to a queue and process them all in a single operation.

### Key Features

- **Multiple file selection** - Add many files at once
- **Queue management** - Add, remove, and reorder files
- **Progress tracking** - See progress for each file individually
- **Error resilience** - Failed conversions don't stop the queue
- **Statistics** - Track total, completed, and failed conversions

### When to Use Batch Processing

Batch processing is ideal for:
- Converting multiple images to the same format (e.g., PNG → JPEG)
- Converting a folder of mesh files (e.g., STL → OBJ)
- Applying the same settings to many files
- Processing large collections of files

---

## Getting Started

### Accessing Batch Processing

1. Launch the GUI application (`converter-gui` or `converter-gui.exe`)
2. Look for the **"Batch Processing Queue"** panel in the main window
3. The queue panel shows:
   - Current queue items
   - Queue statistics (total, completed, failed)
   - Action buttons (Add Files, Clear Queue, Process)

### First-Time Setup

When you first use batch processing:
- The queue is empty
- All buttons are enabled
- Statistics show "Total: 0 | Completed: 0 | Failed: 0"

---

## Adding Files to Queue

### Method 1: Add Files Button

1. Click the **"Add Files"** button in the batch queue panel
2. In the file dialog, select one or more files:
   - **Windows/Linux:** Hold `Ctrl` to select multiple files, or `Shift` to select a range
   - **macOS:** Hold `Cmd` to select multiple files, or `Shift` to select a range
3. Click **"Open"**
4. Files are added to the queue with default settings

### Method 2: Drag and Drop Multiple Files

1. Open your file manager
2. Select multiple files (use `Ctrl`/`Cmd` or `Shift` to select)
3. Drag the selected files into the batch queue panel
4. Release the mouse button
5. Files are added to the queue

### Method 3: Add from Single File Conversion

1. Select a file in the main drop zone (as you would for single conversion)
2. Configure format and options
3. Click **"Add to Queue"** button (instead of "Convert")
4. The file is added to the queue with your configured settings

### Queue Item Details

Each item in the queue shows:
- **File icon** - Visual indicator of file type
- **Source file name** - The input file
- **Output format** - Target format (e.g., "→ JPEG")
- **Status** - Current processing status:
  - **Pending** - Waiting to be processed
  - **Processing** - Currently converting (shows progress %)
  - **Completed** - Successfully converted
  - **Failed** - Conversion failed (error shown)
  - **Cancelled** - Manually cancelled

---

## Processing the Queue

### Starting Batch Processing

1. Ensure your queue has files (at least one item)
2. Review queue items and settings
3. Click the **"Process Queue"** button
4. Processing begins automatically

### During Processing

- **Status updates** - Each item's status updates in real-time
- **Progress indicators** - Processing items show progress percentage
- **Queue statistics** - Updated as items complete or fail
- **UI remains responsive** - You can still interact with the application

### Processing Behavior

- **Parallel processing (v0.3.0)** - Multiple files are processed simultaneously
  - Files are processed concurrently using a thread pool
  - Default concurrency: Number of CPU cores (capped at 8)
  - Configurable via Settings → Conversion → Max Concurrent Conversions
  - **Performance:** Up to 4x faster on 4-core systems compared to sequential processing
- **Error handling** - Failed conversions don't stop the queue
- **Continue on error** - Processing continues even if some files fail
- **Automatic progression** - New items start automatically as slots become available

### Pause, Resume, and Cancel (v0.3.0)

**Status:** Backend implementation complete, UI controls in progress (Sprint 10)

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
- You can clear cancelled items or restart processing

**Visual Feedback:**
- Pause button shows when processing is active
- Resume button shows when processing is paused
- Cancel button is always available during processing
- Status indicators show current state (Processing, Paused, Cancelled)

**Use Cases:**
- **Pause:** Temporarily stop to free up system resources
- **Resume:** Continue processing after pause
- **Cancel:** Stop processing entirely (e.g., wrong files in queue)

**Note:** These controls are currently being implemented in Sprint 10. Backend functionality is complete and ready for UI integration.

### Completion

When all items are processed:
- Status bar shows "Batch processing complete"
- Statistics show final counts (Total, Completed, Failed)
- Success message appears in messages area
- You can review results for each item

---

## Queue Management

### Removing Individual Items

1. Find the item you want to remove in the queue
2. Click the **"Remove"** button next to that item
3. Item is removed from the queue immediately
4. Queue statistics update automatically

**Note:** You can remove items even while processing is running (pending items only).

### Clearing the Entire Queue

1. Click the **"Clear Queue"** button
2. Confirm the action (if prompted)
3. All items are removed from the queue
4. Statistics reset to zero

**Warning:** Clearing the queue during processing will cancel pending items but won't stop the currently processing item.

### Reordering Items

**Note:** Reordering is planned for a future version. Currently, items are processed in the order they were added.

### Editing Queue Items (v0.3.0)

**Edit queue items before processing** to fix mistakes or adjust settings:

1. **Open edit dialog** - Click the "Edit" button on any pending queue item
2. **Edit fields** - Change:
   - **Output format** - Select a different format from the dropdown
   - **Output path** - Change the output file location (click "Browse..." to select)
   - **Quality** - Adjust quality for lossy image formats (JPEG, WebP) using the slider
   - **Mesh options** - Adjust mesh conversion options (if applicable)
3. **Validate changes** - The dialog validates your changes:
   - Format compatibility is checked
   - Output path is validated (must be a valid, writable location)
   - Invalid values are rejected with error messages
4. **Save changes** - Click "Save" to update the queue item
5. **Cancel** - Click "Cancel" or close the dialog to discard changes

**Restrictions:**
- **Pending items only** - You can only edit items with "Pending" status
- **Processing items** - Cannot edit items currently being processed
- **Completed/Failed items** - Cannot edit items that are already completed or failed

**Benefits:**
- Fix mistakes without removing and re-adding items
- Adjust settings for individual items (e.g., different quality for each image)
- More flexible batch processing workflow
- Save time when you need to make corrections

---

## Error Handling

### Understanding Queue Errors

When a file fails to convert:
- The item status changes to **"Failed"**
- An error message is displayed for that item
- Processing continues with the next item
- The failed item remains in the queue for review

### Common Error Scenarios

**File Not Found:**
- **Cause:** File was moved or deleted after adding to queue
- **Solution:** Remove the item or fix the file path

**Format Not Supported:**
- **Cause:** Output format not compatible with source file
- **Solution:** Remove item and add with correct format

**File Too Large:**
- **Cause:** File exceeds resource limits
- **Solution:** Adjust resource limits in Advanced Options, or skip this file

**Permission Denied:**
- **Cause:** No write permission for output location
- **Solution:** Change output location or fix permissions

### Handling Failed Items

1. **Review error message** - Check the error shown for the failed item
2. **Fix the issue** - Address the problem (move file, change format, etc.)
3. **Remove and re-add** - Remove failed item, fix settings, add again
4. **Or skip** - Simply remove the item if not needed

### Queue Statistics

The queue panel shows:
- **Total:** Total number of items in queue
- **Completed:** Successfully converted items
- **Failed:** Items that failed conversion

Use these statistics to track batch processing progress.

---

## Tips and Best Practices

### Organizing Your Queue

1. **Group similar files** - Add files of the same type together
2. **Check settings first** - Verify format and options before adding many files
3. **Test with one file** - Convert one file first to verify settings
4. **Review before processing** - Check queue items before starting batch

### Performance Tips

- **Large files** - Batch processing large files may take time; be patient
- **Many files** - Processing 100+ files may take 30+ minutes
- **Progress tracking** - Monitor progress for long-running batches
- **Resource usage** - Large batches may use significant memory

### Best Practices

1. **Backup important files** - Always backup before batch conversion
2. **Test settings** - Convert one file first to verify output quality
3. **Monitor progress** - Keep an eye on processing status
4. **Check results** - Verify output files after batch completes
5. **Handle errors promptly** - Review and fix failed items

### Format Selection Tips

- **Consistent formats** - Use the same output format for all files in a batch
- **Quality settings** - JPEG/WebP quality applies to all lossy conversions
- **Output location** - All files save to the same output directory (unless changed per-item)

---

## Troubleshooting

### Common Issues

**Issue: "Queue processing stuck"**
- **Cause:** Large file or complex format taking time
- **Solution:** Wait for completion (check progress %), or cancel and retry

**Issue: "All files failing"**
- **Cause:** Incorrect format selection or settings
- **Solution:** Check format compatibility, verify settings, test with one file

**Issue: "Queue not processing"**
- **Cause:** Queue empty or all items already processed
- **Solution:** Add new files to queue, or clear and start fresh

**Issue: "Memory errors during batch"**
- **Cause:** Too many large files processed at once
- **Solution:** Process in smaller batches, or increase system memory

**Issue: "Files missing after batch"**
- **Cause:** Output location changed or files saved elsewhere
- **Solution:** Check output directory, review conversion history

### Getting Help

If you encounter issues not covered here:

1. Check error messages for each failed item
2. Review queue statistics to identify patterns
3. Test with a single file to isolate the issue
4. Check file formats and compatibility
5. Verify output location and permissions
6. Report issues through the project repository

---

## Parallel Processing (v0.3.0)

### Overview

Parallel batch processing allows multiple files to be converted simultaneously, significantly improving performance for large batch queues. Instead of processing files one at a time, the application uses a thread pool to process multiple files concurrently.

### How It Works

1. **Thread Pool** - Uses `rayon` library for efficient parallel processing
2. **Concurrency Control** - Limits concurrent conversions to prevent resource exhaustion
3. **Automatic Load Balancing** - Work-stealing scheduler distributes work evenly across threads
4. **Thread-Safe Queue** - All queue operations are thread-safe using `Arc<Mutex<>>`

### Configuration

**Max Concurrent Conversions Setting:**
- **Location:** Settings → Conversion → Max Concurrent Conversions
- **Default:** Number of CPU cores (capped at 8)
- **Range:** 1-16 concurrent conversions
- **Recommendation:** 
  - **4-core system:** 4 concurrent conversions (optimal)
  - **8-core system:** 8 concurrent conversions (optimal)
  - **Lower-end systems:** 2-4 concurrent conversions (to reduce memory usage)

**How to Configure:**
1. Open Settings (menu bar → Settings)
2. Navigate to "Conversion" section
3. Find "Max Concurrent Conversions" setting
4. Adjust slider or enter value (1-16)
5. Settings auto-save after 500ms

### Performance Benefits

**Speedup Examples:**
- **10 files, 2 seconds each:**
  - Sequential: 20 seconds total
  - Parallel (4 cores): ~5 seconds total (**4x speedup**)
- **100 files, 1 second each:**
  - Sequential: 100 seconds total
  - Parallel (4 cores): ~25 seconds total (**4x speedup**)

**Factors Affecting Performance:**
- **CPU cores:** More cores = better parallel performance
- **File size:** Larger files may benefit more from parallel processing
- **File type:** CPU-bound conversions (images, meshes) benefit most
- **Memory:** Each concurrent conversion uses memory; adjust concurrency if memory-constrained

### Thread Safety and Resource Limits

**Thread Safety:**
- All queue operations are thread-safe
- Status updates are synchronized
- Progress tracking works correctly with parallel processing
- No data races or race conditions

**Resource Limits:**
- **Memory:** Each concurrent conversion loads a file into memory
  - Estimate: ~3x file size for images, ~2x for meshes
  - Example: 4 concurrent 10MB images = ~120MB memory usage
- **CPU:** Optimal concurrency = number of CPU cores
- **Disk I/O:** Multiple file writes may impact performance on slower drives

**Resource Management:**
- Concurrency limits prevent excessive memory usage
- Default cap at 8 prevents system overload
- Adjust concurrency based on your system's capabilities

### Progress Tracking

**Per-Item Progress:**
- Each item shows individual progress (0-100%)
- Progress updates in real-time during conversion
- Multiple items can show progress simultaneously

**Overall Progress:**
- Overall queue progress shown in status bar
- Calculated as: (Completed + Failed) / Total
- Updates as items complete

**Example Display:**
- "Processing: 5/10 (50%)" - 5 items completed out of 10 total
- Individual items show: "Processing (45%)" - 45% complete

### Troubleshooting Parallel Processing

**Issue: "High memory usage during batch processing"**
- **Cause:** Too many concurrent conversions
- **Solution:** Reduce "Max Concurrent Conversions" in Settings (try 2-4)

**Issue: "CPU usage at 100%"**
- **Cause:** Normal behavior for parallel processing
- **Solution:** This is expected; parallel processing uses all CPU cores for maximum speed

**Issue: "Some files failing during parallel processing"**
- **Cause:** Resource exhaustion or file-specific issues
- **Solution:** 
  - Check individual error messages for failed items
  - Reduce concurrency if memory-constrained
  - Verify file integrity and format compatibility

**Issue: "Slower than expected"**
- **Cause:** I/O bottleneck (slow disk) or low concurrency setting
- **Solution:** 
  - Increase "Max Concurrent Conversions" if CPU usage is low
  - Check disk I/O performance
  - Verify files are on fast storage (SSD recommended)

## Technical Details

### Processing Architecture

- **Parallel processing** - Multiple files processed simultaneously using thread pool
- **Thread-safe** - Queue updates are thread-safe using `Arc<Mutex<BatchQueue>>`
- **Error isolation** - Each conversion is independent (failures don't affect others)
- **Progress tracking** - Real-time status updates for all concurrent operations
- **Resource management** - Configurable concurrency limits prevent resource exhaustion

### Resource Limits

Batch processing respects the same resource limits as single conversions:
- **Max file size:** 100 MB (default, configurable)
- **Max dimensions:** 65535 pixels (images)
- **Max vertices/faces:** 10,000,000 (meshes)

These limits apply to each file individually.

### Performance Considerations

- **Processing time** - Significantly reduced with parallel processing (4x speedup on 4-core systems)
- **Memory usage** - Each concurrent conversion loads a file into memory (adjust concurrency if memory-constrained)
- **Disk I/O** - Multiple file writes may impact performance on slower drives (SSD recommended)
- **UI responsiveness** - UI remains responsive during batch processing (conversions run in background threads)
- **CPU usage** - Parallel processing uses all CPU cores (100% CPU usage is normal and expected)

---

**For more information, see:**
- [GUI Usage Guide](GUI_USAGE_GUIDE.md) - General GUI documentation
- [Settings Guide](SETTINGS_GUIDE.md) - Configuration options
- [README.md](../README.md) - Project overview

