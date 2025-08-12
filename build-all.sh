#!/bin/bash

# Ez-Encrypt Multi-Platform Build Script
# This script builds the application for Windows, macOS, and Linux

set -e  # Exit on any error

echo "🔨 Ez-Encrypt Multi-Platform Build Script"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

print_status "Rust version: $(rustc --version)"

# Create builds directory
BUILDS_DIR="builds"
if [ ! -d "$BUILDS_DIR" ]; then
    mkdir -p "$BUILDS_DIR"
    print_status "Created builds directory"
fi

# Clean previous builds
print_status "Cleaning previous builds..."
cargo clean
rm -rf "$BUILDS_DIR"/*

# Build targets
TARGETS=(
    "x86_64-pc-windows-gnu,Windows,ez-encrypt.exe"
    "x86_64-apple-darwin,macOS,ez-encrypt"
    "x86_64-unknown-linux-gnu,Linux,ez-encrypt"
)

# Function to build for a target
build_target() {
    local target=$1
    local platform=$2
    local executable=$3

    print_status "Building for $platform ($target)..."

    # Check if target is installed
    if ! rustup target list --installed | grep -q "$target"; then
        print_status "Installing target $target..."
        if ! rustup target add "$target"; then
            print_warning "Failed to install target $target. Skipping..."
            return 1
        fi
    fi

    # Build the target
    if cargo build --release --target "$target"; then
        # Create platform directory
        platform_dir="$BUILDS_DIR/$platform"
        mkdir -p "$platform_dir"

        # Copy executable
        source_path="target/$target/release/$executable"
        if [ -f "$source_path" ]; then
            cp "$source_path" "$platform_dir/"
            print_success "Built $platform successfully -> $platform_dir/$executable"

            # Get file size
            if command -v du &> /dev/null; then
                size=$(du -h "$platform_dir/$executable" | cut -f1)
                print_status "File size: $size"
            fi
        else
            print_error "Expected executable not found: $source_path"
            return 1
        fi
    else
        print_error "Build failed for $platform ($target)"
        return 1
    fi
}

# Build for each target
successful_builds=0
total_builds=${#TARGETS[@]}

echo
print_status "Starting builds for ${total_builds} platforms..."
echo

for target_info in "${TARGETS[@]}"; do
    IFS=',' read -r target platform executable <<< "$target_info"

    if build_target "$target" "$platform" "$executable"; then
        ((successful_builds++))
    fi
    echo
done

# Summary
echo "=========================================="
print_status "Build Summary:"
print_status "Successful builds: $successful_builds/$total_builds"

if [ $successful_builds -eq $total_builds ]; then
    print_success "All builds completed successfully! 🎉"
else
    print_warning "Some builds failed. Check the output above for details."
fi

# List created files
if [ -d "$BUILDS_DIR" ] && [ "$(ls -A $BUILDS_DIR)" ]; then
    echo
    print_status "Created files:"
    find "$BUILDS_DIR" -type f -name "ez-encrypt*" -exec ls -lh {} \; | while read -r line; do
        echo "  $line"
    done
fi

echo
print_status "Build script completed!"

# Platform-specific notes
echo
echo "📋 Platform-specific notes:"
echo "  Windows: Use builds/Windows/ez-encrypt.exe"
echo "  macOS:   Use builds/macOS/ez-encrypt (may need: chmod +x ez-encrypt)"
echo "  Linux:   Use builds/Linux/ez-encrypt (may need: chmod +x ez-encrypt)"
echo
echo "💡 For distribution:"
echo "  1. Test each executable on its target platform"
echo "  2. Consider code signing for Windows/macOS"
echo "  3. Package with README and license files"
