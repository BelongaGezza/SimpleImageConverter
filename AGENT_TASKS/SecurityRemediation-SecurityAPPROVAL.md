# Security Remediation - Security Approval

**Project:** SimpleImageConverter  
**Approval Date:** June 14, 2026  
**Approver:** Casey Morgan, Security Specialist  
**Decision:** APPROVED  
**Scope:** Final V5 security approval after V2-V4 re-validation for the v1.0.0 default shipped profile.

## Approval Decision

Security remediation is APPROVED for the default shipped v1.0.0 profile: default-feature `img-convert`, `mesh-convert`, and `converter-gui`.

The prior V3 and V4 blockers are resolved. STL and PLY now perform parser preflight before third-party parser handoff, and GUI conversion helpers deny overwrite by default unless an explicit policy is supplied by a verified confirmation path.

## Validation Results

- V2 dependency and CI remediation: PASS.
- V3 parser and decode DoS remediation: PASS.
- V4 output path and write safety remediation: PASS.
- V5 final gate: PASS.

## Commands Run

- `cargo audit`: PASS with allowed unmaintained warnings only.
- `cargo deny check advisories licenses bans sources`: PASS.
- `cargo test -p common`: PASS.
- `cargo test -p img-core`: PASS.
- `cargo test -p mesh-core`: PASS.
- `cargo test -p converter-gui`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.

## Accepted Residual Risks

- `RUSTSEC-2026-0105` / `core2` unmaintained and yanked is accepted as a time-boxed residual dependency risk. It is transitive through `image -> ravif -> rav1e -> bitstream-io -> core2`; no direct replacement is available through the current dependency graph. Owner: Security Specialist. Review by: 2026-09-14. Removal condition: upstream dependency graph no longer requires `core2` or a maintained replacement becomes available.
- Existing allowed unmaintained advisories for `paste`, `derivative`, and `instant` remain accepted maintenance risks under `deny.toml`; no high/critical security advisory remains unapproved for shipped default feature sets.
- Optional feature sets excluded from the v1.0.0 default shipped profile (`mesh-core/step`, `mesh-core/step-opencascade`, `converter-gui/viewer-3d`, and `converter-gui-modern`) require separate security validation before public release.
- Release signing and notarization remain a tracked distribution-security milestone. This approval covers code-level remediation gates, not platform signing completion.

## Release Blockers

No security remediation blocker remains for the default shipped v1.0.0 profile.

Before public distribution, complete the separately tracked signing/notarization work and rerun the V5 gate on the release branch/artifacts.
