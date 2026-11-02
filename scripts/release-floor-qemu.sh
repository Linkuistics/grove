#!/usr/bin/env bash
# Actual separately admitted callers under both pinned automatic exec models.
set -euo pipefail
IFS=$'\n\t'
# shellcheck source=scripts/release-floor-qemu-backend.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/release-floor-qemu-backend.sh"
# --ravel-archives-2.0 DIR holds the recorded archives of the 2.0 lane. With
# it, the floor cases also run the 2.0 lane with config.rhai.
ravel_2_archive_dir=""
if [[ "${1:-}" == --ravel-archives-2.0 ]]; then
  ravel_2_archive_dir="$(cd "${2:?--ravel-archives-2.0 needs a directory}" && pwd)"
  shift 2
fi
[[ $# -ge 2 ]] || die 'usage: release-floor-qemu.sh [--ravel-archives-2.0 DIR] ARCHIVES RAVEL_ARCHIVES [LINUX_TARGET...]'
archives="$(cd "$1" && pwd)"
ravel_archive_dir="$(cd "$2" && pwd)"
shift 2
targets=("$@")
((${#targets[@]})) || targets=(aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu)
for target in "${targets[@]}"; do
  [[ "$target" == aarch64-unknown-linux-gnu || "$target" == x86_64-unknown-linux-gnu ]] || die "unknown QEMU floor target $target"
done
DOCKER_PLATFORM="$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')"
[[ "$DOCKER_PLATFORM" == linux/arm64 ]] || die 'QEMU floor requires arm64 Docker'
[[ "$(zig version)" == 0.17.0 ]] || die 'QEMU floor requires Zig 0.17.0'
SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" --archives "$ravel_archive_dir" \
  --output "$SCRATCH/ravel" "${targets[@]}"
# shellcheck disable=SC2034 # sourced runner reads these globals
RAVEL_INPUTS="$SCRATCH/ravel"
if [[ -n "$ravel_2_archive_dir" ]]; then
  python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" --lane 2.0 --archives "$ravel_2_archive_dir" \
    --output "$SCRATCH/ravel-2.0" "${targets[@]}"
  # shellcheck disable=SC2034
  RAVEL_2_INPUTS="$SCRATCH/ravel-2.0"
fi
# shellcheck disable=SC2034
CASES="$REPO_ROOT/scripts/grove-floor-native-cases.sh"
prepare_qemu_backend "$SCRATCH/native"
# The unusable control is real target bubblewrap, executing under QEMU.
bash "$REPO_ROOT/scripts/release-floor-backend.sh" "$SCRATCH/backends" "${targets[@]}"
for target in "${targets[@]}"; do
  archive="$(find_archive "$archives" "$target")"
  assert_archive "$archive" "$(basename "$archive" .tar.xz)" || die "invalid Grove archive $archive"
  mount="$SCRATCH/$target"
  stage_linux_inputs "$target" "$archive" "$mount"
  cp "$REPO_ROOT/scripts/grove-installed-smoke.sh" "$REPO_ROOT/scripts/grove-floor-directory-case.sh" "$mount/"
  cp -a "$SCRATCH/native" "$mount/"
  cp "$SCRATCH/backends/$target/bwrap" "$mount/bwrap"
  cp "$SCRATCH/backends/$target/libcap.so.2" "$mount/libcap.so.2"
  zig cc -target aarch64-linux-musl -static -O2 -Wall -Wextra -Werror \
    "$REPO_ROOT/scripts/floor-native-security.c" -o "$mount/security"
  for mode in missing qemu-unusable supported; do
    echo "floor-qemu: $target: $mode: private binfmt, filtered seccomp, uid1000/no-new-privileges/no bounding caps, network none"
    # shellcheck disable=SC2034 # read by sourced smoke_under_qemu
    FLOOR_MODE="$mode"
    smoke_under_qemu "$target" "$archive" "$(docker_platform "$target")" "$mount"
  done
done
lanes='Ravel 1.1'
[[ -z "$ravel_2_archive_dir" ]] || lanes='Ravel 1.1 and 2.0, with the 2.0 confined directory case'
echo "floor-qemu: actual caller lifecycle/imports/publication/denials and two backend refusal controls passed for $lanes: $(IFS=' ' && echo "${targets[*]}")"
