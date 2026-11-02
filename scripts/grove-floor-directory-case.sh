#!/usr/bin/env bash
# The confined directory case of the Ravel 2.0 lane at the Linux floor.
#
#   grove-floor-directory-case.sh grove-prefix ravel-2.0-prefix
#
# This is the Linux port of the confined standalone case of
# crates/grove/tests/directory_route.rs. A confined `grove run impl` with
# directory delivery reads the mandatory spine, its resource and the named
# kind from Ravel's retained snapshots, and reads the descriptor. It cannot
# write to a snapshot, a source or a path outside its working directory. It
# cannot read a source directly or the protected configuration. Grove
# publishes the evidence only after the exact acknowledgement.
#
# The sources are plain local directories outside the working directory, and
# no native copy exists. The floor userland has no Git, so the plain Git
# checkout source of the macOS case is not part of this port.
#
# The control runs the same probes unconfined. Each denied read and each
# denied write outside the snapshots then succeeds, so those denials are the
# sandbox's. Ravel publishes each snapshot read-only, so the snapshot write
# probes fail unconfined too.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "grove-floor-directory: $*" >&2; exit 1; }
[[ $# == 2 ]] || fail 'expected the Grove prefix and the Ravel 2.0 prefix'
prefix="$1"
ravel="$2/bin/ravel"
[[ ! -e "$2/libexec/ravel" ]] || fail 'the 2.0 prefix holds a 1.x worker'

# Canonical, so that the snapshot paths that Ravel reports compare exactly.
root="$(cd "$(mktemp -d)" && pwd -P)"
trap 'chmod -R u+w "$root" 2>/dev/null; rm -rf "$root"' EXIT
home="$root/home"
core="$root/sources/core"
engineering="$root/sources/engineering"
mkdir -p "$home/.config/ravel" "$root/tmp" "$root/tools" "$root/cwd" "$root/out" "$root/unconfined" \
  "$core/skills/grove/references" "$engineering/skills/grove-impl"
spine='# grove spine (fixture)
resource: references/execute.md'
resource='execute procedure, first revision'
kind='# grove-impl (fixture), first revision'
printf '%s\n' "$spine" >"$core/skills/grove/SKILL.md"
printf '%s\n' "$resource" >"$core/skills/grove/references/execute.md"
printf '%s\n' "$kind" >"$engineering/skills/grove-impl/SKILL.md"
for utility in cat chmod cmp cp dirname env grep ln ls mkdir rm sed touch; do
  resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing utility $utility"
  ln -s "$resolved" "$root/tools/$utility"
done
ln -s "$ravel" "$root/tools/ravel"

cat >"$home/.config/ravel/settings.json" <<EOF
{"skills": {"schemaVersion": 1,
  "sources": [{"id": "core", "type": "local", "path": "$core", "roots": ["skills"]},
              {"id": "engineering", "type": "local", "path": "$engineering", "roots": ["skills"]}],
  "mandatory": [{"source": "core", "path": "skills/grove/SKILL.md"}]}}
EOF
cat >"$home/.config/ravel/config.rhai" <<EOF
export const SCHEMA_VERSION = 4;
export const VERSION = "floor-directory-standalone";
fn select(request, context) {
    #{ status: "selected", program: "$root/agent.sh", args: [request.prompt, ravel::json_encode(request.skills)],
        provider: "fixture", model: "none", effort: "none", reason: "the confined standalone fake agent" }
}
EOF

# Each probe prints `name=ok` when the operation succeeds and `name=denied`
# when it fails. The arguments are the directory to write in, the snapshot
# spine, the snapshot kind and the descriptor.
cat >"$root/probes.sh" <<EOF
probe() { name=\$1; shift; if "\$@" >/dev/null 2>&1; then echo "\$name=ok"; else echo "\$name=denied"; fi; }
append() { printf x >>"\$1"; }
probe descriptor-read /bin/cat "\$4"
probe cwd-write /usr/bin/touch "\$1/probe"
probe snapshot-write /usr/bin/touch "\$(/usr/bin/dirname "\$2")/probe"
probe snapshot-append append "\$3"
probe source-read /bin/cat "$core/skills/grove/SKILL.md"
probe source-write /usr/bin/touch "$core/probe"
probe outside-write /usr/bin/touch "$root/outside"
probe config-read /bin/cat "$home/.config/ravel/config.rhai"
probe settings-read /bin/cat "$home/.config/ravel/settings.json"
EOF

# The fake agent takes the mandatory paths and the search directories from
# Ravel's preamble, and the skill name from Grove's mandate in the caller
# block, as a real agent must. Its records go to its working directory.
{
  printf '#!/bin/sh\nprobes=%s\n' "'$root/probes.sh'"
  cat <<'EOF'
PATH=/usr/bin:/bin
record=$(pwd -P)
printf 'fake agent: started\n' >&2
printf started >"$record/started"
printf '%s' "$1" >"$record/prompt"
printf '%s' "$2" >"$record/skills.json"
preamble=$(printf '%s\n' "$1" | sed '/^Caller task$/q')
block=$(printf '%s\n' "$1" | sed -n '/^BEGIN RAVEL CALLER TASK /,/^END RAVEL CALLER TASK /p')
[ -n "$block" ] || exit 91
read_as() { printf '%s\n' "$2" >>"$record/reads"; cat "$2" >"$record/$1" || exit 95; }
spine=$(printf '%s\n' "$preamble" | sed -n 's/^[0-9][0-9]*\. "\(.*\)"$/\1/p' | head -n 1)
[ -n "$spine" ] || exit 94
read_as spine "$spine"
read_as resource "$(dirname "$spine")/$(sed -n 's/^resource: //p' "$spine")"
skill=$(printf '%s\n' "$block" | sed -n 's/^\*\*Load the `\([^`]*\)` skill now\.\*\*.*/\1/p' | head -n 1)
[ -n "$skill" ] || exit 93
kind=
for dir in $(printf '%s\n' "$preamble" | sed -n 's/^- "\(.*\)"$/\1/p'); do
  if [ -f "$dir/$skill/SKILL.md" ]; then kind="$dir/$skill/SKILL.md"; break; fi
done
[ -n "$kind" ] || exit 92
read_as kind "$kind"
/bin/sh "$probes" "$record" "$spine" "$kind" "$RAVEL_SKILL_DIRECTORIES" >"$record/probes"
eval "$(printf '%s\n' "$1" | sed -n '/exact command as your final action:/{n;p;}')"
EOF
} >"$root/agent.sh"
chmod +x "$root/agent.sh"

evidence=(prompt skills.json reads spine resource kind probes started)
outputs=()
for name in "${evidence[@]}"; do
  outputs+=(--output "$root/out/$name")
done
status=0
# shellcheck disable=SC2016 # the backticks are Markdown in the task text
(cd "$root/cwd" && env -i HOME="$home" TMPDIR="$root/tmp" PATH="$root/tools" \
  "$prefix/bin/grove" run impl '**Load the `grove-impl` skill now.** Then write the evidence files.' \
  --ui inline "${outputs[@]}" --runtime-read "$root/agent.sh" --runtime-read "$root/probes.sh") \
  >"$root/stdout" 2>"$root/stderr" || status=$?
cat "$root/stderr"
((status == 0)) || fail "the confined directory run failed ($status)"
grep -q 'grove run \[impl\]: completed' "$root/stderr" || fail 'Grove did not report completion'
[[ "$(cat "$root/out/started")" == started ]] || fail 'the agent did not start'

# The mandatory spine is the first read, and every read is a snapshot path
# outside the working directory and outside each source.
reads="$(cat "$root/out/reads")"
snapshots="$home/.cache/ravel/skills/v1/snapshots/"
first="$(printf '%s\n' "$reads" | head -n 1)"
[[ "$first" == "$snapshots"*/skills/grove/SKILL.md ]] || fail "the first read is not the snapshot spine: $first"
while IFS= read -r path; do
  [[ "$path" == "$snapshots"* ]] || fail "a read outside the snapshots: $path"
done <<<"$reads"
[[ "$(cat "$root/out/spine")" == "$spine" ]] || fail 'wrong spine'
[[ "$(cat "$root/out/resource")" == "$resource" ]] || fail 'wrong resource'
[[ "$(cat "$root/out/kind")" == "$kind" ]] || fail 'wrong kind'
grep -q '"mandatory"' "$root/out/skills.json" || fail 'request.skills names no mandatory file'
grep -q 'Standalone invocation: work only on the staged files' "$root/out/prompt" ||
  fail 'the caller block lost the standalone instructions'
for tree in .claude/skills .codex/skills .agents/skills; do
  [[ ! -e "$home/$tree" ]] || fail "a native copy exists at $home/$tree"
done

expected='descriptor-read=ok
cwd-write=ok
snapshot-write=denied
snapshot-append=denied
source-read=denied
source-write=denied
outside-write=denied
config-read=denied
settings-read=denied'
[[ "$(cat "$root/out/probes")" == "$expected" ]] || fail "confined probes: $(cat "$root/out/probes")"

# The control: the same probes, unconfined, on the same paths.
spine_path="$first"
kind_path="$(printf '%s\n' "$reads" | sed -n 3p)"
descriptor="$(sed -n 's/.*"descriptorPath": *"\([^"]*\)".*/\1/p' "$root/out/skills.json")"
[[ -f "$descriptor" ]] || fail "no descriptor at $descriptor"
unconfined="$(/bin/sh "$root/probes.sh" "$root/unconfined" "$spine_path" "$kind_path" "$descriptor")"
echo "grove-floor-directory: unconfined probes: ${unconfined//$'\n'/ }"
for name in source-read source-write outside-write config-read settings-read; do
  grep -qx "$name=ok" <<<"$unconfined" || fail "$name is denied unconfined too, so its confined denial is not the sandbox's"
done
echo 'grove-floor-directory: the confined directory run read the snapshots and was denied every write and protected read'
