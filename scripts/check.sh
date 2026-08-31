#!/usr/bin/env bash
#
# The principal checks, as one runnable command. Run it before describing a
# change and as step 1 of a release; docs/ARCHITECTURE.md and docs/RELEASING.md
# both point here rather than restating the list.
#
# WHY THIS FILE EXISTS. The list used to be prose in two documents, and the two
# had already diverged (only one of them ran shellcheck). Prose that has to be
# retyped is prose that gets partially retyped: `cargo fmt --all --check` sat in
# both lists, was named in no leaf's `## Done when`, and drifted red across four
# leaves of the crate split with nothing to report it (restore-fmt-clean-k33).
# One list, in one executable file, is the fix that fits this repo.
#
# WHAT IT DOES NOT DO. It does not gate. There is no CI in this repository and
# this script does not add one — it is run by a person or a session, and a
# release runs it because docs/RELEASING.md step 1 is now this command. That is
# a weaker claim than a server that refuses a push, and it is stated here rather
# than implied so nobody reads a green tree as a guarded one.
#
# WHY NO rust-toolchain.toml. Measured, twice, rather than assumed:
#   - `/opt/homebrew/bin/cargo` wins PATH on the development machine and is not
#     a rustup proxy, so it ignores rust-toolchain.toml outright — a pin there
#     would be inert on the very machine the drift happened on. (Verified with a
#     throwaway crate naming a nonexistent channel: Homebrew's cargo formatted
#     anyway; rustup's refused.)
#   - The skew it would defend against is currently zero: rustfmt 1.9.0
#     (Homebrew) and 1.8.0-stable (rustup) agree on this tree.
# release-common.sh's `pin_rust_toolchain` is the repo's real answer to "which
# toolchain", and this script deliberately does NOT call it. That pin exists for
# the cross-compiled release build, which needs rustup's Linux `std`. These
# checks run on the host, and pinning them would lint the source under rustup's
# toolchain while the developer builds under whatever their PATH resolves —
# making the check mean something different from the build. Instead the
# resolved toolchain is announced, in release-common.sh's own idiom.

set -euo pipefail
IFS=$'\n\t'
trap 'echo "check: error on line $LINENO" >&2' ERR

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
cd "$REPO_ROOT"

failed=()

# `--workspace` on both cargo lines because this root is *also* a package: a
# bare invocation tests and lints `grove` alone and leaves the other five crates
# unread.
run_check() {
  local label="$1"
  shift
  echo
  echo "=== $label"
  if "$@"; then
    echo "  ✓ $label"
  else
    echo "  ✗ $label"
    failed+=("$label")
  fi
}

echo "check: cargo    $(command -v cargo || echo '(not found)')"
echo "check: rustfmt  $(command -v rustfmt || echo '(not found)') — $(rustfmt --version 2>/dev/null || echo unknown)"

run_check "cargo fmt" cargo fmt --all --check
run_check "shellcheck" shellcheck \
  plugins/install.sh plugins/install.test.sh \
  plugins/grove/conformance.sh plugins/grove/conformance.test.sh \
  scripts/check.sh
run_check "cargo clippy" cargo clippy --workspace --all-targets
run_check "plugin install" bash plugins/install.test.sh
run_check "conformance" bash plugins/grove/conformance.sh
run_check "conformance suite" bash plugins/grove/conformance.test.sh
run_check "cargo test" cargo test --locked --workspace

echo
if ((${#failed[@]} > 0)); then
  echo "check: FAILED — ${#failed[@]} of 7"
  printf 'check:   ✗ %s\n' "${failed[@]}"
  exit 1
fi
echo "check: all 7 principal checks pass"
