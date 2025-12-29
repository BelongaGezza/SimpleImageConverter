// SPDX-License-Identifier: MIT OR Apache-2.0
// Verify test STEP files contain FACETED_BREP entities
// Run with: cargo run --example verify_test_step_files --features step -- <path-to-step-file>

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_minimal_cfg)]

#[cfg(feature = "step")]
use ruststep::ap203::config_control_design;
#[cfg(feature = "step")]
use ruststep::parser;
#[cfg(feature = "step")]
use ruststep::tables::TableInit;

#[cfg(feature = "step")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example verify_test_step_files --features step -- <path-to-step-file>");
        eprintln!("\nExample:");
        eprintln!("  cargo run --example verify_test_step_files --features step -- tests/data/simple_faceted_brep.step");
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    
    println!("=== Verifying STEP Test File ===\n");
    println!("File: {}\n", file_path);
    
    // Read file
    let step_text = match std::fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    
    // Parse STEP file
    let exchange = match parser::parse(&step_text) {
        Ok(exchange) => exchange,
        Err(e) => {
            eprintln!("❌ Error parsing STEP file: {}", e);
            std::process::exit(1);
        }
    };
    
    // Deserialize into Tables
    let tables = match config_control_design::Tables::from_data_sections(&exchange.data) {
        Ok(tables) => tables,
        Err(e) => {
            eprintln!("❌ Error deserializing STEP entities: {:?}", e);
            std::process::exit(1);
        }
    };
    
    // Check for FACETED_BREP entities
    let fb_holders = tables.faceted_brep_holders();
    println!("✓ FACETED_BREP entities found: {}", fb_holders.len());
    
    if fb_holders.is_empty() {
        println!("\n⚠️  WARNING: No FACETED_BREP entities found!");
        println!("   This file may not be suitable for v0.2.0 testing.");
        
        // Check for other entity types
        let msb_holders = tables.manifold_solid_brep_holders();
        let cs_holders = tables.closed_shell_holders();
        
        if !msb_holders.is_empty() {
            println!("   Found {} MANIFOLD_SOLID_BREP entities (not supported in v0.2.0)", msb_holders.len());
        }
        if !cs_holders.is_empty() {
            println!("   Found {} CLOSED_SHELL entities (not supported in v0.2.0)", cs_holders.len());
        }
        
        std::process::exit(1);
    }
    
    // Print details
    println!("\nFACETED_BREP Entity Details:");
    println!("{:-<60}", "");
    for (id, holder) in fb_holders.iter() {
        println!("  Entity #{}: {:?}", id, holder);
        // Note: Accessing holder fields requires more complex traversal
        // For now, we just verify the entities exist
    }
    
    println!("\n✅ File verification complete!");
    println!("   This file is suitable for v0.2.0 FACETED_BREP testing.");
}

#[cfg(not(feature = "step"))]
fn main() {
    eprintln!("STEP feature not enabled. Build with --features step");
    std::process::exit(1);
}

