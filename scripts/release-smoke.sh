#!/usr/bin/env bash
#
# The installed-layout smoke test of the release archives: each target's
# archive, extracted where that target runs and exercised with no Bun or Node
# on PATH.
#
#   scripts/release-smoke.sh [--archives DIR] [TARGET...]
#
# DIR holds one grove-v<version>-<target>.tar.xz per target; the default is
# target/archives, which `task release:archives` fills. The default targets are
# every release target. `task release:smoke` rebuilds the archives from the
# working copy first, so it never reads a stale one.
#
#   aarch64-apple-darwin      natively. It needs a macOS arm64 host.
#   *-unknown-linux-gnu       in a glibc-2.17 userland container of the
#                             target's architecture. It needs Docker, which
#                             emulates a foreign architecture itself, and Zig,
#                             which builds the glibc control's probes.
#
# scripts/release-smoke-target.sh runs inside each environment: it asserts the
# environment, extracts the archive with that environment's tar into a fresh
# prefix, checks the versions, and runs harness-dispatch's installed-layout
# cases, crates/harness-dispatch/scripts/installed-smoke.sh.
#
# WHY A CONTROL. A container that is not really at the floor passes everything
# a floor container passes. So each Linux run also executes a pair of probes
# built from one C source: one against glibc 2.17, which must run, and one that
# calls getrandom against glibc 2.25, which the container must refuse for its
# GLIBC_2.25 symbol version. The first shows the container runs binaries of that
# architecture at all; the refusal of the second then shows it enforces 2.17.
# The observed refusal is printed on every run.
#
# The containers see only the archive, the two scripts and the probes, read
# only, and run as an unprivileged user with no network. The CPU model under
# Docker's emulation is not the CPU floor; see docs/specs/
# harness-selection-and-execution.md, *Delivery and release*.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly TARGET_SCRIPT="$REPO_ROOT/scripts/release-smoke-target.sh"
readonly CASES="$REPO_ROOT/crates/harness-dispatch/scripts/installed-smoke.sh"

# TARGETS, the release target list.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# CentOS Linux 7.9.2009, whose glibc is 2.17, Grove's Linux floor. It is
# end-of-life, which is the point: no maintained distribution ships 2.17. The
# manifest-list digest serves linux/amd64 and linux/arm64/v8, so every run on
# either architecture executes the same userland.
readonly FLOOR_IMAGE="docker.io/library/centos:7@sha256:be65f488b7764ad3638f236b7b515b3678369a5124c47b8d32916d6487418ea4"

die() {
  echo "release-smoke: $*" >&2
  exit 1
}

# The archive DIR holds for TARGET; exactly one.
find_archive() {
  local dir="$1" target="$2" matches=() match
  for match in "$dir"/grove-v*-"$target".tar.xz; do
    [[ -f "$match" ]] && matches+=("$match")
  done
  ((${#matches[@]} == 1)) && {
    echo "${matches[0]}"
    return
  }
  ((${#matches[@]} == 0)) &&
    die "no archive for $target in $dir; build one with 'task release:archives TARGETS=$target'"
  die "more than one archive for $target in $dir: ${matches[*]}"
}

docker_platform() {
  case "$1" in
    aarch64-unknown-linux-gnu) echo linux/arm64 ;;
    x86_64-unknown-linux-gnu) echo linux/amd64 ;;
    *) die "no container platform for $1" ;;
  esac
}

# The glibc control's probe pair for ARCH (uname -m's name for it), in DIR.
build_glibc_probes() {
  local arch="$1" dir="$2"
  cat >"$dir/probe.c" <<'EOF'
#include <stdio.h>
#ifdef ABOVE_FLOOR
#include <sys/random.h>
#endif

int main(void) {
#ifdef ABOVE_FLOOR
  unsigned char byte;
  if (getrandom(&byte, 1, 0) != 1) return 2;
#endif
  puts("glibc probe ran");
  return 0;
}
EOF
  if ! zig cc -target "$arch-linux-gnu.2.17" -O2 -o "$dir/floor-$arch" "$dir/probe.c" ||
    ! zig cc -target "$arch-linux-gnu.2.25" -DABOVE_FLOOR -O2 -o "$dir/above-$arch" "$dir/probe.c"; then
    die "zig cannot build the glibc probes for $arch"
  fi
  rm "$dir/probe.c"
}

smoke_native() {
  local target="$1" archive="$2"
  env -i PATH=/usr/bin:/bin /bin/bash "$TARGET_SCRIPT" "$target" "$archive" "$CASES"
}

# main calls this on the left of `||`, where errexit does not reach, so every
# step before the container returns its own failure.
smoke_in_floor_container() {
  local target="$1" archive="$2" mount="$SCRATCH/$1" platform
  platform="$(docker_platform "$target")" || return 1
  mkdir -p "$mount/probes" || return 1
  build_glibc_probes "${target%%-*}" "$mount/probes"
  cp "$archive" "$TARGET_SCRIPT" "$CASES" "$mount/" || return 1
  if [[ "$platform" == "$DOCKER_PLATFORM" ]]; then
    echo "release-smoke: $target: $platform container, native to Docker's $DOCKER_PLATFORM"
  else
    echo "release-smoke: $target: $platform container, emulated by Docker on $DOCKER_PLATFORM"
  fi
  docker run --rm --network none --user 1000:1000 \
    --platform "$platform" \
    --volume "$mount:/smoke:ro" \
    "$FLOOR_IMAGE" \
    /usr/bin/env -i PATH=/usr/bin:/bin /bin/bash /smoke/release-smoke-target.sh \
    "$target" "/smoke/$(basename "$archive")" /smoke/installed-smoke.sh /smoke/probes && return
  local status=$?
  [[ "$platform" == "$DOCKER_PLATFORM" ]] ||
    echo "release-smoke: $target ran under Docker's emulation of $platform. Docker Desktop's QEMU cannot run this glibc-2.17 userland on arm64 (docs/design/harness-selection-and-execution/runtime-evidence.md, *Installed smoke*)." >&2
  return "$status"
}

main() {
  local archives="$REPO_ROOT/target/archives" targets=() target known
  if [[ "${1:-}" == "--archives" ]]; then
    archives="${2:?--archives needs a directory}"
    shift 2
  fi
  targets=("$@")
  ((${#targets[@]} > 0)) || targets=("${TARGETS[@]}")
  for target in "${targets[@]}"; do
    for known in "${TARGETS[@]}"; do
      [[ "$target" == "$known" ]] && continue 2
    done
    die "unknown target '$target'; the release targets are $(IFS=' ' && echo "${TARGETS[*]}")"
  done
  for target in "${targets[@]}"; do
    [[ "$target" == *-linux-gnu ]] || continue
    command -v zig >/dev/null 2>&1 || die "zig is required for the glibc control's probes; see docs/RELEASING.md"
    docker info >/dev/null 2>&1 || die "Docker is required for the Linux targets, and is not running"
    DOCKER_PLATFORM="$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')"
    echo "release-smoke: floor image $FLOOR_IMAGE, docker $(docker version --format '{{.Server.Version}}') on $DOCKER_PLATFORM, zig $(zig version)"
    break
  done

  SCRATCH="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$SCRATCH'" EXIT
  local archive
  for target in "${targets[@]}"; do
    archive="$(find_archive "$archives" "$target")"
    echo "release-smoke: $target: $archive"
    case "$target" in
      *-apple-darwin) smoke_native "$target" "$archive" ;;
      *-unknown-linux-gnu) smoke_in_floor_container "$target" "$archive" ;;
    esac || die "$target: the installed smoke test failed; see above"
  done
  echo "release-smoke: passed: $(IFS=' ' && echo "${targets[*]}")"
}

main "$@"
