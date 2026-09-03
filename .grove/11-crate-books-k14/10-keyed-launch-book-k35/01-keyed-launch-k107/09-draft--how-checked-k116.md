# how-checked-k116

## Goal

Draft chapter 9 of the `keyed-launch` book — *How this is checked*,
`09-how-checked.md`, slice `checked-without-meaning` — owning `src/channel.rs`
lines 272–404 (133) and `src/conformance.rs` whole (104): 237 lines. This child
takes the book to 2,073 resolved lines and zero deferred.

## Context

- Draft stage, child 9 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *9 · How this is checked — checked without meaning*.
- The conformance kit as the **cross-crate seam** — a consumer's own suite can only
  assert that its configuration works with its build, and the kit is what holds a
  configuration to this crate's contract from outside the consumer; the three
  obligations `check` applies in order, and why the third is not a second spelling
  of `load`, since expansion is the only place the compiled words are walked.
- **Why an empty document fails is the chapter's best case for the spine**: a kit
  that only reports violations reads identically when handed nothing to check, so a
  configuration declaring no keys is a failure in its own right — not because an
  empty file is malformed but because a suite of must-hold claims cannot otherwise
  detect that it did not run.
- The nine inline channel tests, read as **what a `#[cfg(test)]` module inside a
  root buys that an integration test cannot** — reaching `is_channel_name`, a
  private function whose exactness chapter 6 argued and only this module can pin.
  Explain them against chapter 6's fragments, which are behind this page.
- **State once why these nine tests are in the corpus and the 1,319 lines under
  `crates/keyed-launch/tests/` are not**: a root is `src/**/*.rs`, and
  `docs/specs/walkthrough-books.md`'s corpus exception inventory carries no
  `keyed-launch` row. Do not add one.
- Required example anchor: `checking-the-same-file` — `conformance::check` over
  that primary, through an empty document failing and a violated rule reported.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  checked-without-meaning --check all` is valid: 2,073 resolved lines, 0 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone — the final scope still rejects
  the missing tenth page and the `pending` ledger rows — and this file says so.

## Notes

Zero deferred is not `final=true`. The tenth page does not exist yet, and a final
run rejects the book until it does; that is `what-passes-through-k117`'s.
