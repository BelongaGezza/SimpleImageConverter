// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for the `img-convert` CLI binary.

use std::process::Command;

fn img_convert_bin() -> &'static str {
    env!("CARGO_BIN_EXE_img-convert")
}

#[test]
fn test_img_convert_help() {
    let output = Command::new(img_convert_bin())
        .arg("--help")
        .output()
        .expect("Failed to execute img-convert --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("img-convert"));
    assert!(stdout.contains("Convert between 2D image formats"));
}

#[test]
fn test_img_convert_invalid_quality() {
    // Quality is validated before file I/O, so no input file is required.
    let output = Command::new(img_convert_bin())
        .args(["nonexistent.png", "jpg", "--quality", "101"])
        .output()
        .expect("Failed to execute img-convert with invalid quality");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Quality") || stderr.contains("quality"),
        "expected quality error in stderr, got: {stderr}"
    );
}
