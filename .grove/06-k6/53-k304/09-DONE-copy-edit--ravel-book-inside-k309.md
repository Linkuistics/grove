# ravel-book-inside-k309

## Goal

Copy-edit chapters 7 to 11 of the Ravel book against its prose contract,
without a change of structure.

## Context

- Ravel `docs/book/contract.md`, the prose contract and the Read 1 patterns.
- `CONTEXT.md` for each term. The drafts ravel-book-inside-k307 and -k308.

## Done when

- Sentences, terms and cross-references in chapters 7 to 11 agree with the
  contract and with chapters 1 to 6.
- `task docs:ste`, `task docs:examples` and `task docs:excerpts` pass.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.
- The draft ravel-book-inside-k308 rewrote chapters 10 and 11 and corrected
  one paragraph in each of chapters 1 and 3. Chapter 11 uses “deliver” and
  “put in the tap” where the release domain says “publish”, because
  `docs/book/ste.md` refuses every form of “publish”. Keep one verb for each
  action across the new sections “Choose a formula channel”, “Promote a
  versioned release” and the recovery table.

## Decisions (running log)

- A partial copy edit in the Ravel workspace clarifies the example's default
  skill directory, the worker's environment and descriptors, the example
  check's duration, and promotion's effects. The full chapter read and
  terminology comparison remain unfinished. The Ravel working copy carries
  these edits; do not retire this leaf until that read is complete.
- The copy edit makes chapter 8's checkout-build trace explicit and corrects
  its count of short Ravel processes. Chapter 7 now names skill preparation
  before the worker, and chapter 11 describes promotion as availability to new
  or upgraded installations. The existing art and proof leaves already carry
  the figure and contract follow-up.
- Ravel change `lsvquslm` seals the copy edit. On the final text,
  `task docs:ste` reported zero problems, `task docs:examples` passed 27
  examples, and `task docs:excerpts` passed 22 excerpts. The art and proof
  siblings already stand, so this stage adds no successor.
