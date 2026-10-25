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
# The caller selects the development environment. On the personal machine,
# `mise exec -- task check` supplies the configured tools and honours project
# version files. These checks announce the resolved Cargo and rustfmt and use
# that same environment throughout. They do not replace its toolchain with
# release-common.sh's `pin_rust_toolchain`: that separate pin owns cross-compiled
# release builds, which need rustup's target standard libraries.

set -euo pipefail
IFS=$'\n\t'
trap 'echo "check: error on line $LINENO" >&2' ERR

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
cd "$REPO_ROOT"

# Tests use an explicit public installation; normal checks resolve it from PATH.
# An override allows independently staged layouts without changing installation.
export GROVE_TEST_RAVEL="${GROVE_TEST_RAVEL:-$(command -v ravel || true)}"

failed=()
checks=0

# Select the whole workspace explicitly for both runtime and documentation
# tests; a default-member change must not silently reduce the gate's scope.
run_check() {
  local label="$1"
  shift
  checks=$((checks + 1))
  echo
  echo "=== $label"
  if "$@"; then
    echo "  ✓ $label"
  else
    echo "  ✗ $label"
    failed+=("$label")
  fi
}

# **Every book root under `docs/walkthroughs/`, by discovery.** A sixth book is
# gated by existing — there is no list here to forget to add it to, which is the
# whole point: a script naming one book is a script that silently stops covering
# the campaign the moment it grows.
#
# Final validation, not a scoped prefix. A `--through` run proves a canonical
# prefix and deliberately leaves later blocks deferred; only `--final` asserts
# that every byte of the book's declared corpus is reconstructed, which is the
# claim `docs/walkthroughs/` exists to make.
book_check() {
  local books=0 broken=0 book
  # Directories, not manifests. Globbing `*/walkthrough.toml` would let a book
  # whose manifest was deleted leave the gate without a word — the one failure
  # a discovery loop exists to make impossible. A directory under
  # `docs/walkthroughs/` is a book root, and a book root with no manifest is a
  # failure `book-check` itself reports as `U002`.
  for book in docs/walkthroughs/*/; do
    # The literal glob comes back when the directory is empty. Nothing to check
    # is not the same as everything checked, so it is reported as a failure —
    # a silent pass here would read exactly like a green campaign.
    [[ -d "$book" ]] || continue
    book="${book%/}"
    books=$((books + 1))
    echo "  book-check $book"
    if ! cargo run --quiet -p book-validation --bin book-check -- \
      --repo . --book "$book" --final --check all; then
      broken=$((broken + 1))
    fi
  done
  if ((books == 0)); then
    echo "  no book roots found under docs/walkthroughs/" >&2
    return 1
  fi
  echo "  $books book(s) checked, $broken failing"
  ((broken == 0))
}

echo "check: cargo    $(command -v cargo || echo '(not found)')"
echo "check: rustfmt  $(command -v rustfmt || echo '(not found)') — $(rustfmt --version 2>/dev/null || echo unknown)"

run_check "cargo fmt" cargo fmt --all --check
run_check "shellcheck" shellcheck \
  plugins/install.sh plugins/install.test.sh \
  plugins/grove/conformance.sh plugins/grove/conformance.test.sh \
  scripts/check.sh scripts/ravel-policy-typecheck.sh scripts/ravel-policy-typecheck.test.sh scripts/release-publish.sh scripts/release.test.sh \
  scripts/release-prepare.sh scripts/release-prepare.test.sh \
  scripts/release-notes.sh \
  scripts/release-notes/codex-headless.sh scripts/release-notes/codex-headless.test.sh \
  scripts/release-doctor.sh scripts/release-common.sh scripts/release-build.sh scripts/release-formula.test.sh scripts/release-registration.test.sh \
  scripts/release-smoke.sh scripts/release-smoke-target.sh scripts/release-smoke-qemu.sh \
  scripts/release-smoke-environment.sh scripts/ravel-target-smoke.sh \
  scripts/release-floor-backend.sh scripts/release-floor-backend.test.sh \
  scripts/atomic-directory-publish.test.sh \
  scripts/release-floor-backend-probe.sh scripts/release-floor-backend-probe-host.sh \
  scripts/floor-seccomp.test.sh scripts/release-floor-native.sh scripts/grove-floor-native-cases.sh \
  scripts/release-floor-qemu.sh scripts/release-floor-qemu-backend.sh scripts/release-floor-qemu.test.sh \
  scripts/ravel-target-smoke-target.sh scripts/ravel-installed-smoke.sh scripts/ravel-target-inputs.test.sh scripts/grove-installed-smoke.sh \
  plugins/linkuistics/skills/technical-communication/scripts/tc-hook \
  scripts/pi-extension-load.test.sh
run_check "cargo clippy" cargo clippy --workspace --all-targets
run_check "plugin install" bash plugins/install.test.sh
run_check "conformance" bash plugins/grove/conformance.sh
run_check "conformance suite" bash plugins/grove/conformance.test.sh
# The vendored checker ships its own self-test; `-I` keeps the interpreter
# from importing anything out of the current directory or the user's site.
run_check "technical-communication checker self-test" \
  python3 -I plugins/linkuistics/skills/technical-communication/scripts/ste-lint.py --selftest
# Loads the Pi extension in a real Pi, with no model request; it skips visibly
# when Pi is not installed.
run_check "technical-communication Pi extension loads" bash scripts/pi-extension-load.test.sh
run_check "release preparation" bash scripts/release-prepare.test.sh
run_check "Ravel archive admission" python3 scripts/test_ravel_target_inputs.py
run_check "Linux floor backend admission" task release:floor:controls
run_check "native floor seccomp admission" task release:floor:seccomp:controls
run_check "QEMU floor admission" task release:floor:qemu:controls
run_check "legacy driver preservation controls" task legacy:controls
run_check "release Codex helper" bash scripts/release-notes/codex-headless.test.sh
# The ignored public caller fixtures exercise the independently installed SDK
# and worker explicitly; the workspace runtime run does not include them.
run_check "Ravel caller assets and provenance" task ravel:review \
  "RAVEL=${GROVE_TEST_RAVEL:?Set GROVE_TEST_RAVEL to an installed Ravel front}"
run_check "release tasks" task release:test
# nextest runs runtime tests; doctests remain Cargo's responsibility.
# https://nexte.st/docs/running/#options-and-arguments
run_check "cargo nextest" cargo nextest run --locked --workspace --no-fail-fast
run_check "cargo doctests" cargo test --locked --workspace --doc
run_check "book-check" book_check

echo
if ((${#failed[@]} > 0)); then
  echo "check: FAILED — ${#failed[@]} of $checks"
  printf 'check:   ✗ %s\n' "${failed[@]}"
  exit 1
fi
echo "check: all $checks principal checks pass"
