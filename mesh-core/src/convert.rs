// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use common::error::Result;
use common::progress::{NoOpProgressReporter, ProgressReporter};

/// Main mesh converter orchestrator
pub struct MeshConverter;

impl MeshConverter {
    /// Create a new mesh converter
    pub fn new() -> Self {
        Self
    }

    /// Convert mesh from one format to another
    ///
    /// # Arguments
    ///
    /// * `input_data` - The raw mesh data to convert
    /// * `reader` - Format-specific reader for the input format
    /// * `writer` - Format-specific writer for the output format
    ///
    /// # Returns
    ///
    /// The converted mesh data as a byte vector, or an error if conversion fails.
    pub fn convert(
        &self,
        input_data: &[u8],
        reader: &dyn MeshReader,
        writer: &dyn MeshWriter,
    ) -> Result<Vec<u8>> {
        self.convert_with_progress(input_data, reader, writer, &NoOpProgressReporter)
    }

    /// Convert mesh with progress reporting
    ///
    /// Same as `convert()` but accepts a progress reporter for status updates.
    ///
    /// # Arguments
    ///
    /// * `input_data` - The raw mesh data to convert
    /// * `reader` - Format-specific reader for the input format
    /// * `writer` - Format-specific writer for the output format
    /// * `progress` - Progress reporter for status updates
    ///
    /// # Returns
    ///
    /// The converted mesh data as a byte vector, or an error if conversion fails.
    pub fn convert_with_progress(
        &self,
        input_data: &[u8],
        reader: &dyn MeshReader,
        writer: &dyn MeshWriter,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<u8>> {
        progress.status("Reading input mesh...");
        progress.report(0.1);
        let mesh = reader.read(input_data)?;
        progress.report(0.5);
        progress.status("Writing output mesh...");
        let output = writer.write(&mesh)?;
        progress.report(1.0);
        progress.status("Conversion complete");
        Ok(output)
    }
}

impl Default for MeshConverter {
    fn default() -> Self {
        Self::new()
    }
}
