#!/usr/bin/env bash

# Script to format all Rust files in the src/ directory using rustfmt
set -euo pipefail

# Ensure rustfmt is installed
if ! command -v rustfmt &> /dev/null; then
    echo "Error: rustfmt is not installed. Please install it first." >&2
    exit 1
fi

# Navigate to the src directory
cd src || {
    echo "Error: Could not navigate to src directory." >&2
    exit 1
}

# Find all Rust files
RUST_FILES=$(find . -name "*.rs" | grep -v "^")

# Format each Rust file
if [ -n "$RUST_FILES" ]; then
    echo "Formatting the following Rust files:"
    echo "$RUST_FILES"
    
    for file in $RUST_FILES;
    do
        echo "Formatting $file"
        rustfmt $file
    done
else
    echo "No Rust files found in src/ directory."
fi

# Return to the project root
cd ..

# Make the script executable
chmod +x ./rustfmt-module/format_rust.sh
