# fragment-range-unchecked-when-final-k206

## Goal

`book-check` never compares a fragment's fence bytes against that fragment's
**own** declared `lines="A-B"` once the book is final. Decide whether it should,
and if so make it, then find out what the newly-live check reports across all
six books.

Every per-fragment range in `docs/walkthroughs/` — 985 fragment directives — is
today an annotation no check reads on the path every finished book takes. The
campaign leans on those ranges: they are what a reader uses to locate a fence in
its source root, and what `source-index.md` republishes as the book's ledger.

## Context

Found and demonstrated at `read-path-fragment-boundary-k192`, which repaired one
instance of the defect the gap allows (two adjacent fragments over
`crates/ordinal-fs-tree/src/fs/read.rs` each declaring a range one line off from
the bytes they carried) and cut this leaf rather than widening into the
validator. That leaf's running log carries the full derivation; the short form:

- `check_bytes` (`crates/book-validation/src/validator.rs:743`) branches on
  whether the root has any `Child::Defer`. **With** a defer it iterates
  `reachable_literals` and compares each literal against
  `source_range(bytes, fragment.range)` — the fragment's own range
  (`validator.rs:766-782`). **Without** one — the state every finished book is
  in, and what `--final` demands — it concatenates all children via
  `expand_children` and compares that single stream against the **root's** range
  (`validator.rs:786-805`). Fragment ranges are not read on this path.
- The only other consumer of a fragment's range is `check_child_partition`
  (`validator.rs:554`), which checks that children tile their parent
  contiguously and in order. That is arithmetic over the *declarations* and is
  never compared against fence bytes.
- So any redistribution of source lines across an internal fragment boundary is
  invisible, provided the declarations still tile and the concatenation still
  reconstructs the root.

**The check therefore weakens exactly when a book is finished** — strongest
while drafting, silent at `--final`. Whether that inversion is deliberate is
part of the question; nothing found so far records a reason for it.

Demonstrated empirically on `docs/walkthroughs/ordinal-fs-tree`, with the
instrument seen dirty before the clean read was credited:

| run | mutation | result |
| --- | --- | --- |
| baseline | none | `valid: 17 files, 8720 resolved lines`, exit 0 |
| positive control | ` X` appended to one line inside a fence | `F008 … expected 0x0a, actual 0x20`, exit 1 |
| gap test | source line 81 (the non-blank `}`) **and** line 82 moved from the tail of the first fence to the head of the second, declarations left at `1-81` / `82-155` | `valid: 17 files, 8720 resolved lines`, exit 0 |

The gap test is the one that matters: the misplaced line is **not** blank, both
declarations are then wrong, and the book still passes. The blankness of source
line 82 explained why k192's instance was hard to see by eye; it is not why it
passed.

## Done when

- The question is answered rather than only reported: either `check_bytes`
  compares each literal fragment against its own declared range on the no-defer
  path too, or there is a stated reason in the code why it must not — one that
  survives the observation that the defer path already does exactly this.
- If the check is added, it lands with a test that **fails against today's
  validator**: a fixture whose fragments tile and reconstruct correctly while at
  least one fragment's fence bytes do not equal its own declared range. Without
  that fixture the new check is unfalsified, and a check that has never been
  seen to fail is not a check.
- The newly-live check is run over **all six** books under
  `docs/walkthroughs/` — `overview`, `grove-llm`, `grove-loop`,
  `jj-workspace`, `keyed-launch`, `ordinal-fs-tree` — and every fragment it
  reports is enumerated. Expect the count to be nonzero: k192 found one such
  boundary from a single reviewer's sweep, and nothing has ever checked the
  other 983.
- Each reported fragment is repaired by moving the **declaration** onto the
  bytes the fence actually carries, not by moving reproduced source bytes to fit
  a wrong declaration — and its row in that book's `source-index.md` moves with
  it. k192's log states the reasoning; do not re-derive it.
- `bash scripts/check.sh` passes.

## Notes

**This is likely bigger than one session, and the seam is obvious.** The
validator change plus its fixture is one increment; the cross-book audit and its
repairs are another, and could be one leaf per affected book. Decompose rather
than running long — the audit cannot even be sized until the check exists and
has been run once.

**The frozen-corpus rule does not bind.** Nothing under `crates/` that any book
documents changes: `book-validation` is a workspace member deliberately outside
the documented corpus (root brief, *P1*), so editing it moves no ledger, no line
count and no other book's transcript. Any repairs are page-and-index edits.

**A repair may not be a re-tiling.** If a reported fragment's fence bytes cannot
be named by *any* contiguous source range, that is a different and worse defect
than an off-by-one, and it belongs in its own leaf rather than being silently
absorbed.
