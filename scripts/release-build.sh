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

readonly TARGETS=(
  aarch64-apple-darwin
  aarch64-unknown-linux-gnu
  x86_64-unknown-linux-gnu
)

# Glibc floor for Linux targets — wide compatibility, RHEL 7-era.
readonly LINUX_GLIBC=2.17

die() {
  echo "release-build: $*" >&2
  exit 1
}

# The bundled stock rmux daemon/CLI is built at exactly the version grove's
# rmux-sdk resolves to in Cargo.lock — the rmux workspace publishes
# rmux ⇔ rmux-sdk ⇔ ratatui-rmux in lockstep, and the daemon must speak the
# SDK's wire protocol (ADR-0030 §2). Single source of truth = Cargo.lock; no
# hand-maintained pin to drift. release-doctor confirms a published rmux crate
# exists at this version before the build commits to it.
locked_rmux_sdk_version() {
  awk '
    /^\[\[package\]\]/ { name = "" }
    /^name = / { name = $3 }
    name == "\"rmux-sdk\"" && /^version = / { gsub(/"/, "", $3); print $3; exit }
  ' "$REPO_ROOT/Cargo.lock"
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

# Build the stock rmux daemon+CLI from the published crate, for one target,
# into a per-target install root (binary lands at $root/bin/rmux). rmux is a
# binary-only crate (no lib to wrap in a local manifest), so `cargo install`
# is the build path; `--locked` uses rmux's own published Cargo.lock for
# provenance; `--no-default-features` drops the `web` share server grove defers
# to the rmux-web grove (ADR-0030 §1). rmux has no openssl/curl, so the same
# zig cross used for grove's Linux targets builds it cleanly. The Linux cross
# uses `cargo-zigbuild install` — invoked on the binary DIRECTLY, because
# `cargo zigbuild …` dispatches to the `zigbuild` subcommand, which has no
# `install`. Writes to the caller's stderr only — safe inside build_target's
# command substitution.
build_rmux() {
  local target="$1" version="$2" root="$3"
  mkdir -p "$root"
  case "$target" in
    *-apple-darwin)
      cargo install rmux --version "=$version" \
        --no-default-features --locked --bin rmux \
        --target "$target" --root "$root" --force 1>&2 || return 1
      ;;
    *-unknown-linux-gnu)
      cargo-zigbuild install rmux --version "=$version" \
        --no-default-features --locked --bin rmux \
        --target "${target}.${LINUX_GLIBC}" --root "$root" --force 1>&2 || return 1
      ;;
    *)
      die "unknown target: $target"
      ;;
  esac
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
      # The trellis zellij-fork (which pulled curl + openssl-sys transitively via
      # isahc, needing `--features trellis/vendored_curl` for the OpenSSL-less
      # zigbuild cross-build) was removed in `020-rip-out`; a plain zigbuild now
      # suffices. Revisit if the rmux daemon dep reintroduces a system-lib need.
      cargo zigbuild --release --target "${target}.${LINUX_GLIBC}" || return 1
      ;;
    *)
      die "unknown target: $target"
      ;;
  esac

  # Bundle the stock rmux daemon/CLI beside grove (ADR-0030). RMUX_VERSION is
  # set in main() before this loop; command substitution inherits it.
  local rmux_root="$DIST_DIR/rmux-install/$target"
  build_rmux "$target" "$RMUX_VERSION" "$rmux_root" || return 1

  local stage="$DIST_DIR/staging/grove-v${version}-${target}"
  mkdir -p "$stage"
  cp "$REPO_ROOT/target/$target/release/grove" "$stage/grove"
  cp "$REPO_ROOT/target/$target/release/grove-llm" "$stage/grove-llm"
  cp "$rmux_root/bin/rmux" "$stage/rmux"
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
  "$REPO_ROOT/scripts/release-doctor.sh"
  require_clean_tagged_tree
  local version
  version="$(read_version)"
  echo "release-build: building grove v${version}"

  # Derived once, used per target inside build_target (ADR-0030 §2).
  RMUX_VERSION="$(locked_rmux_sdk_version)"
  [[ -n "$RMUX_VERSION" ]] \
    || die "could not read rmux-sdk version from Cargo.lock (is rmux-sdk a resolved dependency?)"
  echo "release-build: bundling stock rmux daemon v${RMUX_VERSION} (locked to rmux-sdk)"

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
  rm -rf "$DIST_DIR/staging" "$DIST_DIR/rmux-install"

  echo
  echo "release-build: artifacts in $DIST_DIR"
  ls -la "$DIST_DIR"
  echo
  echo "Inspect, then run scripts/release-publish.sh"
}

main "$@"
