#!/bin/bash
# Linux GUI Packaging Script
# Creates a portable TAR.GZ archive for Linux GUI distribution

# SECURITY: Exit on error, undefined variables, and pipe failures
set -euo pipefail

# Extract version from Git tag or Cargo.toml if not provided
if [ -z "$1" ]; then
    # Try to get version from Git tag
    VERSION=$(git describe --tags --exact-match 2>/dev/null | sed 's/^v//' || echo "")
    if [ -z "$VERSION" ]; then
        # Fall back to Cargo.toml
        VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d'"' -f2)
    fi
    if [ -z "$VERSION" ]; then
        VERSION="0.2.2"
    fi
    echo "Using version: $VERSION" >&2
else
    VERSION="$1"
fi

# SECURITY: Validate version format to prevent injection attacks
# Allow semantic versioning: X.Y.Z or X.Y.Z-pre (e.g., 0.2.2, 0.2.2-alpha1)
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$'; then
    echo "Error: Invalid version format: '$VERSION'. Expected format: X.Y.Z or X.Y.Z-pre" >&2
    exit 1
fi

# SECURITY: Sanitize version for path usage (remove any remaining special characters)
VERSION_SANITIZED=$(echo "$VERSION" | tr -cd '0-9A-Za-z.-')
if [ "$VERSION_SANITIZED" != "$VERSION" ]; then
    echo "Warning: Version sanitized from '$VERSION' to '$VERSION_SANITIZED' for path safety" >&2
    VERSION="$VERSION_SANITIZED"
fi

TARGET="${2:-x86_64-unknown-linux-gnu}"

# SECURITY: Validate target format to prevent path traversal
if ! echo "$TARGET" | grep -qE '^[a-zA-Z0-9_-]+$'; then
    echo "Error: Invalid target format: '$TARGET'. Only alphanumeric, underscore, and hyphen allowed." >&2
    exit 1
fi

echo "Packaging SimpleImageConverter GUI for Linux..."

# Set paths
RELEASE_DIR="release/linux-x64-gui-v${VERSION}"
TAR_NAME="simpleimageconverter-gui-${VERSION}-linux-x64.tar.gz"
BIN_DIR="target/${TARGET}/release"
NATIVE_BIN_DIR="target/release"

# Clean previous release
rm -rf "$RELEASE_DIR"
rm -f "$TAR_NAME"

# Create release directory
mkdir -p "$RELEASE_DIR"

# Determine binary location (check cross-compiled first, then native)
GUI_BINARY_PATH=""

if [ -f "$BIN_DIR/converter-gui" ]; then
    GUI_BINARY_PATH="$BIN_DIR/converter-gui"
    echo "Using cross-compiled binary from: $BIN_DIR" >&2
elif [ -f "$NATIVE_BIN_DIR/converter-gui" ]; then
    GUI_BINARY_PATH="$NATIVE_BIN_DIR/converter-gui"
    echo "Using native binary from: $NATIVE_BIN_DIR" >&2
else
    echo "Error: GUI binary not found. Expected locations:" >&2
    echo "  - $BIN_DIR/converter-gui" >&2
    echo "  - $NATIVE_BIN_DIR/converter-gui" >&2
    echo "Run 'cargo build --release --bin converter-gui' or 'cargo build --release --target $TARGET --bin converter-gui' first." >&2
    exit 1
fi

# Copy binary
echo "Copying GUI binary..."
cp "$GUI_BINARY_PATH" "$RELEASE_DIR/"

# Make binary executable
chmod +x "$RELEASE_DIR/converter-gui"

# Copy documentation
echo "Copying documentation..."
if [ -f "README.md" ]; then
    cp "README.md" "$RELEASE_DIR/"
fi
if [ -f "LICENSE-APACHE" ]; then
    cp "LICENSE-APACHE" "$RELEASE_DIR/"
fi
if [ -f "LICENSE-MIT" ]; then
    cp "LICENSE-MIT" "$RELEASE_DIR/"
fi

# Create README for Linux GUI users
cat > "$RELEASE_DIR/INSTALL.txt" << 'EOF'
# SimpleImageConverter GUI for Linux

## Installation

1. Extract this archive:
   tar -xzf simpleimageconverter-gui-*.tar.gz

2. Run the application:
   ./converter-gui

3. (Optional) Install system-wide:
   sudo cp converter-gui /usr/local/bin/
   converter-gui

   Or add the directory to your PATH in ~/.bashrc or ~/.zshrc:
   export PATH="$PATH:/path/to/simpleimageconverter"

## Usage

1. Launch `./converter-gui`
2. Drag and drop a file into the drop zone, or click "Browse Files..." to select a file
3. Select the output format from the radio buttons
4. Adjust options (optional):
   - Change output filename
   - Select output location
   - Adjust quality slider (for JPEG/WebP images)
5. Click "Convert" to start the conversion
6. View results in the status bar and messages area

## Features

- Drag-and-drop file support
- Visual format selection
- Quality settings for lossy image formats
- User-friendly error messages
- Progress indicators for long operations
- Thread-safe conversion processing
- Batch processing (v0.2.2)
- Preview functionality (v0.2.2)
- Settings persistence (v0.2.2)
- Conversion history (v0.2.2)

## Supported Formats

**Images:** PNG, JPEG, BMP, GIF, TIFF, WebP (SVG read-only)
**Meshes:** STL, OBJ, PLY, OFF, glTF, DXF (STEP read-only, feature-gated)

## Dependencies

The GUI application requires:
- X11 or Wayland display server
- Standard system libraries (usually pre-installed)

For more information, see the main README.md file.
EOF

# Create TAR.GZ archive
echo "Creating TAR.GZ archive..."
tar -czf "$TAR_NAME" -C release "linux-x64-gui-v${VERSION}"

# Display results
TAR_SIZE=$(du -h "$TAR_NAME" | cut -f1)
echo ""
echo "Package created successfully!"
echo "  File: $TAR_NAME"
echo "  Size: $TAR_SIZE"
echo "  Location: $(pwd)/$TAR_NAME"
echo ""
echo "Next steps:"
echo "  1. Test the package by extracting and running the binary"
echo "  2. Upload to GitHub Releases"
echo "  3. Create DEB package (optional): cargo install cargo-deb && cargo deb"

