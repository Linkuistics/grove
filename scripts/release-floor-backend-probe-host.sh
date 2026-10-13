#!/usr/bin/env bash
# Execute prepared backends through the existing native/glibc/CPU instruments.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-floor-backend.sh
source "$repo_root/scripts/release-floor-backend.sh"
[[ $# == 1 ]] || die 'usage: release-floor-backend-probe-host.sh BACKENDS'
backends="$(cd "$1" && pwd)"
[[ "$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')" == linux/arm64 ]] ||
  die 'existing floor QEMU instrument requires arm64 Docker'
DOCKER_PLATFORM=linux/arm64
TARGET_SCRIPT="$REPO_ROOT/scripts/ravel-target-smoke-target.sh"
CASES="$REPO_ROOT/scripts/release-floor-backend-probe.sh"
SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" --output "$SCRATCH/ravel" \
  aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
for target in aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu; do
  archive="$SCRATCH/ravel/archives/ravel-v0.1.0-$target.tar.xz"
  mount="$SCRATCH/$target"
  platform="$(docker_platform "$target")"
  stage_linux_inputs "$target" "$archive" "$mount"
  cp "$backends/$target/bwrap" "$mount/bwrap"
  verify_floor_elf "${target%%-*}" "$mount/bwrap"
  cp "$backends/$target/libcap.so.2" "$mount/libcap.so.2"
  [[ "$platform" != "$DOCKER_PLATFORM" ]] || smoke_in_floor_container "$target" "$archive" "$platform" "$mount"
  smoke_under_qemu "$target" "$archive" "$platform" "$mount"
done
echo 'floor-backend-probe: both real target backends execute; see individual namespace outcomes above'
