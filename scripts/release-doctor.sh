#!/usr/bin/env bash
#
# Verify the prerequisites for running the release pipeline. Checks
# only — never installs anything. Emits a punch list of every missing
# item with the README's remediation command, then exits non-zero if
# anything is missing.
#
# Designed to run twice: standalone before committing to a release
# attempt, and (cheaply) as the first step of release-build.sh so the
# build fails fast on a misconfigured machine instead of mid-toolchain.

set -euo pipefail
IFS=$'\n\t'
trap 'echo "release-doctor: error on line $LINENO" >&2' ERR

readonly REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Must stay in sync with release-build.sh's TARGETS array. The native
# host target is included even though rustup typically auto-installs it
# — explicit checks survive a partial rustup setup.
readonly TARGETS=(
  aarch64-apple-darwin
  aarch64-unknown-linux-gnu
  x86_64-unknown-linux-gnu
)

failed=0

mark_pass() {
  echo "  ✓ $*"
}

mark_fail() {
  echo "  ✗ $*"
  failed=1
}

remediation() {
  echo "      remediation: $*"
}

check_zig() {
  if ! command -v zig >/dev/null 2>&1; then
    mark_fail "zig: not on PATH"
    remediation "brew install zig"
    return
  fi
  local version
  version="$(zig version 2>/dev/null || echo unknown)"
  mark_pass "zig: $version"
}

check_cargo_zigbuild() {
  if cargo zigbuild --help >/dev/null 2>&1; then
    mark_pass "cargo-zigbuild: available"
  else
    mark_fail "cargo-zigbuild: not installed"
    remediation "cargo install cargo-zigbuild"
  fi
}

check_rustup_targets() {
  if ! command -v rustup >/dev/null 2>&1; then
    mark_fail "rustup: not on PATH"
    remediation "install rustup from https://rustup.rs/"
    return
  fi
  local installed
  installed="$(rustup target list --installed)"
  local target
  for target in "${TARGETS[@]}"; do
    if grep -qx "$target" <<<"$installed"; then
      mark_pass "rustup target: $target"
    else
      mark_fail "rustup target: $target not installed"
      remediation "rustup target add $target"
    fi
  done
}

check_rmux_daemon_build() {
  # ADR-0030: grove bundles a from-source stock rmux daemon/CLI, built at the
  # exact version grove's rmux-sdk resolves to in Cargo.lock (lockstep publish).
  # Derive that version and confirm a published rmux crate exists at it, so a
  # caret bump of rmux-sdk ahead of a matching rmux binary fails the punch list
  # HERE rather than mid-build. rmux has no openssl/curl (its `web` feature
  # gates only pure-Rust crypto, and release-build drops it with
  # --no-default-features), so the zig/zigbuild/rustup-targets checked above
  # cover the cross-build with no new prerequisite.
  local version
  version="$(awk '
    /^\[\[package\]\]/ { name = "" }
    /^name = / { name = $3 }
    name == "\"rmux-sdk\"" && /^version = / { gsub(/"/, "", $3); print $3; exit }
  ' "$REPO_ROOT/Cargo.lock")"

  if [[ -z "$version" ]]; then
    mark_fail "rmux daemon: could not read rmux-sdk version from Cargo.lock"
    remediation "ensure rmux-sdk is a resolved dependency (cargo build) so Cargo.lock pins it"
    return
  fi

  if ! command -v curl >/dev/null 2>&1; then
    mark_fail "rmux daemon: curl not on PATH (needed to verify the published rmux crate)"
    remediation "brew install curl"
    return
  fi

  # crates.io sparse index: newline-delimited JSON, one record per version.
  if curl -fsS "https://index.crates.io/rm/ux/rmux" 2>/dev/null \
       | grep -qF "\"vers\":\"${version}\""; then
    mark_pass "rmux daemon: stock rmux ${version} published (matches locked rmux-sdk)"
  else
    mark_fail "rmux daemon: no published rmux crate at ${version} (locked rmux-sdk version)"
    remediation "bump rmux-sdk to a version with a matching published rmux binary crate, or await the rmux ${version} release"
  fi
}

check_gh_auth() {
  if ! command -v gh >/dev/null 2>&1; then
    mark_fail "gh: not installed"
    remediation "brew install gh && gh auth login"
    return
  fi
  if gh auth status >/dev/null 2>&1; then
    mark_pass "gh: authenticated"
  else
    mark_fail "gh: not authenticated"
    remediation "gh auth login"
  fi
}

main() {
  echo "release-doctor: checking release prerequisites"
  echo

  check_zig
  check_cargo_zigbuild
  check_rustup_targets
  check_rmux_daemon_build
  check_gh_auth

  echo
  if (( failed == 0 )); then
    echo "release-doctor: all prerequisites met"
    exit 0
  fi
  echo "release-doctor: missing prerequisites — fix the items marked above" >&2
  exit 1
}

main "$@"
