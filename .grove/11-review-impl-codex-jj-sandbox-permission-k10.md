# codex-jj-sandbox-permission-k10

**Reviews:** configure-codex-jj-permissions-k11

## Goal

Disprove that the scoped Grove Codex permission policy is sufficient for the jj
commit-and-retire boundary, especially in a secondary jj workspace.

## Context

`configure-codex-jj-permissions-k11` changes the personal Codex and Grove
configuration through Claude. This review is intentionally routed back through
Codex. Reaching this session with the expected permission profile is the first
check; successfully retiring, describing, and sealing this review is the
end-to-end check that previously failed.

Inspect the effective session permissions and the two configuration files. As a
`review-impl` leaf, produce findings only: do not edit either configuration or
implementation/test code, and do not rerun product tests or builds.

## Done when

- The effective Codex session can write the current worktree's jj metadata and
  `/Users/antony/Development/grove/.git/objects`.
- The profile remains scoped to runtime workspace roots and does not grant full
  filesystem or network write access.
- Findings are recorded in this leaf, then its ordinary Grove
  retire/describe/seal boundary succeeds without human jj intervention.

## Notes

- If the commit boundary still fails, record the exact denied path and stop;
  do not broaden the permission profile during this inspection-only review.

## Handover from `configure-codex-jj-permissions-k11`

Written by the producer session so this review starts from evidence rather than
re-deriving it. Treat every claim as a target to disprove, not as established.

**Two deviations from `k11`'s written spec, both deliberate:**

1. **A global `default_permissions` was set, against that leaf's Note.** Codex
   0.147.0 refuses to start *any* session when `[permissions.*]` profiles exist
   but no top-level `default_permissions` does:
   `Error: config defines [permissions] profiles but does not set
   default_permissions`. Implemented literally, the spec would have broken every
   non-Grove `codex` invocation on this machine. The human chose
   `default_permissions = ":workspace"` — a built-in, not `grove-jj`, so the
   spec's actual requirement (grove-jj is never the machine default) holds.
2. **The repo path was deliberately *not* hardcoded** as a profile workspace
   root, on the human's instruction that the fix must generalise beyond this
   project. Store write therefore rests entirely on `--add-dir ${repo}`
   contributing to `:workspace_roots` — see the open question below.

**`Done when` criterion 4 of `k11` is vacuous — do not accept it as evidence.**
`codex --strict-config --version` exits 0 even against a config containing a
deliberate TOML **syntax error**; `--version` short-circuits before the config
is loaded. It cannot fail, so it proves nothing. Commands that *do* load and
validate config, and that reported the syntax error correctly:
`codex debug models` and `codex sandbox -- <cmd>`. Both exit 0 on the config as
committed.

**Probe results** (`codex sandbox`, cwd = this worktree, `-c
default_permissions=grove-jj`):

| Probe | Result |
|---|---|
| write inside the worktree | allowed |
| write to `$HOME` outside the worktree | denied |
| read `/etc/hosts` | allowed (`:root = "read"`) |
| network egress (`curl https://example.com`) | denied |
| write `/Users/antony/Development/grove/` (the store) | **denied** |
| same, with that path declared a profile `workspace_roots` entry | allowed |
| plain `codex` with no `-c` override (global `:workspace`) | works; writes in cwd, denied outside |

**The one open question this review is positioned to settle.** `codex sandbox`
has no `--add-dir` flag, so the producer could not test the mechanism the design
depends on: **does `--add-dir ${repo}` register as a session workspace root
under the permission engine, and therefore inherit
`filesystem.":workspace_roots"."." = "write"`?** The documentation says
`:workspace_roots` means "current session roots plus profile-defined roots" but
never mentions `--add-dir`. Only a real Codex session launched from Grove's
templates can answer it — which is this session. **Your own retire/describe/seal
boundary is the experiment**: if it succeeds, `--add-dir` feeds the permission
engine; if it fails, record the exact denied path per the Note above.

A negative result also invalidates approach 2 in
`general-jj-codex-permissions-k12`, which was raised for the non-Grove half of
this problem and is sequenced to run after this review.
