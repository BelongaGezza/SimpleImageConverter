#!/bin/bash
# Linux Packaging Script
# Creates a portable TAR.GZ archive for Linux distribution

set -e

VERSION="${1:-0.2.0}"
TARGET="${2:-x86_64-unknown-linux-gnu}"

echo "Packaging SimpleImageConverter for Linux..."

# Set paths
RELEASE_DIR="release/linux-x64-v${VERSION}"
TAR_NAME="simpleimageconverter-${VERSION}-linux-x64.tar.gz"
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

# Create README for Linux users
cat > "$RELEASE_DIR/INSTALL.txt" << 'EOF'
# SimpleImageConverter for Linux

## Installation

1. Extract this archive:
   tar -xzf simpleimageconverter-*.tar.gz

2. (Optional) Install system-wide:
   sudo cp img-convert mesh-convert /usr/local/bin/

   Or add the directory to your PATH in ~/.bashrc or ~/.zshrc:
   export PATH="$PATH:/path/to/simpleimageconverter"

3. Run the tools:
   ./img-convert --help
   ./mesh-convert --help

## Alternative: DEB Package

For Ubuntu/Debian systems, you can also install the .deb package:
   sudo dpkg -i simpleimageconverter_*.deb
   sudo apt-get install -f  # Install dependencies if needed

## Usage

# Convert image
./img-convert input.png jpg

# Convert mesh
./mesh-convert model.stl obj

For more information, see the main README.md file.
EOF

# Create TAR.GZ archive
echo "Creating TAR.GZ archive..."
tar -czf "$TAR_NAME" -C release "linux-x64-v${VERSION}"

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
echo "  3. Create DEB package (optional): cargo install cargo-deb && cargo deb"

