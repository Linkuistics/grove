#!/usr/bin/env bash
# Real-filesystem no-replace/rename controls for both host platforms.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fail() { echo "atomic-publish-test: $*" >&2; exit 1; }
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/empty" "$scratch/nonempty"
printf payload >"$scratch/source/file"
printf retained >"$scratch/nonempty/marker"
printf retained >"$scratch/file"
ln -s absent "$scratch/link"
for entry in empty nonempty file link; do
  if python3 "$repo_root/scripts/atomic_directory_publish.py" "$scratch/source" "$scratch/$entry" >"$scratch/refusal" 2>&1; then
    fail "publication overwrote $entry"
  fi
  grep -Fq 'refusing publication' "$scratch/refusal" || fail "$entry did not report refusal"
  [[ "$(cat "$scratch/source/file")" == payload ]] || fail "$entry moved source data"
done
[[ "$(cat "$scratch/nonempty/marker")" == retained && "$(cat "$scratch/file")" == retained && -L "$scratch/link" ]] ||
  fail 'publication changed existing entries'
python3 "$repo_root/scripts/atomic_directory_publish.py" "$scratch/source" "$scratch/output"
[[ "$(cat "$scratch/output/file")" == payload && ! -e "$scratch/source" ]] || fail 'fresh publication did not rename'
if python3 "$repo_root/scripts/atomic_directory_publish.py" "$scratch/missing" "$scratch/unpublished" >"$scratch/refusal" 2>&1; then
  fail 'missing source was published'
fi
[[ ! -e "$scratch/unpublished" ]] || fail 'missing source left partial output'
# Docker's /dev/shm is a different filesystem; exercise EXDEV without fallback.
if [[ "$(uname -s)" == Linux ]]; then
  cross="$(mktemp -d /dev/shm/atomic-publish.XXXXXX)"
  printf payload >"$cross/file"
  if python3 "$repo_root/scripts/atomic_directory_publish.py" "$cross" "$scratch/cross" >"$scratch/refusal" 2>&1; then
    rm -rf "$cross"
    fail 'cross-filesystem publication copied instead of refusing'
  fi
  [[ -f "$cross/file" && ! -e "$scratch/cross" ]] || fail 'cross-filesystem failure lost data or exposed output'
  grep -Fq 'cross-device' "$scratch/refusal" || fail 'cross-filesystem refusal was not EXDEV'
  rm -rf "$cross"
  echo 'atomic-publish-test: Linux cross-filesystem refusal retained source and exposed no output'
fi
echo 'atomic-publish-test: fresh rename and existing-directory/file/symlink/missing refusals passed'
