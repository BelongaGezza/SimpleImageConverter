# Testing Guide
## SimpleImageConverter Project

**Last Updated:** January 27, 2025

---

## Overview

SimpleImageConverter uses a comprehensive testing strategy covering unit tests, integration tests, security tests, and fuzz testing.

---

## Running Tests

### All Tests

```bash
# Run all tests in workspace
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture
```

### By Category

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# Security tests
cargo test --workspace security

# Integration tests
cargo test --workspace integration
```

### By Crate

```bash
# Test specific crate
cargo test -p img-core
cargo test -p mesh-core
cargo test -p common
```

---

## Test Organization

### Unit Tests

Located in each crate's `src/` directory:

- `common/src/*.rs` - Unit tests for common utilities
- `img-core/src/**/*.rs` - Unit tests for image formats
- `mesh-core/src/**/*.rs` - Unit tests for mesh formats

### Integration Tests

Located in `tests/` directories:

- `img-core/tests/integration.rs` - Image conversion integration tests
- `mesh-core/tests/integration.rs` - Mesh conversion integration tests
- `tests/integration/cli_tests.rs` - CLI tool integration tests

### Security Tests

Located in `tests/security.rs`:

- `img-core/tests/security.rs` - Image format security tests
- `mesh-core/tests/security.rs` - Mesh format security tests

**Security Test Coverage:**
- Oversized input rejection
- Malformed file handling
- Format spoofing detection
- Integer overflow protection
- Resource limit enforcement

---

## Fuzz Testing

Fuzz testing uses `cargo-fuzz` to test format parsers with arbitrary input.

### Setup

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Install libfuzzer (Linux)
sudo apt-get install libfuzzer-dev

# Install libfuzzer (macOS)
brew install llvm
```

### Running Fuzz Tests

```bash
cd fuzz

# Fuzz PNG reader
cargo fuzz run fuzz_png_reader

# Fuzz JPEG reader
cargo fuzz run fuzz_jpeg_reader

# Fuzz STL reader
cargo fuzz run fuzz_stl_reader
```

### Fuzz Test Targets

- `fuzz_png_reader` - Tests PNG format reader with arbitrary input
- `fuzz_jpeg_reader` - Tests JPEG format reader with arbitrary input
- `fuzz_stl_reader` - Tests STL format reader with arbitrary input

**Goal:** Ensure format readers never panic on arbitrary input, only return errors.

---

## Test Coverage

### Current Coverage

- ✅ Unit tests for all format readers/writers
- ✅ Integration tests for format conversions
- ✅ Security tests for input validation
- ✅ Resource limit tests
- ✅ Error handling tests
- ⏳ CLI integration tests (require built binaries)
- ⏳ Fuzz testing (optional, requires setup)

### Coverage Goals

- **Unit Tests:** 80%+ coverage
- **Integration Tests:** All format combinations
- **Security Tests:** All attack vectors
- **Fuzz Tests:** All format readers

---

## Writing Tests

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert!(result.is_ok());
    }
}
```

### Integration Test Example

```rust
use img_core::{ImageConverter, FormatRegistry, ImageFormat};

#[test]
fn test_conversion() {
    let reader = FormatRegistry::get_reader(ImageFormat::Png)?;
    let writer = FormatRegistry::get_writer(ImageFormat::Jpeg)?;
    let converter = ImageConverter::new();
    // ... test conversion
}
```

### Security Test Example

```rust
#[test]
fn test_reject_oversized_input() {
    let format = PngFormat::new();
    let oversized_data = vec![0u8; 200 * 1024 * 1024]; // 200MB
    let result = format.read(&oversized_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds limit"));
}
```

---

## CI/CD Testing

Tests run automatically in CI/CD:

- **Unit Tests:** Run on every push/PR
- **Integration Tests:** Run on every push/PR
- **Security Tests:** Run on every push/PR
- **Fuzz Tests:** Optional, run manually

See `.github/workflows/ci.yml` for details.

---

## Test Data

Test data should be:

- Small (for fast tests)
- Representative (cover common cases)
- Edge cases (boundary conditions)
- Security-focused (malformed, oversized)

**Note:** Large test files should be in `tests/test_data/` (not committed to repo if >1MB)

---

## Debugging Tests

### Run Single Test

```bash
cargo test test_name

# With output
cargo test test_name -- --nocapture
```

### Run Tests in Specific File

```bash
cargo test --test integration
cargo test --test security
```

### Verbose Output

```bash
cargo test --workspace -- --nocapture --test-threads=1
```

---

## References

- **Rust Testing:** https://doc.rust-lang.org/book/ch11-00-testing.html
- **Cargo Fuzz:** https://github.com/rust-fuzz/cargo-fuzz
- **Security Testing:** `docs/THREAT_MODEL.md`

---

*For questions about testing, see CONTRIBUTING.md*

