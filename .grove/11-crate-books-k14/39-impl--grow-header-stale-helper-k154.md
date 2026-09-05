# grow-header-stale-helper-k154

## Goal

Repair the two defects in `crates/grove-loop/src/task_grow.rs`'s module header —
a shared helper named `leaf_slug` that has never existed, and a `#` heading with
no body whose two paragraphs sit under the next heading — inside the file's
frozen 518-line count, and reconcile the page that reproduces the changed bytes.

## Context

- **Defect 1, the name.** Lines 27 to 34 read *Three helpers here are
  `pub(crate)`, and `leaf-decompose` is why* and name them `new_leaf`,
  `leaf_slug` and `refuse_finish_kind`. **There is no `leaf_slug` in this
  workspace and there never was**: `grep -rn leaf_slug crates/` hits this comment
  and nothing else, and the commit that created the file
  (`loop-crate-verbs-k21`, `ae64b1188fc1`) wrote the comment with it, over a
  first version that already carried seven `pub(crate)` functions and no eighth.
  So it was wrong when written rather than outrun — the same provenance
  `stale-enumerations-k139` established for `task_name.rs`'s six.
- **The count is right; the third name is `allocated`.** Three of the file's
  seven `pub(crate)` functions are called from `tree_lifecycle` —
  `refuse_finish_kind` (346, 560), `new_leaf` (388, 573) and `allocated` (132,
  405). The other four are the verb surface `verbs.rs` calls plus
  `task_template_body`, which has **no caller outside this file at all**; its
  only outside mention is the intra-doc link at `tree_lifecycle.rs` line 1,052.
- **Defect 1b, the attribution.** `leaf_decompose` reaches only **two** of the
  three. It calls `refuse_finish_kind` at line 560 and `new_leaf` at 573, and
  then checks its own promotion with `tree_lifecycle::promoted` (line 645) rather
  than with `allocated` — a second reading of the same report, in chapter 12's
  block. `allocated`'s own doc comment names its other caller and it is
  `initialize_grove`, not `leaf_decompose`. A repair that only swaps the name
  leaves the *why* clause overclaiming.
- **Defect 2, the mis-nesting.** Line 25 is `// # What went, and stayed gone` and
  line 27 is `// # Three helpers here are `pub(crate)`, and `leaf-decompose` is
  why` — two headings in a row, the first with no body. The paragraphs that
  belong to the first (lines 36 to 49: the path-walking appender's collision
  machinery, and the deleted `leaf-add-pair`) sit **below** the second, so a
  reader working down the file meets them as reasons three helpers are
  `pub(crate)`. Reordering the two headings, or giving the first its paragraphs
  back, fixes it.
- **No instrument in this repository sees either defect.** Lines 1 to 49 are
  plain `//` comments rather than `//!` inner doc comments, so `cargo doc
  --no-deps --document-private-items -p grove-loop` renders none of them and
  reports **no warning for this file at all** — verified while drafting chapter
  10, against 30 warnings elsewhere in the crate. That is the blind spot beside
  the four things `cargo doc` does see.
- **One page reproduces the changed bytes** and must change in the same commit:
  `docs/walkthroughs/grove-loop/10-growing.md`, fragments `«grow-header-the-four»`
  (lines 1-24) and `«grow-header-what-went»` (lines 25-49). Its
  `#a-list-of-exactly-it` section adjudicates both defects at length and must be
  rewritten to describe the repaired comment rather than the broken one; two
  `concept-index.md` entries name them and follow.
- **Found by `growing-k146`**, by enumerating the file's own `pub(crate)`
  functions against the comment's list — which is a check the page can perform
  in front of the reader, because the chapter reproduces all seven.

## Done when

- `grep -rn 'leaf_slug' .` returns nothing, and the header's helper sentence
  names the three functions `tree_lifecycle` actually calls, with the
  `leaf_decompose` attribution narrowed to the two it reaches.
- `# What went, and stayed gone` has its own paragraphs under it, or is removed
  in favour of the heading that does.
- **`crates/grove-loop/src/task_grow.rs` is still exactly 518 lines**, so no
  ownership range, manifest `lines` value or fragment range moves.
- `10-growing.md` reproduces the new bytes, its adjudication is rewritten to the
  repaired text, and the two `concept-index.md` entries are updated or dropped.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  over the book at whatever slice it is proved at when this runs, and every other
  book the commit touched is green too.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** The bytes it
changes are reproduced by a finished page, and the freeze rule requires one
commit to carry the source change, every affected ledger and page, and a green
validator run. That is satisfiable today for this file — chapter 10 is written —
but the same commit should not race the later parts still drafting against the
same crate. Run it after `grove-loop-k123`'s last child has taken the book to
green `--final` validation, alongside `unresolved-doc-links-k151` and
`unreachable-root-clause-k152`, which are deferred for the same reason. Until
then chapter 10's adjudication stands as the record.

**Do not widen this into a sweep of the crate's headers.** `stale-enumerations-k139`
swept `task_name.rs`; the remaining modules are swept by the chapters that read
them, and each cuts its own leaf.

## Decisions (running log)
