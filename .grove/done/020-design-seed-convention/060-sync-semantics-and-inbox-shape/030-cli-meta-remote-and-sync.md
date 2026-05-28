# 030-cli-meta-remote-and-sync

**Kind:** work

## Goal

Introduce the opt-in remote configuration verbs and the manual sync
verb. After this leaf retires, multi-machine users can run
`grove meta remote add <url>` to wire `grove-meta` to a remote, and
`grove meta sync` performs the push-pending + pull-latest catch-up
(the cron-friendly entry point).

## Scope

- **New sub-namespace: `grove meta remote`**, with subverbs:
  - `grove meta remote add <url>` — sets `git remote add origin
    <url>` on the `grove-meta` worktree, runs `git fetch origin
    grove-meta`, and configures `branch.grove-meta.remote=origin` and
    `branch.grove-meta.merge=refs/heads/grove-meta` so subsequent
    `git push` / `git pull` from inside the worktree work
    argument-less. Refuses if a remote is already configured (use
    `remove` first).
  - `grove meta remote remove` — unsets the upstream tracking and
    removes the origin remote. Local commits are not affected.
  - `grove meta remote list` — prints the configured remote(s) and
    upstream tracking state.
- **New verb: `grove meta sync`** — `git fetch origin` (ff-merging
  `grove-meta` into the worktree) followed by `git push origin
  grove-meta`. Both legs are best-effort with clear stderr messaging
  on failure. Exit non-zero only on conflict (non-ff after fetch, or
  push rejected after fetch). Designed to be cron-friendly: stdout
  silent on no-op, stderr informative on action or failure.
- **Parent-subcommand wiring.** If the `grove meta` parent
  subcommand does not already exist when this leaf executes
  (because leaf 070 has not yet added `grove meta init`), this leaf
  introduces it as an extensible parent. When 070 lands, it adds
  `init` to the existing parent — no conflict.
- **No auto-cron, no daemon.** The verb exists for users to cron at
  whatever cadence suits them. Document a representative cron line in
  the verb's `--help` text (e.g.
  `*/15 * * * * cd $(git rev-parse --show-toplevel)/.grove-meta &&
  grove meta sync >/dev/null` — adjust path discovery as needed).

## Out of scope

- Auto-scheduling (out of scope; user-owned cron).
- Multi-remote topologies / mesh sync — ADR-0005 explicitly defers
  this. Single upstream is the supported topology.
- Branch rename (covered by leaf 070).
- Authentication — whatever git auth the user has configured globally
  applies; grove does not own an auth subsystem.

## Done when

- `grove meta remote add|remove|list` exists, sets/clears upstream
  tracking on `add`/`remove`, and `list` reflects the current state
  accurately.
- `grove meta sync` exists and performs the fetch-then-push cycle
  with the documented best-effort semantics.
- A manual two-machine test (or simulated equivalent with two
  worktrees pointing at a shared bare repo) demonstrates: capture on
  A pushes to the bare repo; `grove meta sync` on B pulls A's
  observation into B's inbox directory; `grove inbox drain` on B
  sees the observation.
- ADR-0005 referenced from one pointer comment in the meta-remote/sync
  CLI's implementation site.

## Pointers

- ADR-0005: `docs/adr/0005-grove-meta-sync-semantics.md`
- Existing CLI entry: `src/cli.rs`, `src/inboxes.rs`.
- Parent BRIEF: `.grove/020-design-seed-convention/060-sync-semantics-and-inbox-shape/BRIEF.md`.

## Notes

- The `--help` text for `grove meta sync` should call out the cron use
  case explicitly; the post-mortem evidence (git-bug #1221, the
  Sapling/jj cache pattern) is the rationale for promoting this from
  "manual command users may forget" to "command we expect users to
  cron."
- `grove meta remote add` should refuse politely (exit non-zero with
  a clear message) if invoked on a `grove-meta` worktree that doesn't
  yet exist locally — `grove meta init` (leaf 070) is the precondition.
