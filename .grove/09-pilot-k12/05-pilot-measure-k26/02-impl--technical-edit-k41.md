# technical-edit-k41

## Goal

Run the **technical edit** over the `jj-workspace` book: is what the book says
about the code true, complete for its scope, and not silently stale against the
frozen corpus. One commit.

## Context

- The authority is `crates/jj-workspace/` as frozen — `src/lib.rs` (343),
  `src/refusal.rs` (230), `src/jj.rs` (81) and `Cargo.toml` (44). `tests/` is
  evidence, not a root, and is legitimate reading for checking a claim.
- `CONTEXT-MAP.md`'s argument that `jj-workspace` is deliberately not a bounded
  context, and `docs/adr/jj-is-the-only-lane.md`, are the two places the book is
  most likely to have got the crate's boundary subtly wrong.
- The preregistration `pilot-preregistration-k24` committed is binding and not
  reopenable here. Read its attribution rule before you start and satisfy it as
  you go; reconstructing afterwards what a stage changed is what the one-commit
  boundary exists to make unnecessary.
- The baseline this stage is measured against is the previous stage's commit, and
  the record of what this stage did is the diff between that commit and yours. So
  this stage lands in **exactly one commit**, carrying its edits and nothing else.
- The corpus is frozen: do not edit `crates/jj-workspace/`. A defect found here
  becomes its own leaf under the root brief's cross-book rule.

## Done when

- Every claim the book makes about behaviour is either checked against the source
  or against a test, and the ones that could not be checked are named.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**A wrong claim about the code is a book defect, not a code defect.** If the
source is actually wrong, the freeze holds and it becomes its own leaf under the
root brief's cross-book rule — this session does not fix it.