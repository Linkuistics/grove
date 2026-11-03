#!/usr/bin/env bash
# Validate the pinned profile of the QEMU floor route and refuse a changed
# source before Docker runs.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
python3 "$root/scripts/floor_seccomp.py" >"$scratch/profile.json"
python3 - "$root/scripts/floor-seccomp/default.json" "$scratch/profile.json" <<'PY'
import json
import sys
source, p = (json.load(open(path)) for path in sys.argv[1:])
assert p['defaultAction'] == 'SCMP_ACT_ERRNO'
# The pinned rules, then exactly two additions.
assert p['syscalls'][:-2] == source['syscalls']
assert p['syscalls'][-2] == {'names': ['mount', 'pivot_root', 'umount2'], 'action': 'SCMP_ACT_ALLOW'}
assert p['syscalls'][-1] == {'names': ['unshare'], 'action': 'SCMP_ACT_ALLOW', 'args': [{'index': 0, 'value': 0x10020000, 'op': 'SCMP_CMP_EQ'}]}
PY
cp "$root/scripts/floor-seccomp/default.json" "$scratch/changed.json"
printf '\n' >>"$scratch/changed.json"
if python3 "$root/scripts/floor_seccomp.py" "$scratch/changed.json" >"$scratch/refused.json" 2>"$scratch/error"; then
  echo 'floor-seccomp-test: accepted changed upstream source' >&2
  exit 1
fi
grep -q 'differs from pinned' "$scratch/error"
[[ ! -s "$scratch/refused.json" ]]
if python3 "$root/scripts/floor_seccomp.py" "$scratch/missing.json" >"$scratch/refused.json" 2>"$scratch/error"; then
  echo 'floor-seccomp-test: accepted missing upstream source' >&2
  exit 1
fi
[[ ! -s "$scratch/refused.json" ]]
echo 'floor-seccomp-test: pinned derivation and changed/missing source refusals passed'
