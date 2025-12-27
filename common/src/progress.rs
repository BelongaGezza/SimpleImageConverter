// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

/// Trait for reporting conversion progress
pub trait ProgressReporter {
    /// Report progress as a percentage (0.0 to 1.0)
    fn report(&self, progress: f32);

    /// Report a status message
    fn status(&self, message: &str);
}

/// No-op progress reporter for when progress reporting is not needed
pub struct NoOpProgressReporter;

impl ProgressReporter for NoOpProgressReporter {
    fn report(&self, _progress: f32) {}
    fn status(&self, _message: &str) {}
}
