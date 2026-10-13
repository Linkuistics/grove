#!/usr/bin/env bash
# Installed Grove callers, readable assets and separate public Ravel dependency.
# Bash 3.2/system tools only, including in the Linux floor userland.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "grove-installed-smoke: $*" >&2; exit 1; }
[[ $# == 3 ]] || fail 'usage: grove-installed-smoke.sh grove-prefix version ravel-prefix'
prefix="$(cd "$1" && pwd -P)"
version="$2"
ravel="$(cd "$3" && pwd -P)/bin/ravel"
[[ "$prefix" != "${ravel%/bin/ravel}" ]] || fail 'Grove and Ravel need separate prefixes'
[[ "$("$ravel" --version)" == 'ravel 1.0.0' ]] || fail 'not the pinned independent Ravel'
[[ "$version" != 1.0.0 ]] || fail 'fixture must prove different project versions'
for command in grove grove-llm; do
  [[ "$("$prefix/bin/$command" --version)" == "$command $version" ]] || fail "$command version mismatch"
done
assets="$prefix/libexec/ravel-policy"
for asset in README.md grove.ts provenance.ts examples/grove-static.ts examples/grove-review.ts examples/review.ts sample/policy.ts typecheck/static.ts typecheck/review.ts; do
  [[ -f "$assets/$asset" ]] || fail "missing caller asset $asset"
done
for retired in bin/ravel bin/harness-dispatch libexec/ravel libexec/harness-dispatch; do
  [[ ! -e "$prefix/$retired" ]] || fail "Grove bundles $retired"
done
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
work="$(cd "$work" && pwd -P)"
mkdir -p "$work/tools" "$work/linked/bin" "$work/home/.config/ravel" "$work/tmp" "$work/project/.jj/repo" "$work/project/.grove"
for utility in cat chmod cmp cp dirname env grep ln ls mkdir rm touch; do
  resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing utility $utility"
  ln -s "$resolved" "$work/tools/$utility"
done
ln -s "$ravel" "$work/tools/ravel"
export PATH="$work/tools"
for runtime in bun node harness-dispatch; do
  ! command -v "$runtime" >/dev/null 2>&1 || fail "isolated PATH contains $runtime"
done
for command in grove grove-llm; do
  ln -s "$prefix/bin/$command" "$work/linked/bin/$command"
done
# These paths are temporary owned paths with no shell/TypeScript metacharacters.
# The fake agent closes the lifecycle through its own launch-directory marker;
# it does not retire work or run a second real driver.
cat > "$work/agent.sh" <<'EOF'
#!/bin/sh
printf 'managed-noise\n'
case "$1" in
  impl) touch "$GROVE_LAUNCH_DIR/teardown" ;;
  check) printf published > published.txt ;;
  *) exit 91 ;;
esac
exec "$2" end-session grove-continue
EOF
chmod +x "$work/agent.sh"
cat > "$work/home/.config/ravel/policy.ts" <<EOF
import { definePolicy } from "ravel/sdk";
import { withProvenance } from "$assets/provenance.ts";
import { groveReviewSelector } from "$assets/examples/grove-review.ts";
const route = request => ({ program: "/bin/sh", args: [request.kind === "check" ? "agent.sh" : "$work/agent.sh", request.kind, "$ravel"], provider: "fixture", model: "none", effort: "none" });
export const policy = withProvenance(definePolicy({ schemaVersion: 2, version: "installed-grove",
  ...groveReviewSelector({ routes: { impl: route, check: route }, reviews: {} }) }));
EOF
printf '# fixture\n' > "$work/project/.grove/_BRIEF.md"
printf '# work\n' > "$work/project/.grove/01-impl--work-k1.md"
for label in direct symlink; do
  front="$prefix/bin/grove"
  [[ "$label" == direct ]] || front="$work/linked/bin/grove"
  for caller in lifecycle standalone; do
    rm -f "$work/project/published.txt" "$work/project/result.txt"
    args=()
    [[ "$caller" == lifecycle ]] || args=(run check Write --ui=inline --input "$work/agent.sh" --output "$work/project/published.txt")
    (cd "$work/project" && env -i HOME="$work/home" TMPDIR="$work/tmp" PATH="$work/tools" \
      "$front" "${args[@]+"${args[@]}"}") >"$work/stdout" 2>"$work/stderr" ||
      fail "$label $caller failed: $(cat "$work/stderr")"
    grep -Fq managed-noise "$work/stderr" || fail "$label $caller lost child diagnostics: stderr=$(cat "$work/stderr") stdout=$(cat "$work/stdout")"
    ! grep -Fq grove-continue "$work/stdout" || fail "$label $caller leaked result data"
    if [[ "$caller" == standalone ]]; then
      [[ "$(cat "$work/project/published.txt")" == published ]] || fail 'standalone did not publish output'
    else
      grep -Fq 'loop complete' "$work/stderr" || fail 'lifecycle did not consume teardown after reap'
      [[ -f "$work/project/.grove/01-impl--work-k1.md" ]] || fail 'fake lifecycle retired the fixture'
    fi
  done
  printf 'grove-installed-smoke: %s lifecycle, standalone, installed caller imports and separate versions passed\n' "$label"
done
