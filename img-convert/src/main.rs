// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use clap::Parser;
use common::error::Result;
use common::io::{read_file_bytes, write_file_bytes};
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

    /// Quality setting (0-100)
    #[arg(short, long, default_value_t = 90)]
    quality: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

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

    // Read input file
    let input_data = read_file_bytes(input_path)?;

    // Get format handlers
    let reader = FormatRegistry::get_reader(input_format)?;
    let writer = FormatRegistry::get_writer(output_format)?;

    // Convert
    let converter = ImageConverter::new();
    let quality = QualitySettings::new(args.quality);
    let output_data = converter.convert(&input_data, reader.as_ref(), writer.as_ref(), &quality)?;

    // Write output file
    write_file_bytes(&output_path, &output_data)?;

    println!(
        "Successfully converted {} to {}",
        args.input,
        output_path.display()
    );

    Ok(())
}
