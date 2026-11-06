#!/usr/bin/env bash
# Measure how the commands of the hooks installation write
# ~/.claude/settings.json, and which outside writes a held hard link keeps.
#
# Every harness command runs with HOME and CLAUDE_CONFIG_DIR inside SCRATCH,
# a new empty directory, and with SCRATCH as its working directory. No
# command signs in, and no model request runs.
#
# codebase-memory-mcp runs only in server mode, with HOME inside SCRATCH.
# Its `install` command stops every running codebase-memory-mcp server on
# the machine, so this script never runs that command.
#
# Usage: settings-writes.sh SCRATCH
set -euo pipefail
[[ $# == 1 && -d "$1" && -z "$(ls -A "$1")" ]] || { echo 'usage: settings-writes.sh EMPTY_SCRATCH_DIR' >&2; exit 2; }
S="$(cd "$1" && pwd -P)"
T="$(cd "$(dirname "$0")" && pwd -P)"
W="$HOME/Development/AgenticOrchestration/WritingSkills"
commit="$(git -C "$W" rev-parse main)"
CLAUDE_BIN="$(readlink -f "$(command -v claude)")"
PLUGIN=technical-communication-hooks@writing-hooks
cd "$S"

show() { echo; echo "## $1"; }
hide() { sed "s|$S|S|g"; }
cc() { # HOME-DIR args...
  local h="$1"; shift
  env -u CLAUDE_CODE_PLUGIN_CACHE_DIR -u CLAUDE_CODE_PLUGIN_DIRS -u CLAUDE_CODE_PLUGIN_SEED_DIR \
    -u CLAUDE_SECURESTORAGE_CONFIG_DIR HOME="$h" CLAUDE_CONFIG_DIR="$h/.claude" "$CLAUDE_BIN" "$@"
}
ino() { stat -f %i "$1"; }
restore() { rm -rf "${S:?}/home"; cp -Rp "$1" "$S/home"; }
F="$S/home/.claude/settings.json"

show "Subjects"
echo "macOS $(sw_vers -productVersion), $(sysctl -n machdep.cpu.brand_string)"
echo "claude $("$CLAUDE_BIN" --version) sha256 $(shasum -a 256 <"$CLAUDE_BIN" | cut -c1-16)"
echo "WritingSkills main $commit"
for f in settings-writes.sh race.py dirwatch.py mcpserve.py; do echo "$f $(shasum -a 256 <"$T/$f" | cut -c1-16)"; done

# A keg-shaped tree with the writing hooks marketplace, reached through opt.
keg="$S/prefix/Cellar/grove/1/libexec"
mkdir -p "$keg/skills/writing" "$keg/.claude-plugin" "$S/prefix/opt"
git -C "$W" archive "$commit" plugins/writing | tar -x -C "$keg/skills/writing" --strip-components 2
cat >"$keg/.claude-plugin/marketplace.json" <<'JSON'
{"name":"writing-hooks","owner":{"name":"Linkuistics"},"plugins":[{"name":"technical-communication-hooks","source":"./skills/writing/craft/technical-communication","description":"Measurement copy."}]}
JSON
ln -s ../Cellar/grove/1 "$S/prefix/opt/grove"
M="$S/prefix/opt/grove/libexec"

# Three templates: before the registration, before the installation and
# before the disabling.
mkdir -p "$S/home/.claude"
printf '{\n  "marker": 1,\n  "enabledPlugins": {"other@x": true}\n}\n' >"$F"
cp -Rp "$S/home" "$S/t_add"
cc "$S/home" plugin marketplace add "$M" >/dev/null 2>&1
cp -Rp "$S/home" "$S/t_install"
cc "$S/home" plugin install "$PLUGIN" >/dev/null 2>&1
cp -Rp "$S/home" "$S/t_disable"

show "Write method of each command"
method() { # label template args...
  local label="$1" tpl="$2"; shift 2
  restore "$tpl"
  chmod 640 "$F"; chmod +a "everyone allow read" "$F"; xattr -w com.linkuistics.probe one "$F"
  local before; before="$(shasum -a 256 <"$F")"; local i0; i0="$(ino "$F")"; ln "$F" "$S/held"
  python3 -I "$T/dirwatch.py" "$S/home/.claude" "$S/watch" & local wp=$!
  sleep 0.3
  cc "$S/home" "$@" >"$S/out" 2>&1 || echo "exit $?"
  sleep 0.3; kill -TERM "$wp"; wait "$wp" || true
  echo "### $label: $(tail -1 "$S/out" | cut -c1-80)"
  echo "inode: $([[ "$(ino "$F")" == "$i0" ]] && echo same || echo replaced)"
  echo "held link keeps the earlier bytes: $([[ "$(shasum -a 256 <"$S/held")" == "$before" ]] && echo yes || echo no)"
  echo "temporary files: $(grep -o "added={'settings.json.tmp[^']*'" "$S/watch" | sed "s/added={'//; s/'$//" | sed -E 's/[0-9]+\.[0-9a-f]+$/<pid>.<hex>/' | sort | uniq -c | tr -s ' ' | tr '\n' ';')"
  echo "renames onto the path: $(grep -c "changed={'settings.json'" "$S/watch" || true)"
  echo "mode $(stat -f %Lp "$F"); ACL: $(/bin/ls -le "$F" | tail -n +2 | tr -s ' ' | tr '\n' ' ')"
  echo "extended attributes: $(xattr "$F" | tr '\n' ' ')"
  echo "top-level keys: $(jq -c 'keys_unsorted' "$F")"
  rm -f "$S/held"
}
method "marketplace add" "$S/t_add" plugin marketplace add "$M"
method "plugin install" "$S/t_install" plugin install "$PLUGIN"
method "plugin disable" "$S/t_disable" plugin disable "$PLUGIN"

show "A command whose values are already present"
unchanged() { # label template jq-filter args...
  local label="$1" tpl="$2" filt="$3"; shift 3
  restore "$tpl"
  if [[ "$filt" != "." ]]; then jq "$filt" "$F" >"$F.new"; mv "$F.new" "$F"; fi
  local i0; i0="$(ino "$F")"; local c0; c0="$(jq -S -c . "$F")"
  cc "$S/home" "$@" >/dev/null 2>&1 || true
  echo "$label: inode $([[ "$(ino "$F")" == "$i0" ]] && echo same || echo replaced), content $([[ "$(jq -S -c . "$F")" == "$c0" ]] && echo same || echo changed)"
}
ekm="$(jq -c '.extraKnownMarketplaces' "$S/t_install/.claude/settings.json")"
unchanged "marketplace add, extraKnownMarketplaces already present" "$S/t_add" ".extraKnownMarketplaces = $ekm" plugin marketplace add "$M"
unchanged "marketplace add, marketplace already registered" "$S/t_install" "." plugin marketplace add "$M"
unchanged "plugin install, enabledPlugins already true" "$S/t_install" ".enabledPlugins[\"$PLUGIN\"] = true" plugin install "$PLUGIN"

show "A deny-delete ACL on the file"
restore "$S/t_install"; chmod +a "everyone deny delete" "$F"; i0="$(ino "$F")"
cc "$S/home" plugin install "$PLUGIN" >/dev/null 2>&1 && rc=0 || rc=$?
echo "plugin install exit $rc, inode $([[ "$(ino "$F")" == "$i0" ]] && echo same || echo replaced), enabled $(jq -c ".enabledPlugins[\"$PLUGIN\"]" "$F")"
chmod -a "everyone deny delete" "$F"

show "Race of an outside write against each command"
for m in rename inplace; do
  python3 -I "$T/race.py" "$S/t_add" "$S/home" "$m" 3 135 "$S/race.add.$m.jsonl" -- "$CLAUDE_BIN" plugin marketplace add "$M"
  python3 -I "$T/race.py" "$S/t_install" "$S/home" "$m" 3 135 "$S/race.install.$m.jsonl" -- "$CLAUDE_BIN" plugin install "$PLUGIN"
  python3 -I "$T/race.py" "$S/t_disable" "$S/home" "$m" 3 135 "$S/race.disable.$m.jsonl" -- "$CLAUDE_BIN" plugin disable "$PLUGIN"
done
python3 -I - "$S" <<'PY'
import glob, json, os, sys
# lead = time of the command's rename minus time of the outside write.
for f in sorted(glob.glob(sys.argv[1] + "/race.*.jsonl")):
    rows = [json.loads(l) for l in open(f)]
    counts, lost, kept_before = {}, [], []
    for d in rows:
        counts[d["probe"]] = counts.get(d["probe"], 0) + 1
        mine = [t for t, i in d["inode_changes"] if i != d["outside_inode"]]
        if not mine:
            continue
        lead = mine[-1] - d["write"]
        if d["probe"] in ("lost", "held"):
            lost.append(lead)
        elif lead > 0:
            kept_before.append(lead)
    name = os.path.basename(f)[5:-6]
    print(f"{name:18} runs {len(rows)} {dict(sorted(counts.items()))} "
          f"missing from live at leads {min(lost, default=0):.1f}..{max(lost, default=0):.1f} ms, "
          f"in live at leads from {min(kept_before, default=0):.1f} ms")
PY

show "--scope local"
restore "$S/t_install"; mkdir -p "$S/proj" "$S/elsewhere"; i0="$(ino "$F")"
(cd "$S/proj" && cc "$S/home" plugin install --scope local "$PLUGIN" >/dev/null 2>&1)
echo "user settings.json inode $([[ "$(ino "$F")" == "$i0" ]] && echo same || echo replaced)"
echo "project file: $(cd "$S/proj" && find . -type f) $(jq -c . "$S/proj/.claude/settings.local.json")"
echo "installation record: $(jq -c ".plugins[\"$PLUGIN\"][0] | {scope, projectPath}" "$S/home/.claude/plugins/installed_plugins.json" | hide)"
echo "status in another directory: $(cd "$S/elsewhere" && cc "$S/home" plugin list --json 2>/dev/null | jq -r ".[] | select(.id == \"$PLUGIN\") | .enabled")"
echo "status in the project: $(cd "$S/proj" && cc "$S/home" plugin list --json 2>/dev/null | jq -r ".[] | select(.id == \"$PLUGIN\") | .enabled")"

show "A private configuration directory with the shared plugins directory"
restore "$S/t_add"; mkdir -p "$S/home/.claude/plugins" "$S/private"
cp -p "$F" "$S/private/settings.json"; ln -s "$S/home/.claude/plugins" "$S/private/plugins"
pcc() { env -u CLAUDE_CODE_PLUGIN_CACHE_DIR -u CLAUDE_CODE_PLUGIN_DIRS -u CLAUDE_CODE_PLUGIN_SEED_DIR \
  -u CLAUDE_SECURESTORAGE_CONFIG_DIR HOME="$S/home" CLAUDE_CONFIG_DIR="$S/private" "$CLAUDE_BIN" "$@"; }
pcc plugin marketplace add "$M" >/dev/null 2>&1; pcc plugin install "$PLUGIN" >/dev/null 2>&1
echo "real settings.json unchanged: $(cmp -s "$S/t_add/.claude/settings.json" "$F" && echo yes || echo no)"
echo "installPath: $(jq -r ".plugins[\"$PLUGIN\"][0].installPath" "$S/home/.claude/plugins/installed_plugins.json" | hide)"

show "settings.json as a link"
restore "$S/t_install"; mkdir -p "$S/linked"; mv "$F" "$S/linked/target.json"; ln -s "$S/linked/target.json" "$F"
i0="$(ino "$S/linked/target.json")"
cc "$S/home" plugin install "$PLUGIN" >/dev/null 2>&1
echo "path type: $(stat -f %HT "$F"), target inode $([[ "$(ino "$S/linked/target.json")" == "$i0" ]] && echo same || echo replaced), target enabled $(jq -c ".enabledPlugins[\"$PLUGIN\"]" "$S/linked/target.json")"

show "What plugin disable changes"
digest_tree() { (cd "$1" && find . -path ./settings.json -prune -o -path ./backups -prune -o \( -type f -o -type l \) -print \
  | LC_ALL=C sort | while read -r p; do if [[ -L "$p" ]]; then echo "$p -> $(readlink "$p")"; else echo "$p $(shasum -a 256 <"$p" | cut -c1-12)"; fi; done); }
restore "$S/t_disable"; digest_tree "$S/home/.claude" >"$S/tree.before"; jq -S . "$F" >"$S/settings.before"
cc "$S/home" plugin disable "$PLUGIN" >/dev/null 2>&1
digest_tree "$S/home/.claude" >"$S/tree.after"
echo "files compared outside settings.json: $(wc -l <"$S/tree.before" | tr -d ' '), changed: $(diff "$S/tree.before" "$S/tree.after" | grep -c '^[<>]' || true)"
echo "settings.json difference:"; diff "$S/settings.before" <(jq -S . "$F") || true

show "plugin install --marketplace"
restore "$S/t_add"
python3 -I "$T/dirwatch.py" "$S/home/.claude" "$S/watch" & wp=$!; sleep 0.3
cc "$S/home" plugin install --marketplace "$M" technical-communication-hooks >/dev/null 2>&1
sleep 0.3; kill -TERM "$wp"; wait "$wp" || true
echo "renames onto the path: $(grep -c "changed={'settings.json'" "$S/watch" || true)"

show "The list commands"
restore "$S/t_disable"
for args in "plugin list --json" "plugin marketplace list --json"; do
  i0="$(ino "$F")"; m0="$(stat -f %m "$F")"
  read -r -a words <<<"$args"
  cc "$S/home" "${words[@]}" >/dev/null 2>&1 && rc=0 || rc=$?
  echo "$args: exit $rc, inode $([[ "$(ino "$F")" == "$i0" ]] && echo same || echo replaced), mtime $([[ "$(stat -f %m "$F")" == "$m0" ]] && echo same || echo changed)"
done

show "codebase-memory-mcp in server mode"
if CBM="$(command -v codebase-memory-mcp)"; then
  "$CBM" --version 2>&1 | head -1
  H="$S/cbm"; mkdir -p "$H/.claude"
  cat >"$H/.claude/settings.json" <<'JSON'
{
  "marker": 1,
  "hooks": {
    "SessionStart": [{"matcher": "startup", "hooks": [{"type": "command", "command": "\"$HOME/.claude/hooks/cbm-session-reminder\""}]}],
    "PreToolUse": [{"matcher": "Grep|Glob", "hooks": [{"type": "command", "command": "\"$HOME/.claude/hooks/cbm-code-discovery-gate\""}]}]
  }
}
JSON
  python3 -I "$T/mcpserve.py" "$H" "$H/.claude/settings.json" 20 -- "$CBM" | hide
else
  echo "codebase-memory-mcp is not installed"
fi
