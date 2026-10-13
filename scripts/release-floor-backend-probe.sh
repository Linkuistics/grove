#!/usr/bin/env bash
# In-target backend execution diagnostic, NOT standalone acceptance.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "floor-backend-probe: $*" >&2; exit 1; }
[[ $# == 2 ]] || fail 'expected an independently admitted Ravel prefix and version'
[[ "$("$1/bin/ravel" --version)" == "ravel $2" ]] || fail 'separate Ravel version failed'
[[ "$(/smoke/bwrap --version)" == 'bubblewrap 0.11.0' ]] || fail 'real backend version failed'
cmp /smoke/libcap.so.2 /usr/lib64/libcap.so.2.22 || fail 'floor libcap differs from build input'
status=0
output="$(/smoke/bwrap --unshare-user --unshare-pid --unshare-ipc --unshare-uts \
  --ro-bind /usr /usr --ro-bind /lib64 /lib64 --proc /proc --dev /dev \
  -- /usr/bin/true 2>&1)" || status=$?
if ((status == 0)); then
  echo 'floor-backend-probe: real namespace probe succeeded; full standalone acceptance still required'
elif [[ "$output" == *namespace* && ("$output" == *'Operation not permitted'* ||
  "$output" == *'No permissions'* || "$output" == *'Invalid argument'* ||
  "$output" == *'Creating new namespace failed, likely because the kernel does not support user namespaces.'*) ]]; then
  echo "floor-backend-probe: backend executes, environment REFUSES namespaces (exit $status): $output"
  echo 'floor-backend-probe: standalone confinement is NOT accepted by this diagnostic'
else
  fail "unexpected namespace probe failure (exit $status): $output"
fi
