#!/usr/bin/env bash
# Pinned independent Ravel inputs through Grove's existing platform/floor seams.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-smoke.sh
source "$repo_root/scripts/release-smoke.sh"
TARGET_SCRIPT="$REPO_ROOT/scripts/ravel-target-smoke-target.sh"
CASES="$REPO_ROOT/scripts/ravel-installed-smoke.sh"

ravel_smoke_main() {
  local archives=() targets=() target archive
  if [[ "${1:-}" == --archives ]]; then
    [[ $# -ge 2 ]] || die '--archives requires a directory'
    archives=(--archives "$2")
    shift 2
  fi
  targets=("$@")
  ((${#targets[@]} > 0)) || targets=("${TARGETS[@]}")
  SCRATCH="$(mktemp -d)"
  trap 'rm -rf "$SCRATCH"' EXIT
  # Admission happens before Docker/probe setup or archive extraction/invocation.
  python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" "${archives[@]+"${archives[@]}"}" \
    --output "$SCRATCH/inputs" "${targets[@]}"
  for target in "${targets[@]}"; do
    [[ "$target" == *-linux-gnu ]] || continue
    command -v zig >/dev/null 2>&1 || die 'zig is required for floor probes'
    docker info >/dev/null 2>&1 || die 'Docker is required for Linux targets and is not running'
    DOCKER_PLATFORM="$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')"
    [[ "$DOCKER_PLATFORM" == linux/arm64 ]] || die "Linux floor checks need arm64 Docker; got $DOCKER_PLATFORM"
    echo "ravel-smoke: $FLOOR_IMAGE; Docker $(docker version --format '{{.Server.Version}}'); Zig $(zig version)"
    break
  done
  for target in "${targets[@]}"; do
    archive="$SCRATCH/inputs/archives/ravel-v0.1.0-$target.tar.xz"
    case "$target" in
      *-apple-darwin) smoke_native "$target" "$archive" ;;
      *-linux-gnu) smoke_linux "$target" "$archive" ;;
    esac || die "$target: Ravel installed smoke failed"
  done
  echo "ravel-smoke: passed: $(IFS=' ' && echo "${targets[*]}")"
}
ravel_smoke_main "$@"
