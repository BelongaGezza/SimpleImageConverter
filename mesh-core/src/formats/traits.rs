// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::mesh::Mesh;
use common::error::Result;

/// Trait for reading mesh formats
pub trait MeshReader {
    /// Read a mesh from bytes
    fn read(&self, data: &[u8]) -> Result<Mesh>;
}

/// Trait for writing mesh formats
pub trait MeshWriter {
    /// Write a mesh to bytes
    fn write(&self, mesh: &Mesh) -> Result<Vec<u8>>;
}
