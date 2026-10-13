#!/usr/bin/env bash
# Validate the pinned profile and refuse a changed source before Docker runs.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
python3 "$root/scripts/floor_seccomp.py" >"$scratch/profile.json"
python3 - "$scratch/profile.json" <<'PY'
import json
import sys
p = json.load(open(sys.argv[1]))
assert p['defaultAction'] == 'SCMP_ACT_ERRNO'
assert p['syscalls'][-3]['args'][0]['value'] == 0x3c020011
assert p['syscalls'][-2]['args'][0]['value'] == 0x10000000
assert p['syscalls'][-1]['names'] == ['mount', 'pivot_root', 'umount2']
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
python3 "$root/scripts/floor_seccomp.py" --qemu >"$scratch/qemu.json"
python3 - "$scratch/profile.json" "$scratch/qemu.json" <<'PY'
import json, sys
native, qemu = (json.load(open(p)) for p in sys.argv[1:])
assert qemu['defaultAction'] == native['defaultAction'] == 'SCMP_ACT_ERRNO'
assert qemu['syscalls'][:-1] == native['syscalls']
assert qemu['syscalls'][-1] == {'names': ['unshare'], 'action': 'SCMP_ACT_ALLOW', 'args': [{'index': 0, 'value': 0x10020000, 'op': 'SCMP_CMP_EQ'}]}
PY
