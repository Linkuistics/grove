# specialised-ending-k5

## Goal

Try to disprove the spec amendment `specialised-ending-k2` made to
`docs/specs/mandate-delivered-methodology.md`. Inspection only: read that commit's
diff and the current spec, ADR, `content/` and `CONTEXT.md`. Do not edit
`content/`, do not write the composer, and do not run the build — this leaf
produces findings.

`composer-k3` plans the whole increment against this text, so a defect found here
is cheap and the same defect found after slicing is not.

## Reviews

`specialised-ending-k2` — its commit names that handle. The amendment touches
five places in the spec: the `kinds=` decision, the `content/MANDATE.md`
decision, two new decision sections (*The ending is specialised where the
conditional lives*, *A `finish` session that reopens the grove signals a
relaunch*), a new requirement, a new test-seam bullet, and a new *Out of scope*
entry.

## Context

Six claims the producing session made on judgement rather than on evidence. Each
is stated as the reviewer should try to break it, not as a summary.

1. **The omission/absorption asymmetry.** The spec now argues that a long
   explicit list beats `kinds=!finish` because an *omission* is checkable by a
   universal claim over the closed kind set while an *absorption* is not.
   Attack: is that true, or could a test catch absorption too? If a mechanical
   check for wrongly-inherited guidance exists, the whole argument for the
   eighteen-member list weakens and the ADR's rejection of negation should be
   re-read rather than applied.

2. **"The finish endings must be triggering, not procedural."** The argument is
   that a reopening `finish` session never fetches the teardown procedure, so an
   ending deferred there never reaches it. Attack: does anything else in the
   `finish` mandate already force the fetch? If the session is told to fetch the
   cycle body before deciding *whether* to tear down, the argument dissolves.

3. **The ADR verdict on D4.** The spec records that the reopened-`finish` ending
   fails the when-to-write test on hard-to-reverse and on real-trade-off, and
   clears only surprising. Attack the middle one: is "no signal, the human reruns
   `grove`" genuinely not a live alternative? A reviewer who can name a reader
   who would pick it has found a trade-off the producer denied.

4. **"No confirmation is carried across the reopening."** Verified by reading
   only — the sentinel is never retired, so a fresh `finish` session proposes and
   waits again. Attack it against the code: `src/finish_transaction.rs`,
   `src/finish_cleanup.rs` and the driver's finish path. Is there any state that
   survives a reopening and would let a later session skip the human gate?

5. **The `skill-finish` split is claimed to have *more than one* universal
   fragment** — the negative trigger, and the clause telling every session its
   escalations are discretionary. Attack the count in both directions: is the
   second clause genuinely universal, and are there others (the sentinel's
   "cannot starve nor preempt real work" is a candidate) that the spec's
   three-scope contract silently drops into `kinds=finish`?

6. **The D1 duplicate enumeration.** The spec claims each of the launcher's
   instructions is already delivered by a named `kinds=*` unit —
   `skill-bootstrap`, `skill-decompose`, `skill-retire`, `skill-commit`,
   `skill-signal`, `skill-finish` — and that exactly one clause ("use the grove
   skill") is not. The producer checked this and the list was one unit short on
   the first pass, which is reason to re-check rather than to trust it: read the
   launcher clause by clause against those six units, confirm each target really
   is `kinds=*`, and confirm nothing else in the launcher lacks a home. A
   launcher instruction with no `kinds=*` home is a real hole — reducing the file
   to framing would delete guidance nothing replaces.

## Done when

- Each of the six is confirmed or refuted, with the evidence, and any further
  defect found is reported the same way.
- Every citation in the new spec text is checked against what it cites: the ADR's
  reopen condition on `kinds=`, `CONTEXT.md`'s *Complete finish cycle* and
  *Triggering unit* entries, and `tests/session_kind_guidance.rs` as the named
  precedent and home for the ending guard.
- The new requirement is judged as a *test contract*: are its five scenarios
  actually assertable against the composer seam the spec agrees, or does one of
  them need a surface that does not exist?
- The findings are written where the integrating session can act on them, and
  `integrate-review-design` is cut only if there is something worth acting on.
  A review that finds nothing creates nothing and retires.

## Notes

**Prose is in scope; the design's settled decisions are not.** D1–D4 were taken
in `plan-k1`'s grilling with the human. Refute the *arguments the spec makes for
them* and the *contract it states*, not the decisions themselves; if a decision
looks wrong, that is a finding to raise, not a thing to reverse here.

**The wider scope audit is already externalised** as `unit-scope-audit-k4`. Do
not re-raise it as a finding.
