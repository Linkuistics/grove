#!/usr/bin/env bash
# Repeat the hook-adapter seam on revised formula output: "Hook adapters" and
# "Native-only hook acceptance" of "Test seams" in
# docs/specs/codex-skill-provisioning.md. The earlier instrument,
# keg-registration.sh, measured real source directories under share. This one
# measures the keg layout with private payloads and public aliases.
#
# A private Homebrew runs in SCRATCH: a --shared clone of the owner's Homebrew
# repository at its HEAD, with its own prefix. A scratch tap holds two
# formulae:
#
# - `grove`, a disposable copy of Grove's keg. It takes the SKILL_SOURCES
#   constant and the install block of scripts/templates/grove.rb.tmpl, and it
#   installs ARCHIVE, a native Grove archive. Version 1 and version 2 differ
#   only in one marker line at the end of always-on/instruction.md of the
#   writing source, which the SessionStart hook prints. A hook's text
#   therefore names the keg that it read.
# - `personal`, the formula file of the tap working copy TAP, verbatim. The
#   private Homebrew downloads its URL.
#
# Each harness runs in its own scratch HOME, through the documented commands.
# The hook-adapter Claude Code HOME has no sign-in, so no request reaches a
# model: its SessionStart and UserPromptSubmit hooks run before the refusal.
# Pi runs without a provider. Codex links only the owner's auth.json, and each
# `codex exec` sends one short prompt. The guardrail HOMEs link only
# Library/Keychains for the sign-in, and each capability session spends one
# real Claude Code session of two turns.
#
# The guardrail capability trial repeats "Guardrail plugin trial" of
# docs/repository-moves-record.md. Its no-op control runs first: a scratch copy
# of the revised keg whose hook script only exits 0. One verdict judges both
# sessions, and the control must fail the refusal assertion.
#
# Nothing writes to the owner's Homebrew prefix, HOME or harness
# configuration. The instrument prints digests of those files before and
# after, and digests of each subject before and after.
#
# Usage: projected-keg.sh EMPTY_SCRATCH_DIR NATIVE_ARCHIVE TAP_WORKING_COPY
# The documented commands reach the harness's shell verbatim, so their
# "$(brew --prefix)" stays in single quotes. The listings name plain kegs.
# shellcheck disable=SC2016,SC2012,SC2088
set -euo pipefail
[[ $# == 3 && -d "$1" && -z "$(ls -A "$1")" && -f "$2" && -f "$3/Formula/personal.rb" ]] || {
  echo 'usage: projected-keg.sh EMPTY_SCRATCH_DIR NATIVE_ARCHIVE TAP_WORKING_COPY' >&2
  exit 2
}
S="$(cd "$1" && pwd -P)"
ARCHIVE="$(cd "$(dirname "$2")" && pwd -P)/$(basename "$2")"
TAP_WC="$(cd "$3" && pwd -P)"
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)"
SCRIPT="$REPO/docs/evaluations/hook-adapters/projected-keg.sh"
TEMPLATE="$REPO/scripts/templates/grove.rb.tmpl"
PERSONAL_RB="$TAP_WC/Formula/personal.rb"
TOP="$(basename "$ARCHIVE" .tar.xz)"
OWNER_BREW="$(brew --repository)"
OWNER_PREFIX="$(brew --prefix)"
ACCEPTED_PERSONAL="$OWNER_PREFIX/Cellar/personal/2026-11-06"
TC_PLUGIN="craft/technical-communication"
PAYLOAD_REL="libexec/skill-sources/writing/$TC_PLUGIN"
show() { printf '\n## %s\n' "$*"; }
scrub() { sed -e "s|$S|S|g" -e "s|$OWNER_PREFIX|PREFIX|g"; }
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }

# tree_manifest DIR: one line for each entry below DIR, with its relative
# path, its type, its permission mode, and its link target or SHA-256.
tree_manifest() {
  python3 -I - "$1" <<'PY'
import hashlib, os, stat, sys
root = sys.argv[1]
for base, dirs, files in os.walk(root):
    dirs.sort()
    for name in sorted(dirs + files):
        path = os.path.join(base, name)
        rel = os.path.relpath(path, root)
        st = os.lstat(path)
        mode = oct(stat.S_IMODE(st.st_mode))
        if stat.S_ISLNK(st.st_mode):
            print(rel, "link", mode, os.readlink(path))
        elif stat.S_ISDIR(st.st_mode):
            print(rel, "dir", mode)
        elif stat.S_ISREG(st.st_mode):
            print(rel, "file", mode, hashlib.sha256(open(path, "rb").read()).hexdigest())
        else:
            print(rel, "other", mode)
PY
}

# hook_declarations DIR: each hook declaration of a plugin. That is the
# `hooks:` block of each SKILL.md frontmatter, each hooks/hooks.json and the
# `hooks` key of each plugin.json.
hook_declarations() {
  python3 -I - "$1" <<'PY'
import json, os, sys
root = sys.argv[1]
for base, dirs, files in sorted(os.walk(root)):
    dirs.sort()
    for name in sorted(files):
        path = os.path.join(base, name)
        rel = os.path.relpath(path, root)
        if name == "SKILL.md":
            lines = open(path).read().split("\n")
            end = lines.index("---", 1)
            block, inside = [], False
            for line in lines[1:end]:
                if line.startswith("hooks:"):
                    inside = True
                elif inside and line and not line[0].isspace():
                    inside = False
                if inside:
                    block.append(line)
            print(rel, "frontmatter:", json.dumps(block))
        elif name == "hooks.json":
            print(rel, "hooks.json:", json.dumps(json.load(open(path)), sort_keys=True))
        elif name == "plugin.json":
            print(rel, "plugin.json hooks:", json.dumps(json.load(open(path)).get("hooks")))
PY
}

# owner_state: digests of the owner's files that the run must leave alone.
owner_state() {
  local f
  for f in "$HOME/.claude/settings.json" "$HOME/.claude/plugins/installed_plugins.json" \
    "$HOME/.claude/plugins/known_marketplaces.json" "$HOME/.codex/config.toml"; do
    local label="~${f#"$HOME"}"
    if [[ -f "$f" ]]; then echo "$label $(sha "$f")"; else echo "$label absent"; fi
  done
  echo "~/.pi/agent/extensions $(ls -l "$HOME/.pi/agent/extensions" 2>/dev/null | shasum -a 256 | cut -c1-16)"
  echo "~/.claude/plugins/cache $(find "$HOME/.claude/plugins/cache" -maxdepth 3 2>/dev/null | sort | shasum -a 256 | cut -c1-16)"
  echo "PREFIX/opt/grove -> $(readlink "$OWNER_PREFIX/opt/grove")"
  echo "PREFIX/opt/personal -> $(readlink "$OWNER_PREFIX/opt/personal")"
  echo "PREFIX/Cellar/grove: $(ls "$OWNER_PREFIX/Cellar/grove" | tr '\n' ' ')"
  echo "PREFIX/Cellar/personal: $(ls "$OWNER_PREFIX/Cellar/personal" | tr '\n' ' ')"
  echo "PREFIX/share/ravel/skills $(ls -l "$OWNER_PREFIX/share/ravel/skills" 2>/dev/null | shasum -a 256 | cut -c1-16)"
  echo "accepted personal keg $(tree_manifest "$ACCEPTED_PERSONAL" | shasum -a 256 | cut -c1-16)"
}

# subject_digests: each subject that the run reads.
subject_digests() {
  echo "script $(sha "$SCRIPT")"
  echo "archive $(sha "$ARCHIVE")"
  echo "template $(sha "$TEMPLATE")"
  echo "personal formula $(sha "$PERSONAL_RB")"
  echo "keg-root claude marketplace $(sha "$REPO/assets/keg-root/.claude-plugin/marketplace.json")"
  echo "keg-root codex marketplace $(sha "$REPO/assets/keg-root/.agents/plugins/marketplace.json")"
}

show "subjects"
echo "archive: $(basename "$ARCHIVE")"
subject_digests
echo "tap working copy commit of personal.rb: $(git -C "$TAP_WC" log -1 --format=%H -- Formula/personal.rb)"
echo "homebrew: $(git -C "$OWNER_BREW" rev-parse HEAD)"
echo "claude: $(claude --version)"
echo "codex: $(codex --version)"
echo "pi: $(pi --version 2>&1 | head -1)"
show "owner state before"
owner_state | tee "$S/owner-before.txt"

show "archive marketplace files against assets/keg-root"
mkdir -p "$S/archive"
tar -xJf "$ARCHIVE" -C "$S/archive"
for f in .claude-plugin/marketplace.json .agents/plugins/marketplace.json; do
  if cmp -s "$S/archive/$TOP/$f" "$REPO/assets/keg-root/$f"; then echo "$f: equal"; else echo "$f: DIFFERENT"; fi
  python3 -I -c 'import json,sys
d=json.load(open(sys.argv[1]))
for p in d["plugins"]:
    s=p["source"]; print("  plugin", p["name"], "source", s if isinstance(s,str) else s.get("path"))' "$S/archive/$TOP/$f"
done

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

# make_source MARKER: a source archive of the disposable formula.
make_source() {
  rm -rf "$S/src/$TOP"
  tar -xJf "$ARCHIVE" -C "$S/src"
  printf '\nProbe keg: %s\n' "$1" \
    >>"$S/src/$TOP/share/ravel/skills/writing/$TC_PLUGIN/always-on/instruction.md"
  COPYFILE_DISABLE=1 tar -C "$S/src" --no-xattrs -cJf "$S/grove-$1.tar.xz" "$TOP"
}
sources_constant="$(grep -E '^  SKILL_SOURCES = ' "$TEMPLATE")"
install_block="$(sed -n '/^  def install$/,/^  end$/p' "$TEMPLATE")"
[[ -n "$sources_constant" && -n "$install_block" ]] || { echo "the template lacks SKILL_SOURCES or install" >&2; exit 1; }
write_formula() { # VERSION MARKER
  cat >"$TAP/Formula/grove.rb" <<EOF
class Grove < Formula
  desc "Disposable copy of Grove's keg for the hook adapter measurement"
  homepage "https://github.com/Linkuistics/grove"
  url "file://$S/grove-$2.tar.xz"
  version "$1"
  sha256 "$(sha "$S/grove-$2.tar.xz")"

$sources_constant

$install_block
end
EOF
}
make_source one
make_source two
cp "$PERSONAL_RB" "$TAP/Formula/personal.rb"

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
  echo "native skill directories in the HOME: $(ls -d "$CCH/.claude/skills" "$CCH/.claude/plugins/cache"/*/*/*/skills 2>/dev/null | scrub | tr '\n' ' ')"
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
    echo "no hook review within 30 seconds. The last screen:"
    tail -15 <<<"$screen"
  fi
  tmux -L hookprobe send-keys -t review C-c
  sleep 1
  tmux -L hookprobe send-keys -t review C-c
  sleep 1
  tmux -L hookprobe kill-server 2>/dev/null || true
}

# codex_cache_against_payload: compare the Codex cache copy with the private
# payload of the current keg, entry by entry.
codex_cache_against_payload() {
  local cache="$CXH/.codex/plugins/cache/writing-hooks/technical-communication/local"
  local payload
  payload="$(cd "$S/brew/opt/grove" && pwd -P)/$PAYLOAD_REL"
  tree_manifest "$payload" >"$S/payload.manifest"
  tree_manifest "$cache" >"$S/cache.manifest"
  echo "payload entries: $(wc -l <"$S/payload.manifest" | tr -d ' '), cache entries: $(wc -l <"$S/cache.manifest" | tr -d ' ')"
  if diff "$S/payload.manifest" "$S/cache.manifest" >"$S/cache.diff"; then
    echo "cache copy equals the payload in paths, types, modes and SHA-256"
  else
    echo "cache copy differs from the payload:"
    head -20 "$S/cache.diff"
  fi
}

# pi_session LABEL: one Pi session without a provider. Prints the extension
# link's stored target, its canonical target and what the reply log recorded.
pi_session() {
  (cd "$PIH/work" && pie pi -p 'Say hi.' </dev/null >"$PIH/$1.out" 2>&1) || true
  local link="$PIH/.pi/agent/extensions/technical-communication.ts"
  echo "link names: $(readlink "$link" | scrub)"
  echo "prefix alias: share/ravel/skills/writing -> $(readlink "$S/brew/share/ravel/skills/writing")"
  echo "link resolves to: $(python3 -I -c 'import os,sys; print(os.path.realpath(sys.argv[1]))' "$link" | scrub)"
  echo "pi output: $(head -c 160 "$PIH/$1.out" | tr '\n' ' ')"
  echo "reply log records: $(cat "$PIH"/.local/state/technical-communication/sessions/pi--*.jsonl 2>/dev/null |
    python3 -I -c 'import json,sys; print([(json.loads(l)["kind"], json.loads(l).get("source")) for l in sys.stdin])')"
}

# guardrail_session HOME LABEL: one signed-in Claude Code session of two
# turns. The first turn is `/guardrail`. The second asks for the gated
# command, which the allow rule permits. Prints the readings and the verdict
# of the refusal assertion.
GATED='rm -rf scratch-dir'
guardrail_session() {
  local home="$1" label="$2"
  local -a env
  IFS=' ' read -r -a env <<<"$(harness_env "$home")"
  rm -rf "$home/work/scratch-dir"
  mkdir -p "$home/work/scratch-dir/inner"
  echo probe >"$home/work/scratch-dir/inner/file.txt"
  python3 -I -c 'import json,sys
for text in sys.argv[1:]:
    print(json.dumps({"type": "user", "message": {"role": "user", "content": [{"type": "text", "text": text}]}}))' \
    '/guardrail' \
    "Use the Bash tool once to run exactly this command: $GATED. Then say whether the command ran, and quote any hook or permission message that you saw." \
    >"$home/$label.in"
  (cd "$home/work" && "${env[@]}" claude -p --input-format stream-json --output-format stream-json \
    --verbose --include-hook-events --permission-mode acceptEdits --tools Bash,Skill \
    --allowedTools "Bash($GATED)" --model claude-sonnet-5-5 \
    <"$home/$label.in" >"$home/$label.jsonl" 2>"$home/$label.err") || echo "claude exit $?"
  echo "stream sha256: $(sha "$home/$label.jsonl")"
  python3 -I - "$home/$label.jsonl" "$home/work/scratch-dir" "$home/.claude/projects" <<'PY' | scrub
import glob, json, os, re, sys
events = []
for line in open(sys.argv[1]):
    try:
        events.append(json.loads(line))
    except ValueError:
        pass
def hook_name(e):
    return str(e.get("hook_event") or e.get("hook_name") or "")
inits = [e for e in events if (e.get("type"), e.get("subtype")) == ("system", "init")]
print("init events:", len(inits))
for e in inits[:1]:
    print("init session:", e.get("session_id"), "model:", e.get("model"))
    print("init plugins:", [(p["name"], p["path"]) for p in e.get("plugins", []) if p["path"] != "builtin"])
    print("init guardrail skills and commands:",
          sorted({s for s in e.get("skills", []) + e.get("slash_commands", []) if "guardrail" in s}))
text = "\n".join(json.dumps(e) for e in events)
session_id = inits[0].get("session_id") if inits else None
files = glob.glob(os.path.join(sys.argv[3], "*", f"{session_id}.jsonl")) if session_id else []
commands = sorted({c for f in files for c in re.findall(r"<command-name>([^<]*)</command-name>", open(f).read())})
print("commands in the session file:", commands)
pre_started = [e for e in events if e.get("subtype") == "hook_started" and hook_name(e).startswith("PreToolUse")]
pre_responses = [e for e in events if e.get("subtype") == "hook_response" and hook_name(e).startswith("PreToolUse")]
asks = [e for e in pre_responses if re.search(r'permissionDecision\W+ask', json.dumps(e))]
print("PreToolUse hook starts:", len(pre_started), "responses:", len(pre_responses), "ask responses:", len(asks))
for e in asks:
    reason = re.findall(r'permissionDecisionReason\W+([^"\\]+)', json.dumps(e))
    print("ask reason:", reason[:1])
print("permission_denied events:", len(re.findall(r'"permission_denied"', text)))
for e in events:
    if e.get("type") == "user":
        for part in e.get("message", {}).get("content", []):
            if isinstance(part, dict) and part.get("type") == "tool_result":
                content = part.get("content")
                if isinstance(content, list):
                    content = " ".join(c.get("text", "") for c in content if isinstance(c, dict))
                print("tool result:", "error" if part.get("is_error") else "ok", str(content)[:160].replace("\n", " "))
    elif e.get("type") == "result":
        print("result:", e.get("subtype"), str(e.get("result", ""))[:240].replace("\n", " "))
present = os.path.isdir(sys.argv[2])
print("scratch-dir after the session:", "present" if present else "absent")
refused = bool(asks) and present
print("refusal assertion:", "PASS" if refused else "FAIL")
PY
}

# The readings below continue past a failed command, so that each reading is
# printed. A display filter that matches nothing is a reading too.
set +e

show "install grove version 1"
write_formula 1 one
B install probe/hooks/grove 2>&1 | grep -E '^(Error|🍺)' | scrub
echo "opt/grove -> $(readlink "$S/brew/opt/grove")"
ls -A "$S/brew/opt/grove/" | tr '\n' ' '
echo
echo "keg alias: share/ravel/skills/writing -> $(readlink "$S/brew/opt/grove/share/ravel/skills/writing")"
echo "payload is a real directory: $([[ -d "$S/brew/opt/grove/libexec/skill-sources/writing" && ! -L "$S/brew/opt/grove/libexec/skill-sources/writing" ]] && echo yes || echo no)"
echo "prefix entry: share/ravel/skills/writing -> $(readlink "$S/brew/share/ravel/skills/writing")"

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
codex_cache_against_payload
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

show "upgrade grove to version 2, with cleanup"
write_formula 2 two
B upgrade probe/hooks/grove 2>&1 | grep -E '^(Error|🍺|Removing)' | scrub
B cleanup probe/hooks/grove 2>&1 | scrub | tail -2
echo "Cellar/grove: $(ls "$S/brew/Cellar/grove" | tr '\n' ' ')"
echo "opt/grove -> $(readlink "$S/brew/opt/grove")"
echo "prefix entry: share/ravel/skills/writing -> $(readlink "$S/brew/share/ravel/skills/writing")"

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
grep -A2 'marketplaces.writing-hooks' "$CXH/.codex/config.toml" | scrub
echo "cached marker: $(grep -o 'Probe keg: [a-z]*' "$CXH/.codex/plugins/cache/writing-hooks/technical-communication/local/always-on/instruction.md")"
codex_cache_against_payload
show "codex: session after the repair, before a review"
codex_session cx-v2-repaired
show "codex: the startup review after the repair"
codex_review
show "codex: session after the second review"
codex_session cx-v2-trusted

show "pi: after the upgrade"
pi_session pi-v2

show "install the revised personal formula"
B install probe/hooks/personal 2>&1 | grep -E '^(Error|🍺)' | scrub
echo "opt/personal -> $(readlink "$S/brew/opt/personal")"
echo "keg alias: share/ravel/skills/personal -> $(readlink "$S/brew/opt/personal/share/ravel/skills/personal")"
PERSONAL_KEG="$(cd "$S/brew/opt/personal" && pwd -P)"

show "guardrail payload against the accepted keg"
echo "accepted keg: $ACCEPTED_PERSONAL" | scrub
for part in libexec/plugins .claude-plugin; do
  tree_manifest "$ACCEPTED_PERSONAL/$part" >"$S/accepted-$(basename "$part").manifest"
  tree_manifest "$PERSONAL_KEG/$part" >"$S/revised-$(basename "$part").manifest"
  if diff "$S/accepted-$(basename "$part").manifest" "$S/revised-$(basename "$part").manifest"; then
    echo "$part: $(wc -l <"$S/revised-$(basename "$part").manifest" | tr -d ' ') entries equal in paths, types, modes, link targets and SHA-256"
  else
    echo "$part: DIFFERENT"
  fi
done
hook_declarations "$ACCEPTED_PERSONAL/libexec/plugins/guardrail" >"$S/accepted.hooks"
hook_declarations "$PERSONAL_KEG/libexec/plugins/guardrail" >"$S/revised.hooks"
cat "$S/revised.hooks"
if diff "$S/accepted.hooks" "$S/revised.hooks"; then echo "hook declarations: equal"; else echo "hook declarations: DIFFERENT"; fi

# guardrail_home NAME: a scratch HOME with only the sign-in linked.
guardrail_home() {
  local home="$S/$1"
  mkdir -p "$home/tmp" "$home/work" "$home/Library"
  ln -s "$HOME/Library/Keychains" "$home/Library/Keychains"
  echo "$home"
}

show "guardrail no-op control: a copy of the revised keg whose hook only exits 0"
NOOP="$S/noop-keg"
cp -RP "$PERSONAL_KEG" "$NOOP"
HOOK_REL="libexec/plugins/guardrail/skills/guardrail/scripts/guardrail-hook.sh"
printf '#!/usr/bin/env bash\nexit 0\n' >"$NOOP/$HOOK_REL"
echo "no-op hook script sha256: $(sha "$NOOP/$HOOK_REL"), mode $(stat -f %Lp "$NOOP/$HOOK_REL")"
CTL="$(guardrail_home guardrail-control)"
IFS=' ' read -r -a CTL_ENV <<<"$(harness_env "$CTL")"
"${CTL_ENV[@]}" claude plugin marketplace add "$NOOP" 2>&1 | tail -1 | scrub
"${CTL_ENV[@]}" claude plugin install guardrail@personal 2>&1 | tail -1 | scrub
guardrail_session "$CTL" control | tee "$S/control.verdict"
if grep -q '^refusal assertion: FAIL' "$S/control.verdict"; then
  echo "control: failed the refusal assertion, as required"
else
  echo "control: DID NOT FAIL. The trial below proves nothing"
fi

show "guardrail capability trial: the documented commands from the revised keg"
GRD="$(guardrail_home guardrail-trial)"
IFS=' ' read -r -a GRD_ENV <<<"$(harness_env "$GRD")"
"${GRD_ENV[@]}" sh -c 'claude plugin marketplace add "$(brew --prefix)/opt/personal"' 2>&1 | tail -1 | scrub
"${GRD_ENV[@]}" sh -c 'claude plugin install guardrail@personal' 2>&1 | tail -1 | scrub
echo "stored source: $(python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))["personal"]["source"])' \
  "$GRD/.claude/plugins/known_marketplaces.json" | scrub)"
"${GRD_ENV[@]}" claude plugin details guardrail@personal 2>&1 | grep -E 'Skills|Hooks|guardrail$' | scrub
guardrail_session "$GRD" trial

show "owner state after"
owner_state | tee "$S/owner-after.txt"
if diff "$S/owner-before.txt" "$S/owner-after.txt"; then echo "owner state: unchanged"; else echo "owner state: CHANGED"; fi
show "subject digests after"
subject_digests
