#!/usr/bin/env bash
#
# Verify the prerequisites for running the release pipeline. Installs
# nothing; the one thing it does change is its own PATH, pinning the Rust
# toolchain exactly as release-build.sh does (see release-common.sh) so
# that what it checks is what the build will get. Emits a punch list of
# every missing item with its remediation command, then exits non-zero if
# anything is missing.
#
# Designed to run twice: standalone before committing to a release
# attempt, and (cheaply) as the first step of release-build.sh so the
# build fails fast on a misconfigured machine instead of mid-toolchain.

set -euo pipefail
IFS=$'\n\t'
trap 'echo "release-doctor: error on line $LINENO" >&2' ERR

readonly REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# TARGETS, pin_rust_toolchain, resolved_rustc, target_std_installed.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# Pin the toolchain before checking anything, so every check below sees the
# same PATH release-build.sh will build under. Standalone this is the fix; run
# from release-build.sh it is a no-op, because that script pinned already and
# exported PATH.
pin_rust_toolchain

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

note() {
  echo "      note: $*"
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

# Set by check_rust_toolchain, consumed by check_target_std. Empty means the
# toolchain check failed and the target check has nothing to interrogate.
rustc_bin=""
rustc_is_rustup=0

# Report the toolchain the BUILD will use, not the one rustup reports.
#
# This distinction is the whole point of the check. `rustup target list
# --installed` describes rustup's toolchain; `cargo build` uses whatever cargo
# and rustc resolve to on PATH, which on a machine with Homebrew's `rust`
# formula is a different toolchain entirely, with a different set of installed
# target stds. Asking the wrong tool is how this doctor printed "all
# prerequisites met" and the build then died on a missing `std`.
check_rust_toolchain() {
  local cargo_bin shim_dir
  cargo_bin="$(command -v cargo || true)"
  rustc_bin="$(resolved_rustc)"
  shim_dir="$(rustup_shim_dir)"

  if [[ -z "$cargo_bin" || -z "$rustc_bin" ]]; then
    mark_fail "rust toolchain: cargo and/or rustc not on PATH"
    remediation "install rustup from https://rustup.rs/"
    rustc_bin=""
    return
  fi

  mark_pass "cargo: $cargo_bin ($("$cargo_bin" --version 2>/dev/null | cut -d' ' -f1-2))"
  mark_pass "rustc: $rustc_bin ($("$rustc_bin" --version 2>/dev/null | cut -d' ' -f1-2))"

  # Informational, never a failure: a non-rustup toolchain carrying all three
  # target stds builds a release perfectly well, and check_target_std is the
  # gate that says so. This line exists to sharpen the remediation when that
  # gate does fail.
  if [[ "$rustc_bin" == "$shim_dir/"* ]]; then
    rustc_is_rustup=1
  fi
}

# The load-bearing check: does the resolved toolchain actually carry a std for
# every target the build will ask for?
check_target_std() {
  if [[ -z "$rustc_bin" ]]; then
    return
  fi
  local target
  for target in "${TARGETS[@]}"; do
    if target_std_installed "$rustc_bin" "$target"; then
      mark_pass "target std: $target"
    else
      mark_fail "target std: $target has no std in the rustc above"
      remediation "rustup target add $target"
      if (( rustc_is_rustup == 0 )); then
        note "no rustup shim at $(rustup_shim_dir), so the build falls back to"
        note "the rustc above — any target rustup has already installed is"
        note "invisible to it. Install rustup, or set CARGO_HOME to where it is."
      fi
    fi
  done
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

  check_rust_toolchain
  check_target_std
  check_zig
  check_cargo_zigbuild
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
