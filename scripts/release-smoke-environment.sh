#!/usr/bin/env bash
# Shared in-target system and glibc-floor assertions; sourced, never executed.
fail() {
  echo "release-smoke: $TARGET: FAIL: $*" >&2
  exit 1
}

note() {
  echo "release-smoke: $TARGET: $*"
}

# `uname -sm` where TARGET's executables run.
target_system() {
  case "$1" in
    aarch64-apple-darwin) echo "Darwin arm64" ;;
    aarch64-unknown-linux-gnu) echo "Linux aarch64" ;;
    x86_64-unknown-linux-gnu) echo "Linux x86_64" ;;
    *) fail "no environment is known for this target" ;;
  esac
}

# The C library must be the floor, and must be seen to enforce it: the probe
# built against glibc 2.17 runs, and its twin, which needs getrandom's
# GLIBC_2.25 symbol version, is refused for that version. The twin's refusal is
# evidence only beside the floor probe's success, which shows this userland
# runs binaries of its architecture at all.
assert_glibc_floor() {
  local probes="$1" machine libc output status=0
  machine="$(uname -m)"
  libc="$(getconf GNU_LIBC_VERSION)" || fail "getconf reports no GNU C library"
  [[ "$libc" == "glibc 2.17" ]] || fail "this userland's C library is $libc, not the glibc 2.17 floor"
  output="$("$probes/floor-$machine" 2>&1)" ||
    fail "the probe built against glibc 2.17 did not run here: $output"
  [[ "$output" == "glibc probe ran" ]] || fail "the probe built against glibc 2.17 printed: $output"
  output="$("$probes/above-$machine" 2>&1)" || status=$?
  if ((status == 0)) || [[ "$output" != *"version \`GLIBC_2.25' not found"* ]]; then
    fail "positive control: the probe built against glibc 2.25 was not refused for its symbol version (exit $status): $output"
  fi
  note "$libc; control: the glibc 2.25 probe was refused (exit $status): $output"
}
