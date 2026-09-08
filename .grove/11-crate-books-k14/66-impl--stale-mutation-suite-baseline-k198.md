# stale-mutation-suite-baseline-k198

## Goal

Reconcile the `grove-loop` book's mutation-study baseline with the suite as it
now stands: **twenty citations across eight chapters say the control is 558
tests, and it is 560** — and decide, for each reddened-count claim that rests on
that baseline, whether the count itself moved with it.

## Context

- **The number is 560.** `cargo test --no-fail-fast -p grove-loop -p grove-llm`
  in this working tree reports **560 passed, 0 failed, 560 total**, measured
  twice. The mutation chapters describe a *workspace copy* that is neither a jj
  repository nor a checkout of the plugin marketplace, in which eleven of those
  fail before any mutation — but the copy runs the same tests, so the total is
  the total.
- **Two tests were added after the measurements were taken**, both by defect
  leaves that landed later in this node:
  `a_live_grove_offers_no_way_to_scaffold_over_it`
  (`crates/grove-loop/tests/verbs.rs:147`, from `unreachable-root-clause-k152`)
  and `both_scaffolding_doors_name_the_first_leaf_the_same`
  (`crates/grove-llm/tests/root_init.rs:166`, from
  `default-root-slug-two-spellings-k159`). 558 + 2 = 560.
- **Twenty citations, eight chapters, and the phrasing differs each time** — so
  a single find-and-replace will not close it and a pattern list will leak.
  `11-a-grove-begins.md` 176, 674, 1373, 1502, 1511; `12-leaf-to-node.md` 660,
  1625, 1638, 1699; `13-outcomes.md` 1776, 1781, 1904; `14-finishing.md` 1517,
  1525; `15-the-verbs.md` 803, 818, 1217; `16-the-lease.md` 1964;
  `17-the-epoch.md` 994; `19-the-core.md` 123. (A twenty-first `558` hit, in
  `grove-llm/07-what-order-holds.md:515`, is a hash inside a console transcript
  and is not this claim.)
- **The derived counts moved too, and that is the expensive half.** Chapter 11's
  *panicking unconditionally on entry to `root_init` reddens 41 tests*
  (`11-a-grove-begins.md:1513`) is **43** when re-measured, and the two extra are
  exactly the two tests above. Every *reddens N* and *leaves all 558 green*
  clause in the list is a claim about a run, so each has to be re-derived rather
  than adjusted by two: a mutant the new tests do not reach keeps its count, and
  one they do reach does not.
- **`k159` edited `11-a-grove-begins.md` and did not touch the totals**, which is
  how it stayed quiet: the page names that very leaf as an existing test a few
  hundred lines above the baseline it falsifies.
- **Found by the reviewer on `refused-grove-test-overclaims-k161`**, which
  re-derived chapter 11's table and confirmed its own delta left the total
  untouched — it added no test — but could not leave the stale 558 unreported.

## Done when

- Every one of the twenty citations states the baseline the suite actually has,
  each re-derived rather than copied from a neighbour, and the *eleven failing*
  half of the control is re-confirmed in the same copy rather than assumed.
- Every count derived from a mutation run in those eight chapters is
  **re-measured**, not adjusted arithmetically — including `11-a-grove-begins.md`'s
  41, and including every *leaves all 558 tests green* clause, since "green" is a
  claim about tests that did not exist when it was written.
- The tables naming individual observing tests are re-checked against the same
  runs, since a new test can join an observer column as easily as a total
  (`a-cited-test-list-is-a-measurement`).
- Whether the baseline is worth a mechanical check is decided either way with the
  reason recorded — this is the second roll-up in this book to go stale because a
  later leaf added a row nothing reports (`stale-book-counts-k196` records the
  first), so the condition is now a pattern rather than an incident.
- `book-check --final --check all` is green over every book touched, and
  `bash scripts/check.sh` is no worse than before.

## Notes

**No source change, so the freeze is not in play** — this is prose against a
measurement, and the leaf is deferred behind nothing.

**Measure once, with the suite frozen.** Finish every edit before the run, and
digest the workspace copy before and after: a baseline re-measured while another
leaf is landing a test is the same defect again with a newer number.

## Decisions (running log)
