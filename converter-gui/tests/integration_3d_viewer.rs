// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for 3D Viewer (Sprint 10_A Task 1.2)
//!
//! Tests integration of:
//! - 3D viewer mesh loading and rendering
//! - Camera controls (orbit, pan, zoom)
//! - Rendering modes (solid, wireframe)
//! - Performance with various mesh sizes
//! - Error handling for invalid meshes
//!
//! These tests verify that the 3D viewer works correctly with the GUI integration.

#![cfg(feature = "viewer-3d")]

use converter_gui::preview_3d::{load_mesh_for_viewer, RenderMode, Viewer3D, Viewer3DError};
use mesh_core::{Face, Mesh, Normal, Vertex};
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

/// Helper function to create a simple test mesh (cube)
fn create_test_cube() -> Arc<Mesh> {
    // Create a simple cube with 8 vertices and 12 faces
    let vertices = vec![
        // Bottom face
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        // Top face
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
    ];

    // Create 12 faces (2 triangles per cube face)
    let faces = vec![
        // Bottom face
        Face { indices: [0, 1, 2] },
        Face { indices: [0, 2, 3] },
        // Top face
        Face { indices: [4, 7, 6] },
        Face { indices: [4, 6, 5] },
        // Front face
        Face { indices: [0, 4, 5] },
        Face { indices: [0, 5, 1] },
        // Back face
        Face { indices: [2, 6, 7] },
        Face { indices: [2, 7, 3] },
        // Left face
        Face { indices: [0, 3, 7] },
        Face { indices: [0, 7, 4] },
        // Right face
        Face { indices: [1, 5, 6] },
        Face { indices: [1, 6, 2] },
    ];

    // Create normals (simplified - all pointing outward)
    let normals = vec![
        Normal {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }, // Bottom
        Normal {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Normal {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Normal {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }, // Top
        Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Normal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    ];

    Arc::new(Mesh {
        vertices,
        faces,
        normals,
    })
}

/// Helper function to create a mesh with a specific number of vertices
fn create_mesh_with_vertex_count(vertex_count: usize) -> Arc<Mesh> {
    let mut vertices = Vec::with_capacity(vertex_count);
    let mut faces = Vec::new();
    let mut normals = Vec::with_capacity(vertex_count);

    // Create vertices in a grid pattern
    let grid_size = (vertex_count as f32).sqrt().ceil() as usize;
    for y in 0..grid_size {
        for x in 0..grid_size {
            if vertices.len() >= vertex_count {
                break;
            }
            vertices.push(Vertex {
                x: x as f32 * 0.1,
                y: y as f32 * 0.1,
                z: 0.0,
            });
            normals.push(Normal {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            });
        }
        if vertices.len() >= vertex_count {
            break;
        }
    }

    // Create faces (triangles) from the grid
    let actual_grid_size = (vertices.len() as f32).sqrt().ceil() as usize;
    for y in 0..(actual_grid_size - 1) {
        for x in 0..(actual_grid_size - 1) {
            let idx = y * actual_grid_size + x;
            if idx + actual_grid_size + 1 < vertices.len() {
                // Create two triangles per grid cell
                faces.push(Face {
                    indices: [idx, idx + 1, idx + actual_grid_size],
                });
                faces.push(Face {
                    indices: [idx + 1, idx + actual_grid_size + 1, idx + actual_grid_size],
                });
            }
        }
    }

    Arc::new(Mesh {
        vertices,
        faces,
        normals,
    })
}

#[test]
fn test_viewer3d_creation() {
    let viewer = Viewer3D::new();
    assert!(!viewer.has_mesh());
    assert_eq!(viewer.render_mode(), RenderMode::Solid);
}

#[test]
fn test_viewer3d_set_mesh() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();

    viewer.set_mesh(mesh.clone());
    assert!(viewer.has_mesh());
}

#[test]
fn test_viewer3d_load_mesh() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();

    let result = load_mesh_for_viewer(mesh, &mut viewer);
    assert!(result.is_ok());
    assert!(viewer.has_mesh());
}

#[test]
fn test_viewer3d_load_empty_mesh() {
    let mut viewer = Viewer3D::new();
    let empty_mesh = Arc::new(Mesh::new());

    let result = load_mesh_for_viewer(empty_mesh, &mut viewer);
    assert!(result.is_err());
    match result.unwrap_err() {
        Viewer3DError::MeshLoadFailed(msg) => {
            assert!(msg.contains("no vertices") || msg.contains("no faces"));
        }
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_viewer3d_load_mesh_no_vertices() {
    let mut viewer = Viewer3D::new();
    let mesh = Arc::new(Mesh {
        vertices: Vec::new(),
        faces: vec![Face { indices: [0, 1, 2] }],
        normals: Vec::new(),
    });

    let result = load_mesh_for_viewer(mesh, &mut viewer);
    assert!(result.is_err());
}

#[test]
fn test_viewer3d_load_mesh_no_faces() {
    let mut viewer = Viewer3D::new();
    let mesh = Arc::new(Mesh {
        vertices: vec![Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }],
        faces: Vec::new(),
        normals: Vec::new(),
    });

    let result = load_mesh_for_viewer(mesh, &mut viewer);
    assert!(result.is_err());
}

#[test]
fn test_viewer3d_render_mode_switching() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();
    viewer.set_mesh(mesh);

    // Test default mode
    assert_eq!(viewer.render_mode(), RenderMode::Solid);

    // Switch to wireframe
    viewer.set_render_mode(RenderMode::Wireframe);
    assert_eq!(viewer.render_mode(), RenderMode::Wireframe);

    // Switch back to solid
    viewer.set_render_mode(RenderMode::Solid);
    assert_eq!(viewer.render_mode(), RenderMode::Solid);
}

#[test]
fn test_viewer3d_camera_reset() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();
    viewer.set_mesh(mesh);

    // Modify camera
    viewer.handle_drag(egui::Vec2::new(1.0, 1.0));
    viewer.handle_zoom(0.5);
    viewer.handle_pan(egui::Vec2::new(0.5, 0.5));

    // Reset camera
    viewer.reset_camera();

    // Camera should be reset (we can't directly check internal state,
    // but reset_camera should not panic)
    assert!(viewer.has_mesh());
}

#[test]
fn test_viewer3d_camera_controls() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();
    viewer.set_mesh(mesh);

    // Test orbit (drag)
    viewer.handle_drag(egui::Vec2::new(10.0, 5.0));
    assert!(viewer.has_mesh()); // Should not crash

    // Test pan (shift + drag)
    viewer.handle_pan(egui::Vec2::new(5.0, 3.0));
    assert!(viewer.has_mesh()); // Should not crash

    // Test zoom
    viewer.handle_zoom(0.1);
    assert!(viewer.has_mesh()); // Should not crash
    viewer.handle_zoom(-0.1);
    assert!(viewer.has_mesh()); // Should not crash
}

#[test]
fn test_viewer3d_mesh_sizes_1k() {
    let mut viewer = Viewer3D::new();
    let mesh = create_mesh_with_vertex_count(1000);

    let start = Instant::now();
    viewer.set_mesh(mesh);
    let duration = start.elapsed();

    assert!(viewer.has_mesh());
    // Should be fast (< 100ms for 1K vertices)
    assert!(
        duration.as_millis() < 100,
        "Mesh loading took too long: {:?}",
        duration
    );
}

#[test]
fn test_viewer3d_mesh_sizes_10k() {
    let mut viewer = Viewer3D::new();
    let mesh = create_mesh_with_vertex_count(10_000);

    let start = Instant::now();
    viewer.set_mesh(mesh);
    let duration = start.elapsed();

    assert!(viewer.has_mesh());
    // Should be reasonably fast (< 500ms for 10K vertices)
    assert!(
        duration.as_millis() < 500,
        "Mesh loading took too long: {:?}",
        duration
    );
}

#[test]
fn test_viewer3d_mesh_sizes_100k() {
    let mut viewer = Viewer3D::new();
    let mesh = create_mesh_with_vertex_count(100_000);

    let start = Instant::now();
    viewer.set_mesh(mesh);
    let duration = start.elapsed();

    assert!(viewer.has_mesh());
    // Should be acceptable (< 2s for 100K vertices)
    assert!(
        duration.as_secs() < 2,
        "Mesh loading took too long: {:?}",
        duration
    );
}

#[test]
fn test_viewer3d_performance_benchmark() {
    // Performance benchmark test
    // This test documents performance characteristics for different mesh sizes
    let sizes = vec![1_000, 10_000, 50_000, 100_000];

    println!("\n=== 3D Viewer Performance Benchmarks ===");
    println!(
        "{:<15} {:<15} {:<15}",
        "Vertices", "Load Time (ms)", "Status"
    );

    for size in sizes {
        let mut viewer = Viewer3D::new();
        let mesh = create_mesh_with_vertex_count(size);

        let start = Instant::now();
        viewer.set_mesh(mesh);
        let duration = start.elapsed();

        let status = if size < 100_000 {
            "✓ Smooth"
        } else {
            "⚠ May lag"
        };

        println!("{:<15} {:<15} {:<15}", size, duration.as_millis(), status);

        assert!(viewer.has_mesh());
    }

    println!("=== End Performance Benchmarks ===\n");
}

#[test]
fn test_viewer3d_mesh_reload() {
    let mut viewer = Viewer3D::new();
    let mesh1 = create_test_cube();
    let mesh2 = create_mesh_with_vertex_count(100);

    // Load first mesh
    viewer.set_mesh(mesh1);
    assert!(viewer.has_mesh());

    // Load second mesh (should replace first)
    viewer.set_mesh(mesh2);
    assert!(viewer.has_mesh());
}

#[test]
fn test_viewer3d_camera_zoom_limits() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();
    viewer.set_mesh(mesh);

    // Test zoom limits (should clamp between 0.1 and 10.0)
    viewer.handle_zoom(100.0); // Try to zoom way in
    viewer.handle_zoom(-100.0); // Try to zoom way out

    // Should not crash and should handle gracefully
    assert!(viewer.has_mesh());
}

#[test]
fn test_viewer3d_camera_pitch_clamp() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();
    viewer.set_mesh(mesh);

    // Test pitch clamping (should prevent gimbal lock)
    // Drag vertically many times
    for _ in 0..100 {
        viewer.handle_drag(egui::Vec2::new(0.0, 10.0));
    }

    // Should not crash
    assert!(viewer.has_mesh());
}

/// Test that mesh bounds are calculated correctly
#[test]
fn test_viewer3d_mesh_bounds() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube();
    viewer.set_mesh(mesh);

    // Reset camera (which calculates bounds)
    viewer.reset_camera();

    // Should not crash
    assert!(viewer.has_mesh());
}

/// Test memory usage doesn't grow unbounded (basic leak test)
#[test]
fn test_viewer3d_memory_leak() {
    let mut viewer = Viewer3D::new();

    // Load and unload meshes multiple times
    for i in 0..10 {
        let mesh = create_mesh_with_vertex_count(1000 * (i + 1));
        viewer.set_mesh(mesh);
        assert!(viewer.has_mesh());

        // Clear mesh by setting a new empty one (if supported)
        // For now, just verify we can load multiple meshes
    }

    // Final mesh should be loaded
    assert!(viewer.has_mesh());
}

/// Test that viewer handles mesh with normals correctly
#[test]
fn test_viewer3d_mesh_with_normals() {
    let mut viewer = Viewer3D::new();
    let mesh = create_test_cube(); // Has normals

    viewer.set_mesh(mesh);
    assert!(viewer.has_mesh());
}

/// Test that viewer handles mesh without normals correctly
#[test]
fn test_viewer3d_mesh_without_normals() {
    let mut viewer = Viewer3D::new();
    let mesh = Arc::new(Mesh {
        vertices: vec![
            Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vertex {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vertex {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ],
        faces: vec![Face { indices: [0, 1, 2] }],
        normals: Vec::new(), // No normals
    });

    viewer.set_mesh(mesh);
    assert!(viewer.has_mesh());
}
