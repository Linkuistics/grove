#!/usr/bin/env bash
# Recorded independent Ravel inputs of both runtime lanes through Grove's
# existing platform/floor seams.
#
#   ravel-target-smoke.sh [--archives DIR] [--archives-2.0 DIR] [TARGET...]
#
# The 1.1 lane runs the published 1.1.0 archives with `policy.ts`. The 2.0 lane
# runs the recorded 2.0 archives with `config.rhai`. Each lane has its own
# cases script and its own scratch tree. A target that a lane does not record
# is named with the reason, and the run does not count it as passed.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-smoke.sh
source "$repo_root/scripts/release-smoke.sh"
TARGET_SCRIPT="$REPO_ROOT/scripts/ravel-target-smoke-target.sh"

lane_cases() {
  case "$1" in
    1.1) echo "$REPO_ROOT/scripts/ravel-installed-smoke.sh" ;;
    2.0) echo "$REPO_ROOT/scripts/ravel-installed-smoke-rhai.sh" ;;
    *) die "unknown lane $1" ;;
  esac
}

ravel_smoke_main() {
  local archives_1=() archives_2=() targets=() target state reason lane archive root
  local run_1=() run_2=() not_run=() passed=()
  while [[ "${1:-}" == --archives || "${1:-}" == --archives-2.0 ]]; do
    [[ $# -ge 2 ]] || die "$1 requires a directory"
    if [[ "$1" == --archives ]]; then archives_1=(--archives "$2"); else archives_2=(--archives "$2"); fi
    shift 2
  done
  targets=("$@")
  ((${#targets[@]} > 0)) || targets=("${TARGETS[@]}")
  root="$(mktemp -d)"
  # shellcheck disable=SC2064 # remove precisely this owned directory
  trap "rm -rf '$root'" EXIT
  run_1=("${targets[@]}")
  while IFS=$'\t' read -r target state reason; do
    if [[ "$state" == recorded ]]; then
      run_2+=("$target")
    else
      not_run+=("lane 2.0 $target: $reason")
    fi
  done < <(python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" --lane 2.0 --status "${targets[@]}")
  # Admission of both lanes happens before Docker/probe setup or any archive
  # extraction or invocation.
  python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" "${archives_1[@]+"${archives_1[@]}"}" \
    --output "$root/lane-1.1/inputs" "${run_1[@]}"
  if ((${#run_2[@]} > 0)); then
    python3 "$REPO_ROOT/scripts/ravel_target_inputs.py" --lane 2.0 "${archives_2[@]+"${archives_2[@]}"}" \
      --output "$root/lane-2.0/inputs" "${run_2[@]}"
  fi
  for target in "${run_1[@]}" "${run_2[@]+"${run_2[@]}"}"; do
    [[ "$target" == *-linux-gnu ]] || continue
    command -v zig >/dev/null 2>&1 || die 'zig is required for floor probes'
    docker info >/dev/null 2>&1 || die 'Docker is required for Linux targets and is not running'
    DOCKER_PLATFORM="$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')"
    [[ "$DOCKER_PLATFORM" == linux/arm64 ]] || die "Linux floor checks need arm64 Docker; got $DOCKER_PLATFORM"
    echo "ravel-smoke: $FLOOR_IMAGE; Docker $(docker version --format '{{.Server.Version}}'); Zig $(zig version)"
    break
  done
  for lane in 1.1 2.0; do
    local lane_targets=()
    if [[ "$lane" == 1.1 ]]; then
      lane_targets=("${run_1[@]}")
    else
      lane_targets=("${run_2[@]+"${run_2[@]}"}")
    fi
    # The sourced runner reads CASES and stages under SCRATCH, one tree a lane.
    CASES="$(lane_cases "$lane")"
    SCRATCH="$root/lane-$lane"
    for target in "${lane_targets[@]+"${lane_targets[@]}"}"; do
      archive="$(find "$SCRATCH/inputs/archives" -name "ravel-v*-$target.tar.xz")"
      echo "ravel-smoke: lane $lane: $target: $(basename "$archive") with $(basename "$CASES")"
      case "$target" in
        *-apple-darwin) smoke_native "$target" "$archive" ;;
        *-linux-gnu) smoke_linux "$target" "$archive" ;;
      esac || die "lane $lane: $target: Ravel installed smoke failed"
      passed+=("lane $lane $target")
    done
  done
  echo "ravel-smoke: passed: $(IFS=',' && echo "${passed[*]}")"
  for reason in "${not_run[@]+"${not_run[@]}"}"; do
    echo "ravel-smoke: not run: $reason"
  done
}
ravel_smoke_main "$@"
