#!/usr/bin/env bash
#
# The recorded regeneration command for this directory's figures: every `.d2`
# source renders to the `.svg` beside it. Pass an output directory to render
# elsewhere, which is how `task figures:check` compares without writing here.

set -euo pipefail

readonly PINNED_D2='v0.9.0'

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
out="${1:-$here}"

actual="$(d2 --version)"
if [[ "$actual" != "$PINNED_D2" ]]; then
  echo "regenerate: needs D2 $PINNED_D2, found $actual" >&2
  exit 1
fi

for src in "$here"/*.d2; do
  name="$(basename "$src" .d2)"
  d2 --layout=elk --theme=0 --dark-theme=200 --pad=24 "$src" "$out/$name.svg"
done
