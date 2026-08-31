# plan-k2

**Reviews:** plan-k1

## Goal

An adversarial read of the root brief and `plan-k1`'s decision log, before any
of it is built on. Findings only; fix nothing.

## Context

Eighteen decisions, four of them reversed mid-interview, settled in one sitting
and executed thereafter by a long chain of sessions with no human present. The
reversals were the interview working rather than a fault — but they mean the
final set was never read whole by anyone who had not just argued their way to
it.

## Done when

Every decision has been read against the repository it claims about, and the
findings are recorded. Cut an integration only if there is something worth
acting on.

## Notes

Read for these in particular.

- **Decisions that reversed.** 1 (depth), 9 (frozen corpus), 13 (loop in
  grove), and the scope widening in 4. Each reversal cascaded, and a
  consequence may have been missed where it was recorded rather than traced.
- **Whether the frozen corpus survives contact.** Decision 9 forbids inline
  fixes across a campaign of many sessions over 14,525 lines. Ask what happens
  the first time a book cannot be written truthfully without a code change.
- **Whether the architecture move is safe as stated.** Decision 5 moves three
  sections out of a document whose anchors are cited from Rust source and
  asserted at compile time. The claim is that red tests make this mechanical.
  Check that claim rather than accepting it.
- **Whether the set is right.** Decision 6 folds `crates/grove` into the
  overview *and* decision 1 requires complete source coverage — so the overview
  owns production source as fragments. Ask whether that is coherent.
- **Whether four products belong in one grove at all**, and whether the
  ordering in the brief's *Decomposition* actually keeps the documentation
  unblocked, given `pick` is a walk and not a scheduler.
- **What the brief assumes and never states.** The interview settled what was
  asked; a fresh reader is the only one who can see what was not.
