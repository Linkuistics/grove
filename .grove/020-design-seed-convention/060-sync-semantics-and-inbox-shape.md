# 060-sync-semantics-and-inbox-shape

**Kind:** planning

## Goal

Grill the remote-sync semantics of the `grove-meta` branch — when does
it push, when does it pull, what does the LLM see when two machines
both wrote — and **reconsider the inbox storage shape in light of those
semantics**.

Today's default state is **no remote configured for `grove-meta`**:
`grove install` (and the not-yet-renamed `grove meta init`) creates the
branch and the worktree, but does not add an upstream. That default
must be preserved: a single-machine user gets a working local-only
inbox with no surprise network behaviour. Multi-machine users opt in
explicitly via a new CLI verb that configures the upstream (and
possibly performs an initial fetch).

The current inbox shape (one markdown file per addressed grove at
`inboxes/<name>.md`) was chosen before sync was modelled. A candidate
alternative is "inbox as a directory of observation files, each with
a unique suffix" (UUID, machine-id, or username) so that multiple
writers can ff-push to `grove-meta` without merge conflicts. This
planning leaf decides:

1. Whether the inbox shape stays or changes (single file vs directory
   of files).
2. What the new opt-in CLI verb is for configuring a remote on
   `grove-meta` (`grove meta remote add <url>` or similar).
3. What the CLI must do at capture and drain time to keep multi-machine
   state consistent **when a remote is configured**, and explicitly do
   nothing remote-touching when one is not.

## Why this came before the rename

The rename leaf (`070-grove-meta-rename-and-init.md`) rewrites ADR 0002
and every reference to `inboxes/<name>.md`. If this leaf concludes the
inbox shape changes (directory of files, not a single file), those
references must change shape *and* path — doing the rename first would
edit them once for the rename and again for the shape. Cleanest
ordering: settle shape and sync semantics first, then a single ADR
rewrite covers both rename and shape.

## Context

- ADR 0002 (`docs/adr/0002-grove-inboxes-branch-and-inbox-model.md` —
  not yet renamed) currently defines an inbox as "a markdown file at
  `inboxes/<name>.md`" and is silent on sync. Read it first.
- ADR 0003 (`docs/adr/0003-cross-repo-inbox-handoff.md`) requires the
  target repo's worktree to be present locally for cross-repo writes.
  That covers cross-repo sync but leaves multi-machine sync of the
  *same* repo unaddressed.
- The feature commit (`b71c6d5`) does no `git fetch`, no `git push`,
  and no merge on `grove-meta`. `grove inbox add` is a local
  path-scoped commit; `drain` is a local clear-and-commit. Anything
  that needs to move between machines today must be pushed/pulled by
  the user manually. This is the gap to close (or to consciously
  ratify as "manual sync is fine for v1").
- The grove project's own repo is the immediate dogfood: the user
  works across multiple machines, so the multi-writer path is not
  hypothetical.
- Glossary (`CONTEXT.md`) terms in play: `Inbox`, `Seed`, `Drain`,
  `grove-meta branch`. The `Inbox` and `grove-meta branch` entries
  will update inline as decisions resolve; do not pre-edit before the
  grilling settles them.

## Questions to grill

These are the design-tree branches the planning session should walk.
The grilling procedure (`grilling.md`) asks them one at a time with a
recommended answer; this list is a checklist of *what must be settled
before this leaf retires*, not a script.

1. **Drain pull**. Before drain, does the bootstrap step `git fetch` +
   ff-merge `grove-meta` so the session triages the latest known
   state? If yes, what happens if fetch fails (offline, no remote,
   auth)? Recommended default: fetch best-effort, warn-and-continue
   on failure; the inbox is still useful locally even if disconnected.
2. **Capture push**. After `grove inbox add` commits, does the CLI
   `git push` to the remote? If yes, what happens if push is rejected
   (non-ff from another machine's interleaved write)? Recommended
   default: push best-effort, *fail loudly with a clear remediation*
   on non-ff so the user/LLM does not assume the observation reached
   the addressed grove.
3. **Drain commit push**. After drain clears the file, does the
   commit push? Same conflict shape as 2. Recommended: yes, same
   best-effort + loud-on-non-ff rule.
4. **Inbox shape — file or directory**. The candidate is: replace
   `inboxes/<name>.md` (one file) with `inboxes/<name>/<entry-id>.md`
   (a directory of single-observation files). With disjoint paths,
   two machines' captures cannot conflict on push; ff merges become
   the common case rather than the exception. Trade-offs to grill:
   - **For directory shape**: ff-conflict-less multi-writer; drain
     becomes "iterate over directory contents"; each observation is
     its own atomic unit (timestamped, attributable); `git log
     inboxes/<name>/` shows per-observation history rather than
     per-batch history.
   - **Against directory shape**: more files (modest), the "single
     markdown file you can `cat`" affordance disappears (mitigated
     by `grove inbox show <name>` already concatenating for display),
     ADR 0002 needs amendment (or supersession by a new ADR), the
     drain CLI's clear-and-commit becomes "remove all files in
     directory and commit" rather than "truncate and commit".
   - **A middle path**: keep the single-file shape but require
     `grove inbox add` to pull-rebase-then-append-then-push under a
     lock. Simpler data model, more failure modes (rebase races,
     editor races on the worktree file).
5. **Entry naming if directory shape wins**. Candidates:
   `<UTC-iso8601>-<short-uuid>.md`, `<UTC-iso8601>-<hostname>.md`,
   `<UTC-iso8601>-<user>-<short-uuid>.md`. The constraint: globally
   unique under multi-writer, sortable by capture time, scrutable
   when listing the directory by hand. Recommended:
   `<UTC-iso8601>-<short-uuid>.md` — uuid covers uniqueness without
   leaking hostname/user; timestamp prefix keeps chronological order.
6. **What about the cleared-after-drain state?** Today drain commits
   an empty file; the file's existence is the signal "this grove is
   known." Under directory shape, drain removes all files but leaves
   the directory (with a `.gitkeep` or similar)? Or removes the
   directory entirely and recreates on next add? Recommended: keep
   the directory (with `.gitkeep`) so the directory's presence is
   still the "known grove" signal.
7. **Cross-machine sync of the worktree itself.** The `grove-meta`
   worktree on machine A and on machine B are independent
   filesystems; nothing keeps them in lockstep beyond manual push.
   Should `grove meta init` set the upstream tracking branch so push
   is `git push` with no args? Recommended: yes.
8. **Lock/race semantics inside one machine.** Two `grove inbox add`
   commands running concurrently on the same machine touch
   `<repo>/.grove-meta/`. Under directory shape, they cannot
   collide (disjoint paths). Under single-file shape they can. This
   reinforces the directory-shape recommendation but is worth
   recording explicitly.

## Done when

- Each question above has a recorded decision (in this file's notes
  during grilling, or — for the durable ones — in ADR 0002's revision,
  or in a new ADR 0004 if the shape changes substantively).
- If the inbox shape changes: ADR 0002 is updated (or superseded by a
  new ADR), and the `070-grove-meta-rename-and-init.md` leaf's
  "Done when" is amended to include the shape change in the same
  rewrite. `CONTEXT.md`'s `Inbox` entry is updated inline (path,
  shape, and example).
- If the inbox shape stays single-file: the sync mechanism (locking,
  pull-rebase, conflict handling) is recorded as a new ADR.
- The CLI work needed to implement the chosen sync semantics
  (fetch-before-drain, push-after-add, push-after-drain, locking if
  applicable, shape change if applicable) is captured either as new
  child leaves of this planning leaf (decomposing it into
  `060-sync-semantics-and-inbox-shape/`) or by amending the
  `070-grove-meta-rename-and-init.md` leaf's scope. Choose by
  size — if the additional CLI work is small enough to fit alongside
  the rename, fold it in; otherwise decompose.

## Notes

- **Resist scope creep into the rename.** The rename leaf's job is
  rename + `grove meta init`. If shape changes here, the *next* leaf
  (rename + init + shape) covers the implementation. Do not start
  renaming code in this planning session.
- **Resist scope creep into the TUI.** The TUI leaf (070) reads
  whatever shape this leaf chooses; do not anticipate its needs while
  grilling.
- **Walk-away-ability still rules.** Whatever shape and sync semantics
  are chosen must keep the property: delete the `grove` CLI and the
  contents of `<repo>/.grove-meta/` are still plain markdown files on
  a plain git branch, legible by `git log` and `cat`.
- **No PRD unless a genuine agreement point emerges.** ADR amendments
  and the in-leaf decision log are likely sufficient; if the shape
  change feels like a published-spec moment, raise a PRD then.
