#!/bin/bash
# Linux Packaging Script
# Creates a portable TAR.GZ archive for Linux distribution

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
        VERSION="0.2.0"
    fi
    echo "Using version: $VERSION" >&2
else
    VERSION="$1"
fi

# SECURITY: Validate version format to prevent injection attacks
# Allow semantic versioning: X.Y.Z or X.Y.Z-pre (e.g., 0.2.0, 0.2.0-alpha1)
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

echo "Packaging SimpleImageConverter for Linux..."

# Set paths
RELEASE_DIR="release/linux-x64-v${VERSION}"
TAR_NAME="simpleimageconverter-${VERSION}-linux-x64.tar.gz"
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

