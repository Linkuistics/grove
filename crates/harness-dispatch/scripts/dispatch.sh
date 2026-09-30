#!/usr/bin/env bash
#
# Build, type-check and install harness-dispatch's compiled policy worker.
# The Taskfile's `dispatch:*` tasks are the entry points; this script owns the
# mechanics so that an extraction of `crates/harness-dispatch` takes them along.
#
#   dispatch.sh build-id            print the worker source digest
#   dispatch.sh build [OUT_DIR]     compile the worker, and the declarations and
#                                   readable sources of its SDK and examples,
#                                   into OUT_DIR (default: the checkout's
#                                   target/libexec/harness-dispatch)
#   dispatch.sh typecheck [OUT_DIR] type-check the worker, the SDK, the examples
#                                   and the fixtures against OUT_DIR's
#                                   declarations
#   dispatch.sh install PREFIX      build the release pair and install it as
#                                   PREFIX/bin and PREFIX/libexec/harness-dispatch
#
# WHY THE WORKER CARRIES A SOURCE DIGEST. The worker builds only through this
# script, never from cargo, so nothing orders a `cargo test` after a worker
# rebuild. Instead the front's build.rs computes the same digest over the same
# files and refuses a worker that reports another one (exit 5, naming
# `task dispatch:worker`). The two implementations must agree byte for byte:
# the sorted `shasum -a 256` listing of `source_files`, hashed again. A
# disagreement fails every command-seam test loudly, so it cannot drift unseen.

set -euo pipefail
IFS=$'\n\t'

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly CRATE_DIR
readonly WORKER_DIR="$CRATE_DIR/worker"
readonly BUN_VERSION="1.4.2"
readonly WORKER_NAME="harness-dispatch-policy"

die() {
  echo "dispatch: $*" >&2
  exit 1
}

sha256() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$@"
  else
    sha256sum "$@"
  fi
}

# The files whose bytes decide the compiled worker, relative to the crate.
# build.rs enumerates the same set; change both together.
source_files() {
  (
    cd "$CRATE_DIR"
    find worker/src worker/sdk worker/examples -type f -name '*.ts'
    printf '%s\n' worker/tsconfig.json worker/tsconfig.declarations.json scripts/dispatch.sh
  ) | LC_ALL=C sort
}

build_id() {
  local file
  (
    cd "$CRATE_DIR"
    source_files | while IFS= read -r file; do sha256 "$file"; done
  ) | sha256 | cut -d' ' -f1
}

require_bun() {
  command -v bun >/dev/null 2>&1 ||
    die "bun $BUN_VERSION is required to build the worker; install it (https://bun.sh/docs/installation) and retry"
  local found
  found="$(bun --version)"
  [[ "$found" == "$BUN_VERSION" ]] ||
    die "bun $BUN_VERSION is pinned, found $found; install bun $BUN_VERSION (upgrades rerun the runtime-evidence probes first)"
}

require_type_checker() {
  [[ -f "$WORKER_DIR/node_modules/typescript/bin/tsc" ]] ||
    die "the pinned type checker is not installed; run 'task dispatch:deps'"
}

tsc() {
  bun "$WORKER_DIR/node_modules/typescript/bin/tsc" "$@"
}

# `cargo metadata` names the target directory and the workspace version; bun,
# already required, reads the JSON.
metadata_field() {
  cargo metadata --format-version 1 --no-deps --manifest-path "$CRATE_DIR/Cargo.toml" |
    bun -e "
      const metadata = JSON.parse(await Bun.stdin.text());
      const field = process.argv[1];
      if (field === 'target') console.log(metadata.target_directory);
      else console.log(metadata.packages.find(p => p.name === 'harness-dispatch').version);
    " "$1"
}

default_out_dir() {
  echo "$(metadata_field target)/libexec/harness-dispatch"
}

build() {
  local out_dir="${1:-$(default_out_dir)}"
  require_bun
  require_type_checker
  local id version
  id="$(build_id)"
  version="$(metadata_field version)"
  mkdir -p "$out_dir/sdk" "$out_dir/examples"

  # All four no-autoload switches, stated even where 1.4.2's default already
  # agrees, so that a Bun upgrade changing a default cannot change the build.
  # Bun 1.4.2 leaves a copy of its ~60 MB compile template in its cwd after
  # every compile, so it runs in a throwaway directory rather than the source
  # tree, where jj would try to snapshot the copy.
  (
    scratch="$(mktemp -d)"
    # shellcheck disable=SC2064 # expand now: the trap must remove this directory
    trap "rm -rf '$scratch'" EXIT
    cd "$scratch"
    bun build --compile \
      --no-compile-autoload-dotenv \
      --no-compile-autoload-bunfig \
      --no-compile-autoload-tsconfig \
      --no-compile-autoload-package-json \
      --define "HARNESS_DISPATCH_BUILD_ID=\"$id\"" \
      --define "HARNESS_DISPATCH_PACKAGE_VERSION=\"$version\"" \
      "$WORKER_DIR/src/main.ts" --outfile "$out_dir/$WORKER_NAME"
  )

  # The declarations and readable sources an owner's editor reads, as sdk/
  # and examples/ beside the worker; the worker carries its own embedded copy.
  # An example imports `harness-dispatch/sdk` as an owner's policy does, and
  # its declarations keep that specifier.
  tsc -p "$WORKER_DIR/tsconfig.declarations.json" --outDir "$out_dir"
  cp "$WORKER_DIR/sdk/index.ts" "$out_dir/sdk/index.ts"
  cp "$WORKER_DIR"/examples/*.ts "$out_dir/examples/"
  echo "dispatch: worker $version ($id) in $out_dir"
}

typecheck() {
  local out_dir="${1:-$(default_out_dir)}"
  require_bun
  require_type_checker
  [[ -f "$out_dir/sdk/index.d.ts" ]] ||
    die "no SDK declarations in $out_dir; run 'task dispatch:worker'"
  tsc -p "$WORKER_DIR/tsconfig.json"

  # Fixtures import `harness-dispatch/sdk` and the examples as an owner's
  # policy does, resolved to the shipped declarations rather than the sources.
  local scratch
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$scratch'" EXIT
  cat >"$scratch/tsconfig.json" <<EOF
{
  "extends": "$WORKER_DIR/tsconfig.json",
  "compilerOptions": {
    "types": [],
    "paths": {
      "harness-dispatch/sdk": ["$out_dir/sdk/index.d.ts"],
      "harness-dispatch/examples/*": ["$out_dir/examples/*.d.ts"]
    }
  },
  "include": ["$WORKER_DIR/typecheck/*.ts"]
}
EOF
  tsc -p "$scratch/tsconfig.json"
  echo "dispatch: worker, SDK and fixtures type-check"
}

install_pair() {
  local prefix="${1:?install needs a PREFIX}"
  require_bun
  require_type_checker
  cargo build --release --locked --manifest-path "$CRATE_DIR/Cargo.toml" -p harness-dispatch
  build "$prefix/libexec/harness-dispatch"
  mkdir -p "$prefix/bin"
  install -m 0755 "$(metadata_field target)/release/harness-dispatch" "$prefix/bin/harness-dispatch"

  # The installed front must find and accept the installed worker: inspect a
  # throwaway explicit policy through it.
  local scratch
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$scratch'" EXIT
  cat >"$scratch/policy.ts" <<'EOF'
export const policy = {
  schemaVersion: 1,
  version: "install-check",
  catalog: [{ id: "check", provider: "check", model: "check", effort: "check", program: "true", args: [{ slot: "prompt" }] }],
  routes: { check: "check" },
};
EOF
  "$prefix/bin/harness-dispatch" inspect --kind check --config "$scratch/policy.ts" >/dev/null
  echo "dispatch: installed $("$prefix/bin/harness-dispatch" --version) under $prefix"
}

main() {
  local command="${1:-}"
  shift || true
  case "$command" in
    build-id) build_id ;;
    build) build "$@" ;;
    typecheck) typecheck "$@" ;;
    install) install_pair "$@" ;;
    *) die "usage: dispatch.sh build-id | build [OUT_DIR] | typecheck [OUT_DIR] | install PREFIX" ;;
  esac
}

main "$@"
