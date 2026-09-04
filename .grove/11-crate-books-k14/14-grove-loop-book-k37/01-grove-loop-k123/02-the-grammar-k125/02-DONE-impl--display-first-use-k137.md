# display-first-use-k137

## Goal

Adjudicate the disagreement between the early-use ledger's `Display` row and the
page that actually uses `Display` first, and make the manifest, the structure
brief, the ledger and the affected pages agree — whichever way it is settled.

## Context

- **The disagreement.** `docs/walkthroughs/grove-loop/walkthrough.toml`'s
  mandatory `[[early-use]]` row for `` `TaskName::compose`, `impl Display for
  TaskName` `` declares its first use at
  `03-kind-slug-handle.md#the-handle-is-the-identity`, and
  `docs/specs/grove-loop-book-structure.md`'s *Early uses the order forces* gives
  the reasoning: `every_positioned_name_ends_in_its_own_handle` is the handle's
  structural claim and can only be asserted over a rendered whole name.
- **What contradicts it.** Chapter 2's block `shape-refusal-tests` reproduces
  `crates/grove-loop/src/task_name.rs` lines 1,418 and 1,455, where
  `a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading` calls
  `to_string()` on a `TaskName` twice and asserts each name renders back to its
  own bytes. In page order the first use is chapter 2.
- **Nothing is red.** `check_early_uses` in
  `crates/book-validation/src/ledger.rs` matches a mandatory row byte-for-byte
  and checks only that the first-use chapter is strictly before the owner's; it
  never checks that a named first use is the earliest one. Chapter 2 is green
  with the row untouched.
- **`the-tokens-k134` left the row alone** and stated the rendering behaviour in
  prose at `02-the-tokens.md#refusals-inside-the-shape`, so the page is
  self-contained either way and this leaf is free to settle it in either
  direction.
- Precedent for the shape of the fix, if the brief is what moves:
  `structure-brief-dependency-count-k132`, which corrected the same brief and
  said what the gap was.

## Done when

- The question is answered in one of two directions, with the reason recorded:
  either the row's first use moves to chapter 2 — in which case the manifest row,
  the structure brief's *Early uses the order forces* table, the ledger row and
  chapter 2's anchor set all change together — or the row stands as a statement
  about the *load-bearing* first use, in which case the structure brief says so
  explicitly, so a later reviewer does not re-open it.
- If `TaskName::compose` and `Display` are split into two rows, both survive the
  byte-for-byte mandatory-row check.
- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-verdicts --check all` is still valid, and the resolved and deferred line
  counts are unchanged at 887 and 9,646.
- `bash scripts/check.sh` is red on `book-check` alone, as it is for every child
  of `the-grammar-k125` but the last.

## Notes

**This runs before chapter 3 deliberately.** `03-kind-slug-handle.md` is the page
that would otherwise create the anchor the disputed row promises, and settling
the question after that page exists means editing it rather than writing it.

**The corpus is frozen.** Nothing here touches `crates/grove-loop/`.

## Decisions (running log)

1. **The row is wrong, and it is split rather than reinterpreted.** The
   specification settles the direction:
   `docs/specs/walkthrough-books.md`, *Early-use ledger*, says a row is written
   *when a page first uses* a later-owned symbol, and its fourth column is *the
   minimum definition or behavior that the earlier page must state locally*. A
   row naming a page **later** than the real first use leaves the earlier page's
   local-statement obligation unrecorded, which is the leak the ledger exists to
   close — so the *load-bearing first use* reading is not available, however
   sound the brief's reasoning about the handle was. `the-tokens-k134` having to
   state the rendering behaviour at `02-the-tokens.md#refusals-inside-the-shape`
   anyway is the evidence that the obligation is real and lands on chapter 2.
2. **The two symbols are not one family, so one row becomes two.** Verified
   against `crates/grove-loop/src/task_name.rs` directly by enumerating every
   occurrence of `compose` and of the `Display` impl: `impl fmt::Display for
   TaskName` is defined at line 613 and `compose` at 896, both in block
   `591-1020`, which chapter 4 owns. `TaskName`'s `Display` is called at 882,
   1,213 and 1,296 (chapter 4's own blocks), 1,418 and 1,455 (chapter 2's
   `1313-1521`) and 1,587 (chapter 3's `1551-1714`) — so the earliest in page
   order is 1,418, in chapter 2. `TaskName::compose` is called at 1,562, 1,567,
   1,572 and 1,579 only, all inside chapter 3's `1551-1714`, and mentioned in doc
   comments at 1,106 and 1,116, inside chapter 4's. So `compose`'s chapter-3
   first use was always right and only `Display`'s was wrong. Two rows, one token
   each.
3. **The `Display` row's anchor is `#refusals-inside-the-shape`, not
   `#the-handle-in-this-grammar`.** Chapter 2's reproduced module header
   (`task_name.rs` 57–76, at `#the-handle-in-this-grammar`) *discusses* the
   renderer — *both of `TaskName`'s renderings end in a call to the former* —
   but performs no rendering, and the behaviour it leans on is already declared
   by the chapter-1 cast row for `TaskName` (*renders back to the bytes it was
   parsed from*). The first place the page **exercises** `Display` is the
   `to_string()` pair in `shape-refusal-tests`, under
   `#refusals-inside-the-shape` — which is also the anchor `the-tokens-k134`
   chose for the three rows it added over the same block, so the four rows the
   page owes now share one anchor.
4. **Five artifacts changed together, and the sweep was an enumeration rather
   than a file list.** `docs/walkthroughs/grove-loop/walkthrough.toml` (one
   `[[early-use]]` becomes two), `source-index.md`'s *Early uses* table (the same
   split, with the `Display` row placed after `` `a_kind`, `slug` `` because the
   canonical sort is chapter, then anchor byte offset, then owner slice order,
   then the bytewise symbols cell — and `` `a_kind` `` sorts before `` `impl ``),
   `docs/specs/grove-loop-book-structure.md`'s *Early uses the order forces*
   (three rows to four, plus a paragraph saying which reading was rejected and
   why, so a later reviewer does not re-open it), `02-the-tokens.md`'s local
   statement (*Three names* to *Four names*, with the rendering direction stated
   where the `to_string()` calls are read), and `01-orientation.md` (below).
   Then `grep -rn --hidden` for `TaskName::compose` and `Display for TaskName`
   over the whole repo, classifying every hit rather than checking a list of
   places: the survivors are the frozen source, these artifacts, and the
   `.grove/` records amended in decision 6.
5. **Chapter 1's roll-up was already stale and my split would have made it
   staler, so it no longer counts.** `01-orientation.md` said the source index
   carries the thirteen cast rows *alongside three more rows whose first use is
   on a later page*. Three was true when `orientation-k124` wrote it and became
   six when `the-tokens-k134` added `TaskName::distinguished`, `Parts::leaf` and
   the `a_kind` / `slug` helpers; this leaf would have made it seven. Counted
   directly: 20 rows in the ledger, 13 at `01-orientation.md#the-cast` and 7
   elsewhere. The sentence now states the structural fact — that the set grows as
   chapters land and the ledger is where it is counted — rather than a number
   that every later chapter invalidates. This is the summary layer the
   section-level finding would otherwise not have reached.
6. **Four live `.grove/` pointers were amended; the historical ones were not.**
   `kind-slug-handle-k135` carried a conditional — *settles whether
   the row still names this page* — which is now answered, and it runs next;
   `the-name-k136` listed the rows that close on its slice and had
   the old pair, so it now enumerates all six `canonical-or-nothing` rows and
   says to re-read the ledger rather than trust the count;
   `02-the-grammar-k125/BRIEF.md` stated the two anchors as fixed obligations
   and its *Found while drafting* note left the direction open, so both now
   record the settlement; `01-grove-loop-k123/BRIEF.md`'s symbols-cell spelling
   note said the family is *two tokens* in one row. The *Found while drafting*
   narrative of what was observed is left as written — it is history and it was
   accurate — with the outcome appended to it.
7. **The leaf's one in-session reviewer was spent on the adjudication**, because
   it is hard to reverse — chapter 3 is written against it — and no `review-*`
   leaf sits beside this one. One fresh context, six named axes, the conclusion
   stripped out. Classified: two axes clean (`compose`'s chapter, and the
   split's legality and table placement, both re-derived independently);
   **three findings valid and actionable** (decisions 8, 9, 10); one **contract
   I had stated unclearly** — the criterion for when a reproduced block owes a
   row was in my head and nowhere in the corpus, which is what let the reviewer
   read `#the-handle-in-this-grammar` as the right anchor; and one **noise**: it
   proposed leaving the mandatory row alone and adding a chapter-2 floor row
   instead, which on inspection produces two rows naming two different first uses
   for one symbol, a contradiction the split exists to remove.
8. **The anchor survives, but only because the criterion has two clauses.** The
   reviewer's strongest point is real: the specification's trigger is *first
   uses* **or** *reproduces source bytes whose referent belongs to a later
   slice*, and `entry_path`'s mandatory row is anchored on the module header that
   *names* it, so naming plainly counts. Checked what chapter 2's reproduced
   header at `task_name.rs` 57–76 actually names: `Handle`, `Handle::render`,
   `peel_key`, `split_shape`, `task_tree::handle_key` and `TaskName` — **not**
   `Display`, which appears only as the description *both of `TaskName`'s
   renderings*. And chapter 1's cast row for `TaskName` already states that it
   *renders back to the bytes it was parsed from*, which is the minimum that
   passage needs. So `#refusals-inside-the-shape` stands, and the general rule —
   *named or exercised, unless an existing row already covers it* — is now
   written into `docs/specs/grove-loop-book-structure.md` rather than left
   implicit. Stating it is what stops this being re-opened, which the *Done when*
   asks for.
9. **The floor rows the criterion now demands are `floor-rows-chapter-two-k138`,
   not this leaf.** `impl Display for TaskNameError` is the sharpest: chapter 2
   calls it at source 1,344, 1,386 and 1,472 — the first of them *before* the
   `TaskName` round-trip at 1,418 — and it is defined at 727, in chapter 4's
   block. `peel_key`, `split_shape` and `Handle::render` are named in the
   reproduced header and covered by no row. Adding one of them here and leaving
   the rest would split one enumeration across two sessions, which is the partial
   sweep that leaks; and they are a different symbol from this leaf's stated
   goal. Cut with `leaf-insert` against `kind-slug-handle-k135` so it runs before
   chapter 3, and its body names the four candidates while telling that session
   not to trust the list.
10. **Two contradictions my own edits left, both fixed here.**
    `01-grove-loop-k123/BRIEF.md` said *the families are the brief's; only the
    spelling is the validator's* two lines below text describing a family being
    split, which read as a leaf overriding a brief-level decision; it now says
    that a family whose members have different first uses is not a family, and
    that splitting one is a correction made in the brief. And decision 2 above
    claimed an exhaustive enumeration while listing two call sites; `Display` is
    also called at 882, 1,213, 1,296 and 1,587, none of which is earlier than
    1,418 in page order, and it now says so. Separately, `01-orientation.md`'s
    replacement sentence was writing about its own authoring problem; reworded.
11. **The DONE leaves were left alone.** `orientation-k124`'s decision log still
    records *the early-use ledger is sixteen rows* and *the families are
    unchanged*; both were true when written, both are narration of what that
    session did, and editing a retired leaf's record to track a later correction
    is how an audit trail stops being one.
12. **`bash scripts/check.sh` could not be run to completion, and the *Done
    when*'s last clause is evidenced a different way.** Two attempts. The first
    was killed rather than measured — it had started before the last of this
    leaf's edits, so its subjects moved under it. The second ran with everything
    frozen and **wedged for 17 hours 12 minutes** inside `cargo test --locked
    --workspace`, in `crates/grove/tests/loop_driver.rs`, with
    `a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver`
    FAILED and `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`
    never returning. That is `driver-lease-fixture-timing-k85`'s recorded wedge,
    fixture for fixture. Everything ahead of it passed: `cargo fmt`,
    `shellcheck`, `cargo clippy`, `plugin install` (21), `conformance` (93 rules,
    2,433 loaded-path checks), the `conformance suite` (11 controls), and every
    `book-validation` suite — including the 18 in `ledger_and_pages.rs`, which
    are the tests that guard what this leaf changed. Re-run alone afterwards,
    `cargo test --locked -p book-validation` is **150 passed, 0 failed**.
13. **`book-check`'s step was reproduced verbatim instead**, since it is the
    clause the *Done when* actually names and `check.sh` never reached it. Ran
    the script's own discovery loop — `--repo . --book <root> --final --check
    all` over `docs/walkthroughs/*/` — with this result: 6 books checked, 1
    failing. `grove-llm`, `jj-workspace`, `keyed-launch`, `ordinal-fs-tree` and
    `overview` are all `final=true` and valid, which is the cross-tree control
    that these edits broke nothing outside this book; `grove-loop` is the single
    failure, and every diagnostic in it is the book being a prefix — 19 `M101`
    missing-page rows for `03-kind-slug-handle.md` onward, 2 `M103` for the
    navigation and README lines that point at chapter 3, and 21 `F009`, of which
    19 are *every early-use row except the one whose owner is complete*. That
    last number is itself the arithmetic control: 20 rows minus the
    `four-verdicts`-owned row that is legitimately `explained` is 19, exactly one
    more than before this leaf added a row.
14. **One observation for `k85` that is not written into it from here.** The
    wedged run left **eight orphaned `configured-command.sh` fixture children
    reparented to PID 1**, aged 17 hours to 5 days — so they outlive the runs
    that spawned them and accumulate across sessions. This does not contradict
    k85's *the `loop_driver` binary at 0% CPU and no children*; it completes it,
    because a reparented child is not a child. `structure-brief-dependency-count-k132`
    set the precedent for exactly this and it holds: putting an unreviewed
    datapoint into another live leaf's Context under this leaf's commit is how a
    charter stops being that leaf's. Recorded here for k85 to pick up.
