# structure-brief-dependency-count-k132

## Goal

Correct the dependency count in `docs/specs/grove-loop-book-structure.md`, which
says four where `crates/grove-loop/Cargo.toml` declares five, before the count is
propagated into the eighteen chapters of the `grove-loop` book that have not been
drafted yet.

## Context

- **The defect, exactly.** The brief's *1 · Orientation — `allowed-to-mean`*
  section says *the manifest's four dependencies each carry their reason in
  situ* and then names four: `anyhow`, `libc`, `keyed-launch` and
  `ordinal-fs-tree`. The `[dependencies]` table declares **five** —
  `crates/grove-loop/Cargo.toml` lines 27, 28, 29, 30 and 38 — and the fifth is
  `jj-workspace`, which the manifest's own comment accounts for as one of *the
  three modules it composes* without giving it a clause of its own. The brief's
  *Worked examples* table repeats the number in chapter 1's observable end: *the
  twelve verbs, four dependencies and one error, named*. Both say four.
- **The book already says five**, and states which four carry a reason in situ.
  `orientation-k124` checked the count against the manifest rather than against
  the brief, under the draft stage's technical-truth charter, and recorded the
  disagreement in its decision log. So the page is right and the brief is wrong;
  nothing is red, and nothing will go red — which is exactly why this is worth a
  leaf rather than a note. A `copy-edit`, `proof` or assembly session reading the
  brief against the book would find a page that departs from its own contract and
  could correct it in the wrong direction.
- The specification's rule is that a brief and its book disagreeing *is a defect
  in one of them, not a licence to prefer either*
  (`docs/specs/walkthrough-books.md`, and the brief's own opening). This leaf
  settles which.
- **`jj-workspace` is not an incidental dependency.** Five modules of the crate
  `use` it directly — `tree_lifecycle.rs`, `driver_lease.rs`, `prompt.rs`,
  `session_config.rs` and `loop_driver.rs` — and `lib.rs` republishes `Commit`
  and `Workspace`, so the
  clause the manifest does not carry for it is a genuine gap in the manifest's
  own account, and the corrected brief should say so rather than merely bumping
  four to five.

## Done when

- `docs/specs/grove-loop-book-structure.md` states five dependencies in both
  places, names which four carry a reason in situ, and says what `jj-workspace`
  is reached for.
- The wording matches what `docs/walkthroughs/grove-loop/01-orientation.md`
  already says, so the two agree in the direction the source supports.
- `bash scripts/check.sh` is no worse than it was: red on `book-check` alone
  while the book is a prefix.

## Notes

**The corpus is frozen and this leaf does not touch it.** The manifest comment is
not wrong — it names three composed modules plus two libraries, which is five —
so there is no source defect here and no fix is owed in `crates/grove-loop/`.

## Decisions (running log)

1. **The brief is corrected, not the book, and not the manifest.** Verified
   against `crates/grove-loop/Cargo.toml` directly: `[dependencies]` opens at
   line 26 and declares five — `anyhow` 27, `jj-workspace` 28, `keyed-launch`
   29, `libc` 30, `ordinal-fs-tree` 38. The manifest's own comment is
   consistent with five (*the three modules it composes plus `anyhow` and
   `libc`*), so there is no source defect and the freeze is untouched.
2. **Two sites in the brief, and only two.** Enumerated every `dependenc` hit in
   `docs/specs/grove-loop-book-structure.md` rather than sweeping a pattern
   list: lines 258 (the *1 · Orientation* prose) and 695 (the *Worked examples*
   row) were the only count claims; 134, 262–263, 321 and 997 are about the
   imposed set or a named dependency and carry no count. The section fix and
   the summary-table fix are both made, so the roll-up layer is not left stale.
3. **The correction states the gap rather than bumping the number.** The prose
   now says the table declares five, that four carry a reason in situ, and that
   the fifth — `jj-workspace` — has none, the comment accounting for it only as
   one of *the three modules it composes*. It names what it is reached for: the
   crate's version-control seam, `use`d directly by `tree_lifecycle.rs`,
   `driver_lease.rs`, `prompt.rs`, `session_config.rs` and `loop_driver.rs`,
   with `lib.rs` re-exporting `Commit` and `Workspace`. Enumerated by grep over
   `crates/grove-loop/src/`: exactly those five `use` lines plus the `pub use`
   in `lib.rs` and one doc-comment link in `tree_lifecycle.rs` line 194.
4. **The wording tracks the page rather than being invented here.**
   `docs/walkthroughs/grove-loop/01-orientation.md` line 510 already calls the
   re-export *the version-control seam, which five modules of this crate reach
   directly*, and line 89 already says the table declares five and
   `jj-workspace` has none. The brief now says the same thing in the same terms.
5. **The stale `libc` clause is left alone.** The page records that the
   manifest's `flock(2)`-probe attribution no longer holds and that
   `manifest-dependency-clauses-k133` owns the source fix. This leaf describes
   *that a reason is present in situ*, not that every such reason is accurate,
   so nothing here pre-empts k133.
6. **One live pointer outside the spec was amended.**
   `.grove/11-crate-books-k14/14-grove-loop-book-k37/BRIEF.md`'s carried-forward
   note read *the structure brief says four dependencies* in the present tense
   and would have misdirected a later stage the moment this landed. Rewritten to
   record the count as settled at five. The remaining `four dependencies` hits
   under `.grove/` are retired task files and decision logs recording what the
   brief said at the time; those are history and are correct as written.
7. **`scripts/check.sh` is no worse than it was, and the second red is the known
   one.** The run reports `FAILED — 2 of 8`: `book-check` and `cargo test`.
   `book-check` is red on `docs/walkthroughs/grove-loop` alone — eight `M101`
   rows for the undrafted pages `14-finishing.md` through
   `21-what-could-not-move.md` — which is the book being a prefix, the state the
   *Done when* anticipates; `jj-workspace`, `keyed-launch`, `ordinal-fs-tree` and
   `overview` are all `valid … final=true`. `cargo test` failed on
   `a_second_driver_refuses_before_tree_access_or_launch` and
   `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`, both
   panicking in `testing/support.rs:136` with *nothing wrote first-ready … after
   120s* — the fixture pair, file and panic text that
   `driver-lease-fixture-timing-k85` is open on, with four sibling fixtures in
   the same binary reporting *running for over 60 seconds* before passing. The
   machine was sharing cores with another repository's `cargo` builds throughout.
   Re-run alone: `cargo test --locked -p grove-loop --test driver_lease` →
   **23 passed, 0 failed, 6.02s**. This diff is markdown-only and those fixtures
   read none of it.
8. **The first run was killed, not measured, and the earlier
   `GROVE_SIGNAL_FILE` suspicion is wrong.** A first `scripts/check.sh` wedged
   for 1h50m; `sample` put `cargo` in `Child::wait` and its `loop_driver` child
   in `Child::wait_with_output` with no live descendant. k85 has already measured
   `GROVE_SIGNAL_FILE` out as a cause — load is the variable — so the reading
   that counts is the second run above, taken with the run's subjects frozen.
9. **`k85` was not amended from here.** Its own record has solo reruns at 117s
   and 103s where this session measured 6.02s, which is a materially different
   reading of the same fixtures and worth its attention — but re-measuring is
   that leaf's work, and editing it from here would put an unreviewed datapoint
   in its Context under another leaf's commit.

