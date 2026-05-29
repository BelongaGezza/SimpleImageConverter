# Security Risk Register
## SimpleImageConverter Project

**Last Updated:** May 29, 2026  
**Owner:** Casey Morgan (Security Specialist)  
**Review Frequency:** Quarterly or on significant changes  
**Source:** `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md` (May 29, 2026) — accepted residual risks RISK-006–009

---

## Purpose

This document tracks identified security risks, their mitigation strategies, and current status. It supports compliance with UK Government Secure by Design Principle 1: "Create Responsibility for Cyber Security Risk."

---

## Risk Assessment Methodology

**Risk Levels:**
- **Critical**: Immediate action required, blocks production deployment
- **High**: Address within current sprint
- **Medium**: Address within next sprint
- **Low**: Monitor and address when resources available

**Risk Scoring:**
- **Likelihood**: Rare, Unlikely, Possible, Likely, Almost Certain
- **Impact**: Negligible, Minor, Moderate, Major, Catastrophic

---

## Active Risks

### RISK-001: Memory Exhaustion via Large File Input
**Status:** ✅ MITIGATED  
**Severity:** Critical → High (after mitigation)  
**Likelihood:** Possible  
**Impact:** Major (DoS, system crash)

**Description:**
Malicious files with extreme sizes or dimensions could cause memory exhaustion, leading to denial of service.

**Mitigation:**
- ✅ Resource limits implemented (`common/src/limits.rs`)
- ✅ File size validation at I/O layer (`common/src/io.rs`)
- ✅ Input size validation in all format readers (Phase 1)
- ✅ Dimension validation in image validation module

**Remaining Risk:**
- Medium: Streaming I/O not yet implemented (large files still loaded into memory) — tracked as **RISK-009**
- **Action:** Implement streaming I/O post-v1.0.0 (v1.2.0 roadmap)

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-002: Format Spoofing Attacks
**Status:** ✅ MITIGATED  
**Severity:** High → Low (after mitigation)  
**Likelihood:** Possible  
**Impact:** Moderate (incorrect processing, potential exploits)

**Description:**
Files with incorrect extensions could bypass format-specific security checks if format detection relies solely on file extension.

**Mitigation:**
- ✅ Two-stage format detection (extension + magic bytes)
- ✅ Format verification in CLI tools (`FormatRegistry::verify_format`)
- ✅ Removed `skip_format_check` flag (Phase 1)

**Remaining Risk:**
- Low: STL/OBJ/DXF rely on parse-validate at read rather than Stage-2 signature (ADR-003 tiered policy) — tracked as **RISK-007**
- **Action:** Document in release notes; monitor for spoofing reports

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-003: Integer Overflow in Dimension Calculations
**Status:** ✅ MITIGATED  
**Severity:** High → Low (after mitigation)  
**Likelihood:** Unlikely  
**Impact:** Major (memory corruption, crashes)

**Description:**
Calculating image/mesh buffer sizes without overflow protection could lead to memory corruption or crashes.

**Mitigation:**
- ✅ Checked arithmetic in `img-core/src/validation.rs`
- ✅ All dimension calculations use `checked_mul`
- ✅ Validation before allocation

**Remaining Risk:**
- Low: Some edge cases may remain
- **Action:** Add fuzz testing for dimension calculations

**Owner:** Casey Morgan  
**Last Reviewed:** December 27, 2025

---

### RISK-004: Dependency Vulnerabilities
**Status:** ⚠️ PARTIALLY MITIGATED  
**Severity:** High  
**Likelihood:** Possible  
**Impact:** Major (exploitable vulnerabilities)

**Description:**
Dependencies may contain known security vulnerabilities that could be exploited.

**Mitigation:**
- ✅ Automated `cargo audit` in CI/CD (Phase 2)
- ✅ `cargo deny` configuration (Phase 2)
- ⚠️ Manual dependency review process (needs documentation)

**Remaining Risk:**
- Medium: No automated blocking of PRs with vulnerable dependencies
- **Action:** Add PR gate to block merges with vulnerabilities

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-005: Path Traversal Attacks
**Status:** ⚠️ PARTIALLY MITIGATED  
**Severity:** Medium  
**Likelihood:** Unlikely  
**Impact:** Moderate (unauthorized file access)

**Description:**
Malicious file paths containing `..` could potentially access files outside intended directories.

**Mitigation:**
- ✅ Path validation in `common/src/validation.rs`
- ✅ Path canonicalization implemented (`SECURITY_AUDIT_v1.0.0.md`)

**Remaining Risk:**
- Low: residual edge cases monitored
- **Action:** Continue directory restriction hardening post-v1.0.0

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-006: img-core ResourceLimits CLI Propagation Gap
**Status:** ⚠️ PARTIALLY MITIGATED  
**Severity:** Medium  
**Likelihood:** Possible  
**Impact:** Moderate (DoS via oversized inputs when CLI limits not enforced)

**Description:**
The `img-convert` CLI exposes `--max-dimension` and `--max-file-size-mb`, but limits are not fully propagated into all `img-core` format readers via `get_reader_with_limits` (unlike the `mesh-core` pattern).

**Mitigation:**
- ✅ Resource limits enforced in `common` and mesh readers
- ✅ Sprint 13 Task 1.2 assigned to mirror `mesh-core` pattern
- **Action:** Complete `FormatRegistry::get_reader_with_limits` in img-core; target v1.0.1 if not closed in Sprint 13

**Remaining Risk:**
- Medium until Task 1.2 complete
- **Reference:** `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md` — accepted with documentation for v1.0.0 if documented

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-007: Mesh Format Stage-2 Detection Gaps (STL/OBJ/DXF)
**Status:** ⚠️ ACCEPTED (ADR-003)  
**Severity:** Medium → Low  
**Likelihood:** Unlikely  
**Impact:** Moderate (format misidentification before parse)

**Description:**
Per ADR-003 tiered policy, STL, OBJ, and DXF use extension-based Stage-1 identification with parse-validate at read time rather than signature verification at detection. This is an intentional trade-off documented in the architect review.

**Mitigation:**
- ✅ GLB, glTF, PLY, OFF have Stage-2 signature/heuristic checks (implemented)
- ✅ Parse-time validation in format readers with `ResourceLimits`
- ✅ Spoofing/mismatch tests for formats with clear headers
- **Action:** Document tiered policy in release notes; no change required for v1.0.0 ship

**Remaining Risk:**
- Low: accepted residual per architect review

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-008: Fuzz Testing Not in CI
**Status:** ⚠️ PARTIALLY MITIGATED  
**Severity:** Medium  
**Likelihood:** Unlikely  
**Impact:** Moderate (undiscovered parser edge cases)

**Description:**
LibFuzzer targets exist under `fuzz/` but are not executed in CI. Formal fuzzing coverage is incomplete (PNG, JPEG, STL only; BMP/GIF/OBJ/PLY not fuzzed).

**Mitigation:**
- ✅ Fuzz targets configured locally
- ✅ Unit and integration tests provide baseline coverage
- **Action:** Add fuzz CI job post-v1.0.0; expand targets per `TESTING_VALIDATION_REPORT.md`

**Remaining Risk:**
- Medium: accepted for v1.0.0 per architect review
- **Reference:** `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md` — post-v1.0.0 roadmap

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-009: Streaming I/O Not Implemented
**Status:** ⚠️ PARTIALLY MITIGATED  
**Severity:** High → Medium  
**Likelihood:** Possible  
**Impact:** Major (memory pressure on very large files)

**Description:**
All format readers load full file contents into memory. Resource limits cap maximum size, but streaming I/O would further reduce memory footprint for legitimate large files.

**Mitigation:**
- ✅ File size and dimension limits enforced (`common/src/limits.rs`)
- ✅ Document as known limitation in v1.0.0 release notes
- **Action:** Implement streaming I/O in v1.2.0 roadmap

**Remaining Risk:**
- Medium: RISK-001 residual; accepted with documentation for v1.0.0
- **Reference:** `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md`

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

## Closed Risks

### RISK-001-CLOSED: Missing Input Validation in Format Readers
**Status:** ✅ CLOSED  
**Severity:** Critical  
**Date Closed:** December 27, 2025

**Description:**
Format readers did not validate input size before parsing, allowing memory exhaustion attacks.

**Resolution:**
- Added input size validation to all format readers (Phase 1)
- All readers now check `data.len()` against ResourceLimits before parsing

**Owner:** Casey Morgan  
**Last Reviewed:** May 29, 2026

---

### RISK-006-CLOSED: Missing Security Logging
**Status:** ✅ CLOSED  
**Severity:** Medium  
**Date Closed:** January 2026

**Description:**
Security event logging was not implemented in earlier reviews.

**Resolution:**
- Security logging implemented and verified in `SECURITY_AUDIT_v1.0.0.md` (Grade A)
- Confirmed in `SECURITY_REVIEW_CRITICAL_DECEMBER_2025.md` follow-up

---

### RISK-007-CLOSED: Corrupted Output Files
**Status:** ✅ CLOSED  
**Severity:** Medium  
**Date Closed:** December 2025

**Description:**
Conversion errors could produce corrupted output files.

**Resolution:**
- Output file validation in CLI tools
- Round-trip validation for converted files

---

## Risk Trends

**Overall Security Posture:** Strong — Grade A audit; accepted residuals documented for v1.0.0

**Trends (May 29, 2026):**
- ✅ Critical risks: 0 active
- ✅ High risks: 0 blocking (RISK-009 accepted with limits + documentation)
- ⚠️ Medium risks: RISK-004, RISK-006, RISK-008 partially mitigated; accepted for v1.0.0 per architect review
- ✅ ADR-003 tiered mesh detection (RISK-007) accepted with documentation

**Next Review Date:** August 29, 2026

---

## Risk Mitigation Roadmap

### Q2 2026 (Current — v1.0.0 release)
- ✅ Security audit Grade A (`SECURITY_AUDIT_v1.0.0.md`)
- ✅ Architect review residual risks RISK-006–009 documented
- 🔄 Sprint 13: close RISK-006 via img-core `get_reader_with_limits` (Task 1.2)
- ⏳ Post-v1.0.0: fuzz CI (RISK-008), streaming I/O (RISK-009)

### Q3–Q4 2026
- v1.0.1: img-core limits if not closed in Sprint 13
- v1.2.0: streaming I/O implementation
- Expand fuzz targets and CI integration

---

## References

- **Secure by Design Guidance:** `docs/SECURE_BY_DESIGN_GUIDANCE.md`
- **Security Review:** `COMPREHENSIVE_ARCHITECTURE_SECURITY_REVIEW.md`
- **Architect Release Review:** `SYSTEM_ARCHITECT_V1.0.0_RELEASE_REVIEW.md`
- **Security Audit (v1.0.0):** `SECURITY_AUDIT_v1.0.0.md`
- **Architecture:** `Phase3_Architecture.md`

---

*This register should be reviewed quarterly or when significant architectural changes occur.*

