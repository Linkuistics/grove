#!/usr/bin/env bash
# Measure each harness's registration of the hook adapters from a Grove keg,
# by the documented commands, and what each harness runs across `brew upgrade`
# with cleanup. "Hook adapters" of docs/specs/codex-skill-provisioning.md
# states the commands and the seam.
#
# A private Homebrew runs in SCRATCH: a --shared clone of the owner's Homebrew
# repository at its HEAD, with its own prefix. A disposable formula `grove` in
# a scratch tap installs ARCHIVE, a native Grove archive, with the install
# block of scripts/templates/grove.rb.tmpl. Version 1 and version 2 differ
# only in one marker line at the end of always-on/instruction.md of the
# writing source, so a hook's session-start text names the keg that it read.
#
# Each harness runs in its own scratch HOME. Claude Code runs without a
# sign-in, so no request reaches a model: its SessionStart and
# UserPromptSubmit hooks run before the refusal. Pi also runs without a
# provider. Codex links only the owner's auth.json, as the acceptance trials
# did, and each `codex exec` sends one short prompt. Nothing writes to the
# owner's Homebrew prefix, HOME or harness configuration.
#
# Usage: keg-registration.sh EMPTY_SCRATCH_DIR NATIVE_ARCHIVE
# The documented commands reach the harness's shell verbatim, so their
# "$(brew --prefix)" stays in single quotes. The listings name plain kegs.
# shellcheck disable=SC2016,SC2012
set -euo pipefail
[[ $# == 2 && -d "$1" && -z "$(ls -A "$1")" && -f "$2" ]] ||
  { echo 'usage: keg-registration.sh EMPTY_SCRATCH_DIR NATIVE_ARCHIVE' >&2; exit 2; }
S="$(cd "$1" && pwd -P)"
ARCHIVE="$(cd "$(dirname "$2")" && pwd -P)/$(basename "$2")"
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)"
TOP="$(basename "$ARCHIVE" .tar.xz)"
OWNER_BREW="$(brew --repository)"
show() { printf '\n## %s\n' "$*"; }
scrub() { sed "s|$S|S|g"; }

show "subjects"
echo "archive: $(basename "$ARCHIVE") sha256 $(shasum -a 256 "$ARCHIVE" | cut -d' ' -f1)"
echo "homebrew: $(git -C "$OWNER_BREW" rev-parse HEAD)"
echo "claude: $(claude --version)"
echo "codex: $(codex --version)"
echo "pi: $(pi --version 2>&1 | head -1)"

# The private Homebrew. Homebrew's inherited_sandbox? refuses a sandbox that
# permits writes to bin/brew, and its own profile permits writes under the
# temporary roots, so that one file is read-only.
mkdir -p "$S/cache" "$S/logs" "$S/tmp" "$S/brew-home" "$S/src"
git clone -q --shared "$OWNER_BREW" "$S/brew"
git -C "$S/brew" checkout -q "$(git -C "$OWNER_BREW" rev-parse HEAD)"
for entry in portable-ruby portable-ruby-arm64-darwin portable-ruby-version; do
  [[ -e "$S/brew/Library/Homebrew/vendor/$entry" ]] ||
    ln -s "$OWNER_BREW/Library/Homebrew/vendor/$entry" "$S/brew/Library/Homebrew/vendor/$entry"
done
chmod a-w "$S/brew/bin/brew"
B() {
  env -i PATH="$S/brew/bin:/usr/bin:/bin" HOME="$S/brew-home" HOMEBREW_NO_AUTO_UPDATE=1 \
    HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_ENV_HINTS=1 HOMEBREW_CACHE="$S/cache" \
    HOMEBREW_LOGS="$S/logs" HOMEBREW_TEMP="$S/tmp" "$S/brew/bin/brew" "$@"
}
B tap-new --no-git probe/hooks >/dev/null 2>&1
TAP="$(B --repository probe/hooks)"

# version MARKER: a source archive of the disposable formula.
make_source() {
  rm -rf "$S/src/$TOP"
  tar -xJf "$ARCHIVE" -C "$S/src"
  printf '\nProbe keg: %s\n' "$1" \
    >>"$S/src/$TOP/share/ravel/skills/writing/craft/technical-communication/always-on/instruction.md"
  COPYFILE_DISABLE=1 tar -C "$S/src" --no-xattrs -cJf "$S/grove-$1.tar.xz" "$TOP"
}
install_block="$(sed -n '/^  def install$/,/^  end$/p' "$REPO/scripts/templates/grove.rb.tmpl")"
write_formula() { # VERSION MARKER
  cat >"$TAP/Formula/grove.rb" <<EOF
class Grove < Formula
  desc "Disposable copy of Grove's keg for the hook adapter measurement"
  homepage "https://github.com/Linkuistics/grove"
  url "file://$S/grove-$2.tar.xz"
  version "$1"
  sha256 "$(shasum -a 256 "$S/grove-$2.tar.xz" | cut -d' ' -f1)"

$install_block
end
EOF
}
make_source one
make_source two

# The harness environments. PATH puts the private brew first, so the
# documented "$(brew --prefix)" names the scratch prefix.
harness_env() { # HOME
  echo env -i "PATH=$S/brew/bin:/usr/bin:/bin:/opt/homebrew/bin:$HOME/.local/bin" "HOME=$1" \
    "USER=$USER" "LOGNAME=$USER" "SHELL=/bin/zsh" "LANG=en_US.UTF-8" "TMPDIR=$1/tmp" \
    "HOMEBREW_NO_AUTO_UPDATE=1"
}
CCH="$S/claude"
CXH="$S/codex"
PIH="$S/pi"
mkdir -p "$CCH/tmp" "$CCH/work" "$CXH/tmp" "$CXH/work" "$CXH/.codex" "$PIH/tmp" "$PIH/work" \
  "$PIH/.pi/agent/extensions"
ln -s "$HOME/.codex/auth.json" "$CXH/.codex/auth.json"
# A newer Codex release makes the interactive start ask to update first, and
# its first choice runs `brew upgrade --cask codex`. The scratch configuration
# turns that check off, so the start reaches the hook review.
printf 'check_for_update_on_startup = false\n' >"$CXH/.codex/config.toml"
IFS=' ' read -r -a CC_ENV <<<"$(harness_env "$CCH")"
IFS=' ' read -r -a CX_ENV <<<"$(harness_env "$CXH")"
IFS=' ' read -r -a PI_ENV <<<"$(harness_env "$PIH")"
CX_ENV+=("CODEX_HOME=$CXH/.codex")
cc() { "${CC_ENV[@]}" "$@"; }
cx() { "${CX_ENV[@]}" "$@"; }
pie() { "${PI_ENV[@]}" "$@"; }

# claude_session LABEL: one session without a sign-in. Prints the plugins of
# the init event, each technical-communication skill, each hook event and the
# marker of the session-start text.
claude_session() {
  (cd "$CCH/work" && cc claude -p 'Say hi.' --output-format stream-json --verbose \
    --include-hook-events --max-turns 1 </dev/null >"$CCH/$1.jsonl" 2>&1) || true
  python3 -I - "$CCH/$1.jsonl" <<'PY' | scrub
import json, re, sys
for line in open(sys.argv[1]):
    try:
        event = json.loads(line)
    except ValueError:
        continue
    kind = (event.get("type"), event.get("subtype"))
    if kind == ("system", "init"):
        print("init plugins:", [(p["name"], p["path"]) for p in event.get("plugins", []) if p["path"] != "builtin"])
        print("init technical-communication skills:", [s for s in event.get("skills", []) if "technical-communication" in s])
    elif kind[1] in ("hook_started", "hook_response"):
        text = json.dumps(event)
        marker = re.findall(r"Probe keg: [a-z]+", text)
        print(kind[1], event.get("hook_event") or event.get("hook_name"), marker)
    elif kind[0] == "result":
        print("result:", event.get("subtype"), str(event.get("result", ""))[:80])
PY
}

# codex_session LABEL: one `codex exec` session. Prints whether the session
# file holds a hook's session-start marker and what the reply log recorded.
codex_session() {
  local before after
  before="$(find "$CXH/.codex/sessions" -name '*.jsonl' 2>/dev/null | sort)"
  (cd "$CXH/work" && cx codex exec --json --skip-git-repo-check --color never -s read-only \
    'Reply with the single word ready.' </dev/null >"$CXH/$1.jsonl" 2>"$CXH/$1.err") || echo "codex exec exit $?"
  after="$(find "$CXH/.codex/sessions" -name '*.jsonl' | sort)"
  local file
  file="$(comm -13 <(echo "$before") <(echo "$after") | tail -1)"
  echo "agent reply: $(python3 -I -c 'import json,sys
for l in open(sys.argv[1]):
    e=json.loads(l)
    if e.get("type")=="item.completed": print(e["item"].get("text"))' "$CXH/$1.jsonl")"
  echo "session-start marker in the session file: $(grep -o 'Probe keg: [a-z]*' "$file" | sort -u | tr '\n' ' ')"
  local id
  id="$(basename "$file" .jsonl | grep -oE '[0-9a-f]{8}-[0-9a-f-]{27}$')"
  local log="$CXH/.local/state/technical-communication/sessions/codex--$id.jsonl"
  if [[ -f "$log" ]]; then
    echo "reply log records: $(python3 -I -c 'import json,sys; print([json.loads(l)["kind"] for l in open(sys.argv[1])])' "$log")"
  else
    echo "reply log records: none"
  fi
}

# codex_review: start the interactive TUI and wait for its hook review. Codex
# first draws its composer and replaces it with the review once the hook list
# arrives, so the wait is for the review itself. Print the review and choose
# "Trust all and continue". A start that shows no review within 30 seconds
# reports that.
codex_review() {
  local screen=""
  tmux -L hookprobe kill-server 2>/dev/null || true
  tmux -L hookprobe new-session -d -x 160 -y 45 -s review \
    "cd $CXH/work && $(printf '%q ' "${CX_ENV[@]}") TERM=xterm-256color codex -s read-only; sleep 60"
  for _ in $(seq 1 30); do
    sleep 1
    screen="$(tmux -L hookprobe capture-pane -p -t review)"
    grep -q 'Hooks need review' <<<"$screen" && break
  done
  if grep -q 'Hooks need review' <<<"$screen"; then
    sed -n '/Hooks need review/,/enter confirm/p' <<<"$screen"
    tmux -L hookprobe send-keys -t review Down Enter
    for _ in $(seq 1 30); do
      sleep 1
      screen="$(tmux -L hookprobe capture-pane -p -t review)"
      grep -q 'Hooks need review' <<<"$screen" || break
    done
    echo "chose: Trust all and continue"
  else
    echo "no hook review within 30 seconds"
  fi
  tmux -L hookprobe send-keys -t review C-c
  sleep 1
  tmux -L hookprobe send-keys -t review C-c
  sleep 1
  tmux -L hookprobe kill-server 2>/dev/null || true
}

# pi_session LABEL: one Pi session without a provider. Prints the extension
# link's target and what the reply log recorded.
pi_session() {
  (cd "$PIH/work" && pie pi -p 'Say hi.' </dev/null >"$PIH/$1.out" 2>&1) || true
  echo "link resolves to: $(python3 -I -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' \
    "$PIH/.pi/agent/extensions/technical-communication.ts" | scrub)"
  echo "pi output: $(head -c 160 "$PIH/$1.out" | tr '\n' ' ')"
  echo "reply log records: $(cat "$PIH"/.local/state/technical-communication/sessions/pi--*.jsonl 2>/dev/null |
    python3 -I -c 'import json,sys; print([(json.loads(l)["kind"], json.loads(l).get("source")) for l in sys.stdin])')"
}

# The readings below continue past a failed command, so that each reading is
# printed. A display filter that matches nothing is a reading too.
set +e

show "install version 1"
write_formula 1 one
B install probe/hooks/grove 2>&1 | grep -E '^(Error|🍺)' | scrub
echo "opt/grove -> $(readlink "$S/brew/opt/grove")"
ls -A "$S/brew/opt/grove/" | tr '\n' ' '
echo

show "claude: the documented commands"
cc sh -c 'claude plugin marketplace add "$(brew --prefix)/opt/grove"' 2>&1 | tail -1 | scrub
cc sh -c 'claude plugin install technical-communication-hooks@writing-hooks' 2>&1 | tail -1 | scrub
echo "stored source: $(python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))["writing-hooks"]["source"])' \
  "$CCH/.claude/plugins/known_marketplaces.json" | scrub)"
cc claude plugin details technical-communication-hooks@writing-hooks 2>&1 | grep -E 'Skills|Hooks'
show "claude: session at version 1"
claude_session cc-v1

show "codex: the documented commands"
cx sh -c 'codex plugin marketplace add "$(brew --prefix)/opt/grove"' 2>&1 | head -1 | scrub
cx sh -c 'codex plugin add technical-communication@writing-hooks' 2>&1 | head -1 | scrub
grep -A2 'marketplaces.writing-hooks' "$CXH/.codex/config.toml" | scrub
show "codex: session before the review"
codex_session cx-v1-untrusted
show "codex: the startup review"
codex_review
echo "trusted hooks: $(grep -c '^trusted_hash' "$CXH/.codex/config.toml")"
grep -m1 '^trusted_content_hash' "$CXH/.codex/config.toml"
show "codex: session after the review"
codex_session cx-v1-trusted

show "pi: the documented command"
pie sh -c 'ln -s "$(brew --prefix)/share/ravel/skills/writing/craft/technical-communication/harnesses/pi/technical-communication.ts" ~/.pi/agent/extensions/'
show "pi: session at version 1"
pi_session pi-v1

show "upgrade to version 2, with cleanup"
write_formula 2 two
B upgrade probe/hooks/grove 2>&1 | grep -E '^(Error|🍺|Removing)' | scrub
B cleanup probe/hooks/grove 2>&1 | scrub | tail -2
echo "Cellar/grove: $(ls "$S/brew/Cellar/grove" | tr '\n' ' ')"
echo "opt/grove -> $(readlink "$S/brew/opt/grove")"

show "claude: after the upgrade"
cc claude plugin list 2>&1 | grep -A3 technical-communication-hooks | scrub
claude_session cc-v2

show "codex: after the upgrade, before the repair"
cx codex plugin list 2>&1 | grep -v '^[a-z-]*@openai-curated' | tail -3 | scrub
echo "cached marker: $(grep -o 'Probe keg: [a-z]*' "$CXH/.codex/plugins/cache/writing-hooks/technical-communication/local/always-on/instruction.md")"
codex_session cx-v2-before-repair

show "codex: the repair"
cx sh -c 'codex plugin marketplace add "$(brew --prefix)/opt/grove"' 2>&1 | head -1 | scrub
cx sh -c 'codex plugin add technical-communication@writing-hooks' 2>&1 | head -1 | scrub
echo "cached marker: $(grep -o 'Probe keg: [a-z]*' "$CXH/.codex/plugins/cache/writing-hooks/technical-communication/local/always-on/instruction.md")"
show "codex: session after the repair, before a review"
codex_session cx-v2-repaired
show "codex: the startup review after the repair"
codex_review
show "codex: session after the second review"
codex_session cx-v2-trusted

show "pi: after the upgrade"
pi_session pi-v2
