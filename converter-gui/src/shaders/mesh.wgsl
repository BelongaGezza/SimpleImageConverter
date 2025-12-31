// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! WGSL shaders for 3D mesh rendering
//!
//! This file contains vertex and fragment shaders for rendering 3D meshes
//! in the preview panel using wgpu.

// Uniform buffer for camera transformation
struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Vertex input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

// Vertex output
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

// Vertex shader
@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform position to clip space
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    
    // Pass through world position and normal
    out.world_position = vertex.position;
    out.normal = vertex.normal;
    
    return out;
}

// Fragment shader for solid rendering
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.5));
    let normal = normalize(in.normal);
    
    // Calculate diffuse lighting
    let ndotl = max(dot(normal, light_dir), 0.0);
    let diffuse = vec3<f32>(0.7, 0.7, 0.8) * ndotl;
    
    // Add ambient lighting
    let ambient = vec3<f32>(0.3, 0.3, 0.4);
    
    // Final color
    let color = diffuse + ambient;
    
    return vec4<f32>(color, 1.0);
}

// Fragment shader for wireframe rendering
@fragment
fn fs_wireframe(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple wireframe color (white/gray)
    return vec4<f32>(0.8, 0.8, 0.8, 1.0);
}

