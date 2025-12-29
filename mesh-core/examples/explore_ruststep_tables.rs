// SPDX-License-Identifier: MIT OR Apache-2.0
// Experimental code to explore ruststep Tables API
// This is research code for understanding how to use ruststep AP203 Tables

#![allow(dead_code)]
#![allow(unused_imports)]
// Note: ap203 is enabled on ruststep dependency when step feature is enabled
// This warning is expected and can be ignored
#![allow(unexpected_cfgs)]

// Run with: cargo run --example explore_ruststep_tables --features step

#[cfg(feature = "step")]
use ruststep::{ast, parser};

// Try to import AP203 types
// Note: ap203 feature is enabled on ruststep when step feature is enabled
#[cfg(feature = "step")]
use ruststep::ap203::config_control_design;

#[cfg(feature = "step")]
fn main() {
    println!("Exploring ruststep Tables API...");

    // TODO: Create a simple STEP file string for testing
    let step_text = r#"ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Test'), '2;1');
FILE_NAME('test.step', '2025-12-27T00:00:00', ('Riley'), ('SimpleImageConverter'), '', '', '');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
ENDSEC;

DATA;
#1 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));
ENDSEC;

END-ISO-10303-21;
"#;

    // Parse STEP file
    match parser::parse(step_text) {
        Ok(exchange) => {
            println!("✓ Successfully parsed STEP file");
            println!("Number of data sections: {}", exchange.data.len());

            // Explore the Exchange structure
            for (i, data_section) in exchange.data.iter().enumerate() {
                println!(
                    "Data section {}: {} entities",
                    i,
                    data_section.entities.len()
                );

                // Explore entities
                for entity in &data_section.entities {
                    match entity {
                        ast::EntityInstance::Simple { id, record } => {
                            println!("  Entity #{}: {}", id, record.name);
                            println!("    Parameters: {:?}", record.parameter);
                        }
                        ast::EntityInstance::Complex { id, subsuper } => {
                            println!("  Complex Entity #{}: subtypes", id);
                            for record in subsuper {
                                println!("    Subtype: {}", record.name);
                            }
                        }
                    }
                }
            }

            // Try to build AP203 Tables
            #[cfg(feature = "step")]
            {
                println!("\nTrying to explore AP203 Tables API...");
                // Note: This is exploratory - actual usage may differ
                // let tables = config_control_design::Tables::default();
                println!("AP203 feature is available (enabled via ruststep dependency)");
            }
        }
        Err(e) => {
            println!("✗ Failed to parse STEP file: {}", e);
        }
    }
}

#[cfg(not(feature = "step"))]
fn main() {
    println!("STEP feature not enabled. Run with --features step");
}
