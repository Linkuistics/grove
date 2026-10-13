#!/usr/bin/env bash
#
# Build per-target release archives, and render a Homebrew formula from
# scripts/templates/grove.rb.tmpl for a release.
#
#   scripts/release-build.sh
#       The release: every target, from a clean tree at a version tag.
#       Output in target/dist/: one grove-v<ver>-<target>.tar.xz per target,
#       and grove.rb. After this completes, inspect target/dist/ and run
#       release-publish.sh.
#   scripts/release-build.sh --unreleased [TARGET...]
#       Archives of the working copy at the workspace version, for the named
#       targets (default: every target), in target/archives/. No tag, clean
#       tree, formula or publishing prerequisite, and nothing publishable: the
#       installed-layout smoke tests extract these.
#
# Each run empties its whole output directory before anything can fail, so a
# failed run leaves no archive from an earlier one; an archive appears under its
# final name only once it has passed the manifest check.
#
# Every archive holds one directory, grove-v<ver>-<target>/, laid out as an
# installation prefix: bin/ holds grove and grove-llm; libexec/ravel-policy/
# holds Grove's readable caller assets. Ravel ships independently.
# `archive_manifest` in release-common.sh lists every file, and each archive is
# checked against it as soon as it is packed.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly DIST_DIR="$REPO_ROOT/target/dist"
readonly ARCHIVES_DIR="$REPO_ROOT/target/archives"
readonly TEMPLATE="$REPO_ROOT/scripts/templates/grove.rb.tmpl"

# TARGETS, pin_rust_toolchain, archive_manifest and assert_archive — shared
# with release-doctor.sh and the release tests, so the doctor checks the
# toolchain and target list this build uses and the tests check its layout.
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

# The version every shipped crate inherits, read without a tag. `cargo pkgid`
# prints `<source>#<version>`, or `<source>#<name>@<version>` when the name
# differs from the directory.
workspace_version() {
  local id
  id="$(cargo pkgid --quiet -p grove)" || die "cannot read grove's package version"
  echo "${id##*[#@]}"
}

build_target() {
  local target="$1" version="$2" out_dir="$3"
  # NOTE: the explicit `|| return 1`s are load-bearing. build_target runs inside
  # a command substitution (`archive="$(build_target ...)"`), and `set -e` does
  # NOT reliably abort a function in that context (notably under macOS's bash
  # 3.2). A bare failing cargo would otherwise fall through to the `cp` below
  # and tarball a STALE binary from a previous build. The caller also tests the
  # exit status. For the same reason everything but the archive path goes to
  # stderr: stdout is the caller's value.
  # Name only the two executable packages that ship in Grove archives.
  case "$target" in
    *-apple-darwin)
      cargo build --release --locked --target "$target" \
        -p grove -p grove-llm || return 1
      ;;
    *-unknown-linux-gnu)
      # Grove's two executables have no bundled runtime or system-library deps.
      cargo zigbuild --release --locked --target "${target}.${LINUX_GLIBC}" \
        -p grove -p grove-llm || return 1
      ;;
    *)
      die "unknown target: $target"
      ;;
  esac

  local top="grove-v${version}-${target}" binary
  local stage="$out_dir/staging/$top"
  mkdir -p "$stage/bin" || return 1
  for binary in grove grove-llm; do
    cp "$REPO_ROOT/target/$target/release/$binary" "$stage/bin/$binary" || return 1
  done
  mkdir -p "$stage/libexec" || return 1
  cp -R "$REPO_ROOT/assets/ravel-policy" "$stage/libexec/ravel-policy" || return 1
  cp "$REPO_ROOT/LICENSE" "$REPO_ROOT/README.md" "$stage/" || return 1

  local archive="$out_dir/$top.tar.xz"
  if ! pack_archive "$out_dir/staging" "$top" "$archive.partial" ||
    ! assert_archive "$archive.partial" "$top" >&2; then
    rm -f "$archive.partial"
    return 1
  fi
  mv "$archive.partial" "$archive" || return 1
  echo "$archive"
}

sha256_of() {
  shasum -a 256 "$1" | awk '{print $1}'
}

render_formula() {
  local version="$1" arg
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
  if grep -n '@[A-Z_0-9]*@' "$DIST_DIR/grove.rb" >&2; then
    die "grove.rb still holds template placeholders; every target needs its checksum"
  fi
}

main() {
  # render_formula's associative array needs bash 4; say so before building.
  ((BASH_VERSINFO[0] >= 4)) || die "bash 4 or later is required; found $BASH_VERSION"
  cd "$REPO_ROOT"
  # Before the doctor, so the doctor inherits the pinned PATH and checks the
  # toolchain the cargo invocations below will use.
  pin_rust_toolchain
  echo "release-build: cargo $(command -v cargo || echo '(not found)')"
  # build_target copies from this directory, so cargo must build into it
  # whatever CARGO_TARGET_DIR or a `build.target-dir` setting would say.
  export CARGO_TARGET_DIR="$REPO_ROOT/target"

  local release=1 out_dir="$DIST_DIR" version targets=() target known
  if [[ "${1:-}" == "--unreleased" ]]; then
    release=0
    out_dir="$ARCHIVES_DIR"
    shift
  elif (($# > 0)); then
    die "usage: release-build.sh [--unreleased [TARGET...]]"
  fi
  rm -rf "$out_dir"

  if ((release)); then
    "$REPO_ROOT/scripts/release-doctor.sh"
    require_clean_tagged_tree
    version="$(read_version)"
    [[ "$(workspace_version)" == "$version" ]] ||
      die "the tag is v${version} but the workspace version is $(workspace_version); cut the version with cargo release"
    targets=("${TARGETS[@]}")
    echo "release-build: building grove v${version}"
  else
    targets=("$@")
    ((${#targets[@]} > 0)) || targets=("${TARGETS[@]}")
    for target in "${targets[@]}"; do
      for known in "${TARGETS[@]}"; do
        [[ "$target" == "$known" ]] && continue 2
      done
      die "unknown target '$target'; the release targets are $(IFS=' ' && echo "${TARGETS[*]}")"
    done
    version="$(workspace_version)"
    echo "release-build: unreleased archives of the working copy at v${version}"
  fi

  mkdir -p "$out_dir/staging"

  local sha_args=()
  for target in "${targets[@]}"; do
    echo "release-build: target $target"
    local archive
    # Test the substitution explicitly — do not rely on `set -e` propagating out
    # of a command substitution (it does not, reliably). A failed build_target
    # must abort the release, never silently ship a stale tarball.
    if ! archive="$(build_target "$target" "$version" "$out_dir")"; then
      die "build failed for $target — see the output above"
    fi
    sha_args+=("${target}=$(sha256_of "$archive")")
  done

  ((release)) && render_formula "$version" "${sha_args[@]}"
  rm -rf "$out_dir/staging"

  echo
  echo "release-build: artifacts in $out_dir"
  ls -la "$out_dir"
  if ((release)); then
    echo
    echo "Inspect, then run scripts/release-publish.sh"
  fi
}

main "$@"
