# keyed-launch-k120

## Goal

Copy-edit the `keyed-launch` book — `docs/walkthroughs/keyed-launch/`, ten
chapters and two lookup surfaces over a 2,073-line corpus — to a green
`book-check --final --check all` and a green `bash scripts/check.sh`.

## Context

- The draft is complete and validates: 9 roots, 2,073 resolved lines, 0 deferred,
  `final=true`, all 8 principal checks passing at `what-passes-through-k117`.
  Every ownership block is `resolved` and every early-use row `explained`, so
  nothing structural is outstanding.
- The prose contract is `docs/specs/walkthrough-books.md` plus this book's
  structure brief, `docs/specs/keyed-launch-book-structure.md` — in particular
  *What each chapter's prose owes*, whose three obligations a copy-edit must not
  flatten. The precedent to be uniform with is the other four books under
  `docs/walkthroughs/`, `ordinal-fs-tree` being the worked example.
- The glossary is `CONTEXT.md`; the book reserves `loop-control-channel` from it
  and two `docs/USAGE.md` anchors. All three exist, and `M201` is green.
- **Two defects are already named, in `keyed-launch-book-k35`'s `## Handed
  forward`** — chapter 7's `&mdash;`/`&ndash;` entities against the literal
  characters the other chapters use, and two byte-identical labels in
  `concept-index.md` pointing at different targets. Clear both entries from the
  brief as you close them; a brief is current-state context, not a log.
- **Read `## Carried forward from the draft` in the same brief before editing
  chapters 7 or 8.** It carries the one thing a whole-document prose pass is most
  likely to break: the prose obligation is *directional*, and evening the chapters
  out is a defect that makes the book look more uniform.

## Done when

- Every sentence-level, terminology and consistency defect the charter owns is
  fixed across all twelve files, and the two handed-forward entries are closed
  and cleared from the node brief.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/keyed-launch --final --check all` is valid, unchanged at 9
  files and 2,073 resolved lines.
- `bash scripts/check.sh` passes, all 8 principal checks.
- **Last act:** `grove-llm leaf-add keyed-launch-book-k35 keyed-launch --kind
  art`, unless a live later sibling under `keyed-launch-book-k35` already holds
  that stage.

## Notes

**Do not restructure.** A structural finding is a `draft` re-run leaf and a
contiguous correction run through `proof`, never a fix here. The distinction is
load-bearing for this book: the ten-page shape, the chapter boundaries, the
twenty ownership blocks and the concept order are all the structure brief's, and
changing one silently invalidates the manifest and the ledger.

**The corpus is frozen.** Do not edit `crates/keyed-launch/`. Literal fragments
are exact source bytes — a copy-edit that "fixes" a sentence inside a four-backtick
fence turns `F008` red, and every reproduced comment in chapters 7 and 8 is such a
fence.

**Two counts in chapter 10 are right and look wrong.** The page says the book's
stated outcome names **seven** tests and that the page itself names **nine**. Both
were established by enumeration at `what-passes-through-k117`; the earlier grove
artifacts said six, and that was their error, not the brief's. Do not reconcile
either number.
