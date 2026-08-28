# drop-git-lane-k7

## Goal

Make jj the only lane. A non-jj working tree is refused **before any mutation**,
with the command that fixes it; every plain-git branch, and the colocated
git-index machinery that exists only because grove drives git plumbing, is
deleted.

## Context

- `docs/specs/module-decomposition.md`, decision 8 and `## Out of scope`
  (*"The plain-git lane. Dropped."*). The rejected alternative — narrowing the
  safety principle to *where the version control system can* — is what keeps the
  finish transaction alive on one lane and leaves the VCS seam the largest of the
  five modules; it is rejected, not open.
- `minimalism-k1`'s `## Deletion list`, *Reconciled* row 5: `repo.rs`'s git lane
  — `git_index_path`, the empty-internal-hooks-path rule, and every plain-git
  branch; **`vcs_of` survives as a precondition gate rather than a dispatch**.
- `src/finish_cleanup/auxiliary*` (1,257 + 960 + 1,634 test lines) — the
  colocated git-index backup, which `minimalism-k1` records exists *only*
  because grove drives git plumbing on a colocated repo: **jj has no index**.

## Done when

- `vcs_of` (or its successor) is a precondition gate: a working tree that is not
  jj-enabled is refused with `jj git init --colocate` named as the remedy, and
  nothing downstream branches on lane.
- No plain-git branch survives anywhere in `src/`. Verify by enumerating every
  git-shelling call site and classifying each, not by sweeping a pattern list —
  `references/execute.md`'s rule, including its positive and cross-tree controls.
- The git-index backup family is deleted.
- `docs/adr/jj-is-the-only-lane.md` is **added**: dropping plain Git is
  hard-to-reverse, surprising without the safety principle behind it, and carries
  a real rejected alternative. `ADR-FORMAT.md` governs its shape. Once the spec
  is rewritten to current state (`spec-to-current-state-k23`) there is no other
  record saying why, which is precisely the when-to-write test.
- `docs/adr/grove-does-not-stage-its-own-renames.md` is **amended**: the decision
  survives and gets simpler; its Git-lane consequences go. Its migration
  references go too — `delete-migration-k6` has already landed by then.
- `docs/USAGE.md`, `docs/ARCHITECTURE.md` and `docs/CONFIGURATION.md` no longer
  describe a git lane.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is the precondition for both leaves after it: the VCS
seam cannot state *"a non-jj working tree is refused before any mutation"* as its
own guarantee while a git branch still exists behind it, and the finish
transaction's deletion is far smaller once the auxiliary family is already gone.

**Do not extract the crate here.** `extract-jj-workspace-k9` does that. This leaf
narrows `repo.rs`; it does not move it.

**The refusal is the product.** Principle 2 — an error that only reports
detection is unfinished. `TaskNameError` (`src/task_name.rs:314`) is the model:
every variant carries what is on disk *and* what it should be.
