#!/bin/bash
# macOS Packaging Script
# Creates a portable TAR.GZ archive for macOS distribution

set -e

VERSION="${1:-0.2.0}"
TARGET="${2:-x86_64-apple-darwin}"

echo "Packaging SimpleImageConverter for macOS..."

# Set paths
RELEASE_DIR="release/macos-x64-v${VERSION}"
TAR_NAME="simpleimageconverter-${VERSION}-macos-x64.tar.gz"
BIN_DIR="target/${TARGET}/release"

# Clean previous release
rm -rf "$RELEASE_DIR"
rm -f "$TAR_NAME"

# Create release directory
mkdir -p "$RELEASE_DIR"

# Verify binaries exist
if [ ! -f "$BIN_DIR/img-convert" ]; then
    echo "Error: Binary not found: $BIN_DIR/img-convert"
    echo "Run 'cargo build --release --target $TARGET' first."
    exit 1
fi

if [ ! -f "$BIN_DIR/mesh-convert" ]; then
    echo "Error: Binary not found: $BIN_DIR/mesh-convert"
    echo "Run 'cargo build --release --target $TARGET' first."
    exit 1
fi

# Copy binaries
echo "Copying binaries..."
cp "$BIN_DIR/img-convert" "$RELEASE_DIR/"
cp "$BIN_DIR/mesh-convert" "$RELEASE_DIR/"

# Make binaries executable
chmod +x "$RELEASE_DIR/img-convert"
chmod +x "$RELEASE_DIR/mesh-convert"

# Copy documentation
echo "Copying documentation..."
if [ -f "README.md" ]; then
    cp "README.md" "$RELEASE_DIR/"
fi
if [ -f "LICENSE" ]; then
    cp "LICENSE" "$RELEASE_DIR/"
fi

# Create README for macOS users
cat > "$RELEASE_DIR/INSTALL.txt" << 'EOF'
# SimpleImageConverter for macOS

## Installation

1. Extract this archive:
   tar -xzf simpleimageconverter-*.tar.gz

2. (Optional) Move binaries to a location in your PATH:
   sudo cp img-convert mesh-convert /usr/local/bin/

   Or add the directory to your PATH in ~/.zshrc or ~/.bash_profile:
   export PATH="$PATH:/path/to/simpleimageconverter"

3. Run the tools:
   ./img-convert --help
   ./mesh-convert --help

## Usage

# Convert image
./img-convert input.png jpg

# Convert mesh
./mesh-convert model.stl obj

For more information, see the main README.md file.
EOF

# Create TAR.GZ archive
echo "Creating TAR.GZ archive..."
tar -czf "$TAR_NAME" -C release "macos-x64-v${VERSION}"

# Display results
TAR_SIZE=$(du -h "$TAR_NAME" | cut -f1)
echo ""
echo "Package created successfully!"
echo "  File: $TAR_NAME"
echo "  Size: $TAR_SIZE"
echo "  Location: $(pwd)/$TAR_NAME"
echo ""
echo "Next steps:"
echo "  1. Test the package by extracting and running the binaries"
echo "  2. Upload to GitHub Releases"
echo "  3. Update Homebrew Cask (if applicable)"

