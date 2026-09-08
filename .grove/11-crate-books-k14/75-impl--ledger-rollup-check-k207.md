# ledger-rollup-check-k207

## Goal

Decide and build the mechanical check that reports when a book's assembly
chapter states a ledger roll-up the ledger itself contradicts — the condition
that let `grove-loop`'s chapter 21 say *Fifty rows* over the whole span from
`canonicalisation-sites-k149` to `stale-book-counts-k196` while `bash
scripts/check.sh` stayed green across all six books.

## Context

- **The condition, not the instance.** An assembly chapter owns no source roots,
  so `book-check` expands no fragment on the page and **every sentence on it is
  an unchecked claim about other pages**. Its roll-ups are derived from the two
  ledgers in `source-index.md`, and nothing re-derives them.
- **Two instances on one page, from two different sessions.**
  `stale-book-counts-k196` found *Fifty rows … the other thirty-seven* in *The
  closed ledgers*, where enumerating the table gives 52 and 39; it went stale at
  `canonicalisation-sites-k149`, which added the `target` and `existing_path`
  rows and had no reason to read chapter 21. `landed-leaf-tally-chapter-21-k200`
  is the second, against *four of that second group have since landed* in the
  same chapter. Two independent instances is what makes this a condition rather
  than a slip — but note that k200's subject is the **tree**, not a ledger, and
  its own preferred remedy is to delete the tally rather than check it. Only the
  ledger-derived roll-ups are in scope here.
- **What is already derivable.** `book-check` parses both ledgers
  (`crates/book-validation/src/ledger.rs`), so the true row count, the *at The
  cast* split, the `explained` tally, the top-level block count and the owned-line
  sum are all available to it without new parsing. The gap is the prose side.
- **The design question is the locator.** The roll-up is spelled in **English
  number words inside running prose** — *Fifty-two rows, every one `explained`.
  Thirteen of them …* — so a checker has to be told where to look. One thing is
  already in hand: `walkthrough.toml` marks the page, `slice = "assembly"`
  (`docs/walkthroughs/grove-loop/walkthrough.toml:182`), and the *Owned source*
  paragraph on that same page already writes its roll-up in **digits**
  (`445 + 451 + … = 10,557`), so a digits-in-prose form is inside the book's
  established voice rather than a new register. Whether the answer is a declared
  assertion in the manifest, a marked span in the page, a required digit form, or
  something else is this leaf's to settle.
- **It is cross-book by construction.** Every book has a closing chapter, and a
  check that only knows `grove-loop`'s is a reminder rather than a check —
  `book-assurance-surface-k39`'s discovery rule is the standard the other four
  book checks are held to.

## Done when

- The mechanism is chosen and the reason recorded, including what it does **not**
  cover: a roll-up derived from anything other than the ledgers is out of scope,
  and saying so is part of the deliverable.
- The check exists and is reached by `bash scripts/check.sh`, discovering book
  roots rather than listing them, so a sixth book joins it by existing.
- **It has been watched to fail.** Mutate a ledger row or a stated number and see
  the check go red, then restore; a check never seen red is not evidence.
- Every existing book passes it, and any roll-up it turns red is either corrected
  here or handed to its own leaf, stated either way.
- `book-check --final --check all` is green over every book, and `bash
  scripts/check.sh` is no worse than before.

## Notes

**Prose-and-tooling only; no source root moves**, so the frozen-corpus rule does
not bite — unless the chosen mechanism requires a page to be reworded, which is
still prose and still free.

**Do not widen this into checking assembly chapters generally.** The tree tallies,
the cross-chapter attributions and the *what could not move* argument itself are
claims no ledger can settle, and a mechanism that pretends otherwise is worse than
none. Ledger-derived roll-ups are the tractable subset; that is why this leaf is
scoped to them.

## Decisions (running log)
