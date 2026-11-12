#!/usr/bin/env bash
# Real CLI controls use pinned public archives; no archive authenticity seam.
# ARCHIVES holds the 1.1 lane's published archives. ARCHIVES_2 holds the
# recorded archives of the 2.0 lane (default target/ravel-archives-2.0).
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
archives="${1:?usage: ravel-target-inputs.test.sh ARCHIVES [ARCHIVES_2]}"
archives_2="${2:-$repo_root/target/ravel-archives-2.0}"
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
[[ -x "$scratch/valid/aarch64-apple-darwin/ravel-v1.1.0-aarch64-apple-darwin/bin/ravel" ]] || fail 'no macOS prefix'
refused 'output already exists' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$archives" --output "$scratch/valid"
mkdir "$scratch/missing" "$scratch/tampered" "$scratch/wrong"
refused 'missing archive' python3 "$repo_root/scripts/ravel_target_inputs.py" \
  --archives "$scratch/missing" --output "$scratch/missing-layout"
mac=ravel-v1.1.0-aarch64-apple-darwin.tar.xz
linux=ravel-v1.1.0-aarch64-unknown-linux-gnu.tar.xz
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
  linux_prefix="$scratch/valid/aarch64-unknown-linux-gnu/ravel-v1.1.0-aarch64-unknown-linux-gnu"
  for program in bin/ravel libexec/ravel/ravel-policy; do
    header="$(od -An -tx1 -N20 "$linux_prefix/$program" | tr -d ' \n')"
    printf '%s\t%s\n' "$header" "ravel-v1.1.0-aarch64-unknown-linux-gnu/$program"
  done >"$headers"
  { read -r magic && read -r mask && read -r limit; } < <(guest aarch64)
  check_registration "$headers" "$magic" "$mask"
  { read -r magic && read -r mask && read -r limit; } < <(guest x86_64)
  refused 'escape the CPU model' check_registration "$headers" "$magic" "$mask"
  sed 's|libexec/ravel/ravel-policy|libexec/harness-dispatch/harness-dispatch-policy|' "$headers" >"$scratch/mixed"
  { read -r magic && read -r mask && read -r limit; } < <(guest aarch64)
  refused 'incomplete Ravel 1.1 pair' check_registration "$scratch/mixed" "$magic" "$mask"
)
printf 'ravel-inputs-test: cache and QEMU-registration controls passed\n'

# The 2.0 lane admits only its recorded archives. Each lane must refuse the
# other's bytes by digest, even under its own archive name.
inputs() { python3 "$repo_root/scripts/ravel_target_inputs.py" "$@"; }
recorded=()
recorded_names=()
unrecorded=()
while IFS=$'\t' read -r target state detail; do
  if [[ "$state" == recorded ]]; then
    recorded+=("$target")
    recorded_names+=("$detail")
  else
    unrecorded+=("$target")
  fi
done < <(inputs --lane 2.0 --status)
((${#recorded[@]} > 0)) || fail 'the 2.0 lane records no archive'
inputs --lane 2.0 --archives "$archives_2" --output "$scratch/lane-2" "${recorded[@]}"
for target in "${recorded[@]}"; do
  prefix="$(find "$scratch/lane-2/$target" -mindepth 1 -maxdepth 1 -type d)"
  [[ -x "$prefix/bin/ravel" && -f "$prefix/share/ravel/examples/sample.rhai" ]] || fail "no 2.0 prefix for $target"
  [[ ! -e "$prefix/libexec/ravel" ]] || fail "the 2.0 prefix for $target holds a 1.x worker"
done
mkdir "$scratch/swapped-2" "$scratch/swapped-1"
# One changed byte, in the middle of each recorded archive, Linux included.
for index in "${!recorded[@]}"; do
  target="${recorded[$index]}"
  name="${recorded_names[$index]}"
  mkdir "$scratch/changed-2-$target"
  python3 - "$archives_2/$name" "$scratch/changed-2-$target/$name" <<'PY'
import sys
from pathlib import Path
data = bytearray(Path(sys.argv[1]).read_bytes())
data[len(data) // 2] ^= 0x01
Path(sys.argv[2]).write_bytes(bytes(data))
PY
  refused 'SHA-256' inputs --lane 2.0 --archives "$scratch/changed-2-$target" \
    --output "$scratch/changed-2-$target-layout" "$target"
  [[ ! -e "$scratch/changed-2-$target-layout" ]] || fail "exposed a changed $target archive"
done
name_1="ravel-v1.1.0-${recorded[0]}.tar.xz"
name_2="${recorded_names[0]}"
cp "$archives/$name_1" "$scratch/swapped-2/$name_2"
refused 'SHA-256' inputs --lane 2.0 --archives "$scratch/swapped-2" --output "$scratch/swapped-2-layout" "${recorded[0]}"
cp "$archives_2/$name_2" "$scratch/swapped-1/$name_1"
refused 'SHA-256' inputs --archives "$scratch/swapped-1" --output "$scratch/swapped-1-layout" "${recorded[0]}"
for output in swapped-2-layout swapped-1-layout; do
  [[ ! -e "$scratch/$output" ]] || fail "exposed refused $output"
done
# The unit cases of scripts/test_ravel_target_inputs.py refuse an unrecorded
# target with a synthetic lane. This control uses the real record while it
# still leaves a target unrecorded.
if ((${#unrecorded[@]} > 0)); then
  refused 'records no archive' inputs --lane 2.0 --archives "$archives_2" --output "$scratch/unrecorded-layout"
  [[ ! -e "$scratch/unrecorded-layout" ]] || fail 'exposed a layout for an unrecorded target'
else
  printf 'ravel-inputs-test: the 2.0 lane records every target, so no real unrecorded target exists\n'
fi
for target in "${recorded[@]}"; do
  refused 'SHA-256' bash "$repo_root/scripts/ravel-target-smoke.sh" \
    --archives-2.0 "$scratch/changed-2-$target" "$target"
done
printf 'ravel-inputs-test: 2.0 lane admission, changed-byte, cross-lane and unrecorded-target controls passed\n'
