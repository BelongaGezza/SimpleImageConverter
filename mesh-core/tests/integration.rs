// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use mesh_core::{mesh::Face, mesh::Vertex, FormatRegistry, Mesh, MeshConverter, MeshFormat};

/// Helper to create a simple test mesh (a single triangle)
fn create_test_triangle() -> Mesh {
    let mut mesh = Mesh::new();

    // Single triangle with three vertices
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

    // One face
    mesh.faces.push(Face { indices: [0, 1, 2] });

    mesh
}

#[test]
fn test_stl_round_trip_conversion() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Get format handlers
    let reader = FormatRegistry::get_reader(MeshFormat::Stl).unwrap();
    let writer = FormatRegistry::get_writer(MeshFormat::Stl).unwrap();

    // Write mesh to STL
    let stl_data = writer.write(&original_mesh).unwrap();
    assert!(!stl_data.is_empty());

    // Read STL back
    let read_result = reader.read(&stl_data);
    assert!(read_result.is_ok());

    let read_mesh = read_result.unwrap();

    // Verify structure matches
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());

    // Verify vertices are approximately the same (allowing for floating point precision)
    for (original, read) in original_mesh.vertices.iter().zip(read_mesh.vertices.iter()) {
        assert!((original.x - read.x).abs() < 0.001);
        assert!((original.y - read.y).abs() < 0.001);
        assert!((original.z - read.z).abs() < 0.001);
    }
}

#[test]
fn test_mesh_converter_stl_round_trip() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Get format handlers
    let reader = FormatRegistry::get_reader(MeshFormat::Stl).unwrap();
    let writer = FormatRegistry::get_writer(MeshFormat::Stl).unwrap();

    // Write mesh to STL using writer
    let stl_data = writer.write(&original_mesh).unwrap();

    // Use MeshConverter for round-trip conversion
    let converter = MeshConverter::new();
    let converted_data = converter
        .convert(&stl_data, reader.as_ref(), writer.as_ref())
        .unwrap();

    // Verify converted data is valid STL
    assert!(!converted_data.is_empty());

    // Read back the converted data
    let read_result = reader.read(&converted_data);
    assert!(read_result.is_ok());

    let read_mesh = read_result.unwrap();
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}

#[test]
fn test_obj_round_trip_conversion() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Get format handlers
    let reader = FormatRegistry::get_reader(MeshFormat::Obj).unwrap();
    let writer = FormatRegistry::get_writer(MeshFormat::Obj).unwrap();

    // Write mesh to OBJ
    let obj_data = writer.write(&original_mesh).unwrap();
    assert!(!obj_data.is_empty());

    // Read OBJ back
    let read_result = reader.read(&obj_data);
    assert!(read_result.is_ok());

    let read_mesh = read_result.unwrap();

    // Verify structure matches
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());

    // Verify vertices are approximately the same (allowing for floating point precision)
    for (original, read) in original_mesh.vertices.iter().zip(read_mesh.vertices.iter()) {
        assert!((original.x - read.x).abs() < 0.001);
        assert!((original.y - read.y).abs() < 0.001);
        assert!((original.z - read.z).abs() < 0.001);
    }
}

#[test]
fn test_mesh_converter_obj_round_trip() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Get format handlers
    let reader = FormatRegistry::get_reader(MeshFormat::Obj).unwrap();
    let writer = FormatRegistry::get_writer(MeshFormat::Obj).unwrap();

    // Write mesh to OBJ using writer
    let obj_data = writer.write(&original_mesh).unwrap();

    // Use MeshConverter for round-trip conversion
    let converter = MeshConverter::new();
    let converted_data = converter
        .convert(&obj_data, reader.as_ref(), writer.as_ref())
        .unwrap();

    // Verify converted data is valid OBJ
    assert!(!converted_data.is_empty());

    // Read back the converted data
    let read_result = reader.read(&converted_data);
    assert!(read_result.is_ok());

    let read_mesh = read_result.unwrap();
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}

#[test]
fn test_ply_round_trip_conversion() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Get format handlers
    let reader = FormatRegistry::get_reader(MeshFormat::Ply).unwrap();
    let writer = FormatRegistry::get_writer(MeshFormat::Ply).unwrap();

    // Write mesh to PLY
    let ply_data = writer.write(&original_mesh).unwrap();
    assert!(!ply_data.is_empty());

    // Read PLY back
    let read_result = reader.read(&ply_data);
    assert!(read_result.is_ok());

    let read_mesh = read_result.unwrap();

    // Verify structure matches
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());

    // Verify vertices are approximately the same (allowing for floating point precision)
    for (original, read) in original_mesh.vertices.iter().zip(read_mesh.vertices.iter()) {
        assert!((original.x - read.x).abs() < 0.001);
        assert!((original.y - read.y).abs() < 0.001);
        assert!((original.z - read.z).abs() < 0.001);
    }
}

#[test]
fn test_mesh_converter_ply_round_trip() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Get format handlers
    let reader = FormatRegistry::get_reader(MeshFormat::Ply).unwrap();
    let writer = FormatRegistry::get_writer(MeshFormat::Ply).unwrap();

    // Write mesh to PLY using writer
    let ply_data = writer.write(&original_mesh).unwrap();

    // Use MeshConverter for round-trip conversion
    let converter = MeshConverter::new();
    let converted_data = converter
        .convert(&ply_data, reader.as_ref(), writer.as_ref())
        .unwrap();

    // Verify converted data is valid PLY
    assert!(!converted_data.is_empty());

    // Read back the converted data
    let read_result = reader.read(&converted_data);
    assert!(read_result.is_ok());

    let read_mesh = read_result.unwrap();
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}

#[test]
fn test_cross_format_conversion_stl_to_obj() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Write to STL
    let stl_writer = FormatRegistry::get_writer(MeshFormat::Stl).unwrap();
    let stl_data = stl_writer.write(&original_mesh).unwrap();

    // Convert STL to OBJ
    let stl_reader = FormatRegistry::get_reader(MeshFormat::Stl).unwrap();
    let obj_writer = FormatRegistry::get_writer(MeshFormat::Obj).unwrap();
    let converter = MeshConverter::new();
    let obj_data = converter
        .convert(&stl_data, stl_reader.as_ref(), obj_writer.as_ref())
        .unwrap();

    // Verify OBJ data is valid
    assert!(!obj_data.is_empty());

    // Read OBJ back
    let obj_reader = FormatRegistry::get_reader(MeshFormat::Obj).unwrap();
    let read_mesh = obj_reader.read(&obj_data).unwrap();

    // Verify structure
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}

#[test]
fn test_cross_format_conversion_obj_to_ply() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Write to OBJ
    let obj_writer = FormatRegistry::get_writer(MeshFormat::Obj).unwrap();
    let obj_data = obj_writer.write(&original_mesh).unwrap();

    // Convert OBJ to PLY
    let obj_reader = FormatRegistry::get_reader(MeshFormat::Obj).unwrap();
    let ply_writer = FormatRegistry::get_writer(MeshFormat::Ply).unwrap();
    let converter = MeshConverter::new();
    let ply_data = converter
        .convert(&obj_data, obj_reader.as_ref(), ply_writer.as_ref())
        .unwrap();

    // Verify PLY data is valid
    assert!(!ply_data.is_empty());

    // Read PLY back
    let ply_reader = FormatRegistry::get_reader(MeshFormat::Ply).unwrap();
    let read_mesh = ply_reader.read(&ply_data).unwrap();

    // Verify structure
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}

#[test]
fn test_cross_format_conversion_ply_to_stl() {
    // Create test mesh
    let original_mesh = create_test_triangle();

    // Write to PLY
    let ply_writer = FormatRegistry::get_writer(MeshFormat::Ply).unwrap();
    let ply_data = ply_writer.write(&original_mesh).unwrap();

    // Convert PLY to STL
    let ply_reader = FormatRegistry::get_reader(MeshFormat::Ply).unwrap();
    let stl_writer = FormatRegistry::get_writer(MeshFormat::Stl).unwrap();
    let converter = MeshConverter::new();
    let stl_data = converter
        .convert(&ply_data, ply_reader.as_ref(), stl_writer.as_ref())
        .unwrap();

    // Verify STL data is valid
    assert!(!stl_data.is_empty());

    // Read STL back
    let stl_reader = FormatRegistry::get_reader(MeshFormat::Stl).unwrap();
    let read_mesh = stl_reader.read(&stl_data).unwrap();

    // Verify structure
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}
