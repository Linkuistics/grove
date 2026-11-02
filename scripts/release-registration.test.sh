#!/usr/bin/env bash
# Exercise combined ELF admission with the actual independently built archives.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-smoke.sh
source "$root/scripts/release-smoke.sh"
# shellcheck source=scripts/release-smoke-qemu.sh
source "$root/scripts/release-smoke-qemu.sh"
[[ $# == 2 || $# == 3 ]] || die 'usage: release-registration.test.sh grove-archives ravel-archives [ravel-2.0-archives]'
SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
python3 "$root/scripts/ravel_target_inputs.py" --archives "$2" --output "$SCRATCH/ravel" \
  aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
# The recorded 2.0 lane, whose prefix holds a front and no worker.
archives_2=()
[[ $# == 2 ]] || archives_2=(--archives "$3")
python3 "$root/scripts/ravel_target_inputs.py" --lane 2.0 "${archives_2[@]+"${archives_2[@]}"}" \
  --output "$SCRATCH/ravel-2.0" aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
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
  archive_elf_headers "$(find "$SCRATCH/ravel-2.0/archives" -name "ravel-v*-$TARGET.tar.xz")" \
    "$SCRATCH/ravel-2.0-headers" 2.0
  cat "$SCRATCH/grove-headers" "$SCRATCH/ravel-headers" >"$SCRATCH/headers"
  { read -r magic && read -r mask && read -r _limit; } < <(guest "${TARGET%%-*}")
  check_registration "$SCRATCH/headers" "$magic" "$mask"
  for pattern in '/bin/grove-llm$:incomplete Grove pair' '/libexec/ravel/ravel-policy$:incomplete Ravel 1.1 pair'; do
    grep -v "${pattern%%:*}" "$SCRATCH/headers" >"$SCRATCH/missing"
    refused "${pattern#*:}" check_registration "$SCRATCH/missing" "$magic" "$mask"
  done
  # Both lanes beside Grove, and the 2.0 lane alone, as the target smoke runs it.
  cat "$SCRATCH/headers" "$SCRATCH/ravel-2.0-headers" >"$SCRATCH/both"
  check_registration "$SCRATCH/both" "$magic" "$mask"
  check_registration "$SCRATCH/ravel-2.0-headers" "$magic" "$mask"
  # A 2.0 prefix with a 1.x worker, and a 2.0 front read as a 1.1 pair.
  grep '/libexec/ravel/ravel-policy$' "$SCRATCH/ravel-headers" |
    awk -F '\t' -v OFS='\t' '{ $2 = "ravel-2.0/" $2; print }' |
    cat "$SCRATCH/ravel-2.0-headers" - >"$SCRATCH/worker-2"
  refused 'a Ravel 2.0 prefix with a 1.x worker' check_registration "$SCRATCH/worker-2" "$magic" "$mask"
  awk -F '\t' -v OFS='\t' '{ sub(/^ravel-2\.0\//, "", $2); print }' \
    "$SCRATCH/ravel-2.0-headers" >"$SCRATCH/unlabelled-2"
  refused 'incomplete Ravel 1.1 pair' check_registration "$SCRATCH/unlabelled-2" "$magic" "$mask"
  # Altering ELF padding lets a guest escape binfmt's mask despite correct CPU.
  sed 's/^7f454c460201010000/7f454c460201010001/' "$SCRATCH/headers" >"$SCRATCH/escape"
  refused 'escape the CPU model' check_registration "$SCRATCH/escape" "$magic" "$mask"
  printf 'registration-test: %s both projects, both Ravel lanes and missing/escape/lane controls passed\n' "$TARGET"
done
