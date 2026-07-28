#!/usr/bin/env bash
#
# Shared release-pipeline definitions. SOURCED, never executed:
#
#   source "$(dirname "${BASH_SOURCE[0]}")/release-common.sh"
#
# Holds the two things release-doctor.sh and release-build.sh must agree on —
# the target list, and which Rust toolchain the build gets. They used to agree
# by comment ("must stay in sync"), and on the toolchain they did not agree at
# all, which is how a green doctor shipped a dying build twice.

if [[ -n "${GROVE_RELEASE_COMMON_SOURCED:-}" ]]; then
  return 0
fi
GROVE_RELEASE_COMMON_SOURCED=1

# The single source of truth for what a release builds. The native host target
# is listed explicitly even though rustup usually auto-installs it — an explicit
# check survives a partial rustup setup.
# shellcheck disable=SC2034  # consumed by the scripts that source this file
TARGETS=(
  aarch64-apple-darwin
  aarch64-unknown-linux-gnu
  x86_64-unknown-linux-gnu
)

# Put rustup's shim directory at the FRONT of PATH, so the release build gets a
# coherent rustup toolchain rather than whatever `cargo` happens to resolve to.
#
# WHY THIS SETS RATHER THAN DIAGNOSES. The two Linux targets are rustup-managed;
# Homebrew's `rust` formula ships its own cargo *and* rustc that know nothing
# about them, and on a machine with both installed Homebrew's wins on PATH. The
# result is `error[E0463]: can't find crate for 'std'` several minutes into a
# release build. That has now cost two releases (ship-release-k25,
# observe-mid-turn-live-k31) *despite being known both times* — a remedy the
# operator must remember every time is not a remedy, so the script does it.
# The pin is announced, not silent: the doctor prints the cargo and rustc it
# settled on, and the target check below verifies the outcome rather than
# assuming it.
#
# WHY PATH AND NOT SOMETHING NARROWER. Two narrower remedies were measured and
# rejected:
#   - Resolving `CARGO="$(rustup which cargo)"` and invoking it by absolute path
#     does NOT work. cargo finds `rustc` via $RUSTC or PATH, so rustup's cargo
#     drives Homebrew's rustc and fails identically.
#   - `rustup run <toolchain> cargo …` does NOT work either. It sets only
#     $RUSTUP_TOOLCHAIN and leaves PATH untouched, so on a machine whose PATH
#     lacks the shim directory it resolves Homebrew's binaries and changes
#     nothing.
# Setting $RUSTC alone does work, but leaves a mixed toolchain (one vendor's
# cargo, another's rustc). Prepending the shim directory gives cargo, rustc and
# the cargo-* subcommands from one rustup toolchain, and honours a
# rust-toolchain.toml if this repo ever grows one.
#
# No-op when rustup is not installed; check_rust_toolchain reports the fallout.
pin_rust_toolchain() {
  local shim_dir="${CARGO_HOME:-$HOME/.cargo}/bin"
  [[ -x "$shim_dir/cargo" && -x "$shim_dir/rustc" ]] || return 0
  if [[ "$(command -v cargo || true)" != "$shim_dir/cargo" ]]; then
    PATH="$shim_dir:$PATH"
    export PATH
    hash -r 2>/dev/null || true
  fi
}

# The directory pin_rust_toolchain prefers — also what the doctor reports
# against, so "is this rustup's?" is asked in exactly one place.
rustup_shim_dir() {
  printf '%s\n' "${CARGO_HOME:-$HOME/.cargo}/bin"
}

# The rustc the build will actually compile with. cargo consults $RUSTC first
# and PATH second, so this mirrors cargo's own resolution — asking `rustup`
# instead is what made the doctor green while the build died.
resolved_rustc() {
  if [[ -n "${RUSTC:-}" ]]; then
    printf '%s\n' "$RUSTC"
  else
    command -v rustc || true
  fi
}

# True when $1 (a rustc) has a std for target $2.
#
# `--print target-libdir` is pure path arithmetic off the sysroot: it prints a
# path for any *recognised* target whether or not it is installed. So the probe
# is the directory's existence, not the command's exit status — and that test
# reproduces the E0463 boundary exactly (Homebrew's rustc: dir missing for both
# Linux targets; rustup's: present).
target_std_installed() {
  local rustc="$1" target="$2" libdir
  libdir="$("$rustc" --print target-libdir --target "$target" 2>/dev/null)" || return 1
  [[ -n "$libdir" && -d "$libdir" ]]
}
