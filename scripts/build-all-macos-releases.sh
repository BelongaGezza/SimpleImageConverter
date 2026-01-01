#!/bin/bash
# Build macOS binaries for all release versions
# This script checks out each release, builds the binaries for both architectures, and packages them

set -e

# Source Rust environment if available
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# macOS targets to build
TARGETS=("x86_64-apple-darwin" "aarch64-apple-darwin")

# Check for required tools
check_dependencies() {
    local missing=()
    
    # Try to find cargo in common locations
    if ! command -v cargo &> /dev/null; then
        # Check ~/.cargo/bin
        if [ -f "$HOME/.cargo/bin/cargo" ]; then
            export PATH="$HOME/.cargo/bin:$PATH"
        # Check if .cargo/env exists and source it
        elif [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi
    fi
    
    # Check again after trying to source
    if ! command -v cargo &> /dev/null; then
        missing+=("Rust/Cargo (install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)")
        missing+=("Or install via Homebrew: brew install rust")
    fi
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}Error: Missing required dependencies:${NC}"
        for dep in "${missing[@]}"; do
            echo "  - $dep"
        done
        echo ""
        echo "After installing Rust, make sure to:"
        echo "  1. Restart your terminal, or"
        echo "  2. Run: source \"\$HOME/.cargo/env\""
        exit 1
    fi
    
    echo -e "${GREEN}✓ Found cargo at: $(which cargo)${NC}"
    
    # Check if rustup is available for target management
    if command -v rustup &> /dev/null; then
        echo -e "${GREEN}✓ Found rustup - will install missing targets automatically${NC}"
    else
        echo -e "${YELLOW}Warning: rustup not found - assuming targets are already installed${NC}"
    fi
}

# Install target if needed
install_target() {
    local target=$1
    
    if command -v rustup &> /dev/null; then
        if ! rustup target list --installed | grep -q "^${target}$"; then
            echo "Installing target: $target..."
            rustup target add "$target" || {
                echo -e "${RED}Warning: Could not install target $target${NC}"
                echo "You may need to install it manually: rustup target add $target"
                return 1
            }
        fi
    fi
}

# Check dependencies before starting
check_dependencies

# Install required targets
for target in "${TARGETS[@]}"; do
    install_target "$target"
done

# Save current state
ORIGINAL_BRANCH=$(git branch --show-current 2>/dev/null || echo "")
ORIGINAL_COMMIT=$(git rev-parse HEAD 2>/dev/null || echo "")

# Get all release tags
echo "Finding release tags..."
RELEASE_TAGS=($(git tag --list | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+' | sort -V))

if [ ${#RELEASE_TAGS[@]} -eq 0 ]; then
    echo -e "${RED}Error: No release tags found${NC}"
    exit 1
fi

echo "Found ${#RELEASE_TAGS[@]} release tag(s): ${RELEASE_TAGS[*]}"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building macOS binaries for all releases...${NC}"
if [ -n "$ORIGINAL_BRANCH" ]; then
    echo "Original branch: $ORIGINAL_BRANCH"
fi
if [ -n "$ORIGINAL_COMMIT" ]; then
    echo "Original commit: $ORIGINAL_COMMIT"
fi
echo ""

# Function to build and package a release for a specific target
build_release_target() {
    local version=$1
    local target=$2
    
    echo -e "${YELLOW}Building for ${target}...${NC}"
    
    # Clean build artifacts for this target
    cargo clean --target "$target" 2>/dev/null || true
    
    # Build release binaries for this target
    echo "Building release binaries for $target..."
    if ! cargo build --release --target "$target"; then
        echo -e "${RED}Error: Build failed for v${version} on ${target}${NC}"
        return 1
    fi
    
    # Check which binaries exist
    local has_cli=false
    local has_gui=false
    local bin_dir="target/${target}/release"
    
    if [ -f "${bin_dir}/img-convert" ] && [ -f "${bin_dir}/mesh-convert" ]; then
        has_cli=true
    fi
    
    if [ -f "${bin_dir}/converter-gui" ]; then
        has_gui=true
    fi
    
    # Package CLI binaries if they exist
    if [ "$has_cli" = true ]; then
        echo "Packaging CLI binaries for $target..."
        if [ -f "scripts/package-macos.sh" ]; then
            chmod +x scripts/package-macos.sh
            if ./scripts/package-macos.sh "$version" "$target"; then
                echo -e "${GREEN}✓ CLI package created for v${version} on ${target}${NC}"
            else
                echo -e "${RED}✗ CLI packaging failed for v${version} on ${target}${NC}"
            fi
        else
            echo -e "${YELLOW}Warning: package-macos.sh not found for v${version}${NC}"
        fi
    else
        echo -e "${YELLOW}Note: CLI binaries not found for v${version} on ${target}${NC}"
    fi
    
    # Package GUI binary if it exists
    if [ "$has_gui" = true ]; then
        echo "Packaging GUI binary for $target..."
        if [ -f "scripts/package-gui-macos.sh" ]; then
            chmod +x scripts/package-gui-macos.sh
            if ./scripts/package-gui-macos.sh "$version" "$target"; then
                echo -e "${GREEN}✓ GUI package created for v${version} on ${target}${NC}"
            else
                echo -e "${RED}✗ GUI packaging failed for v${version} on ${target}${NC}"
            fi
        else
            echo -e "${YELLOW}Warning: package-gui-macos.sh not found for v${version}${NC}"
        fi
    else
        echo -e "${YELLOW}Note: GUI binary not found for v${version} on ${target}${NC}"
    fi
    
    echo ""
}

# Function to build and package a release
build_release() {
    local tag=$1
    local version=${tag#v}  # Remove 'v' prefix
    
    echo -e "${YELLOW}========================================${NC}"
    echo -e "${YELLOW}Building v${version} (tag: ${tag})${NC}"
    echo -e "${YELLOW}========================================${NC}"
    
    # Checkout the release tag
    echo "Checking out tag ${tag}..."
    # Stash any local changes before checkout
    if ! git diff --quiet || ! git diff --cached --quiet; then
        echo "Stashing local changes..."
        git stash --include-untracked > /dev/null 2>&1 || true
    fi
    if ! git checkout "$tag" 2>&1; then
        echo -e "${RED}Error: Could not checkout tag ${tag}${NC}"
        return 1
    fi
    
    # Clean all build artifacts
    echo "Cleaning previous build artifacts..."
    cargo clean
    
    # Build for each target architecture
    for target in "${TARGETS[@]}"; do
        build_release_target "$version" "$target"
    done
    
    echo ""
}

# Build all releases
for tag in "${RELEASE_TAGS[@]}"; do
    build_release "$tag"
done

# Return to original state
if [ -n "$ORIGINAL_COMMIT" ]; then
    echo -e "${GREEN}Returning to original state...${NC}"
    if [ -n "$ORIGINAL_BRANCH" ]; then
        git checkout "$ORIGINAL_BRANCH" 2>/dev/null || git checkout "$ORIGINAL_COMMIT"
    else
        git checkout "$ORIGINAL_COMMIT"
    fi
    echo -e "${GREEN}Done!${NC}"
else
    echo -e "${YELLOW}Note: Could not determine original state, staying on current commit${NC}"
fi

# List created packages
echo ""
echo -e "${GREEN}Created packages:${NC}"
if ls release/simpleimageconverter-*-macos-*.tar.gz 2>/dev/null; then
    ls -lh release/simpleimageconverter-*-macos-*.tar.gz
else
    echo "No CLI packages found in release/ directory"
fi
echo ""
if ls release/simpleimageconverter-gui-*-macos-*.tar.gz 2>/dev/null; then
    ls -lh release/simpleimageconverter-gui-*-macos-*.tar.gz
else
    echo "No GUI packages found in release/ directory"
fi

