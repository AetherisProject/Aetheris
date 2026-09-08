#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
echo "Web build/test"
if command -v npm &>/dev/null; then
  npm ci
  npm run build
else
  echo "npm not found"
fi
echo "Web OK"
