# plan-k2

**Reviews:** plan-k1

## Goal

An adversarial read of the requirements `plan-k1` settled, before a design is
built on them: the root brief's *Settled requirements*, *Done when* and *Out of
scope*, against `plan-k1`'s decision log and the code and documents they
describe.

## Context

- **Report only what would change what gets built or how it is tested.** The
  human's rule for this grove is simplicity, and it covers this review: no
  wording polish, and no finding whose only remedy is more words in the brief.
- **Every decision in the log is the human's**, several against the producing
  session's recommendation and several deliberately at the level of a solution.
  A finding does not overrule one, and *a solution stated as a requirement* is
  not a defect here. What a finding can show: two requirements that cannot both
  hold, a fact about today's system stated wrongly, a consequence that changes
  the work and that nobody recorded, or a decision the brief dropped or changed
  on its way out of the log.
- **Where an early log entry and a later one differ, the later binds.**
  `plan-k1`'s *Notes* names the three that were refined.
- **The doubts the producing session could not close for itself:**
  1. *Requirement 5 removes the catalog, and the shipped review rule leans on
     one.* The review examples check a creator's origin against the catalog's
     provider values and name a reviewer by candidate ID. Does the rule still
     work when the only tool-level fact is the label in the run record?
  2. *Requirement 10 while dispatch still `exec`s.* Dispatch reads the policy,
     runs its worker and commits a run record before the handoff, outside the
     sandbox. Is anything left that a confined harness then cannot reach, and
     can a caller-neutral capability carry both Seatbelt and bubblewrap?
  3. *Requirement 11's parity.* Is it well defined for every kind in the pinned
     file, including the standalone `release-notes` route and arrangements that
     compose several profiles?
  4. *Statements of present fact.* The log and the brief say what today's
     system does in many places. Check the ones a requirement rests on.

## Done when

- Each doubt above carries a ruling.
- Findings are recorded as this kind's skill directs, each naming the
  requirement or log entry it is against and what it rests on.
- If a finding is worth acting on, an `integrate-review-requirements` leaf is
  cut where the walk reaches it before the design leaf.

## Notes

`plan-k1` was launched directly, not through harness-dispatch, so it had no run
to name and this leaf carries no `**Creator:**` line.
