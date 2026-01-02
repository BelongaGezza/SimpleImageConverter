# Performance Guide
## Simple Image Converter - Performance Characteristics and Tuning

**Version:** 0.3.0  
**Last Updated:** December 30, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Parallel Batch Processing Performance](#parallel-batch-processing-performance)
3. [Single File Conversion Performance](#single-file-conversion-performance)
4. [Memory Usage](#memory-usage)
5. [3D Viewer Performance](#3d-viewer-performance)
6. [Performance Tuning](#performance-tuning)
7. [Benchmarking](#benchmarking)

---

## Overview

Simple Image Converter is designed for high-performance file conversion with efficient resource usage. This guide documents performance characteristics and provides tuning recommendations.

### Key Performance Features

- **Parallel batch processing** - Up to 4x faster on 4-core systems
- **Thread-safe operations** - Efficient concurrent processing
- **Memory-efficient** - Resource limits prevent memory exhaustion
- **Hardware-accelerated 3D viewer** - Smooth rendering for meshes up to 100k vertices

---

## Parallel Batch Processing Performance

### Performance Characteristics

**v0.3.0 Feature:** Parallel batch processing uses a thread pool (`rayon` library) to process multiple files simultaneously.

**Speedup Examples:**
- **4-core system:** Up to 4x faster than sequential processing
- **8-core system:** Up to 8x faster than sequential processing (with appropriate concurrency setting)

**Real-World Examples:**
- **10 files (2 seconds each):**
  - Sequential: 20 seconds
  - Parallel (4 cores): ~5 seconds
- **100 files (1 second each):**
  - Sequential: 100 seconds
  - Parallel (4 cores): ~25 seconds

### Configuration

**Default Concurrency:**
- Automatically set to number of CPU cores (capped at 8 for memory safety)
- Configurable in Settings → Conversion → Max Concurrent Conversions (range: 1-16)

**Recommended Settings:**
- **High-end systems (8+ cores, 16GB+ RAM):** 8 concurrent conversions
- **Mid-range systems (4 cores, 8GB RAM):** 4 concurrent conversions
- **Low-end systems (2 cores, 4GB RAM):** 2 concurrent conversions
- **Memory-constrained:** Reduce concurrency if experiencing high memory usage

### Performance Factors

**Factors that affect parallel processing performance:**
1. **CPU cores** - More cores enable more parallel operations
2. **File size** - Larger files take longer per conversion
3. **Format complexity** - Some formats (e.g., TIFF, glTF) are more CPU-intensive
4. **Memory availability** - Each concurrent conversion loads a file into memory
5. **Disk I/O speed** - SSD vs HDD affects read/write performance

### Thread Safety

All parallel operations are thread-safe:
- Queue management uses `Arc<Mutex<BatchQueue>>`
- Progress tracking is thread-safe
- Error isolation prevents cascading failures
- Resource limits apply per-file (not per-batch)

---

## Single File Conversion Performance

### Typical Performance

**Small files (< 10 MB):**
- Conversion time: < 1 second
- Memory usage: ~3x file size for images, ~2x for meshes

**Medium files (10-50 MB):**
- Conversion time: 1-5 seconds
- Memory usage: ~3x file size for images, ~2x for meshes

**Large files (> 50 MB):**
- Conversion time: 5-30 seconds (format-dependent)
- Memory usage: ~3x file size for images, ~2x for meshes

### Format-Specific Performance

**Fast formats (typically < 1 second for small files):**
- PNG, JPEG, BMP, GIF
- STL (binary), OBJ

**Moderate formats (1-5 seconds for small files):**
- TIFF, WebP
- PLY, OFF, DXF

**Slower formats (5+ seconds for small files):**
- SVG (rasterization can be slow)
- glTF (complex format with materials)
- STEP (FACETED_BREP extraction)

### Performance Tips

1. **Use appropriate formats** - Choose formats that balance quality and performance
2. **Optimize file sizes** - Compress images before conversion if possible
3. **Batch processing** - Use parallel batch processing for multiple files
4. **Resource limits** - Adjust resource limits only if necessary (affects performance)

---

## Memory Usage

### Memory Characteristics

**Per-file memory usage:**
- **Images:** ~3x file size (read + decode + encode)
- **Meshes:** ~2x file size (read + parse + write)

**Parallel processing memory:**
- Each concurrent conversion loads a file into memory
- Example: 4 concurrent conversions of 10MB files = ~120MB memory (images) or ~80MB (meshes)

### Resource Limits

**Default limits (configurable in Settings):**
- **Max file size:** 100 MB
- **Max image dimension:** 65535 pixels
- **Max vertices/faces:** 10,000,000 each

**Memory safety:**
- Resource limits prevent memory exhaustion
- Limits apply per-file (not per-batch)
- Parallel processing respects individual file limits

### Memory Optimization Tips

1. **Reduce concurrency** - Lower concurrent conversions if memory is constrained
2. **Process in smaller batches** - Split large batches into smaller groups
3. **Close other applications** - Free up memory before large batch operations
4. **Monitor memory usage** - Use system tools to monitor memory during conversion

---

## 3D Viewer Performance

### Performance Characteristics

**Optimized for:**
- Meshes up to 100,000 vertices render smoothly
- Larger meshes may have reduced frame rates
- Performance depends on graphics hardware

**Rendering modes:**
- **Solid mode:** More GPU-intensive (lighting calculations)
- **Wireframe mode:** Faster rendering (edges only)

### Performance Targets

**Target performance:**
- **< 100k vertices:** Smooth rendering (60 FPS)
- **100k-500k vertices:** Acceptable performance (30+ FPS)
- **> 500k vertices:** May have reduced frame rates

### Performance Tips

1. **Use wireframe mode** - Switch to wireframe for better performance on large meshes
2. **Close other applications** - Free up GPU resources
3. **Update graphics drivers** - Ensure latest drivers for best performance
4. **Check wgpu compatibility** - Verify wgpu is available (most modern systems support it)

---

## Performance Tuning

### GUI Performance Optimizations

**v0.3.0 optimizations:**
- **Preview cache:** LRU (Least Recently Used) eviction for optimal hit rate
- **Batch queue rendering:** Virtual scrolling handles 1000+ items efficiently
- **Settings auto-save:** 500ms debounce prevents excessive disk I/O
- **UI updates:** egui framework automatically optimizes redraws

### System-Level Tuning

**Windows:**
- Disable Windows Defender real-time scanning for conversion directories (if safe)
- Use SSD for better I/O performance
- Close unnecessary background applications

**macOS:**
- Disable Spotlight indexing for conversion directories (if safe)
- Use SSD for better I/O performance
- Close unnecessary background applications

**Linux:**
- Use SSD for better I/O performance
- Consider using `ionice` to prioritize conversion processes
- Close unnecessary background applications

---

## Benchmarking

### Running Benchmarks

**Image conversion benchmarks:**
```bash
# Run image conversion benchmarks
cd img-core
cargo bench
```

**Benchmark results location:**
- Results are displayed in terminal
- Detailed results in `target/criterion/` directory

### Benchmark Coverage

**Current benchmarks (img-core):**
- PNG read/write performance
- JPEG read/write performance
- Format conversion performance (PNG → JPEG)
- Large image conversion performance

**Planned benchmarks (Sprint 11 - Task 2.1):**
- Parallel batch processing benchmarks
- Memory profiling for large batch operations
- 3D viewer rendering benchmarks
- End-to-end workflow benchmarks

### Performance Regression Testing

**Best practices:**
1. Run benchmarks before and after performance changes
2. Document baseline performance metrics
3. Monitor for performance regressions
4. Profile hot paths before optimizing

---

## Performance Monitoring

### Monitoring Tools

**System monitoring:**
- **Windows:** Task Manager, Resource Monitor
- **macOS:** Activity Monitor
- **Linux:** `htop`, `top`, `vmstat`

**Rust profiling:**
- **perf** (Linux): `perf record cargo run --bin converter-gui`
- **Instruments** (macOS): Built-in Xcode profiler
- **Visual Studio Profiler** (Windows): Built-in VS profiler

**egui profiling:**
- Enable egui's performance monitor in development builds
- Use `egui::profiler::Profiler` for UI performance metrics

### Key Metrics to Monitor

- **CPU usage:** Should scale with concurrent conversions
- **Memory usage:** Should be predictable based on file sizes and concurrency
- **Disk I/O:** Monitor read/write speeds
- **Frame time (3D viewer):** Target <16ms for 60 FPS

---

## Troubleshooting Performance Issues

### Slow Batch Processing

**Possible causes:**
- Low concurrency setting
- Large files taking long per conversion
- Disk I/O bottleneck
- Insufficient CPU cores

**Solutions:**
- Increase concurrent conversions (if memory allows)
- Process in smaller batches
- Use SSD for better I/O performance
- Close other CPU-intensive applications

### High Memory Usage

**Possible causes:**
- Too many concurrent conversions
- Very large files
- Memory leaks (should not occur)

**Solutions:**
- Reduce concurrent conversions
- Process files in smaller batches
- Increase system RAM if possible
- Monitor for memory leaks (should not occur)

### Slow 3D Viewer

**Possible causes:**
- Very large meshes (>100k vertices)
- Outdated graphics drivers
- Insufficient GPU resources

**Solutions:**
- Use wireframe mode for large meshes
- Update graphics drivers
- Close other GPU-intensive applications
- Check wgpu compatibility

---

## Future Performance Improvements

**Planned for future releases:**
- Streaming I/O for very large files
- Advanced memory pooling
- GPU-accelerated image processing (if beneficial)
- Additional performance benchmarks
- Performance profiling tools integration

---

## References

- [Batch Processing Guide](BATCH_PROCESSING_GUIDE.md) - Parallel processing usage
- [GUI Usage Guide](GUI_USAGE_GUIDE.md) - GUI performance tips
- [CHANGELOG.md](../CHANGELOG.md) - Performance improvements by version

---

**Note:** Formal performance benchmarks for parallel batch processing are planned for Sprint 11 (Task 2.1). This guide will be updated with benchmark results when available.

