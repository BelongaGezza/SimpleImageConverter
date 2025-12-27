# Threat Model
## SimpleImageConverter Project

**Date:** January 27, 2025  
**Author:** Casey Morgan (Security Specialist)  
**Review Frequency:** Quarterly or when architecture changes

---

## Purpose

This document defines the threat model for SimpleImageConverter, identifying potential attackers, attack vectors, and mitigation strategies. It supports compliance with UK Government Secure by Design Principle 3: "Adopt a Risk-Driven Approach."

---

## System Overview

SimpleImageConverter is a command-line tool for converting between image and 3D mesh formats. It processes untrusted file input from users and must protect against malicious files designed to cause denial of service, memory exhaustion, or system compromise.

---

## Attack Surface

### Input Vectors

1. **File Input**
   - Image files (PNG, JPEG, BMP, GIF, etc.)
   - Mesh files (STL, OBJ, PLY, etc.)
   - All files are considered untrusted

2. **Command-Line Arguments**
   - File paths (potential path traversal)
   - Format specifications
   - Resource limit overrides

3. **Dependencies**
   - Third-party format parsing libraries
   - Potential vulnerabilities in dependencies

---

## Threat Actors

### 1. Malicious User
**Capabilities:**
- Can provide any file as input
- Can craft malicious files with extreme sizes/dimensions
- Can attempt format spoofing

**Motivation:**
- Denial of service
- System compromise
- Data exfiltration

**Risk Level:** High

### 2. Compromised System
**Capabilities:**
- Automated attack generation
- Large-scale testing of vulnerabilities
- Supply chain attacks

**Motivation:**
- Widespread exploitation
- Botnet recruitment

**Risk Level:** Medium

### 3. Accidental User
**Capabilities:**
- May provide corrupted files
- May use incorrect file paths

**Motivation:**
- None (accidental)

**Risk Level:** Low

---

## Attack Vectors

### AV-001: Memory Exhaustion via Large Files
**Description:** Attacker provides files with extreme sizes to exhaust system memory.

**Attack Scenario:**
1. Attacker creates a file larger than system memory
2. Tool attempts to load entire file into memory
3. System runs out of memory, causing crash or denial of service

**Mitigation:**
- ✅ Resource limits enforced at I/O layer
- ✅ Input size validation in all format readers
- ✅ File size checked before reading

**Status:** ✅ MITIGATED

---

### AV-002: Integer Overflow in Dimension Calculations
**Description:** Attacker provides files with extreme dimensions that cause integer overflow.

**Attack Scenario:**
1. Attacker creates file with dimensions like 65535x65535
2. Tool calculates buffer size: `width * height * channels`
3. Integer overflow causes incorrect memory allocation
4. Buffer overflow or memory corruption

**Mitigation:**
- ✅ Checked arithmetic in all dimension calculations
- ✅ Resource limits on image dimensions
- ✅ Validation before allocation

**Status:** ✅ MITIGATED

---

### AV-003: Format Spoofing
**Description:** Attacker provides file with incorrect extension to bypass format-specific security checks.

**Attack Scenario:**
1. Attacker creates JPEG file but names it `.png`
2. Tool uses extension-based detection only
3. JPEG parser may have different security properties than PNG parser
4. Potential vulnerability exploitation

**Mitigation:**
- ✅ Two-stage format detection (extension + magic bytes)
- ✅ Format verification before processing
- ✅ Mandatory format verification (no bypass flag)

**Status:** ✅ MITIGATED

---

### AV-004: Path Traversal
**Description:** Attacker provides file path with `..` to access files outside intended directory.

**Attack Scenario:**
1. Attacker provides path like `../../../etc/passwd`
2. Tool processes file from unintended location
3. Unauthorized file access or modification

**Mitigation:**
- ✅ Path validation in `common/src/validation.rs`
- ⚠️ Partial: No canonicalization or directory restrictions yet
- **Action:** Implement path canonicalization (Phase 4+)

**Status:** ⚠️ PARTIALLY MITIGATED

---

### AV-005: Malformed File Parsing
**Description:** Attacker provides malformed files designed to cause parser crashes or panics.

**Attack Scenario:**
1. Attacker creates file with valid header but corrupted data
2. Parser encounters unexpected data structure
3. Parser panics or crashes, causing denial of service

**Mitigation:**
- ✅ All parsers return `Result` types (no panics on bad input)
- ✅ Error handling in all format readers
- ✅ Graceful error messages

**Status:** ✅ MITIGATED

---

### AV-006: Dependency Vulnerabilities
**Description:** Attacker exploits known vulnerabilities in third-party dependencies.

**Attack Scenario:**
1. Dependency has known CVE
2. Attacker crafts input to trigger vulnerability
3. System compromise or data exfiltration

**Mitigation:**
- ✅ Automated `cargo audit` in CI/CD
- ✅ `cargo deny` configuration
- ✅ Regular dependency updates
- ⚠️ No automated blocking of vulnerable dependencies in PRs yet

**Status:** ⚠️ PARTIALLY MITIGATED

---

### AV-007: Resource Exhaustion via Mesh Complexity
**Description:** Attacker provides mesh files with excessive vertex/face counts.

**Attack Scenario:**
1. Attacker creates mesh with millions of vertices
2. Tool attempts to process entire mesh
3. CPU or memory exhaustion

**Mitigation:**
- ✅ Resource limits on vertex/face counts
- ✅ Validation before processing
- ✅ Limits configurable but with safe defaults

**Status:** ✅ MITIGATED

---

## Security Controls

### Defense in Depth Layers

1. **I/O Layer**
   - File size validation
   - Path validation
   - File existence checks

2. **Format Detection Layer**
   - Extension-based detection
   - Magic byte verification
   - Format mismatch detection

3. **Parser Layer**
   - Input size validation
   - Resource limit checks
   - Error handling (no panics)

4. **Data Validation Layer**
   - Dimension validation
   - Integer overflow protection
   - Data integrity checks

5. **Output Layer**
   - Output file validation
   - Round-trip verification

---

## Risk Assessment

### High Risk (Mitigated)
- ✅ Memory exhaustion attacks
- ✅ Integer overflow attacks
- ✅ Format spoofing attacks

### Medium Risk (Partially Mitigated)
- ⚠️ Path traversal (needs canonicalization)
- ⚠️ Dependency vulnerabilities (needs PR blocking)

### Low Risk (Acceptable)
- ✅ Malformed file handling
- ✅ Resource exhaustion (mesh complexity)

---

## Threat Intelligence

### Known CVEs in Image/Mesh Processing
- Monitor RustSec advisories
- Track CVEs in `image`, `stl_io`, `tobj`, `ply-rs` crates
- Review dependency changelogs

### Attack Patterns
- Large file attacks (common in file processing tools)
- Format confusion attacks (common in converters)
- Path traversal (common in file tools)

---

## Incident Response

### Security Event Detection
- Security logging captures failed validations
- Format mismatches logged
- Resource limit violations logged

### Response Procedures
1. Review security logs
2. Identify attack pattern
3. Update resource limits if needed
4. Patch vulnerabilities if found
5. Document incident in risk register

---

## Future Threats

### Streaming I/O
**Threat:** Current implementation loads entire file into memory. Very large files could still cause issues.

**Mitigation Plan:** Implement streaming I/O for files >100MB (Phase 4+)

### Network Input
**Threat:** If tool is extended to accept network input, new attack vectors emerge.

**Mitigation Plan:** Network input would require additional security controls (not currently planned)

### Plugin System
**Threat:** If plugin system is added, third-party plugins could introduce vulnerabilities.

**Mitigation Plan:** Plugin system would require sandboxing and code signing (not currently planned)

---

## References

- **Secure by Design Guidance:** `docs/SECURE_BY_DESIGN_GUIDANCE.md`
- **Security Risk Register:** `SECURITY_RISK_REGISTER.md`
- **Architecture Review:** `COMPREHENSIVE_ARCHITECTURE_SECURITY_REVIEW.md`

---

*This threat model should be reviewed quarterly or when significant architectural changes occur.*

