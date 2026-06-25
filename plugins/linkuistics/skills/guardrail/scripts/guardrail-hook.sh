#!/usr/bin/env bash
# guardrail-hook.sh — a PreToolUse permission gate for the `guardrail` skill.
#
# Claude Code runs this once per matched tool call, passing the PreToolUse event
# as JSON on stdin (fields: .tool_name, .tool_input, .cwd). The script decides:
#   - emit a `permissionDecision: "ask"` object on stdout  -> Claude pauses for
#     explicit user confirmation before the tool runs;
#   - emit nothing (exit 0)                                 -> normal permission
#     flow (the guardrail had no opinion — "defer").
# It NEVER emits "deny": a guardrail pauses, it does not block. The human decides.
#
# Two concerns, switched on .tool_name:
#   - Bash            -> warn before destructive shell commands (the CAREFUL half)
#   - Edit/Write/...  -> warn before edits OUTSIDE the boundary (the FREEZE half)
#
# The FREEZE boundary defaults to the project root and can be narrowed with an
# env var; see `boundary_dir`. The destructive-command list is the `DANGER`
# array below — it is meant to be read and tuned. Add or remove patterns freely;
# each entry is `ERE-regex<TAB>reason`.
set -euo pipefail
IFS=$'\n\t'

# --- decision emitters -------------------------------------------------------

# ask <reason>: emit a PreToolUse "ask" decision and exit. The reason is shown
# to the user in the confirmation prompt.
ask() {
  jq -n --arg reason "$1" '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "ask",
      permissionDecisionReason: $reason
    }
  }'
  exit 0
}

# defer: no opinion — let the normal permission flow proceed.
defer() { exit 0; }

# --- freeze: resolve the edit boundary --------------------------------------

# boundary_dir <cwd>: the directory edits must stay within. Precedence:
#   CLAUDE_FREEZE_DIR (explicit narrow boundary) > CLAUDE_PROJECT_DIR (repo root,
#   set by Claude Code) > the session cwd from the event.
boundary_dir() {
  printf '%s' "${CLAUDE_FREEZE_DIR:-${CLAUDE_PROJECT_DIR:-$1}}"
}

# canonicalize <path>: lexically resolve '.' and '..' in an absolute path
# without touching the filesystem, so it works for not-yet-created files and
# cannot be fooled by a `../../escape`. Pure bash (3.2-safe): no realpath, no
# negative array indices.
canonicalize() {
  local seg n=0 i=0 res=''
  local -a segs out
  IFS='/' read -ra segs <<<"$1"
  for seg in "${segs[@]}"; do
    case "$seg" in
    '' | '.') ;;
    '..') if ((n > 0)); then n=$((n - 1)); fi ;;
    *)
      out[n]="$seg"
      n=$((n + 1))
      ;;
    esac
  done
  while ((i < n)); do
    res="$res/${out[i]}"
    i=$((i + 1))
  done
  printf '%s' "${res:-/}"
}

# --- the destructive-command table (the CAREFUL half) ------------------------
# ERE patterns matched case-insensitively against the whole Bash command line.
# Tuned to catch genuinely irreversible / data-losing operations while keeping
# false alarms low (an over-eager gate trains you to rubber-stamp it).
DANGER=(
  $'(^|[^[:alnum:]_])rm[[:space:]]+([^|;&]*[[:space:]])?-[a-z]*r[a-z]*f|-[a-z]*f[a-z]*r\trecursive forced delete (rm -rf)'
  $'(^|[^[:alnum:]_])rm[[:space:]]+([^|;&]*[[:space:]])?--recursive([[:space:]].*)?--force\trecursive forced delete (rm --recursive --force)'
  $'git[[:space:]]+push([[:space:]].*)?[[:space:]](-f|--force|--force-with-lease)([[:space:]]|$)\tforce push — rewrites remote history'
  $'git[[:space:]]+reset[[:space:]].*--hard\tgit reset --hard — discards working-tree changes'
  $'git[[:space:]]+clean[[:space:]]+-[a-z]*f\tgit clean -f — deletes untracked files'
  $'(^|[^[:alnum:]_])(drop|truncate)[[:space:]]+(table|database|schema)\tdestructive SQL (DROP/TRUNCATE)'
  $'(^|[^[:alnum:]_])mkfs(\\.| )\tmkfs — formats a filesystem'
  $'(^|[^[:alnum:]_])dd[[:space:]].*[[:space:]]of=/dev/\tdd writing to a raw device'
  $'>[[:space:]]*/dev/(sd|nvme|disk|hd)\tredirect overwriting a raw device'
)

# --- main --------------------------------------------------------------------

input="$(cat)"
tool_name="$(jq -r '.tool_name // empty' <<<"$input")"
cwd="$(jq -r '.cwd // empty' <<<"$input")"
[[ -n "$cwd" ]] || cwd="$PWD"

case "$tool_name" in
Bash)
  command="$(jq -r '.tool_input.command // empty' <<<"$input")"
  [[ -n "$command" ]] || defer
  entry=''
  for entry in "${DANGER[@]}"; do
    pattern="${entry%%$'\t'*}"
    reason="${entry#*$'\t'}"
    if grep -Eiq -- "$pattern" <<<"$command"; then
      ask "guardrail: $reason. Confirm you intend to run: $command"
    fi
  done
  defer
  ;;
Edit | Write | MultiEdit | NotebookEdit)
  file_path="$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$input")"
  [[ -n "$file_path" ]] || defer
  [[ "$file_path" = /* ]] || file_path="$cwd/$file_path"
  boundary="$(canonicalize "$(boundary_dir "$cwd")")"
  target="$(canonicalize "$file_path")"
  if [[ "$target" != "$boundary" && "$target" != "$boundary"/* ]]; then
    ask "guardrail (freeze): $target is OUTSIDE the boundary $boundary. Confirm this edit beyond the project."
  fi
  defer
  ;;
*)
  defer
  ;;
esac
