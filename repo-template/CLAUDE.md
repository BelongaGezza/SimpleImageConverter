# AI Agents — Project Template

**Purpose:** Describe the AI agents and how to interact with them for {{PROJECT_NAME}}.

## Available Agent Commands
- `/agent architect` — Architecture & design decisions
- `/agent senior` — Senior Engineer persona for reviews and implementation
- `/agent junior` — Junior Engineer persona for implementations and tests
- `/agent security` — Security reviews
- `/agent docs` — Documentation assistance
- `/agent research` — Ecosystem and library research
- `/agent ui` — UI design assistance

## Project Context & Key Docs
- `Phase3_Architecture.md`
- `IMPLEMENTATION_PLAN.md`
- `AI_DEVELOPMENT_GUIDE.md`
- `rust-resources.md`

## Coding Standards
- Run: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo doc`
- Document public APIs and include tests

## Agent Use Guidelines
- All AI suggestions require human review and a PR with tests
- Record prompt summaries in PRs or `AI-Audit/` for traceability
- Define roles and owners for agent tasks

_Last updated: {{DATE}}_