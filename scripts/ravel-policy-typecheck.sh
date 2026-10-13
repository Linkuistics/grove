#!/usr/bin/env bash
# Typecheck Grove-readable policies against an explicit installed Ravel SDK.
# Reuse the pinned development compiler; no Ravel checkout/build is required.
set -euo pipefail
IFS=$'\n\t'

main() {
  local ravel="${1:?usage: ravel-policy-typecheck.sh /absolute/path/to/ravel}"
  local root worker scratch cleanup
  root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  worker="$root/crates/harness-dispatch/worker"
  [[ "$(bun --version)" == 1.4.2 ]] || {
    echo 'policy typecheck: Bun 1.4.2 is pinned; install it and retry' >&2
    return 1
  }
  [[ -f "$worker/node_modules/typescript/bin/tsc" ]] || {
    echo 'policy typecheck: run task dispatch:deps to install the pinned compiler' >&2
    return 1
  }
  scratch="$(mktemp -d)"
  printf -v cleanup 'rm -rf -- %q' "$scratch"
  # shellcheck disable=SC2064 # freeze the safely quoted path before main returns
  trap "$cleanup" EXIT
  # shellcheck disable=SC2016 # this is JavaScript; interpolation belongs to Bun
  bun -e '
    const { realpathSync, existsSync, writeFileSync } = require("node:fs");
    const { dirname, join } = require("node:path");
    const [ravel, root, scratch] = process.argv.slice(1);
    const prefix = dirname(dirname(realpathSync(ravel)));
    const sdk = join(prefix, "libexec/ravel/sdk/index.d.ts");
    const example = join(prefix, "libexec/ravel/examples/static.d.ts");
    for (const file of [sdk, example]) {
      if (!existsSync(file)) throw new Error(`missing installed Ravel declarations: ${file}`);
    }
    const worker = join(root, "crates/harness-dispatch/worker");
    writeFileSync(join(scratch, "tsconfig.json"), JSON.stringify({
      extends: join(worker, "tsconfig.json"),
      compilerOptions: {
        typeRoots: [join(worker, "node_modules/@types")],
        paths: { "ravel/sdk": [sdk], "ravel/examples/static": [example] }
      },
      include: [join(root, "assets/ravel-policy/**/*.ts")]
    }));
  ' "$ravel" "$root" "$scratch"
  bun "$worker/node_modules/typescript/bin/tsc" -p "$scratch/tsconfig.json"
  echo 'policy typecheck: Grove assets and import fixtures passed against the installed Ravel SDK'
}

main "$@"
