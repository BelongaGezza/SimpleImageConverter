// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for the `mesh-convert` CLI binary.

use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn mesh_convert_bin() -> &'static str {
    env!("CARGO_BIN_EXE_mesh-convert")
}

/// Create a minimal valid ASCII STL file for testing.
fn create_minimal_stl() -> Vec<u8> {
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
fn test_mesh_convert_help() {
    let output = Command::new(mesh_convert_bin())
        .arg("--help")
        .output()
        .expect("Failed to execute mesh-convert --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("mesh-convert"));
    assert!(stdout.contains("Convert between 3D mesh formats"));
}

#[test]
fn test_mesh_convert_invalid_file() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_file = temp_dir.path().join("nonexistent.stl");

    let output = Command::new(mesh_convert_bin())
        .args([nonexistent_file.to_str().unwrap(), "obj"])
        .output()
        .expect("Failed to execute mesh-convert with missing input");

    assert!(!output.status.success());
}

#[test]
fn test_mesh_convert_help_includes_new_options() {
    let output = Command::new(mesh_convert_bin())
        .arg("--help")
        .output()
        .expect("Failed to execute mesh-convert --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("--transform") || stdout.contains("transform"));
    assert!(stdout.contains("--recalculate-normals") || stdout.contains("recalculate-normals"));
    assert!(stdout.contains("--validate") || stdout.contains("validate"));
}

#[test]
fn test_mesh_convert_invalid_transform_option() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    fs::write(&input_file, create_minimal_stl()).unwrap();

    let output = Command::new(mesh_convert_bin())
        .args([
            input_file.to_str().unwrap(),
            "obj",
            "--transform",
            "invalid-option",
        ])
        .output()
        .expect("Failed to execute mesh-convert with invalid transform");

    assert!(!output.status.success());
}

#[test]
fn test_mesh_convert_transform_option() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    fs::write(&input_file, create_minimal_stl()).unwrap();

    let output = Command::new(mesh_convert_bin())
        .args([
            input_file.to_str().unwrap(),
            "obj",
            "--transform",
            "y-up",
            "-o",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute mesh-convert with transform option");

    assert!(
        output.status.success(),
        "Transform option failed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
fn test_mesh_convert_recalculate_normals_option() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    fs::write(&input_file, create_minimal_stl()).unwrap();

    let output = Command::new(mesh_convert_bin())
        .args([
            input_file.to_str().unwrap(),
            "obj",
            "--recalculate-normals",
            "-o",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute mesh-convert with recalculate-normals");

    assert!(
        output.status.success(),
        "Recalculate normals option failed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
fn test_mesh_convert_validate_option() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    fs::write(&input_file, create_minimal_stl()).unwrap();

    let output = Command::new(mesh_convert_bin())
        .args([
            input_file.to_str().unwrap(),
            "obj",
            "--validate",
            "-o",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute mesh-convert with validate option");

    assert!(
        output.status.success(),
        "Validate option failed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_file.exists(), "Output file was not created");
}

#[test]
fn test_mesh_convert_combined_options() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.stl");
    let output_file = temp_dir.path().join("output.obj");
    fs::write(&input_file, create_minimal_stl()).unwrap();

    let output = Command::new(mesh_convert_bin())
        .args([
            input_file.to_str().unwrap(),
            "obj",
            "--transform",
            "z-up",
            "--recalculate-normals",
            "--validate",
            "-o",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute mesh-convert with combined options");

    assert!(
        output.status.success(),
        "Combined options failed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_file.exists(), "Output file was not created");
}
