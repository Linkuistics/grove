# jj command reference

The contract — which interface the repo picks, the working-copy-as-commit model,
the lane, the human-gated list, and the colocated git-read-only policy — is in
`using-jujutsu`'s `SKILL.md`. This file is the command surface. Facts verified
against jj 0.43.0.

## Reading state

| Want | Run |
|---|---|
| Status (always after mutating) | `jj st` |
| What's in `@` | `jj diff --git` |
| Recent history | `jj log -n 10` |
| This branch's work | `jj log -r 'trunk()..@'` |
| Undo target / audit trail | `jj op log` |

- Revsets to know: `@` (working copy), `@-` (its parent), `trunk()..@` (your
  work). More is rarely needed.
- `jj diff`'s default output is jj's own color-words format — unfamiliar, not
  corrupted. Pass `--git` for the familiar one.
- Verify with `jj st` after each mutation, the way you would check `git status`.

## Bookmarks are not branches

Nothing "checks out" a bookmark, and no bookmark is ever current. Two rules cover
almost everything:

- Bookmarks **follow rewrites** of the change they point at (describe, squash,
  rebase) — no manual re-pointing after history edits.
- Bookmarks **never advance** onto newly created changes — after sealing work with
  `jj new`, the bookmark still points where it did. Point it at the sealed change
  before pushing:

```bash
jj bookmark set <name> -r @-    # create or move the bookmark
```

(`jj bookmark move <name> --to <rev>` also works but only moves existing
bookmarks; `set` covers both cases.)

## Sharing work

```bash
jj bookmark set <name> -r @-        # point the bookmark at the sealed change
jj git push                          # push tracking bookmarks
jj git push --named <name>=@-        # first push of a brand-new bookmark
gh pr create ...                     # PRs are a forge concern — gh as usual
```

- `--named` is for *new* bookmark names only; for an existing bookmark,
  `bookmark set` + plain `jj git push`.
- Push rejects empty and undescribed commits by design. The fix is a description
  (or abandoning a stray empty change), not `--allow-empty-description`.
- Push is safety-checked like `git push --force-with-lease`: it updates the remote
  only if it matches the last fetch, so no force flag exists or is needed.
- Fetch with `jj git fetch`.
- Commit signing: if the repo or user config enables it, jj commands that write
  commits can fail in sandboxes with no signing key or agent (UNVERIFIED against
  0.43 — reported by prior art). Surface it to the user rather than working
  around it.

## The safety net

Every mutation — including fetches and snapshots — is one entry in the operation
log. `jj undo` reverts the most recent operation; repeating it walks further back,
and `jj redo` walks forward again. A wrong squash, a bad describe, even a botched
fetch are one `jj undo` away. What is *not* self-service — `jj abandon`,
`jj op restore`, `jj rebase -o`, `jj bookmark delete` + push — is in `SKILL.md`.

## Workspaces

```bash
jj workspace add ../<name>      # new working copy; workspace named after the basename
jj workspace list
jj workspace forget <name>      # then delete the directory
jj workspace update-stale       # after another workspace rewrote this one's @
```

## Per-harness enforcement (optional)

The contract depends only on a shell and works in any harness. Where a harness has
a mechanical pre-tool gate, a guard makes the git-read-only rule hold even when
prose is forgotten mid-session.

### Claude Code — PreToolUse deny-git guard

Architecture after kawaz/claude-plugin-jj (MIT), re-implemented. Two files in the
*project* — a per-repo choice, not something this skill installs. Unlike the
`guardrail` skill's ask-only gate, this one denies: inside a jj-enabled repo a git
mutation is never the right call, and the read-only git forms stay available.

`.claude/hooks/jj-guard.sh`:

```bash
#!/usr/bin/env bash
# Deny raw git mutations when the repo is jj-enabled. PreToolUse, matcher: Bash.
set -euo pipefail
input=$(cat)
cwd=$(printf '%s' "$input" | jq -r '.cwd // "."')
# Only guard where jj itself is usable — keeps the no-binary git fallback open.
command -v jj >/dev/null 2>&1 || exit 0
jj --ignore-working-copy -R "$cwd" root >/dev/null 2>&1 || exit 0
cmd=$(printf '%s' "$input" | jq -r '.tool_input.command // ""')
if printf '%s' "$cmd" | grep -qE '(^|[;&|(][[:space:]]*|&&[[:space:]]*|\|\|[[:space:]]*)git[[:space:]]+(add|am|apply|branch|checkout|cherry-pick|clean|commit|fetch|merge|mv|pull|push|rebase|reset|restore|revert|rm|stash|switch|tag)\b'; then
  jq -n '{hookSpecificOutput: {hookEventName: "PreToolUse",
    permissionDecision: "deny",
    permissionDecisionReason: "jj-enabled repo: drive mutations through jj; git is read-only here (see using-jujutsu)"}}'
fi
exit 0
```

`.claude/settings.json` (merge into any existing `hooks` block):

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR/.claude/hooks/jj-guard.sh\"" }
        ]
      }
    ]
  }
}
```

The decision-JSON contract matches the `guardrail` skill's tested hook in this
repo; see its script for the field-name provenance.

### Other harnesses

No pre-tool guard mechanism verified for Pi or Codex as of 2026-07 — there, the
detection contract in `SKILL.md` is the only layer. Add a recipe here only once
its mechanism is verified to exist.
