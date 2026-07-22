#!/usr/bin/env bash
# Tests for guardrail-hook.sh. Dependency-free (bats is not assumed): pipe a
# representative PreToolUse event in, assert the decision out. Run: ./guardrail-hook.test.sh
set -euo pipefail
IFS=$'\n\t'

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
hook="$here/guardrail-hook.sh"
pass=0
fail=0

# decision_of <event-json>: run the hook, print its permissionDecision, or
# "defer" when it emits nothing. Extra args become env vars for the run.
decision_of() {
  local out
  out="$("${@:2}" bash "$hook" <<<"$1" || true)"
  if [[ -z "$out" ]]; then
    printf 'defer'
  else
    jq -r '.hookSpecificOutput.permissionDecision // "defer"' <<<"$out"
  fi
}

# check <name> <expected> <event-json> [env=val ...]
check() {
  local name="$1" expected="$2" event="$3"
  shift 3
  local got
  got="$(decision_of "$event" env "$@")"
  if [[ "$got" == "$expected" ]]; then
    pass=$((pass + 1))
  else
    fail=$((fail + 1))
    printf 'FAIL: %s — expected %s, got %s\n' "$name" "$expected" "$got" >&2
  fi
}

bash_event() { jq -nc --arg c "$1" '{tool_name:"Bash",cwd:"/repo",tool_input:{command:$c}}'; }
edit_event() { jq -nc --arg p "$1" --arg cwd "${2:-/repo}" '{tool_name:"Edit",cwd:$cwd,tool_input:{file_path:$p}}'; }

# --- CAREFUL half: destructive Bash commands should ask ----------------------
check "rm -rf"               ask "$(bash_event 'rm -rf /tmp/x')"
check "rm -fr"               ask "$(bash_event 'rm -fr build')"
check "rm --recursive"       ask "$(bash_event 'rm --recursive --force dir')"
check "rm -rf piped"         ask "$(bash_event 'find . -name x | xargs rm -rf')"
check "git push --force"     ask "$(bash_event 'git push --force origin main')"
check "git push -f"          ask "$(bash_event 'git push -f')"
check "git push with-lease"  ask "$(bash_event 'git push --force-with-lease')"
check "git reset --hard"     ask "$(bash_event 'git reset --hard HEAD~1')"
check "git clean -fd"        ask "$(bash_event 'git clean -fd')"
check "jj abandon"           ask "$(bash_event 'jj abandon wq')"
check "jj op restore"        ask "$(bash_event 'jj op restore b51416386f26')"
check "jj operation restore" ask "$(bash_event 'jj operation restore b51416386f26')"
check "DROP TABLE"           ask "$(bash_event 'psql -c "DROP TABLE users;"')"
check "truncate table"       ask "$(bash_event 'mysql -e "truncate table sessions"')"
check "mkfs"                 ask "$(bash_event 'mkfs.ext4 /dev/sdb')"
check "dd to device"         ask "$(bash_event 'dd if=/dev/zero of=/dev/sda bs=1M')"
check "redirect to device"   ask "$(bash_event 'echo x > /dev/sda')"

# --- CAREFUL half: safe commands should defer (no false alarms) --------------
check "ls"                   defer "$(bash_event 'ls -la')"
check "npm test"             defer "$(bash_event 'npm test')"
check "rm single file"       defer "$(bash_event 'rm stale.txt')"
check "git push plain"       defer "$(bash_event 'git push origin main')"
check "grep -rf flag"        defer "$(bash_event 'grep -rf patterns.txt src/')"
check "word alarm"           defer "$(bash_event 'echo alarm && perform-task')"
check "git status"           defer "$(bash_event 'git status')"
check "jj new"               defer "$(bash_event 'jj new')"
check "jj op log"            defer "$(bash_event 'jj op log')"
check "jj git push"          defer "$(bash_event 'jj git push')"

# --- FREEZE half: edits outside the boundary should ask ----------------------
check "inside repo abs"      defer "$(edit_event '/repo/src/a.ts')"
check "inside repo rel"      defer "$(edit_event 'src/a.ts')"
check "outside repo"         ask "$(edit_event '/etc/passwd')"
check "escape via dotdot"    ask "$(edit_event '../../etc/passwd' '/repo/sub')"
check "sibling prefix trap"  ask "$(edit_event '/repo-other/x.ts')"

# --- FREEZE half: env-var boundary overrides ---------------------------------
check "narrow freeze in"     defer "$(edit_event '/repo/src/a.ts')" CLAUDE_FREEZE_DIR=/repo/src
check "narrow freeze out"    ask   "$(edit_event '/repo/README.md')" CLAUDE_FREEZE_DIR=/repo/src
check "project-dir boundary" defer "$(jq -nc '{tool_name:"Write",cwd:"/tmp",tool_input:{file_path:"/repo/x"}}')" CLAUDE_PROJECT_DIR=/repo
check "project-dir outside"  ask   "$(jq -nc '{tool_name:"Write",cwd:"/tmp",tool_input:{file_path:"/tmp/x"}}')" CLAUDE_PROJECT_DIR=/repo

# --- other tools are ignored -------------------------------------------------
check "Read ignored"         defer "$(jq -nc '{tool_name:"Read",cwd:"/repo",tool_input:{file_path:"/etc/passwd"}}')"

printf '\n%d passed, %d failed\n' "$pass" "$fail"
((fail == 0))
