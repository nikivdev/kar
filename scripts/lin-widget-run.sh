#!/usr/bin/env bash
set -euo pipefail

ts_path="${1:-}"
if [ -z "$ts_path" ]; then
  echo "lin-widget-run: missing ts path" >&2
  exit 1
fi

# Expand ~ if present.
if [[ "$ts_path" == "~/"* ]]; then
  ts_path="$HOME${ts_path:1}"
fi

log_file="$HOME/Library/Logs/Lin/widget.log"
mkdir -p "$(dirname "$log_file")"
printf '[%s] linWidget trigger path=%s\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$ts_path" >> "$log_file"

sender="$HOME/config/i/kar/scripts/lin-widget-send.mjs"

# Ensure common paths are available for karabiner's restricted shell environment.
export PATH="$HOME/.bun/bin:$HOME/.npm-global/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:$PATH"

if command -v bun >/dev/null 2>&1; then
  bun run "$ts_path"
elif command -v tsx >/dev/null 2>&1; then
  tsx "$ts_path"
elif command -v npx >/dev/null 2>&1; then
  npx --yes tsx "$ts_path"
else
  echo "linWidget: install bun or tsx" >&2
  exit 1
fi | {
  if command -v node >/dev/null 2>&1; then
    node "$sender"
  elif command -v bun >/dev/null 2>&1; then
    bun run "$sender"
  else
    echo "linWidget: install node or bun" >&2
    exit 1
  fi
} 2>> "$log_file"
