#!/bin/bash
# Build Linux binaries for all release versions
# This script checks out each release, builds the binaries, and packages them

set -e

# Source Rust environment if available
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Check for required tools
check_dependencies() {
    local missing=()
    
    if ! command -v cargo &> /dev/null; then
        missing+=("Rust/Cargo (install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)")
    fi
    
    if ! command -v gcc &> /dev/null && ! command -v cc &> /dev/null; then
        missing+=("C compiler (install with: sudo apt-get install build-essential)")
    fi
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}Error: Missing required dependencies:${NC}"
        for dep in "${missing[@]}"; do
            echo "  - $dep"
        done
        exit 1
    fi
}

# Check dependencies before starting
check_dependencies

# Save current state
ORIGINAL_BRANCH=$(git branch --show-current)
ORIGINAL_COMMIT=$(git rev-parse HEAD)

# Release versions and their corresponding commits
declare -A RELEASES=(
    ["0.1.0"]="fd2d21a"
    ["0.1.1"]="5686fec"
    ["0.2.0"]="1597128"
    ["0.2.1"]="dcbad58"
    ["0.2.2"]="156624c"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Linux binaries for all releases...${NC}"
echo "Original branch: $ORIGINAL_BRANCH"
echo "Original commit: $ORIGINAL_COMMIT"
echo ""

# Function to build and package a release
build_release() {
    local version=$1
    local commit=$2
    
    echo -e "${YELLOW}========================================${NC}"
    echo -e "${YELLOW}Building v${version} (commit ${commit})${NC}"
    echo -e "${YELLOW}========================================${NC}"
    
    # Checkout the release commit
    echo "Checking out commit ${commit}..."
    git checkout "$commit" 2>/dev/null || {
        echo -e "${RED}Error: Could not checkout commit ${commit}${NC}"
        return 1
    }
    
    # Clean build artifacts
    echo "Cleaning previous build artifacts..."
    cargo clean
    
    # Build release binaries
    echo "Building release binaries..."
    if ! cargo build --release; then
        echo -e "${RED}Error: Build failed for v${version}${NC}"
        return 1
    fi
    
    # Check which binaries exist
    local has_cli=false
    local has_gui=false
    
    if [ -f "target/release/img-convert" ] && [ -f "target/release/mesh-convert" ]; then
        has_cli=true
    fi
    
    if [ -f "target/release/converter-gui" ]; then
        has_gui=true
    fi
    
    # Package CLI binaries if they exist
    if [ "$has_cli" = true ]; then
        echo "Packaging CLI binaries..."
        if [ -f "scripts/package-linux.sh" ]; then
            chmod +x scripts/package-linux.sh
            if ./scripts/package-linux.sh "$version"; then
                echo -e "${GREEN}✓ CLI package created for v${version}${NC}"
            else
                echo -e "${RED}✗ CLI packaging failed for v${version}${NC}"
            fi
        else
            echo -e "${YELLOW}Warning: package-linux.sh not found for v${version}${NC}"
        fi
    else
        echo -e "${YELLOW}Note: CLI binaries not found for v${version}${NC}"
    fi
    
    # Package GUI binary if it exists
    if [ "$has_gui" = true ]; then
        echo "Packaging GUI binary..."
        if [ -f "scripts/package-gui-linux.sh" ]; then
            chmod +x scripts/package-gui-linux.sh
            if ./scripts/package-gui-linux.sh "$version"; then
                echo -e "${GREEN}✓ GUI package created for v${version}${NC}"
            else
                echo -e "${RED}✗ GUI packaging failed for v${version}${NC}"
            fi
        else
            echo -e "${YELLOW}Warning: package-gui-linux.sh not found for v${version}${NC}"
        fi
    else
        echo -e "${YELLOW}Note: GUI binary not found for v${version}${NC}"
    fi
    
    echo ""
}

# Build all releases
for version in "${!RELEASES[@]}"; do
    commit="${RELEASES[$version]}"
    build_release "$version" "$commit"
done

# Return to original state
echo -e "${GREEN}Returning to original state...${NC}"
git checkout "$ORIGINAL_BRANCH" 2>/dev/null || git checkout "$ORIGINAL_COMMIT"
echo -e "${GREEN}Done!${NC}"

# List created packages
echo ""
echo -e "${GREEN}Created packages:${NC}"
ls -lh release/*.tar.gz 2>/dev/null || echo "No packages found in release/ directory"

