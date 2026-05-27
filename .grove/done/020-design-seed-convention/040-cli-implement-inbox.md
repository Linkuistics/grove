# 040-cli-implement-inbox

**Kind:** work

## Goal

Implement the CLI surface that makes the inbox convention real and
enforces process correctness: `grove install` materialises the
`grove-inboxes` branch and the `.grove-inboxes/` worktree; `grove start`
and `grove continue` perform inbox drain at session bootstrap; a new
verb (provisionally `grove inbox <name>`) appends an observation to the
named inbox.

## Context

- The two ADRs from sibling leaves 010 and 020 — these define the
  contract this implementation upholds. Read them first.
- Existing CLI in the repo: explore `src/` for the `install`, `update`,
  `start`, `continue` verb implementations. The new code follows the
  same patterns (path-scoped commits, harness-respecting install paths,
  etc. — see the existing `Install scope` and `Path-scoped commit`
  glossary entries in `CONTEXT.md`).
- `docs/adr/0001-install-and-update-create-commits.md` — establishes
  the auto-commit semantics; the new `grove inbox <name>` write should
  follow the same pattern (path-scoped commit, only the changed inbox
  file).

## Done when

- `grove install` (and `grove update` where appropriate) materialises:
  - the `grove-inboxes` branch (created from the default branch if it
    doesn't already exist remotely; near-orphan / empty-tree start is
    fine);
  - the `<repo>/.grove-inboxes/` worktree on that branch with an
    `inboxes/` subdirectory.
- `grove start <name>` and `grove continue <name>` both perform a
  drain step at session bootstrap: print the contents of
  `<repo>/.grove-inboxes/inboxes/<name>.md` (if any) for the session to
  triage. After triage, commit the cleared inbox file (path-scoped
  commit on the `grove-inboxes` branch).
- New verb `grove inbox <target-grove-name>` (or whatever naming the
  implementation settles on) accepts an observation via argument or
  stdin, appends it to `<repo>/.grove-inboxes/inboxes/<name>.md`
  (creating the file if absent), and commits the change. Works
  regardless of whether `<target-grove-name>` has a worktree.
- Cross-repo case: `grove inbox --repo <path> <target-grove-name>`
  (or equivalent) writes to a *different* repo's inbox worktree at
  `<path>/.grove-inboxes/inboxes/<name>.md`. Path discovery itself is
  out of scope (see ADR 0003); the verb only needs to accept an
  explicit path.
- Tests cover: install materialisation idempotency; drain on
  start/continue with empty and non-empty inboxes; append against an
  existing inbox, a brand-new seed, and a cross-repo path; the
  no-direct-git invariant (the LLM never has to call `git` or `mv`
  directly).
- `grove finish` is updated only minimally: it does NOT touch the
  finished grove's inbox file (the brief-chain promotion already
  drains anything relevant on the last session). The inbox file is
  left on the branch — it just becomes a seed again if observations
  arrive later.

## Notes

- If the work turns out too big for one focused session, this leaf
  becomes a planning task that decomposes into install-materialisation
  / drain-on-bootstrap / capture-verb sub-leaves. Don't pre-decompose
  speculatively.
- Resist scope creep into the TUI. The 050 leaf handles navigation;
  this leaf only needs the verbs that capture and drain.
- The CLI is the enforcement boundary: the LLM should be able to
  perform every inbox gesture through `grove` subcommands without
  knowing the underlying file paths, branch names, or git plumbing.
  Where a verb's signature feels awkward, that is signal the convention
  needs a tweak — record it (don't paper over with hidden complexity in
  the implementation).
