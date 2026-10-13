#!/usr/bin/env bash
# Native-only actual caller acceptance; QEMU support is a separate instrument.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck disable=SC1091 # reusable runner is checked with this file by check.sh
# shellcheck source=scripts/release-floor-backend.sh
source "$root/scripts/release-floor-backend.sh"
[[ $# == 2 || $# == 3 ]] || die 'usage: release-floor-native.sh BACKENDS ARCHIVES [RAVEL_ARCHIVES]'
backends="$(cd "$1" && pwd)"
archives="$(cd "$2" && pwd)"
target=aarch64-unknown-linux-gnu
[[ "$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')" == linux/arm64 ]] ||
  die 'native floor requires arm64 Docker'
[[ "$(zig version)" == 0.17.0 ]] || die 'native floor probes require Zig 0.17.0'
SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
dependency=()
[[ $# != 3 ]] || dependency=(--archives "$(cd "$3" && pwd)")
python3 "$root/scripts/ravel_target_inputs.py" "${dependency[@]+"${dependency[@]}"}" \
  --output "$SCRATCH/ravel" "$target"
# shellcheck disable=SC2034 # used by sourced stage_linux_inputs
RAVEL_INPUTS="$SCRATCH/ravel"
archive="$(find_archive "$archives" "$target")"
assert_archive "$archive" "$(basename "$archive" .tar.xz)" || die 'invalid Grove archive'
mount="$SCRATCH/$target"
# shellcheck disable=SC2034 # used by sourced stage_linux_inputs
CASES="$root/scripts/grove-floor-native-cases.sh"
stage_linux_inputs "$target" "$archive" "$mount"
cp "$root/scripts/grove-installed-smoke.sh" "$mount/"
cp "$backends/$target/bwrap" "$mount/bwrap"
cp "$backends/$target/libcap.so.2" "$mount/libcap.so.2"
verify_floor_elf aarch64 "$mount/bwrap"
verify_floor_input "$mount/bwrap" 05cc65bb55a89c7f6268f786957c4a06146fae7e0e6ea70d31a701d8e6837a76
verify_floor_input "$mount/libcap.so.2" 47c73bce965ec946e14062b123a712df3505d79db5ec242602337e8830d91398
python3 "$root/scripts/floor_seccomp.py" >"$mount/seccomp.json"
zig cc -target aarch64-linux-gnu.2.17 -O2 "$root/scripts/floor-native-security.c" -o "$mount/security"
echo "floor-native: image $FLOOR_IMAGE, Docker $(docker version --format '{{.Server.Version}}'), kernel $(docker info --format '{{.KernelVersion}}'), Zig $(zig version)"
for input in "$archive" "$mount/"* "$mount/probes/"*; do
  [[ ! -f "$input" ]] || shasum -a 256 "$input"
done
find "$mount" -type f -exec shasum -a 256 {} \; | LC_ALL=C sort >"$SCRATCH/inputs-before"
for mode in missing default-seccomp masked-proc supported; do
  options=(--cap-drop ALL --security-opt no-new-privileges)
  [[ "$mode" == default-seccomp ]] || options+=(--security-opt "seccomp=$mount/seccomp.json")
  # Docker documents this option; private procfs cannot mount with masked paths:
  # https://docs.docker.com/reference/cli/docker/container/run/#security-opt
  [[ "$mode" == masked-proc ]] || options+=(--security-opt systempaths=unconfined)
  [[ "$mode" == missing ]] || options+=(--mount "type=bind,source=$mount/bwrap,target=/usr/bin/bwrap,readonly")
  echo "floor-native: $mode: uid/gid 1000, cap-drop ALL, no-new-privileges, network none, read-only inputs"
  docker run --rm --network none --user 1000:1000 --platform linux/arm64 \
    "${options[@]}" --mount "type=bind,source=$mount,target=/smoke,readonly" \
    "$FLOOR_IMAGE" /usr/bin/env -i PATH=/usr/bin:/bin "FLOOR_MODE=$mode" \
    /bin/bash /smoke/release-smoke-target.sh "$target" "/smoke/$(basename "$archive")" \
    /smoke/installed-smoke.sh /smoke/probes
  find "$mount" -type f -exec shasum -a 256 {} \; | LC_ALL=C sort >"$SCRATCH/inputs-after"
  cmp "$SCRATCH/inputs-before" "$SCRATCH/inputs-after" || die 'native staged inputs changed during execution'
done
echo 'floor-native: real native standalone/lifecycle/imports, sandbox denials and three pre-selection refusal controls passed'
