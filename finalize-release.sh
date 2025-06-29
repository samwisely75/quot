#!/bin/bash

# Finalize release script - run after release branch is merged to main
# Usage: ./finalize-release.sh v0.1.3

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <version-tag>"
    echo "Example: $0 v0.1.3"
    exit 1
fi

VERSION_TAG=$1

echo "Finalizing release $VERSION_TAG..."

# Switch to main and pull latest
git checkout main
git pull origin main

# Create and push tag
git tag $VERSION_TAG
git push origin $VERSION_TAG

# Merge main back to develop
git checkout develop
git merge main
git push origin develop

echo "Release $VERSION_TAG finalized successfully!"
echo "GitHub Release will be automatically created by the release workflow."
