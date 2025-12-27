// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use clap::Parser;
use common::error::Result;
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;
use mesh_core::{FormatRegistry, MeshConverter};
use std::path::Path;

#[derive(Parser)]
#[command(name = "mesh-convert")]
#[command(about = "Convert between 3D mesh formats", long_about = None)]
struct Args {
    /// Input file path
    input: String,

    /// Output format
    format: String,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// Transform coordinate system (y-up or z-up)
    #[arg(short, long)]
    transform: Option<String>,

    /// Recalculate normals
    #[arg(long)]
    recalculate_normals: bool,

    /// Validate mesh
    #[arg(long)]
    validate: bool,

    /// Maximum file size in MB (default: 100)
    #[arg(long, default_value_t = 100)]
    max_file_size_mb: usize,

    /// Maximum vertices (default: 10,000,000)
    #[arg(long, default_value_t = 10_000_000)]
    max_vertices: usize,

    /// Maximum faces (default: 10,000,000)
    #[arg(long, default_value_t = 10_000_000)]
    max_faces: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Build resource limits from CLI args
    let limits = ResourceLimits::builder()
        .max_file_size_mb(args.max_file_size_mb)
        .max_vertices(args.max_vertices)
        .max_faces(args.max_faces)
        .build();

    // Validate input file using common validation
    let input_path = Path::new(&args.input);
    common::validation::validate_file_path(input_path)?;

    // Detect input format
    let input_format = FormatRegistry::detect_from_path(input_path)?;

    // Detect output format
    let output_format = FormatRegistry::detect_format(&args.format)?;

    // Determine output path
    let output_path = if let Some(output) = args.output {
        Path::new(&output).to_path_buf()
    } else {
        // Generate output path from input path
        let mut output = input_path.to_path_buf();
        output.set_extension(&args.format);
        output
    };

    // Read input file with size validation
    let input_data = read_file_bytes_checked(input_path, &limits)?;

    // Get format handlers with resource limits
    let reader = FormatRegistry::get_reader_with_limits(input_format, limits.clone())?;
    let writer = FormatRegistry::get_writer(output_format)?;

    // Convert
    let converter = MeshConverter::new();
    let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref())?;

    // Write output file
    write_file_bytes(&output_path, &output_data)?;

    println!(
        "Successfully converted {} to {}",
        args.input,
        output_path.display()
    );

    // Note: Transform, recalculate_normals, and validate options are placeholders
    // for future enhancements (Sprint 5+)
    if let Some(transform) = args.transform {
        eprintln!(
            "Warning: Transform option '{}' not yet implemented",
            transform
        );
    }

    if args.recalculate_normals {
        eprintln!("Warning: Recalculate normals option not yet implemented");
    }

    if args.validate {
        eprintln!("Warning: Validate option not yet implemented");
    }

    Ok(())
}
