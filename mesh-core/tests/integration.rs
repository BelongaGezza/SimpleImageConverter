// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use mesh_core::{FormatRegistry, MeshConverter, MeshFormat, Mesh, mesh::Vertex, mesh::Face};

/// Helper to create a simple test mesh (a single triangle)
fn create_test_triangle() -> Mesh {
    let mut mesh = Mesh::new();
    
    // Single triangle with three vertices
    mesh.vertices.push(Vertex { x: 0.0, y: 0.0, z: 0.0 });
    mesh.vertices.push(Vertex { x: 1.0, y: 0.0, z: 0.0 });
    mesh.vertices.push(Vertex { x: 0.5, y: 1.0, z: 0.0 });
    
    // One face
    mesh.faces.push(Face {
        indices: [0, 1, 2],
    });
    
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
    let converted_data = converter.convert(&stl_data, reader.as_ref(), writer.as_ref()).unwrap();
    
    // Verify converted data is valid STL
    assert!(!converted_data.is_empty());
    
    // Read back the converted data
    let read_result = reader.read(&converted_data);
    assert!(read_result.is_ok());
    
    let read_mesh = read_result.unwrap();
    assert_eq!(read_mesh.vertices.len(), original_mesh.vertices.len());
    assert_eq!(read_mesh.faces.len(), original_mesh.faces.len());
}

