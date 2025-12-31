# Sprint X Task Assignment
## [Feature/Task Name] - v[X.Y.Z]

**Sprint Duration:** X weeks (Weeks X-Y)
**Target Release:** vX.Y.Z

---

## CRITICAL: Role Selection (READ FIRST - STOP HERE UNTIL COMPLETE)

**You are an unassigned agent. You MUST claim a role before proceeding.**

### Step 1: Review Available Roles

Look at the Role Assignment table below. Find a role where:
- Status = `Available`
- No agent is currently assigned

### Step 2: Claim Your Role

Once you find an available role:
1. **Edit this document** to update that role's row:
   - Change Status from `Available` to `In Progress`
   - Add your session identifier to the "Assigned Agent" column
2. **Read the persona file** listed in the "Persona File" column
3. **Adopt that persona** for all remaining work on this tasking

### Step 3: Become Your Role

After claiming, you ARE that agent. Read the persona file and embody:
- The agent's name and identity
- Their expertise and communication style
- Their decision-making authority
- Their specific responsibilities

**IMPORTANT:** If all roles show "In Progress" or "Complete", STOP. Do not proceed - there is no work available for you.

---

## Role Assignment

| Role | Persona File | Status | Assigned Agent | Tasks | Dependencies |
|------|--------------|--------|----------------|-------|--------------|
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | Available | - | Tasks 3.2, 3.3 | None |
| Junior Engineer 3D (Alex Rivera) | `.agents/junior-engineer-3d.md` | Available | - | Tasks 1.1, 2.1, 2.2 | None |
| Security Specialist (Casey Morgan) | `.agents/security-specialist.md` | Available | - | Task 4.2 | Tasks 3.1-3.3 |
| Documentation Specialist (Morgan Lee) | `.agents/documentation-specialist.md` | Available | - | Task 4.3 | All other tasks |

**Status Values:**
- `Available` - Role can be claimed by any unassigned agent
- `In Progress` - Role has been claimed (see Assigned Agent column)
- `Complete` - All tasks for this role are finished
- `Blocked` - Role is waiting on dependencies (cannot be claimed yet)

### Role Claiming Example

**Before** (you see this):
```
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | Available | - | Tasks 3.2, 3.3 | None |
```

**After** (you update to this):
```
| UI Designer (Jamie Chen) | `.agents/ui-designer.md` | In Progress | agent-session-12345 | Tasks 3.2, 3.3 | None |
```

Then read `.agents/ui-designer.md` and become Jamie Chen for this tasking.

---

## Agent Persona Reference

The following persona files define each role's identity, expertise, and responsibilities:

| Role | Persona File | Key Expertise |
|------|--------------|---------------|
| System Architect | `.agents/system-architect.md` | Architecture, design decisions, technology selection |
| Senior Engineer | `.agents/senior-engineer.md` | Core implementation, code reviews, sprint coordination |
| Junior Engineer 2D | `.agents/junior-engineer-2d.md` | 2D image formats (PNG, JPEG, BMP, GIF) |
| Junior Engineer 3D | `.agents/junior-engineer-3d.md` | 3D mesh formats (PLY, OFF, glTF, STEP) |
| Security Specialist | `.agents/security-specialist.md` | Security reviews, vulnerability analysis |
| Documentation Specialist | `.agents/documentation-specialist.md` | API docs, user guides, examples |
| Researcher | `.agents/researcher.md` | Ecosystem monitoring, library evaluation |
| UI Designer | `.agents/ui-designer.md` | GUI design, egui implementation, UX |

---

## Progress Summary

**Overall Status:** [ ] Not Started / [ ] In Progress / [x] Complete

### Current Status
- [ ] Task 1: Description
- [ ] Task 2: Description
- [x] Task 3: Description (Complete)

**Last Updated:** [Date]
**Last Updated By:** [Role Name - Agent ID]

---

## Your Mission

[Clear description of the sprint mission and overall goals.]

---

## Required Reading (After Claiming Role)

1. **Your persona file** (from Role Assignment table) - Adopt this identity
2. **SPRINT_X_SUMMARY.md** - Executive briefing and sprint overview
3. **SPRINT_X_TASKING.md** - Complete detailed task breakdown (if exists)
4. **AI_DEVELOPMENT_GUIDE.md** - Team coordination guidelines
5. **rust-resources.md** - Check for library updates and best practices

---

## Task Definitions

### Phase X: [Phase Name] (Days X-Y)

#### Task X.Y: Task Name
**Priority:** Critical / High / Medium
**Estimated:** X hours
**Status:** [ ] Not Started / [ ] In Progress / [x] Complete / [ ] Blocked
**Assigned Role:** [Role Name from Role Assignment table]

**Dependencies:**
- :hourglass_flowing_sand: **Task A.B:** Dependency description - **Source:** `AGENT_TASKS/[filename].md` - **Status:** [Check source document]
- :white_check_mark: **Task C.D:** Dependency description - **Source:** `AGENT_TASKS/[filename].md` - **Status:** Complete

**Dependency Check (REQUIRED before starting):**
Before beginning this task, you MUST:
1. Locate each dependency's source document
2. Check the current status of all dependencies
3. Verify all blocking dependencies are marked complete
4. If dependencies are incomplete, mark this task as "Blocked" and wait
5. Document when dependencies were verified

**Dependency Verification Log:**
- [Date/Time]: Checked Task A.B - Status: [Status]
- [Date/Time]: All dependencies verified - Proceeding

**What to Do:**
- Task description
- Step-by-step instructions
- Implementation details

**Reference Documents:**
- Path to relevant documents
- Related code files

**Acceptance Criteria:**
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

**Completion Record:**
```
Status: [x] Complete
Completed By: [Role Name] - [Agent ID]
Completed On: [Date]
Notes:
- Implementation notes
- Any issues encountered
```

---

## Status Update Obligations

**As the agent who claimed a role, YOU MUST:**
1. Update task status in this document as work progresses
2. Check and update dependency statuses before starting dependent work
3. Update the Role Assignment table when you complete all your tasks
4. Update progress summary sections regularly
5. Document blockers and wait for dependencies when blocked

**Update Frequency:**
- When claiming role: Mark role as "In Progress" with your ID
- When starting a task: Mark task as "In Progress"
- When blocked: Mark task as "Blocked", document why, WAIT
- When completing a task: Mark as "Complete" with notes
- When all tasks done: Update role status to "Complete"

---

## Coordination with Other Roles

### Waiting for Dependencies

If your task depends on another role's work:
1. Check the dependency's source document for current status
2. If incomplete, mark your task as "Blocked"
3. **DO NOT PROCEED** - wait for the dependency to be marked complete
4. Periodically re-check the dependency status
5. Resume work only after dependency shows "Complete"

### Handoff Protocol

When completing work that other roles depend on:
1. Mark your task as "Complete" with detailed notes
2. Update any shared documents or code
3. Ensure acceptance criteria are met
4. Other agents will check this document before starting their dependent work

---

## Questions or Blockers?

**Contact Points (via tasking documents):**
- Senior Engineer tasks: `AGENT_TASKS/SENIOR_ENGINEER_SPRINTX.md`
- Architecture questions: `AGENT_TASKS/SYSTEM_ARCHITECT_SPRINTX.md`

**Reference Documents:**
- Architecture: `Phase3_Architecture.md`
- Team coordination: `AI_DEVELOPMENT_GUIDE.md`

---

**Document Version:** 3.0
**Created:** [Date]
**Status:** Ready for Role Assignment

