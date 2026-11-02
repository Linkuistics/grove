#!/usr/bin/env bash
# Called only with the admitted private archive, staged read-only by the host.
set -euo pipefail
IFS=$'\n\t'
# The Linux mount uses canonical filenames so existing QEMU mechanics stay shared.
# shellcheck source=scripts/release-smoke-environment.sh
source "$(dirname "${BASH_SOURCE[0]}")/release-smoke-environment.sh"

ravel_target_main() {
  (($# == 3 || $# == 4)) || fail 'usage: ravel-target-smoke-target.sh TARGET ARCHIVE CASES [PROBES]'
  TARGET="$1"
  local archive="$2" cases="$3" probes="${4:-}" scratch top
  [[ "$(uname -sm)" == "$(target_system "$TARGET")" ]] || fail "wrong target environment: $(uname -sm)"
  case "$TARGET" in
    *-linux-gnu)
      [[ -n "$probes" ]] || fail 'Linux needs glibc probes'
      assert_glibc_floor "$probes"
      ;;
    *-apple-darwin) note "$(uname -srm), macOS $(sw_vers -productVersion)" ;;
  esac
  # The host admitted this archive against its lane's record. Its name carries
  # the version that the cases expect the front to report.
  top="$(basename "$archive" .tar.xz)"
  local version="${top#ravel-v}"
  version="${version%-"$TARGET"}"
  [[ "$top" == "ravel-v$version-$TARGET" && -n "$version" && "$archive" == *.tar.xz ]] ||
    fail "wrong recorded archive name: $(basename "$archive")"
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # capture this function-local path before it returns
  trap "rm -rf '$scratch'" EXIT
  mkdir "$scratch/prefix" "$scratch/home" "$scratch/tmp"
  # HOME/TMPDIR are confined to the test subprocess, never the enclosing session.
  export HOME="$scratch/home" TMPDIR="$scratch/tmp"
  tar -xJf "$archive" -C "$scratch/prefix" || fail 'cannot extract validated archive'
  [[ "$(ls -A "$scratch/prefix")" == "$top" ]] || fail 'unexpected extracted top level'
  "$BASH" "$cases" "$scratch/prefix/$top" "$version"
}
ravel_target_main "$@"
