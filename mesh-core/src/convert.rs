// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use crate::formats::traits::{MeshReader, MeshWriter};
use crate::mesh::{transform_coordinates, recalculate_normals, validate_mesh, CoordinateSystem};
use common::error::Result;
use common::progress::{NoOpProgressReporter, ProgressReporter};

/// Options for mesh conversion
#[derive(Debug, Clone, Default)]
pub struct ConversionOptions {
    /// Coordinate system transform (from, to)
    pub transform: Option<(CoordinateSystem, CoordinateSystem)>,
    /// Whether to recalculate normals
    pub recalculate_normals: bool,
    /// Whether to validate the mesh
    pub validate: bool,
}

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
        self.convert_with_options(input_data, reader, writer, &ConversionOptions::default())
    }

    /// Convert mesh with options
    ///
    /// Same as `convert()` but accepts conversion options for transforms, validation, etc.
    ///
    /// # Arguments
    ///
    /// * `input_data` - The raw mesh data to convert
    /// * `reader` - Format-specific reader for the input format
    /// * `writer` - Format-specific writer for the output format
    /// * `options` - Conversion options (transforms, validation, etc.)
    ///
    /// # Returns
    ///
    /// The converted mesh data as a byte vector, or an error if conversion fails.
    pub fn convert_with_options(
        &self,
        input_data: &[u8],
        reader: &dyn MeshReader,
        writer: &dyn MeshWriter,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>> {
        self.convert_with_options_and_progress(
            input_data,
            reader,
            writer,
            options,
            &NoOpProgressReporter,
        )
    }

    /// Convert mesh with options and progress reporting
    ///
    /// Full-featured conversion with all options and progress reporting.
    ///
    /// # Arguments
    ///
    /// * `input_data` - The raw mesh data to convert
    /// * `reader` - Format-specific reader for the input format
    /// * `writer` - Format-specific writer for the output format
    /// * `options` - Conversion options (transforms, validation, etc.)
    /// * `progress` - Progress reporter for status updates
    ///
    /// # Returns
    ///
    /// The converted mesh data as a byte vector, or an error if conversion fails.
    pub fn convert_with_options_and_progress(
        &self,
        input_data: &[u8],
        reader: &dyn MeshReader,
        writer: &dyn MeshWriter,
        options: &ConversionOptions,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<u8>> {
        progress.status("Reading input mesh...");
        progress.report(0.1);
        let mut mesh = reader.read(input_data)?;
        progress.report(0.3);

        // Apply coordinate transform if requested
        if let Some((from, to)) = options.transform {
            progress.status("Transforming coordinate system...");
            mesh = transform_coordinates(mesh, from, to)?;
            progress.report(0.5);
        } else {
            progress.report(0.5);
        }

        // Recalculate normals if requested
        if options.recalculate_normals {
            progress.status("Recalculating normals...");
            mesh = recalculate_normals(mesh)?;
            progress.report(0.7);
        } else {
            progress.report(0.7);
        }

        // Validate mesh if requested
        if options.validate {
            progress.status("Validating mesh...");
            validate_mesh(&mesh)?;
            progress.report(0.9);
        } else {
            progress.report(0.9);
        }

        progress.status("Writing output mesh...");
        let output = writer.write(&mesh)?;
        progress.report(1.0);
        progress.status("Conversion complete");
        Ok(output)
    }

    /// Convert mesh with progress reporting (legacy method)
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
        self.convert_with_options_and_progress(
            input_data,
            reader,
            writer,
            &ConversionOptions::default(),
            progress,
        )
    }
}

impl Default for MeshConverter {
    fn default() -> Self {
        Self::new()
    }
}
