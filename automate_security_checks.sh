#!/bin/bash

# Function to run snyk test
run_snyk_test() {
    echo "Running snyk test...
    
    if command -v snyk &> /dev/null; then
        snyk test
    else
        echo "snyk is not installed. Please install it first."
    fi
}

# Function to update dependencies and verify versions
update_dependencies() {
    echo "Updating dependencies...
    
    # Update Cargo.toml to resolve syntax issues manually
    echo "Manually updating dependencies in Cargo.toml...
    
    # Example: Update oqs, zeroize, and secrecy to latest versions if needed
    # This is a placeholder; actual updates should be done via Cargo.toml edits
    
    # Check current versions
    echo "Current versions:"
    echo "oqs: $(grep 'oqs' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')"
    echo "zeroize: $(grep 'zeroize' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')"
    echo "secrecy: $(grep 'secrecy' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')"
    
    # Run cargo update (if syntax issues are resolved)
    if cargo update > /dev/null 2>&1; then
        echo "Cargo update completed successfully."
    else
        echo "Cargo update failed. Check Cargo.toml syntax."
    fi
}

# Main execution
run_snyk_test
update_dependencies