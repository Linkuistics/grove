# the-lease-k164

## Goal

Draft chapter 16 of the `grove-loop` book, *One live driver per working tree* —
`crates/grove-loop/src/driver_lease.rs` lines 1–819, the file's whole production
half, 819 lines — to a valid slice through `one-per-working-tree`; and decompose
`what-a-runner-cannot-k129` into one child per chapter first.

## Context

- **The charter is *supply the argument*, and it is this chapter's alone.** At
  12% comment prose over 819 lines this is the thinnest-argued root in the corpus
  and the largest owned block in the book. Per mechanism: the line that enforces
  it, the failure it
  prevents, and the record clause it keeps. The record is
  `docs/adr/one-live-driver-per-working-tree.md`, named in prose and never linked.
- **The counterexample is chapter 13's and is inherited, not re-argued.** See the
  node brief; `13-outcomes.md` lines 25–37 carry the wording.
- The block owns no `#[cfg(test)]` code, so it owes no *supply the claim* work
  and no mutation study. The tests that observe its refusals are chapter 17's,
  and the attribution is that chapter's obligation.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  one-per-working-tree --check all` is valid at 8,751 resolved lines, 1,782
  deferred, `final=false`.
- The `lease-and-epoch` ownership row reads `resolved`, and the chapter-1 cast
  row owned by `one-per-working-tree` reads `explained`.
- Contents and navigation updated; `scripts/check.sh` red on `book-check` alone,
  and this file says so.

## Decisions (running log)

**The leaf decomposes into five children, one per chapter, all cut now.** 2,601
lines over five chapters is not one session, the task body said to expect it, and
the three sibling parts that held more than one chapter (`the-grammar-k125`,
`the-walk-k126`, `no-word-for-k127`) each became a node one-chapter-per-child
while the one-chapter part (`the-surface-k128`) stayed a leaf. Children in page
order, because `--through` proves a canonical prefix.

**The node brief's *chapter 1* was wrong and is corrected to *chapter 13*.**
k129's leaf body said chapter 1 had already stated the re-derivation
counterexample in one sentence; the structure brief's §16 and its *The stated
outcome* both say chapter 13, and the pages agree with the brief. Grepping
`deriv|only state|must not hold|untracked` over both pages returns two hits in
`13-outcomes.md` (lines 26, 33) and none in `01-orientation.md` — the same
pattern dirty in one page and clean in the other, so the clean read is a
measurement rather than a broken instrument. Corrected in the node brief with the
evidence beside it.

**`probe_lease_holder` (line 458) names a function that does not exist, and the
page adjudicates it.** Enumerating every backticked token in lines 1–819 — twenty
tokens, classified rather than swept — leaves exactly one that resolves to
nothing: `grep -rn probe_lease_holder` over the whole repository hits only the
comment, while the control `probe_live_lease` hits five sites in the same trees.
The reader the sentence describes is `probe_live_lease_with_post_unlock_hook`,
which does parse the lease record, so **the behaviour is right and only the
address is wrong** — chapter 6's `llm_cli` shape. `jj file annotate` puts the
comment in `loop-crate-driver-k22`, the commit that created the file, and the
pre-move copy at `src/driver_lease.rs` carries it identically, so it was wrong
when written rather than outrun.

**A leaf is cut for the source fix rather than left to a passing session.**
`grow-header-stale-helper-k154` states the rule this follows — *the remaining
modules are swept by the chapters that read them, and each cuts its own leaf* —
and there is no leaf already touching this file's comments.

**`cargo doc` earns thirty warnings over the crate and none over this file, and
the page says what that is worth.** Verified by `touch`ing the file and re-running
`cargo doc --no-deps --document-private-items -p grove-loop`: thirty warnings,
none naming `driver_lease.rs`. The block's 103 comment lines are 9 `//!`, 61
`///` and 33 plain `//`, and line 458 is one of the 33 — so the clean run is true
and is evidence about seventy lines, not about the block.

**`tests/env_hygiene.rs` (line 736) is at `crates/grove/tests/`, not this
crate's, and the claim really is owned there.** Read as a `grove-loop`-relative
path it names nothing — that directory holds six entries and none is it — but
`the_suite_cannot_reach_a_live_loop_signal_file` asserts `raw.is_empty()` and
`both_guards_are_present_and_neither_subsumes_the_other` asserts `force = true`
in the config file, so the code bears the comment out. Not an adjudication: the
page names the real path and moves on.

**A separate leaf is cut for that test's own stale doc comment.**
`crates/grove/tests/env_hygiene.rs` line 27 says the config *force-overrides
`GROVE_SIGNAL_FILE` into `target/`* while the assertion nine lines below demands
`is_empty()` and its message explicitly rejects redirection. Outside this book's
corpus, reproduced by no page, and outside this leaf's goal — so it is
externalised rather than absorbed.

**The close-on-exec guarantee is stated as an enumeration with its one
exception.** Five opens in the block (157, 389, 471, 534, 652), four marked
(163, 391, 473, 654); the fifth is `/dev/urandom` in `random_nonce`, whose
descriptor is dropped inside the function that opened it and is never live across
a spawn. `acquired_driver_descriptors_are_close_on_exec` checks the two the guard
retains — `_worktree_directory` and `lease_file` — which is exactly the pair that
can reach one. Stated as *four of five, and why the fifth is not a gap*, per the
asymmetry rule.

**Two landed sentences rank this root by size and both are wrong, and
`lease-size-ranking-k171` holds the correction.** The structure brief's chapter 16
section says *the third largest* and `13-outcomes.md` says *the second largest
block in the corpus is 1,383 lines*. Enumerating the roots gives 2,725 / 2,023 /
1,714 / **1,383**, so the root is fourth; enumerating the owned blocks gives 819 /
808 / 775, so this chapter's block is first, and 1,383 is not a block at all since
the root splits into 819 and 564. A repository-wide grep for `third largest`
returned only this session's own files, which was the **instrument failing** —
the phrase wraps as `the third\nlargest` in the brief and a one-line pattern
cannot match it. The argument both sentences make is sound, so chapter 16
inherits the argument and counts the lines itself rather than repeating either
rank.

**Chapter 15 left two structural defects in the book's shared pages, and both are
fixed here rather than leafed, because they are this stage's charter and in the
lines this chapter had to edit.** `README.md`'s contents carried chapter 15 twice
— once linked and once as the unlinked placeholder its own session should have
consumed — and `concept-index.md` had chapter 15's twenty-six entries inserted
*inside* chapter 14's run rather than after it, breaking the index's chapter
order. Neither is a sentence-level defect a later stage owns; both are the
ordered section plan as a reader meets it.

**The gate is red on `book-check` alone and that is the shape.** `scripts/check.sh`
reports `check: FAILED — 1 of 8`, with `cargo test` and the six other checks green
and `book-check` the only failure. Its nineteen findings over this book are all
the prefix class and none touches this chapter: eight `F003` for the four defers
that remain (`lease-tests`, `whose-file`, `the-prompt-core`, `loop-driver`), four
`F009` for `lease-tests`'s still-`deferred` ledger row and the three chapter-1
cast rows chapters 18 to 20 own, five `M101` for the missing pages 17 to 21, and
two `M103` for the contents list and the `Next` link this chapter cannot yet
carry. Run in final mode over each book root separately on this exact tree, the
other five books report **0 findings**.

**One reading was thrown away rather than reported, and the reason is in the
brief chain.** A second gate run overlapped an unrelated `cargo test -p
apianyware-emit-sbcl` on this machine and produced a failing
`a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver`
plus two `prompt.rs` mandate tests hung past sixty seconds — the wedge
`no-word-for-k127` promoted as *both wedge under load and a timeout reads exactly
like an observer*. That run was killed rather than read, and its `2 of 8` is not a
measurement of anything: `cargo test`'s failure there is partly the `pkill` used
to clear it. The clean run above is the one this leaf reports, and `jj diff
--name-only` confirms this change touches **no `.rs` file at all**, so no Rust
test's verdict can belong to it.
