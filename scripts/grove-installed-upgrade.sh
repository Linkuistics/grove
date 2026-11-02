#!/usr/bin/env bash
# An installed upgrade from a Grove 22.8 layout, with the 1.1 Ravel lane.
#
#   grove-installed-upgrade.sh OLD_PREFIX NEW_PREFIX NEW_VERSION RAVEL_1_1_PREFIX
#
# OLD_PREFIX is a Grove 22.8 installation, such as its Homebrew keg. It is only
# read: the check copies it into a scratch Cellar. The real 22.8 executable
# first writes its legacy native layout into a scratch HOME, which also holds
# foreign entries, aliases and modes. A 22.8 driver then stays in a session,
# reading a skill through a legacy link, while the check upgrades the scratch
# Cellar to NEW_PREFIX the way Homebrew does. The new Grove runs one lifecycle
# and two failures. Every entry of the HOME and the old keg must keep its type,
# mode, link target and bytes. The old session must then read the same skill
# bytes and end normally.
#
# This models tap state A to C for Grove only: the unversioned dependency stays
# Ravel 1.1. Rollout rehearses the Homebrew states with real formulae.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "grove-installed-upgrade: $*" >&2; exit 1; }
[[ $# == 4 ]] || fail 'usage: grove-installed-upgrade.sh old-prefix new-prefix new-version ravel-1.1-prefix'
old="$(cd "$1" && pwd -P)"
new="$(cd "$2" && pwd -P)"
version="$3"
ravel_prefix="$(cd "$4" && pwd -P)"
ravel="$ravel_prefix/bin/ravel"
old_version="$("$old/bin/grove" --version)"
[[ "$old_version" == 'grove 22.8.'* ]] || fail "the old prefix reports '$old_version', not a 22.8 layout"
[[ ! -e "$old/libexec/ravel-policy/sample/config.rhai" ]] || fail 'the old prefix already holds the Rhai set'
[[ "$("$new/bin/grove" --version)" == "grove $version" ]] || fail 'the new prefix does not report its version'
[[ -f "$new/libexec/ravel-policy/sample/config.rhai" ]] || fail 'the new prefix holds no Rhai set'
[[ "$("$ravel" --version)" == 'ravel 1.1.0' && -x "$ravel_prefix/libexec/ravel/ravel-policy" ]] ||
  fail 'the Ravel prefix is not the pinned independent 1.1.0 layout'

work="$(mktemp -d)"
reader_pid=
cleanup() {
  [[ -z "$reader_pid" ]] || kill "$reader_pid" 2>/dev/null || true
  rm -rf "$work"
}
trap cleanup EXIT
work="$(cd "$work" && pwd -P)"
home="$work/home"
cellar="$work/brew/Cellar/grove"
mkdir -p "$work/tools" "$work/tmp" "$work/brew/bin" "$work/brew/opt" "$cellar" "$work/foreign/skill" \
  "$home/.config/ravel" "$home/.codex/skills" "$home/.claude"
for utility in cat chmod cmp cp dirname env grep ln ls mkdir rm sleep touch; do
  resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing utility $utility"
  ln -s "$resolved" "$work/tools/$utility"
done
ln -s "$ravel" "$work/tools/ravel"

# Homebrew's shape: a versioned keg, an opt link and bin links into the keg.
cp -Rp "$old" "$cellar/old"
ln -s ../Cellar/grove/old "$work/brew/opt/grove"
for command in grove grove-llm; do
  ln -s "../Cellar/grove/old/bin/$command" "$work/brew/bin/$command"
done

# The owner's policy imports the caller assets through the stable opt path,
# as the caveats direct. The TypeScript set keeps those paths across upgrades.
assets="$work/brew/opt/grove/libexec/ravel-policy"
cat > "$work/agent.sh" <<EOF
#!/bin/sh
case "\$1" in
  impl) ;;
  reader)
    cat "$home/.agents/skills/grove/SKILL.md" > "$work/read-before"
    touch "$work/reader-started"
    tries=0
    while [ ! -e "$work/release" ]; do
      tries=\$((tries + 1))
      [ "\$tries" -le 1200 ] || exit 92
      sleep 0.1
    done
    cat "$home/.agents/skills/grove/SKILL.md" > "$work/read-after"
    ;;
  *) exit 91 ;;
esac
touch "\$GROVE_LAUNCH_DIR/teardown"
exec "$ravel" end-session grove-continue
EOF
chmod +x "$work/agent.sh"
cat > "$home/.config/ravel/policy.ts" <<EOF
import { definePolicy } from "ravel/sdk";
import { withProvenance } from "$assets/provenance.ts";
import { groveReviewSelector } from "$assets/examples/grove-review.ts";
const route = (role: string) => () => ({ program: "/bin/sh", args: ["$work/agent.sh", role], provider: "fixture", model: "none", effort: "none" });
export const policy = withProvenance(definePolicy({ schemaVersion: 2, version: "installed-upgrade",
  ...groveReviewSelector({ routes: { impl: route("impl"), research: route("reader") }, reviews: {} }) }));
EOF

# A project at PATH whose one leaf has KIND.
project() {
  mkdir -p "$1/.jj/repo" "$1/.grove"
  printf '# fixture\n' > "$1/.grove/_BRIEF.md"
  printf '# work\n' > "$1/.grove/01-$2--work-k1.md"
}
# Run the Grove front FRONT in DIR, writing LABEL.out and LABEL.err.
launch() {
  local label="$1" dir="$2" front="$3"
  shift 3
  (cd "$dir" && env -i HOME="$home" TMPDIR="$work/tmp" PATH="$work/tools" "$front" "$@") \
    >"$work/$label.out" 2>"$work/$label.err"
}
# One line for each entry below each ROOT: type, mode, link target or SHA-256.
digest() {
  local root entry mode
  for root in "$@"; do
    find "$root" -print | LC_ALL=C sort | while IFS= read -r entry; do
      if [[ "$(uname -s)" == Darwin ]]; then mode="$(stat -f %Lp "$entry")"; else mode="$(stat -c %a "$entry")"; fi
      if [[ -L "$entry" ]]; then
        printf 'L %s %s -> %s\n' "$entry" "$mode" "$(readlink "$entry")"
      elif [[ -d "$entry" ]]; then
        printf 'D %s %s\n' "$entry" "$mode"
      else
        printf 'F %s %s %s\n' "$entry" "$mode" "$(shasum -a 256 "$entry" | cut -d' ' -f1)"
      fi
    done
  done
}

# The real 22.8 provisioner writes its layout, because ~/.codex exists.
project "$work/seed" impl
launch seed "$work/seed" "$work/brew/bin/grove" || fail "the 22.8 seed lifecycle failed: $(cat "$work/seed.err")"
grep -Fq 'loop complete' "$work/seed.err" || fail 'the 22.8 seed lifecycle did not complete'
[[ -L "$home/.agents/.grove/current" && -L "$home/.agents/skills/grove" ]] ||
  fail 'the 22.8 provisioner wrote no legacy layout'
[[ -f "$home/.agents/skills/grove/SKILL.md" ]] || fail 'the legacy grove link does not resolve'
# Foreign entries, aliases, a killed startup's leftover, and distinct modes.
mkdir -p "$home/.agents/skills/my-skill" "$home/.agents/skills/link-Abc123"
printf 'mine\n' > "$home/.agents/skills/my-skill/SKILL.md"
printf '#!/bin/sh\n' > "$home/.agents/skills/my-skill/run.sh"
chmod 600 "$home/.agents/skills/my-skill/SKILL.md"
chmod 755 "$home/.agents/skills/my-skill/run.sh"
chmod 700 "$home/.agents/skills/my-skill"
printf 'foreign\n' > "$work/foreign/skill/SKILL.md"
ln -s "$work/foreign/skill" "$home/.agents/skills/foreign"
ln -s "$home/.agents/.grove/current/plugins" "$home/.agents/skills/link-Abc123/link"
ln -s "$home/.agents/skills/grove" "$home/.codex/skills/grove"
ln -s ../.agents/skills "$home/.claude/skills"

# A 22.8 driver stays in a session that reads a skill through the legacy link.
project "$work/reader" research
launch reader "$work/reader" "$work/brew/bin/grove" &
reader_pid=$!
tries=0
until [[ -e "$work/reader-started" ]]; do
  tries=$((tries + 1))
  ((tries <= 600)) || fail "the 22.8 reader session did not start: $(cat "$work/reader.err")"
  sleep 0.1
done
digest "$home" "$cellar/old" "$work/foreign" > "$work/before"

# Homebrew's upgrade: a new keg beside the old, then the opt and bin links move.
cp -Rp "$new" "$cellar/new"
ln -sfn ../Cellar/grove/new "$work/brew/opt/grove"
for command in grove grove-llm; do
  ln -sf "../Cellar/grove/new/bin/$command" "$work/brew/bin/$command"
done
[[ "$("$work/brew/bin/grove" --version)" == "grove $version" ]] || fail 'the upgrade did not move the bin link'

# The new Grove: one lifecycle, a directory with no tree, and a refused launch.
project "$work/after" impl
launch after "$work/after" "$work/brew/bin/grove" || fail "the upgraded lifecycle failed: $(cat "$work/after.err")"
grep -Fq 'loop complete' "$work/after.err" || fail 'the upgraded lifecycle did not complete'
mkdir "$work/empty"
! launch empty "$work/empty" "$work/brew/bin/grove" || fail 'the upgraded Grove started without a tree'
# Grove reports a refused launch, stops the loop and leaves the leaf live.
project "$work/refused" unrouted
launch refused "$work/refused" "$work/brew/bin/grove" || true
if ! grep -Fq 'policy code: incomplete_mapping' "$work/refused.err" || ! grep -Fq 'loop stopped' "$work/refused.err"; then
  fail "the unrouted launch did not stop at Ravel's refusal: $(cat "$work/refused.err")"
fi
[[ -f "$work/refused/.grove/01-unrouted--work-k1.md" ]] || fail 'the refused launch retired its leaf'
digest "$home" "$cellar/old" "$work/foreign" > "$work/after-digest"
diff -u "$work/before" "$work/after-digest" >&2 ||
  fail 'the upgrade or the new Grove changed a native entry, a link, a mode, a pointer, a byte or the old keg'

# The old session reads the same skill bytes and ends normally.
touch "$work/release"
wait "$reader_pid" || fail "the 22.8 reader session failed after the upgrade: $(cat "$work/reader.err")"
reader_pid=
grep -Fq 'loop complete' "$work/reader.err" || fail 'the 22.8 reader session did not complete'
cmp "$work/read-before" "$work/read-after" || fail 'the 22.8 reader read different skill bytes after the upgrade'
printf 'grove-installed-upgrade: %s to %s left %s entries unchanged, and the active 22.8 reader completed\n' \
  "$old_version" "$version" "$(wc -l < "$work/before" | tr -d ' ')"
