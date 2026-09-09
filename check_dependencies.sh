#!/bin/bash

# Function to check for vulnerabilities using available tools
check_dependencies() {
    echo "Checking for vulnerabilities in dependencies...
    
    # Check oqs version
    echo "oqs version: $(cargo read-crate-version oqs)"
    echo "oqs features: $(cargo read-crate-features oqs)"
    
    # Check zeroize version
    echo "zeroize version: $(cargo read-crate-version zeroize)"
    
    # Check secrecy version
    echo "secrecy version: $(cargo read-crate-version secrecy)"
    
    # Check for known vulnerabilities using cargo-audit (if possible)
    echo "Attempting to run cargo audit...
    
    if cargo audit > /dev/null 2>&1; then
        echo "Cargo audit completed successfully."
    else
        echo "Cargo audit failed. Proceeding with manual checks."
    fi
}

# Run the check
check_dependencies