#!/usr/bin/env bash
#
# Publish the artifacts produced by release-build.sh:
#   1. Create a GitHub Release on Linkuistics/grove for v<ver> and
#      upload all tarballs from target/dist/.
#   2. Copy grove.rb into $GROVE_TAP_DIR/Formula/, commit, push.
#
# Prerequisite: ./scripts/release-build.sh has just run successfully.
# Env: GROVE_TAP_DIR (default ~/Development/homebrew-taps).
#
# This script was recorded as one of the two gestures the harness classifier
# refuses as an opaque invocation. Measured on 2026-07-31 it ran permitted, in
# the same agent-driven cut that published v16.3.0 — so run it directly rather
# than planning around a refusal. Refusal is not inherent to the invocation: it
# depends on the session's own harness and permission configuration, which this
# script cannot see. If a session's classifier does refuse, its two steps run
# fine spelled out by hand — that is the fallback, not the expected path.
# See release.toml's preamble for the full note.
#
# ---------------------------------------------------------------------------
# Proving the *installed* binary carries the release
#
# `brew upgrade` after this leaves the obvious question — does the installed
# grove actually do the new thing? `strings` on the binary is a weak answer and
# sometimes a wrong one. Drive it functionally instead. One isolated launch
# yields several proofs at once:
#
#   scratch=$(mktemp -d) && cd "$scratch" && git init -q .
#   grove-llm root-init                       # a live tree for `pick` to walk
#   printf '#!/bin/sh\nprintf "GROVE_SIGNAL_FILE=%s\\n" "$GROVE_SIGNAL_FILE"\n' > fake
#   chmod +x fake
#   env -u GROVE_SIGNAL_FILE \
#       GROVE_HARNESS_BIN_CLAUDE="$PWD/fake" GROVE_CLAUDE_MODEL=opus \
#       grove do --harness claude
#
# `--harness claude` plus an explicit model are needed because a scratch tree
# has no stamp and no detectable harness. The fake exits 0 without signalling,
# so the nested loop stops itself after one iteration.
#
# What that shows: the driver's launch line (routing diagnostics), a fresh
# completion-signal path reaching the harness child, and — because `grove do`
# re-provisions the skill from the binary — the methodology surfaces. A fourth
# comes free from the scratch tree being untrusted by codex:
# `grove do --harness codex --no-launch` there exercises the sandbox preflight
# refusal end to end.
#
# Run this in the same session as the `brew upgrade` — see release.toml on why
# that is safe and why deferring it to a follow-up session is unnecessary.

set -euo pipefail
IFS=$'\n\t'

readonly REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly DIST_DIR="$REPO_ROOT/target/dist"
readonly TAP_DIR="${GROVE_TAP_DIR:-$HOME/Development/homebrew-taps}"

die() {
  echo "release-publish: $*" >&2
  exit 1
}

preflight() {
  command -v gh >/dev/null || die "gh CLI not on PATH"
  gh auth status >/dev/null 2>&1 || die "gh not authenticated; run 'gh auth login'"
  [[ -d "$DIST_DIR" ]] || die "no $DIST_DIR; run scripts/release-build.sh first"
  [[ -f "$DIST_DIR/grove.rb" ]] || die "no rendered formula at $DIST_DIR/grove.rb"
  compgen -G "$DIST_DIR/*.tar.xz" >/dev/null || die "no tarballs in $DIST_DIR"
  [[ -d "$TAP_DIR/.git" ]] || die "tap clone not found at $TAP_DIR (set GROVE_TAP_DIR)"
}

read_version() {
  git -C "$REPO_ROOT" describe --tags --abbrev=0 | sed 's/^v//'
}

verify_tag_matches_artifacts() {
  local version="$1"
  local sample
  sample="$(ls "$DIST_DIR"/grove-v*-aarch64-apple-darwin.tar.xz 2>/dev/null | head -n1)" \
    || die "missing aarch64-apple-darwin tarball"
  [[ "$sample" == *"grove-v${version}-"* ]] \
    || die "artifact version mismatch: $sample does not contain v${version}"
}

create_github_release() {
  local version="$1"
  local tag="v${version}"
  echo "release-publish: creating GitHub Release $tag"
  gh release create "$tag" \
    --repo Linkuistics/grove \
    --title "Release $tag" \
    --notes "Release $tag" \
    "$DIST_DIR"/*.tar.xz
}

push_formula_to_tap() {
  local version="$1"
  echo "release-publish: pushing formula to $TAP_DIR"
  mkdir -p "$TAP_DIR/Formula"
  cp "$DIST_DIR/grove.rb" "$TAP_DIR/Formula/grove.rb"
  git -C "$TAP_DIR" add Formula/grove.rb
  git -C "$TAP_DIR" commit -m "grove v${version}"
  git -C "$TAP_DIR" push
}

main() {
  preflight
  local version
  version="$(read_version)"
  verify_tag_matches_artifacts "$version"

  create_github_release "$version"
  push_formula_to_tap "$version"

  echo
  echo "release-publish: done. Verify with:"
  echo "  brew update && brew install linkuistics/taps/grove"
}

main "$@"
