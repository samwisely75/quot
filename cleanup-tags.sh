#!/bin/bash

# Clean up conflicting tags script
# This script removes any tags that follow the release/VERSION pattern
# which can conflict with release branches

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

print_status "Checking for conflicting release/* tags..."

# Find all local tags that match release/* pattern
CONFLICTING_LOCAL_TAGS=$(git tag -l | grep "^release/" || true)

if [ -n "$CONFLICTING_LOCAL_TAGS" ]; then
    print_warning "Found conflicting local tags:"
    echo "$CONFLICTING_LOCAL_TAGS"
    
    for tag in $CONFLICTING_LOCAL_TAGS; do
        print_status "Deleting local tag: $tag"
        git tag -d "$tag"
    done
else
    print_success "No conflicting local tags found"
fi

# Find all remote tags that match release/* pattern
CONFLICTING_REMOTE_TAGS=$(git ls-remote --tags origin | grep "refs/tags/release/" | sed 's/.*refs\/tags\///' || true)

if [ -n "$CONFLICTING_REMOTE_TAGS" ]; then
    print_warning "Found conflicting remote tags:"
    echo "$CONFLICTING_REMOTE_TAGS"
    
    for tag in $CONFLICTING_REMOTE_TAGS; do
        print_status "Deleting remote tag: $tag"
        git push origin ":refs/tags/$tag"
    done
else
    print_success "No conflicting remote tags found"
fi

print_success "Tag cleanup completed!"
print_status ""
print_status "Valid tag patterns for releases:"
print_status "✅ v0.1.3, v1.0.0, v2.1.4-beta"
print_status "❌ release/0.1.3, 0.1.3, version-0.1.3"
