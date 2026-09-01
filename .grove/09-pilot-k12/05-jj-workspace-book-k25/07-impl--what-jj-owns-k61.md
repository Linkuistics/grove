# what-jj-owns-k61

## Goal

Draft `07-what-jj-owns.md`, slice `assembly`, and close the book: green **final**
validation with no deferred holes, and a green `bash scripts/check.sh`. This is
the last commit of the draft stage.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *7 · What jj owns*. It owns **no** production source and is therefore
  final-only: it has no scoped prefix to prove, which follows from the slice
  owning no `[[block]]` rather than from any manifest field.
- It carries **no** worked-example section — six chapters do, and the seventh's
  job is synthesis (structure brief, *Worked examples*).
- What it must produce: one table assembling the six refusals into *what was
  subtracted, who owns it, and what the subtraction bought*; the test that
  separates a justified subtraction from an abdication — the underlying tool's
  guarantee must be at least as strong as the one the consumer needs, and
  checkable — applied to **all six**, including the one where the answer is least
  comfortable; the closed early-use and source ledgers; and the final
  verification commands with their results.
- The reader-facing outcome the whole book is for is stated here (decision 6 of
  `jj-workspace-structure-k17`): the reader can take a boundary in their own
  code, ask what the tool underneath already owns, and tell a justified
  subtraction from an abdication.

## Done when

- `07-what-jj-owns.md` exists; `README.md` lists all seven chapters as links with
  no plain-text future titles left; navigation's final-page form is on this page
  and `06-refusal.md` gains its Next.
- Every `[[early-use]]` row is `explained` and every ownership row is `resolved`.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/jj-workspace --final --check all` exits 0, reporting 4 roots,
  698 lines, zero deferred ranges and `final=true`.
- `bash scripts/check.sh` passes — **all** checks, including `book-check`. This
  is the child that closes the gate every earlier child left red.
- The draft stage record is complete: `## Provenance` lists all seven commits of
  the range, and `## Baseline` carries the final page inventory, the
  book-directory digest and the final-validation result.

## Notes

**This closes the draft stage and nothing more.** Developmental edit, technical
edit, copy edit, art and proof are `pilot-measure-k26`'s, one leaf and one commit
each. A synthesis chapter is a tempting place to smooth the six chapters ahead of
it; doing so would destroy the unedited baseline the pilot exists to measure
against.
