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
#[ignore] // Requires built binaries
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
#[ignore] // Requires built binaries
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

