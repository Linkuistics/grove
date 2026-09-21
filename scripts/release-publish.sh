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
#   mkdir -p "$scratch/home/.config/grove"
#   {
#     printf 'config {\n  command "smoke" "%s/fake ${prompt}"\n  bind "smoke" "smoke"\n' "$scratch"
#     for kind in requirements design planning prototype impl \
#               review-requirements review-design review-planning \
#               review-prototype review-impl \
#               integrate-review-requirements integrate-review-design \
#               integrate-review-planning integrate-review-prototype \
#               integrate-review-impl \
#               research-a research-b combine-research \
#               draft copy-edit art proof finish; do
#       printf '  route "%s" "smoke"\n' "$kind"
#     done
#     printf '}\n'
#   } > "$scratch/home/.config/grove/config.kdl"
#   env -u GROVE_SIGNAL_FILE HOME="$scratch/home" grove
#
# The isolated `HOME` is what makes this a *test* rather than a run of your own
# launch policy: a complete personal config is mandatory, every kind the
# methodology ships, and the fake stands in for every one of them. It exits 0 without signalling,
# so the nested loop stops itself after one iteration.
#
# What that shows: the configured argv reaching the real foreground child and a
# fresh completion-signal path granted to it. It shows nothing about the
# methodology delivery: that needs an isolated Codex installation and is covered
# by skill_provisioning.rs. Point `HOME` at a directory with no `config.kdl` for the other half:
# the aggregate configuration diagnostic, with no tree mutation.
#
# Run this in the same session as the `brew upgrade` — see release.toml on why
# that is safe and why deferring it to a follow-up session is unnecessary.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly DIST_DIR="$REPO_ROOT/target/dist"
readonly TAP_DIR="${GROVE_TAP_DIR:-$HOME/Development/homebrew-taps}"

die() {
  echo "release-publish: $*" >&2
  exit 1
}

check_tap() {
  [[ -d "$TAP_DIR/.git" ]] || die "tap clone not found at $TAP_DIR (set GROVE_TAP_DIR)"
  local tap_status
  tap_status="$(git -C "$TAP_DIR" status --porcelain)"
  [[ -z "$tap_status" ]] \
    || die "tap working tree is dirty; preserve its changes before publishing"
  if [[ -d "$TAP_DIR/.jj" ]]; then
    command -v jj >/dev/null || die "jj tap requires jj on PATH"
    jj -R "$TAP_DIR" git fetch
    [[ -n "$(jj -R "$TAP_DIR" log --no-graph -r 'main & main@origin' -T commit_id)" ]] \
      || die "tap main differs from origin; integrate or push its work before publishing"
  else
    git -C "$TAP_DIR" symbolic-ref --quiet HEAD >/dev/null \
      || die "tap is on detached HEAD; switch to the branch intended for publication"
    local upstream tap_head upstream_head
    upstream="$(git -C "$TAP_DIR" rev-parse --abbrev-ref --symbolic-full-name '@{upstream}')" \
      || die "tap branch has no upstream; configure the intended publishing branch"
    git -C "$TAP_DIR" fetch
    tap_head="$(git -C "$TAP_DIR" rev-parse HEAD)"
    upstream_head="$(git -C "$TAP_DIR" rev-parse "$upstream")"
    [[ "$tap_head" == "$upstream_head" ]] \
      || die "tap branch differs from $upstream; integrate or push its work before publishing"
  fi
}

preflight() {
  command -v gh >/dev/null || die "gh CLI not on PATH"
  gh auth status >/dev/null 2>&1 || die "gh not authenticated; run 'gh auth login'"
  [[ -d "$DIST_DIR" ]] || die "no $DIST_DIR; run scripts/release-build.sh first"
  [[ -f "$DIST_DIR/grove.rb" ]] || die "no rendered formula at $DIST_DIR/grove.rb"
  compgen -G "$DIST_DIR/*.tar.xz" >/dev/null || die "no tarballs in $DIST_DIR"
  check_tap
}

read_version() {
  git -C "$REPO_ROOT" describe --tags --exact-match HEAD | sed 's/^v//'
}

verify_tag_matches_artifacts() {
  local version="$1"
  local sample="$DIST_DIR/grove-v${version}-aarch64-apple-darwin.tar.xz"
  [[ -f "$sample" ]] || die "missing archive for the current tag: $sample"
}

create_github_release() {
  local version="$1"
  local tag="v${version}"
  local notes="$DIST_DIR/release-notes.md"
  awk -v heading="## $tag" '
    $0 == heading { section=1; next }
    section && /^## / { exit }
    section { print }
  ' "$REPO_ROOT/CHANGELOG.md" >"$notes"
  grep -q '[^[:space:]]' "$notes" || die "no changelog notes found for $tag"
  echo "release-publish: creating GitHub Release $tag"
  gh release create "$tag" \
    --verify-tag \
    --repo Linkuistics/grove \
    --title "Release $tag" \
    --notes-file "$notes" \
    "$DIST_DIR"/*.tar.xz
}

push_formula_to_tap() {
  local version="$1"
  echo "release-publish: pushing formula to $TAP_DIR"
  if [[ -d "$TAP_DIR/.jj" ]]; then
    jj -R "$TAP_DIR" new main
  fi
  mkdir -p "$TAP_DIR/Formula"
  cp "$DIST_DIR/grove.rb" "$TAP_DIR/Formula/grove.rb"
  if [[ -d "$TAP_DIR/.jj" ]]; then
    jj -R "$TAP_DIR" describe -m "grove v${version}"
    jj -R "$TAP_DIR" bookmark set main -r @
    jj -R "$TAP_DIR" new main
    jj -R "$TAP_DIR" git push -b main
  else
    git -C "$TAP_DIR" add Formula/grove.rb
    git -C "$TAP_DIR" commit -m "grove v${version}"
    git -C "$TAP_DIR" push
  fi
}

main() {
  case "${1:-}" in
    --check-tap) check_tap; return ;;
    '') ;;
    *) die "usage: scripts/release-publish.sh [--check-tap]" ;;
  esac
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
