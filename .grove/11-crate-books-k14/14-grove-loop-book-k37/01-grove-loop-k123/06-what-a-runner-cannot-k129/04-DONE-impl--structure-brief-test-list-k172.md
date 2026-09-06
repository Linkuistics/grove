# structure-brief-test-list-k172

## Goal

Correct `docs/specs/grove-loop-book-structure.md`'s chapter 17 section, whose
test list is wrong in both directions, so the rest of Part V and every later
editorial stage read a brief that agrees with the block.

## Context

- **The defect, exactly.** The section *17 · Which calls the lease admits —
  `which-calls-are-admitted`* (line 497) introduces its list with **The whole
  inline test module:** and then names **nine** tests.
  `crates/grove-loop/src/driver_lease.rs` lines 820–1383 contain **eighteen**
  `#[test]` functions — confirmed by the harness, which reports
  `18 passed` for `cargo test -p grove-loop --lib driver_lease`.
- **And one of the nine is not in the block at all.**
  `an_alias_equivalent_second_owner_is_refused_immediately` is
  `crates/grove-loop/tests/driver_lease.rs` line 398, an out-of-process
  integration test in a directory this book's corpus rule calls evidence rather
  than a root. It is owned by no chapter and must not be listed as though a
  chapter reproduced it.
- **The nine that survive are all real**, so this is an omission plus one
  misattribution, not a rewrite. The nine missing names are
  `epoch_acquisition_retries_open_lock_path_replacement_in_event_order`,
  `an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound`,
  `the_epoch_contention_diagnostic_names_the_lock_mode_and_operation`,
  `only_a_nonempty_loop_control_value_is_ambient_context`,
  `replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff`,
  `ambient_context_from_another_worktree_names_both_roots`,
  `an_inactive_epoch_is_reported_without_claiming_a_session_is_active`,
  `a_rotated_epoch_refuses_the_old_signal_path` and
  `an_epoch_signal_path_round_trips_record_separator_bytes`.
- **`17-the-epoch.md` already carries the correct enumeration**, in file order,
  under `#eighteen-not-nine`, and adjudicates the brief's error on the page. The
  page is the source to reconcile the brief *to*; do not reconcile it the other
  way.
- **Sweep for the count rather than for the sentence.** `the-lease-k164` found a
  size claim in a third site its own leaf body did not name, because the phrase
  was hard-wrapped and a one-line grep missed it. Enumerate every place the
  brief speaks about chapter 17's tests — its chapter 17 section, *What each
  chapter's prose owes* (chapter 17 is on the *supply the claim* list), *The
  mapping onto the corpus*, and the worked-example table — rather than searching
  for `nine`.
- **Nothing in the book changes.** The manifest, the ledger and the page are
  already right: the `lease-tests` ownership row reads `resolved` at 564 lines
  and `book-check --through which-calls-are-admitted` is valid. This leaf edits
  `docs/specs/` only, which is outside the frozen corpus.

## Done when

- The brief's chapter 17 section names eighteen tests, in file order, and no
  longer names `an_alias_equivalent_second_owner_is_refused_immediately` as part
  of the inline module.
- Every other site in the brief that states or implies chapter 17's test count
  agrees, found by enumeration rather than by grep.
- `bash scripts/check.sh` is red on `book-check` alone, as it is throughout this
  node.

## Notes

**This is the fifth structure-brief correction in this book** — after
`structure-brief-dependency-count-k132`, `pick-test-count-k147`,
`structure-brief-lexical-pair-k150`, `structure-brief-chapter-attributions-k163`
and `lease-size-ranking-k171`. The class is not closed and each later chapter
still owes the enumeration for its own block; what this leaf closes is chapter
17's list.

**A corrected sentence is not a checked sentence** (`pick-test-count-k147`). Read
the whole of every sentence you touch, not the clause you came for.

## Decisions (running log)

**Ten were missing, not nine — and the leaf body's own list was the second wrong
list.** The block holds eighteen `#[test]` functions; the brief's nine held eight
real names plus the misattributed
`an_alias_equivalent_second_owner_is_refused_immediately`, so the shortfall is
ten. This leaf's *Context* named only nine of them, omitting
`a_successful_liveness_probe_releases_the_lease_before_validation`
(`driver_lease.rs` line 1305, the sixteenth in file order). Settled by
enumerating the block with `awk 'NR>=820 && NR<=1383'` over `#[test]` and
diffing that against both the brief and `17-the-epoch.md#eighteen-not-nine` —
which agree with the source name-for-name and in order — rather than by trusting
either prose list. **A list handed to you is evidence of the same rank as the
list you came to fix.**

**The chapter 17 section is the only site.** Enumerated rather than grepped, per
the leaf's instruction: *What each chapter's prose owes* names chapter 17 on the
*supply the claim* list but states no per-chapter count (its figures are the
block set's — 3,984 lines, 38%, 14% prose); *The mapping onto the corpus* gives
`driver_lease.rs` two blocks and chapter 17 564 lines (1383 − 820 + 1 ✓); the
chapter-sequence table and the worked-example row carry no count; and the
outbound-link and glossary-anchor tables carry only anchors. A reverse sweep —
each of the eighteen names, and `alias_equivalent`, grepped across the whole
brief — returns nothing outside the corrected section.

**Recorded, not corrected: the rule sentence is worded two ways.** The brief says
*a handoff is ordered rather than timed out of trouble*; `17-the-epoch.md`'s
heading and rule blockquote say *ordered rather than raced*, and expand it to
*no deadline is any part of it*. Both are true of the block, so this is not a
defect in either and was left alone; a later editorial stage checking brief↔page
rule agreement should expect the divergence rather than read it as drift.

**Verification.** `cargo test -p grove-loop --lib driver_lease` reports
`18 passed`. `bash scripts/check.sh` is `FAILED — 1 of 8`, red on `book-check`
alone, whose five failures are chapter 18–21's absence and the README/navigation
entries that wait on them — unchanged by this leaf, which touched `docs/specs/`
only.
