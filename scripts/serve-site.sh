#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PORT="${1:-8000}"

cd "$ROOT_DIR/site"

echo "Serving static site at http://127.0.0.1:${PORT}/"
python3 -m http.server "$PORT"
