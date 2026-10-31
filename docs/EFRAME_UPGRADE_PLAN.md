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
