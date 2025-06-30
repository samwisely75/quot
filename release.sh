#!/bin/bash

# Release script for quot following git flow workflow
# This script automates the entire release process
#
# Usage:
#   ./release.sh           - Run full release process with version increment
#   ./release.sh --dry-run - Preview version increment without making changes
#   ./release.sh -n        - Same as --dry-run
#
# Version increment options:
#   1) patch - Bug fixes (0.1.3 -> 0.1.4)
#   2) minor - New features (0.1.3 -> 0.2.0)  
#   3) major - Breaking changes (0.1.3 -> 1.0.0)
#   4) keep  - Keep current version (re-release same version)
#   5) custom - Enter version manually

set -e  # Exit on any error

# Check for dry-run flag
DRY_RUN=false
if [ "$1" = "--dry-run" ] || [ "$1" = "-n" ]; then
    DRY_RUN=true
fi

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

# Function to increment version based on type
increment_version() {
    local version=$1
    local type=$2
    
    IFS='.' read -ra VERSION_PARTS <<< "$version"
    local major=${VERSION_PARTS[0]}
    local minor=${VERSION_PARTS[1]}
    local patch=${VERSION_PARTS[2]}
    
    case $type in
        "major")
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        "minor")
            minor=$((minor + 1))
            patch=0
            ;;
        "patch")
            patch=$((patch + 1))
            ;;
        *)
            print_error "Invalid version type: $type. Use major, minor, or patch"
            exit 1
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Prompt for version increment type
print_status "Current version: $CURRENT_VERSION"
echo ""
print_status "Select version increment type:"
print_status "1) patch (0.1.3 -> 0.1.4) - Bug fixes"
print_status "2) minor (0.1.3 -> 0.2.0) - New features"
print_status "3) major (0.1.3 -> 1.0.0) - Breaking changes"
print_status "4) keep ($CURRENT_VERSION) - Keep current version (re-release)"
print_status "5) custom - Enter version manually"
echo ""

read -p "Enter your choice (1-5): " choice

case $choice in
    1)
        NEW_VERSION=$(increment_version "$CURRENT_VERSION" "patch")
        ;;
    2)
        NEW_VERSION=$(increment_version "$CURRENT_VERSION" "minor")
        ;;
    3)
        NEW_VERSION=$(increment_version "$CURRENT_VERSION" "major")
        ;;
    4)
        NEW_VERSION=$CURRENT_VERSION
        print_warning "Keeping current version: $NEW_VERSION"
        print_warning "Note: This may cause conflicts if the version was already released."
        ;;
    5)
        read -p "Enter new version (e.g., 1.0.0): " NEW_VERSION
        # Validate version format
        if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            print_error "Invalid version format. Use semantic versioning (e.g., 1.0.0)"
            exit 1
        fi
        ;;
    *)
        print_error "Invalid choice. Exiting."
        exit 1
        ;;
esac

print_status "Updating version from $CURRENT_VERSION to $NEW_VERSION"

if [ "$DRY_RUN" = true ]; then
    print_warning "DRY RUN MODE - No changes will be made"
    if [ "$CURRENT_VERSION" = "$NEW_VERSION" ]; then
        print_status "Would keep current version: $NEW_VERSION (no version bump)"
    else
        print_status "Would update version in Cargo.toml from $CURRENT_VERSION to $NEW_VERSION"
    fi
    print_status "Would commit version changes (if any)"
    print_status "Would continue with release process for version $NEW_VERSION"
    exit 0
fi

# Update version in Cargo.toml (only if version changed)
if [ "$CURRENT_VERSION" != "$NEW_VERSION" ]; then
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

    # Verify the change was made
    UPDATED_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    if [ "$UPDATED_VERSION" != "$NEW_VERSION" ]; then
        print_error "Failed to update version in Cargo.toml"
        exit 1
    fi

    print_success "Version updated to $NEW_VERSION in Cargo.toml"

    # Update Cargo.lock
    print_status "Updating Cargo.lock..."
    cargo check --quiet

    # Commit the version change
    print_status "Committing version bump..."
    git add Cargo.toml Cargo.lock
    git commit -m "bump version to $NEW_VERSION"

    print_success "Version bump committed!"
else
    print_success "Keeping current version: $NEW_VERSION (no version bump needed)"
fi

# Set CURRENT_VERSION to the new version for the rest of the script
CURRENT_VERSION=$NEW_VERSION

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

# Note: The GitHub Actions workflow will automatically:
# - Create and push the version tag
# - Build cross-platform binaries
# - Create GitHub release with assets
# - Merge release branch to main
# - Merge main back to develop
# - Clean up release branch

# Step 10: Final status
print_success "=================================="
print_success "RELEASE INITIATED!"
print_success "=================================="
print_status "Release branch '$RELEASE_BRANCH' has been created and pushed."
print_status ""
print_status "The release workflow will automatically:"
print_status "1. Create version tag (v$CURRENT_VERSION)"
print_status "2. Build cross-platform binaries"
print_status "3. Create GitHub release with assets"
print_status "4. Merge release branch to main"
print_status "5. Merge main back to develop"
print_status "6. Clean up release branch"
print_status ""
print_status "You can monitor the release at:"
print_status "https://github.com/samwisely75/quot/actions"

print_success "🚀 Release v$CURRENT_VERSION initiated!"
print_status "🎯 GitHub Actions will handle the rest automatically!"
