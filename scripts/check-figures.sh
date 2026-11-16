#!/usr/bin/env bash
#
# Checks that committed figures regenerate from their sources. Each figure
# directory holds a `regenerate.sh`, the recorded regeneration command, which
# takes an output directory. The check renders into a scratch directory and
# compares bytes with every committed `.svg`, in both directions. A stale SVG,
# a source with no committed SVG, and an SVG that no source produces all fail.
#
# With no argument, the check finds every directory of the repository that
# holds a `regenerate.sh`. A repository that gains committed rendered figures
# is then covered without a change to this script or to `task figures:check`.
#
# A byte comparison shows that the committed SVG matches its source under the
# pinned tool. It does not show that the figure renders where its reader reads
# it. That is a separate check, made in the declared viewing environment.

set -euo pipefail

if [[ $# -eq 0 ]]; then
  root="$(cd "$(dirname "$0")/.." && pwd)"
  dirs=()
  while IFS= read -r recipe; do
    dirs+=("$(dirname "$recipe")")
  done < <(find "$root" \( -name .git -o -name .jj -o -name target -o -name node_modules \) -prune \
    -o -type f -name regenerate.sh -print | LC_ALL=C sort)
  if [[ ${#dirs[@]} -eq 0 ]]; then
    echo "figures: no directory holds a regenerate.sh, so no committed figure needs a check"
    exit 0
  fi
  set -- "${dirs[@]}"
fi

failed=0
for dir in "$@"; do
  if [[ ! -f "$dir/regenerate.sh" ]]; then
    echo "figures: $dir: holds no regenerate.sh" >&2
    failed=1
    continue
  fi
  scratch="$(mktemp -d)"
  if ! bash "$dir/regenerate.sh" "$scratch" >/dev/null 2>"$scratch.log"; then
    cat "$scratch.log" >&2
    echo "figures: $dir: regeneration failed" >&2
    failed=1
    rm -rf "$scratch" "$scratch.log"
    continue
  fi
  for svg in "$scratch"/*.svg; do
    [[ -e "$svg" ]] || continue
    name="$(basename "$svg")"
    if [[ ! -f "$dir/$name" ]]; then
      echo "figures: $dir/$name: generated but not committed" >&2
      failed=1
    elif ! cmp -s "$svg" "$dir/$name"; then
      echo "figures: $dir/$name: differs from its regenerated source" >&2
      failed=1
    fi
  done
  for svg in "$dir"/*.svg; do
    [[ -e "$svg" ]] || continue
    name="$(basename "$svg")"
    if [[ ! -f "$scratch/$name" ]]; then
      echo "figures: $dir/$name: no source regenerates it" >&2
      failed=1
    fi
  done
  rm -rf "$scratch" "$scratch.log"
done

if [[ $failed -ne 0 ]]; then
  exit 1
fi
echo "figures: $# director$([[ $# -eq 1 ]] && echo y || echo ies) regenerate byte-identically"
