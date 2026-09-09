#!/bin/bash

# Function to check and set up snyk
setup_snyk() {
    echo "Setting up snyk...
    
    # Install snyk if not installed
    if ! command -v snyk &> /dev/null; then
        echo "Installing snyk...
        curl https://snyk.io/install.sh | sh"
        echo "Please log in to snyk with: snyk auth"
    else
        echo "snyk is already installed."
    fi
    
    # Run snyk test
    echo "Running snyk test...
    snyk test"
}

# Function to configure dependabot for security updates
configure_dependabot() {
    echo "\nConfiguring dependabot for security updates...
    
    # Check if dependabot.yml exists
    if [ -f dependabot.yml ]; then
        echo "dependabot.yml found. Updating security settings...
        
        # Ensure security updates are enabled
        sed -i 's/security-updates:.*/security-updates: true/' dependabot.yml
        echo "Updated dependabot.yml to enable security updates."
    else
        echo "dependabot.yml not found. Creating it...
        
        cat > dependabot.yml << 'EOF'
version: 2
include:
  - directory: "/"
    update:
      dependencies:
        - update-all:
            update: true
      default-branch: main
EOF
        echo "Created dependabot.yml with security updates enabled."
    fi
}

# Function to manually verify security-related crates
verify_versions() {
    echo "\nVerifying current versions of security-related crates:"
    
    # Extract versions from Cargo.toml
    oqs_version=$(grep 'oqs' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')
    zeroize_version=$(grep 'zeroize' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')
    secrecy_version=$(grep 'secrecy' Cargo.toml | awk -F'=' '{print $2}' | tr -d '"')
    
    echo "oqs: $oqs_version"
    echo "zeroize: $zeroize_version"
    echo "secrecy: $secrecy_version"
    
    # Check for updates and compliance
    echo "\nCheck the latest releases for these crates:"
    echo "- oqs: https://github.com/openquantumsafe/oqs/releases"
    echo "- zeroize: https://github.com/rust-crypto/zeroize/releases"
    echo "- secrecy: https://github.com/rust-crypto/secrecy/releases"
}

# Main execution
setup_snyk
configure_dependabot
verify_versions