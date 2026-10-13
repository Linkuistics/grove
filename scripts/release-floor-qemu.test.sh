#!/usr/bin/env bash
# Early admission controls must refuse without Docker, downloads or compilation.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir "$scratch/existing" "$scratch/archives" "$scratch/ravel"
for output in "$scratch/existing" "$scratch/absent/../existing"; do
  if bash "$root/scripts/release-floor-qemu-backend.sh" "$output" >"$scratch/refusal" 2>&1; then
    echo 'floor-qemu-test: accepted an existing/normalized output' >&2
    exit 1
  fi
  grep -q 'output already exists' "$scratch/refusal"
done
if bash "$root/scripts/release-floor-qemu.sh" "$scratch/archives" "$scratch/ravel" wrong-target >"$scratch/refusal" 2>&1; then
  echo 'floor-qemu-test: accepted an unsupported target' >&2
  exit 1
fi
grep -q 'unknown QEMU floor target' "$scratch/refusal"
echo 'floor-qemu-test: exclusive/normalized output and target admission refusals passed'
