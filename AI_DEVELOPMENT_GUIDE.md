# AI Development Guidance

**For:** Claude AI, Claude Code, Cursor 2.2  
**Purpose:** Coordination and best practices for AI-assisted development

---

## Project Context

This is a Rust-based image and 3D mesh format converter project being developed through AI-assisted tooling. The project follows agile sprint methodology with clear phases and deliverables.

**Key Documents:**
- `README.md` - Project overview
- `IMPLEMENTATION_PLAN.md` - Sprint-by-sprint plan
- `Phase3_Architecture.md` - Detailed architecture
- `Phase2_Full_Specification.md` - Format specifications

---

## Tool-Specific Roles

### Claude AI
**Primary Role:** Architecture, design decisions, documentation

**Use for:**
- High-level architecture discussions
- Design pattern decisions
- Documentation review and creation
- Sprint planning and retrospectives
- Error handling strategy
- API design

**Best Practices:**
- Reference architecture documents before making design decisions
- Always consider cross-platform implications
- Prioritize type safety and Rust idioms
- Document reasoning for architectural choices

### Claude Code
**Primary Role:** Implementation, refactoring, testing

**Use for:**
- Writing Rust code following architecture
- Implementing format readers/writers
- Writing tests (unit, integration)
- Code refactoring
- Performance optimization
- Bug fixing

**Best Practices:**
- Follow architecture defined in Phase 3
- Use trait-based design for extensibility
- Write tests alongside implementation
- Run `cargo fmt` and `cargo clippy` before committing
- Reference `IMPLEMENTATION_PLAN.md` for current sprint tasks

### Cursor 2.2
**Primary Role:** Rapid prototyping, debugging, IDE integration

**Use for:**
- Quick prototypes to validate approaches
- Interactive debugging
- Exploring crate APIs
- Testing format conversions
- Performance profiling
- Incremental development

**Best Practices:**
- Use for experimentation before formal implementation
- Validate assumptions with small tests
- Document findings for team
- Share discoveries that affect architecture

---

## Development Workflow

### Sprint Workflow

1. **Sprint Start (Claude AI)**
   - Review sprint goals from `IMPLEMENTATION_PLAN.md`
   - Break down user stories into tasks
   - Create task checklist
   - Set Definition of Done

2. **Implementation (Claude Code)**
   - Follow architecture from Phase 3
   - Implement tasks one by one
   - Write tests alongside code
   - Update task checklist

3. **Validation (Cursor 2.2)**
   - Test implementations interactively
   - Debug issues
   - Performance checks
   - Edge case testing

4. **Sprint Review (Claude AI)**
   - Review completed work
   - Update documentation
   - Retrospective notes
   - Plan next sprint

### Code Review Checklist

Before marking any task complete:
- [ ] Follows architecture design
- [ ] Passes `cargo test`
- [ ] Passes `cargo clippy`
- [ ] Formatted with `cargo fmt`
- [ ] Documentation comments added
- [ ] Error handling implemented
- [ ] Tests written and passing
- [ ] No compilation warnings

---

## Architecture Adherence

### Core Principles

1. **Library-First Design**
   - `img-core` and `mesh-core` are libraries
   - CLI binaries are thin wrappers
   - GUI will import libraries directly

2. **Trait-Based Formats**
   - All formats implement `Reader` and `Writer` traits
   - Use `FormatRegistry` for format management
   - Never hard-code format handling

3. **Error Handling**
   - Use `ConversionError` from `common` crate
   - Provide context with errors
   - User-friendly error messages in CLI

4. **Data Structures**
   - `ImageData` for images
   - `Mesh` for 3D data
   - Builder pattern for construction
   - Immutable where possible

### File Organization

```
workspace/
├── common/           # Shared error types, utilities
├── img-core/         # 2D conversion library
├── img-convert/      # 2D CLI binary
├── mesh-core/        # 3D conversion library
├── mesh-convert/     # 3D CLI binary
└── converter-gui/    # Future GUI
```

**When creating new files:**
- Place in appropriate crate
- Follow module structure from Phase 3
- Update `mod.rs` or `lib.rs`
- Add corresponding tests

---

## Common Patterns

### Adding a New Image Format

1. Create `img-core/src/formats/{format}.rs`
2. Implement format struct
3. Implement `ImageReader` trait
4. Implement `ImageWriter` trait
5. Implement `ImageFormat` trait
6. Register in `FormatRegistry::new()`
7. Add to `ImageFormatType` enum
8. Write tests in same file or `tests/`

### Adding a New Mesh Format

1. Create `mesh-core/src/formats/{format}.rs`
2. Implement format struct
3. Implement `MeshReader` trait
4. Implement `MeshWriter` trait
5. Implement `MeshFormat` trait
6. Register in `FormatRegistry::new()`
7. Add to `MeshFormatType` enum
8. Write tests

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_conversion() {
        // Arrange
        let input = /* test data */;
        
        // Act
        let result = convert(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
```

---

## Communication Protocol

### Issue Tracking
- Use GitHub Issues for bugs and features
- Label appropriately (bug, enhancement, documentation)
- Reference sprint in title: `[Sprint 2] Bug: PNG transparency`

### Commit Messages
Format: `<type>(<scope>): <message>`

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance

Examples:
```
feat(img-core): Add PNG format support
fix(mesh-convert): Handle invalid STL files
docs(readme): Update installation instructions
test(img-core): Add JPEG quality tests
```

### Branch Strategy
- `main` - Stable, tested code
- `develop` - Active development
- `sprint-N` - Sprint-specific branches
- `feature/description` - Feature branches
- `fix/description` - Bug fix branches

---

## Sprint Coordination

### Current Sprint Tracking

**Location:** `IMPLEMENTATION_PLAN.md`

Each sprint has:
- Clear goal
- User stories
- Task breakdown
- Definition of Done
- Review checklist

### Daily Updates

Update sprint progress in one of:
- Issue comments
- Sprint board (GitHub Projects)
- Commit messages
- CHANGELOG.md

### Sprint Transitions

Between sprints:
1. Review completed sprint
2. Update CHANGELOG.md
3. Tag release if appropriate
4. Plan next sprint
5. Update README if needed

---

## Testing Strategy

### Test Levels

1. **Unit Tests** - In each module
   - Test individual functions
   - Mock dependencies
   - Fast execution

2. **Integration Tests** - In `tests/` directory
   - Test full conversions
   - Use real files
   - Test format combinations

3. **Benchmarks** - In `benches/` directory
   - Performance testing
   - Memory profiling
   - Regression detection

### Test Data

**Location:** `tests/test_data/`

Structure:
```
test_data/
├── images/
│   ├── sample.png
│   ├── transparent.png
│   └── gradient.jpg
└── meshes/
    ├── cube.stl
    ├── sphere.obj
    └── bunny.ply
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific test
cargo test test_png_to_jpg

# With output
cargo test -- --nocapture

# Integration only
cargo test --test integration

# Benchmarks
cargo bench
```

---

## Performance Guidelines

### Optimization Rules

1. **Measure First**
   - Profile before optimizing
   - Use `cargo bench` for baseline
   - Document improvements

2. **Prioritize Correctness**
   - Correct > Fast > Small
   - Don't sacrifice safety for speed

3. **Memory Management**
   - Avoid unnecessary clones
   - Use references where possible
   - Consider streaming for large files

4. **Release Builds**
   - Always test in release mode
   - Profile release builds
   - Check binary sizes

---

## Documentation Standards

### Code Documentation

```rust
/// Brief one-line summary
///
/// Longer description with details about behavior,
/// edge cases, and usage examples.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function returns an error and why
///
/// # Examples
///
/// ```
/// let result = function(arg1, arg2);
/// assert!(result.is_ok());
/// ```
pub fn function(arg1: Type1, arg2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

### README Updates

Update README when:
- New features added
- Usage changes
- Requirements change
- Status updates

### Changelog Updates

Update CHANGELOG.md after:
- Completing sprint
- Fixing bugs
- Adding features
- Making breaking changes

---

## Troubleshooting

### Common Issues

**Build fails:**
1. Check Rust version: `rustc --version`
2. Update dependencies: `cargo update`
3. Clean build: `cargo clean && cargo build`

**Tests fail:**
1. Check test data exists
2. Run individual test: `cargo test test_name -- --nocapture`
3. Check for platform-specific issues

**Cross-compilation issues:**
1. Verify target installed: `rustup target list --installed`
2. Check cross-compiler: `x86_64-w64-mingw32-gcc --version`
3. Set linker in `.cargo/config.toml`

**Performance issues:**
1. Profile with `cargo flamegraph`
2. Check allocations
3. Use release build for testing

---

## Quick Reference

### Essential Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test
cargo test --workspace

# Format and Lint
cargo fmt
cargo clippy

# Documentation
cargo doc --open

# Benchmarks
cargo bench

# Clean
cargo clean
```

### Important Files

- `Cargo.toml` - Workspace configuration
- `.gitignore` - Git ignore rules
- `IMPLEMENTATION_PLAN.md` - Sprint plan
- `Phase3_Architecture.md` - Detailed design
- `README.md` - Project overview

### Key Contacts

- Repository: (Private during development)
- Issues: GitHub Issues (when public)
- Discussions: GitHub Discussions (when public)

---

**This document evolves with the project. Update as needed.**

**Last Updated:** December 26, 2025  
**Version:** 1.0
