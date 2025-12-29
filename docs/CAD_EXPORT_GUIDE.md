# CAD Export Guide - STEP with FACETED_BREP
## How to Export STEP Files Compatible with Simple Image Converter v0.2.0

**Last Updated:** December 29, 2025  
**Status:** v0.2.0 - FACETED_BREP Support  
**Target Audience:** CAD Users

---

## Overview

Simple Image Converter v0.2.0 supports **FACETED_BREP** entities in STEP files. FACETED_BREP represents pre-tessellated (triangulated) geometry, which is easier to convert to mesh formats.

**Important:** 
- ✅ **Supported:** FACETED_BREP (pre-tessellated geometry)
- ❌ **Not Supported (v0.2.0):** Full B-Rep with NURBS surfaces, cylinders, spheres, etc.
- 📅 **Planned (v0.3.0):** Full B-Rep support

**If your STEP file contains curved surfaces (NURBS, cylinders, etc.), you must export with tessellation enabled.**

---

## What is FACETED_BREP?

**FACETED_BREP** is a STEP entity type that represents 3D solids using **pre-tessellated** (triangulated) faces. This means:

- All surfaces are already converted to triangles
- No parametric surfaces (NURBS, cylinders, spheres, etc.)
- Easier to convert to mesh formats (STL, OBJ, PLY, etc.)

**Full B-Rep** (MANIFOLD_SOLID_BREP) contains:
- Parametric surfaces (NURBS, cylinders, spheres, etc.)
- Requires tessellation during conversion
- More complex to process

**For v0.2.0, only FACETED_BREP is supported.**

---

## CAD Software Instructions

### SolidWorks

**Steps:**
1. Open your model in SolidWorks
2. Go to **File → Save As**
3. Select **STEP (*.step, *.stp)** as file type
4. Click **Options** (or **Save Options**)
5. In the STEP export options:
   - Look for **"Tessellation"** or **"Faceted"** option
   - Enable tessellation/faceted export
   - Set tessellation quality (if available)
6. Click **OK** and save

**Alternative Method:**
- Some versions have **"Export as Faceted"** checkbox
- Enable this option before saving

**Note:** If your model contains only planar faces, it may already export as FACETED_BREP automatically.

---

### FreeCAD

**Steps:**
1. Open your model in FreeCAD
2. Select the object you want to export
3. Go to **File → Export**
4. Select **STEP (*.step, *.stp)** format
5. In the export dialog:
   - Look for **"FACETED_BREP"** or **"Tessellated"** option
   - Enable FACETED_BREP export
   - Adjust tessellation parameters if available
6. Click **Save**

**Alternative Method:**
- Use **Mesh → Create Mesh from Shape** first
- Then export as STEP (may create FACETED_BREP automatically)

**Note:** FreeCAD's STEP export may default to FACETED_BREP for simple geometry.

---

### Fusion 360

**Steps:**
1. Open your model in Fusion 360
2. Go to **File → Export**
3. Select **STEP (*.step, *.stp)** format
4. In export options:
   - Look for **"Tessellation"** or **"Faceted"** option
   - Enable tessellation
   - Set tessellation quality/accuracy
5. Click **Export**

**Alternative Method:**
- Some versions have **"Export as Mesh"** option
- This may create FACETED_BREP-compatible files

**Note:** Fusion 360 may require enabling tessellation in export settings.

---

### OpenSCAD

**Steps:**
1. Design your model in OpenSCAD
2. Go to **File → Export → Export as STL** (first)
3. Then import the STL into a CAD tool that supports STEP export
4. Export as STEP with tessellation enabled

**Alternative Method:**
- Use **CGAL** library functions that output faceted geometry
- Export directly to STEP if OpenSCAD supports it

**Note:** OpenSCAD primarily works with faceted geometry, so exported STEP files may already be FACETED_BREP.

---

### AutoCAD

**Steps:**
1. Open your 3D model in AutoCAD
2. Go to **File → Export**
3. Select **STEP (*.step, *.stp)** format
4. In export options:
   - Look for **"Tessellation"** or **"Faceted"** option
   - Enable tessellation
5. Click **Save**

**Note:** AutoCAD's STEP export may require enabling tessellation explicitly.

---

### Onshape

**Steps:**
1. Open your model in Onshape
2. Right-click on the part/assembly
3. Select **Export**
4. Choose **STEP (*.step, *.stp)** format
5. In export options:
   - Look for **"Tessellation"** or **"Faceted"** option
   - Enable tessellation
6. Click **Export**

**Note:** Onshape may have tessellation options in export settings.

---

### Blender

**Steps:**
1. Open your model in Blender
2. Go to **File → Export → STEP**
3. If STEP export is not available:
   - Export as **STL** first
   - Import STL into a CAD tool that supports STEP export
   - Export as STEP with tessellation enabled

**Alternative Method:**
- Use **Blender STEP Export** addon (if available)
- Enable tessellation options

**Note:** Blender primarily works with mesh geometry, so exported STEP files may already be FACETED_BREP.

---

## Verification

### How to Check if Your STEP File Contains FACETED_BREP

**Method 1: Using Simple Image Converter**
```bash
# Try to convert the STEP file
./mesh-convert model.step stl

# If it works, the file contains FACETED_BREP
# If it fails with "no FACETED_BREP entities" error, tessellation is needed
```

**Method 2: Text Search**
Open the STEP file in a text editor and search for:
- `FACETED_BREP` - If found, file contains FACETED_BREP entities ✅
- `MANIFOLD_SOLID_BREP` - If found without FACETED_BREP, tessellation may be needed ⚠️

**Method 3: NIST STEP File Analyzer**
1. Download NIST STEP File Analyzer: https://www.nist.gov/services-resources/software/step-file-analyzer
2. Open your STEP file
3. Check entity types in the generated report
4. Look for `FACETED_BREP` entities

---

## Troubleshooting

### Error: "No FACETED_BREP entities found"

**Cause:** Your STEP file contains full B-Rep (MANIFOLD_SOLID_BREP) with parametric surfaces.

**Solution:**
1. Re-export your STEP file with tessellation enabled (see CAD software instructions above)
2. Ensure tessellation quality is set appropriately
3. Verify the exported file contains FACETED_BREP entities

### Error: "STEP file contains curved surfaces"

**Cause:** Your STEP file contains NURBS surfaces, cylinders, spheres, etc. that require full B-Rep support.

**Solution:**
1. Export with tessellation enabled
2. Increase tessellation quality/accuracy
3. Re-export the file

### File Exports but Conversion Fails

**Possible Causes:**
1. Tessellation not enabled during export
2. File contains unsupported entity types
3. File is corrupted or malformed

**Solution:**
1. Verify tessellation was enabled during export
2. Check file with NIST STEP File Analyzer
3. Try exporting again with different settings

---

## Export Settings Recommendations

### Tessellation Quality

**For Best Results:**
- **High Quality:** Better accuracy, larger file size
- **Medium Quality:** Good balance (recommended)
- **Low Quality:** Faster export, may lose detail

**For Simple Image Converter:**
- Medium to high quality recommended
- Very high quality may not be necessary (FACETED_BREP is already triangulated)

### File Size Considerations

**FACETED_BREP files:**
- Generally larger than full B-Rep (geometry is pre-tessellated)
- Size depends on tessellation quality
- Higher quality = larger file size

**Recommendation:**
- Use medium quality for most cases
- Use high quality only if detail is critical

---

## Future Support (v0.3.0)

**Planned Features:**
- ✅ Full B-Rep support (NURBS surfaces, cylinders, spheres, etc.)
- ✅ Automatic tessellation during conversion
- ✅ Support for MANIFOLD_SOLID_BREP with parametric surfaces

**Until v0.3.0:**
- Export with FACETED_BREP tessellation enabled
- Follow this guide for CAD software-specific instructions

---

## Additional Resources

- **FACETED_BREP API Research:** `FACETED_BREP_API_FINDINGS.md`
- **STEP Format Reference:** `docs/STEP_FORMAT_REFERENCE.md`
- **Format Support Matrix:** `docs/FORMATS.md`
- **NIST STEP File Analyzer:** https://www.nist.gov/services-resources/software/step-file-analyzer

---

## Summary

**For v0.2.0:**
1. ✅ Export STEP files with **tessellation enabled**
2. ✅ Ensure **FACETED_BREP** entities are created
3. ✅ Use medium to high tessellation quality
4. ✅ Verify file contains FACETED_BREP before conversion

**Quick Checklist:**
- [ ] CAD software supports STEP export with tessellation
- [ ] Tessellation option enabled during export
- [ ] File exported successfully
- [ ] File verified to contain FACETED_BREP entities
- [ ] Conversion test successful

---

**Last Updated:** December 29, 2025  
**Maintained By:** Sam Parker (Junior Engineer, 2D Formats)  
**For:** CAD Users and Implementation Team

