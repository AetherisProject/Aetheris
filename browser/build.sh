#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
echo "Browser extension build"
mkdir -p dist
zip -r dist/aetheris-browser.zip chrome/ -x '*.DS_Store'
echo "Browser OK"
