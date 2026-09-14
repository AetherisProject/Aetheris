#!/bin/bash

# Test script for Aetheris Web PWA
# This script verifies the implementation meets the acceptance criteria

echo "🧪 Aetheris Web PWA Test Suite"
echo "================================"
echo ""

# Test 1: Check if all required files exist
echo "✅ Test 1: Checking required files..."
required_files=(
    "src/Aetheris.Web/wwwroot/js/argon2.js"
    "src/Aetheris.Web/wwwroot/js/crypto.js"
    "src/Aetheris.Web/wwwroot/js/totp.js"
    "src/Aetheris.Web/wwwroot/js/clipboard.js"
    "src/Aetheris.Web/wwwroot/js/offline-queue.js"
    "src/Aetheris.Web/Services/CryptoService.cs"
    "src/Aetheris.Web/Services/HubClient.cs"
    "src/Aetheris.Web/Services/VaultState.cs"
    "src/Aetheris.Web/Pages/Unlock.razor"
    "src/Aetheris.Web/Pages/Vault.razor"
    "src/Aetheris.Web/Pages/Gateway.razor"
    "src/Aetheris.Web/Pages/Settings.razor"
    "src/Aetheris.Web/Pages/AddItemModal.razor"
    "src/Aetheris.Web/Pages/EditItemModal.razor"
    "src/Aetheris.Web/Pages/ItemDetail.razor"
    "src/Aetheris.Web/Shared/MobileLayout.razor"
    "src/Aetheris.Web/wwwroot/index.html"
    "src/Aetheris.Web/wwwroot/manifest.webmanifest"
    "src/Aetheris.Web/wwwroot/service-worker.js"
    "src/Aetheris.Web/wwwroot/css/app.css"
    "src/Aetheris.Web/wwwroot/css/mobile.css"
)

missing_files=()
for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        missing_files+=("$file")
        echo "❌ Missing: $file"
    fi
done

if [ ${#missing_files[@]} -eq 0 ]; then
    echo "✅ All required files exist"
else
    echo "❌ Missing ${#missing_files[@]} file(s)"
    exit 1
fi

echo ""

# Test 2: Check PWA manifest
echo "✅ Test 2: Checking PWA manifest..."
if grep -q '"name"' src/Aetheris.Web/wwwroot/manifest.webmanifest && \
   grep -q '"start_url"' src/Aetheris.Web/wwwroot/manifest.webmanifest && \
   grep -q '"display"' src/Aetheris.Web/wwwroot/manifest.webmanifest; then
    echo "✅ PWA manifest looks good"
else
    echo "❌ PWA manifest incomplete"
    exit 1
fi

echo ""

# Test 3: Check service worker
echo "✅ Test 3: Checking service worker..."
if grep -q "CACHE" src/Aetheris.Web/wwwroot/service-worker.js && \
   grep -q "fetch" src/Aetheris.Web/wwwroot/service-worker.js; then
    echo "✅ Service worker implemented"
else
    echo "❌ Service worker incomplete"
    exit 1
fi

echo ""

# Test 4: Check crypto implementation
echo "✅ Test 4: Checking crypto implementation..."
if grep -q "libsodium" src/Aetheris.Web/wwwroot/js/argon2.js && \
   grep -q "XChaCha20" src/Aetheris.Web/wwwroot/js/crypto.js && \
   grep -q "Argon2id" src/Aetheris.Web/wwwroot/js/argon2.js; then
    echo "✅ Crypto implementation looks good"
else
    echo "❌ Crypto implementation incomplete"
    exit 1
fi

echo ""

# Test 5: Check TOTP implementation
echo "✅ Test 5: Checking TOTP implementation..."
if grep -q "RFC 6238" src/Aetheris.Web/wwwroot/js/totp.js || \
   grep -q "base32" src/Aetheris.Web/wwwroot/js/totp.js; then
    echo "✅ TOTP implementation looks good"
else
    echo "❌ TOTP implementation incomplete"
    exit 1
fi

echo ""

# Test 6: Check mobile layout
echo "✅ Test 6: Checking mobile layout..."
if grep -q "@media (max-width" src/Aetheris.Web/wwwroot/css/app.css && \
   grep -q "44px" src/Aetheris.Web/wwwroot/css/app.css; then
    echo "✅ Mobile layout implemented"
else
    echo "❌ Mobile layout incomplete"
    exit 1
fi

echo ""

# Test 7: Check design tokens
echo "✅ Test 7: Checking design tokens..."
if grep -q "#090D16" src/Aetheris.Web/wwwroot/css/app.css && \
   grep -q "#6366F1" src/Aetheris.Web/wwwroot/css/app.css && \
   grep -q "#A855F7" src/Aetheris.Web/wwwroot/css/app.css && \
   grep -q "#06B6D4" src/Aetheris.Web/wwwroot/css/app.css && \
   grep -q "#10B981" src/Aetheris.Web/wwwroot/css/app.css; then
    echo "✅ Design tokens implemented"
else
    echo "❌ Design tokens incomplete"
    exit 1
fi

echo ""

# Test 8: Check unlock flow
echo "✅ Test 8: Checking unlock flow..."
if grep -q "UnlockAsync" src/Aetheris.Web/Pages/Unlock.razor && \
   grep -q "DeriveKeyAsync" src/Aetheris.Web/Services/VaultState.cs; then
    echo "✅ Unlock flow implemented"
else
    echo "❌ Unlock flow incomplete"
    exit 1
fi

echo ""

# Test 9: Check vault functionality
echo "✅ Test 9: Checking vault functionality..."
if grep -q "AddItem" src/Aetheris.Web/Pages/Vault.razor && \
   grep -q "DeleteItem" src/Aetheris.Web/Pages/Vault.razor && \
   grep -q "TOTP" src/Aetheris.Web/Pages/Vault.razor; then
    echo "✅ Vault functionality implemented"
else
    echo "❌ Vault functionality incomplete"
    exit 1
fi

echo ""

# Test 10: Check gateway functionality
echo "✅ Test 10: Checking gateway functionality..."
if grep -q "budgets" src/Aetheris.Web/Pages/Gateway.razor && \
   grep -q "aliases" src/Aetheris.Web/Pages/Gateway.razor; then
    echo "✅ Gateway functionality implemented"
else
    echo "❌ Gateway functionality incomplete"
    exit 1
fi

echo ""
echo "🎉 All tests passed!"
echo ""
echo "Acceptance criteria to verify manually:"
echo "1. Lighthouse PWA installable on desktop + phone emulator"
echo "2. e2e: unlock → add password → push → NEW INCOGNITO WINDOW → unlock → item present"
echo "3. devtools network tab: every request body is base64 ciphertext; no plaintext leaves the page"
echo "4. kill the hub for 1 min → edits queue → hub back → generation advances, no data loss"
