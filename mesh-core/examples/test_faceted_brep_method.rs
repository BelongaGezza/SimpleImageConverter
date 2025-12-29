// SPDX-License-Identifier: MIT OR Apache-2.0
// Quick test to see if faceted_brep_holders() method exists in ruststep
// This will fail to compile if the method doesn't exist

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(feature = "step")]
fn test_faceted_brep_method() {
    use ruststep::ap203::config_control_design::Tables;
    
    let tables = Tables::default();
    
    // Test if faceted_brep_holders() exists
    // If this compiles, the method exists!
    let _fb_holders = tables.faceted_brep_holders();
}

#[cfg(feature = "step")]
fn main() {
    test_faceted_brep_method();
}

#[cfg(not(feature = "step"))]
fn main() {
    println!("STEP feature not enabled");
}

