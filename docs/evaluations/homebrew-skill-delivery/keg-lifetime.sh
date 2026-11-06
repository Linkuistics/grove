#!/usr/bin/env bash
# Measure how Claude Code and Codex register a hooks plugin from a local
# marketplace inside a Homebrew-shaped keg, reached through an opt link, and
# what each harness reads after the link moves to a new keg and the old keg
# is erased, as `brew upgrade` followed by its cleanup does.
#
# No model request runs. Every harness command runs in a scratch HOME,
# CLAUDE_CONFIG_DIR and CODEX_HOME inside SCRATCH, a new empty directory.
# The last part launches `claude -p` without a sign-in. Its SessionStart hook
# runs before the sign-in refusal, so no request reaches a model.
#
# Usage: keg-lifetime.sh SCRATCH
set -euo pipefail
[[ $# == 1 && -d "$1" && -z "$(ls -A "$1")" ]] || { echo 'usage: keg-lifetime.sh EMPTY_SCRATCH_DIR' >&2; exit 2; }
S="$(cd "$1" && pwd -P)"
mkdir -p "$S/home" "$S/codex" "$S/prefix/Cellar/grove" "$S/prefix/opt"
W="$HOME/Development/AgenticOrchestration/WritingSkills"
commit="$(git -C "$W" rev-parse main)"

make_keg() { # version marker
  local keg="$S/prefix/Cellar/grove/$1/libexec"
  mkdir -p "$keg/skills/writing" "$keg/.claude-plugin" "$keg/.agents/plugins"
  git -C "$W" archive "$commit" plugins/writing | tar -x -C "$keg/skills/writing" --strip-components 2
  printf '%s\n' "$2" >"$keg/skills/writing/craft/technical-communication/KEG_VERSION"
  cat >"$keg/.claude-plugin/marketplace.json" <<'JSON'
{
  "name": "writing-hooks",
  "owner": { "name": "Linkuistics" },
  "plugins": [
    {
      "name": "technical-communication-hooks",
      "source": "./skills/writing/craft/technical-communication",
      "description": "Measurement copy of the writing hooks plugin."
    }
  ]
}
JSON
  cat >"$keg/.agents/plugins/marketplace.json" <<'JSON'
{
  "name": "writing-hooks",
  "interface": { "displayName": "Writing hooks" },
  "plugins": [
    {
      "name": "technical-communication",
      "source": { "source": "local", "path": "./skills/writing/craft/technical-communication" },
      "policy": { "installation": "AVAILABLE", "authentication": "ON_INSTALL" },
      "category": "Productivity"
    }
  ]
}
JSON
}

cc() { env -u CLAUDE_CODE_PLUGIN_CACHE_DIR -u CLAUDE_CODE_PLUGIN_DIRS -u CLAUDE_CODE_PLUGIN_SEED_DIR \
  -u CLAUDE_SECURESTORAGE_CONFIG_DIR HOME="$S/home" CLAUDE_CONFIG_DIR="$S/home/.claude" claude "$@"; }
cx() { env HOME="$S/home" CODEX_HOME="$S/codex" codex "$@"; }
show() { echo "## $1"; }

make_keg 1 keg-one
ln -s ../Cellar/grove/1 "$S/prefix/opt/grove"
M="$S/prefix/opt/grove/libexec"

show "claude: add marketplace by opt path"
cc plugin marketplace add "$M" 2>&1 | tail -3
show "claude: install"
cc plugin install technical-communication-hooks@writing-hooks 2>&1 | tail -3
show "claude: known_marketplaces.json"
cat "$S/home/.claude/plugins/known_marketplaces.json"
show "claude: installed_plugins.json"
cat "$S/home/.claude/plugins/installed_plugins.json"
show "claude: cache tree"
find "$S/home/.claude/plugins" -maxdepth 4 -type d | sed "s|$S|S|" | head -30

show "codex: add marketplace by opt path"
cx plugin marketplace add "$M" 2>&1 | tail -3
show "codex: add plugin"
cx plugin add technical-communication@writing-hooks 2>&1 | tail -3
show "codex: config.toml"
cat "$S/codex/config.toml" 2>/dev/null || true
show "codex: plugin tree"
find "$S/codex" -maxdepth 6 -name KEG_VERSION -print -exec cat {} \; | sed "s|$S|S|"

# The upgrade: a new keg, the opt link moved, the old keg erased.
make_keg 2 keg-two
ln -sfn ../Cellar/grove/2 "$S/prefix/opt/grove"
rm -rf "$S/prefix/Cellar/grove/1"

show "after upgrade: claude plugin list"
cc plugin list 2>&1 | tail -8
show "after upgrade: claude installPath content"
path="$(jq -r '.plugins["technical-communication-hooks@writing-hooks"][0].installPath' "$S/home/.claude/plugins/installed_plugins.json")"
echo "installPath: ${path/#$S/S}"
cat "$path/KEG_VERSION" 2>&1 || echo "installPath unreadable"
show "after upgrade: codex plugin list"
cx plugin list 2>&1 | tail -8 || true
show "after upgrade: codex cached KEG_VERSION"
find "$S/codex" -name KEG_VERSION -print -exec cat {} \; | sed "s|$S|S|"
show "after upgrade: codex marketplace list"
cx plugin marketplace list 2>&1 | sed "s|$S|S|" | tail -6 || true

# Codex: register again by the opt path, then add the plugin again.
show "codex: marketplace add again"
cx plugin marketplace add "$M" 2>&1 | sed "s|$S|S|" | tail -2
cx plugin add technical-communication@writing-hooks 2>&1 | tail -1
show "codex: cached KEG_VERSION after the second add"
find "$S/codex" -name KEG_VERSION -exec cat {} \;

# Claude Code: the plugin root that a SessionStart hook receives.
P="$S/prefix/Cellar/probe/1/libexec"
mkdir -p "$P/.claude-plugin" "$P/plugins/probe/.claude-plugin" "$P/plugins/probe/hooks" "$S/out"
echo '{"name":"probe","skills":[]}' >"$P/plugins/probe/.claude-plugin/plugin.json"
# The quoted heredoc keeps the JSON literal. OUT becomes the output directory.
sed "s|OUT|$S/out|" >"$P/plugins/probe/hooks/hooks.json" <<'JSON'
{"hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"/bin/sh",
"args":["-c","printf '%s\\n' \"$CLAUDE_PLUGIN_ROOT\" >> OUT/root.txt"],"timeout":5}]}]}}
JSON
echo '{"name":"probe-market","owner":{"name":"x"},"plugins":[{"name":"probe","source":"./plugins/probe"}]}' \
  >"$P/.claude-plugin/marketplace.json"
ln -s ../Cellar/probe/1 "$S/prefix/opt/probe"
cc plugin marketplace add "$S/prefix/opt/probe/libexec" 2>&1 | tail -1
cc plugin install probe@probe-market 2>&1 | tail -1
(cd "$S" && env -u ANTHROPIC_API_KEY -u CLAUDE_CODE_OAUTH_TOKEN HOME="$S/home" CLAUDE_CONFIG_DIR="$S/home/.claude" \
  claude -p x --max-turns 1 </dev/null 2>&1 | tail -1) || true
show "claude: CLAUDE_PLUGIN_ROOT of the SessionStart hook"
sed "s|$S|S|" "$S/out/root.txt"
