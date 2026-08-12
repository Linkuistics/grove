# general-jj-codex-permissions-k12

## Goal

Decide how Codex sessions get write access to a Jujutsu **store that lives
outside the working tree**, for *all* uses of Codex where jj is involved — not
only Grove-launched sessions in this one project.

## Context

Raised by the human during `configure-codex-jj-permissions-k11`, on seeing that
leaf's proposed fix:

> I'm not sure if all of this is specific to the current project and directory.
> This needs to be a solution for all uses of codex where jj is involved.

`k11` settled the **Grove** half only. Its `grove-jj` profile grants write to
`:workspace_roots`, and each Grove session's store arrives per launch through
`--add-dir ${repo}` in `~/.config/grove/config.kdl`. That generalises across
groves — the path is never hardcoded — but it generalises *because Grove passes
the flag*. Nothing carries that grant to Codex started any other way.

The uncovered case: a secondary jj workspace (`.jj/` is a pointer; the real
store is in the default workspace) where the human runs plain `codex`, or
`codex --profile sol-xhigh`, from the worktree. Those sessions resolve to the
machine default `default_permissions = ":workspace"`, which grants write to the
session workspace only. Committing touches the store outside it, so `jj`
commands that write will be denied — the same failure `k11` fixed for Grove.

Note this is **not a regression** from the legacy `workspace-write` sandbox,
which was equally worktree-scoped. It is a pre-existing gap that `k11` made
visible and fixed in one place.

## Done when

- A decision exists for how non-Grove Codex sessions reach the jj store, and the
  reasoning behind its breadth trade-off is recorded where it will be found
  again (an ADR if it clears the when-to-write bar; otherwise inline here and in
  the config comments).
- The chosen mechanism is implemented, or a follow-up `impl` leaf carries it.
- Whatever is chosen keeps the property `k11` established and
  `codex-jj-sandbox-permission-k10` re-checks: **no** full-filesystem write and
  **no** shell network access.

## Notes

**Verified facts from `k11`** (probed with `codex sandbox` under codex-cli
0.147.0 — re-derive rather than trust if the version has moved):

- `[permissions.NAME.filesystem]` accepts the special keys `:root`, `:minimal`,
  `:workspace_roots`, `:tmpdir`, `:slash_tmp`; values `read` / `write` / `deny`,
  with `deny` winning at equal specificity.
- `[permissions.NAME.workspace_roots]` with an explicit absolute path **does**
  yield write when `filesystem.":workspace_roots"."." = "write"`. Directly
  verified: a write to `/Users/antony/Development/grove/.jj/repo/` succeeded
  with the root declared and was denied without it.
- Entries accept absolute or `~`-relative paths. **No globs, no wildcards, no
  environment interpolation** — so "every repo under a directory" cannot be
  expressed as a pattern, only as that directory itself.
- Codex has **no VCS awareness** in the permission model: no automatic grant for
  a git common dir or linked worktree, and no repository-root special key.
- `--profile <name>` *layers* `$CODEX_HOME/<name>.config.toml` over the base
  config; it does not replace it.

**Candidate approaches, none yet chosen:**

1. **Declare a broad workspace root** — e.g. `"~/Development" = true` under
   `[permissions.grove-jj.workspace_roots]` (or a new profile). Covers every
   repo beneath it with no per-repo edit. Cost: write access to the whole
   development tree in any session selecting that profile, which is a real
   widening of the blast radius. The absence of globs means this is the *only*
   way to express "many repos" declaratively.
2. **Keep it per-launch** — document `--add-dir <store>` as the supported way,
   optionally behind a shell alias or wrapper that resolves
   `jj workspace root --name default` and passes it. Preserves tight scoping;
   cost is that it must be remembered, or a wrapper must be maintained.
3. **A dedicated jj profile file** — `~/.codex/jj.config.toml` carrying the
   permission profile plus `default_permissions`, selected with `--profile jj`.
   Isolates the policy, but Codex takes only one `--profile`, so it cannot
   compose with `sol-xhigh` and would have to duplicate its model settings.

**Sequencing:** prefer running this *after*
`codex-jj-sandbox-permission-k10`, which empirically settles whether `--add-dir`
registers as a session workspace root under the permission engine. If it does
not, approach 2 is dead and Grove's own configuration needs revisiting too.
