// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use clap::Parser;
use common::error::Result;
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;
use mesh_core::{
    parse_coordinate_system, ConversionOptions, CoordinateSystem, FormatRegistry, MeshConverter,
};
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

    // Detect input format (two-stage: extension + signature where possible)
    let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;

    // Get format handlers with resource limits
    let reader = FormatRegistry::get_reader_with_limits(input_format, limits.clone())?;
    let writer = FormatRegistry::get_writer(output_format)?;

    // Build conversion options
    let mut conversion_options = ConversionOptions::default();

    // Parse transform option
    if let Some(transform_str) = args.transform {
        // Parse transform string (e.g., "z-up:y-up" or just "y-up" for auto-detect)
        // Note: When only one coordinate system is specified (e.g., "y-up"), we assume
        // the input is Z-up (common for CAD/STL files) and transform to the specified
        // coordinate system. For explicit transforms, use "from:to" format.
        let transform = if transform_str.contains(':') {
            let parts: Vec<&str> = transform_str.split(':').collect();
            if parts.len() != 2 {
                return Err(common::error::ConversionError::InvalidInput(format!(
                    "Invalid transform format: '{}'. Use 'from:to' (e.g., 'z-up:y-up')",
                    transform_str
                )));
            }
            (
                parse_coordinate_system(parts[0])?,
                parse_coordinate_system(parts[1])?,
            )
        } else {
            // Auto-detect: assume Z-up input (common for CAD/STL), transform to specified
            let to = parse_coordinate_system(&transform_str)?;
            (CoordinateSystem::ZUp, to)
        };
        conversion_options.transform = Some(transform);
    }

    // Set recalculation and validation flags
    conversion_options.recalculate_normals = args.recalculate_normals;
    conversion_options.validate = args.validate;

    // Convert with options
    let converter = MeshConverter::new();
    let output_data = converter.convert_with_options(
        &input_data,
        reader.as_ref(),
        writer.as_ref(),
        &conversion_options,
    )?;

    // Write output file
    write_file_bytes(&output_path, &output_data)?;

    // Security: Validate output file by verifying it can be read back
    // This ensures the conversion produced a valid file
    let output_data_read = read_file_bytes_checked(&output_path, &limits)?;
    // Try to read the output file back to verify it's valid
    let output_reader = FormatRegistry::get_reader_with_limits(output_format, limits.clone())?;
    if output_reader.read(&output_data_read).is_err() {
        eprintln!("Warning: Output file validation failed - file may be corrupted");
    }

    println!(
        "Successfully converted {} to {}",
        args.input,
        output_path.display()
    );

    Ok(())
}
