# Security TODO — SimpleImageConverter

> Reviewed: 2026-10-31 | Stack: Rust (workspace) | Severity: 🟠 Medium | Reviewer: Security Review
> Previous review: 2026-08-25 | Updated 2026-10-31 for eframe upgrade branch

Local image/mesh conversion tool. No network calls, no auth, no sensitive data. Posture is good for this type of project — `cargo-deny`, `cargo audit` in CI, fuzz targets, and explicit advisory tracking are all in place. However, `cargo deny check advisories` currently **fails** locally (see below), so CI's security job is likely red or will go red on the next run.

## TODOs

### HIGH — `cargo deny check advisories` fails (new advisories since last review)
- [x] **RUSTSEC-2026-0257** (`webbrowser 0.8.15`): `cargo update -p webbrowser` cannot fix it (`egui-winit 0.27` pins `webbrowser 0.8`). Addressed on branch `feature/eframe-upgrade` (eframe/egui 0.37, `links` feature omitted, `webbrowser` no longer in the tree). **Closes when that PR merges**; see `docs/EFRAME_UPGRADE_PLAN.md`.
- [ ] Unmaintained notices still failing after the upgrade (no known exploit; ignore with owner + review date in `deny.toml`, or remove by upgrading):
  - RUSTSEC-2026-0192 `ttf-parser`, RUSTSEC-2026-0206 `rustybuzz`: via `resvg → usvg → fontdb` (needs a resvg bump)
  - RUSTSEC-2026-0370 `zvariant`, -0371 `zbus_names`, -0372 `zvariant_derive`: Linux-only, via `eframe → egui-winit → accesskit_winit → accesskit_unix`. Dropping eframe's `accesskit` feature would remove them but loses screen-reader support.

### MEDIUM — Cargo.lock not committed (still open)
- [ ] `.gitignore` line 5 still excludes `Cargo.lock`, and `git ls-files` shows it is not tracked. This is a binary application, so the lock file should be committed for reproducible, auditable builds. Without it, the advisory findings above are not tied to a fixed dependency set.
- [ ] Remove `Cargo.lock` from `.gitignore`, commit it, and update it deliberately via `cargo update` with a changelog entry. Also consider `--locked` on CI build/test steps once it is committed.

### LOW — Stale `deny.toml` ignore entry: RUSTSEC-2026-0105 (core2)
- [x] Review date (2026-09-14) has passed. Verified: `core2` is no longer in `Cargo.lock` (`bitstream-io 4.10.0` now uses `no_std_io2`), and `cargo deny` reports "advisory was not encountered".
- [ ] Remove the `RUSTSEC-2026-0105` entry and its comment from `deny.toml`.
- [ ] Re-check the other ignores (`RUSTSEC-2024-0436` paste, `-0388` derivative, `-0384` instant): all three crates are still in `Cargo.lock`, so they remain valid, but none has a review date. Add owner + review date to each.
- [ ] Consider a CI check that fails when an ignore entry's review date is in the past.

### LOW — `cargo-deny` advisory database not pinned in CI
- [ ] `ci.yml` runs `cargo deny check advisories licenses bans sources` against a live advisory DB, so results change between runs (as just happened with the new advisories). This is a trade-off: a live DB catches new issues quickly. Decide deliberately; if keeping it live, consider splitting advisories into a separate non-blocking/scheduled job so unrelated PRs are not blocked by new upstream advisories.
- [ ] `deny.toml` sets `db-path = "~/.cargo/advisory-db"`; verify this works on CI runners (`~` may not expand) or drop the setting to use the default.

### LOW — Fuzz targets not run in CI
- [ ] `fuzz/` exists, but `.github/workflows/` (`ci.yml`, `docs.yml`, `release.yml`) contains no fuzz step. Add at least `cargo fuzz build` on PRs to confirm targets compile, and ideally a short scheduled run (e.g. 60s per target).

### INFO — Release profile is well-hardened
- `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"` confirmed in `Cargo.toml`. No action needed.

### INFO — Licence compliance is actively managed
- `deny.toml` allows only permissive licences (plus font-specific licences for `epaint`). No action needed; maintain as dependencies change.

### INFO — Environment note
- `cargo-audit` is not installed on this machine (CI installs it). Install locally (`cargo install cargo-audit --locked`) to reproduce the CI audit step.
