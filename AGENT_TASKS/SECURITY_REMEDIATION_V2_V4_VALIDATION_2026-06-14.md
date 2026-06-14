# Security Remediation V2-V4 Validation

**Created:** June 14, 2026  
**Validator:** Casey Morgan, Security Specialist  
**Scope:** Validation of V2, V3, and V4 after Senior Engineer implementation.

## Executive Decision

Final re-validation after Senior Engineer follow-up fixes is APPROVED for V5.

V2 passes for the default shipped profile: `bytes` is updated to `1.11.1`, `lz4_flex` is absent, `cargo audit` passes, CI runs the full `cargo deny check advisories licenses bans sources` command, and the same full `cargo deny` command passed locally after adding a documented, time-boxed `core2` ignore.

V3 passes. Image/SVG/GUI read hardening remains valid, and the previous STL/PLY parser-order gaps are resolved: STL preflight now runs before `stl_io`, and PLY header/count preflight now runs before `ply-rs-bw` payload parsing.

V4 passes. CLI and GUI conversions use shared output validation and atomic writes; GUI conversion helpers deny overwrite by default, and overwrite-capable helper variants require explicit policy from a verified confirmation path.

## V2: Dependency and CI Remediation

**Status:** Pass

Validated:
- `Cargo.lock` contains `bytes 1.11.1`.
- No `lz4_flex` package is present in `Cargo.lock`.
- `cargo tree -i lz4_flex` reported no matching package for the default graph.
- `cargo tree -p mesh-core --features step -i lz4_flex` reported no matching package.
- `cargo tree --target all -i bytes` shows the target-specific GUI path resolves to `bytes 1.11.1`.
- `.github/workflows/ci.yml` runs `cargo deny check advisories licenses bans sources`.

Command result:
- `cargo audit`: passed with allowed warnings only.
- `cargo deny check advisories licenses bans sources`: passed locally.

Accepted residual risk:
- `RUSTSEC-2026-0105` (`core2` unmaintained/yanked) is ignored in `deny.toml` with Security Specialist ownership, review by 2026-09-14, and removal condition tied to upstream `image`/`ravif`/`rav1e` dependency graph updates. This is accepted for V5 because it is an unmaintained/yanked transitive advisory, not a known high/critical security vulnerability in shipped default feature sets.

## V3: Parser and Decode DoS Fixes

**Status:** Pass

Accepted areas:
- `common::limits::ResourceLimits` now has `max_decoded_image_bytes` and `max_vertices_per_polygon`.
- Decoded image byte math uses checked arithmetic.
- PNG/JPEG/BMP/GIF/WebP dimension preflight exists where implemented in `img-core/src/formats/decode.rs`.
- SVG validates `width * height * 4` before `Pixmap::new`.
- Image readers support `with_limits`, and `img-convert` / GUI image conversion use `get_reader_with_limits`.
- GUI mesh preview/viewer paths use `read_file_bytes_checked`.
- OFF and OBJ enforce polygon vertex limits and triangulated face budgets before pushing generated faces.

Follow-up validation:
- `mesh-core/src/formats/stl.rs` calls `preflight_stl` before `stl_io::read_stl`; exact binary STL files validate declared triangle count and `84 + 50 * n` byte length with checked arithmetic before parser handoff.
- `mesh-core/src/formats/ply.rs` calls `preflight_ply` before `ply-rs-bw`; the preflight bounds the header scan, validates declared vertex/face counts, and checks ASCII face-list polygon sizes before third-party parsing when list counts are visible.
- Regression tests now cover STL pre-parser face-count and length-mismatch rejection plus GUI overwrite denial. Existing image, SVG, mesh, preview, and triangulation tests continue to pass.

Command result:
- `cargo test -p common`: passed.
- `cargo test -p img-core`: passed.
- `cargo test -p mesh-core`: passed.
- `cargo test -p converter-gui`: passed.

## V4: Output Path and Write Safety

**Status:** Pass

Accepted areas:
- `common` provides `OutputWritePolicy`, `ValidatedOutputPath`, `validate_output_path`, and `write_file_bytes_atomic`.
- Atomic writes use a temporary file in the destination directory and `persist_noclobber` when overwrite is not allowed.
- `img-convert` and `mesh-convert` use shared output validation and atomic writes.
- CLI conversions refuse overwrite by default and require `--overwrite`.
- Common tests cover overwrite denial/allow, invalid filenames, allowed roots, parent creation, and atomic writes.

Follow-up validation:
- `converter-gui/src/conversion.rs` default `convert_image` and `convert_mesh` wrappers use `OutputWritePolicy::default()`, which denies overwrite.
- Normal app and batch conversion call sites use the default-deny wrappers.
- Explicit `convert_image_with_policy` and `convert_mesh_with_policy` variants are documented for use only after a verified confirmation path.

## Full Command Summary

Passed:
- `cargo audit`
- `cargo test -p common`
- `cargo test -p img-core`
- `cargo test -p img-convert`
- `cargo test -p mesh-core`
- `cargo test -p mesh-convert`
- `cargo test -p converter-gui`
- `cargo fmt --check`
- `cargo clippy --workspace -- -D warnings`

Passed after follow-up:
- `cargo deny check advisories licenses bans sources`
- `cargo test --workspace`

Additional dependency checks:
- `cargo tree -i lz4_flex`: no matching package.
- `cargo tree -p mesh-core --features step -i lz4_flex`: no matching package.
- `cargo tree --target all -i bytes`: target-specific path uses `bytes 1.11.1`.

## V5 Readiness

V5 final security approval may proceed and has been executed in `AGENT_TASKS/SecurityRemediation-SecurityAPPROVAL.md`.

Final security gates passed:
1. `cargo audit`
2. `cargo deny check advisories licenses bans sources`
3. `cargo test -p common`
4. `cargo test -p img-core`
5. `cargo test -p mesh-core`
6. `cargo test -p converter-gui`
7. `cargo fmt --check`
8. `cargo clippy --workspace -- -D warnings`
9. `cargo test --workspace`

## Senior Engineer Follow-Up Remediation

**Updated:** June 14, 2026  
**Implementer:** Jordan Rivera, Senior Engineer follow-up sub-agent

Fixes made:
- Added STL preflight before `stl_io::read_stl`: exact binary STL files validate declared triangle count and `84 + 50 * n` byte length with checked arithmetic; ASCII STL keeps a documented facet-count fallback before parser handoff.
- Added PLY preflight before `ply-rs-bw`: bounded header scan up to `end_header`, declared vertex/face count enforcement, and ASCII face-list count checks where the polygon count is visible before third-party parsing.
- Changed GUI conversion helpers to deny overwrite by default. New explicit policy-based helper variants require callers to pass `allow_overwrite: true` only from a verified confirmation path; existing GUI paths now return a controlled existing-output error rather than silently overwriting.
- Added GUI conversion regression tests proving image and mesh conversion helpers deny existing outputs by default.

Verification:
- `cargo test -p mesh-core`: passed.
- `cargo test -p converter-gui`: passed.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace -- -D warnings`: passed.
- `cargo audit`: passed with allowed warnings only.
- `cargo install cargo-deny --locked`: failed before download because crates.io access hit Windows Schannel `CRYPT_E_NO_REVOCATION_CHECK` certificate revocation errors.
- `cargo deny check advisories licenses bans sources`: not locally runnable because `cargo-deny` is not installed.

Final re-validation:
- V2, V3, and V4 were rerun and passed.
- V5 approval completed with documented residual risks.
