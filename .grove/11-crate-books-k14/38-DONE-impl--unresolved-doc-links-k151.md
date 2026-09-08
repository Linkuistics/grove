# unresolved-doc-links-k151

## Goal

Repair the five unresolved rustdoc intra-doc links in `crates/grove-loop/src/`,
reconcile every book page that reproduces the changed bytes, and add the
early-use rows those links owe.

## Context

- **The defect, enumerated.** `cargo doc --no-deps --document-private-items -p
  grove-loop` reports five `unresolved link` warnings. Three are in
  `src/task_tree.rs`: line 580 `[`pick`]`, line 586 `[`select`]`, line 638
  `[`kind`]`. The other two are `src/prompt.rs` line 28 `[`crate::methodology`]`
  and `src/lib.rs` line 283 `[`Resolution::Ambiguous`]`. Found by
  `kind-and-briefs-k144`.
- **The cause for the `task_tree.rs` three.** `pick`, `select` and `kind` are
  public functions of `crate::verbs`, and `task_tree` does not import `verbs` —
  its only `crate::` imports are `task_name::{…}` and `Reference`. So rustdoc
  resolves none of the three, renders the text as literal `[kind]`, and warns.
  **The prose around each link is true**; only the link is broken, which is why
  chapters 7 and 8 adjudicated rather than corrected.
- **Nothing fails today.** A broken intra-doc link is a rustdoc warning, not a
  build or test failure, and `scripts/check.sh` does not run `cargo doc`. The
  instrument is the `cargo doc` invocation above and the rendered docblock —
  never reading the source, because nothing warns.
- **The likely fix is a qualified path** — `[`verbs::kind`]` or
  `[`crate::verbs::kind`]` — which changes bytes on one line and no line counts.
  Check the other two against their own modules before assuming the same shape;
  `crate::methodology` may name something that no longer exists, in which case
  the repair is to the sentence rather than to the path.
- **Two book pages reproduce the changed bytes** and must change in the same
  commit: `docs/walkthroughs/grove-loop/07-the-walk.md` (fragments
  `«walk-pick-in»` 580-585 and `«walk-select-in»` 586-595) and
  `08-kind-and-briefs.md` (fragment `«kind-in»` 638-656). `01-orientation.md`
  reproduces `lib.rs` 283. `prompt.rs` line 28 is chapter 19's, `«the-prompt-core»`,
  which is **not yet written** — see *Notes*.
- **Two early-use rows are owed and are not in the ledger.** Chapter 7's block
  reproduces `[`select`]`, whose referent `verbs::select` has no row anywhere;
  `pick` has one (owner `first-live-leaf`) but it is about the walk rather than
  about the public verb. `kind-and-briefs-k144` added the two rows its own block
  owed — `verbs::kind` and `verbs::brief_chain` — and chapter 7's equivalent
  sweep did not run. Adding a row requires the earlier page to state the minimum
  locally, so this leaf edits chapter 7's prose as well as the ledger.

## Done when

- `cargo doc --no-deps --document-private-items -p grove-loop` reports no
  `unresolved link` warning.
- Every book page reproducing a changed line reproduces the new bytes, and
  `book-check` is green over every book the commit touched, at whatever slice
  each is proved at when this runs.
- The `verbs::select` row exists in the early-use ledger with chapter 7 stating
  its minimum locally, and chapters 7 and 8's adjudicating paragraphs are
  rewritten to describe the repaired links rather than the broken ones.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** `prompt.rs`
line 28 belongs to chapter 19, which `what-a-runner-cannot-k129` has not written;
fixing it now would put the corrected bytes in the source before the page that
must reproduce them exists. Run this after `grove-loop-k123`'s last child has
taken the book to green `--final` validation. Until then the three adjudications
on the pages stand as the record.

**The freeze holds.** One commit carries the source change, every affected ledger
and page, and a green validator run over every book it touched — or the leaf is
deferred behind the books it would invalidate, which is what the paragraph above
does. None of the fixes changes a line count, so no ownership range moves.

## Decisions (running log)

1. **The deferral condition holds.** Every leaf under `14-grove-loop-book-k37`
   is terminal, and chapter 19's `«core-header»` fragment (`prompt.rs` 1-41)
   exists and reproduces line 28. So the leaf runs now rather than deferring
   again.

2. **The defect reproduces exactly, and is exactly five.** `cargo doc --no-deps
   --document-private-items -p grove-loop` reports 31 warnings, of which five
   are `unresolved link` — at `task_tree.rs` 580, 586 and 638, `prompt.rs` 28
   and `lib.rs` 283, as the task file enumerates. The remaining twenty-six are
   four other classes — 11 `` `write` is both a function and a macro ``, 10
   *public documentation … links to private item*, 4 `` `complete` is both a
   function and a module ``, 1 *redundant explicit link target* — and are **not**
   this leaf's: its `Done when` names `unresolved link` only, and the other
   classes are recorded in the book as separate residues. (The count in an
   earlier draft of this entry said three classes and 8 private-item links; both
   were wrong, and the corrected figures are what a `grep '^warning:' | uniq -c`
   over the run reports.)

3. **The task file's single cause is wrong for two of the five.** Its *Context*
   attributes the `task_tree.rs` three to `verbs` not being imported, and that
   holds. It then assumes `verbs::select` exists and merely lacks an early-use
   row. It does not exist: `verbs.rs` exposes `root_init`, `pick`, `kind`,
   `brief_chain`, `resolve`, `leaf_add`, `leaf_insert`, `stale_cross_refs`,
   `leaf_decompose`, `leaf_retire`, `leaf_prune`, `finish_commit`, `complete`
   and `signal_channel` — and no `select`. Nor has it ever: a sweep of all
   1,512 commits for `pub fn select` / `pub use …select` in `verbs.rs` returns
   nothing, against a positive control (`pub fn pick`, same loop, same file)
   that hits 236. `grove-llm --help` lists no `select` verb either. So
   `[`select`]` at 586 names no item in any scope, and its repair is not a
   qualified path to the same name.

4. **`crate::methodology` is deleted, not mis-scoped.** The sentence at
   `prompt.rs` 28 is *about* the deletion — `prompt-names-the-kind-k18` removed
   this module's dependency on it — and no `methodology` module exists anywhere
   in the workspace. The repair is to unlink, keeping the name as a code span;
   a link is wrong for a thing whose absence is the point.

5. **The five repairs, each exactly true.**
   - `task_tree.rs` 580 (`pick_in`): `[`crate::verbs::pick`]'s path` — `pick_in`
     is `select_in` with the handle and the kind dropped, so it is that verb's
     path and not that verb. Reflows onto line 581; still two lines.
   - `task_tree.rs` 586 (`select_in`): `[`crate::verbs::pick`]` — `verbs::pick`
     is literally `sought(select_in(tree)?)`, so this is the one link of the
     three whose referent is an identity.
   - `task_tree.rs` 638 (`kind_in`): `[`crate::verbs::kind`]` — `verbs::kind`
     calls `kind_in` directly.
   - `prompt.rs` 28: unlinked to `` `crate::methodology` ``.
   - `lib.rs` 283: `[`verbs::Resolution::Ambiguous`]` — `Resolution` is `pub`
     inside the private `mod task_tree` and reaches the public API only as
     `verbs::Resolution`, so the re-export path is the only one that resolves.
     The `verbs::` prefix matches `[`verbs::resolve`]` on the line above.
   No line count changes.

6. **The blast radius is much wider than the task file's two pages, and the
   validator sees only the narrow part.** `book-check --final` reports fragment
   mismatches on four pages — `01-orientation.md` (`«library-root-reference»`),
   `07-the-walk.md` (`«walk-pick-in»`, `«walk-select-in»`), `08-kind-and-briefs.md`
   (`«kind-in»`) and `19-the-core.md` (`«core-header»`). It is silent about every
   *prose* claim the same change falsifies, and there are three classes of those:
   the crate's warning count, stated on eight pages; the adjudicating paragraphs
   on chapters 8 and 19; and the ledger and index rows.

7. **The book's `thirty warnings` was already stale before this leaf, by one.**
   Measured against `@-`: thirty-one warnings — ten `task_name.rs`, ten `lib.rs`,
   six `task_tree.rs`, four `verbs.rs`, one `prompt.rs`. Chapter 14 states the same
   breakdown with *nine* in `lib.rs`, so the extra one arrived after the chapter was
   drafted; it is the `write` ambiguity at `lib.rs:268`, in the wording
   `grove-root-join-clauses-k148` landed. After this leaf: twenty-six — ten
   `task_name.rs`, nine `lib.rs`, four `verbs.rs`, three `task_tree.rs`, none in
   `prompt.rs`. Every page states twenty-six; the stale one is corrected rather
   than decremented, because both readings had to be re-run anyway.

8. **Each control table was re-derived, not decremented.** Planting
   `[`no_such_symbol_planted_here`]` and re-running: production `//!` header of
   `driver_lease.rs` → 27; inside its `#[cfg(test)]` module → 26; `//!` header of
   `session_config.rs` → 27; `///` docblock in `prompt.rs` → 27; `///` docblock in
   `loop_driver.rs` → 27; inside `loop_driver.rs`'s `#[cfg(test)]` module → 26;
   inside its plain `//` header → 26. So chapters 17, 18, 19 and 20 keep their
   arguments intact with every number moved, and chapter 20's four-row table is
   26 / **27** / 26 / 26. Watching controls B and C stay flat is the part that
   matters: had either moved, decrementing the old table would have preserved a
   false argument.

9. **Chapter 19's instrument reading inverts.** `prompt.rs` earned exactly one
   warning and now earns none, so the sentence *names this file in exactly one of
   them* becomes *in none of them* — and the silence now covers all 171 of the
   file's comment lines rather than 170. The chapter keeps its measurement because
   the planted-link control still takes the crate to twenty-seven.

10. **The early-use ledger had fifty-two rows, not the fifty chapter 21 states.**
    `kind-and-briefs-k144` added two after that page was drafted. With
    `verbs::pick` it is fifty-three: thirteen at *The cast* and forty elsewhere,
    every one `explained`, counted by enumerating the table rather than by reading
    the page. Chapter 21 now states fifty-three and says why the count has moved.

11. **A false clause found in passing is not this leaf's, and is externalised.**
    The book calls `verbs::kind` *the tree-opening half* of `kind_in`
    (`08-kind-and-briefs.md:139`, `15-the-verbs.md:325`, and the `verbs::kind` row
    of the early-use ledger, which says outright *it opens the tree*). It does
    not: `pub fn kind(tree: &Tree, leaf: Option<&Path>)` takes an already-open
    tree, as `verbs::pick` and `verbs::brief_chain` do, and chapter 6 reproduces
    the comment saying every verb has since `loop-crate-verbs-k21`. The clause is
    true only of the *write* verbs, which take a `TreeWrite` that reopens. Found
    while writing the `verbs::pick` row beside it; it serves no part of this
    leaf's goal, so it goes to the tree rather than into this commit. This leaf's
    own new prose says *the public half … takes the same already-open `&Tree`*
    instead, so the commit adds no fourth instance.

12. **Cut `tree-opening-half-claim-k195`** at position 63 under `crate-books-k14`,
    `--kind impl`, carrying the three surfaces, the signatures that falsify them,
    and the instruction to enumerate the verbs by first-argument type rather than
    to grep the hard-wrapped phrase. It touches no source, so it is deferred behind
    nothing.

13. **The whole set is green.** `cargo doc --no-deps --document-private-items -p
    grove-loop` reports no `unresolved link` warning; `book-check --final --check
    all` is `valid: 13 files, 10542 resolved lines, 0 deferred lines` over
    `docs/walkthroughs/grove-loop`; and `bash scripts/check.sh` reports *all 8
    principal checks pass* with all six books green — the same transcript chapter
    21's *Final verification* section reproduces, unchanged.

14. **Decisions 3 and 5 are superseded: the three `task_tree.rs` links are
    unlinked, not re-pointed.** The in-session adversarial reviewer this leaf
    spent its one allowance on found the referent, and it is neither the public
    verb chapter 8 named nor the nothing decision 3 concluded. `task_tree.rs`
    declares `pick` at 1,039, `select` at 1,044 and `kind` at 1,053 — inside its
    own `#[cfg(test)] mod tests`, each `read(grove_root)?` followed by the
    matching `*_in`. That is exactly what *X against a tree already read*
    distinguishes this function from, and it makes one reading fit all three
    identical sentences. The public-verb reading fits none of them: `verbs::pick`
    and `verbs::kind` take an already-open `&Tree` themselves, so the qualifier
    would distinguish nothing, and the comments would have been made vacuous by
    the repair. The same-module reading is corroborated three ways: `select_in_write`
    two functions below writes `[`select_in`]`, a same-module target, in the same
    sentence frame; `tests::kind`'s own comment writes `[`pick`]` and resolves,
    because inside the module it does; and the three compositions' own doc
    comments — reproduced in chapter 6 — spell `pick`, `select` and `kind` as
    **code spans**, which is the convention the repair restores.

15. **So no path repairs them, and that is the finding.** `cargo doc` compiles
    with `cfg(test)` off, so a link from production *into* a test module has no
    documented item at the end of any spelling. Chapter 17 measures that wall from
    the inside — a broken link planted within a test module leaves the count
    unmoved — and this is the same wall from the production side. The repair is to
    unlink, uniform with `prompt.rs` 28, so four of the five links are unlinkings
    and only `lib.rs` 283 takes a path. Cost of the wrong first repair: two
    incoherent source comments and a false *named nothing anywhere* claim in
    chapter 7, both caught before the commit.

16. **Two further findings are not this leaf's and go to the tree.** Chapter 21
    counts fifty early-use rows against the table's fifty-two, and the count went
    stale at `canonicalisation-sites-k149` — verified by counting the table at
    each commit back to `what-could-not-move-k130`, where it was 50 throughout and
    became 52 at k149. Chapter 20 says all **eight** of `loop_driver.rs`'s
    intra-doc links resolve; only seven are intra-doc links, the eighth token
    being at line 24 inside the 54-line plain `//` header the same paragraph says
    `cargo doc` never reads. Neither is caused by this commit — the ledger has 52
    rows before and after it — so both go to `stale-book-counts-k196` rather than
    into it.

17. **Two of `tree-opening-half-claim-k195`'s surfaces were repaired here after
    all**, because this leaf's own change falsified the *links to* clause they
    were built around: `08-kind-and-briefs.md`'s identification of the referent,
    and the `verbs::kind` early-use row. k195 was rewritten to the six that
    survive, and gained the sentence a correction has to agree with —
    `15-the-verbs.md`'s *It opens the tree itself, unlike every other verb here*,
    at line 90, which the same chapter contradicts at line 325.

18. **One reviewer finding is accepted and left as it stands.** `lib.rs` 283 is
    now 84 columns against 78–81 on the lines it wraps with. Reflowing it would
    change the paragraph's line count, which is the one thing the freeze forbids
    here; the file already carries 95-column code lines, so nothing enforces the
    narrower wrap. A visible trade-off rather than a defect.

19. **One claim in the new chapter 7 prose was wrong and was caught on re-read.**
    It said the three compositions spell the names as code spans *and never as
    links*. `tests::kind`'s own comment writes `[`pick`]` as a link at line 1,051
    — and that instance is the point rather than a counter-example, because inside
    the test module it resolves. The sentence now says so.
