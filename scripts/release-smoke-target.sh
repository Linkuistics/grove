#!/usr/bin/env bash
#
# One release target's installed smoke test, run inside that target's own
# environment. scripts/release-smoke.sh prepares the environment and runs this
# there, with no Bun or Node on PATH; it is not an entry point of its own.
#
#   release-smoke-target.sh TARGET ARCHIVE CASES [PROBES]
#
# It first asserts that it runs where TARGET runs: the system and machine, and
# on Linux a glibc of exactly 2.17 that is seen to refuse a newer binary (the
# probes release-smoke.sh builds into PROBES). Then it extracts ARCHIVE, named
# grove-v<version>-<target>.tar.xz, into a fresh prefix with this environment's
# own tar, checks that grove, grove-llm and harness-dispatch report that
# version, and hands the prefix to harness-dispatch's installed-layout cases,
# CASES (crates/harness-dispatch/scripts/installed-smoke.sh). Like those cases,
# it needs only bash 3.2, coreutils, grep and cmp, because macOS's /bin/bash
# and CentOS 7's both run it.

set -euo pipefail
IFS=$'\n\t'

# shellcheck source=scripts/release-smoke-environment.sh
source "$(dirname "${BASH_SOURCE[0]}")/release-smoke-environment.sh"

main() {
  (($# == 3 || $# == 4)) || {
    echo "usage: release-smoke-target.sh TARGET ARCHIVE CASES [PROBES]" >&2
    exit 1
  }
  TARGET="$1"
  local archive="$2" cases="$3" probes="${4:-}"
  local system
  system="$(target_system "$TARGET")"
  [[ "$(uname -sm)" == "$system" ]] || fail "this is $(uname -sm), not $system, where this target runs"
  case "$TARGET" in
    *-linux-gnu)
      [[ -n "$probes" ]] || fail "a Linux target needs the glibc probes"
      assert_glibc_floor "$probes"
      ;;
    *-apple-darwin)
      note "$(uname -srm), macOS $(sw_vers -productVersion)"
      ;;
  esac

  local scratch
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$scratch'" EXIT
  mkdir "$scratch/prefix" "$scratch/home" "$scratch/tmp"
  # Nothing outside the scratch directory: no personal configuration, and the
  # cases' own temporary files where the trap removes them.
  export HOME="$scratch/home" TMPDIR="$scratch/tmp"

  local top version command reported
  top="$(basename "$archive" .tar.xz)"
  version="${top#grove-v}"
  version="${version%-"$TARGET"}"
  [[ "$top" == "grove-v$version-$TARGET" && -n "$version" ]] ||
    fail "$archive is not named grove-v<version>-$TARGET.tar.xz"
  tar -xJf "$archive" -C "$scratch/prefix" || fail "$(tar --version | head -n 1) cannot extract $archive"
  [[ "$(ls -A "$scratch/prefix")" == "$top" ]] ||
    fail "$archive does not unpack to exactly $top/: $(ls -A "$scratch/prefix")"
  for command in grove grove-llm harness-dispatch; do
    reported="$("$scratch/prefix/$top/bin/$command" --version)" || fail "bin/$command --version exited $?"
    [[ "$reported" == "$command $version" ]] || fail "bin/$command reports '$reported', not '$command $version'"
  done
  note "extracted with $(tar --version | head -n 1); grove, grove-llm and harness-dispatch report $version"
  "$BASH" "$cases" "$scratch/prefix/$top" "$version"
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
