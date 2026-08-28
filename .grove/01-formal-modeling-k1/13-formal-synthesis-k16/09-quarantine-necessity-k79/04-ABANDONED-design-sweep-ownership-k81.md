# sweep-ownership-k81

## Goal

Settle, as **one** question, the `FN-32` site the in-place candidate can reach
and Q4's reaper-coverage hole: *no shared-safety obligation in this repository
constrains the quarantine reaper's ownership proof.* The node brief's instruction
is explicit — **the two commissions are one question; do not solve them
separately** — and this leaf is where that instruction is honoured.

## Context

**Why they are one question.** `FN-32`'s transaction-side sites are
`SCreatePreparing`, `SQuarantineRename` and `SCreateMarker`. A candidate with
neither quarantine nor marker reaches only the first, which it inherits from the
incumbent unchanged, so `FN-32` is trivial over everything the candidate changes
— the defect `finish-verdicts-k78` found in `relax_EN_03` and could not repair
there. The only site the candidate *can* offer is the resumed sweep's: whatever
document a re-entered in-place disposal reads to know the entries it is removing
are Grove's. That is the same artifact Q4-6 asks about, so a decision that gives
`FN-32` a candidate-reachable site is a decision about the reaper's ownership
proof, and vice versa.

**The either/or, verbatim from the node brief.** *Either an obligation is stated
over the sweep's ownership proof (a manifest change, both families, cascading) or
the catalogue records the silence and the `none` cells are annotated as such* —
**and say which, because a matrix whose `none` cells cannot be read is worse than
one row short.**

**What is already established and must not be re-derived.**

- `FN-32` is stated over `groveActs - Reap`, with `Reap` excluded **on purpose**,
  so that a mutation aimed at the sweep cannot kill it and one aimed at a
  transaction step cannot kill `FN-21.c`. The two must stay separable; a model
  discharging both from one predicate makes each other's mutations unable to kill
  anything ([`finish.als`](../../../../crates/grove-finish/models/finish.als),
  beside `foreignAtReservedName`).
- The sweep's own fail-closed ownership is `FN-21.b`/`FN-21.c`, both **incumbent
  mechanics**, so neither is evidence about a candidate.
- **Do not carry the ADR's old universal sentence about "the reaper's actions"
  forward.** `FN-27`, `FN-28`(c)(d) and `FN-30`(b) *are* quantified over a set
  containing `Reap` and stayed green under row x1: the claim set did look at the
  sweep, and the sweep passed. The true statement is the narrow one — what it
  never asks is whether the sweep can **prove** what it touches.
- The Quint face is that `OWNERSHIP_PROVEN` is a free `const` rather than
  something the marker's presence derives, so no control in that column can make
  removing the marker cost a proof of ownership.

**What `honest-classification-k80` hands you, and why this leaf waits on it.**
Child 1 decides whether §*States* admits a member for a partially disposed root
and, if so, what artifact makes it hold. That artifact — if there is one — is the
candidate's ownership proof and therefore this leaf's `FN-32` site. If child 1
refuses the member, this leaf's question changes shape rather than disappearing:
a candidate with no reserved-name artifact has no ownership proof at all, and
whether *that* is a broken shared-safety claim or a silence the catalogue must
record is precisely what is left to decide.

**The manifest cost, if the obligation branch is taken.** A new obligation opens
an empty `(family, obligation)` cell that **both** families must fill with a
command before any coverage-asserting run is green again. `models/run.sh --list`
prints **130** obligations today. The Alloy finish cell is the long pole
(180 commands, ~15-18 min alone); budget an Apalache cell on its own if
`QUINT_VERIFY=1` is needed.

**And `integrate-review-design honest-classification-k85` left you two measured
facts rather than a tidier instrument, both of which are yours.**

- **The candidate no longer succeeds over bytes it has not proved, and the gap it
  leaves is the shape of your question.** `nextInPlaceDisposable` now walks only
  what this transaction's own evacuation moved, and when an entry is still
  `AtRoot` once those are gone the candidate **blocks** (`RecoveryPending`)
  rather than unlinking it — `scenario_in_place_late_result`, 56 of 8000 traces,
  with the candidate's ordinary successful exit still reached in 2130. That
  removes an unsafe success from your inheritance and **decides nothing of your
  either/or**: whether a resumed sweep may prove ownership over such an entry —
  and over the evacuated ones `SDisposeRootEntry` already unlinks — is still
  exactly your question. The block is the honest placeholder for a proof that
  does not exist yet, not a decision that none can.
- **`inv_FN_25b` is RED under the candidate, and it is pre-existing.** Once the
  candidate has released the task root, a recovery meeting an absent root with no
  correlation ticket blocks through `recoverOp`'s `rootless` site with `diagnose`
  returning the **empty set** — nothing of Grove's is left at a reserved name for
  `groveOwnedCorrelated` to read, so the block carries no diagnosis and `FN-25.b`
  fails. The incumbent leaves a quarantine there and never reaches it. Measured
  on the committed model at `6d0188dd`, **before** `k85` touched anything, so it
  is `honest-classification-k80`'s candidate rather than `k85`'s repair. It is
  the same disk `wit_FN_18_and_then_reads_as_nothing_outstanding` reads as
  `SIdle`, which is why it is yours: a candidate that leaves nothing at a
  reserved name has nothing for a resumed invocation to correlate to, and
  *that* is the ownership proof's absence showing up in a second claim.
  `scenario_in_place_late_result` declares the red in prose rather than asserting
  it, and `crates/grove-finish/models/README.md` says so where the module is
  documented.

## Done when

- The either/or is **decided and executed**, not described: either a
  shared-safety obligation over the sweep's ownership proof exists in the
  catalogue with a command in each family filling its cell, or the catalogue
  records in §*Q4* that the matrix is structurally silent about the sweep's
  ownership proof and every affected `none` cell in both READMEs carries that
  annotation.
- Whichever branch is taken, `FN-32` has a site the in-place candidate can reach,
  or the leaf states in the catalogue why it cannot have one and what follows for
  Q1's retained set — because a retained set cannot classify a difference one of
  its members is blind to.
- `FN-32` and `FN-21.c` are still separable: a mutation aimed at the sweep kills
  one and not the other, and both directions are asserted rather than argued.
- Both families' finish cells are green, with command counts, cell counts and
  wall times recorded; `models/run.sh --list`'s obligation count is recorded
  before and after and its movement (or non-movement) is stated.
- Nothing here classifies Q1 or Q4's three rows. The leaf records what its result
  makes readable for `q1-q4-verdict-k83` and stops.

## Notes

**Q4-7 needs neither branch and has a better reason already.** Row 45's green is
a vacuity artifact of its own mutation — the replace transition is the *only*
`groveActs - Reap` member whose marker mutation is gated on ownership
([`finish.als`](../../../../crates/grove-finish/models/finish.als):5930), so
narrowing it away removes the claim's content with it. What would replace that
row is a control narrowing the replace transition away **while keeping an
`FN-32` site with content** — which is the site this leaf is deciding, so the
replacement control may become available here. It is not owed here.

**Invent a fresh negative-control handle.** `formal-synthesis-k99` is burnt: a
sweep whose report lives inside its own subject measures itself, and that handle
went from 0 sites to 1 the moment a durable record said it found 0.
