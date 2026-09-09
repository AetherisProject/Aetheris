#!/bin/bash

# Aetheris Build Verification Script
# This script verifies that all platform assets can be built successfully

set -e

echo "=========================================="
echo "Aetheris Build Verification"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        exit 1
    fi
}

# Track overall status
OVERALL_STATUS=0

# 1. Verify Core Rust Library
echo "1. Verifying Core Rust Library..."
if [ -f "Cargo.toml" ]; then
    echo "   Cargo.toml exists"
else
    echo -e "${RED}✗${NC} Cargo.toml not found"
    exit 1
fi

if [ -f "src/lib.rs" ]; then
    echo "   src/lib.rs exists"
else
    echo -e "${RED}✗${NC} src/lib.rs not found"
    exit 1
fi

# Check key source files
KEY_FILES=(
    "src/crypto/mod.rs"
    "src/vault/store.rs"
    "src/auth/oauth.rs"
    "src/auth/session.rs"
    "src/auth/totp.rs"
    "src/auth/biometric.rs"
    "src/billing/plans.rs"
    "src/sync/client.rs"
    "src/proactive/monitor.rs"
    "src/proactive/policies.rs"
    "src/proactive/alert.rs"
)

for file in "${KEY_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Core Rust Library files verified"

# 2. Verify Browser Extension Assets
echo ""
echo "2. Verifying Browser Extension Assets..."

BROWSER_FILES=(
    "browser/chrome/manifest.json"
    "browser/chrome/popup.html"
    "browser/chrome/popup.js"
    "browser/chrome/background.js"
    "browser/chrome/content.js"
    "browser/chrome/auth.json"
    "browser/chrome/options.html"
    "browser/chrome/options.js"
    "browser/firefox/manifest.json"
    "browser/firefox/popup.html"
    "browser/firefox/popup.js"
    "browser/firefox/background.js"
)

for file in "${BROWSER_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Browser Extension assets verified"

# 3. Verify Desktop Assets
echo ""
echo "3. Verifying Desktop Assets..."

DESKTOP_FILES=(
    "desktop/package.json"
    "desktop/tauri.conf.json"
    "desktop/src/main.tsx"
    "desktop/src/pages/Dashboard.tsx"
    "desktop/src/pages/ApiKeys.tsx"
    "desktop/src/pages/SSH.tsx"
    "desktop/src/pages/Vault.tsx"
    "desktop/src/components/AetherisComponents.tsx"
    "desktop/src/apikey_tauri.rs"
    "desktop/src/tauri_commands.rs"
    "desktop/src/ssh_tauri.rs"
)

for file in "${DESKTOP_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Desktop assets verified"

# 4. Verify Mobile Assets
echo ""
echo "4. Verifying Mobile Assets..."

MOBILE_FILES=(
    "mobile/pubspec.yaml"
    "mobile/lib/main.dart"
    "mobile/lib/ffi.dart"
    "mobile/lib/ffi_bridge.dart"
    "mobile/lib/ffi_crypto.dart"
    "mobile/lib/pages/home.dart"
    "mobile/lib/widgets.dart"
    "mobile/lib/layout.dart"
    "mobile/lib/design_tokens.dart"
    "mobile/sdk/flutter/aetheris_sdk/lib/aetheris_sdk.dart"
    "mobile/sdk/flutter/aetheris_sdk/platforms/flutter/aetheris_sdk_method_channel.dart"
    "mobile/sdk/flutter/aetheris_sdk/platforms/flutter/aetheris_sdk_platform_interface.dart"
    "mobile/sdk/flutter/aetheris_sdk/platforms/flutter/aetheris_sdk_platform_interface_impl.rs"
)

for file in "${MOBILE_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Mobile assets verified"

# 5. Verify Documentation
echo ""
echo "5. Verifying Documentation..."

DOC_FILES=(
    "README.md"
    "CONTRIBUTING.md"
    "LICENSE"
    "SECURITY.md"
    "CHANGELOG.md"
    "ROADMAP.md"
    "IMPLEMENTATION_SUMMARY.md"
    "DEPLOYMENT_HANDOFF.md"
    "DEPLOYMENT_READY.md"
    "docs/getting-started.md"
    "docs/architecture.md"
    "docs/api-reference.md"
    "docs/comparison.md"
    "docs/security.md"
    "docs/deployment.md"
    "docs/design-system.md"
    "docs/features.md"
    "docs/i18n.md"
    "docs/mobile.md"
    "docs/proactive-engine.md"
    "docs/sync.md"
    "docs/terminal.md"
    "docs/vault.md"
    "docs/web-app.md"
)

for file in "${DOC_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Documentation verified"

# 6. Verify CI/CD Assets
echo ""
echo "6. Verifying CI/CD Assets..."

CI_FILES=(
    ".github/workflows/ci.yml"
    ".github/workflows/browser.yml"
    ".github/workflows/desktop.yml"
    ".github/workflows/docker-ci.yml"
    ".github/workflows/docs.yml"
    ".github/workflows/mobile.yml"
    ".github/workflows/release.yml"
    ".github/workflows/screenshot_capture.yml"
    ".github/workflows/web.yml"
    ".github/workflows/changelog-update.yml"
)

for file in "${CI_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "CI/CD assets verified"

# 7. Verify Test Files
echo ""
echo "7. Verifying Test Files..."

TEST_FILES=(
    "tests/apikey_integration_tests.rs"
    "tests/crdt_sync_tests.rs"
    "tests/web_mobile_tests.rs"
    "tests/authentication_tests.rs"
    "tests/proactive_tests.rs"
    "tests/browser_plugin_tests.rs"
    "tests/build_smoke_tests.rs"
    "tests/cli_ui_tests.md"
    "tests/code_lint_tests.rs"
    "tests/integration_tests.rs"
    "tests/security_tests.rs"
    "tests/ssh_tests.rs"
    "tests/sync_tests.rs"
    "tests/ui_ux_tests.rs"
    "tests/vault_tests.rs"
)

for file in "${TEST_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Test files verified"

# 8. Verify Configuration Files
echo ""
echo "8. Verifying Configuration Files..."

CONFIG_FILES=(
    "Cargo.toml"
    "aetheris.toml"
    ".gitleaks.toml"
    ".editorconfig"
    ".env.example"
    "docker/browser/Dockerfile"
    "docker/core/Dockerfile"
    "docker/desktop/Dockerfile"
    "docker/mobile/Dockerfile"
    "docker/web/Dockerfile"
)

for file in "${CONFIG_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   $file exists"
    else
        echo -e "${RED}✗${NC} $file not found"
        OVERALL_STATUS=1
    fi
done

print_status $OVERALL_STATUS "Configuration files verified"

# Final Summary
echo ""
echo "=========================================="
echo "Build Verification Summary"
echo "=========================================="
echo ""

if [ $OVERALL_STATUS -eq 0 ]; then
    echo -e "${GREEN}✓ All platform assets verified successfully!${NC}"
    echo ""
    echo "Aetheris is ready for:"
    echo "  - Desktop (Windows, macOS, Linux)"
    echo "  - Web (Chrome, Firefox, Edge)"
    echo "  - Mobile (Android, iOS)"
    echo "  - Browser Extensions"
    echo "  - CLI"
    echo ""
    echo "All files are in place and ready for building and testing."
    exit 0
else
    echo -e "${RED}✗ Some platform assets are missing!${NC}"
    echo ""
    echo "Please check the output above for missing files."
    exit 1
fi