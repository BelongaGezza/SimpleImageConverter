# Security Risk Register
## SimpleImageConverter Project

**Last Updated:** January 27, 2025  
**Owner:** Casey Morgan (Security Specialist)  
**Review Frequency:** Quarterly or on significant changes

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
- Medium: Streaming I/O not yet implemented (large files still loaded into memory)
- **Action:** Implement streaming I/O for Phase 4

**Owner:** Casey Morgan  
**Last Reviewed:** January 27, 2025

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
- Low: Some formats may not have reliable magic bytes
- **Action:** Enhance magic byte detection for all formats

**Owner:** Casey Morgan  
**Last Reviewed:** January 27, 2025

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
**Last Reviewed:** January 27, 2025

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
**Last Reviewed:** January 27, 2025

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
- ⚠️ No canonicalization or directory restriction

**Remaining Risk:**
- Medium: Path validation doesn't prevent all traversal attempts
- **Action:** Implement path canonicalization and directory restrictions

**Owner:** Casey Morgan  
**Last Reviewed:** January 27, 2025

---

### RISK-006: Missing Security Logging
**Status:** ❌ NOT MITIGATED  
**Severity:** Medium  
**Likelihood:** Possible  
**Impact:** Moderate (undetected attacks, no audit trail)

**Description:**
No security event logging means failed validations and suspicious inputs are not tracked.

**Mitigation:**
- ❌ Security logging not yet implemented
- **Action:** Implement security event logging (Phase 2)

**Remaining Risk:**
- High: No visibility into security events
- **Action:** Add security logging for failed validations

**Owner:** Casey Morgan  
**Last Reviewed:** January 27, 2025

---

### RISK-007: Corrupted Output Files
**Status:** ✅ MITIGATED  
**Severity:** Medium → Low (after mitigation)  
**Likelihood:** Unlikely  
**Impact:** Minor (data corruption)

**Description:**
Conversion errors could produce corrupted output files that fail validation in downstream systems.

**Mitigation:**
- ✅ Output file validation in CLI tools (Phase 1)
- ✅ Round-trip validation for converted files

**Remaining Risk:**
- Low: Some edge cases may not be caught
- **Action:** Enhance output validation with format-specific checks

**Owner:** Casey Morgan  
**Last Reviewed:** January 27, 2025

---

## Closed Risks

### RISK-001-CLOSED: Missing Input Validation in Format Readers
**Status:** ✅ CLOSED  
**Severity:** Critical  
**Date Closed:** January 27, 2025

**Description:**
Format readers did not validate input size before parsing, allowing memory exhaustion attacks.

**Resolution:**
- Added input size validation to all format readers (Phase 1)
- All readers now check `data.len()` against ResourceLimits before parsing

---

## Risk Trends

**Overall Security Posture:** Improving

**Trends:**
- ✅ Critical risks reduced from 2 to 0
- ✅ High risks reduced from 3 to 1
- ⚠️ Medium risks: 2 active, 1 partially mitigated
- ✅ Low risks: 3 active (acceptable)

**Next Review Date:** April 27, 2025

---

## Risk Mitigation Roadmap

### Q1 2025 (Current)
- ✅ Phase 1: Critical security fixes (COMPLETE)
- 🔄 Phase 2: Security infrastructure (IN PROGRESS)
  - ✅ CI/CD security scanning
  - ⏳ Security logging
  - ⏳ Security risk register (COMPLETE)

### Q2 2025
- Phase 3: Architecture enhancements
- Phase 4: Testing and documentation
- Path traversal protection
- Streaming I/O implementation

---

## References

- **Secure by Design Guidance:** `docs/SECURE_BY_DESIGN_GUIDANCE.md`
- **Security Review:** `COMPREHENSIVE_ARCHITECTURE_SECURITY_REVIEW.md`
- **Architecture:** `Phase3_Architecture.md`

---

*This register should be reviewed quarterly or when significant architectural changes occur.*

