# Claude Code Project Configuration

## Project: SimpleImageConverter
A Rust-based image and 3D mesh format conversion utility.

## Available Team Agents

This project uses specialized AI agents for different development tasks. To invoke an agent, use the `/agent` command followed by the agent name:

### Agent Commands

| Command | Agent | Use For |
|---------|-------|---------|
| `/agent architect` | System Architect | Architecture decisions, design reviews, technology selection |
| `/agent senior` | Senior Engineer | Core implementation, code reviews, mentoring |
| `/agent junior-2d` | Junior Engineer (2D) | PNG, JPEG, BMP, GIF format implementations |
| `/agent junior-3d` | Junior Engineer (3D) | PLY, OFF, glTF mesh format implementations |
| `/agent security` | Security Specialist | Security reviews, vulnerability analysis, input validation |
| `/agent docs` | Documentation Specialist | API docs, examples, user guides |
| `/agent research` | Researcher | Ecosystem monitoring, library evaluation, rust-resources.md |
| `/agent ui` | UI Designer | GUI design, egui implementation (Phase 4) |

## Project Context

### Key Documentation
- `Phase3_Architecture.md` - System architecture and design
- `rust-resources.md` - Rust ecosystem knowledge base (living document)
- `IMPLEMENTATION_PLAN.md` - Sprint planning and milestones
- `AI_DEVELOPMENT_GUIDE.md` - Team coordination guidelines

### Tech Stack
- **Language:** Rust (Edition 2021)
- **2D Formats:** image crate
- **3D Formats:** tobj, ply-rs, gltf crates
- **3D Math:** nalgebra
- **CAD (future):** truck crate
- **GUI (Phase 4):** egui/eframe

## Coding Standards

### Rust Conventions
```rust
// Use Result for fallible operations
pub fn convert(source: &Path, target: &Path) -> Result<(), ConversionError>

// Use thiserror for error types
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
}

// Document all public APIs
/// Converts an image from one format to another.
///
/// # Errors
/// Returns an error if the source file cannot be read.
pub fn convert(...) -> Result<...>
```

### Before Committing
```bash
cargo fmt        # Format code
cargo clippy     # Lint checks
cargo test       # Run tests
cargo doc        # Verify docs build
```

## Agent Prompt Files

Agent definitions are stored in `.agents/`:
- `.agents/system-architect.md`
- `.agents/senior-engineer.md`
- `.agents/junior-engineer-2d.md`
- `.agents/junior-engineer-3d.md`
- `.agents/security-specialist.md`
- `.agents/documentation-specialist.md`
- `.agents/researcher.md`
- `.agents/ui-designer.md`

To use an agent, read the corresponding file and adopt that persona for the conversation.

## Quick Reference

### Decision Escalation
```
Junior → Senior → Architect
Security can veto on security issues
Researcher advises on ecosystem/libraries
```

### Key Principle
**Always consult `rust-resources.md` before making technical decisions.**

## Custom Instructions

When working on this project:
1. Follow the established patterns in the codebase
2. Check rust-resources.md for library guidance
3. Write tests for new functionality
4. Document public APIs
5. Consider security implications (all file input is untrusted)
6. Update rust-resources.md with lessons learned
