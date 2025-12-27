// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

use clap::Parser;
use common::error::Result;

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
}

fn main() -> Result<()> {
    let args = Args::parse();

    // TODO: Implement conversion logic in Sprint 3
    println!(
        "mesh-convert: Converting {} to {} format",
        args.input, args.format
    );

    if let Some(output) = args.output {
        println!("Output: {}", output);
    }

    if let Some(transform) = args.transform {
        println!("Transform: {}", transform);
    }

    if args.recalculate_normals {
        println!("Recalculating normals");
    }

    if args.validate {
        println!("Validating mesh");
    }

    Ok(())
}
