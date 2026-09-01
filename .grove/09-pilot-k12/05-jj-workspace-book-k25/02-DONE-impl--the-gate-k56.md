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

## Decisions (running log)

**1 · The worked example precedes every fragment on the page, and chapter 1's
order is not followed.** Orientation put its manifest fragments ahead of
`#commit-tour`; this page cannot. Both early-use rows this slice adds declare
`02-the-gate.md#worked-resolution` as their first use, and the ledger sorts by
anchor occurrence in the page, so any fragment that named `Refusal::not_a_workspace`
or reached the subprocess seam before that anchor would make the declared first
use false. Placing the trace first also satisfies the book contract's
*catalogue after the example* rule for free. Rejected: declaring a different
first-use anchor, which would have contradicted the structure brief's *Early
uses* table, whose wording the task requires verbatim.

**2 · The two blocks are refined into five and three intent-named children.** The
specification permits refinement and the structure brief leaves the partition to
the owning slice; the top-level IDs, owners, ranges and line-count credit are
unchanged. Forty-seven lines would otherwise be one fence carrying four separate
arguments — the value's private fields, the constructor's three claims, the walk
itself, and two accessors that spawn nothing — and `M105`'s one-paragraph-per-
literal rule is what makes the split pay. The `impl Workspace {` line is grouped
with `resolve`'s doc comment rather than with the walk, so the fence a reader
meets first is the argued contract and the fence after it is nine lines of code
the contract has already explained.

**3 · The pointer-file premise is cited to jj's glossary at `docs.jj-vcs.dev`,
and corroborated by inspection rather than by memory.** The structure brief
requires a link to jj's own documentation. `https://jj-vcs.github.io/jj/latest/`
now answers `301` to `docs.jj-vcs.dev`, and no `workspaces/` page exists under
`latest/`; the glossary's *Workspace* entry is where jj states that non-initial
workspaces hold pointers. The file-versus-directory shape itself is documented
nowhere official, so it was verified directly on jj 0.44.0 — a `jj git init`
workspace has `.jj/repo` as a directory, and a `jj workspace add` sibling has it
as a nineteen-byte file — and the page says so rather than asserting it. The
stale URL still in `refusal.rs` is a corpus change and was externalised as
`jj-docs-url-k64`, not fixed here.

**4 · No in-session reviewer, and no `review-impl` leaf.** The leaf-wide
allowance is unspent deliberately. This is the draft stage of a preregistered
measurement whose entire value is being an **unedited** baseline: five editorial
stages are already scheduled against it at `pilot-measure-k26`, and a reviewer
run here would either pre-empt those stages' claims or contaminate the baseline
they are diffed against. Byte-exactness is proved mechanically by `book-check`
rather than by judgement, which is the class of doubt an in-session reviewer
would otherwise be spent on.
