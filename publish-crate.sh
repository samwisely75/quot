#!/bin/bash

# Helper script for publishing to crates.io
# 
# Steps to publish:
# 1. Go to https://crates.io and log in with GitHub
# 2. Go to https://crates.io/me and create a new API token
# 3. Run: cargo login <your-token>
# 4. Run this script

set -e

echo "🚀 Publishing quot to crates.io..."
echo

# Check if logged in
echo "📋 Checking authentication..."
if ! cargo login --help >/dev/null 2>&1; then
    echo "❌ cargo login not available"
    exit 1
fi

# Run tests first
echo "🧪 Running tests..."
cargo test --quiet
echo "✅ All tests passed"

# Check formatting and clippy
echo "🔍 Checking code quality..."
cargo fmt -- --check
cargo clippy -- -D warnings
echo "✅ Code quality checks passed"

# Final dry run
echo "📦 Running final dry run..."
cargo publish --dry-run >/dev/null
echo "✅ Package ready for publication"

# Ask for confirmation
echo
echo "🎯 Ready to publish quot v0.1.0 to crates.io!"
echo
echo "This will:"
echo "  - Publish your crate to crates.io"
echo "  - Make it installable via 'cargo install quot'"
echo "  - Make it available to the Rust community"
echo
read -p "Do you want to proceed? (y/N): " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🚀 Publishing..."
    cargo publish
    echo
    echo "🎉 Successfully published quot to crates.io!"
    echo
    echo "📦 Your crate is now available at: https://crates.io/crates/quot"
    echo "⚡ Users can install it with: cargo install quot"
    echo
    echo "🏷️  Don't forget to:"
    echo "   - Push the updated Cargo.toml: git push"
    echo "   - Update your GitHub release if needed"
else
    echo "❌ Publication cancelled"
fi
