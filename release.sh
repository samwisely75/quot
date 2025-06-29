#!/bin/bash

# Release script for quot following git flow workflow
# This script automates the entire release process

set -e  # Exit on any error

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

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
print_status "Current version: $CURRENT_VERSION"

# Check if we're on develop branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "develop" ]; then
    print_error "Must be on develop branch to start release. Currently on: $CURRENT_BRANCH"
    exit 1
fi

print_status "Starting release process for version $CURRENT_VERSION"

# Step 1: Create release branch
RELEASE_BRANCH="release/$CURRENT_VERSION"
print_status "Creating release branch: $RELEASE_BRANCH"

# Clean up any existing conflicting tags that might cause issues
print_status "Cleaning up any conflicting tags..."
if git tag -l | grep -q "^release/$CURRENT_VERSION$"; then
    print_warning "Found conflicting tag 'release/$CURRENT_VERSION', deleting it..."
    git tag -d "release/$CURRENT_VERSION"
fi

if git ls-remote --tags origin | grep -q "refs/tags/release/$CURRENT_VERSION$"; then
    print_warning "Found conflicting remote tag 'release/$CURRENT_VERSION', deleting it..."
    git push origin ":refs/tags/release/$CURRENT_VERSION"
fi

git checkout -b "$RELEASE_BRANCH"

# Step 2: Run comprehensive tests
print_status "Running comprehensive test suite..."
cargo test --verbose --all-features

print_success "All tests passed!"

# Step 3: Run clippy with strict warnings
print_status "Running clippy checks..."
cargo clippy --all-features -- -D warnings

print_success "Clippy checks passed!"

# Step 4: Check formatting
print_status "Checking code formatting..."
cargo fmt -- --check

print_success "Code formatting is correct!"

# Step 5: Build release version
print_status "Building release version..."
cargo build --release --all-features

print_success "Release build completed!"

# Step 6: Test the binary
print_status "Testing the release binary..."
echo 'Hello "world"!' | ./target/release/quot > /tmp/quot_test_output
EXPECTED='"Hello \"world\"!\n"'
ACTUAL=$(cat /tmp/quot_test_output)
if [ "$ACTUAL" = "$EXPECTED" ]; then
    print_success "Binary functionality test passed!"
else
    print_error "Binary test failed. Expected: $EXPECTED, Got: $ACTUAL"
    exit 1
fi

# Step 7: Run any additional release tests
print_status "Running additional release validation..."
./target/release/quot --help > /dev/null

print_success "All release validations passed!"

# Step 8: Commit any final changes (if needed)
if ! git diff --quiet; then
    print_warning "There are uncommitted changes. Committing them now..."
    git add .
    git commit -m "chore: final release preparations for v$CURRENT_VERSION"
fi

# Step 9: Push release branch
print_status "Pushing release branch to origin..."
git push origin "$RELEASE_BRANCH"

print_success "Release branch pushed successfully!"

# Step 10: Create and push release tag
VERSION_TAG="v$CURRENT_VERSION"
print_status "Creating release tag: $VERSION_TAG"
git tag "$VERSION_TAG"
git push origin "$VERSION_TAG"

print_success "Release tag '$VERSION_TAG' created and pushed!"

# Step 11: Final status
print_success "=================================="
print_success "RELEASE FULLY AUTOMATED!"
print_success "=================================="
print_status "Release branch '$RELEASE_BRANCH' has been created and pushed."
print_status "Release tag '$VERSION_TAG' has been created and pushed."
print_status ""
print_status "The release workflow will automatically:"
print_status "1. Build cross-platform binaries"
print_status "2. Create GitHub release with assets"
print_status "3. Merge release branch to main"
print_status "4. Merge main back to develop"
print_status "5. Clean up release branch"
print_status ""
print_status "You can monitor the release at:"
print_status "https://github.com/samwisely75/quot/actions"

print_success "🚀 Release v$CURRENT_VERSION is now fully automated!"
print_status "🎯 No further manual steps required!"
