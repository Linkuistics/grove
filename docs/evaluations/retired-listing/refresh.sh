#!/usr/bin/env bash
# Measure what a refresh of Grove's retired `linkuistics` listing does to an
# installed `grove@linkuistics` plugin. This is the seam "Retired listing" of
# docs/specs/codex-skill-provisioning.md.
#
# A local bare repository plays the GitHub repository Linkuistics/grove. Its
# first commit has the tree of LISTED_COMMIT, a Grove commit whose
# marketplace lists the packs. Its second commit changes only
# .claude-plugin/marketplace.json, to the bytes of RETIRED_FILE. Each scratch
# home carries a Git `insteadOf` rule for the HTTPS and SSH forms of the
# GitHub URL. A user's command `claude plugin marketplace add
# Linkuistics/grove` therefore clones the bare repository, and the
# registration records the source `github` with the repository
# `Linkuistics/grove`, as a real user's registration does.
#
# Two homes add the marketplace at the first commit and install the plugin.
# The bare repository then moves to the second commit.
# - The home "manual" refreshes with `claude plugin marketplace update
#   linkuistics`, then runs `claude plugin update grove@linkuistics`, then
#   removes the marketplace.
# - The home "auto" has `autoUpdate: true` for the marketplace, and only
#   starts sessions.
#
# No model request runs. Every Claude Code command runs under `env -i` with a
# scratch HOME inside SCRATCH, a new empty directory. A session runs
# `claude -p` without a sign-in. Its init event lists the plugins and skills
# before the sign-in refusal. An interactive start runs Claude Code in a
# private tmux server for 30 seconds, with onboarding marked complete and the
# folder trusted, and prints its last screen. Before the "auto" home's
# interactive start, its marketplace's `lastUpdated` moves a week back, so
# that a minimum interval cannot hold back the automatic refresh. Output names
# SCRATCH as `S`.
#
# Usage: refresh.sh SCRATCH LISTED_COMMIT RETIRED_FILE
set -euo pipefail
[[ $# == 3 && -d "$1" && -z "$(ls -A "$1")" && -f "$3" ]] ||
  { echo 'usage: refresh.sh EMPTY_SCRATCH_DIR LISTED_COMMIT RETIRED_FILE' >&2; exit 2; }
S="$(cd "$1" && pwd -P)"
G="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
listed="$(git -C "$G" rev-parse --verify "$2^{commit}")"
retired_file="$(cd "$(dirname "$3")" && pwd -P)/$(basename "$3")"
MARKET=.claude-plugin/marketplace.json

scrub() { sed "s|$S|S|g"; }
show() { echo; echo "## $1"; }
digest() { shasum -a 256 "$1" | cut -c1-64; }

echo "claude: $(claude --version)"
echo "git: $(git --version)"
echo "script: $(digest "$0")"
echo "listed commit: $listed"
echo "retired file: $(digest "$retired_file")"

# The remote. Each commit has fixed names and dates, so its identity depends
# only on its tree.
export GIT_AUTHOR_NAME=measurement GIT_AUTHOR_EMAIL=measurement@example.invalid
export GIT_COMMITTER_NAME=measurement GIT_COMMITTER_EMAIL=measurement@example.invalid
export GIT_AUTHOR_DATE='2026-01-01T00:00:00Z' GIT_COMMITTER_DATE='2026-01-01T00:00:00Z'
git init -q --bare "$S/remote.git"
git -C "$S/remote.git" symbolic-ref HEAD refs/heads/main
git init -q "$S/src"
git -C "$G" archive "$listed" | tar -x -C "$S/src"
git -C "$S/src" add -A
git -C "$S/src" commit -q -m listed
first="$(git -C "$S/src" rev-parse HEAD)"
cp "$retired_file" "$S/src/$MARKET"
git -C "$S/src" commit -q -a -m retired
second="$(git -C "$S/src" rev-parse HEAD)"
unset GIT_AUTHOR_NAME GIT_AUTHOR_EMAIL GIT_COMMITTER_NAME GIT_COMMITTER_EMAIL GIT_AUTHOR_DATE GIT_COMMITTER_DATE

show "remote"
if [[ "$(git -C "$S/src" rev-parse "$first^{tree}")" != "$(git -C "$G" rev-parse "$listed^{tree}")" ]]; then
  echo "the first commit's tree differs from the listed commit's tree" >&2
  exit 1
fi
echo "the first commit has the tree of the listed commit"
echo "files changed by the second commit: $(git -C "$S/src" diff --name-only "$first" "$second" | tr '\n' ' ')"
echo "plugins at the first commit: $(git -C "$S/src" show "$first:$MARKET" | python3 -I -c 'import json,sys; print([p["name"] for p in json.load(sys.stdin)["plugins"]])')"
echo "plugins at the second commit: $(git -C "$S/src" show "$second:$MARKET" | python3 -I -c 'import json,sys; print([p["name"] for p in json.load(sys.stdin)["plugins"]])')"
push() { git -C "$S/src" push -q -f "$S/remote.git" "$1:refs/heads/main"; }
push "$first"

# make_home NAME: a scratch home whose Git sends both GitHub URL forms of
# Linkuistics/grove to the bare repository.
make_home() {
  local h="$S/$1"
  mkdir -p "$h/tmp" "$h/work"
  cat >"$h/.gitconfig" <<EOF
[url "$S/remote.git"]
	insteadOf = https://github.com/Linkuistics/grove.git
	insteadOf = git@github.com:Linkuistics/grove.git
EOF
}
# run_in NAME COMMAND...: run COMMAND in the home NAME, from an allow-listed
# environment.
run_in() {
  local h="$S/$1"
  shift
  (cd "$h/work" && env -i "PATH=/usr/bin:/bin:/opt/homebrew/bin:$HOME/.local/bin" "HOME=$h" \
    "USER=$USER" "LOGNAME=$USER" "SHELL=/bin/zsh" "LANG=en_US.UTF-8" "TMPDIR=$h/tmp" "$@")
}
# report [-1] NAME COMMAND...: run COMMAND in the home NAME and print its
# output, or only its last line with -1, and then its exit status.
report() {
  local last=0 out status=0
  [[ "$1" == -1 ]] && { last=1; shift; }
  out="$(run_in "$@" 2>&1)" || status=$?
  if ((last)); then printf '%s\n' "$out" | tail -1 | scrub; else printf '%s\n' "$out" | scrub; fi
  echo "exit: $status"
}
# state NAME: the marketplace clone's commit, the installed plugins and the
# plugin cache of the home NAME.
state_clone() {
  local clone
  clone="$(git -C "$S/$1/.claude/plugins/marketplaces/linkuistics" rev-parse HEAD 2>/dev/null || echo none)"
  case "$clone" in
    "$first") echo "marketplace clone: the first commit" ;;
    "$second") echo "marketplace clone: the second commit" ;;
    *) echo "marketplace clone: $clone" ;;
  esac
}
state() {
  local h="$S/$1" p="$S/$1/.claude/plugins"
  state_clone "$1"
  echo "known_marketplaces.json: $(python3 -I -c 'import json,sys
try: d=json.load(open(sys.argv[1]))
except OSError: d=None
print(sorted(d) if d is not None else "absent")' "$p/known_marketplaces.json")"
  echo "installed_plugins.json: $(python3 -I -c 'import json,sys
try: d=json.load(open(sys.argv[1]))
except OSError: print("absent"); raise SystemExit
print({k: [(e.get("scope"), e.get("version"), e.get("gitCommitSha", "")[:12]) for e in v] for k, v in d.get("plugins", {}).items()})' "$p/installed_plugins.json")"
  echo "enabledPlugins: $(python3 -I -c 'import json,sys
try: print(json.load(open(sys.argv[1])).get("enabledPlugins"))
except OSError: print("absent")' "$h/.claude/settings.json")"
  echo "cache: $(cd "$p" && find cache -maxdepth 3 -mindepth 3 -type d 2>/dev/null | sort | tr '\n' ' ')"
  echo "plugin list:"
  report "$1" claude plugin list | sed 's/^/  /'
}
# session NAME LABEL: one session without a sign-in. Prints the plugins and
# the number of grove skills of the init event, and the result.
session() {
  run_in "$1" claude -p 'Say hi.' --output-format stream-json --verbose --max-turns 1 \
    </dev/null >"$S/$1/$2.jsonl" 2>&1 || true
  python3 -I - "$S/$1/$2.jsonl" <<'PY' | scrub
import json, sys
init = False
for line in open(sys.argv[1]):
    try:
        event = json.loads(line)
    except ValueError:
        continue
    kind = (event.get("type"), event.get("subtype"))
    if kind == ("system", "init"):
        init = True
        print("init plugins:", [(p["name"], p["path"]) for p in event.get("plugins", []) if p["path"] != "builtin"])
        grove = sorted(s for s in event.get("skills", []) if s.startswith("grove:"))
        print("init grove skills:", len(grove), grove[:3])
    elif kind[0] == "result":
        print("result:", event.get("subtype"), str(event.get("result", ""))[:80])
if not init:
    print("no init event")
PY
}

# interactive NAME: an interactive start of 30 seconds in the home NAME.
# Prints the marketplace clone's commit and the last screen.
interactive() {
  local h="$S/$1" sock="retired-listing-$$" screen
  python3 -I - "$h/.claude.json" <<'PY'
import json, os, sys
path = sys.argv[1]
data = json.load(open(path)) if os.path.exists(path) else {}
data.update(hasCompletedOnboarding=True, theme="dark")
json.dump(data, open(path, "w"))
PY
  tmux -L "$sock" new-session -d -s start -x 200 -y 50 \
    "cd '$h/work' && env -i 'PATH=/usr/bin:/bin:/opt/homebrew/bin:$HOME/.local/bin' 'HOME=$h' \
      'USER=$USER' 'LOGNAME=$USER' SHELL=/bin/zsh LANG=en_US.UTF-8 'TMPDIR=$h/tmp' TERM=xterm-256color claude"
  sleep 5
  if tmux -L "$sock" capture-pane -p -t start | grep -q 'trust this folder'; then
    tmux -L "$sock" send-keys -t start Down
    sleep 1
    tmux -L "$sock" send-keys -t start Enter
  fi
  sleep 25
  screen="$(tmux -L "$sock" capture-pane -p -t start)"
  tmux -L "$sock" kill-server
  state_clone "$1"
  echo "screen:"
  printf '%s\n' "$screen" | grep -v '^[[:space:]]*$' | grep -v '^[─━]*$' | sed 's/[[:space:]]*$//; s/^/  /' | scrub
}

for home in manual auto; do
  make_home "$home"
  show "$home: add the marketplace at the first commit"
  report -1 "$home" claude plugin marketplace add Linkuistics/grove
  show "$home: install grove@linkuistics"
  report -1 "$home" claude plugin install grove@linkuistics
done
# The home "auto" turns on automatic updates for the marketplace in the
# setting that `marketplace add` wrote.
python3 -I - "$S/auto/.claude/settings.json" <<'PY'
import json, sys
path = sys.argv[1]
data = json.load(open(path))
data["extraKnownMarketplaces"]["linkuistics"]["autoUpdate"] = True
json.dump(data, open(path, "w"), indent=2)
PY
for home in manual auto; do
  show "$home: state at the first commit"
  state "$home"
  show "$home: session at the first commit"
  session "$home" first
done

push "$second"
show "remote moved to the second commit"

show "manual: claude plugin marketplace update linkuistics"
report manual claude plugin marketplace update linkuistics
show "manual: state after the refresh"
state manual
show "manual: plugin details after the refresh"
report manual claude plugin details grove@linkuistics
show "manual: session after the refresh"
session manual refreshed
show "manual: a second refresh"
report -1 manual claude plugin marketplace update linkuistics
show "manual: claude plugin update grove@linkuistics"
report manual claude plugin update grove@linkuistics
show "manual: state after the plugin update"
state manual
show "manual: session after the plugin update"
session manual updated
show "manual: interactive start after the refresh"
interactive manual

show "auto: first session after the remote moved"
session auto start1
show "auto: state after the first session"
state auto
show "auto: second session after the remote moved"
session auto start2
show "auto: state after the second session"
state auto
python3 -I - "$S/auto/.claude/plugins/known_marketplaces.json" <<'PY'
import json, sys
path = sys.argv[1]
data = json.load(open(path))
data["linkuistics"]["lastUpdated"] = "2026-10-31T00:00:00.000Z"
json.dump(data, open(path, "w"), indent=2)
PY
show "auto: interactive start with lastUpdated a week back"
interactive auto
show "auto: state after the interactive start"
state auto

show "manual: claude plugin marketplace remove linkuistics"
report manual claude plugin marketplace remove linkuistics
show "manual: state after the removal"
state manual

show "digests after the run"
echo "script: $(digest "$0")"
echo "retired file: $(digest "$retired_file")"
