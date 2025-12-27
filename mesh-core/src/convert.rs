// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use common::error::Result;

/// Main mesh converter orchestrator
pub struct MeshConverter;

impl MeshConverter {
    /// Create a new mesh converter
    pub fn new() -> Self {
        Self
    }

    /// Convert mesh from one format to another
    pub fn convert(
        &self,
        input_data: &[u8],
        reader: &dyn MeshReader,
        writer: &dyn MeshWriter,
    ) -> Result<Vec<u8>> {
        let mesh = reader.read(input_data)?;
        writer.write(&mesh)
    }
}

impl Default for MeshConverter {
    fn default() -> Self {
        Self::new()
    }
}
