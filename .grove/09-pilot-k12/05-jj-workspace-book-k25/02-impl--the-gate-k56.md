# the-gate-k56

## Goal

Draft `02-the-gate.md`, slice `one-lane`, and take the book to
`book-check --through one-lane --check all`.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *2 · The gate*. Thesis: **there is no second lane**, so the absence of a
  workspace is never a case to handle.
- Two blocks, 92 lines, both in `crates/jj-workspace/src/lib.rs` and separated
  by the whole of the `impl`: `workspace-value-and-gate` (`72-118`) and
  `gate-main-repo-and-canonical` (`275-319`).
- Required worked-example anchor `worked-resolution`, with three endings: the
  same tree resolved from a subdirectory; the same tree with no `.jj/`, ending
  in the two-line remedy with nothing created; and a secondary workspace
  resolving to a different main repo.
- This chapter states, as its premise, the one jj behaviour it rests on: a
  workspace that borrows another's repository has `.jj/repo` as a pointer
  **file**, one that holds its own has the repository directory there. Link jj's
  own documentation for it. One paragraph, never a primer.
- It reserves `CONTEXT.md#stated-vcs`, already declared in the manifest and
  already present in `CONTEXT.md` from `orientation-k55`.
- Two early-use rows are first used on this page and are the price of the
  narrative order: `jj::output` / `jj::produced_output` and
  `Refusal::not_a_workspace` / `Refusal::unresolvable_path`. Complete their
  minimum local statements at `#worked-resolution`.
- **Those two rows are this slice's to add to the ledger, and they are not in the
  manifest.** `orientation-k55` could not put them there: `book-validation`
  requires every manifest `[[early-use]]` row's first-use anchor to resolve in a
  page the snapshot holds, and scoped mode forbids page 02 from existing while
  chapter 1 is being proved. The specification's *Early-use ledger* already
  permits an author to add rows beyond the manifest's, which is what this is; the
  underlying disagreement is `early-use-scope-k63`'s to settle. Take the two rows'
  wording verbatim from the structure brief's *Early uses* table, give both
  `pending` until their owners land, and sort them after the five chapter-1 rows.

## Done when

- `02-the-gate.md` exists; both blocks are resolved, their defers replaced by
  inserts, `source-index.md`'s two rows moved to `resolved`, fragment index rows
  added, `README.md` and navigation updated, concept-index entries curated.
- `book-check --through one-lane --check all` exits 0.
- The draft stage record's `## Provenance` names this commit.
- `cargo test --locked --workspace`, `cargo clippy` and `cargo fmt --all --check`
  pass. `scripts/check.sh` is red on `book-check` alone.

## Notes

**Do not edit any page an earlier slice owns**, beyond the ledger rows and
navigation this slice is required to change. The stage boundary is a diff.
