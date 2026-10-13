#!/usr/bin/env bash
# Real host-kernel namespace backend, isolated from each emulated target libc.
set -euo pipefail
IFS=$'\n\t'
# shellcheck source=scripts/release-floor-backend.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/release-floor-backend.sh"

prepare_qemu_backend() (
  local output="$1" stage library
  [[ ! -e "$output" && ! -L "$output" ]] || die "output already exists: $output"
  [[ "$(zig version)" == 0.17.0 ]] || die 'QEMU namespace backend requires Zig 0.17.0'
  mkdir -p "$(dirname "$output")"
  output="$(cd "$(dirname "$output")" && pwd -P)/$(basename "$output")"
  [[ ! -e "$output" && ! -L "$output" ]] || die "output already exists: $output"
  stage="$(mktemp -d "$(dirname "$output")/.qemu-backend.XXXXXX")"
  # A subshell caller owns cleanup even on a fetch/compiler refusal.
  trap 'rm -rf "$stage"' EXIT
  mkdir "$stage/layout"
  fetch_floor_source bubblewrap-0.11.0.tar.xz \
    https://github.com/containers/bubblewrap/releases/download/v0.11.0/bubblewrap-0.11.0.tar.xz \
    988fd6b232dafa04b8b8198723efeaccdb3c6aa9c1c7936219d5791a8b7a8646 "$stage/bubblewrap.tar.xz"
  fetch_floor_source libcap-2.77.tar.xz \
    https://www.kernel.org/pub/linux/libs/security/linux-privs/libcap2/libcap-2.77.tar.xz \
    897bc18b44afc26c70e78cead3dbb31e154acc24bee085a5a09079a88dbf6f52 "$stage/libcap.tar.xz"
  tar -xJf "$stage/bubblewrap.tar.xz" -C "$stage"
  tar -xJf "$stage/libcap.tar.xz" -C "$stage"
  printf '#define PACKAGE_STRING "bubblewrap 0.11.0"\n#define ENABLE_REQUIRE_USERNS 1\n' >"$stage/config.h"
  # Copy real files, never loader/libc symlinks pointing outside this closure.
  copy_from_image linux/arm64 "$FLOOR_IMAGE" /usr/lib64/ld-2.17.so "$stage/layout/ld-linux-aarch64.so.1"
  copy_from_image linux/arm64 "$FLOOR_IMAGE" /usr/lib64/libc-2.17.so "$stage/layout/libc.so.6"
  copy_from_image linux/arm64 "$FLOOR_IMAGE" /usr/lib64/libattr.so.1.1.0 "$stage/layout/libattr.so.1"
  library="$stage/layout/libcap.so.2"
  copy_from_image linux/arm64 "$FLOOR_IMAGE" /usr/lib64/libcap.so.2.22 "$library"
  verify_floor_input "$library" 47c73bce965ec946e14062b123a712df3505d79db5ec242602337e8830d91398
  ln -s layout/libattr.so.1 "$stage/libattr.so"
  # Upstream source/config unchanged; host-native syscalls, exact floor libraries.
  # https://github.com/containers/bubblewrap/blob/v0.11.0/meson.build
  # Link libattr directly so RUNPATH reaches the full dependency closure:
  # https://man7.org/linux/man-pages/man8/ld.so.8.html
  # Zig's emitted RUNPATH is tested with the actual loader in each userland.
  zig build-exe -target aarch64-linux-gnu.2.17 -O ReleaseFast -lc -dynamic \
    -L "$stage" -needed-lattr \
    --dynamic-linker /usr/libexec/floor-backend/ld-linux-aarch64.so.1 \
    -rpath /usr/libexec/floor-backend \
    -I "$stage" -I "$stage/libcap-2.77/libcap/include" \
    -cflags -O2 -g0 -D_GNU_SOURCE "-ffile-prefix-map=$stage=/floor-build" -- \
    "$stage/bubblewrap-0.11.0/bubblewrap.c" "$stage/bubblewrap-0.11.0/bind-mount.c" \
    "$stage/bubblewrap-0.11.0/network.c" "$stage/bubblewrap-0.11.0/utils.c" \
    "$library" "-femit-bin=$stage/layout/bwrap"
  verify_floor_elf aarch64 "$stage/layout/bwrap"
  printf 'zig\t%s\nfloor-image\t%s\n' "$(zig version)" "$FLOOR_IMAGE" >"$stage/layout/inputs.tsv"
  for library in "$stage/layout/"* "$stage/"*.tar.xz; do
    [[ "$library" == */inputs.tsv ]] || shasum -a 256 "$library" | sed "s|$stage/||" >>"$stage/layout/inputs.tsv"
  done
  python3 "$REPO_ROOT/scripts/atomic_directory_publish.py" "$stage/layout" "$output"
  echo "floor-qemu-backend: real native bubblewrap with isolated glibc 2.17 closure at $output"
)
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  [[ $# == 1 ]] || die 'usage: release-floor-qemu-backend.sh OUTPUT'
  prepare_qemu_backend "$1"
fi
