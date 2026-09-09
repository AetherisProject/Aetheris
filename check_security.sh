#!/bin/bash

# Function to check current versions of security-related crates
check_versions() {
    echo "Checking current versions of security-related crates:"
    
    # Extract versions from Cargo.toml
    oqs_version=$(grep 'oqs' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')
    zeroize_version=$(grep 'zeroize' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')
    secrecy_version=$(grep 'secrecy' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')
    
    echo "oqs: $oqs_version"
    echo "zeroize: $zeroize_version"
    echo "secrecy: $secrecy_version"
}

# Function to run snyk test
run_snyk_test() {
    echo "\nRunning snyk test...
    
    if command -v snyk &> /dev/null; then
        echo "Running snyk test...
        snyk test"
    else
        echo "snyk is not installed. Please install it first."
    fi
}

# Main execution
check_versions
run_snyk_test