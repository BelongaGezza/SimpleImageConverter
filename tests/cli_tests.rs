// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for CLI tools
//!
//! These tests verify end-to-end functionality of the CLI tools.

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn get_img_convert_binary() -> String {
    // In CI, binaries are in target/release
    // Locally, they might be in target/debug
    let release_path = "target/release/img-convert";
    let debug_path = "target/debug/img-convert";
    
    if Path::new(release_path).exists() {
        release_path.to_string()
    } else if Path::new(debug_path).exists() {
        debug_path.to_string()
    } else {
        // Try cargo run for development
        "cargo".to_string()
    }
}

fn get_mesh_convert_binary() -> String {
    let release_path = "target/release/mesh-convert";
    let debug_path = "target/debug/mesh-convert";
    
    if Path::new(release_path).exists() {
        release_path.to_string()
    } else if Path::new(debug_path).exists() {
        debug_path.to_string()
    } else {
        "cargo".to_string()
    }
}

#[test]
#[ignore] // CLI integration test - run with `cargo test -- --ignored` after building binaries
fn test_img_convert_help() {
    let binary = get_img_convert_binary();
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "img-convert", "--", "--help"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .arg("--help")
            .output()
            .expect("Failed to execute command")
    };
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("img-convert"));
    assert!(stdout.contains("Convert between 2D image formats"));
}

#[test]
#[ignore] // CLI integration test - run with `cargo test -- --ignored` after building binaries
fn test_mesh_convert_help() {
    let binary = get_mesh_convert_binary();
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--", "--help"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .arg("--help")
            .output()
            .expect("Failed to execute command")
    };
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("mesh-convert"));
    assert!(stdout.contains("Convert between 3D mesh formats"));
}

#[test]
#[ignore] // Requires test data
fn test_img_convert_invalid_quality() {
    let binary = get_img_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.png");
    
    // Create a minimal valid PNG file for testing
    // (In real tests, this would use actual test data)
    
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "img-convert", "--", 
                   test_file.to_str().unwrap(), "jpg", "--quality", "101"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[test_file.to_str().unwrap(), "jpg", "--quality", "101"])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should fail with invalid quality
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Quality") || stderr.contains("quality"));
}

#[test]
#[ignore] // Requires test data
fn test_mesh_convert_invalid_file() {
    let binary = get_mesh_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_file = temp_dir.path().join("nonexistent.stl");
    
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--",
                   nonexistent_file.to_str().unwrap(), "obj"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[nonexistent_file.to_str().unwrap(), "obj"])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should fail with file not found
    assert!(!output.status.success());
}

/// Create a minimal valid ASCII STL file for testing
/// Returns a simple triangle as STL
fn create_minimal_stl() -> Vec<u8> {
    // Minimal valid ASCII STL with a single triangle
    let stl_content = r#"solid TestTriangle
  facet normal 0.0 0.0 1.0
    outer loop
      vertex 0.0 0.0 0.0
      vertex 1.0 0.0 0.0
      vertex 0.5 1.0 0.0
    endloop
  endfacet
endsolid TestTriangle
"#;
    stl_content.as_bytes().to_vec()
}

#[test]
#[ignore] // Requires built binaries
fn test_mesh_convert_help_includes_new_options() {
    let binary = get_mesh_convert_binary();
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--", "--help"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .arg("--help")
            .output()
            .expect("Failed to execute command")
    };
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Check for v0.1.1 features in help text
    assert!(stdout.contains("--transform") || stdout.contains("transform"));
    assert!(stdout.contains("--recalculate-normals") || stdout.contains("recalculate-normals"));
    assert!(stdout.contains("--validate") || stdout.contains("validate"));
}

#[test]
#[ignore] // Requires built binaries and test data
fn test_mesh_convert_invalid_transform_option() {
    let binary = get_mesh_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    
    // Create minimal STL file
    let stl_data = create_minimal_stl();
    fs::write(&input_file, &stl_data).unwrap();
    
    // Try invalid transform option
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--",
                   input_file.to_str().unwrap(),
                   "obj",
                   "--transform", "invalid-option"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[input_file.to_str().unwrap(),
                   "obj",
                   "--transform", "invalid-option"])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should fail with invalid transform option
    assert!(!output.status.success());
}

#[test]
#[ignore] // Requires built binaries and test data
fn test_mesh_convert_transform_option() {
    let binary = get_mesh_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    
    // Create minimal STL file
    let stl_data = create_minimal_stl();
    fs::write(&input_file, &stl_data).unwrap();
    
    // Test valid transform option (y-up)
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--",
                   input_file.to_str().unwrap(),
                   "obj",
                   "--transform", "y-up",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[input_file.to_str().unwrap(),
                   "obj",
                   "--transform", "y-up",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should succeed with valid transform option
    assert!(output.status.success(), 
            "Transform option failed. stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    // Verify output file was created
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
#[ignore] // Requires built binaries and test data
fn test_mesh_convert_recalculate_normals_option() {
    let binary = get_mesh_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    
    // Create minimal STL file
    let stl_data = create_minimal_stl();
    fs::write(&input_file, &stl_data).unwrap();
    
    // Test recalculate-normals option
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--",
                   input_file.to_str().unwrap(),
                   "obj",
                   "--recalculate-normals",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[input_file.to_str().unwrap(),
                   "obj",
                   "--recalculate-normals",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should succeed with recalculate-normals option
    assert!(output.status.success(),
            "Recalculate normals option failed. stderr: {}",
            String::from_utf8_lossy(&output.stderr));
    
    // Verify output file was created
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
#[ignore] // Requires built binaries and test data
fn test_mesh_convert_validate_option() {
    let binary = get_mesh_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    
    // Create minimal STL file
    let stl_data = create_minimal_stl();
    fs::write(&input_file, &stl_data).unwrap();
    
    // Test validate option
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--",
                   input_file.to_str().unwrap(),
                   "obj",
                   "--validate",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[input_file.to_str().unwrap(),
                   "obj",
                   "--validate",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should succeed with validate option (assuming valid mesh)
    assert!(output.status.success(),
            "Validate option failed. stderr: {}",
            String::from_utf8_lossy(&output.stderr));
    
    // Verify output file was created
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
#[ignore] // Requires built binaries and test data
fn test_mesh_convert_combined_options() {
    let binary = get_mesh_convert_binary();
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    
    // Create minimal STL file
    let stl_data = create_minimal_stl();
    fs::write(&input_file, &stl_data).unwrap();
    
    // Test all options together
    let output = if binary == "cargo" {
        Command::new("cargo")
            .args(&["run", "--bin", "mesh-convert", "--",
                   input_file.to_str().unwrap(),
                   "obj",
                   "--transform", "z-up",
                   "--recalculate-normals",
                   "--validate",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new(&binary)
            .args(&[input_file.to_str().unwrap(),
                   "obj",
                   "--transform", "z-up",
                   "--recalculate-normals",
                   "--validate",
                   "-o", output_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command")
    };
    
    // Should succeed with all options combined
    assert!(output.status.success(),
            "Combined options failed. stderr: {}",
            String::from_utf8_lossy(&output.stderr));
    
    // Verify output file was created
    assert!(output_file.exists(), "Output file was not created");
}

