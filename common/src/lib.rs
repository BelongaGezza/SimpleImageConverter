// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

pub mod error;
pub mod io;
pub mod limits;
pub mod progress;
pub mod security;
pub mod validation;

pub use error::{ConversionError, Result};
pub use limits::ResourceLimits;
pub use security::{log_security_error, SecurityEvent, SecurityEventType};
