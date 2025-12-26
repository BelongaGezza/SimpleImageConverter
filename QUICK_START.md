# Quick Start - Repository Initialization

**Purpose:** Step-by-step guide to initialize the GitHub repository and begin Sprint 1

---

## Prerequisites

- [ ] Git installed
- [ ] GitHub account with access
- [ ] Rust toolchain installed (1.70+)
- [ ] Repository documentation files ready

---

## Step 1: Create GitHub Repository

### Via GitHub Web Interface

1. Go to https://github.com/new
2. Repository settings:
   - **Name:** `SimpleImageConverter` (or your preferred name)
   - **Description:** "High-performance Rust CLI toolkit for converting between image and 3D mesh formats"
   - **Visibility:** ⚠️ **Private** (during development)
   - **Initialize:** ❌ Do NOT add README, .gitignore, or license (we have them)

3. Click "Create repository"

---

## Step 2: Initialize Local Repository

```bash
# Navigate to project directory
cd /path/to/project

# Initialize Git
git init

# Add all documentation files
git add README.md
git add LICENSE
git add .gitignore
git add IMPLEMENTATION_PLAN.md
git add CONTRIBUTING.md
git add CHANGELOG.md
git add AI_DEVELOPMENT_GUIDE.md
git add PROJECT_SUMMARY.md

# Add all specification documents
git add Phase2_Full_Specification.md
git add Phase2.1_Decisions.md
git add Phase3_Architecture.md
git add Language_Comparison.md
git add POC_2D_Results.md

# First commit
git commit -m "Initial commit: Project foundation and documentation

- Add comprehensive README with project overview
- Add MIT License
- Add 12-sprint implementation plan (23 weeks)
- Add architecture documentation (Phase 2 & 3)
- Add AI development guide for Claude/Cursor coordination
- Add contributing guidelines and changelog
- Configure .gitignore for Rust project

Sprint 1 ready to begin."
```

---

## Step 3: Connect to GitHub

```bash
# Add remote (replace with your repository URL)
git remote add origin https://github.com/BelongaGezza/SimpleImageConverter.git

# Verify remote
git remote -v

# Push to GitHub
git branch -M main
git push -u origin main
```

---

## Step 4: Configure Repository Settings

### 4.1 Branch Protection (Optional for now)
- Go to Settings > Branches
- Consider adding branch protection after Sprint 1

### 4.2 GitHub Projects (Sprint Board)
1. Go to Projects tab
2. Create new project: "Sprint Board"
3. Choose "Board" layout
4. Columns: "Backlog", "Sprint", "In Progress", "Review", "Done"

### 4.3 Issues (Sprint 1 Tasks)
Create issues for Sprint 1 tasks from `IMPLEMENTATION_PLAN.md`:

```
Title: [Sprint 1] Create Cargo workspace structure
Labels: sprint-1, setup
Milestone: Sprint 1

Description:
Create workspace structure:
- common/ crate
- img-core/ crate
- img-convert/ crate
- mesh-core/ crate
- mesh-convert/ crate

See IMPLEMENTATION_PLAN.md Sprint 1, Day 3-4
```

### 4.4 Milestones
Create milestones for each sprint:
- Sprint 1: Foundation (Weeks 1-2)
- Sprint 2: img-convert Core (Weeks 3-4)
- Sprint 3: mesh-convert Core (Weeks 5-6)
- ... etc.

---

## Step 5: Create Initial Workspace Structure

```bash
# Create workspace root Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = [
    "common",
    "img-core",
    "img-convert",
    "mesh-core",
    "mesh-convert",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
authors = ["Your Name <you@example.com>"]
license = "MIT"
repository = "https://github.com/BelongaGezza/SimpleImageConverter"

[workspace.dependencies]
anyhow = "1.0"
thiserror = "1.0"
clap = { version = "4.5", features = ["derive"] }

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
panic = "abort"
EOF

# Create each crate
cargo new --lib common
cargo new --lib img-core
cargo new --bin img-convert
cargo new --lib mesh-core
cargo new --bin mesh-convert

# Verify workspace
cargo check
```

---

## Step 6: Set Up CI/CD

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
    - name: Run tests
      run: cargo test --workspace

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
        components: rustfmt
    - name: Check formatting
      run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
        components: clippy
    - name: Run clippy
      run: cargo clippy --workspace -- -D warnings

  build-windows:
    name: Build Windows
    runs-on: windows-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
    - name: Build
      run: cargo build --release
```

Commit and push:
```bash
git add .github/
git add Cargo.toml
git add common/ img-core/ img-convert/ mesh-core/ mesh-convert/
git commit -m "feat: Initialize Cargo workspace with CI/CD

- Create workspace structure with 5 crates
- Add GitHub Actions CI workflow
- Configure release profile for optimized builds

Sprint 1: Day 3-8 complete"
git push
```

---

## Step 7: Verify Setup

```bash
# Check workspace builds
cargo check --workspace

# Run initial tests (should pass even with empty crates)
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint check
cargo clippy --workspace

# View CI status on GitHub
# Go to Actions tab and verify CI runs
```

---

## Step 8: Begin Sprint 1 Development

### Sprint 1 Remaining Tasks

Refer to `IMPLEMENTATION_PLAN.md` Sprint 1 for detailed tasks:

**Day 9-10: Documentation**
- Create docs/ folder
- Add ARCHITECTURE.md (can copy from Phase3_Architecture.md)
- Add examples/ folder

**Completion:**
- All Sprint 1 tasks done
- CI/CD passing
- Ready for Sprint 2

---

## Checklist Summary

### Repository Setup
- [ ] GitHub repository created (private)
- [ ] Local git initialized
- [ ] Initial commit with documentation
- [ ] Remote connected and pushed
- [ ] Projects board created
- [ ] Milestones configured

### Workspace Setup
- [ ] Cargo workspace created
- [ ] All 5 crates initialized
- [ ] Workspace compiles
- [ ] CI/CD configured and passing

### Documentation
- [ ] README.md in repo
- [ ] LICENSE in repo
- [ ] All planning docs in repo
- [ ] AI guide accessible
- [ ] Changelog tracking started

### Team Coordination
- [ ] Claude AI has access to repo
- [ ] Claude Code can commit
- [ ] Cursor 2.2 configured
- [ ] Sprint 1 tasks defined

---

## Troubleshooting

### "Failed to push"
- Check remote URL: `git remote -v`
- Verify credentials
- Try SSH instead of HTTPS

### "Workspace doesn't compile"
- Check Rust version: `rustc --version`
- Update toolchain: `rustup update`
- Clean and rebuild: `cargo clean && cargo check`

### "CI workflow fails"
- Check GitHub Actions logs
- Verify workflow YAML syntax
- Ensure all dependencies available

---

## Next Steps

1. ✅ Complete repository setup
2. ✅ Verify CI/CD passing
3. → Begin Sprint 1 implementation
4. → Regular commits with Sprint 1 tasks
5. → Sprint 1 review at end of Week 2

---

## Resources

- **Project Docs:** All .md files in repository
- **Implementation Plan:** `IMPLEMENTATION_PLAN.md`
- **Architecture:** `Phase3_Architecture.md`
- **AI Guide:** `AI_DEVELOPMENT_GUIDE.md`
- **Rust Book:** https://doc.rust-lang.org/book/
- **Cargo Guide:** https://doc.rust-lang.org/cargo/

---

**Good luck with Sprint 1!** 🚀

Remember: The foundation is critical. Take time to set up properly.

---

_Last Updated: December 26, 2025_
_Sprint: 1 (Foundation)_
