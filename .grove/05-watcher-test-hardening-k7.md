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
