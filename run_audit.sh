#!/bin/bash

# Function to run cargo audit in a controlled environment
run_cargo_audit() {
    echo "Running cargo audit...
    
    # Navigate to the project directory
    cd /home/admin/Aetheris || exit 1
    
    # Run cargo audit
    cargo audit --no-deps || echo "Cargo audit failed due to potential syntax issues. Check Cargo.toml."
}

# Main execution
run_cargo_audit