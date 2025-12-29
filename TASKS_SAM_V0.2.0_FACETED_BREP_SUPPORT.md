# Task Assignment - Sam Parker (Junior Engineer, 2D Formats)
## v0.2.0 FACETED_BREP Support - Research & Documentation

**Assigned By:** Jordan Rivera (Senior Engineer)  
**Date:** January 29, 2025  
**Engineer:** Sam Parker  
**Priority:** 🔥 **HIGH - SUPPORT IMPLEMENTATION**  
**Status:** ✅ **READY TO PROCEED**

---

## Executive Summary

The System Architect has **approved** the FACETED_BREP approach for v0.2.0. Riley needs your support to complete the implementation. Your role is to:

1. **Research** ruststep AP203 API for FACETED_BREP entities
2. **Document** FACETED_BREP limitations and CAD export guidance
3. **Collect** test STEP files with FACETED_BREP entities
4. **Review** error messages for clarity

**Current Status:**
- ✅ Tables API research complete (excellent work!)
- ✅ Verification code working
- 🚧 **NEXT:** FACETED_BREP API research and documentation

---

## Task 1: FACETED_BREP API Research (4-6 hours)

**Objective:** Research ruststep AP203 API for FACETED_BREP entities

**Steps:**
1. [ ] Check ruststep documentation for FACETED_BREP
   - Search docs.rs: https://docs.rs/ruststep/0.4/
   - Look for `FacetedBrep` type
   - Check for `faceted_brep_holders()` method

2. [ ] Check ruststep source code
   - GitHub: https://github.com/ricosjp/ruststep
   - Look for AP203 entity definitions
   - Find `FacetedBrep` struct definition
   - Check Tables implementation for getter methods

3. [ ] Verify entity structure
   - What fields does `FacetedBrep` have?
   - How to access `outer` (CLOSED_SHELL)?
   - What's the relationship to `ManifoldSolidBrep`?

4. [ ] Create example code snippets
   ```rust
   // Example: How to access FACETED_BREP entities
   let fb_holders = tables.faceted_brep_holders();
   // or
   let fb_holders = tables.manifold_solid_brep_holders();
   // Check if they're FACETED_BREP type
   ```

5. [ ] Document findings
   - Create `FACETED_BREP_API_FINDINGS.md`
   - Include code examples
   - Share with Riley immediately

**Success Criteria:**
- ✅ Know if `faceted_brep_holders()` exists
- ✅ Understand entity structure
- ✅ Have working code examples
- ✅ Documentation complete

**Resources:**
- ruststep docs.rs: https://docs.rs/ruststep/0.4/
- ruststep GitHub: https://github.com/ricosjp/ruststep
- Your previous research: `TABLES_API_FINDINGS_FOR_RILEY.md`

---

## Task 2: Documentation Updates (4-6 hours)

**Objective:** Update documentation with FACETED_BREP limitations and guidance

**Files to Update:**

1. [ ] `docs/FORMATS.md`
   - Add STEP format entry
   - Document FACETED_BREP support status
   - Add limitations section
   - Include CAD export guidance

2. [ ] `docs/STEP_FORMAT_REFERENCE.md` (if exists)
   - Update with FACETED_BREP information
   - Document entity traversal path
   - Add code examples

3. [ ] `README.md`
   - Update format support matrix
   - Add STEP limitations note
   - Link to detailed documentation

4. [ ] Create `docs/CAD_EXPORT_GUIDE.md` (new)
   - How to export STEP with FACETED_BREP
   - CAD software-specific instructions:
     - SolidWorks
     - FreeCAD
     - Fusion 360
     - OpenSCAD
   - Troubleshooting tips

**Content to Include:**

**FACETED_BREP Limitations:**
- Only supports pre-tessellated geometry
- Does not support NURBS surfaces, cylinders, etc.
- Requires CAD export with tessellation enabled
- Full B-Rep support planned for v0.3.0

**CAD Export Guidance:**
- How to enable tessellation in common CAD tools
- Export settings recommendations
- File format options

**Error Messages:**
- What users will see if file doesn't have FACETED_BREP
- How to fix the issue
- Links to documentation

**Success Criteria:**
- ✅ All documentation updated
- ✅ Clear limitations explained
- ✅ CAD export guidance complete
- ✅ Error messages documented

---

## Task 3: Test File Collection (2-4 hours)

**Objective:** Collect test STEP files with FACETED_BREP entities

**Steps:**
1. [ ] Find or create test STEP files
   - Simple geometry (cube, sphere, etc.)
   - Must contain FACETED_BREP entities
   - Not just MANIFOLD_SOLID_BREP

2. [ ] Verify file contents
   - Use ruststep to parse files
   - Check for FACETED_BREP entities
   - Verify entity structure

3. [ ] Document test files
   - Create `tests/step/test_files.md`
   - List each file:
     - Source (where it came from)
     - Entity types present
     - Expected conversion result
     - Notes

4. [ ] Organize test files
   - Place in `tests/step/` directory
   - Name descriptively (e.g., `cube_faceted_brep.step`)
   - Add to `.gitignore` if too large

**Test File Requirements:**
- ✅ Contains FACETED_BREP entities
- ✅ Simple geometry (easy to verify)
- ✅ Valid STEP file format
- ✅ Reasonable size (< 1MB)

**Sources:**
- CAD software exports (FreeCAD, OpenSCAD)
- Online STEP file repositories
- Create simple test files manually

**Success Criteria:**
- ✅ At least 3-5 test files collected
- ✅ Files verified to contain FACETED_BREP
- ✅ Documentation complete
- ✅ Files organized in test directory

---

## Task 4: Error Message Review (2-4 hours)

**Objective:** Review and improve error messages for clarity

**Steps:**
1. [ ] Review current error messages in code
   - Check `mesh-core/src/formats/step.rs`
   - Identify all error messages
   - Evaluate clarity and helpfulness

2. [ ] Test error scenarios
   - Non-FACETED_BREP file
   - Empty file
   - Invalid STEP file
   - Missing entities

3. [ ] Improve error messages
   - Make them user-friendly
   - Include actionable guidance
   - Reference documentation
   - Suggest solutions

4. [ ] Create error message guidelines
   - Document best practices
   - Provide examples
   - Share with team

**Error Message Requirements:**
- ✅ Clear and concise
- ✅ Actionable (tells user what to do)
- ✅ References documentation
- ✅ Professional tone

**Example Good Error Message:**
```
STEP file contains curved surfaces (NURBS, cylinders, etc.) which require 
full B-Rep support. This is planned for v0.3.0.

For v0.2.0, please export your STEP file with tessellation enabled:
- SolidWorks: File → Save As → Options → "Tessellated" 
- FreeCAD: Export → STEP → "FACETED_BREP" option
- See docs/CAD_EXPORT_GUIDE.md for more details
```

**Success Criteria:**
- ✅ All error messages reviewed
- ✅ Error messages improved
- ✅ Guidelines documented
- ✅ Tested with real scenarios

---

## Implementation Strategy

### Priority Order

1. **Task 1 (API Research)** - **HIGHEST PRIORITY**
   - Riley needs this immediately
   - Blocking implementation
   - Do this first

2. **Task 2 (Documentation)** - **HIGH PRIORITY**
   - Users need this information
   - Can be done in parallel with implementation
   - Update as implementation progresses

3. **Task 3 (Test Files)** - **MEDIUM PRIORITY**
   - Needed for testing
   - Can be done incrementally
   - Collect as you find them

4. **Task 4 (Error Messages)** - **MEDIUM PRIORITY**
   - Can be done after implementation
   - Review and improve iteratively

### Collaboration with Riley

- **Share findings immediately** when discovered
- **Update documentation** as implementation progresses
- **Help with API questions** as they come up
- **Test files** should be shared as soon as collected

---

## Getting Help

### When to Ask

**Ask immediately if:**
- Can't find FACETED_BREP API information
- Documentation unclear
- Need clarification on requirements

### Who to Ask

1. **Riley:** For implementation questions, test file requirements
2. **Senior Engineer:** For documentation standards, error message review
3. **Resources:** ruststep documentation, CAD software docs

---

## Code Quality Requirements

### Documentation Standards

1. **Clarity:**
   - Clear and concise
   - User-friendly language
   - Actionable guidance

2. **Completeness:**
   - Cover all scenarios
   - Include examples
   - Reference related docs

3. **Accuracy:**
   - Verify information
   - Test examples
   - Update as needed

---

## Timeline

**Week 1 (Days 1-3):**
- Day 1: Task 1 (API Research) - **URGENT**
- Day 2: Task 2 (Documentation) start
- Day 3: Task 2 completion + Task 3 start

**Week 2 (Days 4-5):**
- Day 4: Task 3 (Test Files) completion
- Day 5: Task 4 (Error Messages) + final review

**Target:** All tasks complete by end of Week 2

---

## Success Criteria

### End of Week 1

- ✅ FACETED_BREP API research complete
- ✅ Documentation updated (at least initial version)
- ✅ At least 2-3 test files collected
- ✅ Findings shared with Riley

### End of Week 2

- ✅ All documentation complete
- ✅ Test files collected and documented
- ✅ Error messages reviewed and improved
- ✅ Ready for v0.2.0 release

---

## Key Messages

**GOOD WORK:** Your research and verification work has been valuable. Continue supporting Riley.

**PRIORITY:** Help Riley with FACETED_BREP API research. This is blocking implementation.

**CONTINUE:** Documentation and test file collection. Share findings immediately.

**GOAL:** By end of Week 1, have all research and initial documentation complete.

---

**Status:** 🔥 **HIGH PRIORITY - READY TO PROCEED**  
**Priority:** **HIGH**  
**Support:** Available for Riley immediately

**Thanks for your support, Sam!**

---

*Assigned By: Jordan Rivera (Senior Engineer)*  
*Date: January 29, 2025*  
*Architect Approval: Alex Chen (System Architect) - January 29, 2025*

