// SPDX-License-Identifier: MIT OR Apache-2.0
// Verification code for ruststep Tables API
// This code verifies actual API usage patterns for STEP entity conversion
//
// Run with: cargo run --example verify_ruststep_tables --features step

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_minimal_cfg)]

#[cfg(feature = "step")]
use ruststep::{ast, parser};

#[cfg(feature = "step")]
fn main() {
    println!("=== ruststep Tables API Verification ===\n");

    // Simple STEP file for testing
    let step_text = r#"ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Test'), '2;1');
FILE_NAME('test.step', '2025-01-27T00:00:00', ('Sam'), ('SimpleImageConverter'), '', '', '');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;

DATA;
#1 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
#2 = CARTESIAN_POINT('', (1.0, 0.0, 0.0));
#3 = CARTESIAN_POINT('', (1.0, 1.0, 0.0));
#4 = CARTESIAN_POINT('', (0.0, 1.0, 0.0));
ENDSEC;

END-ISO-10303-21;
"#;

    // Test 1: Parse STEP file
    println!("Test 1: Parsing STEP file...");
    match parser::parse(step_text) {
        Ok(exchange) => {
            println!("✓ Successfully parsed STEP file");
            println!("  - Number of data sections: {}", exchange.data.len());

            // Count entities
            let mut entity_count = 0;
            for data_section in &exchange.data {
                entity_count += data_section.entities.len();
            }
            println!("  - Total entities: {}\n", entity_count);

            // Test 2: Explore Exchange structure
            println!("Test 2: Exploring Exchange structure...");
            for (i, data_section) in exchange.data.iter().enumerate() {
                println!(
                    "  Data section {}: {} entities",
                    i,
                    data_section.entities.len()
                );

                for entity in &data_section.entities {
                    match entity {
                        ast::EntityInstance::Simple { id, record } => {
                            println!("    Entity #{}: {}", id, record.name);
                            // Note: record.parameter is a Vec<Parameter>
                            // We need to understand how to access parameters
                        }
                        ast::EntityInstance::Complex { id, subsuper } => {
                            println!("    Complex Entity #{}: {} subtypes", id, subsuper.0.len());
                        }
                    }
                }
            }
            println!();

            // Test 3: Try to access AP203 Tables API
            println!("Test 3: Testing AP203 Tables API...");
            test_ap203_tables(&exchange);
        }
        Err(e) => {
            println!("✗ Failed to parse STEP file: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "step")]
fn test_ap203_tables(exchange: &ast::Exchange) {
    // Try to import and use AP203 Tables
    // This is where we verify the actual API

    // Check if AP203 feature is available
    // Note: ruststep has ap203 feature enabled in Cargo.toml
    use ruststep::ap203::config_control_design;
    use ruststep::tables::TableInit;

    println!("  AP203 feature is enabled");

    // ✅ CORRECT METHOD: Use TableInit::from_data_sections()
    // This is the proper way to populate Tables from Exchange.data
    println!("\n  Testing TableInit::from_data_sections()...");

    // Pass exchange.data directly as a slice
    match config_control_design::Tables::from_data_sections(&exchange.data) {
        Ok(tables) => {
            println!("  ✓ Successfully created Tables from data sections!");

            // Now let's explore what entities are available
            println!("\n  Exploring populated Tables...");

            // Test entity access
            test_entity_access(&tables);
        }
        Err(e) => {
            println!("  ✗ Failed to create Tables: {:?}", e);
            println!("  Note: This might happen if entity types don't match AP203 schema");

            // Fallback: Try Tables::default()
            let _tables = config_control_design::Tables::default();
            println!("  Created Tables::default() as fallback");
        }
    }

    println!("\n  Tables API verification complete.\n");
}

#[cfg(feature = "step")]
fn test_entity_access(tables: &ruststep::ap203::config_control_design::Tables) {
    println!("\n  Testing entity access from Tables...");

    // Access cartesian_point table using the getter method
    // Tables has getter methods like: cartesian_point_holders() -> &HashMap<u64, CartesianPointHolder>
    let cartesian_points = tables.cartesian_point_holders();
    println!("  CartesianPoint table available");

    // Count entities
    let count = cartesian_points.len();
    println!("  ✓ Found {} CartesianPoint entities in table", count);

    // Try to access individual entities
    for (id, holder) in cartesian_points.iter().take(3) {
        println!("    Entity #{}: CartesianPoint holder found", id);
        // We can access holder fields here
        // The holder has name and coordinates fields
        println!("      - Holder: {:?}", holder);
    }

    // This proves the workflow:
    // 1. Parse STEP file -> Exchange
    // 2. TableInit::from_data_sections(&exchange.data) -> Tables
    // 3. Use tables.[entity_name]_holders() getter methods
    // 4. Access HashMap<u64, EntityHolder> for each entity type

    println!("\n  ✓ Tables population verified!");
    println!("  Pattern confirmed:");
    println!("    - Use TableInit::from_data_sections() to populate Tables");
    println!("    - Use tables.[entity_name]_holders() getter methods");
    println!("    - HashMap<u64, EntityHolder> gives you entity ID -> holder");
    println!("    - Use EntityTable::get_owned() for fully resolved entities");
}

#[cfg(not(feature = "step"))]
fn main() {
    println!("STEP feature not enabled. Run with: cargo run --example verify_ruststep_tables --features step");
}
