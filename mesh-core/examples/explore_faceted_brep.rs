// SPDX-License-Identifier: MIT OR Apache-2.0
// Explore FacetedBrep entity structure in ruststep
// This code explores the FacetedBrep type and its fields

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_minimal_cfg)]

#[cfg(feature = "step")]
use ruststep::ap203::config_control_design;
#[cfg(feature = "step")]
use ruststep::tables::{IntoOwned, TableInit};

#[cfg(feature = "step")]
fn main() {
    println!("=== Exploring FacetedBrep Structure ===\n");

    // Create empty Tables for exploration
    let tables = config_control_design::Tables::default();

    // Test 1: Verify faceted_brep_holders() exists
    println!("Test 1: Checking faceted_brep_holders() method...");
    let fb_holders = tables.faceted_brep_holders();
    println!("✓ tables.faceted_brep_holders() exists!");
    println!("  Type: {}", std::any::type_name_of_val(fb_holders));
    println!("  Found {} FacetedBrep entities\n", fb_holders.len());

    // Test 2: Explore FacetedBrepHolder structure
    if let Some((id, holder)) = fb_holders.iter().next() {
        println!("Test 2: Exploring FacetedBrepHolder structure...");
        println!("  Entity ID: {}", id);
        println!("  Holder type: {}", std::any::type_name_of_val(holder));
        println!("  Holder: {:?}\n", holder);

        // Try to access holder fields
        // Note: We need to check ruststep source to see what fields are available
        // Common pattern: holder.name, holder.outer (for CLOSED_SHELL reference)
    }

    // Test 3: Check relationship to ManifoldSolidBrep
    println!("Test 3: Checking relationship to ManifoldSolidBrep...");
    let msb_holders = tables.manifold_solid_brep_holders();
    println!("  ManifoldSolidBrep entities: {}", msb_holders.len());
    println!("  Note: FACETED_BREP is a subtype of MANIFOLD_SOLID_BREP in STEP AP203\n");

    // Test 4: Check if we can get owned FacetedBrep
    println!("Test 4: Testing IntoOwned trait for FacetedBrep...");
    if let Some((id, holder)) = fb_holders.iter().next() {
        // Try to get owned version (fully resolved entity)
        // This requires checking if FacetedBrepHolder implements IntoOwned
        println!(
            "  Attempting to get owned FacetedBrep for entity #{}...",
            id
        );
        // let owned: config_control_design::FacetedBrep = holder.get_owned(&tables)?;
        // This will be tested with actual STEP file data
    }

    println!("\n=== Summary ===");
    println!("1. ✓ faceted_brep_holders() method exists");
    println!("2. Returns HashMap<u64, FacetedBrepHolder>");
    println!("3. Need to explore FacetedBrepHolder fields");
    println!("4. Need to check IntoOwned trait implementation");
    println!("5. Need to test with actual FACETED_BREP STEP file");
}

#[cfg(not(feature = "step"))]
fn main() {
    println!("STEP feature not enabled. Run with: cargo run --example explore_faceted_brep --features step");
}
