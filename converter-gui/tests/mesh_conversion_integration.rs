// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! GUI mesh conversion integration tests (Sprint 13 Task 3.1 support).

use common::limits::ResourceLimits;
use converter_gui::conversion::convert_mesh;
use mesh_core::{
    mesh::{Face, Vertex},
    ConversionOptions, FormatRegistry, Mesh, MeshFormat,
};
use std::path::Path;
use tempfile::TempDir;

fn create_test_triangle() -> Mesh {
    let mut mesh = Mesh::new();
    mesh.vertices.push(Vertex {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
    mesh.vertices.push(Vertex {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    });
    mesh.vertices.push(Vertex {
        x: 0.5,
        y: 1.0,
        z: 0.0,
    });
    mesh.faces.push(Face { indices: [0, 1, 2] });
    mesh
}

fn write_stl_fixture(path: &Path) {
    let mesh = create_test_triangle();
    let writer = FormatRegistry::get_writer(MeshFormat::Stl).unwrap();
    let data = writer.write(&mesh).unwrap();
    std::fs::write(path, data).unwrap();
}

#[test]
fn test_gui_convert_mesh_stl_to_obj() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("triangle.stl");
    let output = temp.path().join("triangle.obj");
    write_stl_fixture(&input);

    let limits = ResourceLimits::default();
    let options = ConversionOptions::default();

    let result = convert_mesh(&input, &output, MeshFormat::Obj, options, &limits);
    assert!(result.is_ok(), "STL→OBJ failed: {:?}", result.err());
    assert!(output.exists());

    let obj_data = std::fs::read(&output).unwrap();
    assert!(!obj_data.is_empty());
    let reader = FormatRegistry::get_reader(MeshFormat::Obj).unwrap();
    let mesh = reader.read(&obj_data).unwrap();
    assert_eq!(mesh.vertices.len(), 3);
    assert_eq!(mesh.faces.len(), 1);
}

#[test]
fn test_gui_convert_mesh_stl_to_ply() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("triangle.stl");
    let output = temp.path().join("triangle.ply");
    write_stl_fixture(&input);

    let limits = ResourceLimits::default();
    let options = ConversionOptions::default();

    let result = convert_mesh(&input, &output, MeshFormat::Ply, options, &limits);
    assert!(result.is_ok(), "STL→PLY failed: {:?}", result.err());
    assert!(output.exists());

    let ply_data = std::fs::read(&output).unwrap();
    let reader = FormatRegistry::get_reader(MeshFormat::Ply).unwrap();
    assert!(reader.read(&ply_data).is_ok());
}

#[test]
fn test_gui_convert_mesh_rejects_extension_signature_mismatch() {
    let temp = TempDir::new().unwrap();
    // Write GLB magic bytes to a .stl path — ADR-003 should reject at two-stage detection.
    let input = temp.path().join("spoof.stl");
    let output = temp.path().join("spoof.obj");
    std::fs::write(&input, b"glTF\x00\x00\x00\x00").unwrap();

    let limits = ResourceLimits::default();
    let options = ConversionOptions::default();

    let result = convert_mesh(&input, &output, MeshFormat::Obj, options, &limits);
    assert!(result.is_err(), "expected format mismatch rejection");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("mismatch") || msg.contains("Invalid format") || msg.contains("invalid"),
        "unexpected error: {msg}"
    );
}
