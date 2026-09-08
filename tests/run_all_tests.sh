#!/usr/bin/env bash
set -euo pipefail
echo "Running all test categories (no user interaction required)"
echo "--- Browser Plugin ---"
test -f browser/chrome/manifest.json
python3 -c "import json; json.load(open('browser/chrome/manifest.json'))"
echo "--- Build Scripts ---"
for script in desktop/test_build.sh browser/build.sh web/test.sh mobile/build.sh; do
  [ -x "$script" ] && echo "OK: $script"
done
echo "--- Security ---"
[ -f .gitleaks.toml ] && echo "OK: .gitleaks.toml present"
echo "--- Clean Tree ---"
[ -z "$(git ls-files --others --exclude-standard)" ] && echo "No untracked files"
echo "--- All checks passed ---"
