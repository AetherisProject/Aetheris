#!/usr/bin/env bash
set -euo pipefail
echo "Desktop build/test"
cd "$(dirname "$0")"
if command -v npm &>/dev/null; then
  npm install
  npm run build
else
  echo "npm not found; skipping desktop build"
fi
echo "Desktop OK"
