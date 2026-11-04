#!/usr/bin/env bash
# Run the Bun version that the repository's mise.toml pins, with the given
# arguments. mise's shims and hook select the pin inside the repository. This
# helper also selects it from a noninteractive shell, a scratch copy outside
# the repository, or a PATH whose first Bun is the machine's global version.
set -euo pipefail
IFS=$'\n\t'

die() {
  echo "pinned bun: $*" >&2
  exit 1
}

main() {
  local root config pin bin found
  root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  config="$root/mise.toml"
  pin="$(sed -n 's/^bun = "\([^"]*\)"$/\1/p' "$config" 2>/dev/null || true)"
  [[ -n "$pin" ]] || die "no bun pin in $config"

  bin="$(command -v bun || true)"
  found="$([[ -n "$bin" ]] && "$bin" --version 2>/dev/null || echo none)"
  if [[ "$found" != "$pin" ]]; then
    command -v mise >/dev/null 2>&1 ||
      die "Bun $pin is pinned in mise.toml, PATH offers $found, and mise is absent; put Bun $pin first on PATH"
    # An installed version is a no-op. A missing one is fetched by version.
    mise install --quiet "bun@$pin" >&2 || die "mise could not install bun@$pin"
    bin="$(mise where "bun@$pin")/bin/bun"
    found="$("$bin" --version 2>/dev/null || echo none)"
    [[ "$found" == "$pin" ]] || die "mise's bun@$pin at $bin reports $found"
  fi
  # Bun subprocesses that call `bun` by name get the same version.
  PATH="$(dirname "$bin"):$PATH"
  export PATH
  exec "$bin" "$@"
}

main "$@"
