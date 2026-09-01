# scope-and-commit-k59

## Goal

Draft `05-scope-and-commit.md`, slice `no-transactions`, and take the book to
`book-check --through no-transactions --check all`.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *5 · Scope and commit*. Thesis: **no transactions, and reads add no history.**
- Two blocks, 139 lines, in `crates/jj-workspace/src/lib.rs`: `commit-identity`
  (`62-71`) and `scope-tracking-and-commit` (`146-274`).
- Required worked-example anchor `worked-commit`: the carried operation at **full
  resolution** — the same starting tree, names and change id `orientation-k55`
  fixed, a caller's path through `relative` and `fileset` into the exact
  `jj commit` argv, the change-id read, and the `Commit` returned.
- The chapter's central claim is the **asymmetry**: every probe passes
  `--ignore-working-copy` except `is_tracked`, whose answer depends on the
  working copy, and that exception is *measured* (jj 0.44.0) rather than assumed.
  Name the measurement as a measurement.
- Two jj behaviours are stated here as premises: the working copy is snapshotted
  before every command and the operation log *is* the transaction record; and a
  change id survives `describe`, `squash` and a rebase.
- Carry the path algebra as one argument rather than three functions: `resolve`
  canonicalising the root is *why* `relative` must canonicalise the caller's
  path, and canonicalising the **parent** is why a path the caller has just
  deleted is still committable.
- It reserves `CONTEXT.md#task-commit-boundary`.

## Done when

- `05-scope-and-commit.md` exists; both blocks resolved, defers replaced, ledger
  rows moved to `resolved`, fragment index rows added, navigation and contents
  updated, concept-index entries curated.
- The `Commit` and `is_tracked` early-use rows move to `explained`.
- `book-check --through no-transactions --check all` exits 0.
- The draft stage record's `## Provenance` names this commit.
- `cargo test --locked --workspace`, `cargo clippy` and `cargo fmt --all --check`
  pass. `scripts/check.sh` is red on `book-check` alone.

## Notes

**Two refusals mean different things and the chapter must separate them**: the
refusal from the commit itself means there is no commit; the refusal from reading
the change id afterwards means the commit landed and could not be named.
