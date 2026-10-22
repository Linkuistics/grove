#!/usr/bin/env bash
# Exercise combined ELF admission with the actual independently built archives.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-smoke.sh
source "$root/scripts/release-smoke.sh"
# shellcheck source=scripts/release-smoke-qemu.sh
source "$root/scripts/release-smoke-qemu.sh"
[[ $# == 2 ]] || die 'usage: release-registration.test.sh grove-archives ravel-archives'
SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
python3 "$root/scripts/ravel_target_inputs.py" --archives "$2" --output "$SCRATCH/ravel" \
  aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
check_registration() (assert_archive_matches "$@")
refused() {
  local expected="$1"
  shift
  if "$@" >"$SCRATCH/refusal" 2>&1; then die "control accepted $expected"; fi
  grep -Fq "$expected" "$SCRATCH/refusal" || die "wrong refusal: $(cat "$SCRATCH/refusal")"
}
for TARGET in aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu; do
  archive="$(find_archive "$1" "$TARGET")"
  assert_archive "$archive" "$(basename "$archive" .tar.xz)"
  archive_elf_headers "$archive" "$SCRATCH/grove-headers"
  archive_elf_headers "$SCRATCH/ravel/archives/ravel-v1.1.0-$TARGET.tar.xz" "$SCRATCH/ravel-headers"
  cat "$SCRATCH/grove-headers" "$SCRATCH/ravel-headers" >"$SCRATCH/headers"
  { read -r magic && read -r mask && read -r _limit; } < <(guest "${TARGET%%-*}")
  check_registration "$SCRATCH/headers" "$magic" "$mask"
  for pattern in '/bin/grove-llm$' '/libexec/ravel/ravel-policy$'; do
    grep -v "$pattern" "$SCRATCH/headers" >"$SCRATCH/missing"
    refused 'lacks the complete Ravel pair or lists an incomplete Grove pair' \
      check_registration "$SCRATCH/missing" "$magic" "$mask"
  done
  # Altering ELF padding lets a guest escape binfmt's mask despite correct CPU.
  sed 's/^7f454c460201010000/7f454c460201010001/' "$SCRATCH/headers" >"$SCRATCH/escape"
  refused 'escape the CPU model' check_registration "$SCRATCH/escape" "$magic" "$mask"
  printf 'registration-test: %s both projects and missing/escape controls passed\n' "$TARGET"
done
