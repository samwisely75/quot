#!/bin/bash

# Build script for creating cross-platform releases
# This script builds quot for all major platforms and architectures

set -e

VERSION=${1:-"0.1.0"}
RELEASE_DIR="release-$VERSION"

echo "🚀 Building quot v$VERSION for all platforms..."

# Clean previous builds
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Build targets
TARGETS=(
    "x86_64-pc-windows-msvc"      # Windows x64
    "aarch64-pc-windows-msvc"     # Windows ARM64
    "x86_64-apple-darwin"         # macOS Intel
    "aarch64-apple-darwin"        # macOS Apple Silicon
    "x86_64-unknown-linux-gnu"    # Linux x64
    "aarch64-unknown-linux-gnu"   # Linux ARM64
)

# Platform-specific binary names
declare -A BINARY_NAMES=(
    ["x86_64-pc-windows-msvc"]="quot-windows-x64.exe"
    ["aarch64-pc-windows-msvc"]="quot-windows-arm64.exe"
    ["x86_64-apple-darwin"]="quot-macos-x64"
    ["aarch64-apple-darwin"]="quot-macos-arm64"
    ["x86_64-unknown-linux-gnu"]="quot-linux-x64"
    ["aarch64-unknown-linux-gnu"]="quot-linux-arm64"
)

# Install targets first
echo "📦 Installing Rust targets..."
for target in "${TARGETS[@]}"; do
    rustup target add "$target"
done

# Build for each target
for target in "${TARGETS[@]}"; do
    echo "🔨 Building for $target..."
    
    # Special handling for Linux ARM64 (requires cross-compilation setup)
    if [[ "$target" == "aarch64-unknown-linux-gnu" ]]; then
        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            # On Linux, install cross-compilation tools
            sudo apt-get update -qq
            sudo apt-get install -y gcc-aarch64-linux-gnu
            export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            # On macOS, use Docker for Linux ARM64 builds
            echo "🐳 Using Docker for Linux ARM64 build..."
            docker run --rm -v "$(pwd)":/workspace -w /workspace \
                rust:latest bash -c "
                    rustup target add aarch64-unknown-linux-gnu && \
                    apt-get update && apt-get install -y gcc-aarch64-linux-gnu && \
                    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
                    cargo build --release --target aarch64-unknown-linux-gnu
                "
        else
            echo "⚠️  Skipping Linux ARM64 build on this platform"
            continue
        fi
    else
        cargo build --release --target "$target"
    fi
    
    # Copy binary to release directory
    binary_name="${BINARY_NAMES[$target]}"
    if [[ "$target" == *"windows"* ]]; then
        cp "target/$target/release/quot.exe" "$RELEASE_DIR/$binary_name"
    else
        cp "target/$target/release/quot" "$RELEASE_DIR/$binary_name"
        chmod +x "$RELEASE_DIR/$binary_name"
    fi
    
    echo "✅ Built $binary_name"
done

echo "📋 Creating checksums..."
cd "$RELEASE_DIR"
if command -v sha256sum >/dev/null 2>&1; then
    sha256sum * > SHA256SUMS
elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 * > SHA256SUMS
fi
cd ..

echo "🎉 Release build complete!"
echo "📁 Binaries are in: $RELEASE_DIR/"
echo ""
echo "📦 Available binaries:"
ls -la "$RELEASE_DIR/"

echo ""
echo "🚀 To create a GitHub release:"
echo "   1. Create a tag: git tag v$VERSION"
echo "   2. Push the tag: git push origin v$VERSION"
echo "   3. The GitHub Action will automatically create the release"
echo ""
echo "Or upload manually from the $RELEASE_DIR/ directory"
