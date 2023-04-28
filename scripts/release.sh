#!/bin/bash

set -e

# Set the name and version of the executable
name="toodeep"
version="0.1.0"

# Build the executable
cargo build --release

# Create a directory for the release
mkdir -p "./dist"

# Create a tar file of the executable
tar -czvf "./dist/$name-$version.tar.gz" "target/release/$name"

git tag "v$version"

# Push the tag
git push origin "v$version"

# Create a github release
gh release create "v$version" "./dist/$name-$version.tar.gz" --title "$version"