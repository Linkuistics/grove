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

**1 · The mechanism is a marked paragraph plus a required digit form, not a
manifest assertion.** A roll-up is located by one or more `<!-- rollup «…» -->`
directive lines immediately above the paragraph that states it; the paragraph is
the checked region, and each declared quantity's derived value — rendered in the
ledger's own digit form, `grouped()` — must occur in it as a standalone number
token. A manifest assertion was rejected because it can only be page-scoped: on a
594-line assembly chapter *the number occurs somewhere on the page* is satisfied
by coincidence, and the two observed defects (`Fifty rows` for 52, `the other
thirty-seven` for 39) sit in a paragraph with a dozen other numbers. Marking in
the page also puts the declaration where the sentence is, which is the only place
a later editor rewriting that sentence will see it.

**2 · A marked figure is written in digits.** The check compares digits, so a
declared quantity has to be spelled in digits in the paragraph that states it.
The alternative was an English-number renderer — *Fifty-two*, *thirty-nine*,
*twenty-one* — and that is a translation layer that can be wrong in both
directions, for a page where a wrong translation reads as a real finding. The
*Owned source* paragraph of every book already writes its roll-up in digits, so
this is the established voice rather than a new register.

The converse is **not** enforced and must not be read into the rule: a digit in a
marked paragraph is not necessarily checked. `jj-workspace`'s owned-source
paragraph states *the 698 the root brief froze*, which is a frozen historical
figure no ledger derives, and it sits inside a marked paragraph unchecked and
correctly so. What the mark says is that the declared quantities are held, not
that everything numeric beside them is.

**3 · Each quantity derives from the authoritative side of the ledger it names.**
Ownership blocks, source roots and owned-line totals derive from
`walkthrough.toml`, because `F009` already proves those three tables equal to it
byte for byte. Early-use quantities derive from the **ledger table**, because the
manifest's `[[early-use]]` rows are a floor rather than the set
(`docs/specs/walkthrough-books.md`, *Early-use ledger*) and a count taken from the
manifest would be wrong by construction in four of the six books.

**4 · Marking is mandatory where the condition was observed.** Inside a section
anchored `the-closed-ledgers`, a paragraph whose bolded lead-in is `**Ownership.**`,
`**Early use.**` or `**Owned source.**` must carry at least one `rollup`
directive. Without that, the directive is opt-in and deleting it is silent — a
reminder rather than a check. Discovery is by book directory, as `book-check`
already works, so a sixth book joins by existing.

**5 · What this does not cover, deliberately.** Only ledger-derived figures: a
tree tally, a cross-chapter attribution, a per-part line sum, a *what could not
move* argument and a percentage of the corpus are claims no ledger can settle, and
a mechanism that pretended otherwise would be worse than none
(`landed-leaf-tally-chapter-21-k200`'s subject is the tree, and stays out). Three
narrower limits, all stated in the specification: a marked paragraph is checked
for **occurrence**, so a stale figure left beside a corrected one still passes;
two quantities with equal values are satisfied by one occurrence; and a roll-up
stated outside the three mandatory lead-ins is checked only if it is marked.

**6 · Seen red from both sides, then green.** Three observations, each restored
afterwards and the book re-proved. A figure mutated in a marked paragraph —
`5 top-level blocks` to `6` in the overview — reports `F011` naming the derived
5. Deleting a mark in a closed-ledgers section reports the unmarked account.
And the one that matters: a well-formed early-use row inserted into
`grove-loop`'s ledger in canonical order — accepted in silence by `F009` and by
every other check, which is precisely what `canonicalisation-sites-k149` did —
turns chapter 21 red on `early-use-rows` (53) and `early-use-rows-at` (14). That
third is the condition this leaf exists for, reproduced and now caught.

**7 · One roll-up surface is left uncovered and handed on.** Every book's
`concept-index.md` states ledger roll-ups in its list items — *Twenty-five
ownership blocks and fourteen early-use rows, closed*, *Twenty blocks over nine
roots* — and the paragraph-scoped directive cannot mark a list item without
splitting the list in rendered Markdown. All of them are currently true; the
gap is structural rather than a standing defect, so it goes to its own leaf
rather than widening this one. Everything they state is also stated in a marked
paragraph on the assembly chapter, so the gap is a second copy going stale, not
an unheld claim.
