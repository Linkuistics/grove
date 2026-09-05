# stale-enumerations-k139

## Goal

Correct six stale enumerations in `crates/grove-loop/src/task_name.rs`'s
comments, and rewrite the paragraphs on chapters 3 and 4 that adjudicate them, in
one commit that leaves `book-check` green over the book's proved prefix.

## Context

Four of the six were found by `kind-slug-handle-k135` while drafting chapter 3
and are adjudicated on `docs/walkthroughs/grove-loop/03-kind-slug-handle.md`;
rows 3 and 6 are chapter 4's and are adjudicated on `04-the-name.md`, row 6
having been added by `the-name-k136` when it drafted that page. **Read both pages
first** — each names the defect and the enumeration behind it, so this leaf does
not have to re-derive them.

| # | Line | What it says | What the file holds |
|---:|---:|---|---|
| 1 | 284 | `Kind::is_finish` licenses *the three places that ask* | seven calls in six functions; `tree_lifecycle.rs` 621, 757 and 921 refuse to decompose, retire or prune an existing `finish` leaf and are none of the three named categories. `task_tree.rs` 615 is a malformed-tree refusal rather than the sorting at 629, and the site at `tree_lifecycle.rs` 230 is `finish_commit` rather than anything in `loop_driver.rs`, which never calls `is_finish` |
| 2 | 450 | `Handle::parse` is *the only peel of the terminal `-k<digits>` outside `split_shape`* | three callers of `peel_key`: `split_shape` at 973, `Handle::parse` at 478, `terminal_key` at 1,008. The clause the sentence ends on stays true and gets a better premise — all three ask `peel_key`, so none of them finds the key itself |
| 3 | 989 | `peel_key`'s *the two callers disagree about what an over-wide key means* | three callers, as above. Line 980's *shared by `split_shape` and `Handle::parse`* is incomplete in the same way, and line 1,004 already states the correct relation twenty-five lines later. These paragraphs are *about* `peel_key` and are **attached to** `terminal_key` — the `///` run from 977 to 1,005 is unbroken — which is `kit-fixture-and-peel-doc-k140`'s to move, after this leaf. Reword the count in place; do not move anything |
| 4 | 1,536 | a slug carrying `--` would leave `UnknownKind` *quoting a token nobody wrote* | `UnknownKind` occurs exactly once in the repository — in this comment. It was a `TaskNameError` variant under the closed kind set; `open-kind-k20` replaced it, and the variant that carries a refused kind today is `BadKind`, whose own doc at 689 records the change. The argument survives the renaming |
| 5 | 1,691 | `handle_key` answered *key 3* for *three references* no entry could wear | the loop at 1,707 has four: `-k3`, `A-k3`, `DONE-k3`, `a_b-k3`, each ending in a terminal `-k3` |
| 6 | 1,120 | the kind-shape fixture sweeps *one word, two words, four words, a pair where one token is a proper prefix of another, digits, and a single character* | eight tokens whose longest is **three** words (`integrate-review-impl`, `integrate-review-prototype`), and no pair in which one is a proper prefix of another — those two share the two-word prefix `integrate-review` and neither is a prefix of the other. Added by `the-name-k136`, which adjudicates it at `04-the-name.md#the-conformance-kit`. *Four words* is a plain miscount. The prefix clause names a hazard **this grammar cannot have** — a longest match against a closed label set is what a prefix relation between two kinds would confuse, and `open-kind-k20` took the set away — so the wording has to change rather than the fixture gaining a sample |

Rows 1, 2, 4 and 5 are chapter 3's bytes; rows 3 and 6 are chapter 4's.

- **Keep the line count of every reproduced block identical.** The book's ledger
  holds an exact line count per source root and per ownership block, and a
  comment that gains or loses a line shifts every later block of a 1,714-line
  file and silently breaks pages three finished sessions already proved. Reword
  inside the existing lines.
- **`task_name.rs` is in one book only**, so the cross-book rule costs one
  validator run: `book-check --repo . --book docs/walkthroughs/grove-loop
  --through <the last proved slice> --check all`, plus `bash scripts/check.sh`,
  which stays red on `book-check` alone until chapter 21.
- The corpus freeze's whole-set rule applies (root brief, *Notes*): the source
  change, every affected ledger and page, and a green validator run travel in
  **one** commit, or the leaf is deferred behind the books it would invalidate
  and says so here.
- Each page's adjudicating paragraph has to go, not be softened: once the comment
  is right, a paragraph saying it is wrong is itself a false claim. The
  enumerations those paragraphs carry are worth keeping — the seven `is_finish`
  sites and the three peels are useful to a reader either way — so rewrite them
  as statements about the code rather than as adjudications of the comment.

## Done when

- All six comments state what the file holds, and no reproduced block's line
  count has changed.
- Chapters 3 and 4 no longer adjudicate a comment that is now correct, and each
  still names the enumeration a reader would otherwise have to perform.
- `book-check` over the proved prefix is valid with the same resolved and
  deferred line counts as before this leaf, and `cargo test -p grove-loop`
  passes.
- One commit carries all of it.

## Notes

**Not in this leaf's scope, and recorded so it is not lost.**
`crates/grove-llm/src/cli.rs` line 327 spells `default_value = "plan"`, a second,
independent spelling of the default root slug that `crates/grove-loop/src/tree_lifecycle.rs`
line 56 holds as `DEFAULT_ROOT_SLUG`. Nothing ties the two together, so they can
drift, and that sits badly beside `Kind::requirements`'s own justification at
`task_name.rs` 267–269 — *a constructor rather than a constant so the literal has
one home*. It is a code defect rather than a comment one, it is in another
crate's frozen corpus, and the `grove-llm` book is already written and proved
against those bytes; fixing it here would invalidate that book without carrying
its pages. It earns its own leaf if anyone wants it, placed under the cross-book
rule.

**The two claims the structure brief already knew about are not these.**
`src/lib.rs` line 68 and `src/session_config.rs` line 89 belong to
`every-member-version-comment-k84` and `template-source-read-count-k86`, which sit
after this book. This leaf does not touch either.

## Decisions (running log)

1. **Row 1's replacement names four categories where the old comment named
   three, and folds the two selection calls into one word.** Four lines cannot
   carry seven site-by-site descriptions, and a comment that tried would be the
   next stale enumeration. *Selection, creation, `finish_commit`, and the three
   refusals to decompose, retire or prune an existing one* covers all seven —
   two in `selected`, one in `refuse_finish_kind`, one in `finish_commit`, and
   `decomposable`, `retire_parts` and `plan_leaf` — with the count stated up
   front and the site-by-site reading left to chapter 3, which is where a reader
   who wants the addresses already goes.
2. **Row 6 drops the *proper prefix* descriptor rather than adding a fixture to
   satisfy it.** The task file called this out and the source bears it out: a
   proper-prefix relation between two kinds is what a longest match against a
   closed label set would have had to disambiguate, and `open-kind-k20` took the
   set away, so the first-`--` split cannot consult it. The shape the two
   `integrate-review-*` tokens actually pose is a **shared** two-word prefix,
   which the split can still get wrong by truncating, and that is what the
   comment now names. Chapter 4 keeps the distinction as prose about the code.
3. **Chapter 4 keeps its misattachment adjudication.** Only the caller count was
   this leaf's; the `///` run from 977 to 1,005 is still unbroken and still
   attached to `terminal_key`, so the paragraph that says so is still true.
   `kit-fixture-and-peel-doc-k140` owns the move.
4. **One adjacent false path in chapter 3 was fixed while verifying row 2.** The
   page cited `terminal_key` as *a public function at
   `crates/grove-loop/src/task_tree.rs` line 1,007*; it is `task_name.rs` line
   1,007, and `task_tree.rs` only calls it. It sits in the sentence immediately
   above the paragraph this leaf rewrote and concerns the same three callers, so
   cutting a leaf for a one-token path correction would cost more than it saves.
5. **Two `concept-index.md` entries restated the corrected enumerations and were
   reworded** — *the three peels of a terminal `-k<digits>`* (there is one peel
   and three callers) and *the two the comment names that the fixture does not
   carry* (there are none now). An index that contradicts the page it points at
   is the same defect this leaf exists to remove.

## Verification

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  canonical-or-nothing --check all` → **valid: 13 files, 2150 resolved lines,
  8383 deferred lines, final=false** — the same three numbers as before this
  leaf, which is what proves no reproduced block's line count moved.
- `cargo test -p grove-loop` → 245 + 23 + 3 + 16 + 20 + 22 passed, 0 failed.
- `bash scripts/check.sh` → **FAILED — 1 of 8**, red on `book-check` alone and
  on nothing else. `cargo fmt`, `shellcheck`, `cargo clippy`, `plugin install`,
  `conformance`, the conformance suite and the whole-workspace `cargo test` are
  green, and the other five books validate `final=true`. The `grove-loop`
  book's failures are all the absence of chapters 5–21: `M101` for each missing
  page, `M103` for the navigation lines that point at page 5, and the `F003` /
  `F009` rows the ledger still holds as deferred or pending. That is the shape
  this node's brief predicts and not a lapse.

## Found while fixing

**`UnknownKind` still occurs in `CHANGELOG.md` line 374, and correctly.** That
line records the rename itself (`TaskNameError::UnknownKind` → `BadKind`), so it
is history rather than a stale claim. Chapter 3's old paragraph asserted the
token occurred *exactly once in the whole repository*; it did not, and the
replacement makes no uniqueness claim at all.
