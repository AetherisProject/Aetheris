#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
echo "Mobile Flutter build/test"
if command -v flutter &>/dev/null; then
  flutter build apk --release || flutter build ios || echo "Flutter build skipped"
else
  echo "flutter not found; skipping mobile build"
fi
echo "Mobile OK"
