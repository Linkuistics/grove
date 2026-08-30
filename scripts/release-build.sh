#!/usr/bin/env bash
#
# Build per-target tarballs for the current git tag and render a Homebrew
# formula from scripts/templates/grove.rb.tmpl.
#
# Output: target/dist/
#   grove-v<ver>-<target>.tar.xz       (one per target)
#   grove.rb                           (rendered formula)
#
# After this completes, inspect target/dist/ and run release-publish.sh.

set -euo pipefail
IFS=$'\n\t'

readonly REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly DIST_DIR="$REPO_ROOT/target/dist"
readonly TEMPLATE="$REPO_ROOT/scripts/templates/grove.rb.tmpl"

# TARGETS and pin_rust_toolchain — shared with release-doctor.sh so the doctor
# checks the toolchain and target list this build actually uses.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# Glibc floor for Linux targets — wide compatibility, RHEL 7-era.
readonly LINUX_GLIBC=2.17

die() {
  echo "release-build: $*" >&2
  exit 1
}

require_clean_tagged_tree() {
  [[ -z "$(git -C "$REPO_ROOT" status --porcelain)" ]] \
    || die "working tree is dirty; commit or stash before releasing"
  git -C "$REPO_ROOT" describe --tags --exact-match HEAD >/dev/null 2>&1 \
    || die "HEAD is not a tagged commit; run 'cargo release <level> --execute' first"
}

read_version() {
  git -C "$REPO_ROOT" describe --tags --abbrev=0 | sed 's/^v//'
}

build_target() {
  local target="$1" version="$2"
  # NOTE: the explicit `|| return 1` is load-bearing. build_target runs inside a
  # command substitution (`archive="$(build_target ...)"`), and `set -e` does NOT
  # reliably abort a function in that context (notably under macOS's bash 3.2). A
  # bare failing cargo would otherwise fall through to the `cp` below and tarball a
  # STALE binary from a previous build. The caller also tests the exit status.
  case "$target" in
    *-apple-darwin)
      cargo build --release --target "$target" || return 1
      ;;
    *-unknown-linux-gnu)
      # grove is pure Rust with no system-lib deps (the TUI tower that once pulled
      # curl/openssl was shed in `shed-tui-k20`), so a plain zigbuild cross-build
      # suffices with no vendored-lib feature.
      cargo zigbuild --release --target "${target}.${LINUX_GLIBC}" || return 1
      ;;
    *)
      die "unknown target: $target"
      ;;
  esac

  local stage="$DIST_DIR/staging/grove-v${version}-${target}"
  mkdir -p "$stage"
  cp "$REPO_ROOT/target/$target/release/grove" "$stage/grove"
  cp "$REPO_ROOT/target/$target/release/grove-llm" "$stage/grove-llm"
  cp "$REPO_ROOT/LICENSE" "$REPO_ROOT/README.md" "$stage/"

  local archive="$DIST_DIR/grove-v${version}-${target}.tar.xz"
  tar -C "$DIST_DIR/staging" -cJf "$archive" "grove-v${version}-${target}"
  echo "$archive"
}

sha256_of() {
  shasum -a 256 "$1" | awk '{print $1}'
}

render_formula() {
  local version="$1"
  shift
  local -A shas
  for arg in "$@"; do
    shas["${arg%%=*}"]="${arg#*=}"
  done

  sed \
    -e "s|@VERSION@|${version}|g" \
    -e "s|@SHA_AARCH64_APPLE_DARWIN@|${shas[aarch64-apple-darwin]}|g" \
    -e "s|@SHA_AARCH64_UNKNOWN_LINUX_GNU@|${shas[aarch64-unknown-linux-gnu]}|g" \
    -e "s|@SHA_X86_64_UNKNOWN_LINUX_GNU@|${shas[x86_64-unknown-linux-gnu]}|g" \
    "$TEMPLATE" >"$DIST_DIR/grove.rb"
}

main() {
  cd "$REPO_ROOT"
  # Before the doctor, so the doctor inherits the pinned PATH and checks the
  # toolchain the cargo invocations below will use.
  pin_rust_toolchain
  echo "release-build: cargo $(command -v cargo || echo '(not found)')"
  "$REPO_ROOT/scripts/release-doctor.sh"
  require_clean_tagged_tree
  local version
  version="$(read_version)"
  echo "release-build: building grove v${version}"

  rm -rf "$DIST_DIR"
  mkdir -p "$DIST_DIR/staging"

  local sha_args=()
  for target in "${TARGETS[@]}"; do
    echo "release-build: target $target"
    local archive
    # Test the substitution explicitly — do not rely on `set -e` propagating out
    # of a command substitution (it does not, reliably). A failed build_target
    # must abort the release, never silently ship a stale tarball.
    if ! archive="$(build_target "$target" "$version")"; then
      die "build failed for $target — see cargo output above"
    fi
    sha_args+=("${target}=$(sha256_of "$archive")")
  done

  render_formula "$version" "${sha_args[@]}"
  rm -rf "$DIST_DIR/staging"

  echo
  echo "release-build: artifacts in $DIST_DIR"
  ls -la "$DIST_DIR"
  echo
  echo "Inspect, then run scripts/release-publish.sh"
}

main "$@"
