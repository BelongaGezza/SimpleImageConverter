# Release Build Report — Sprint 13 Phase 4 (macOS)

**Date:** May 29, 2026  
**Engineer:** Jordan Rivera (Senior Engineer)  
**Workspace version:** 0.3.0 (no version bump per Task 4.3 gate)  
**Scope:** macOS ARM64 only this session

---

## Build environment

| Item | Value |
|------|--------|
| OS | macOS 26.5 (Build 25F71) |
| CPU | Apple Silicon (`arm64`) |
| Rust target | `aarch64-apple-darwin` |
| `rustc` | 1.92.0 (ded5c06cf 2025-12-08) |
| Session | macOS M-series |

---

## Verification (Task 4.1)

| Gate | Command | Result |
|------|---------|--------|
| Tests | `cargo test --workspace` | **PASS** — 0 failed; 2 ignored (img-core doc tests) |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | **PASS** |
| Format | `cargo fmt --all --check` | **PASS** |

---

## Release binaries

Built with:

```bash
cargo build --release --bin converter-gui --bin img-convert --bin mesh-convert
```

Packaging scripts consume `target/aarch64-apple-darwin/release/` when present (cross-target layout on this host).

| Binary | Path | Size | Arch |
|--------|------|------|------|
| `converter-gui` | `target/aarch64-apple-darwin/release/converter-gui` | 5.6 MiB (5,881,408 B) | arm64 |
| `img-convert` | `target/aarch64-apple-darwin/release/img-convert` | 2.4 MiB (2,567,552 B) | arm64 |
| `mesh-convert` | `target/aarch64-apple-darwin/release/mesh-convert` | 1.9 MiB (2,033,824 B) | arm64 |

Host-native copies also exist under `target/release/` (same sizes, May 29, 2026).

---

## CLI smoke tests

| Test | Result |
|------|--------|
| `./target/release/img-convert --help` | **PASS** |
| `./target/release/mesh-convert --help` | **PASS** |
| Image conversion | **PASS** — 1×1 PNG → JPEG (`target/release-smoke/`; no binary fixtures under `tests/data/`) |
| Mesh conversion | **PASS** — ASCII STL → OBJ (`target/release-smoke/triangle.stl`) |

GUI manual smoke remains **Task 3.1 / 3.2** (human).

---

## Packaging (Task 4.2)

Commands:

```bash
bash scripts/package-macos.sh 0.3.0 aarch64-apple-darwin
bash scripts/package-gui-macos.sh 0.3.0 aarch64-apple-darwin
```

| Archive | Size | SHA256 |
|---------|------|--------|
| `simpleimageconverter-0.3.0-macos-arm64.tar.gz` | 2.4 MiB | `d871bb2b7f633e5a61d1463c6ea9c85d20536f5f0536c67e77d3d9867063d56c` |
| `simpleimageconverter-gui-0.3.0-macos-arm64.tar.gz` | 3.2 MiB | `8a53d502fb3f66a7da1bfe57ed1135136b83dbab416d31ff3f5abf33b3cd459d` |

Checksum file: `release/SHA256SUMS-macos-0.3.0.txt`

### Archive contents

**CLI** (`simpleimageconverter-0.3.0-macos-arm64.tar.gz`):

- `macos-arm64-v0.3.0/img-convert`
- `macos-arm64-v0.3.0/mesh-convert`
- `macos-arm64-v0.3.0/README.md`
- `macos-arm64-v0.3.0/INSTALL.txt`

**GUI** (`simpleimageconverter-gui-0.3.0-macos-arm64.tar.gz`):

- `macos-arm64-gui-v0.3.0/converter-gui`
- `macos-arm64-gui-v0.3.0/README.md`, `INSTALL.txt`
- `macos-arm64-gui-v0.3.0/LICENSE-MIT`, `LICENSE-APACHE`

Archives at repository root (script default). **Not committed** to git (local release artefacts).

---

## Out of scope this session

| Platform | Status |
|----------|--------|
| Windows x64 | Pending — `scripts/package-windows.ps1` on Windows 11 |
| Linux x64 | Pending — `scripts/package-linux.sh` on Ubuntu 24.04+ |
| macOS x64 | Not built (Apple Silicon session; arm64 only) |

---

## Blockers before Task 4.3 (tag / GitHub Release)

1. **Task 3.1** — Human manual GUI checklist on macOS + Windows 11  
2. **Task 3.2** — Cross-platform GUI smoke (Win/macOS/Linux)  
3. **Task 5.1** — Architect formal approval (`SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md`)  
4. **Task 4.3** — Version bump to 1.0.0, `v1.0.0` tag, GitHub Release (**not** executed)  
5. **Windows/Linux** Tasks 4.1–4.2 on respective platforms  

---

*Sprint 13 Phase 4 — Release Execution (macOS).*
