#!/usr/bin/env bash
# Real-file admission controls; no Docker, compiler or network needed.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fail() { echo "floor-backend-test: $*" >&2; exit 1; }
[[ -f "$repo_root/scripts/release-floor-backend.sh" ]] || fail 'backend preparation is missing'
# shellcheck source=scripts/release-floor-backend.sh
source "$repo_root/scripts/release-floor-backend.sh"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
printf 'pinned input\n' >"$scratch/input"
digest="$(sha256_of "$scratch/input")"
verify_floor_input "$scratch/input" "$digest"
cp "$scratch/input" "$scratch/mutated"
printf 'mutation\n' >>"$scratch/mutated"
for file in mutated missing; do
  if (verify_floor_input "$scratch/$file" "$digest") >"$scratch/refusal" 2>&1; then
    fail "$file input was admitted"
  fi
  grep -Fq 'refusing' "$scratch/refusal" || fail "$file did not report its input refusal"
done
if bash "$repo_root/scripts/release-floor-backend.sh" "$scratch/output" wrong-target >"$scratch/refusal" 2>&1; then
  fail 'unknown target was admitted'
fi
[[ ! -e "$scratch/output" ]] || fail 'unknown target published output'
grep -Fq 'unknown floor target' "$scratch/refusal" || fail 'unknown target refusal was not specific'
mkdir "$scratch/existing"
printf retained >"$scratch/existing/sentinel"
if bash "$repo_root/scripts/release-floor-backend.sh" "$scratch/existing" >"$scratch/refusal" 2>&1; then
  fail 'existing output was overwritten'
fi
[[ "$(cat "$scratch/existing/sentinel")" == retained ]] || fail 'existing output changed'
grep -Fq 'output already exists' "$scratch/refusal" || fail 'existing output refusal was not specific'
if bash "$repo_root/scripts/release-floor-backend.sh" "$scratch/absent/../existing" >"$scratch/refusal" 2>&1; then
  fail 'destination resolving to an existing directory was admitted'
fi
[[ ! -e "$scratch/existing/layout" ]] || fail 'publication merged into existing output'
grep -Fq 'output already exists' "$scratch/refusal" || fail 'normalized output refusal was not specific'
mkdir "$scratch/staged" "$scratch/collision"
printf payload >"$scratch/staged/file"
if python3 "$repo_root/scripts/atomic_directory_publish.py" "$scratch/staged" "$scratch/collision" >"$scratch/refusal" 2>&1; then
  fail 'atomic publication overwrote an empty destination'
fi
[[ -f "$scratch/staged/file" && ! -e "$scratch/collision/file" ]] || fail 'failed publication moved data'
python3 "$repo_root/scripts/atomic_directory_publish.py" "$scratch/staged" "$scratch/published"
[[ -f "$scratch/published/file" && ! -e "$scratch/staged" ]] || fail 'fresh publication did not rename the directory'
printf '\177ELF\002\001\001\000\000\000\000\000\000\000\000\000\002\000\267\000' >"$scratch/arm"
verify_floor_elf aarch64 "$scratch/arm"
if (verify_floor_elf x86_64 "$scratch/arm") >"$scratch/refusal" 2>&1; then
  fail 'wrong target architecture was admitted'
fi
grep -Fq 'wrong ELF' "$scratch/refusal" || fail 'architecture refusal was not specific'
cp "$scratch/arm" "$scratch/exempt"
printf '\001' | dd of="$scratch/exempt" bs=1 seek=8 count=1 conv=notrunc 2>/dev/null
if (verify_floor_elf aarch64 "$scratch/exempt") >"$scratch/refusal" 2>&1; then
  fail 'QEMU-exempt backend was admitted'
fi
grep -Fq 'wrong ELF' "$scratch/refusal" || fail 'CPU-model escape refusal was not specific'
echo 'floor-backend-test: digest, missing, target/padding, normalized destination and atomic publication controls passed'
