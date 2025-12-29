#!/bin/bash
# macOS Packaging Script
# Creates a portable TAR.GZ archive for macOS distribution

set -e

# Extract version from Git tag or Cargo.toml if not provided
if [ -z "$1" ]; then
    # Try to get version from Git tag
    VERSION=$(git describe --tags --exact-match 2>/dev/null | sed 's/^v//' || echo "")
    if [ -z "$VERSION" ]; then
        # Fall back to Cargo.toml
        VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d'"' -f2)
    fi
    if [ -z "$VERSION" ]; then
        VERSION="0.2.0"
    fi
    echo "Using version: $VERSION" >&2
else
    VERSION="$1"
fi

TARGET="${2:-x86_64-apple-darwin}"

echo "Packaging SimpleImageConverter for macOS..."

# Set paths
RELEASE_DIR="release/macos-x64-v${VERSION}"
TAR_NAME="simpleimageconverter-${VERSION}-macos-x64.tar.gz"
BIN_DIR="target/${TARGET}/release"
NATIVE_BIN_DIR="target/release"

# Clean previous release
rm -rf "$RELEASE_DIR"
rm -f "$TAR_NAME"

# Create release directory
mkdir -p "$RELEASE_DIR"

# Determine binary location (check cross-compiled first, then native)
IMG_CONVERT_PATH=""
MESH_CONVERT_PATH=""

if [ -f "$BIN_DIR/img-convert" ]; then
    IMG_CONVERT_PATH="$BIN_DIR/img-convert"
    MESH_CONVERT_PATH="$BIN_DIR/mesh-convert"
    echo "Using cross-compiled binaries from: $BIN_DIR" >&2
elif [ -f "$NATIVE_BIN_DIR/img-convert" ]; then
    IMG_CONVERT_PATH="$NATIVE_BIN_DIR/img-convert"
    MESH_CONVERT_PATH="$NATIVE_BIN_DIR/mesh-convert"
    echo "Using native binaries from: $NATIVE_BIN_DIR" >&2
else
    echo "Error: Binaries not found. Expected locations:" >&2
    echo "  - $BIN_DIR/img-convert" >&2
    echo "  - $NATIVE_BIN_DIR/img-convert" >&2
    echo "Run 'cargo build --release' or 'cargo build --release --target $TARGET' first." >&2
    exit 1
fi

# Copy binaries
echo "Copying binaries..."
cp "$IMG_CONVERT_PATH" "$RELEASE_DIR/"
cp "$MESH_CONVERT_PATH" "$RELEASE_DIR/"

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

