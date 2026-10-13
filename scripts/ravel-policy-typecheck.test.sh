#!/usr/bin/env bash
# Catch accidental dependence on the old dispatch source/compiler, narrowing of
# policy includes, or fallback from the explicitly supplied installed SDK.
set -euo pipefail
IFS=$'\n\t'

main() {
  local root ravel scratch cleanup
  root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  ravel="${GROVE_TEST_RAVEL:?Set GROVE_TEST_RAVEL to an installed Ravel front}"
  scratch="$(mktemp -d)"
  printf -v cleanup 'rm -rf -- %q' "$scratch"
  # shellcheck disable=SC2064 # freeze the safely quoted path before main returns
  trap "$cleanup" EXIT
  # Spaces exercise the generated tsconfig and task argument boundaries too.
  local fragment="$scratch/Grove fragment"
  mkdir -p "$fragment/scripts" "$fragment/tools/ravel-policy" "$fragment/assets"
  cp "$root/Taskfile.yml" "$fragment/"
  cp "$root/scripts/ravel-policy-typecheck.sh" "$fragment/scripts/"
  cp "$root/tools/ravel-policy/"{package.json,bun.lock,tsconfig.json} "$fragment/tools/ravel-policy/"
  cp -R "$root/assets/ravel-policy" "$fragment/assets/"
  # No crates directory or old generated declarations exist in this fragment.
  task --dir "$fragment" ravel:policy:typecheck "RAVEL=$ravel"

  # Real compiler rejection proves that the asset includes have not gone empty.
  printf 'export const invalid: string = 42;\n' >"$fragment/assets/ravel-policy/typecheck/invalid.ts"
  if task --dir "$fragment" ravel:policy:typecheck "RAVEL=$ravel" >"$scratch/invalid.log" 2>&1; then
    echo 'policy compiler suite: invalid policy type passed' >&2
    return 1
  fi
  grep -Fq 'TS2322' "$scratch/invalid.log" || {
    cat "$scratch/invalid.log" >&2
    return 1
  }
  rm "$fragment/assets/ravel-policy/typecheck/invalid.ts"

  # A front in another prefix with no declarations must not borrow the real SDK.
  mkdir -p "$scratch/no-sdk/bin"
  printf '#!/bin/sh\nexit 0\n' >"$scratch/no-sdk/bin/ravel"
  chmod +x "$scratch/no-sdk/bin/ravel"
  if task --dir "$fragment" ravel:policy:typecheck "RAVEL=$scratch/no-sdk/bin/ravel" >"$scratch/no-sdk.log" 2>&1; then
    echo 'policy compiler suite: absent installed SDK passed' >&2
    return 1
  fi
  grep -Fq 'missing installed Ravel declarations:' "$scratch/no-sdk.log" || {
    cat "$scratch/no-sdk.log" >&2
    return 1
  }
  echo 'policy compiler suite: source isolation, type rejection and absent-SDK refusal passed'
}

main "$@"
