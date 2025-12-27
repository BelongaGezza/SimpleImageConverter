# Examples
## Simple Image Converter

This directory contains example code demonstrating how to use the Simple Image Converter libraries.

---

## Structure

```
examples/
├── README.md
├── image_conversion.rs      # Basic image conversion example
├── mesh_conversion.rs        # Basic mesh conversion example
└── batch_processing.rs       # Batch processing example (future)
```

---

## Running Examples

```bash
# Run an example
cargo run --example image_conversion

# Build all examples
cargo build --examples
```

---

## Examples

### Image Conversion

Basic example of converting between image formats:

```rust
use img_core::ImageConverter;
use img_core::QualitySettings;

// Load image
let input_data = std::fs::read("input.png")?;

// Convert
let converter = ImageConverter::new();
let quality = QualitySettings::new(90);
let output_data = converter.convert(&input_data, reader, writer, &quality)?;

// Save
std::fs::write("output.jpg", output_data)?;
```

### Mesh Conversion

Basic example of converting between mesh formats:

```rust
use mesh_core::MeshConverter;

// Load mesh
let input_data = std::fs::read("input.stl")?;

// Convert
let converter = MeshConverter::new();
let output_data = converter.convert(&input_data, reader, writer)?;

// Save
std::fs::write("output.obj", output_data)?;
```

---

## Note

Examples will be added as formats are implemented in subsequent sprints.

---

_Last Updated: December 26, 2025_

