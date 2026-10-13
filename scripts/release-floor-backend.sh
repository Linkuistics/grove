#!/usr/bin/env bash
# Test-only target bubblewrap; never added to a production archive/install.
set -euo pipefail
IFS=$'\n\t'
# Reuse the pinned CentOS platforms and image-copy mechanics without running it.
# shellcheck source=scripts/release-smoke.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/release-smoke.sh"

verify_floor_input() {
  local file="$1" expected="$2"
  [[ -f "$file" ]] || die "missing floor input $file; refusing it"
  [[ "$(sha256_of "$file")" == "$expected" ]] || die "floor input $file differs from SHA-256 $expected; refusing it"
}

verify_floor_elf() {
  local arch="$1" file="$2" machine header
  case "$arch" in
    aarch64) machine=b700 ;;
    x86_64) machine=3e00 ;;
    *) die "unknown backend architecture $arch" ;;
  esac
  header="$(od -An -tx1 -N20 "$file" | tr -d ' \n')"
  # Same ELF64/LE/padding requirements as the existing binfmt registrations.
  [[ "$header" == "7f454c460201010000000000000000000200$machine" ]] ||
    die "wrong ELF architecture/type/padding for $arch in $file; refusing it"
}

fetch_floor_source() {
  local name="$1" url="$2" digest="$3" dest="$4" cache="$REPO_ROOT/target/floor-sources" download
  mkdir -p "$cache"
  if [[ -e "$cache/$name" ]]; then
    cp "$cache/$name" "$dest"
    verify_floor_input "$dest" "$digest"
    return
  fi
  download="$(mktemp "$cache/.download.XXXXXX")"
  if ! curl --fail --silent --show-error --location --output "$download" "$url"; then
    rm -f "$download"
    die "cannot fetch $url; refusing floor source"
  fi
  cp "$download" "$dest"
  rm -f "$download"
  verify_floor_input "$dest" "$digest"
  cp "$dest" "$cache/$name"
}

prepare_floor_backends() {
  (($# >= 1)) || die 'usage: release-floor-backend.sh OUTPUT [LINUX_TARGET...]'
  local output="$1" targets=() target arch library digest
  shift
  [[ ! -e "$output" && ! -L "$output" ]] || die "output already exists: $output"
  targets=("$@")
  ((${#targets[@]})) || targets=(aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu)
  for target in "${targets[@]}"; do
    case "$target" in
      aarch64-unknown-linux-gnu | x86_64-unknown-linux-gnu) ;;
      *) die "unknown floor target: $target" ;;
    esac
  done
  # Resolve after parent creation, so absent/../existing cannot pass admission.
  mkdir -p "$(dirname "$output")"
  output="$(cd "$(dirname "$output")" && pwd -P)/$(basename "$output")"
  [[ ! -e "$output" && ! -L "$output" ]] || die "output already exists: $output"
  [[ "$(zig version)" == 0.17.0 ]] || die 'floor backend requires Zig 0.17.0'
  docker info >/dev/null || die 'Docker is required to extract pinned floor libcap'
  # A sibling stage makes publication a same-filesystem rename, never a copy.
  SCRATCH="$(mktemp -d "$(dirname "$output")/.floor-backend.XXXXXX")"
  trap 'rm -rf "$SCRATCH"' EXIT
  mkdir "$SCRATCH/layout"
  fetch_floor_source bubblewrap-0.11.0.tar.xz \
    https://github.com/containers/bubblewrap/releases/download/v0.11.0/bubblewrap-0.11.0.tar.xz \
    988fd6b232dafa04b8b8198723efeaccdb3c6aa9c1c7936219d5791a8b7a8646 "$SCRATCH/bubblewrap.tar.xz"
  fetch_floor_source libcap-2.77.tar.xz \
    https://www.kernel.org/pub/linux/libs/security/linux-privs/libcap2/libcap-2.77.tar.xz \
    897bc18b44afc26c70e78cead3dbb31e154acc24bee085a5a09079a88dbf6f52 "$SCRATCH/libcap.tar.xz"
  tar -xJf "$SCRATCH/bubblewrap.tar.xz" -C "$SCRATCH"
  tar -xJf "$SCRATCH/libcap.tar.xz" -C "$SCRATCH"
  # Match upstream's four source files/config: libcap required, SELinux disabled,
  # user namespaces required. Header-only libcap input; runtime is floor 2.22.
  # https://github.com/containers/bubblewrap/blob/v0.11.0/meson.build
  printf '#define PACKAGE_STRING "bubblewrap 0.11.0"\n#define ENABLE_REQUIRE_USERNS 1\n' >"$SCRATCH/config.h"
  printf 'zig\t%s\nfloor-image\t%s\n' "$(zig version)" "$FLOOR_IMAGE" >"$SCRATCH/layout/inputs.tsv"
  shasum -a 256 "$SCRATCH/bubblewrap.tar.xz" "$SCRATCH/libcap.tar.xz" | \
    sed "s|$SCRATCH/||" >>"$SCRATCH/layout/inputs.tsv"
  for target in "${targets[@]}"; do
    arch="${target%%-*}"
    mkdir "$SCRATCH/layout/$target"
    library="$SCRATCH/layout/$target/libcap.so.2"
    copy_from_image "$(docker_platform "$target")" "$FLOOR_IMAGE" /usr/lib64/libcap.so.2.22 "$library"
    case "$arch" in
      aarch64) digest=47c73bce965ec946e14062b123a712df3505d79db5ec242602337e8830d91398 ;;
      x86_64) digest=79733b599d1c3a2415ff5575b49e4521e5db7d6eefaa40986aadf3e644404fc6 ;;
    esac
    verify_floor_input "$library" "$digest"
    zig cc -target "$arch-linux-gnu.2.17" -O2 -g0 -D_GNU_SOURCE \
      "-ffile-prefix-map=$SCRATCH=/floor-build" \
      -I "$SCRATCH" -I "$SCRATCH/libcap-2.77/libcap/include" \
      "$SCRATCH/bubblewrap-0.11.0/bubblewrap.c" "$SCRATCH/bubblewrap-0.11.0/bind-mount.c" \
      "$SCRATCH/bubblewrap-0.11.0/network.c" "$SCRATCH/bubblewrap-0.11.0/utils.c" \
      "$library" -o "$SCRATCH/layout/$target/bwrap"
    verify_floor_elf "$arch" "$SCRATCH/layout/$target/bwrap"
    shasum -a 256 "$SCRATCH/layout/$target/bwrap" "$library" | \
      sed "s|$SCRATCH/layout/||" >>"$SCRATCH/layout/inputs.tsv"
  done
  python3 "$REPO_ROOT/scripts/atomic_directory_publish.py" "$SCRATCH/layout" "$output"
  echo "floor-backend: prepared ${#targets[@]} real bubblewrap 0.11.0 backends at $output"
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  prepare_floor_backends "$@"
fi
