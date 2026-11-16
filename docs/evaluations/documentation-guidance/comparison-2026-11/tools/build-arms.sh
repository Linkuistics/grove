#!/usr/bin/env bash
# Export each arm's six plugins at the revisions in arms.tsv.
# Usage: build-arms.sh REPOS_DIR OUT_DIR
set -euo pipefail
repos=$1 out=$2
here=$(cd "$(dirname "$0")" && pwd)
rm -rf "$out"
grep -v '^#' "$here/arms.tsv" | while IFS=$'\t' read -r arm plugin repo rev; do
  dest="$out/$arm/$plugin"
  mkdir -p "$dest"
  git -C "$repos/$repo" archive "$rev" "plugins/$plugin" | tar -x -C "$dest" --strip-components=2
done
