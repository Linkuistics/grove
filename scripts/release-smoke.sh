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
#   *-unknown-linux-gnu       in a glibc-2.17 userland of the target's
#                             architecture. It needs Docker, and Zig, which
#                             builds the glibc control's probes. A target
#                             native to Docker runs in a container of that
#                             userland. x86_64 on an arm64 Docker runs in the
#                             same userland under a pinned user-mode QEMU (WHY
#                             OUR OWN QEMU, below). Any other target foreign to
#                             Docker is refused.
#
# scripts/release-smoke-target.sh runs inside each environment: it asserts the
# environment, extracts the archive with that environment's tar into a fresh
# prefix, checks the versions, and runs harness-dispatch's installed-layout
# cases, crates/harness-dispatch/scripts/installed-smoke.sh.
#
# WHY A CONTROL. A userland that is not really at the floor passes everything a
# floor userland passes. So each Linux run also executes a pair of probes built
# from one C source: one against glibc 2.17, which must run, and one that calls
# getrandom against glibc 2.25, which the userland must refuse for its
# GLIBC_2.25 symbol version. The first shows the userland runs binaries of that
# architecture at all; the refusal of the second then shows it enforces 2.17.
# The observed refusal is printed on every run.
#
# WHY OUR OWN QEMU. Docker Desktop on Apple silicon emulates amd64 with QEMU
# 8.1.5, which gives x86-64 guests no vDSO, and CentOS 7's glibc then crashes
# (docs/design/harness-selection-and-execution/runtime-evidence.md, *Installed
# smoke*). QEMU 10.2.3 supplies the vDSO, but with no reserved address space it
# bounds a 64-bit guest by nothing, so on an arm64 kernel with 48-bit addresses
# it maps the guest above 2^47, where no x86-64 kernel maps a process; the
# worker's JavaScriptCore heap then cannot allocate. Reserving the space (-R)
# fails, because QEMU maps the vsyscall page at 0xffffffffff600000. A guest base
# of 2^47 (-B 0x800000000000) maps the host's upper half onto the guest's
# [0, 2^47) instead. The worker's front scrubs QEMU_* from its environment, so
# the options reach QEMU through a native interpreter registered in
# binfmt_misc, and scripts/release-smoke-qemu.sh registers it in a private
# instance, leaving the Docker host's own registration alone.
#
# The Linux environments see only their inputs, read only, and run the smoke
# test as an unprivileged user with no network. The CPU model under emulation
# is not the CPU floor; see docs/specs/harness-selection-and-execution.md,
# *Delivery and release*.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly TARGET_SCRIPT="$REPO_ROOT/scripts/release-smoke-target.sh"
readonly QEMU_SCRIPT="$REPO_ROOT/scripts/release-smoke-qemu.sh"
readonly CASES="$REPO_ROOT/crates/harness-dispatch/scripts/installed-smoke.sh"

# TARGETS, the release target list.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# CentOS Linux 7.9.2009, whose glibc is 2.17, Grove's Linux floor. It is
# end-of-life, which is the point: no maintained distribution ships 2.17. The
# manifest-list digest serves linux/amd64 and linux/arm64/v8, so every run on
# either architecture executes the same userland.
readonly FLOOR_IMAGE="docker.io/library/centos:7@sha256:be65f488b7764ad3638f236b7b515b3678369a5124c47b8d32916d6487418ea4"

# QEMU 10.2.3's user-mode emulators, built for Docker's architecture:
# tonistiigi/binfmt's qemu-v10.2.3-68, pinned by manifest-list digest. This
# build takes the argument after the executable's path as its argv[0], the
# layout binfmt_misc's P flag passes.
readonly QEMU_IMAGE="docker.io/tonistiigi/binfmt@sha256:400a4873b838d1b89194d982c45e5fb3cda4593fbfd7e08a02e76b03b21166f0"

# The helper container that holds the emulated userland: Ubuntu 24.04, for
# util-linux's unshare and mount and coreutils' chroot --userspec.
readonly HELPER_IMAGE="docker.io/library/ubuntu:24.04@sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3"

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

# The binfmt_misc interpreter for ARCH's guests, in DIR: a static executable
# for Docker's arm64 that runs /qemu/qemu, where release-smoke-qemu.sh puts
# the emulator inside the userland, with ARCH's options (WHY OUR OWN QEMU)
# before the arguments binfmt_misc passes it.
build_qemu_interpreter() {
  local arch="$1" dir="$2" options
  case "$arch" in
    # QEMU 10.2.3 sets no guest address limit for a 64-bit guest without -R:
    # https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/linux-user/main.c
    x86_64) options='"-B","0x800000000000",' ;;
    *) die "no QEMU options are known for $arch guests" ;;
  esac
  cat >"$dir/interpreter.c" <<'EOF'
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
  static char *const fixed[] = {"/qemu/qemu", QEMU_OPTIONS};
  const int count = sizeof fixed / sizeof *fixed;
  char *args[count + argc];
  int n = 0;
  for (int i = 0; i < count; i++) args[n++] = fixed[i];
  for (int i = 1; i < argc; i++) args[n++] = argv[i];
  args[n] = NULL;
  execv(args[0], args);
  perror("release-smoke interpreter: /qemu/qemu");
  return 127;
}
EOF
  zig cc -target aarch64-linux-musl -static -O2 "-DQEMU_OPTIONS=$options" \
    -o "$dir/interpreter" "$dir/interpreter.c" || die "zig cannot build the QEMU interpreter"
  rm "$dir/interpreter.c"
}

# Copy PATH out of IMAGE for PLATFORM to DEST without running the image; a PATH
# of / exports its whole filesystem as a tar file.
copy_from_image() {
  local platform="$1" image="$2" path="$3" dest="$4" container status=0
  container="$(docker create --platform "$platform" "$image" /bin/true)" || return 1
  if [[ "$path" == / ]]; then
    docker export --output "$dest" "$container" || status=$?
  else
    docker cp --quiet "$container:$path" "$dest" || status=$?
  fi
  docker rm "$container" >/dev/null || status=$?
  return "$status"
}

# main calls the routes below on the left of `||`, where errexit does not
# reach, so every step before the run returns its own failure.

# The inputs every Linux route reads, in MOUNT: the archive, the two
# in-environment scripts and the glibc probes.
stage_linux_inputs() {
  local target="$1" archive="$2" mount="$3"
  mkdir -p "$mount/probes" || return 1
  build_glibc_probes "${target%%-*}" "$mount/probes"
  cp "$archive" "$TARGET_SCRIPT" "$CASES" "$mount/"
}

smoke_in_floor_container() {
  local target="$1" archive="$2" platform="$3" mount="$SCRATCH/$1"
  stage_linux_inputs "$target" "$archive" "$mount" || return 1
  echo "release-smoke: $target: $platform container, native to Docker's $DOCKER_PLATFORM"
  docker run --rm --network none --user 1000:1000 \
    --platform "$platform" \
    --volume "$mount:/smoke:ro" \
    "$FLOOR_IMAGE" \
    /usr/bin/env -i PATH=/usr/bin:/bin /bin/bash /smoke/release-smoke-target.sh \
    "$target" "/smoke/$(basename "$archive")" /smoke/installed-smoke.sh /smoke/probes
}

# The floor image's userland for PLATFORM under the pinned QEMU, in a helper
# container of Docker's own architecture (WHY OUR OWN QEMU).
smoke_under_qemu() {
  local target="$1" archive="$2" platform="$3" mount="$SCRATCH/$1" arch="${1%%-*}"
  stage_linux_inputs "$target" "$archive" "$mount" || return 1
  copy_from_image "$platform" "$FLOOR_IMAGE" / "$mount/rootfs.tar" || return 1
  copy_from_image "$DOCKER_PLATFORM" "$QEMU_IMAGE" "/usr/bin/qemu-$arch" "$mount/qemu" || return 1
  build_qemu_interpreter "$arch" "$mount"
  cp "$QEMU_SCRIPT" "$mount/" || return 1
  echo "release-smoke: $target: $platform userland under $QEMU_IMAGE, in a $DOCKER_PLATFORM $HELPER_IMAGE container"
  docker run --rm --network none \
    --platform "$DOCKER_PLATFORM" \
    --volume "$mount:/smoke:ro" \
    "$HELPER_IMAGE" \
    /bin/bash /smoke/release-smoke-qemu.sh /smoke "$target" "$(basename "$archive")"
}

smoke_linux() {
  local target="$1" archive="$2" platform
  platform="$(docker_platform "$target")" || return 1
  if [[ "$platform" == "$DOCKER_PLATFORM" ]]; then
    smoke_in_floor_container "$target" "$archive" "$platform"
  elif [[ "$target" == x86_64-unknown-linux-gnu && "$DOCKER_PLATFORM" == linux/arm64 ]]; then
    smoke_under_qemu "$target" "$archive" "$platform"
  else
    echo "release-smoke: $target: no environment runs $platform on Docker's $DOCKER_PLATFORM" >&2
    return 1
  fi
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
      *-unknown-linux-gnu) smoke_linux "$target" "$archive" ;;
    esac || die "$target: the installed smoke test failed; see above"
  done
  echo "release-smoke: passed: $(IFS=' ' && echo "${targets[*]}")"
}

main "$@"
