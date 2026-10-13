#!/usr/bin/env bash
# Real CLI controls use pinned public archives; no archive authenticity seam.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
archives="${1:?usage: ravel-target-inputs.test.sh ARCHIVES}"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
fail() {
  echo "ravel-inputs-test: $*" >&2
  exit 1
}
refused() {
  local expected="$1"
  shift
  if "$@" >"$scratch/refused.log" 2>&1; then fail "accepted $expected"; fi
  cat "$scratch/refused.log"
  grep -Fq "$expected" "$scratch/refused.log" || fail "wrong refusal for $expected"
}
# A full, independently versioned release must prepare all three layouts.
python3 "$repo_root/scripts/ravel_target_inputs.py" --archives "$archives" --output "$scratch/valid"
[[ -x "$scratch/valid/aarch64-apple-darwin/ravel-v0.1.0-aarch64-apple-darwin/bin/ravel" ]] || fail 'no macOS prefix'
refused 'output already exists' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$archives" --output "$scratch/valid"
mkdir "$scratch/missing" "$scratch/tampered" "$scratch/wrong"
refused 'missing archive' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$scratch/missing" --output "$scratch/missing-layout"
mac=ravel-v0.1.0-aarch64-apple-darwin.tar.xz
linux=ravel-v0.1.0-aarch64-unknown-linux-gnu.tar.xz
cp "$archives/$mac" "$scratch/tampered/$mac"
printf 'tamper' >>"$scratch/tampered/$mac"
refused 'SHA-256' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$scratch/tampered" --output "$scratch/tampered-layout" aarch64-apple-darwin
cp "$archives/$linux" "$scratch/wrong/$mac"
refused 'SHA-256' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$scratch/wrong" --output "$scratch/wrong-layout" aarch64-apple-darwin
# One valid target followed by a missing one must not expose partial output.
cp "$archives/$mac" "$scratch/missing/$mac"
refused 'missing archive' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$scratch/missing" --output "$scratch/partial-layout"
for output in missing-layout tampered-layout wrong-layout partial-layout; do
  [[ ! -e "$scratch/$output" ]] || fail "exposed refused $output"
done
# The reusable smoke runner must also fail before any execution on wrong input.
refused 'SHA-256' bash "$repo_root/scripts/ravel-target-smoke.sh" \
  --archives "$scratch/tampered" aarch64-apple-darwin
printf 'ravel-inputs-test: real three-target layouts and all refusal controls passed\n'

# A corrupted default cache must not be silently repaired by network access.
mkdir -p "$scratch/consumer/scripts" "$scratch/consumer/target/ravel-archives" "$scratch/tools"
cp "$repo_root/scripts/ravel_target_inputs.py" "$scratch/consumer/scripts/"
cp "$scratch/tampered/$mac" "$scratch/consumer/target/ravel-archives/"
printf '%s\n' '#!/bin/sh' 'echo unexpected-download >&2; exit 91' >"$scratch/tools/curl"
chmod +x "$scratch/tools/curl"
refused 'SHA-256' env PATH="$scratch/tools:$PATH" python3 \
  "$scratch/consumer/scripts/ravel_target_inputs.py" --output "$scratch/cache-layout" aarch64-apple-darwin
[[ ! -e "$scratch/cache-layout" ]] || fail 'corrupt cache exposed a layout'

# The QEMU registration must cover both executables, and refuse another target
# or a mixture of unrelated fronts/workers. Run the actual assertion function.
(
  # shellcheck source=scripts/release-smoke-qemu.sh
  source "$repo_root/scripts/release-smoke-qemu.sh"
  TARGET=aarch64-unknown-linux-gnu
  # die exits its shell; contain each assertion so the refusal harness can read it.
  check_registration() (assert_archive_matches "$@")
  headers="$scratch/headers"
  linux_prefix="$scratch/valid/aarch64-unknown-linux-gnu/ravel-v0.1.0-aarch64-unknown-linux-gnu"
  for program in bin/ravel libexec/ravel/ravel-policy; do
    header="$(od -An -tx1 -N20 "$linux_prefix/$program" | tr -d ' \n')"
    printf '%s\t%s\n' "$header" "ravel-v0.1.0-aarch64-unknown-linux-gnu/$program"
  done >"$headers"
  { read -r magic && read -r mask && read -r limit; } < <(guest aarch64)
  check_registration "$headers" "$magic" "$mask"
  { read -r magic && read -r mask && read -r limit; } < <(guest x86_64)
  refused 'escape the CPU model' check_registration "$headers" "$magic" "$mask"
  sed 's|libexec/ravel/ravel-policy|libexec/harness-dispatch/harness-dispatch-policy|' "$headers" >"$scratch/mixed"
  { read -r magic && read -r mask && read -r limit; } < <(guest aarch64)
  refused 'no complete dispatch or Ravel pair' check_registration "$scratch/mixed" "$magic" "$mask"
)
printf 'ravel-inputs-test: cache and QEMU-registration controls passed\n'
