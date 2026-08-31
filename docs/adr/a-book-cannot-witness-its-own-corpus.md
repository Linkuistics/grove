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
cross-check with **derivation plus a much smaller second statement**. Rather than
comparing two hand-written lists — which can be wrong together — the manifest
declares the corpus **rule**: include patterns, explicit additions, and explicit
exclusions. The validator enumerates the real directory and requires the declared
root set to equal the derived set, so a production file added to the crate and
forgotten by the book is a finding rather than a silence.

Derivation on its own is not an external witness, and calling it one was the
mistake worth recording here. It proves that the declared patterns matched; it
cannot prove the author declared the right patterns, and it cannot judge an
exception, since a `reason` is prose. A manifest free to choose its own patterns
could include one file, declare that one root, and pass. So two constraints carry
the externality that derivation cannot:

- **the rule's floor is not the book's to choose** — `include` must contain the
  base patterns derived from `[book].subject`, `<subject>/Cargo.toml` and
  `<subject>/src/**/*.rs`; and
- **every exception is declared twice** — each addition and exclusion carries a
  `class` from a closed list, and the complete set of them across all books is
  restated in `docs/specs/walkthrough-books.md` and compared against every
  manifest by a repository test.

What scales with the corpus — every root, every block — is derived and per-book;
only the handful of exceptions, which are exactly what derivation cannot check,
are written down twice. That is not the one-book design repeated: six rows in one
table is not six sets of root and ownership tables.

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

The three grounds do not all survive intact, and the honest account matters more
here than a preserved verdict.

**Two of them are about the fragment graph and are kept.** Parents, children,
insertion order and ranges below a top-level block stay in Markdown and nowhere
else, expansion stays Markdown-only, and raw Markdown alone still reconstructs
the code. The manifest may name a source root and a top-level ownership block,
and nothing below them.

**One of them is reversed, deliberately, and under a new trust order.** "Ownership
and ranges already visible in Markdown" reached the top-level ownership blocks:
they were visible then and are visible now, in `source-index.md`'s Ownership
blocks table, and the manifest carries the same rows. That is duplication, and
the old rejection was right that duplication forces a choice about which copy to
trust when they drift. What changed is that the choice is now made in advance
and checked, rather than left open: the manifest is the contract, the directives
are the execution, and the tables are the derived index reconciled against both,
with any disagreement reported as `F009`. A duplicate under a stated trust order
and a bidirectional check is a different object from a duplicate under neither.

The part of the obligation that was genuinely nowhere in Markdown is the rest of
it — the corpus boundary, the page inventory and the slice order — which was
compiled into `validator.rs`, where one book could leave the question of where it
belonged unasked. The rejection read as complete because of that, not because
every clause of it missed.

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
- An exception class the two named here cannot express, or an exception inventory
  long enough that maintaining it costs more than the self-declaration it
  prevents. Both mean the corpus rule has stopped fitting the repository.
- The fragment validator moving to the `writing-code-walkthroughs` skill, where a
  book need not sit in a Cargo workspace at all and the include-glob rule may not
  be expressible.
