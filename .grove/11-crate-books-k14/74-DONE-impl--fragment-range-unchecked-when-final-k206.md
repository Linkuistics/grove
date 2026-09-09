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

## Decisions (running log)

**The check is added, not justified away.** Nothing in `book-validation` records
a reason for the inversion, and the defer path already compares every reachable
literal against its own declared range — so a stated reason "why it must not"
would have to explain why the same comparison is correct for a partial root and
wrong for a complete one. There is no such asymmetry: the whole-stream
comparison a complete root additionally gets is strictly about the *root's*
range, and `check_child_partition` is arithmetic over declarations only. The
per-fragment comparison is therefore hoisted out of the `has_defer` branch and
runs on both paths, with the whole-stream comparison kept for complete roots and
suppressed only when the per-fragment pass already reported (one defect, one
diagnostic). `crates/book-validation/src/validator.rs`, `check_bytes`.

**The fixture came first, and its first version was red for the wrong reason.**
`a_fragment_whose_bytes_are_not_its_own_declared_range_is_a_byte_failure`
(`crates/book-validation/tests/fragment_failures.rs`). The first attempt declared
a `source-library` root of `1-4` over a four-line source; it was red before the
change and **still red after** it. Dumping the whole diagnostic list named the
cause: `F006 … source root 'source-library' is duplicated or disagrees with the
declared inventory`, because `support::manifest()` fixes that root at 103 lines.
An inventory-invalid root skips its byte comparison by design, so the fixture was
never reaching the code under test. The neighbouring tests hide this — they
assert the *absence* of codes, which an invalid root satisfies for free.

The fixture is therefore built on the real root: `source-library` at `1-103`,
its single manifest-required top-level block `library-crate-surface` as a
composite, and two literals under it declaring `1-2` and `3-103` while the first
fence carries only line 1. Declarations tile, expansion reconstructs, `F007` is
asserted absent, and `F008` names `part-1`. **Red against the pre-change
validator and green against the new one, both observed** — the before-run alone
would only have proved it was red.

**The spec already required this; the implementation did not.** `docs/specs/
walkthrough-books.md`'s invariant list states *a literal fragment's fence bytes
equal the named source range exactly*, without qualification by scope. What made
the silence look deliberate was the *Final fragment checks* paragraph, which
enumerates final mode's obligations and named only the expanded-root comparison.
That paragraph is edited in place to name both comparisons and to say why
neither implies the other — so the two passages now agree, and the invariant is
enforced where it was already promised. No ADR: nothing was decided here that
the spec did not already say.

**The audit reports zero, and the expectation of a nonzero count was an
extrapolation rather than a measurement.** *Done when* expected repairs, on the
ground that k192 found one boundary from a single reviewer's sweep and nothing
had checked the other 983. Two independent instruments disagree with that
expectation:

- An out-of-band scanner (Python, parsing the fragment directives straight out
  of the pages and comparing each fence against `source[a-1:b]`) scanned **985**
  directives — matching the task file's own figure — and split them 878 literal
  / 107 composite, which is exactly the `literal` / `composite` row split of the
  six `source-index.md` ledgers. It reports **0** fences unequal to their own
  declared range.
- `book-check --final --check all` with the new check live is green on all six
  books: `overview` 3 files / 204 lines, `grove-llm` 4 / 1,017, `jj-workspace`
  4 / 752, `keyed-launch` 9 / 2,073, `grove-loop` 13 / 10,557, `ordinal-fs-tree`
  17 / 8,720.

Both instruments were seen dirty before either clean read was credited. The
scanner: a declaration moved by one (`1-82` → `1-81`) and, separately, a single
byte changed inside a fence — each reported exactly one mismatch, naming the
right fragment. `book-check`: the task file's own gap test replayed — source
lines 81–82 moved from the tail of `read-tree-discovery` to the head of
`read-directory-listing`, declarations left tiling at `1-82` / `83-155` — which
was `exit 0, valid` in the table above and is now
`F008 … expected 0x7d, actual EOF; emitting fragment read-tree-discovery`,
exit 1. The working tree was restored byte-identically after every mutation
(`jj diff` empty).

**So there are no repairs, and this leaf does not decompose.** The *Notes*
foresaw a validator increment and then a cross-book audit that "cannot even be
sized until the check exists and has been run once". It has now been run once,
and it sizes to nothing: no fragment is reported, so there is no repair leaf to
cut and no `source-index.md` row to move. The seam was real; the second side of
it turned out to be empty.

**A hazard worth naming for the next session that measures here.** Under this
harness a freshly linked binary can stall at exec — 0% CPU, 32-byte RSS, no
output at all, indistinguishable from a slow validator — and it stalls *any*
later invocation until the accumulated stalled processes are reaped. `book-check
--help` timing out is the tell: a binary that cannot print its usage is not
running your logic. Reap with `pkill -9`, then re-run.

**One entry in `CHANGELOG.md`, under `## Unreleased`.** The change is behaviour
a reader runs — `book-check` is stricter than it was — so it is logged where the
file's own rule puts a non-binary component's change, prefixed with the component
it touched. The entry carries the audit's figures, because "the check now exists"
and "the corpus was clean when it was switched on" are two different facts and
the second is the one a future session will want.
