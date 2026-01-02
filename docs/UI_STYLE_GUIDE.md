# UI Style Guide
## Simple Image Converter - GUI Design System

**Version:** 0.3.0  
**Last Updated:** December 30, 2025  
**Status:** Sprint 11 - Task 1.1 Complete

---

## Overview

This document defines the UI style guide and design system for Simple Image Converter's graphical interface. The style system ensures consistent visual appearance, spacing, colors, and interaction patterns across all UI components.

All UI components should use the centralized style constants defined in `converter-gui/src/ui/style.rs` rather than hardcoding values.

---

## Design Principles

1. **Consistency First:** All UI elements should follow consistent styling patterns
2. **Accessibility:** Colors meet WCAG AA contrast requirements where applicable
3. **Clarity:** Clear visual hierarchy guides user attention
4. **Efficiency:** Consistent patterns reduce cognitive load

---

## Spacing System

All spacing values are centralized in `style::spacing`:

| Constant | Value | Usage |
|----------|-------|-------|
| `SMALL` | 2px | Tight spacing between related elements (e.g., radio buttons) |
| `MEDIUM` | 5px | Spacing within groups and collapsible sections |
| `STANDARD` | 10px | Spacing between sections and major UI elements |
| `LARGE` | 20px | Spacing between major sections |
| `EXTRA_LARGE` | 30px | Spacing between major panels (reserved for future use) |

**Usage Example:**
```rust
use crate::ui::style;

ui.add_space(style::spacing::STANDARD);  // ✅ Correct
ui.add_space(10.0);                      // ❌ Don't hardcode
```

---

## Color System

Colors are organized by purpose in `style::colors`:

### Message Colors

Used for displaying messages to users:

- **`message::INFO`** - Blue (RGB: 0, 100, 255) - Informational messages
- **`message::WARNING`** - Yellow/Orange (RGB: 255, 200, 0) - Warning messages
- **`message::ERROR`** - Red (RGB: 255, 0, 0) - Error messages
- **`message::SUCCESS`** - Green (RGB: 0, 200, 0) - Success messages

### Status Colors

Used for status indicators:

- **`status::READY`** - Gray - Ready/Idle status
- **`status::CONVERTING`** - Blue (RGB: 0, 100, 255) - Active conversion
- **`status::SUCCESS`** - Green (RGB: 0, 200, 0) - Successful operation
- **`status::ERROR`** - Red (RGB: 255, 0, 0) - Failed operation

### Batch Queue Status Colors

Used for batch queue item statuses:

- **`batch_queue::PENDING`** - Gray - Item pending
- **`batch_queue::PROCESSING`** - Blue (RGB: 100, 150, 255) - Item processing
- **`batch_queue::COMPLETED`** - Green (RGB: 50, 200, 50) - Item completed
- **`batch_queue::FAILED`** - Red (RGB: 200, 50, 50) - Item failed
- **`batch_queue::PAUSED`** - Yellow/Orange (RGB: 200, 150, 50) - Processing paused
- **`batch_queue::CANCELLED`** - Gray - Item cancelled

### UI Element Colors

Used for general UI elements:

- **`ui::DROP_ZONE_SELECTED_BG`** - Light green background (RGB: 240, 255, 240)
- **`ui::DROP_ZONE_SELECTED_BORDER`** - Green border (RGB: 0, 200, 0)
- **`ui::DROP_ZONE_DRAG_BG`** - Light blue background (RGB: 240, 248, 255)
- **`ui::DROP_ZONE_DRAG_BORDER`** - Blue border (RGB: 0, 100, 255)
- **`ui::DROP_ZONE_EMPTY_BG`** - Light gray background (RGB: 245, 245, 245)
- **`ui::DROP_ZONE_EMPTY_BORDER`** - Gray border (RGB: 200, 200, 200)
- **`ui::SECONDARY_TEXT`** - Gray - Secondary/helper text
- **`ui::PLACEHOLDER_TEXT`** - Light gray (RGB: 180, 180, 180) - Placeholder text

### Auto-Save Status Colors

Used for settings auto-save indicators:

- **`auto_save::SAVING`** - Blue (RGB: 100, 150, 255) - Currently saving
- **`auto_save::SAVED`** - Green (RGB: 50, 200, 50) - Successfully saved
- **`auto_save::ERROR`** - Red (RGB: 200, 50, 50) - Save failed

**Usage Example:**
```rust
use crate::ui::style;

ui.label(
    RichText::new("Error message")
        .color(style::colors::message::ERROR)  // ✅ Correct
);

ui.label(
    RichText::new("Error message")
        .color(Color32::from_rgb(255, 0, 0))  // ❌ Don't hardcode
);
```

---

## Border System

Border widths are defined in `style::border`:

- **`THIN`** - 1px - Subtle borders (e.g., empty drop zone)
- **`STANDARD`** - 2px - Standard borders (e.g., selected drop zone)
- **`THICK`** - 3px - Emphasis borders (reserved for future use)

**Usage Example:**
```rust
use crate::ui::style;

ui.painter().rect_stroke(
    rect,
    style::corner_radius::STANDARD,
    Stroke::new(style::border::STANDARD, border_color)
);
```

---

## Corner Radius System

Corner radius values are defined in `style::corner_radius`:

- **`SMALL`** - 2px (reserved for future use)
- **`STANDARD`** - 4px - Standard rounded corners (e.g., drop zone)
- **`LARGE`** - 8px (reserved for future use)

**Usage Example:**
```rust
use crate::ui::style;

ui.painter().rect_filled(
    rect,
    style::corner_radius::STANDARD,
    bg_color
);
```

---

## Scroll Area Heights

Maximum heights for scrollable areas:

- **`scroll::MESSAGES_MAX_HEIGHT`** - 150px - Messages area
- **`scroll::BATCH_QUEUE_MAX_HEIGHT`** - 400px - Batch queue list

**Usage Example:**
```rust
use crate::ui::style;

ScrollArea::vertical()
    .max_height(style::scroll::MESSAGES_MAX_HEIGHT)
    .show(ui, |ui| {
        // Content
    });
```

---

## Icons

Icon constants are defined in `style::icons`:

- **`INFO`** - "ℹ" - Informational icon
- **`WARNING`** - "⚠" - Warning icon
- **`ERROR`** - "✗" - Error icon
- **`SUCCESS`** - "✓" - Success icon

**Usage Example:**
```rust
use crate::ui::style;

ui.label(
    RichText::new(style::icons::SUCCESS)
        .size(16.0)
        .color(style::colors::message::SUCCESS)
);
```

---

## Component Patterns

### Buttons

Buttons use egui's default styling. No custom button styles are currently defined, but buttons should:
- Have consistent hover tooltips
- Use clear, action-oriented labels
- Be appropriately enabled/disabled based on context

### Labels

- **Headings:** Use `ui.heading()` for section titles
- **Strong text:** Use `RichText::new().strong()` for emphasis
- **Secondary text:** Use `RichText::new().small().color(style::colors::ui::SECONDARY_TEXT)`
- **Placeholder text:** Use `RichText::new().italics().color(style::colors::ui::PLACEHOLDER_TEXT)`

### Groups

Use `ui.group()` for visually grouped elements:
- Format selector
- Options panel
- History entries
- Queue items

### Collapsible Sections

Use `ui.collapsing()` for expandable/collapsible sections:
- Advanced options
- Settings categories
- Help sections

---

## Visual Hierarchy

1. **Headings** - Section titles (largest)
2. **Strong labels** - Important information
3. **Regular labels** - Standard text
4. **Small labels** - Secondary information (timestamps, metadata)

---

## Accessibility Considerations

### Color Contrast

All defined colors meet WCAG AA contrast requirements when used with appropriate backgrounds:
- Text colors on white/light backgrounds provide sufficient contrast
- Status colors are bright enough to be distinguishable
- Error/warning colors stand out appropriately

### Text Sizing

- Regular text uses default egui sizing (readable)
- Small text uses `.small()` modifier for secondary information
- Headings use `ui.heading()` which provides appropriate sizing

### Keyboard Accessibility

- All interactive elements are keyboard accessible (egui default)
- Tab navigation follows logical order
- Focus indicators are visible

---

## Cross-Platform Consistency

The style system works consistently across:
- **Windows 11** - Native window decorations, High DPI scaling
- **macOS 26** - Retina display support, system appearance
- **Ubuntu LTS 24.04+** - GTK-compatible styling, Wayland/X11 support

egui handles platform-specific rendering automatically, while our style constants ensure visual consistency.

---

## Implementation Checklist

When creating or updating UI components:

- [ ] Use `style::spacing::*` constants instead of hardcoded spacing values
- [ ] Use `style::colors::*` constants instead of hardcoded colors
- [ ] Use `style::border::*` for border widths
- [ ] Use `style::corner_radius::*` for rounded corners
- [ ] Use `style::scroll::*` for scroll area heights
- [ ] Use `style::icons::*` for status icons
- [ ] Follow consistent spacing patterns (SMALL for tight, STANDARD for sections)
- [ ] Use appropriate color categories (message, status, ui)
- [ ] Add helpful tooltips to interactive elements
- [ ] Ensure proper visual hierarchy

---

## Examples

### Standard Section Layout

```rust
use crate::ui::style;

ui.group(|ui| {
    ui.heading("Section Title");
    ui.add_space(style::spacing::MEDIUM);
    
    ui.label("Label text");
    ui.add_space(style::spacing::STANDARD);
    
    // More content...
});
```

### Status Message Display

```rust
use crate::ui::style;

let (icon, color) = if success {
    (style::icons::SUCCESS, style::colors::message::SUCCESS)
} else {
    (style::icons::ERROR, style::colors::message::ERROR)
};

ui.horizontal(|ui| {
    ui.label(RichText::new(icon).size(16.0).color(color));
    ui.label(RichText::new(&message_text).color(color));
});
```

### Empty State Display

```rust
use crate::ui::style;

ui.label(
    RichText::new("No items")
        .italics()
        .color(style::colors::ui::PLACEHOLDER_TEXT)
        .small()
);
```

---

## Future Enhancements

Potential additions to the style system:

- Button style variants (primary, secondary, danger)
- Typography scale (font sizes, weights)
- Animation durations
- Shadow/elevation system
- Dark mode color variants

---

## Maintenance

**Style Constants Location:** `converter-gui/src/ui/style.rs`

**Last Updated:** December 30, 2025 (Sprint 11 - Task 1.1)

**Contributing:** When adding new style constants:
1. Add constants to appropriate module in `style.rs`
2. Document the constant with a doc comment
3. Update this guide if the constant represents a new pattern
4. Ensure all existing components are updated to use new constants

---

**Document Version:** 1.0  
**Created:** December 30, 2025  
**Status:** Complete - Style system established and documented

