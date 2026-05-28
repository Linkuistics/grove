# 060-grove-meta-rename-and-init

**Kind:** work

## Goal

Rename the shared coordination branch from `grove-inboxes` to `grove-meta`
(worktree at `<repo>/.grove-meta/`), and expose an explicit
`grove meta init` verb that creates the branch and attaches the worktree
idempotently. `grove install` / `grove update` continue to materialise
the branch by calling the same code path internally, so a fresh install
needs no extra step. Existing installs that pre-date this feature, or
repos whose worktree has been deleted, are repaired by running
`grove meta init`.

## Why this came before the TUI

The TUI planning leaf (now `080-tui-server.md`) was about to start
grilling against the durable state defined by the surrounding ADRs.
That state's *identity* — the branch's name — is in flux: ADR 0002
already reserves the branch "more broadly for cross-grove coordination
data," but its name lies about that scope. Renaming after the TUI
references the old name would be a migration; renaming now is a single
focused edit. Done before the TUI is grilled so the TUI is designed
against the final names.

The sync-and-shape planning leaf (`050-sync-semantics-and-inbox-shape.md`)
was inserted ahead of this leaf in a later planning session. If that
leaf concludes the inbox storage shape changes (e.g. directory-of-files
instead of single file), this leaf's references to `inboxes/<name>.md`
and the ADR 0002 rewrite must reflect the new shape in the same commit.
Check the outcome of 050 before starting work here.

## Context

- ADR 0002 (`docs/adr/0002-grove-inboxes-branch-and-inbox-model.md`)
  is the load-bearing record. Its title, filename, and content all
  reference `grove-inboxes`. The rename rewrites both. Decision history
  is preserved by a single sentence noting the previously-considered
  name and why it was rejected.
- ADR 0003 (`docs/adr/0003-cross-repo-inbox-handoff.md`) references
  `.grove-inboxes/` paths in its body; update in place.
- The feature commit (`b71c6d5 feat(inboxes): implement grove-inboxes
  branch, capture verbs, and drain prompts`) is the implementation
  surface to rename. `git grep grove-inboxes` and
  `git grep grove_inboxes` from repo root will enumerate hits across
  `src/`, tests, `content/SKILL.md`, `content/prompts/`, `README.md`,
  and `docs/`.
- `CONTEXT.md` was already updated inline during planning: the term is
  `grove-meta branch`, the worktree path is `<repo>/.grove-meta/`, and
  the rejected alternative is recorded in the entry.
- The existing inbox verbs stay where they are: `grove inbox add`,
  `grove inbox drain`, `grove inbox show`. They write to/read from the
  renamed branch but their own surface does not change. The new
  `grove meta` noun is a separate verb cluster; today it has exactly
  one verb (`init`), with room for `grove meta path` /
  `grove meta status` later if a real need surfaces (do not add them
  speculatively).

## Done when

- The branch is named `grove-meta` and its worktree lives at
  `<repo>/.grove-meta/` everywhere it is referenced: CLI code,
  constants, tests, ADRs, `content/SKILL.md`, `content/prompts/`,
  `README.md`, `docs/grove.md`, `docs/concepts.md`, `docs/workflows/`,
  and any other doc that mentions `grove-inboxes`. `git grep
  grove-inboxes` (and the snake_case form) returns zero hits outside
  `.grove/done/` (historical task files are not rewritten).
- `grove meta init` exists as a CLI verb:
  - Creates the `grove-meta` branch if absent (orphan / empty-tree
    start, matching how the `grove-inboxes` branch is created today).
  - Attaches the worktree at `<repo>/.grove-meta/` with an `inboxes/`
    subdirectory if absent.
  - Is idempotent: running it when both branch and worktree are
    already present is a no-op that prints the materialised state
    (path + branch name) on a single line.
  - Returns a non-zero exit and a useful error if the repo is not a
    git repo, or if the worktree path is occupied by something
    unrelated (mirrors the existing inbox-worktree error path).
- `grove install` and `grove update` invoke the same code path as
  `grove meta init` internally so a fresh install still produces a
  working setup in one command. The existing tests for install/update
  materialisation continue to pass with the renamed paths.
- ADR 0002 is renamed to
  `docs/adr/0002-grove-meta-branch-and-inbox-model.md`, its title and
  body rewritten to use `grove-meta`, with one sentence recording the
  rejected `grove-inboxes` name and the reason (scope narrowing). ADR
  0003's body uses the new paths. ADR statuses remain `accepted`; no
  new ADR is needed (the decision recorded in 0002 — "branch reserved
  more broadly" — did not change; only the chosen name did).
- `content/SKILL.md`'s artifacts table and inbox section refer to
  `grove-meta` and `<repo>/.grove-meta/inboxes/<name>.md`. The
  `start.md` / `continue.md` launcher prompts point at the new path.
- New tests cover: `grove meta init` on a fresh repo creates branch +
  worktree; `grove meta init` on an already-materialised repo is a
  no-op with status output; `grove meta init` invoked by `install`
  produces the same end state as running it standalone. Existing
  inbox tests (append, drain, cross-repo, seed compatibility) are
  updated to reference the renamed branch and continue to pass.
- The full test suite is green.

## Notes

- **No migration logic.** The `grove-inboxes` branch has shipped in
  zero published releases — the feature commit
  (`b71c6d5`) is on `capture-issues-for-later-groves`, not on
  `main` — so no installed repo has a `grove-inboxes` branch in
  production. The rename is a one-commit edit, not a migration with
  shims. If a working tree on this branch happens to have a
  `.grove-inboxes/` worktree from local testing, the developer
  removes it by hand; the CLI does not need a `meta migrate` verb.
- **ADR 0002 filename change.** Renaming the ADR file (not just its
  contents) is unusual but justified: the filename appears in cross
  references (e.g. `content/SKILL.md`'s artifacts table cites the
  ADR filename). Update those references in the same commit. Use
  `git mv` so history is preserved.
- **Do not relocate inboxes inside the branch.** `inboxes/<name>.md`
  stays at that path on `grove-meta`. The mental model is "the branch
  has an `inboxes/` subdirectory today; other subdirectories may
  appear later." Do not pre-create `locks/`, `settings/`, etc.; lazy
  and optional (SKILL.md constraint 4).
- **Verb namespace.** `grove meta` is a noun-verb cluster
  (`grove meta init` today, possibly more later). `grove inbox` stays
  its own noun cluster (the two are different things at the same
  level of abstraction — inboxes are a *content type* living inside
  meta, but the verb namespaces are sibling, not nested, to keep the
  common verbs short).
- **`grove finish` is unaffected.** No code touches inbox/meta state
  at finish; the inbox file just becomes a seed again if anything is
  appended later. ADR 0002's "Why drain is a bootstrap step" still
  applies as written.
- **Resist scope creep into the TUI.** The TUI grilling (now
  `060-tui-server.md`) is the next planning step. This leaf is
  rename + one CLI verb, nothing more.
