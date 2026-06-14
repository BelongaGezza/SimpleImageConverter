# Security Remediation Architecture Decisions

**Created:** June 14, 2026  
**Owner:** Alex Chen, System Architect  
**Session:** Cursor subagent session 2026-06-14T18:47+01:00  
**Source Plan:** `AGENT_TASKS/SECURITY_REMEDIATION_PLAN_2026-06-14.md`  
**Source Review:** `SECURITY_REVIEW_REPORT_2026-06-14.md`  
**Status:** Architecture complete; pending Security Specialist V1 signoff

## Scope

This note defines the architecture contract for A1-A5. It does not implement production Rust code. The Senior Engineer owns implementation in `common`, `img-core`, `mesh-core`, CLI crates, and GUI crates. The Security Specialist owns signoff and residual-risk acceptance.

The governing principle remains: all file input is untrusted, including local files selected from GUI dialogs or supplied by command line.

## A1: Shared Resource Limit Contract

### Decision

`common::limits::ResourceLimits` remains the single source of truth for file, decoded image, and mesh expansion limits. All CLI, GUI, preview, image, and mesh reader paths must receive caller-provided limits rather than constructing local defaults when caller limits exist.

The contract must add these fields and constants:

- `DEFAULT_MAX_DECODED_IMAGE_BYTES: usize = 512 * 1024 * 1024`
- `DEFAULT_MAX_VERTICES_PER_POLYGON: usize = 64`
- `max_decoded_image_bytes: usize`
- `max_vertices_per_polygon: usize`

Existing defaults remain:

- `max_file_size = 100 MiB`
- `max_image_dimension = 65_535`
- `max_vertices = 10_000_000`
- `max_faces = 10_000_000`

The decoded-byte cap is the practical image memory ceiling. The dimension cap remains useful for format sanity, but any image whose decoded pixel buffer exceeds 512 MiB must fail even if width and height are individually below 65,535.

### Required Helpers

Add checked helper methods in `ResourceLimits`:

- `check_decoded_image_size(width, height, bytes_per_pixel) -> Result<usize>`
- `check_decoded_bytes(bytes) -> Result<()>`
- `check_polygon_vertices(count) -> Result<()>`
- `check_triangulated_face_budget(current_faces, additional_faces) -> Result<()>`
- `try_max_file_size_mb(mb) -> Result<ResourceLimitsBuilder>` or equivalent checked builder flow

No unchecked multiplication may remain in builder paths that convert MiB or pixel counts to bytes. CLI arguments must be clamped to documented maxima and rejected on overflow.

### `permissive()` Policy

`ResourceLimits::permissive()` must not be available to default production builds. Keep it only for tests and explicit trusted-input builds:

```rust
#[cfg(any(test, feature = "trusted-input"))]
pub fn permissive() -> Self
```

Do not add a CLI or GUI path that silently enables permissive limits. If a future workflow needs unusually large trusted files, it must use an explicit `--trusted-input` or similarly named opt-in that is reviewed by the Security Specialist.

### Error Contract

Prefer specific resource-limit errors over generic parser failures. Acceptable implementation shape:

- `ConversionError::ResourceLimitExceeded { resource, actual, limit }`
- `ConversionError::DecodedImageTooLarge { bytes, limit }`
- `ConversionError::PolygonVertexLimitExceeded { vertices, limit }`

If the existing error enum is not ready for new variants, use `InvalidInput` temporarily with stable, user-safe messages:

- `Decoded image size exceeds configured limit`
- `Polygon vertex count exceeds configured limit`
- `Resource limit exceeded`

Detailed byte counts may be logged in verbose/debug diagnostics, but default user-facing messages should not include full local paths or parser internals.

### Compatibility Impact

CLI:

- Add `--max-decoded-image-mb` with default `512`.
- Add `--max-polygon-vertices` for mesh CLIs with default `64`.
- Reject unreasonable or overflowing limit arguments; do not wrap or saturate silently.

GUI:

- Add decoded image and polygon limits to the existing resource settings model.
- Keep controls in advanced settings if necessary, but the active GUI limits must flow into preview, conversion, image readers, and mesh readers.

Tests:

- Tests may use `ResourceLimits::permissive()` under `cfg(test)`.
- Production code should not call `permissive()` without a trusted-input feature.

## A2: Image Reader Limits API

### Decision

Mirror the existing mesh-reader pattern in `img-core`: image format structs store `ResourceLimits`, expose `with_limits`, and are constructed by `FormatRegistry::get_reader_with_limits`.

Keep the `ImageReader` trait method as:

```rust
fn read(&self, data: &[u8]) -> Result<ImageData>;
```

Do not add limits as a `read` parameter. Storing limits in readers matches `mesh-core`, keeps trait-object call sites simple, and avoids a parallel API shape.

### Required API Shape

In `img-core::formats::registry`:

- `get_reader(format)` remains a compatibility wrapper using `ResourceLimits::default()`.
- `get_reader_with_limits(format, limits)` constructs readers with caller-provided limits.
- `get_writer(format)` may remain unchanged, but writers must validate `ImageData` through limit-aware validation when caller limits are available in orchestration code.

Per-format reader structs must become:

- `PngFormat { limits: ResourceLimits }`
- `JpegFormat { limits: ResourceLimits }`
- `BmpFormat { limits: ResourceLimits }`
- `GifFormat { limits: ResourceLimits }`
- `TiffFormat { limits: ResourceLimits }`
- `WebPFormat { limits: ResourceLimits }`
- `SvgFormat { limits: ResourceLimits }`

Each gets:

- `new() -> Self` with defaults
- `with_limits(limits: ResourceLimits) -> Self`

### Reader Validation Order

Required order for raster formats:

1. Check compressed file byte size using caller limits.
2. Verify extension/content mismatch before decode where the caller has path context.
3. Preflight declared dimensions without full pixel decode where practical.
4. Check dimensions and decoded-byte budget with checked arithmetic.
5. Decode.
6. Before `to_rgba8`, `to_rgb8`, or `into_raw`, re-check decoded budget for the target pixel layout.
7. Validate final `ImageData` with caller limits.

Header preflight expectations:

- PNG: parse IHDR width/height before `image::load_from_memory_with_format`.
- JPEG: scan SOF marker dimensions or use a non-decoding `image` metadata path if confirmed allocation-free.
- GIF: parse logical screen descriptor.
- BMP: parse DIB width/height and bit depth.
- WebP: parse VP8/VP8L/VP8X dimensions.
- TIFF: use a metadata-only dimensions path if available; otherwise treat as higher risk and enforce decoded budget immediately after metadata discovery before raw-buffer conversion.
- SVG: parse SVG tree, validate integer output width/height and `width * height * 4` before `Pixmap::new`.

### Public Compatibility

Preserve `get_reader(format)` for existing code and tests. New CLI and GUI code must use `get_reader_with_limits`; default wrappers are only for compatibility and low-risk tests.

## A3: Secure Output Write API

### Decision

Output validation and atomic writing belong in `common`, not GUI utilities or CLI binaries.

Add shared output APIs:

```rust
pub struct OutputWritePolicy {
    pub allow_overwrite: bool,
    pub allowed_output_root: Option<PathBuf>,
    pub create_parent_dirs: bool,
    pub block_system_dirs: bool,
}

pub struct ValidatedOutputPath {
    path: PathBuf,
    canonical_parent: PathBuf,
}

pub fn validate_output_path(path: &Path, policy: &OutputWritePolicy) -> Result<ValidatedOutputPath>;
pub fn write_file_bytes_atomic(path: &ValidatedOutputPath, data: &[u8], policy: &OutputWritePolicy) -> Result<()>;
```

Implementation may adjust exact field visibility, but callers must not be able to bypass validation accidentally.

### Validation Policy

`validate_output_path` must:

- Reject empty filenames and path components that are not valid file names.
- Canonicalize the parent directory for new files.
- Optionally create parent directories only when `create_parent_dirs` is true, then canonicalize.
- If `allowed_output_root` is set, canonicalize it and require the canonical parent to stay inside it.
- Reject existing output paths unless `allow_overwrite` is true.
- Reject path traversal after canonicalization.
- Reject obvious system directories when `block_system_dirs` is true.

System directory policy:

- Windows: block Windows, System32, Program Files, Program Files (x86), ProgramData roots, and drive roots unless an explicit allowed root permits a subdirectory.
- macOS: block `/System`, `/Library`, `/Applications`, `/bin`, `/sbin`, and `/usr` except explicitly allowed user-controlled roots such as `/usr/local` when reviewed.
- Linux: block `/bin`, `/sbin`, `/usr`, `/lib`, `/lib64`, `/etc`, `/var`, `/boot`, `/dev`, `/proc`, `/sys`, `/run`, and filesystem roots.

Prefer allowlisting via `allowed_output_root` whenever the GUI knows the selected output directory.

### Atomic Write Policy

Use a temporary file in the destination directory and persist/rename on the same filesystem. The `tempfile` crate is acceptable because it is already in the workspace dependency set and is permissively licensed.

Required behavior:

- No overwrite: use a no-clobber persist path where available, e.g. `persist_noclobber`, so races do not overwrite a newly created file.
- Explicit overwrite: use a replace operation documented per platform.
- Ensure the temp file is flushed before persist.
- On failure, remove the temp file where possible and return a controlled error.

CLI behavior:

- Add `--force` to `img-convert` and `mesh-convert`.
- Default behavior refuses to overwrite.
- Success output should avoid full paths unless verbose mode is enabled.

GUI behavior:

- GUI overwrite requires an explicit confirmation.
- GUI and CLI must call the same `common` validation and atomic-write APIs.

## A4: Mesh Pre-Parse Guard Strategy

### Decision

Mesh hardening stays in `mesh-core` format readers and shared helpers in `common`. CLI and GUI crates must not implement format-specific parsing guards.

### Format Strategy

STL:

- For binary STL, read triangle count at byte offset 80 before `stl_io::read_stl`.
- Validate `triangle_count <= max_faces`.
- Validate `84 + triangle_count * 50 == data.len()` with checked arithmetic before third-party parsing.
- If the file looks ASCII STL, perform a lightweight line/token scan to count `facet normal` records before parse and enforce `max_faces`.
- Preserve `stl_io` as the real parser after preflight.

OFF:

- Short-term: keep bounded full-file parse, but stop collecting all lines as the long-term direction.
- Immediately validate declared vertex and face counts before vertex allocation.
- Enforce `max_vertices_per_polygon` on each face.
- Before each fan-triangulated triangle push, call `check_triangulated_face_budget`.
- Track streaming parser rewrite as a follow-up if large OFF files remain a performance target.

PLY:

- Add a manual header preflight before `ply-rs-bw` payload parsing.
- Enforce `element vertex` and `element face` counts against limits before full parse.
- Bound the header scan and require `end_header` before that bound.
- Enforce list-size limits for face vertex lists.
- During triangulation, enforce `max_vertices_per_polygon` and incremental `max_faces`.
- Continue using `ply-rs-bw`, not `ply-rs`.

OBJ:

- Add lightweight preflight scan before `tobj`.
- Count `v`, `vn`, `vt`, and `f` records.
- Track maximum face vertex count from `f` lines and reject above `max_vertices_per_polygon`.
- Estimate triangulated faces as `sum(face_vertices - 2)` with checked arithmetic and enforce `max_faces`.
- Keep material loading non-filesystem by default; any future MTL loading must use checked reads and output-root-style path constraints.

glTF/GLB:

- Validate GLB header, version, total length, and chunk lengths before `gltf::import_slice`.
- For `.gltf`, parse document metadata before importing buffers where possible.
- Enforce accessor counts, index counts, bufferView byte lengths, and buffer declared byte lengths before mesh extraction.
- Reject or explicitly constrain external buffers for v1.0.0. If external buffers are later supported, resolve them only relative to the input file parent and read them through `read_file_bytes_checked`.
- During extraction, enforce incremental vertex and face budgets before pushing.

DXF:

- Keep DXF behind file-size validation and add incremental vertex/face checks while extracting entities.
- Add a preflight scan only if a low-cost entity-count strategy is confirmed for the `dxf` crate input model.
- Treat complex DXF parser hardening as a follow-up unless v1.0.0 ships DXF as a primary advertised format.

STEP:

- STEP remains feature-gated and not part of the default shipped v1.0.0 feature set unless A5 is revised.
- For `step`, enforce file-size validation before ruststep/truck parsing and incremental mesh budgets during extraction.
- For `step-opencascade`, require separate legal/security approval and platform build validation before shipping.

### Trade-Off

The immediate mitigation is preflight plus incremental budget checks around existing parsers. Full streaming parser rewrites are deferred unless tests show the quick mitigations still allocate significantly before rejection.

## A5: Dependency and Release Security Policy

### v1.0.0 Release Matrix

Default v1.0.0 shipped binaries:

- `img-convert`: default features only.
- `mesh-convert`: default features only; no `step`; no `step-opencascade`.
- `converter-gui`: default features only; no `viewer-3d`.

Not shipped by default in v1.0.0:

- `mesh-core/step`
- `mesh-core/step-opencascade`
- `converter-gui/viewer-3d`
- `converter-gui-modern` unless explicitly promoted by the product owner.

Rationale:

- `step` currently brings the vulnerable `truck-meshalgo -> vtkio -> lz4_flex` path identified by the security review.
- `step-opencascade` has native OCCT build, license, and platform packaging implications.
- `viewer-3d` is optional/prototype and expands the GUI dependency surface.

### Required Security Commands

Default release gate:

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo audit
cargo deny check advisories licenses bans sources
```

Feature-specific gates before shipping optional stacks:

```bash
cargo audit --features step -p mesh-core
cargo deny check advisories licenses bans sources --features step -p mesh-core
cargo audit --features viewer-3d -p converter-gui
cargo deny check advisories licenses bans sources --features viewer-3d -p converter-gui
```

For `step-opencascade`, run the same audit/deny checks and require a separate Security Specialist and System Architect approval record before release packaging.

### Advisory Ignore Policy

Every `deny.toml` advisory ignore must include:

- Advisory ID.
- Owning role or person.
- Rationale.
- Whether the advisory is security, unsoundness, or unmaintained-only.
- Review date no more than 30 days after entry for high/critical advisories and no more than 90 days for unmaintained-only advisories.
- Removal condition.

High or critical security advisories may not be ignored for shipped feature sets without explicit Security Specialist and System Architect signoff.

### Signing and Attestation

Release signing is required as a tracked security milestone, but Windows/macOS signing implementation is not assigned to this Windows architecture session.

For v1.0.0 readiness:

- Checksums are not sufficient as the long-term distribution story.
- Windows Authenticode and macOS notarization/signing must be tracked before public distribution.
- macOS signing/notarization work must be documented for a macOS session rather than implemented from Windows.
- Sigstore or GitHub artifact attestations are recommended as a cross-platform follow-up.

## Handoff Requirements

Senior Engineer:

- Implement A1-A4 APIs before parser/app changes.
- Keep parsing hardening in `common`, `img-core`, and `mesh-core`.
- Do not add permissive-limit production paths.
- Preserve default-reader compatibility but migrate CLI/GUI to limit-aware readers.
- Add tests for decoded byte caps, SVG raster caps, output overwrite refusal, atomic writes, and mesh triangulation blow-up.

Security Specialist:

- Perform V1 before Senior Engineer begins implementation.
- Challenge the 512 MiB decoded image cap and 64 vertices-per-polygon default with realistic malicious inputs.
- Approve or request changes to the v1.0.0 feature matrix.
- Review `deny.toml` ignores and feature-specific audit results.

## Open Security Signoffs

- A1 defaults require V1 Security Specialist approval.
- A5 release matrix and advisory-ignore policy require V1 Security Specialist approval.
- `step-opencascade` cannot ship without separate legal/security approval and platform validation.
