# watcher-test-hardening-k7

**Kind:** work

## Goal

Make the driver-side watcher's suite fail when the watcher is wrong, not just
when it is absent. Three mutants currently survive all 20 `tests/loop_driver.rs`
tests.

## Context

Found by the adversarial review in review-k3, which ran the mutations rather
than eyeballing the assertions. The three new tests from driver-side-kill-k2
(`driver_kills_a_hung_session_that_signalled_{done,relaunch}`,
`driver_escalates_to_sigkill_when_the_session_ignores_sigterm`) are not
vacuous — they do fail if no kill ever lands — but they prove only *that* a
kill of some kind eventually happens, and nothing about **when**, **which
signal**, or **only on signal**. Surviving mutants, each 20/20 green:

1. **Drop the `signal_file.exists()` guard** — the driver then SIGTERMs *every*
   session `grace` after launch, signalled or not. Nothing in the suite proves
   the watcher leaves an un-signalled session alone, because every fake harness
   either exits immediately (so `try_wait` wins the race) or signals-then-hangs;
   there is no long-running-never-signals fixture. This mutant would break every
   interactive grove on its first launch with CI green. Worst of the three, and
   it is the same property signal-file-identity-k6 violates in production —
   land that fixture and both are netted.
2. **Swap SIGTERM for SIGKILL** — the escalation test's `trap '' TERM` fixture
   passes whether or not TERM is ever sent, so the TERM-before-KILL ordering
   (called out in this grove's BRIEF Notes) is untested in both directions.
3. **Drop the grace guard**, firing the kill the instant the file appears —
   every timing assertion is an upper bound (`elapsed < 10s`), there is no lower
   bound anywhere in the file. The grace exists precisely so `complete`'s
   Bash-tool call can return and the agent's turn can end before the session
   dies; a regression to grace=0 is invisible.

## Done when

All three mutants fail the suite. In particular: a fixture whose harness runs
long and never signals proves the driver does *not* kill it; TERM and KILL are
distinguished observably (e.g. the fake harness traps TERM and records it,
rather than only ignoring it); and at least one lower-bound timing assertion
pins the grace. Re-run the mutations to confirm rather than asserting the tests
"look stronger".

## Notes

Mutation-testing the suite is the check that caught this — worth repeating on
any future change to the watcher, and worth a line in the grove's own review
habits if it keeps paying off.

**Outcome.** All three re-verified by hand-applying each mutant and
confirming the new/modified test fails, then restoring and confirming the
full suite (23 tests in `tests/loop_driver.rs`, 287+ across the workspace)
passes clean:

- Mutant 1 (guard dropped) — new `driver_leaves_an_unsignalled_session_alone`
  (a long-running, never-signalling fixture with a lower-bound timing
  assertion). Also confirmed via `cargo-mutants` on `293:31: ... with true`.
  Note: the existing `concurrent_loops_...` test's victim already caught this
  mutant too, but only by ~180ms out of its 1.2s margin — too tight to trust
  as this property's real coverage, hence the dedicated fixture.
- Mutant 2 (SIGTERM→SIGKILL swap) — `driver_escalates_to_sigkill_when_the_
  session_ignores_sigterm` reworked: the fixture now traps and *records*
  SIGTERM (rather than `trap '' TERM` ignoring it) so a marker file only
  appears if a real, catchable SIGTERM landed before the SIGKILL. Not an
  auto-generated `cargo-mutants` mutation (it doesn't swap sibling
  constants); verified by hand-editing the literal and confirming the test
  fails, then restoring.
- Mutant 3 (grace guard dropped) — new
  `driver_waits_the_grace_before_sending_sigterm` (large configured grace +
  a lower-bound timing assertion that separates "waited the grace" from
  "killed within ~2 poll intervals regardless of grace"). Confirmed via
  `cargo-mutants` on `294:37: ... with true`.
