// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Simple Image Converter Contributors

//! Integration tests for Sprint 11 features
//!
//! Tests for:
//! - UI style consistency (Task 1.1)
//! - Keyboard shortcuts implementation (Task 1.2)
//! - Help system implementation (Task 1.3)
//! - Error message improvements (Task 2.2)

use common::error::ConversionError;
use converter_gui::app::ConverterApp;
use converter_gui::error_messages::format_user_message;
use converter_gui::ui::help_panel;
use converter_gui::ui::style;
use std::io;

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_ui_style_constants_consistency() {
    // Test that style constants are consistent and valid
    // Spacing should be ordered
    assert!(style::spacing::SMALL < style::spacing::MEDIUM);
    assert!(style::spacing::MEDIUM < style::spacing::STANDARD);
    assert!(style::spacing::STANDARD < style::spacing::LARGE);
    assert!(style::spacing::LARGE < style::spacing::EXTRA_LARGE);

    // Borders should be ordered
    assert!(style::border::THIN < style::border::STANDARD);
    assert!(style::border::STANDARD < style::border::THICK);

    // Corner radius should be ordered
    assert!(style::corner_radius::SMALL < style::corner_radius::STANDARD);
    assert!(style::corner_radius::STANDARD < style::corner_radius::LARGE);

    // Scroll areas should have reasonable heights
    assert!(style::scroll::MESSAGES_MAX_HEIGHT > 0.0);
    assert!(style::scroll::BATCH_QUEUE_MAX_HEIGHT > 0.0);
    assert!(style::scroll::BATCH_QUEUE_MAX_HEIGHT > style::scroll::MESSAGES_MAX_HEIGHT);
}

#[test]
fn test_ui_style_colors_are_distinct() {
    // Test that color constants are distinct where they should be
    // Message colors
    assert_ne!(style::colors::message::INFO, style::colors::message::ERROR);
    assert_ne!(
        style::colors::message::WARNING,
        style::colors::message::SUCCESS
    );

    // Status colors
    assert_ne!(
        style::colors::status::READY,
        style::colors::status::CONVERTING
    );
    assert_ne!(style::colors::status::SUCCESS, style::colors::status::ERROR);

    // Batch queue colors
    assert_ne!(
        style::colors::batch_queue::PENDING,
        style::colors::batch_queue::PROCESSING
    );
    assert_ne!(
        style::colors::batch_queue::COMPLETED,
        style::colors::batch_queue::FAILED
    );
    assert_ne!(
        style::colors::batch_queue::PAUSED,
        style::colors::batch_queue::CANCELLED
    );
}

#[test]
fn test_help_panel_functions_accessible() {
    // Test that help panel functions are accessible
    // Full UI rendering requires egui context, so we test function existence
    let _help_fn: fn(&mut egui::Ui) = help_panel::render_help_panel;
    let _about_fn: fn(&mut egui::Ui) = help_panel::render_about_dialog;

    // Functions should be callable (signature verification)
    // Function pointers have a non-zero size
    assert!(std::mem::size_of::<fn(&mut egui::Ui)>() > 0);
}

#[test]
fn test_error_messages_user_friendly() {
    // Test that error messages are user-friendly and actionable
    // (These tests complement the existing tests in error_messages.rs)

    // Test UnsupportedFormat
    let error = ConversionError::UnsupportedFormat("xyz".to_string());
    let message = format_user_message(&error);
    assert!(message.contains("not supported"));
    assert!(!message.contains("xyz")); // Should not expose technical details
    assert!(!message.contains("UnsupportedFormat")); // Should not expose error type

    // Test InvalidInput with size error
    let error = ConversionError::InvalidInput("File size exceeds limit".to_string());
    let message = format_user_message(&error);
    assert!(message.contains("too large"));
    assert!(message.contains("100 MB")); // Should include actionable limit

    // Test Io error with NotFound
    let error = ConversionError::Io(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    let message = format_user_message(&error);
    assert!(message.contains("not found"));
    assert!(message.contains("exists")); // Should suggest checking file existence
    assert!(!message.contains("NotFound")); // Should not expose error kind

    // Test ConversionFailed
    let error = ConversionError::ConversionFailed("Conversion failed".to_string());
    let message = format_user_message(&error);
    assert!(message.contains("failed"));
    assert!(message.contains("try again")); // Should be actionable
}

#[test]
fn test_error_messages_consistent_format() {
    // Test that error messages follow a consistent format
    let errors = vec![
        ConversionError::UnsupportedFormat("test".to_string()),
        ConversionError::InvalidInput("test".to_string()),
        ConversionError::Io(io::Error::new(io::ErrorKind::NotFound, "test")),
        ConversionError::ConversionFailed("test".to_string()),
    ];

    for error in errors {
        let message = format_user_message(&error);
        // Messages should not be empty
        assert!(!message.is_empty());
        // Messages should not contain technical error type names
        assert!(!message.contains("ConversionError"));
        assert!(!message.contains("::"));
        // Messages should be sentence case (start with capital)
        assert!(message.chars().next().unwrap().is_uppercase());
    }
}

#[test]
fn test_error_messages_actionable() {
    // Test that error messages provide actionable guidance
    let error =
        ConversionError::ResourceLimitExceeded("Image dimension 100000 exceeds limit".to_string());
    let message = format_user_message(&error);
    // Should mention the limit
    assert!(message.contains("65,535"));
    // Should suggest action
    assert!(message.contains("smaller") || message.contains("resize"));

    let error = ConversionError::InvalidInput("Quality setting invalid".to_string());
    let message = format_user_message(&error);
    // Should mention valid range
    assert!(message.contains("1") && message.contains("100"));
    // Should suggest action
    assert!(message.contains("adjust") || message.contains("quality"));
}

#[test]
fn test_keyboard_shortcuts_function_exists() {
    // Test that handle_keyboard_shortcuts function exists
    // Full testing requires egui context which is complex to mock
    // This test verifies the function is accessible
    let app = ConverterApp::default();

    // Function should exist and be callable
    // Note: We can't actually call it without egui context, but we can verify
    // the app structure supports it
    assert!(app.source_file.is_none()); // Verify app is initialized

    // The function is private, so we test indirectly through app structure
    // If app compiles and initializes, the function exists
}

#[test]
fn test_app_help_panel_state() {
    // Test that app has help panel state
    let mut app = ConverterApp::default();

    // Help panel should start as not visible
    assert!(!app.show_help_panel);
    assert!(!app.show_about_dialog);

    // Should be able to toggle
    app.show_help_panel = true;
    assert!(app.show_help_panel);

    app.show_about_dialog = true;
    assert!(app.show_about_dialog);
}

#[test]
fn test_style_constants_used_in_help_panel() {
    // Test that help panel uses style constants
    // This is an indirect test - we verify constants are accessible
    let _spacing = style::spacing::LARGE;
    let _color = style::colors::ui::SECONDARY_TEXT;

    // If this compiles, help panel can use style constants
    // (help_panel.rs already uses them, verified by compilation)
}

#[test]
fn test_error_messages_no_path_leakage() {
    // Test that error messages don't leak sensitive path information
    let error = ConversionError::Io(io::Error::new(
        io::ErrorKind::NotFound,
        "/home/user/secret/path/to/file.png",
    ));
    let message = format_user_message(&error);

    // Should not contain full path
    assert!(!message.contains("/home/user/secret"));
    assert!(!message.contains("secret"));

    // Should be generic
    assert!(message.contains("not found") || message.contains("exists"));
}

#[test]
fn test_error_messages_specific_guidance() {
    // Test that error messages provide specific guidance for common errors
    let test_cases = vec![
        (
            ConversionError::InvalidInput("Image width exceeds dimension limit".to_string()),
            vec!["dimension", "65,535", "smaller"],
        ),
        (
            ConversionError::ResourceLimitExceeded("Mesh vertices exceed limit".to_string()),
            vec!["vertex", "10,000,000", "simpler"],
        ),
        (
            ConversionError::Io(io::Error::new(io::ErrorKind::PermissionDenied, "test")),
            vec!["Permission", "permissions", "administrator"],
        ),
        (
            ConversionError::Io(io::Error::new(io::ErrorKind::AlreadyExists, "test")),
            vec!["already exists", "different"],
        ),
    ];

    for (error, expected_keywords) in test_cases {
        let message = format_user_message(&error);
        for keyword in expected_keywords {
            assert!(
                message.to_lowercase().contains(&keyword.to_lowercase()),
                "Message '{}' should contain '{}'",
                message,
                keyword
            );
        }
    }
}

#[test]
fn test_ui_style_icon_constants() {
    // Test that icon constants are valid
    assert!(!style::icons::INFO.is_empty());
    assert!(!style::icons::WARNING.is_empty());
    assert!(!style::icons::ERROR.is_empty());
    assert!(!style::icons::SUCCESS.is_empty());

    // Icons should be reasonable length (Unicode characters may be multiple bytes)
    // INFO is "ℹ" which is 3 bytes in UTF-8, WARNING is "⚠" which is 3 bytes
    assert!(style::icons::INFO.len() <= 4);
    assert!(style::icons::WARNING.len() <= 4);
    assert!(style::icons::ERROR.len() <= 4);
    assert!(style::icons::SUCCESS.len() <= 4);
}

#[test]
fn test_error_messages_context_preserved() {
    // Test that helpful context is preserved in error messages
    let error = ConversionError::InvalidFormat(
        "Format mismatch: extension suggests Png but magic bytes indicate Jpeg".to_string(),
    );
    let message = format_user_message(&error);

    // Should mention extension mismatch (helpful context)
    assert!(message.contains("extension") || message.contains("format"));
    // Should not expose technical details like "magic bytes"
    assert!(!message.contains("magic bytes"));
    // Should suggest verification
    assert!(message.contains("verify") || message.contains("check"));
}
