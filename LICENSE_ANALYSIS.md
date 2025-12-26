# License Compatibility Analysis
## Simple Image Converter Dependencies

**Project License:** MIT OR Apache-2.0 (dual license)  
**Analysis Date:** December 26, 2025  
**Purpose:** Ensure all dependencies are compatible with our dual licensing

---

## Summary

✅ **All dependencies are compatible with MIT OR Apache-2.0 dual licensing**

Our project uses dual licensing (MIT OR Apache-2.0), which provides maximum compatibility with the Rust ecosystem. All identified dependencies use permissive licenses that are compatible with both MIT and Apache-2.0.

---

## Core Dependencies License Analysis

### Common Utilities

| Crate | Version | License | Compatible | Notes |
|-------|---------|---------|------------|-------|
| anyhow | 1.0 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| thiserror | 1.0 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| clap | 4.5 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| indicatif | 0.17 | MIT | ✅ Yes | Compatible |
| log | 0.4 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| env_logger | 0.11 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |

### 2D Image Processing

| Crate | Version | License | Compatible | Notes |
|-------|---------|---------|------------|-------|
| image | 0.25 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| imageproc | 0.25 | MIT | ✅ Yes | Compatible |
| webp | 0.3 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| ravif | 0.11 | BSD-3-Clause | ✅ Yes | Permissive, compatible |
| exr | 1.72 | BSD-3-Clause | ✅ Yes | Permissive, compatible |
| resvg | 0.44 | MPL-2.0 | ✅ Yes | Weak copyleft, compatible |

### 3D Mesh Processing

| Crate | Version | License | Compatible | Notes |
|-------|---------|---------|------------|-------|
| stl_io | 0.7 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| tobj | 4.0 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| ply-rs | 0.1 | MIT | ✅ Yes | Compatible |
| gltf | 1.4 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| dxf | 0.6 | MIT | ✅ Yes | Compatible |

### STEP Support (truck)

| Crate | Version | License | Compatible | Notes |
|-------|---------|---------|------------|-------|
| truck-modeling | 0.4 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| truck-polymesh | 0.4 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| truck-stepio | 0.4 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |

### Utilities

| Crate | Version | License | Compatible | Notes |
|-------|---------|---------|------------|-------|
| nalgebra | 0.33 | Apache-2.0 | ✅ Yes | Compatible |

### Future GUI (Phase 4)

| Crate | Version | License | Compatible | Notes |
|-------|---------|---------|------------|-------|
| eframe | 0.29 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |
| egui | 0.29 | MIT OR Apache-2.0 | ✅ Yes | Identical to ours |

---

## License Type Breakdown

### MIT OR Apache-2.0 (Identical to ours)
- anyhow
- thiserror
- clap
- log
- env_logger
- image
- webp
- stl_io
- tobj
- gltf
- truck-modeling
- truck-polymesh
- truck-stepio
- eframe
- egui

**Count:** 15 crates  
**Status:** ✅ Perfect compatibility

### MIT Only
- indicatif
- imageproc
- ply-rs
- dxf

**Count:** 4 crates  
**Status:** ✅ Compatible (MIT is more permissive than Apache-2.0)

### Apache-2.0 Only
- nalgebra

**Count:** 1 crate  
**Status:** ✅ Compatible (we dual license)

### BSD-3-Clause
- ravif
- exr

**Count:** 2 crates  
**Status:** ✅ Compatible (permissive license)

### MPL-2.0 (Mozilla Public License)
- resvg

**Count:** 1 crate  
**Status:** ✅ Compatible (weak copyleft, file-level)

---

## License Compatibility Matrix

| Our License | Dependency License | Compatible | Notes |
|-------------|-------------------|------------|-------|
| MIT OR Apache-2.0 | MIT OR Apache-2.0 | ✅ Yes | Perfect match |
| MIT OR Apache-2.0 | MIT | ✅ Yes | More permissive |
| MIT OR Apache-2.0 | Apache-2.0 | ✅ Yes | Covered by our dual |
| MIT OR Apache-2.0 | BSD-3-Clause | ✅ Yes | Permissive |
| MIT OR Apache-2.0 | MPL-2.0 | ✅ Yes | Weak copyleft, file-level |

---

## Special License Considerations

### MPL-2.0 (resvg)
**Nature:** Weak copyleft (file-level)  
**Requirements:**
- Must preserve MPL-2.0 license notices
- Modifications to MPL-2.0 files must be shared under MPL-2.0
- Linking (as we do) is allowed without license propagation

**Impact on our project:** None - we use resvg as a library dependency, not modifying its source files. Our MIT OR Apache-2.0 license remains valid for our code.

**Action Required:** None - standard dependency usage

### BSD-3-Clause (ravif, exr)
**Nature:** Permissive  
**Requirements:**
- Retain copyright notices
- Include license text in binary distributions (handled by cargo)

**Impact on our project:** None - fully compatible

**Action Required:** None - licenses preserved in dependencies

---

## Required License Notices

### In Binary Distributions

When distributing compiled binaries, we should include:

1. **Our LICENSE file** (MIT OR Apache-2.0)
2. **THIRD_PARTY_LICENSES.txt** - Generated automatically by cargo-license or cargo-about

### Recommended Tools

```bash
# Option 1: cargo-license
cargo install cargo-license
cargo license --json > licenses.json

# Option 2: cargo-about (more detailed)
cargo install cargo-about
cargo about generate about.hbs > THIRD_PARTY_LICENSES.txt
```

### Manual Alternative

For releases, include a THIRD_PARTY_LICENSES.txt with:
```
This software includes the following third-party components:

[For each dependency]
- [Crate Name] ([Version]) - [License]
  Copyright (c) [Year] [Copyright Holder]
  [License URL or text]
```

---

## License Compliance Actions

### ✅ No Changes Required to Our License
Our dual MIT OR Apache-2.0 license is compatible with all dependencies.

### ✅ No Changes Required to Dependencies
All dependencies are already properly licensed and compatible.

### ⚠️ Actions Required for Distribution

**Before v1.0.0 Release:**

1. **Generate Third-Party License File**
   ```bash
   cargo install cargo-about
   cargo about generate about.hbs > THIRD_PARTY_LICENSES.txt
   ```

2. **Include in Distributions**
   - Add THIRD_PARTY_LICENSES.txt to release artifacts
   - Include in installer
   - Link from README

3. **Update README.md**
   Add section:
   ```markdown
   ## Third-Party Licenses
   
   This project uses several open-source libraries. See [THIRD_PARTY_LICENSES.txt](THIRD_PARTY_LICENSES.txt) for details.
   ```

4. **Verify License Headers**
   - Ensure all our source files have proper license headers
   - Use SPDX identifiers: `SPDX-License-Identifier: MIT OR Apache-2.0`

---

## Fallback STEP Option: OCCT FFI

**If we use Open CASCADE (OCCT):**

| Component | License | Compatible | Notes |
|-----------|---------|------------|-------|
| OCCT | LGPL-2.1 with exception | ⚠️ Complex | Requires analysis |
| opencascade-rs | MIT OR Apache-2.0 | ✅ Yes | Rust bindings OK |

**LGPL-2.1 Considerations:**
- Dynamic linking: Usually OK with LGPL
- Static linking: Requires source availability or relinking capability
- OCCT has a commercial exception that may allow static linking

**Recommendation:** If OCCT becomes necessary, consult with legal counsel regarding:
1. Distribution method (dynamic vs static linking)
2. OCCT's commercial exception applicability
3. Whether to offer source code for LGPL compliance

**Current Status:** Not applicable - using truck (MIT OR Apache-2.0) as primary STEP solution

---

## License Header Template

### For Rust Source Files

```rust
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors
//
// This file is part of Simple Image Converter.
//
// Licensed under either of
//
// * MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
// * Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//
// at your option.
```

### For Documentation Files

```markdown
<!-- 
SPDX-License-Identifier: MIT OR Apache-2.0
Copyright (c) 2025 Simple Image Converter Contributors
-->
```

---

## Verification Checklist

### Sprint 1 (Foundation)
- [x] Analyze all dependency licenses
- [ ] Add license headers to source files
- [ ] Install cargo-about tool
- [ ] Generate initial THIRD_PARTY_LICENSES.txt

### Before v0.1.0 Release
- [ ] Verify all licenses still compatible
- [ ] Update THIRD_PARTY_LICENSES.txt
- [ ] Include license file in distribution
- [ ] Document license compliance in README

### Before v1.0.0 Release
- [ ] Legal review of license compliance
- [ ] Verify installer includes licenses
- [ ] Check all transitive dependencies
- [ ] Update copyright years
- [ ] Final license audit

---

## FAQ

**Q: Can we change our license later?**  
A: Yes, but requires agreement from all contributors. Our dual license provides maximum flexibility.

**Q: Do contributors need to sign a CLA?**  
A: Not required for MIT OR Apache-2.0, but can add clarity. Consider for v1.0.0.

**Q: Can commercial projects use our code?**  
A: Yes, both MIT and Apache-2.0 allow commercial use.

**Q: What if a dependency changes its license?**  
A: We can pin to the last compatible version or find an alternative. Monitor updates carefully.

**Q: Do we need to include dependency licenses in the binary?**  
A: Technically yes for proper attribution. Use cargo-about to automate this.

---

## Monitoring Strategy

### Continuous Monitoring
1. **Review dependencies quarterly**
2. **Check for license changes in updates**
3. **Audit before each major release**
4. **Use `cargo deny` to catch license issues early**

### Setup cargo-deny

```toml
# deny.toml
[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "MPL-2.0",
]
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-3.0",
]
copyleft = "warn"
```

```bash
cargo install cargo-deny
cargo deny check licenses
```

---

## Conclusion

✅ **All dependencies are compatible with our MIT OR Apache-2.0 dual license.**

**Key Takeaways:**
1. No license conflicts exist
2. No changes to our license needed
3. Standard Rust ecosystem licenses throughout
4. Must include third-party license notices in distributions
5. truck (STEP) uses compatible license; OCCT fallback requires careful analysis if needed

**Recommendation:** Proceed with implementation. Add license compliance steps to Sprint 1 and release checklists.

---

## References

- [MIT License](https://opensource.org/licenses/MIT)
- [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [BSD 3-Clause](https://opensource.org/licenses/BSD-3-Clause)
- [MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/)
- [Rust API Guidelines - Licensing](https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive)
- [cargo-about Documentation](https://github.com/EmbarkStudios/cargo-about)

---

**Status:** ✅ Analysis Complete - No License Issues  
**Next Action:** Add license headers and generate THIRD_PARTY_LICENSES.txt in Sprint 1  
**Updated:** December 26, 2025
