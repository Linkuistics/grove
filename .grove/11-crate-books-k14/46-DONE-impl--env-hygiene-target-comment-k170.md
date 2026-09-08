# env-hygiene-target-comment-k170

## Goal

Correct the stale doc comment on
`crates/grove/tests/env_hygiene.rs`'s `the_suite_cannot_reach_a_live_loop_signal_file`,
which describes a redirect the assertion nine lines below it explicitly forbids.

## Context

- **The defect.** Line 27 opens *`.cargo/config.toml` force-overrides
  `GROVE_SIGNAL_FILE` **into `target/`** for everything cargo runs.* The
  repository's `.cargo/config.toml` sets
  `GROVE_SIGNAL_FILE = { value = "", force = true }`, and the test's own
  assertion is `assert!(raw.is_empty(), "GROVE_SIGNAL_FILE must be force-cleared
  under cargo, **not redirected to** {:?}", …)`. The comment describes the design
  the assertion exists to reject.
- The rest of the docblock is correct and load-bearing: *or loses `force`, which
  is the subtle way to break it, since without `force` an inherited value wins*
  is exactly what `both_guards_are_present_and_neither_subsumes_the_other`
  checks. Only the clause naming `target/` is wrong.
- **The config file itself already argues the point** — *Empty is deliberately
  stronger than redirecting to an inert path* — so the repaired comment can
  simply agree with it rather than invent wording.
- **This is outside every book's corpus.** `crates/grove/tests/` is evidence
  rather than a root under the campaign's corpus rule, no page reproduces these
  bytes, and no ownership range or fragment moves. The freeze rule therefore does
  not defer this leaf behind anything; it is placed here only because it was
  found here.
- **Found while drafting chapter 16 of the `grove-loop` book**, checking the
  claim `driver_lease.rs` line 736 makes about who owns the force-clear. That
  claim is true and this leaf does not touch it.

## Done when

- The docblock describes the force-clear as clearing the variable to empty, and
  no clause survives that a reader could take as licensing a redirect.
- `cargo test -p grove --test env_hygiene` passes unchanged — this leaf changes
  a comment and no assertion.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Do not "fix" this by changing the assertion.** Empty is the stronger guard and
the config file says why: a nonempty path is session-epoch authority to
`grove-llm`, and an inert-looking redirect would still be nonempty.

## Decisions (running log)

1. **The repaired clause forbids the redirect rather than merely omitting it.**
   A bare *clears `GROVE_SIGNAL_FILE`* would have been true and would still have
   left the next reader free to reintroduce an inert path. The docblock now
   carries the config file's own argument — *deliberately stronger than
   redirecting it to an inert path, since every nonempty value now carries
   session-epoch authority* — which is the reason the assertion is written the
   way it is, stated where someone tempted to change the assertion will read it.
2. **No downstream surface moves.** The docblock grew from four lines to six, so
   the check was whether anything cites this file positionally. Nothing does:
   `crates/grove/tests/` is evidence rather than a book root, no fragment names
   `env_hygiene.rs` as a source, and the three prose references to it
   (`driver_lease.rs` line 736, `grove-loop/16-the-lease.md`,
   `grove-loop/20-the-loop.md`) name tests and behaviour, never line numbers or
   these bytes. Chapter 16 already describes the guard correctly — *force-clears
   `GROVE_SIGNAL_FILE` to the empty string* — so the book needed no edit and the
   comment now agrees with the page that adjudicates it.
