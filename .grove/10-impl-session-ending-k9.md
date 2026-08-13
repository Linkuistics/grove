# session-ending-k9

## Goal

Specialise the session-ending instruction so each of the nineteen kinds is told
exactly its own ending and never an exception to someone else's, document the
reopened-`finish` ending, and guard both by test over the whole kind set.

## Context

This is the increment's point. The spec sections are *The ending is specialised
where the conditional lives* and *A `finish` session that reopens the grove
signals a relaunch*; D2 and D4 in the root brief are the decisions they record.

**Three scopes result** from splitting `skill-signal` and `skill-finish` in
`content/SKILL.md`:

- the **relaunch ending**, scoped to the eighteen non-`finish` labels spelled
  out explicitly — such a session is told to run `grove-llm complete` as its
  last action and is told nothing about `--done`, because the exception is not
  about it;
- the **finish endings**, scoped to `finish`, stated as *outcomes of what the
  session did* rather than as a rule with another kind's rule beside it;
- one sentence stays **`kinds=*`** — a session never discovers that a grove is
  finished; the driver does, and tells it by launching a `finish` session. This
  is the negative trigger, and withholding it would attach an unasked question
  to a destructive action. `finish` reads it as a true statement of how it came
  to be launched, so `*` rather than a second spelling of the eighteen.

**The cut is whatever the ending forces, and neither split is assumed to be a
single one.** `specialised-ending-k6` confirmed the shape (claim 3):
`skill-finish` holds **two** universal fragments — the negative trigger, and the
clause telling every session that non-routine asks are discretionary
escalations. The sentinel mechanics, the confirmation gate and the
outcome-specific endings are `finish`-only, and the "can neither starve nor
preempt real work" clause is reopening mechanics rather than a second universal
ending. Prose that rides along on the `finish` side of a forced boundary is not
a scope audit.

**The finish endings stay triggering, not procedural** (confirmed claim 1). A
`finish` session that externalises surfaced work never reaches the teardown
steps, so an ending deferred into `skill-finish-steps` is an ending it never
fetches.

**The third ending is prose, not a mechanism.** `pick` already selects a leaf a
`finish` session adds at the root even though it lands after the sentinel
(`src/tree_read.rs`), and `src/complete.rs` is not gated by kind. Write the
three outcomes:

| what the session did | ending |
|---|---|
| teardown completed | `grove-llm complete --done` — the loop stops |
| externalised work instead | `grove-llm complete` — the loop relaunches and picks the new leaf; the sentinel waits |
| declined, or no human present | no signal — the loop stops, the leaf stays live and resumable |

**The guard** belongs in `tests/session_kind_guidance.rs`, which already
generates its claims from `Kind::ALL` and already states the limits of each
sweep. Four claims, every one about unit membership or a token, because
membership and bytes are what the seam returns: exactly one unit from a
**declared ending set** per kind; no mandate but `finish`'s carries the `--done`
token and `finish`'s does; within every mandate the completion verb is named
only by units the declared set names; and the negative trigger appears in all
nineteen. **Both controls**, on the precedent's own rule that a sweep which
cannot fail is worth nothing: membership shown failing on a kind whose ending
unit's scope is withdrawn, and the complement sweep shown failing on a synthetic
mandate that names the verb outside the declared set.

**Two limbs stay prose and are recorded as prose** — that the `finish` unit
states its endings as outcomes, and that no unit restates an ending in words
naming neither the completion verb nor `--done`. The composer returns opaque
bytes with no role metadata, so a mechanical claim about either would be a
substring heuristic wearing a SHALL. They are carried by the classification
review and pinned for **drift** by a targeted byte-level assertion on the ending
units' own source bytes, which this increment adds beside the claim that needs
it. **The composition golden is not that pin.** It holds each kind's ordered
unit ids, so it moves when a unit is gained, lost, re-scoped or re-ordered and
does *not* move when the prose inside an ending unit is rewritten — which is
exactly the drift these two limbs are exposed to
(`docs/specs/mandate-delivered-methodology.md`, *Every kind's mandate states
exactly one session ending*).

## Done when

- The eighteen non-`finish` kinds' composed mandates carry the relaunch ending
  and no `--done` token; the `finish` mandate carries its own three endings as
  outcomes and no other kind's ending unit; the negative trigger is in all
  nineteen. No composed mandate branches on session kind.
- The reopened-`finish` ending is stated in `content/`.
- The guard exists in `tests/session_kind_guidance.rs` with both controls,
  generated from `Kind::ALL` so a twentieth kind fails loudly and by name rather
  than launching sessions that never signal the loop.
- A targeted byte-level assertion pins the ending units' **own source bytes**.
  That is the drift pin the two prose limbs rely on, and the ID-level golden
  cannot supply it.
- The composition golden and the pinned unit-id set are updated for the
  composition drift they *do* carry — a unit gained, lost, re-scoped, or moved.
- No unit **outside** the session-ending instruction is re-scoped —
  `unit-scope-audit-k4` owns that question and must stay a separate increment.
- `CONTEXT.md`'s *Complete finish cycle* entry already carries the three-ending
  reading; check it against what lands and do not churn it.
- `cargo test` is green.

## Notes

**This slice rewrites `content/` prose that ships into every mandate**, which
`composer-k3` named the likely review-chain candidate. Decide at the end of the
session, per the skill's rule — a `review-impl` leaf cut here would carry the
specific doubt, most usefully whichever of the two prose limbs the guard cannot
reach.

`content/SKILL.md` is still provisioned whole as a harness skill while both
delivery paths are live, so the split must leave the document readable as a
document as well as composable as units.
