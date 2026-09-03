# the-channel-k113

## Goal

Draft chapter 6 of the `keyed-launch` book — *Appearance is the event*,
`06-the-channel.md`, slice `appearance-is-the-event` — owning `src/channel.rs`
lines 1–271: 271 lines.

## Context

- Draft stage, child 6 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *6 · Appearance is the event — the channel*.
- What the channel is and why a launch needs one at all — an interactive child
  returns to its prompt when it finishes rather than exiting, so its own exit is
  not the event anyone is waiting for. **Allocation picks a name and writes
  nothing**, which is what makes *appearance* the event.
- The name grammar — a recognisable prefix plus 128 bits of randomness — and why
  the prefix is named for `signal` and matches what grove's driver already leaves
  behind; `DRAW_RETRY_LIMIT` as a bound rather than an unbounded retry, because at
  128 bits a collision means the randomness source is not random; why `allocate`
  checks the directory rather than leaving it to the child's first write; `Token`
  as opaque to this crate and readable to its caller, and the line framing as
  framing rather than interpretation; `signal` as a free function because the two
  ends are different processes; and `discard_abandoned`, **whose exactness is the
  point** — a looser rule would let this crate's cleanup delete a neighbouring
  file in a directory whose other contents belong to the consumer.
- **This is the chapter that cites `CONTEXT.md` at `loop-control-channel`**, at its
  first use of *channel* for the thing grove's glossary already names. That anchor
  exists in explicit form today.
- **The block stops at line 271.** Lines 272–404 are the inline `#[cfg(test)] mod
  tests` and are chapter 9's, which explains them against this chapter's fragments.
  Do not reach into them; do not add a corpus exception row.
- Required example anchor: `a-path-and-nothing-else` — `Channel::allocate` in the
  control directory through to a path that does not exist, and a token read back
  after `signal`.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  appearance-is-the-event --check all` is valid: 1,229 resolved lines, 844
  deferred, `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

`src/channel.rs` is the only root in this book split at a `cfg` boundary rather
than a conceptual one. The reason belongs on chapter 9's page, not this one.
