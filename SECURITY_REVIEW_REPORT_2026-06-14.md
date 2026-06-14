# Security Review Report

**Project:** SimpleImageConverter  
**Generated:** June 14, 2026  
**Reviewer role:** Security Specialist  
**Audience:** System Architect, Senior Engineer  
**Scope:** Static review of the Rust workspace, format parsers, CLI/GUI file handling, dependency configuration, CI security checks, and release/supply-chain surfaces.

## Executive Summary

No direct remote-code-execution issue or production `unsafe` block was found in the reviewed Rust code. The dominant risk is denial of service from untrusted image/mesh files: several paths check file byte size but allocate or parse attacker-controlled decoded content before applying dimension, pixel, vertex, or face limits.

The live dependency audit found two current vulnerabilities in `Cargo.lock`:

- `RUSTSEC-2026-0007` / GHSA-434x-w66g-qw3r: `bytes 1.11.0`, integer overflow in `BytesMut::reserve`; fix is `bytes >= 1.11.1`.
- `RUSTSEC-2026-0041`: `lz4_flex 0.7.5`, high severity information leak from invalid compressed data; fix is `lz4_flex >= 0.11.6, <0.12.0` or `>=0.12.1`.

`cargo audit` completed and failed because of those vulnerabilities. `cargo deny` could not be run locally because `cargo-deny` is not installed in this environment. CI installs `cargo-deny`, but currently runs only `cargo deny check advisories`, so license, duplicate-version, ban, and source policy checks are configured but not enforced in CI.

## Threat Model

SimpleImageConverter is a local desktop and CLI tool, but every input file, dropped file, batch queue item, command-line path, and persisted setting must be treated as attacker-controlled. Practical attacks include memory exhaustion, CPU exhaustion, output overwrite, path traversal or writes into sensitive locations, dependency-level parser vulnerabilities, and sensitive local path disclosure through logs or UI messages.

## High Priority Findings

### H1. Active dependency vulnerabilities in the current lockfile

**Risk:** High  
**Attack vector:** Vulnerable transitive crates can be reached through GUI, image, CAD, or mesh dependency trees depending on enabled features and platform build.  
**Evidence:** `cargo audit` reported `bytes 1.11.0` and high-severity `lz4_flex 0.7.5`.

Affected paths from the audit:

- `bytes 1.11.0` via `jni -> webbrowser -> egui-winit -> eframe -> converter-gui` and related GUI paths.
- `lz4_flex 0.7.5` via `vtkio -> truck-meshalgo -> mesh-core`.

**Recommended fix:**

- Run `cargo update -p bytes --precise 1.11.1` or update the dependency chain that brings it in.
- Update or feature-gate the `truck-meshalgo` / `vtkio` path so `lz4_flex` resolves to a fixed version.
- Add a release-blocking CI job for `cargo audit` on default features and each shipped optional feature set.

### H2. Image readers allocate decoded images before enforcing dimension or pixel-budget limits

**Risk:** High  
**Attack vector:** A small PNG/JPEG/BMP/GIF/TIFF/WebP file can declare or decompress into very large dimensions. The readers check compressed file size, call `image::load_from_memory_with_format`, then extract raw pixel buffers. A file under the 100 MB default file cap can still expand to multi-GB decoded memory.

Example evidence:

- `img-core/src/formats/png.rs:25-77`: checks `data.len()`, then calls `image::load_from_memory_with_format`, then converts to raw buffers.
- `img-core/src/validation.rs:20-73`: has checked dimension and byte-length validation, but this validation is applied primarily to writer inputs, not before reader allocations.
- Similar reader pattern appears in `jpg.rs`, `bmp.rs`, `gif.rs`, `tiff.rs`, and `webp.rs`.

**Recommended fix:**

- Add image-reader `with_limits` support and thread caller-provided `ResourceLimits` through `img-core::FormatRegistry` and `ImageConverter`.
- Pre-scan container headers where practical before full decode.
- After decode and before `to_rgba8()` / `into_raw()`, call `validate_image_data_with_limits` or a new decoded-byte budget check using checked arithmetic.
- Add a hard decoded-byte cap, not only width/height caps.

### H3. CLI image limits are built but not honored by image readers

**Risk:** High  
**Attack vector:** `img-convert` accepts `--max-file-size-mb` and `--max-dimension`, but then uses readers returned by `FormatRegistry::get_reader`, whose implementations construct `ResourceLimits::default()` internally. User-tightened limits do not control decode behavior.

Evidence:

- `img-convert/src/main.rs:48-52`: builds custom limits from CLI args.
- `img-convert/src/main.rs:83-90`: calls `FormatRegistry::get_reader(input_format)` and converts with that reader.
- `img-core/src/formats/png.rs:27-30`: constructs `ResourceLimits::default()` inside the reader.

**Recommended fix:**

- Mirror the mesh path: implement `FormatRegistry::get_reader_with_limits` for image formats.
- Pass CLI and GUI limits into image readers.
- Add regression tests proving a custom low `max_dimension` rejects a malicious image before large allocation.

### H4. SVG rasterization has no output dimension or decoded-size cap

**Risk:** High  
**Attack vector:** An SVG under the file-size cap can declare enormous output dimensions. The parser then creates a `Pixmap` and copies pixmap bytes to an RGBA image without checking dimensions or total pixels.

Evidence:

- `img-core/src/formats/svg.rs:33-40`: checks file byte size only.
- `img-core/src/formats/svg.rs:51-77`: derives size from the SVG tree, calls `Pixmap::new`, then copies `data.to_vec()`.

**Recommended fix:**

- Validate `pixmap_size.width()` and `pixmap_size.height()` against `ResourceLimits`.
- Add checked multiplication for `width * height * 4` and enforce a decoded-byte ceiling before `Pixmap::new`.
- Review `usvg` external resource/entity behavior and add explicit tests for huge dimensions and entity expansion.

### H5. Mesh parsers often validate resource counts after third-party parse

**Risk:** High  
**Attack vector:** Malicious STL/OBJ/PLY/glTF/DXF input can force third-party parsers to allocate or spend CPU before project limits are checked.

Evidence:

- `mesh-core/src/formats/stl.rs:35-56`: calls `stl_io::read_stl`, then checks mesh resources.
- `mesh-core/src/formats/ply.rs:227-229`: checks final mesh resources after extracting and triangulating payload data.
- `mesh-core/src/formats/off.rs:85-86` is better for declared counts, but still materializes all lines and can inflate faces during triangulation.

**Recommended fix:**

- Pre-parse declared counts before third-party parse where format headers allow it.
- For binary STL, validate the triangle count at byte offset 80 and verify expected file length before `stl_io`.
- For PLY/OFF, enforce both declared count limits and incremental post-triangulation face limits.
- For glTF, pre-check accessor counts and buffer sizes before constructing the full mesh.

### H6. Polygon triangulation can inflate face count beyond limits before rejection

**Risk:** High  
**Attack vector:** A file can declare an acceptable number of faces but use very large polygons. Fan triangulation creates `n - 2` triangles per polygon, causing memory growth before final validation.

Evidence:

- `mesh-core/src/formats/off.rs:150-201`: parses arbitrary `num_face_vertices` and pushes triangles without checking `max_faces` during the loop.
- `mesh-core/src/formats/ply.rs:176-188`: triangulates variable-length face lists and checks `max_faces` only after the full mesh is built.

**Recommended fix:**

- Set a maximum vertices-per-polygon value.
- Before pushing each generated triangle, check whether `mesh.faces.len() + 1` exceeds `limits.max_faces`.
- Add tests for a single large polygon and many near-limit polygons.

### H7. CLI output paths are not validated before writes

**Risk:** High  
**Attack vector:** A user or wrapper script can pass an output path that overwrites any writable file. There is no common output-path validation, system-directory rejection, duplicate path handling, atomic write, or overwrite confirmation in CLI code.

Evidence:

- `img-convert/src/main.rs:67-93`: constructs `output_path` directly from `--output` or input path and calls `write_file_bytes`.
- `mesh-convert/src/main.rs:69-132`: same pattern.
- `common/src/io.rs:51-54`: `write_file_bytes` is a direct `fs::write`, which overwrites existing files.

**Recommended fix:**

- Move GUI output validation into `common`.
- Canonicalize and validate the output parent.
- Reject system directories and suspicious filenames.
- Require `--force` for overwrite.
- Write to a temp file in the target directory and atomically rename.

### H8. GUI 3D viewer and preview paths read full files before size checks

**Risk:** High  
**Attack vector:** Selecting a very large mesh for preview can allocate the entire file before `ResourceLimits` are applied.

Evidence:

- `converter-gui/src/app.rs:944-949`: viewer path uses `std::fs::read(source_file)` directly.
- `converter-gui/src/ui/preview.rs:423-427`: reads with `std::fs::read(mesh_path)`, then checks length after the allocation.

**Recommended fix:**

- Replace these reads with `common::io::read_file_bytes_checked`.
- Thread the current GUI resource settings into viewer/preview loading.
- Add a regression test for a preview file exceeding `max_file_size`.

## Medium Priority Findings

### M1. CLI resource limit arguments are unbounded and multiplication can overflow

**Risk:** Medium  
**Attack vector:** `--max-file-size-mb`, `--max-dimension`, `--max-vertices`, and `--max-faces` accept extremely large values. `ResourceLimitsBuilder::max_file_size_mb` multiplies `mb * 1024 * 1024` without checked arithmetic.

Evidence:

- `common/src/limits.rs:167-168`: unchecked multiplication.
- `img-convert/src/main.rs:48-52` and `mesh-convert/src/main.rs:55-60`: accept CLI values directly.

**Recommended fix:** Clamp CLI values to documented maxima, use checked multiplication, and require an explicit `--trusted` or `--unsafe-large-limits` mode for unusually high values.

### M2. System-directory protection is incomplete and GUI-only

**Risk:** Medium  
**Attack vector:** Current system path blocking is denylist-based, Windows `C:` centric, incomplete for macOS, and not reused by the CLIs.

Evidence:

- `converter-gui/src/utils.rs:163-181`: canonicalizes target or parent and falls back to string matching.
- `converter-gui/src/utils.rs:198-207`: Windows list is hardcoded to `c:\...`.
- `converter-gui/src/utils.rs:217-269`: Unix list does not distinguish macOS system paths such as `/System`, `/Library`, and `/Applications`.

**Recommended fix:** Prefer output-root allowlisting over system-directory denylisting. At minimum, move validation to `common`, canonicalize the parent before writes, add macOS paths, and test junction/symlink behavior.

### M3. Extension-only detection remains possible for formats without signatures

**Risk:** Medium  
**Attack vector:** Formats such as OBJ, STL, DXF, and STEP may lack strong magic-byte signatures. Extension-only detection can steer ambiguous content into a parser that was not intended for it.

**Recommended fix:** Add lightweight content heuristics for unsigned formats, require parser self-validation, and log extension/content mismatches consistently across image and mesh paths.

### M4. Full-file materialization and O(n^2) validation can cause CPU/memory DoS

**Risk:** Medium  
**Attack vector:** OFF parsing creates a `Vec<&str>` for all non-empty lines, and mesh validation duplicate-vertex checks can become quadratic at high vertex counts.

Evidence:

- `mesh-core/src/formats/off.rs:36-44`: converts the entire file to UTF-8 text and collects all lines.
- `mesh-core/src/mesh/validate.rs` was identified as using a duplicate-vertex scan with high worst-case cost.

**Recommended fix:** Stream parsers where practical and replace duplicate scans with hashing/spatial indexing or disable expensive validation by default for untrusted large inputs.

### M5. Error output can disclose local paths and parser internals

**Risk:** Medium  
**Attack vector:** Raw parser errors and full success paths can appear in CLI output, logs, or GUI messages.

Evidence:

- `img-core/src/formats/png.rs:35-40`: includes byte length and dependency error string.
- `img-convert/src/main.rs:107-110` and `mesh-convert/src/main.rs:143-146`: print full input and output paths.

**Recommended fix:** Use sanitized user-facing messages by default and gate detailed diagnostics behind `--verbose` or debug logs.

### M6. CI dependency policy is configured but only partially enforced

**Risk:** Medium  
**Attack vector:** `deny.toml` defines license, ban, duplicate-version, and source policies, but CI runs only advisory checks.

Evidence:

- `.github/workflows/ci.yml:68-71`: runs `cargo audit` and `cargo deny check advisories`.
- `deny.toml:21-62`: defines `[licenses]`, `[bans]`, and `[sources]` policies.

**Recommended fix:** Change CI to `cargo deny check advisories licenses bans sources`. Add per-feature checks for shipped optional stacks.

### M7. Optional CAD/STEP dependency surface needs release-specific security and license treatment

**Risk:** Medium  
**Attack vector:** CAD dependencies pull older and native-heavy parser stacks, including the vulnerable `lz4_flex` path reported by `cargo audit`. OCCT paths also have license and native build-chain implications.

**Recommended fix:** Treat STEP/OCCT builds as separate release profiles. Audit default, `step`, `step-opencascade`, and `viewer-3d` independently, and document which features are shipped in each binary.

## Low Priority Findings

### L1. `read_file_bytes` remains public

`common/src/io.rs:9-15` documents that `read_file_bytes_checked` is preferred for untrusted input, but the unchecked helper remains public. Consider deprecating it, making it crate-private, or adding lint/test coverage for production use.

### L2. Writes are non-atomic

`common/src/io.rs:51-54` uses `fs::write`. A crash can leave partial or corrupt output. Prefer temp-file writes followed by atomic rename.

### L3. Release artifacts have checksums but no signing path confirmed in this review

Checksums help integrity, but distribution should add Authenticode for Windows, notarization/signing for macOS, and/or Sigstore attestations where practical.

### L4. Advisory ignore list needs maintenance

`deny.toml` ignores several unmaintained transitive crates. The live audit also reported `proc-macro-error`, `core2`, and `rand` advisories/warnings. Track ignores with expiry dates and upgrade paths.

## Positive Controls

- No production `unsafe` was found by static search.
- `ResourceLimits` exists centrally and is already used in many file-size and mesh-count paths.
- `validate_image_data_with_limits` uses checked arithmetic for expected image buffer length.
- The project uses `ply-rs-bw`, a patched fork intended to avoid CVE-2020-25573 in the original `ply-rs` dependency path.
- GUI conversion paths have more validation than the CLIs and can be used as the starting point for shared validation utilities.
- CI already has a security job that installs `cargo-audit`, `cargo-deny`, and attempts `cargo-geiger`.

## Recommended Remediation Plan

### Phase 1: Release blockers

1. Fix `cargo audit` failures for `bytes` and `lz4_flex`.
2. Add decoded image byte limits and SVG raster dimension checks.
3. Replace GUI viewer/preview raw reads with `read_file_bytes_checked`.
4. Add CLI output-path validation and overwrite protection.

### Phase 2: Parser hardening

1. Add image reader `with_limits` support and wire CLI/GUI limits through all readers.
2. Pre-parse mesh declared counts before third-party parser allocation where feasible.
3. Add incremental `max_faces` checks during OFF and PLY triangulation.
4. Add dimension bomb and triangulation blow-up tests.

### Phase 3: Supply-chain and policy cleanup

1. Run full `cargo deny check advisories licenses bans sources` in CI.
2. Add feature-specific audit jobs for `step`, `step-opencascade`, and `viewer-3d`.
3. Update dependency/license documentation to match the current lockfile.
4. Add fuzz coverage for SVG, TIFF, PLY, OBJ, OFF, and glTF.

## Verification Performed

- Static search for `unsafe`, `unwrap`, `expect`, `panic`, file reads/writes, command execution, and resource-limit usage.
- Targeted review of parser, CLI, GUI, validation, dependency, and CI files.
- `cargo audit` was run locally and failed with two vulnerabilities plus warnings.
- `cargo deny check advisories licenses bans sources` was attempted locally but could not run because `cargo-deny` is not installed.

## Suggested Owner Split

- **Senior Engineer:** H2-H8, M1-M5 parser and app hardening.
- **System Architect:** H1, M6, M7 dependency policy, release profile, and validation API ownership.
- **Security Specialist:** Review fixes for decoded-size caps, output-path policy, advisory ignores, and feature-specific audit results before release.
