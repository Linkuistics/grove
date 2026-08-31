# A book cannot witness its own corpus

A walkthrough book states its own **fragment graph** and nothing else. What the
book *owes* — which files it must reconstruct, who owns which range, what its
final page sequence is — is stated in a per-book manifest,
`docs/walkthroughs/<book>/walkthrough.toml`, and the set of source roots that
manifest declares is checked against the crate directory it describes rather than
taken on the book's word.

The line is between **content** and **obligation**. Expansion reads the book's
Markdown and the source files and nothing else; the manifest contributes zero
bytes to any reconstructed file, names no fragment below the top level, and
declares no parent or child edge. A reader holding the book and the crate can
still recover every source byte with no tooling and no manifest. What the reader
cannot recover from the book is whether a file was left out, because a book that
omits a file omits it from its pages *and* from its own description of itself.

## The trade-off

The validator used to compile the whole obligation in — `ROOTS`, `BLOCKS`,
`SLICE_ORDER`, `PAGE_BY_OWNER`, `EARLY_USES` — and a repository test compared
those constants against normative tables in the book's specification. Two
independently authored statements of the same corpus, cross-checked, is what made
*complete reconstruction* a claim about an externally stated corpus rather than a
self-declaration. That does not survive six books: the constants would have to
name all six, and the specification would have to carry six sets of tables, which
is the one-book design repeated rather than generalised.

Moving the obligation into the book's own Markdown is the obvious alternative and
it costs the cross-check outright — the book would declare its corpus, prove it
against that declaration, and report success. It also cannot express the part of
the obligation that matters most: scoped mode compares present pages and resolved
blocks against a **plan**, and a plan is a statement about artifacts that do not
yet exist, so it cannot be derived from them.

The manifest keeps the obligation outside the book and replaces the lost
cross-check with a stronger one. Rather than comparing two hand-written lists —
which can be wrong together — the manifest declares the corpus **rule**: include
globs, explicit additions, and explicit exclusions each carrying a reason. The
validator enumerates the real directory and requires the declared root set to
equal the derived set. A forgotten file is a finding; an addition or exclusion is
legible and argued at the point it is taken.

The rule was checked against the frozen corpus before being adopted.
`<crate>/Cargo.toml` plus `<crate>/src/**/*.rs` reproduces each campaign
deliverable's root and line counts exactly, with one exclusion across five crates
(`crates/grove-loop/src/task_grow/tests.rs`); the relocated `ordinal-fs-tree`
book is the same rule with five exclusions and one addition outside `src/`. Every
exclusion is an inline test module or a test-support module — evidence rather
than production source, which is the classification
`linkuistics:writing-code-walkthroughs` asks for at intake.

## The alternative that was rejected, and what changed

`docs/specs/walkthrough-books.md` previously rejected *a TOML sidecar manifest*
on three grounds: it duplicates parents, children, ownership and ranges already
visible in Markdown; drift would force a choice about which representation to
trust; and raw Markdown would no longer be sufficient to reconstruct the code.

All three hold, and all three are about the **fragment graph**. None of them
reaches the obligation, and the obligation was never in Markdown to be
duplicated — it was in `validator.rs`. The rejection read as complete because one
book could compile its answer into Rust and never have to say where it lived.

So the graph clauses are kept, and are now stated positively rather than as the
by-product of a rejection: the manifest may name a source root and a top-level
ownership block, and nothing below them. Fragment parents, children, insertion
order and literal bytes stay in Markdown, expansion stays Markdown-only, and the
`source-index.md` tables remain derived indexes reconciled against both the
manifest and the directives — so drift is a finding under a stated trust order
rather than an unanswerable question about which copy is right.

The format the old text named is deliberately kept. Adopting the same
serialization under a different extension to avoid the appearance of a reversal
would misdescribe what changed: the objection was never to TOML, and the subject
is what moved.

## What would reopen this

- A manifest field that expansion reads. The moment a reconstructed byte depends
  on the manifest, the walk-away property is gone and the original rejection
  applies in full.
- A book whose corpus no rule can describe, so that its manifest degenerates into
  a hand-listed root set with no derivation to check it against. The witness, not
  the file, is what this record is for.
- The fragment validator moving to the `writing-code-walkthroughs` skill, where a
  book need not sit in a Cargo workspace at all and the include-glob rule may not
  be expressible.
