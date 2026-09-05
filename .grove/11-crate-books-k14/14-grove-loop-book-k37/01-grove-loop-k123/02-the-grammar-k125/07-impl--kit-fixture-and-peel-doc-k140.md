# kit-fixture-and-peel-doc-k140

## Goal

Two structural defects in `crates/grove-loop/src/task_name.rs`, found while
drafting chapter 4 and adjudicated on `04-the-name.md` today: a conformance-kit
fixture that no longer poses the question its comment says was measured of it,
and a doc comment attached to the wrong item. One commit carries the source
change, every affected ledger and page, and a green validator run over the book.

## Context

**Read `docs/walkthroughs/grove-loop/04-the-name.md` first.** Its
`#the-conformance-kit` and `#under-the-seam` sections state both defects with the
evidence, and this leaf does not have to re-derive either.

### 1 · The near-miss fixture is refused before canonicity is asked

`listings()`' comment at lines 1,061–1,074 claims *disabling this domain's
canonicity check leaves the kit green without these two entries and red with
them*, citing `docs/formalism-findings.md` entry 020. Measured in a copy of the
workspace, that is no longer true:

| Arm | `listings()` | Mutation | `the_task_tree_domain_conforms` |
|---|---|---|---|
| baseline | all 8 | none | green |
| A | all 8 | the seven canonicity lines removed from `parse` | **green** |
| B | first 6 | same | green |
| control | all 8 plus `5-impl--a-k1.md` | same | **red**, `TheGrammarIsCanonical` |

The kit's listings check renders back only names that reach `Verdict::Entry`.
`5-impl-domain-k29.md` carries a single `-` where the grammar has required `--`
since `grammar-separator-k15`, so it is `Malformed(MissingSeparator)` and never
reaches the comparison; entry 020 was written against the pre-separator grammar,
where the same name parsed and rendered `05-impl-domain-k29.md`. The healthy
fixtures were rewritten to the new grammar and this one was not.
`07-DONE-grove-flip-k28` is `Malformed(NodeWearsOutcome)` and is inert in the kit
for a different reason; it is not inert in the crate, since removing the
node-outcome guard turns it into `Malformed(NotCanonical)`.

**And the same comment's first paragraph names entries the fixture does not
hold.** It enumerates *the charter, a live leaf, both terminal marks, a node
directory, a foreign `README.md`, and both transaction sentinels* — eight items
against eight entries, so the count reads current, but the last two entries are
the near-misses and no entry is a sentinel. Entry 020 records the fixture it was
written for: ten listings, including `FORMAT` and *the three* transaction
sentinels. The clause has to name what the last two entries now are.

**The fix that restores the property is one character**: `5-impl-domain-k29.md`
becomes `5-impl--domain-k29.md`, which parses under a lenient domain and renders
`05-impl--domain-k29.md`. Re-run the mutation and confirm arm A goes red before
committing — the mutation is the instrument, and a fix asserted without it is the
same failure entry 020 recorded. The comment then has to say what is now true,
inside its existing lines.

### 2 · `peel_key`'s doc comment is attached to `terminal_key`

Lines 977–1,005 are one contiguous `///` run: there is no blank line between 992
and 993, so the whole of it documents `terminal_key` at line 1,007, and
`peel_key` at 1,012 has no doc comment at all. Confirmed from rustdoc rather than
from reading the bytes — built with `--document-private-items`,
`fn.terminal_key.html`'s docblock opens *Peel a terminal `-k<digits>` into what
precedes it and the digit run*, which is not `terminal_key`'s signature, and
`fn.peel_key.html`'s docblock is empty.

Lines 977–992 are `peel_key`'s and belong on it. The fix is a **reorder**, not a
reword: move `peel_key`'s definition and its paragraphs above `terminal_key`, or
move the paragraphs down to it. **`stale-enumerations-k139` runs first and
rewords the two-callers count inside those same lines** — carry its wording with
the paragraphs rather than reinstating the old text.

## Done when

- `5-impl-domain-k29.md` is a fixture that reaches the kit's canonicity check,
  demonstrated by the mutation going red with it and green without it, and
  `listings()`' comment states what the fixture now does — **both** paragraphs of
  it, the enumeration of shapes as well as the measurement.
- Lines 977–992 document `peel_key`, and `terminal_key`'s rendered docs open on
  its own summary line. `cargo doc -p grove-loop --no-deps --document-private-items`
  is the check.
- **No ownership block's line count has moved.** The book's ledger holds an exact
  line count per source root and per block, so a comment or item that gains or
  loses a line shifts every later block of a 1,714-line file and breaks pages
  three finished sessions already proved. Both fixes are possible inside the
  existing counts: one is a character, the other a reordering of two items and
  their comments.
- `04-the-name.md`'s two adjudicating passages are rewritten in the same commit.
  Once the comment and the attachment are right, a paragraph saying they are
  wrong is itself a false claim — but the enumerations they carry are worth
  keeping, so restate them as statements about the code. The measurement table
  in `#the-conformance-kit` should become the table of what the fixture now
  poses.
- `book-check --repo . --book docs/walkthroughs/grove-loop --through <the last
  proved slice> --check all` is valid with the same resolved and deferred counts
  as before, and `cargo test -p grove-loop` passes.
- One commit carries all of it.

## Notes

**Why one leaf and not two.** Both are single edits to `task_name.rs` inside
chapter 4's blocks, and the corpus-freeze rule wants the source change, the
ledgers, the pages and a green validator run in one commit. Two leaves would put
two commits through the same file, each having to re-prove the book against the
other's bytes.

**Not `stale-enumerations-k139`'s.** That leaf's method is *reword inside the
existing lines* over five miscounts. Neither of these is a miscount: one changes
what a test exercises and has to be measured, and the other moves items. Its row
3 and this leaf's second defect are about the same lines, which is why it runs
first.

**The corpus is frozen for everything else.** Do not fix
`crates/grove-llm/src/cli.rs` line 327 here; `stale-enumerations-k139`'s notes
already record it and its rule.

## Decisions (running log)
