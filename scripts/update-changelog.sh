#!/usr/bin/env bash
set -euo pipefail
# Auto-update CHANGELOG.md with latest CI status
commit=$(git rev-parse --short HEAD)
msg=$(git log -1 --pretty=%B)
status=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/merlin-tribukait/Aetheris/actions/runs?head_sha=$commit" | \
  python3 -c "import sys,json; d=json.load(sys.stdin); runs=d.get('workflow_runs',[]); print('green' if runs and runs[0]['conclusion']=='success' else 'pending')" 2>/dev/null || echo "unknown")
echo "[$(date -u +%Y-%m-%d)] Commit $commit — CI: $status — $msg" >> .ci-log.md
echo "Updated: $commit / $status"
