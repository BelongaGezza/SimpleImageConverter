# INIT_PROMPT.md — Claude Code Repository Initialisation Prompt

Paste the content between the cut lines as your **first message** to Claude Code
after cloning or initialising a repository on any machine.

**When to use this prompt:**
- First session after `git clone` on any machine
- After Claude Code memory has been reset or cleared
- When onboarding a new AI subagent to this repository

---

## ✂️ COPY FROM HERE ─────────────────────────────────────────────────────────

You are starting a new development session in the SimpleImageConverter repository.
Before taking any other action, perform the following steps in order and output the
result of each step before proceeding to the next.

---

### STEP 1 — Read project context files

Read these files in this exact order:

1. `README.md` — understand what this application is and what it does
2. `CLAUDE.md` — your mandatory operating constraints for this repository

Do not proceed to Step 2 until both files have been fully read.
If either file is missing, state which is missing and continue with the other.

---

### STEP 2 — Detect current development environment

Check for the following and report findings:

- Operating system (Windows / macOS / Linux)
- `rustc` and `cargo` in PATH and their versions
- `xcodebuild` in PATH (macOS — required for signed macOS builds and releases)
- Any CI environment variable set (e.g. `CI`, `GITHUB_ACTIONS`, `CIRCLECI`)

Output this report exactly:

```
ENVIRONMENT REPORT
─────────────────────────────────────────────────────────
OS:                [Windows 11 | macOS | Linux]
Rust toolchain:    [rustc X.Y.Z | NOT available]
Cargo:             [cargo X.Y.Z | NOT available]
Xcode tools:       [available — macOS signing OK | NOT available]
CI mode:           [yes — $VAR_NAME | no]
─────────────────────────────────────────────────────────
Buildable this session:
  [✓/✗] Windows build     (requires Windows OS + Rust)
  [✓/✗] macOS build       (requires macOS + Rust)
  [✓/✗] macOS signed pkg  (requires macOS + Rust + xcodebuild)
  [✓/✗] Linux build       (requires Linux + Rust)

NOT buildable / signable this session:
  [list excluded targets with reason, or "none — all targets buildable"]
─────────────────────────────────────────────────────────
```

---

### STEP 3 — Check for pending items

Check whether the following files exist and, if so, read them:

- `PENDING_APPLE_CHANGES.md` — macOS changes required from prior sessions
- `SETUP_NOTES.md` — one-time setup steps required on this machine

If either file contains unresolved entries, summarise them and ask:
*"There are pending items from prior sessions. Address these now or proceed with
new work?"*

If both files are empty or contain no actionable items, state: *"No pending items."*

---

### STEP 4 — Inventory available agents and resources

Check for the following and list what is present:

**`.agents/`** — specialised agent personas for this project:
- List each `.md` file found and its agent name/role (read the first heading from each).

**`AGENT_TASKS/`** — active and historical agent task briefs:
- List any open or recently created task files.

**`rust-resources.md`** (if present) — Rust ecosystem knowledge base:
- Confirm it exists and note the date of last update if visible.

Output format:

```
AGENT & RESOURCE INVENTORY
─────────────────────────────────────────────────────────
Agents (.agents/):
  • [agent-name] — [role summary]

Active tasks (AGENT_TASKS/):
  • [filename] — [brief description]

Rust resources:
  • rust-resources.md — [present / missing]
─────────────────────────────────────────────────────────
```

---

### STEP 5 — Confirm active constraints

State the following confirmation block verbatim, filling in the bracketed values:

```
CONSTRAINT CONFIRMATION
─────────────────────────────────────────────────────────
Cross-platform guard convention : [PLATFORM: X] comment guards + cfg attrs  ✓
macOS signing restriction       : [ENFORCED — non-macOS | NOT ACTIVE — macOS session]  ✓
Hardcoded path policy           : no absolute paths in committed files        ✓
Security policy                 : all file input is untrusted; validate early ✓
License policy                  : MIT OR Apache-2.0; no GPL/AGPL deps         ✓
Subagent constraint inheritance : all subagents inherit CLAUDE.md rules       ✓
─────────────────────────────────────────────────────────
```

---

### STEP 6 — Write session context to memory

Write a Claude memory note containing:

- Application name and purpose
- Cargo workspace structure and crate responsibilities
- Platform target matrix (Windows / macOS / Linux — all active)
- Current dev machine OS and which builds are available this session
- Available agent roles
- Key conventions: comment guard format, cfg attributes, path policy, security policy

Title the memory: `SimpleImageConverter Session Context — [OS] — [date]`

---

### STEP 7 — Ready

State:

*"Session initialised on [OS]. macOS signed builds [ARE / are NOT] available this
session. Available agents: [comma-separated list]. What would you like to work on?"*

Then wait for instructions. **Do not write any code or modify any files until
explicitly instructed.**

---

## ✂️ COPY TO HERE ─────────────────────────────────────────────────────────

---

## Usage Notes

### Subagent invocation prefix

```
Before starting your task:
1. Read CLAUDE.md in full.
2. Read .agents/[agent-name].md for your role definition.
3. Read rust-resources.md for library and ecosystem guidance.
4. Current OS: [paste OS from Environment Report].
5. Apply all constraints from CLAUDE.md to your work.
```

---

*Schema version: 1.0 — SimpleImageConverter (pure Rust / egui; Windows + macOS + Linux)*
