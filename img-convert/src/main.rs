// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use clap::Parser;
use common::error::{ConversionError, Result};
use common::io::{read_file_bytes_checked, write_file_bytes};
use common::limits::ResourceLimits;
use img_core::{FormatRegistry, ImageConverter, QualitySettings};
use std::path::Path;

#[derive(Parser)]
#[command(name = "img-convert")]
#[command(about = "Convert between 2D image formats", long_about = None)]
struct Args {
    /// Input file path
    input: String,

    /// Output format
    format: String,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// Quality setting (1-100)
    #[arg(short, long, default_value_t = 90)]
    quality: u8,

    /// Maximum file size in MB (default: 100)
    #[arg(long, default_value_t = 100)]
    max_file_size_mb: usize,

    /// Maximum image dimension in pixels (default: 65535)
    #[arg(long, default_value_t = 65535)]
    max_dimension: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate quality parameter (must be 1-100)
    if args.quality == 0 || args.quality > 100 {
        return Err(ConversionError::InvalidInput(
            "Quality must be between 1 and 100".to_string(),
        ));
    }

    // Build resource limits from CLI args
    let limits = ResourceLimits::builder()
        .max_file_size_mb(args.max_file_size_mb)
        .max_image_dimension(args.max_dimension)
        .build();

    // Validate input file using common validation
    let input_path = Path::new(&args.input);
    common::validation::validate_file_path(input_path)?;

    // Security: Two-stage format detection (extension + magic bytes)
    let input_format = FormatRegistry::detect_two_stage(input_path, &input_data)?;

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

    // Security: Verify format matches file content (two-stage detection)
    if let Err(e) = FormatRegistry::verify_format(&input_data, input_format) {
        common::security::log_security_error(&e, Some(input_path));
        return Err(e);
    }

    // Get format handlers
    let reader = FormatRegistry::get_reader(input_format)?;
    let writer = FormatRegistry::get_writer(output_format)?;

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(args.quality);
    let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref(), &quality)?;

    // Write output file
    write_file_bytes(&output_path, &output_data)?;

    // Security: Validate output file by verifying it can be read back
    // This ensures the conversion produced a valid file
    let output_data_read = read_file_bytes_checked(&output_path, &limits)?;
    if let Some(detected_format) = FormatRegistry::detect_from_bytes(&output_data_read) {
        if detected_format != output_format {
            eprintln!(
                "Warning: Output file format verification failed (expected {:?}, detected {:?})",
                output_format, detected_format
            );
        }
    }

    println!(
        "Successfully converted {} to {}",
        args.input,
        output_path.display()
    );

    Ok(())
}
