// SPDX-License-Identifier: MIT OR Apache-2.0
// Research code for FACETED_BREP API in ruststep
// This code explores the ruststep AP203 API to find FACETED_BREP access methods
//
// Run with: cargo run --example research_faceted_brep_api --features step

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_minimal_cfg)]

#[cfg(feature = "step")]
use ruststep::ap203::config_control_design;
#[cfg(feature = "step")]
use ruststep::tables::TableInit;
#[cfg(feature = "step")]
use ruststep::{ast, parser};

#[cfg(feature = "step")]
fn main() {
    println!("=== FACETED_BREP API Research ===\n");

    // Simple STEP file for testing (will need actual FACETED_BREP file later)
    let step_text = r#"ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Test'), '2;1');
FILE_NAME('test.step', '2025-12-29T00:00:00', ('Sam'), ('SimpleImageConverter'), '', '', '');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;

DATA;
#1 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
ENDSEC;

END-ISO-10303-21;
"#;

    // Test 1: Parse STEP file
    println!("Test 1: Parsing STEP file...");
    match parser::parse(step_text) {
        Ok(exchange) => {
            println!("✓ Successfully parsed STEP file\n");

            // Test 2: Create Tables and explore API
            println!("Test 2: Exploring ruststep AP203 Tables API...");
            explore_tables_api(&exchange);
        }
        Err(e) => {
            println!("✗ Failed to parse STEP file: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "step")]
fn explore_tables_api(exchange: &ast::Exchange) {
    use ruststep::ap203::config_control_design::Tables;
    use ruststep::tables::TableInit;

    // Create Tables from Exchange.data
    match Tables::from_data_sections(&exchange.data) {
        Ok(tables) => {
            println!("✓ Successfully created Tables\n");

            // Explore available methods on Tables
            println!("Exploring Tables API methods...\n");

            // Test known methods (from previous research)
            let cartesian_points = tables.cartesian_point_holders();
            println!("✓ tables.cartesian_point_holders() exists");
            println!(
                "  Found {} CartesianPoint entities\n",
                cartesian_points.len()
            );

            let msb_holders = tables.manifold_solid_brep_holders();
            println!("✓ tables.manifold_solid_brep_holders() exists");
            println!("  Found {} ManifoldSolidBrep entities\n", msb_holders.len());

            let cs_holders = tables.closed_shell_holders();
            println!("✓ tables.closed_shell_holders() exists");
            println!("  Found {} ClosedShell entities\n", cs_holders.len());

            // Try to find FACETED_BREP method
            println!("Searching for FACETED_BREP access method...\n");

            // Method 1: Try faceted_brep_holders()
            // This will fail to compile if it doesn't exist, which tells us it's not available
            // Uncomment to test:
            /*
            let fb_holders = tables.faceted_brep_holders();
            println!("✓ tables.faceted_brep_holders() exists");
            println!("  Found {} FacetedBrep entities\n", fb_holders.len());
            */

            // For now, let's check what methods are available by exploring the Tables type
            println!("Note: Checking if faceted_brep_holders() exists requires compilation test");

            // Method 2: Check if FACETED_BREP is a subtype of MANIFOLD_SOLID_BREP
            // In STEP AP203, FACETED_BREP is a subtype of MANIFOLD_SOLID_BREP
            // So we might need to check the type of each ManifoldSolidBrep
            println!("Checking if FACETED_BREP is accessible via ManifoldSolidBrep...\n");

            // Explore ManifoldSolidBrep structure
            for (id, holder) in msb_holders.iter().take(1) {
                println!("  ManifoldSolidBrep #{}:", id);
                println!("    Holder type: {:?}", std::any::type_name_of_val(holder));
                // Try to access holder fields
                // Note: We need to check ruststep source to see what fields are available
            }

            // Method 3: Check entity type names in raw Exchange data
            println!("\nChecking raw Exchange data for FACETED_BREP entities...\n");
            for data_section in &exchange.data {
                for entity_instance in &data_section.entities {
                    if let ast::EntityInstance::Simple { id, record } = entity_instance {
                        if record.name == "FACETED_BREP" {
                            println!("  ✓ Found FACETED_BREP entity #{}", id);
                        }
                    }
                }
            }

            println!("\n=== Research Summary ===");
            println!("1. Tables API uses [entity_name]_holders() pattern");
            println!("2. Need to verify if faceted_brep_holders() exists");
            println!("3. FACETED_BREP may be accessible via ManifoldSolidBrep");
            println!("4. Check ruststep source code for entity type definitions");
        }
        Err(e) => {
            println!("✗ Failed to create Tables: {:?}", e);
        }
    }
}

#[cfg(not(feature = "step"))]
fn main() {
    println!("STEP feature not enabled. Run with: cargo run --example research_faceted_brep_api --features step");
}
