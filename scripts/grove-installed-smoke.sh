#!/usr/bin/env bash
# Installed Grove callers, readable assets and separate public Ravel dependencies.
# Bash 3.2/system tools only, including in the Linux floor userland.
#
#   grove-installed-smoke.sh grove-prefix version ravel-1.1-prefix [ravel-2.0-prefix]
#
# The position of a Ravel prefix names its runtime, because a build of Ravel's
# 2.0 feature line reports `ravel 1.1.0` until the cut. The 1.1 lane checks the
# TypeScript set with `policy.ts`. The 2.0 lane checks the Rhai set with
# `config.rhai` and SCHEMA_VERSION 4. Each lane runs in its own HOME. A `-` in
# the third position skips the 1.1 lane, for a host whose only Ravel is 2.x.
#
# The 2.0 lane also reads the keg's skill sources. It copies the 2.0 front into
# a private prefix whose installation record names this Grove prefix's
# share/ravel/skills. With no settings, `ravel skills resolve --json` must
# report exactly core and the five domain packs, each with its originPath in
# this prefix. That is the "Installed distribution" row of the test seams in
# docs/specs/codex-skill-provisioning.md.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "grove-installed-smoke: $*" >&2; exit 1; }
[[ $# == 3 || $# == 4 ]] ||
  fail 'usage: grove-installed-smoke.sh grove-prefix version ravel-1.1-prefix [ravel-2.0-prefix]'
[[ "$3" != - || $# == 4 ]] || fail 'name at least one Ravel prefix'
prefix="$(cd "$1" && pwd -P)"
version="$2"
[[ "$version" != 1.1.0 && "$version" != 2.0.0 ]] || fail 'fixture must prove different project versions'
for command in grove grove-llm; do
  [[ "$("$prefix/bin/$command" --version)" == "$command $version" ]] || fail "$command version mismatch"
done
assets="$prefix/libexec/ravel-policy"
for asset in README.md grove.ts provenance.ts examples/grove-static.ts examples/grove-review.ts examples/review.ts \
  sample/policy.ts typecheck/static.ts typecheck/review.ts grove.rhai provenance.rhai examples/grove-static.rhai \
  examples/grove-review.rhai examples/review.rhai examples/static.rhai sample/config.rhai; do
  [[ -f "$assets/$asset" ]] || fail "missing caller asset $asset"
done
# No Ravel front, private worker, dispatcher or skill payload ships with Grove.
for retired in bin/ravel bin/harness-dispatch libexec/ravel libexec/harness-dispatch libexec/skills plugins; do
  [[ ! -e "$prefix/$retired" ]] || fail "Grove bundles $retired"
done
scratch="$(mktemp -d)"
# Ravel publishes each skill snapshot read-only.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT
scratch="$(cd "$scratch" && pwd -P)"
# The fake agent closes the lifecycle through its own launch-directory marker;
# it does not retire work or run a second real driver.
cat > "$scratch/agent.sh" <<'EOF'
#!/bin/sh
printf 'managed-noise\n'
case "$1" in
  impl) touch "$GROVE_LAUNCH_DIR/teardown" ;;
  check) printf published > published.txt ;;
  *) exit 91 ;;
esac
exec "$2" end-session grove-continue
EOF
chmod +x "$scratch/agent.sh"

# The owner configuration of RUNTIME for the Ravel front RAVEL, in HOME. Both
# compose Grove's review example with an owner route and wrap the final
# selection once with the provider-evidence wrapper. These paths are temporary
# owned paths with no shell, TypeScript or Rhai metacharacters.
write_configuration() {
  local runtime="$1" ravel="$2" home="$3"
  local agent="$scratch/agent.sh"
  if [[ "$runtime" == 1.1 ]]; then
    cat > "$home/.config/ravel/policy.ts" <<EOF
import { definePolicy } from "ravel/sdk";
import { withProvenance } from "$assets/provenance.ts";
import { groveReviewSelector } from "$assets/examples/grove-review.ts";
const route = request => ({ program: "/bin/sh", args: [request.kind === "check" ? "agent.sh" : "$agent", request.kind, "$ravel"], provider: "fixture", model: "none", effort: "none" });
export const policy = withProvenance(definePolicy({ schemaVersion: 2, version: "installed-grove",
  ...groveReviewSelector({ routes: { impl: route, check: route }, reviews: {} }) }));
EOF
  else
    # A route is a closure, so a function of this script makes it.
    cat > "$home/.config/ravel/config.rhai" <<EOF
import "$assets/examples/grove-review" as grove_review;
import "$assets/provenance" as provenance;
export const SCHEMA_VERSION = 4;
export const VERSION = "installed-grove";
fn route() {
    |request| #{ program: "/bin/sh", args: [if request.kind == "check" { "agent.sh" } else { "$agent" }, request.kind, "$ravel"],
        provider: "fixture", model: "none", effort: "none" }
}
const RULES = #{ routes: #{ impl: route(), check: route() }, reviews: #{} };
fn load_context(request) { grove_review::load_context_with(global::RULES, request) }
fn select(request, context) {
    provenance::with_provenance(grove_review::select_with(global::RULES, request, context), global::VERSION, request)
}
EOF
  fi
}

# The source IDs of Grove's keg, in byte order, as Ravel lists them.
expected_sources='design
engineering
grove-core
requirements
research
writing'

# The keg's sources through a private copy of the 2.0 front RAVEL. Ravel finds
# the installation record beside its canonical executable. It reads the
# default skill directory only when the settings name no sources, and this
# HOME holds no settings. The compact JSON names a source's `id` and its
# `originPath` only in the source records, so grep reads them in order.
check_keg_sources() {
  local ravel="$1" copy="$scratch/keg-ravel" home="$scratch/keg-home" skills="$prefix/share/ravel/skills"
  local json ids origins expected_origins id
  [[ -d "$skills" ]] || fail "the prefix holds no $skills"
  mkdir -p "$copy/bin" "$copy/share/ravel" "$home" "$scratch/keg-tmp"
  cp "$ravel" "$copy/bin/ravel"
  chmod 755 "$copy/bin/ravel"
  printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$skills" >"$copy/share/ravel/installation.json"
  json="$(env -i HOME="$home" TMPDIR="$scratch/keg-tmp" PATH=/usr/bin:/bin "$copy/bin/ravel" skills resolve --json)" ||
    fail "the 2.0 build cannot resolve the keg's sources: $json"
  case "$json" in
    *'"status":"resolved"'*) ;;
    *) fail "the keg's sources did not resolve: $json" ;;
  esac
  ids="$(printf '%s\n' "$json" | grep -o '"id":"[^"]*"' | sed 's/^"id":"//; s/"$//')"
  [[ "$ids" == "$expected_sources" ]] || fail "the keg resolves the sources '$ids', not '$expected_sources'"
  origins="$(printf '%s\n' "$json" | grep -o '"originPath":"[^"]*"' | sed 's/^"originPath":"//; s/"$//')"
  expected_origins="$(for id in $expected_sources; do printf '%s/%s\n' "$skills" "$id"; done)"
  [[ "$origins" == "$expected_origins" ]] || fail "a source lies outside the keg: $origins"
  chmod -R u+w "$home"
  printf 'grove-installed-smoke: Ravel 2.0 resolved the six keg sources with no settings\n'
}

# Run every caller of the installed prefix through the Ravel prefix
# RAVEL_PREFIX, whose runtime is RUNTIME, in a HOME of its own.
run_lane() {
  local runtime="$1" ravel_prefix ravel reported work label front caller command utility
  ravel_prefix="$(cd "$2" && pwd -P)"
  ravel="$ravel_prefix/bin/ravel"
  [[ "$prefix" != "$ravel_prefix" ]] || fail 'Grove and Ravel need separate prefixes'
  reported="$("$ravel" --version)"
  if [[ "$runtime" == 1.1 ]]; then
    [[ "$reported" == 'ravel 1.1.0' ]] || fail "the 1.1 prefix reports '$reported', not the pinned independent Ravel 1.1.0"
    [[ -x "$ravel_prefix/libexec/ravel/ravel-policy" ]] || fail 'the 1.1 prefix holds no private 1.x worker'
  else
    # Before the cut, the feature build reports 1.1.0. Its layout tells it apart.
    case "$reported" in
      'ravel 1.1.0' | 'ravel 2.'[0-9]*.[0-9]*) ;;
      *) fail "the 2.0 prefix reports '$reported', not a 2.x build" ;;
    esac
    [[ "$reported" != 'ravel 2.'*-* ]] || fail "the 2.0 prefix reports the prerelease '$reported'"
    [[ ! -e "$ravel_prefix/libexec/ravel" ]] || fail 'the 2.0 prefix holds a 1.x worker'
  fi
  work="$scratch/ravel-$runtime"
  mkdir -p "$work/tools" "$work/linked/bin" "$work/home/.config/ravel" "$work/tmp" "$work/project/.jj/repo" "$work/project/.grove"
  for utility in cat chmod cmp cp dirname env grep ln ls mkdir rm touch; do
    resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing utility $utility"
    ln -s "$resolved" "$work/tools/$utility"
  done
  ln -s "$ravel" "$work/tools/ravel"
  for command in bun node harness-dispatch; do
    ! PATH="$work/tools" command -v "$command" >/dev/null 2>&1 || fail "isolated PATH contains $command"
  done
  for command in grove grove-llm; do
    ln -s "$prefix/bin/$command" "$work/linked/bin/$command"
  done
  write_configuration "$runtime" "$ravel" "$work/home"
  printf '# fixture\n' > "$work/project/.grove/_BRIEF.md"
  printf '# work\n' > "$work/project/.grove/01-impl--work-k1.md"
  for label in direct symlink; do
    front="$prefix/bin/grove"
    [[ "$label" == direct ]] || front="$work/linked/bin/grove"
    for caller in lifecycle standalone; do
      rm -f "$work/project/published.txt" "$work/project/result.txt"
      args=()
      [[ "$caller" == lifecycle ]] || args=(run check Write --ui=inline --input "$scratch/agent.sh" --output "$work/project/published.txt")
      (cd "$work/project" && env -i HOME="$work/home" TMPDIR="$work/tmp" PATH="$work/tools" \
        "$front" "${args[@]+"${args[@]}"}") >"$work/stdout" 2>"$work/stderr" ||
        fail "$runtime $label $caller failed: $(cat "$work/stderr")"
      grep -Fq managed-noise "$work/stderr" || fail "$runtime $label $caller lost child diagnostics: stderr=$(cat "$work/stderr") stdout=$(cat "$work/stdout")"
      ! grep -Fq grove-continue "$work/stdout" || fail "$runtime $label $caller leaked result data"
      if [[ "$caller" == standalone ]]; then
        [[ "$(cat "$work/project/published.txt")" == published ]] || fail "$runtime standalone did not publish output"
      else
        grep -Fq 'loop complete' "$work/stderr" || fail "$runtime lifecycle did not consume teardown after reap"
        [[ -f "$work/project/.grove/01-impl--work-k1.md" ]] || fail "$runtime fake lifecycle retired the fixture"
      fi
    done
    printf 'grove-installed-smoke: Ravel %s %s lifecycle, standalone, installed caller imports and separate versions passed\n' "$runtime" "$label"
  done
  [[ "$runtime" != 2.0 ]] || check_keg_sources "$ravel"
}

[[ "$3" == - ]] || run_lane 1.1 "$3"
[[ $# == 3 ]] || run_lane 2.0 "$4"
