# SETUP_NOTES.md

This file documents one-time setup steps that must be performed when pulling this
repository onto a new development machine, or after significant toolchain changes.

Claude Code reads this file at session start and will surface any items relevant
to the current machine.

**Mark items complete by appending `[DONE — machine name, date]` after the step.**

---

## All Machines

- Install **Rust** (stable channel via rustup): https://rustup.rs/
  - Minimum version: 1.92 (see `rust-version` in `Cargo.toml`)
  - Run `rustup default stable && rustup update`
  - Verify: `rustc --version && cargo --version`
- Configure Git identity: `git config --global user.name "Your Name"` etc.
- Clone repo and verify workspace builds: `cargo build --workspace`
- Run full test suite: `cargo test --workspace`
- Install security audit tool: `cargo install cargo-audit`
- Verify formatting and lint pass: `cargo fmt --check && cargo clippy --workspace -- -D warnings`

---

## Windows Only

- Install **Visual Studio Build Tools** with "Desktop development with C++" workload
  (required by Rust MSVC toolchain on Windows)
  - Or install the full Visual Studio Community edition
  - Verify Rust sees MSVC: `rustup show` should list `stable-x86_64-pc-windows-msvc`
- Verify high-DPI egui rendering by running the GUI on a 4K display if available

---

## macOS Only

- Install **Xcode Command Line Tools**: `xcode-select --install`
  (required for Rust compiler and for code-signing release builds)
- Add Apple Silicon target (if on Apple Silicon and need universal binary):
  `rustup target add x86_64-apple-darwin` (for Intel slice)
  `rustup target add aarch64-apple-darwin` (should be default on M-series)
- Verify Apple Developer account is set up before cutting a signed release
- Test Retina rendering and dark/light mode appearance of the egui GUI

---

## Linux Only

- Install system libraries required by egui/eframe for Wayland/X11:
  ```
  sudo apt-get update && sudo apt-get install -y \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    pkg-config
  ```
- For Wayland support, also install: `libwayland-dev`
- Verify: `cargo build --workspace` succeeds without link errors
- Test egui GUI on both Wayland and X11 if both are available

---

*Add new setup steps below this line as the project evolves.*
