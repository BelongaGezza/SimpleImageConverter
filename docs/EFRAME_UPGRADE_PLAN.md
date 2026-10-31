# eframe/egui Upgrade Plan (0.27 → 0.37)

> Branch: `feature/eframe-upgrade` | Created: 2026-10-31 | Status: Planned
> Driver: RUSTSEC-2026-0257 (`webbrowser 0.8.15`, `BROWSER` argument injection), see `SECURITY_TODO.md`.

## Why

`egui-winit 0.27.2` pins `webbrowser = "0.8.3"`, so `cargo update -p webbrowser` cannot reach the fixed 1.2.2. The only real fix is a newer eframe/egui. Latest on crates.io is eframe 0.37.0; we are on 0.27, so this is a multi-version API migration.

## Scope

Affected crates: `converter-gui` and `converter-gui-modern` (both pin `eframe`/`egui` 0.27 and `rfd` 0.15). `converter-gui` also has an optional `wgpu = "28"` for the `viewer-3d` feature, which must stay compatible with whatever wgpu version the new eframe uses.

Surface area (approximate): ~209 `egui::`/`eframe::` references. Hot spots: `converter-gui/src/app.rs` (2368 lines), `converter-gui-modern/src/app.rs` (2271), `ui/preview.rs`, `ui/theme.rs`, both `main.rs` (NativeOptions/viewport setup).

Out of scope: feature changes, UI redesign, CAD work.

## Expected outcome and limits

| Advisory | Crate | Cleared by this upgrade? |
|---|---|---|
| RUSTSEC-2026-0257 | webbrowser | Yes (primary goal) |
| RUSTSEC-2026-0370/0371/0372 | zvariant, zbus_names, zvariant_derive | Probably, via newer `rfd`/`ashpd`; verify |
| RUSTSEC-2026-0192 | ttf-parser | **No** as the only path: also pulled in via `resvg → usvg → fontdb`. Needs a resvg bump or an ignore |
| RUSTSEC-2026-0206 | rustybuzz | **No** by itself: comes via `usvg`. Same handling |

(Correction to the 2026-10-31 SECURITY_TODO note, which attributed `ttf-parser`/`rustybuzz` only to the SVG stack: `ttf-parser` also reaches `epaint`, but `usvg` keeps it alive regardless.)

## Plan

Work in small, individually compiling steps on the branch. Do not merge to `main` until every exit criterion below is met.

### Phase 0 — Baseline (before touching versions)
1. Record green baseline on this branch: `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`, and `cargo test -p converter-gui --features viewer-3d`.
2. Save `cargo deny check advisories` output as the "before" list.
3. Read the egui `CHANGELOG.md` entries for 0.28 → 0.37 and write a short list of breaking changes that match our usage (grep targets: `Rounding`, `Frame::none`, `id_source`, `allocate_ui_at_rect`, `NativeOptions`/`viewport`, `CreationContext`, `ctx.input`, texture loading, `Button::image`, `wgpu_render_state`).
4. Check `rust-resources.md` and the researcher notes (it already says the upgrade was planned for v0.3.0) and the minimum Rust version of the target eframe vs. our `rust-version = "1.92"`.

### Phase 1 — Dependency bump
5. Decide target version: latest stable (0.37) unless its MSRV or wgpu requirement conflicts. Pick one version for both crates and for `egui`/`eframe` together (never mixed).
6. Move the shared pins (`eframe`, `egui`, `rfd`) into `[workspace.dependencies]` so the two GUI crates cannot drift.
7. Bump `rfd` to the version compatible with the chosen eframe; confirm `ashpd`/`zbus` resolve to non-flagged versions on Linux.
8. Reconcile the optional `wgpu = "28"` in `converter-gui`: match eframe's wgpu version or keep it isolated behind `viewer-3d`.

### Phase 2 — Fix compile errors, one crate at a time
9. `converter-gui-modern` first (smaller surface, theme + app), then `converter-gui`.
10. Order within each crate: `main.rs` (window/viewport options) → theme/style → `app.rs` → `ui/*` → `preview*`/3D viewer.
11. Commit after each crate compiles (`cargo check -p <crate>`), with conventional messages (`refactor(gui): ...`). No behaviour changes mixed in.

### Phase 3 — Verification
12. Automated: fmt, clippy `-D warnings`, `cargo test --workspace`, `--features viewer-3d`, `cargo doc`, and the existing `converter-gui/tests/*` (security, integration, sprint11).
13. Dependency/security: `cargo deny check advisories licenses bans sources`, `cargo audit` (install locally), confirm `webbrowser` ≥ 1.2.2 in `cargo tree -i webbrowser` (or gone). New transitive licences may need `deny.toml` additions: review each deliberately.
14. Manual on macOS (this machine): launch both GUIs; drag-and-drop, file dialogs, image preview, 3D preview, settings persistence, batch queue, light/dark theme, Retina scaling, window resize. Follow `MANUAL_TESTING_CHECKLIST_SPRINT13.md`.
15. Manual on Windows 11 and Linux (Wayland + X11) are required by the platform matrix. Until run, record them as pending in the PR. Do not claim cross-platform verification without them. CI build jobs cover compile only.
16. Release build check: `cargo build --release` for all three binaries; compare binary size against 0.3.0 (profile is size-optimised).

### Phase 4 — Close out
17. Update `deny.toml`: remove any now-stale ignores; for advisories not cleared (ttf-parser, rustybuzz if still present), add ignore entries with owner and review date.
18. Update `SECURITY_TODO.md`, `rust-resources.md` (egui version, lessons learned, per CLAUDE.md), `CHANGELOG.md`, and `docs/GUI_USAGE_GUIDE.md` if UI behaviour changed.
19. Open a PR from `feature/eframe-upgrade` → `main`; CI must be green (including the security job).
20. Merge only after the exit criteria are met.

## Exit criteria (merge gate)
- [ ] Both GUI crates build on macOS locally and in CI on all CI platforms
- [ ] fmt, clippy `-D warnings`, tests (with and without `viewer-3d`), docs all pass
- [ ] RUSTSEC-2026-0257 no longer reported; remaining advisories either cleared or ignored with owner + review date
- [ ] macOS manual checklist passed; Windows/Linux manual results recorded (or explicitly listed as pending in the PR)
- [ ] No visual or functional regressions vs. 0.3.0 in the manual checklist
- [ ] Docs and changelog updated

## Risks
| Risk | Mitigation |
|---|---|
| Large API drift across ten releases | Read changelog first; migrate in compiling steps; consider stepping through intermediate versions only if the jump proves unmanageable |
| `wgpu` version conflict with `viewer-3d` | Align versions in Phase 1; test the feature explicitly |
| MSRV higher than 1.92 | Check in Phase 0; bump `rust-version` deliberately and update CI if needed |
| Visual changes (default style, spacing) | Compare screenshots before/after; adjust `ui/style.rs`/`theme.rs` |
| Platform-specific regressions (Wayland, Windows DPI) | Platform guards per `CLAUDE.md`; require manual runs before release |
| Remaining advisories via `resvg` | Handle separately (resvg bump or ignore); not a blocker for this branch |

## Rollback
The branch is isolated. If the migration stalls, fall back to the temporary option: ignore RUSTSEC-2026-0257 in `deny.toml` with a justification and review date, and keep the upgrade branch for later.

---

## Phase 0 results (2026-10-31, rustc 1.97.1, macOS)

### Baseline (before any version change)
| Check | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace -- -D warnings` | **Fail (pre-existing)**: 7 × "falling back to `f32`" (future-incompat) in `converter-gui-modern`: `src/app.rs:974,1088,1558`, `src/ui/mode_switch.rs:46`, `src/ui/theme.rs:83` and others. Fix by writing `1.5_f32` / `2.0_f32` |
| `cargo test --workspace` | Pass (all suites ok) |
| `cargo test -p converter-gui --features viewer-3d` | **Fail (pre-existing)**: `converter-gui/src/app.rs:1023` uses `frame` but the parameter is `_frame` (line 502) |
| `cargo doc --workspace --no-deps` | Pass |
| `cargo deny check advisories` | 6 failing: RUSTSEC-2026-0192, -0206, -0257, -0370, -0371, -0372 |

The two pre-existing failures must be fixed in their own commits before the migration so the baseline is green and later failures are attributable to the upgrade.

### MSRV
eframe 0.37.0 requires **Rust 1.95**; the workspace declares `rust-version = "1.92"`. The workspace `rust-version` must be raised to 1.95, and the CI toolchain checked against it (Phase 1).

### Breaking changes that match our usage (from the egui changelog, 0.28 → 0.37)
Counts are occurrences in `converter-gui*/`.

| Change | Version | Our usage | Effort |
|---|---|---|---|
| `Rounding` → `CornerRadius` (values now `u8`/`i8`; same for `Margin`, `Shadow`) | 0.31 | `Rounding` ×6, `inner_margin` | Mechanical, check float → int conversions |
| `StrokeKind` required when painting rects with strokes; `Frame` now includes stroke width in padding | 0.31 | `painter()` ×16, `Stroke::new` ×8 | Visual regression risk, compare screenshots |
| `id_source` → `id_salt` | 0.30 | `from_id_source` ×2 | Mechanical |
| `allocate_ui_at_rect` → `scope_builder` / `allocate_new_ui` | 0.30–0.32 | ×3 | Small |
| `screen_rect` → `content_rect` / `viewport_rect` | 0.33 | ×1 | Small |
| `SidePanel` / `TopBottomPanel` → `Panel`; `CentralPanel::show` deprecated | 0.34 | `TopBottomPanel` ×6, `CentralPanel` ×2 | Moderate (layout) |
| `eframe::App::update` → `App::ui` (+ `logic`); `Context::run_ui`; `Ui` derefs to `Context` | 0.34 | `fn update` in both apps | **Largest change**: touches the main loop in both 2,000+ line `app.rs` files |
| `Context::available_rect` / `used_size` deprecated | 0.34 | not found in grep | None expected |
| `clip_rect_margin` removed; `Window` margins reworked | 0.35–0.36 | not found in grep | Check windows visually |
| MSRV bumps 1.80 → 1.95 | various | see above | Workspace change |

Also in use and worth checking at compile time (no specific breakage found in the changelog): `ColorImage` ×10, `load_texture` ×2, `dropped_files` ×12, `ctx.input` ×12, `ScrollArea` ×9, `ViewportBuilder` / `NativeOptions` ×2 each, `ProgressBar`, `ComboBox`.

Deprecated APIs still compile but emit warnings, which `clippy -D warnings` turns into errors. Treat each deprecation as a required fix, not an optional one.

### Revised first steps
1. Fix the two pre-existing baseline failures (separate commits).
2. Bump `rust-version` to 1.95 and check the CI toolchain.
3. Proceed to Phase 1.

## Phase 1 results (2026-10-31)

- Baseline fixed in separate commits: f32 literals (clippy) and `viewer-3d` (frame param, mutex-guard borrow, missing test imports). Workspace tests pass with and without `viewer-3d`; `cargo clippy --workspace -- -D warnings` is clean. Clippy with `--features viewer-3d` still reports pre-existing lints in `preview_3d.rs` (not run in CI; out of scope here).
- Dependency bump: `eframe`/`egui` 0.37, `rfd` 0.17 now declared once in `[workspace.dependencies]`; both GUI crates use `workspace = true`. Workspace `rust-version` raised 1.92 → 1.95.
- Renderer: eframe 0.37 defaults to **wgpu**; 0.27 defaulted to **glow**. We set `default-features = false` with `glow` to keep the renderer unchanged. Revisit after the migration.
- `links` feature (pulls in `webbrowser`) deliberately omitted; the GUI opens URLs via the `open` crate. `webbrowser` is no longer in the dependency tree, so RUSTSEC-2026-0257 is cleared.
- `cargo deny check advisories` now reports 5: RUSTSEC-2026-0192, -0206, -0370, -0371, -0372. `zvariant`/`zbus_names`/`zvariant_derive` were NOT cleared by this bump; investigate their remaining path (Phase 4).
- `Cargo.lock` remains untracked/ignored (see `SECURITY_TODO.md`), so lock changes are not part of these commits.
- State: **does not compile yet** (18 errors in `converter-gui`, listed below). Phase 2 starts from here.

### Compile errors at start of Phase 2
`App::update` → `App::ui`; `TopBottomPanel` / `egui::menu::bar` removed; `Ui::set_enabled` (×6); `Context::screen_rect`; `Context::wants_keyboard_input`; `allocate_ui_at_rect`; `DroppedFile::path` is now a method; one call with an extra argument (`drop_zone.rs:91`); one type mismatch (`app.rs:852`).

## Phase 2 results (2026-10-31)

- `converter-gui` and `converter-gui-modern` compile against egui/eframe 0.37 with no deprecation warnings. `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo test -p converter-gui --features viewer-3d`, `cargo doc` and `cargo build --release` all pass (macOS).
- Migration map: `App::update` → `App::ui` (a `ctx` clone is taken from `ui.ctx()` so the body is unchanged); `TopBottomPanel` → `Panel` (`min/max/default_height` → `*_size`); `egui::menu::bar` → `MenuBar`; `ui.close_menu` → `ui.close`; `set_enabled` → `if !x { ui.disable() }`; `screen_rect` → `content_rect`; `wants_keyboard_input` → `egui_wants_keyboard_input`; `DroppedFile.path` field → `path()` method; `rect_stroke` gets `StrokeKind::Middle` (matches the old centred stroke); `allocate_ui_at_rect` → `scope_builder`; `Rounding` → `CornerRadius` (`u8`); `from_id_source` → `from_id_salt`; `ctx.style()/set_style` → `global_style()/set_global_style`; `ColorImage::new` → `filled`; `raw_scroll_delta` → `smooth_scroll_delta` (3D zoom; feel may differ); `run_native` creator returns `Ok(Box<..>)`.
- Release binaries: `converter-gui` 6,172,288 B, `converter-gui-modern` 6,122,592 B (compare with 0.3.0 artifacts in Phase 3).
- `deny.toml`: licence exception moved from `epaint` to `epaint_default_fonts` (same Ubuntu font licence, now SPDX `Ubuntu-font-1.0`). `cargo deny check licenses` passes.
- Remaining advisories (5): `zvariant`, `zbus_names`, `zvariant_derive` still reach the lock via `accesskit_unix` (Linux-only, `accesskit` feature; `cargo tree --target all`), not via `rfd`. `ttf-parser` and `rustybuzz` via `usvg`. All five are unmaintained notices with no vulnerability; handle with dated ignores in Phase 4, or drop the `accesskit` feature (loses screen-reader support, not recommended).

### Not yet verified (needs a human at the GUI)
Visual regressions (Frame/stroke padding, panel layout, `Panel` sizing), drag-and-drop, file dialogs, 3D viewer zoom, theme in both modes, Retina scaling. Windows and Linux builds/behaviour are unverified.
