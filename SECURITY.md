# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please report it via one of the following methods:

1. **Email:** [Security Email] (to be configured)
2. **GitHub Security Advisory:** Use the "Report a vulnerability" button on the repository's Security tab

### What to Include

When reporting a vulnerability, please include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)
- Your contact information

### Response Timeline

- **Acknowledgment:** Within 48 hours
- **Initial Assessment:** Within 7 days
- **Fix Timeline:** Depends on severity (see below)
- **Public Disclosure:** After fix is released (coordinated disclosure)

### Severity Levels

- **Critical:** Remote code execution, data breach, authentication bypass
  - **Fix Timeline:** 24-48 hours
- **High:** Privilege escalation, information disclosure
  - **Fix Timeline:** 7 days
- **Medium:** Denial of service, limited information disclosure
  - **Fix Timeline:** 30 days
- **Low:** Minor issues, best practice violations
  - **Fix Timeline:** Next release cycle

## Verifying Package Integrity

All releases include SHA256 checksums for binary integrity verification. **Always verify checksums before installing.**

### Windows

1. Download the package and `checksums-windows.txt` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Verify the checksum:
   ```powershell
   Get-FileHash simpleimageconverter-*-windows-x64.zip -Algorithm SHA256
   ```
3. Compare with the checksum in `checksums-windows.txt`

### macOS

1. Download the package and `checksums-macos.txt` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Verify the checksum:
   ```bash
   shasum -a 256 simpleimageconverter-*-macos-*.tar.gz
   ```
3. Compare with the checksum in `checksums-macos.txt`

### Linux

1. Download the package and `checksums-linux.txt` from [Releases](https://github.com/BelongaGezza/SimpleImageConverter/releases)
2. Verify the checksum:
   ```bash
   sha256sum -c checksums-linux.txt
   ```
   Or manually:
   ```bash
   sha256sum simpleimageconverter-*-linux-x64.tar.gz
   ```
3. Compare with the checksum in `checksums-linux.txt`

## Official Distribution Channels

**Only download SimpleImageConverter from these official sources:**

1. **GitHub Releases** (Primary)
   - URL: `https://github.com/BelongaGezza/SimpleImageConverter/releases`
   - Package names: `simpleimageconverter-{version}-{platform}-{arch}.{ext}`

2. **Package Managers** (Future - v0.3.0+)
   - **Windows:** `winget install BelongaGezza.SimpleImageConverter`
   - **macOS:** `brew install --cask simpleimageconverter` (Homebrew Cask)
   - **Linux:** `apt install simpleimageconverter` (DEB package)

### Warning: Typosquatting

Be cautious of packages with similar names:
- ✅ **Official:** `BelongaGezza.SimpleImageConverter` (winget)
- ✅ **Official:** `simpleimageconverter` (Homebrew Cask)
- ❌ **Suspicious:** `simple-image-converter`, `simpleimageconverter-cli`, etc.

If you find a suspicious package, please report it via the security reporting process.

## Security Best Practices

### For Users

1. **Always verify checksums** before installing
2. **Download from official sources only** (GitHub Releases or official package managers)
3. **Keep software updated** to the latest version
4. **Report security issues** through the proper channels
5. **Review file permissions** after installation

### For Developers

1. **Never commit secrets** (API keys, certificates, passwords)
2. **Use dependency scanning** (`cargo audit`, `cargo deny`)
3. **Follow secure coding practices** (input validation, resource limits)
4. **Test with malicious inputs** before release
5. **Review all dependencies** for known vulnerabilities

## Security Features

### Current (v0.2.0)

- ✅ SHA256 checksums for all packages
- ✅ Input validation in packaging scripts
- ✅ Workflow permission restrictions
- ✅ Release tag verification
- ✅ Resource limits in binaries

### Planned (v0.3.0)

- ⏳ GPG signing for Linux packages
- ⏳ Reproducible builds
- ⏳ SBOM (Software Bill of Materials) generation
- ⏳ Dependency security scanning in CI/CD

### Planned (v0.4.0+)

- ⏳ Code signing for Windows/macOS
- ⏳ Notarization for macOS
- ⏳ SLSA (Supply-chain Levels for Software Artifacts) Level 2+ attestations
- ⏳ Hardware security module (HSM) support

## Known Security Considerations

### Binary Distribution

- Binaries are statically linked (reduces dependency attack surface)
- No external dependencies required at runtime
- Resource limits enforced (file size, memory usage)

### Build Process

- Builds run on GitHub-hosted runners (isolated environment)
- Workflow permissions restricted to minimum required
- Release tags verified before packaging
- Checksums generated automatically

### Supply Chain

- Dependencies are audited before release
- Official package names documented
- Typosquatting monitoring (planned)

## Security Updates

Security updates are released as:
- **Critical/High:** Immediate patch release (e.g., 0.2.1)
- **Medium/Low:** Included in next planned release

All security fixes are documented in release notes.

## Compliance

- **CVE Reporting:** Via GitHub Security Advisories
- **Disclosure Policy:** Coordinated disclosure (90-day timeline)
- **SBOM:** Planned for v0.3.0 (SPDX or CycloneDX format)

## Contact

For security-related questions or to report vulnerabilities:

- **GitHub Security Advisory:** Use the "Report a vulnerability" button
- **Email:** [To be configured]

---

**Last Updated:** December 29, 2025  
**Next Review:** Before v0.2.0 release

