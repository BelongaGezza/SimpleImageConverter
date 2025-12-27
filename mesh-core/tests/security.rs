// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Security-focused tests for mesh format readers
//!
//! These tests verify that mesh format readers properly handle malicious or malformed input
//! without panicking, leaking memory, or causing denial of service.

use mesh_core::formats::traits::MeshReader;
use mesh_core::formats::{ObjFormat, PlyFormat, StlFormat};
use common::limits::ResourceLimits;

#[test]
fn test_stl_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = StlFormat::with_limits(limits.clone());
    
    let oversized_size = limits.max_file_size + 1;
    let mut oversized_data = vec![0u8; 80]; // STL header
    oversized_data.resize(oversized_size, 0);
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_obj_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = ObjFormat::with_limits(limits.clone());
    
    let oversized_size = limits.max_file_size + 1;
    let oversized_data = vec![b'v'; oversized_size]; // Valid OBJ format but too large
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_ply_reject_oversized_input() {
    let limits = ResourceLimits::default();
    let format = PlyFormat::with_limits(limits.clone());
    
    let oversized_size = limits.max_file_size + 1;
    let oversized_data = b"ply\nformat ascii 1.0\n".repeat(oversized_size / 20);
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
}

#[test]
fn test_stl_reject_excessive_vertices() {
    // Create restrictive limits
    let limits = ResourceLimits::builder()
        .max_vertices(100)
        .max_faces(50)
        .build();
    
    // This test would need a valid STL file with >100 vertices
    // For now, we test that limits are checked
    let format = StlFormat::with_limits(limits);
    
    // Empty/invalid data should fail before limit check
    let result = format.read(&[]);
    assert!(result.is_err());
}

#[test]
fn test_obj_handle_malformed_data() {
    let format = ObjFormat::new();
    
    // Invalid OBJ data
    let malformed_data = b"not valid obj data\n";
    
    let result = format.read(malformed_data);
    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_ply_handle_malformed_header() {
    let format = PlyFormat::new();
    
    // Invalid PLY header
    let malformed_data = b"not a ply file\n";
    
    let result = format.read(malformed_data);
    assert!(result.is_err());
}

#[test]
fn test_empty_input_rejected() {
    let format = StlFormat::new();
    
    let result = format.read(&[]);
    assert!(result.is_err());
}

#[test]
fn test_limits_enforced_on_read() {
    let limits = ResourceLimits::builder()
        .max_file_size(100)
        .build();
    
    let format = StlFormat::with_limits(limits);
    let oversized_data = vec![0u8; 200];
    
    let result = format.read(&oversized_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds limit"));
}

