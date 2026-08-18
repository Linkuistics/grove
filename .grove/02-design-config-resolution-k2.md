# config-resolution-k2

## Goal

Make the recorded design say what the six settled requirements decided: that
configuration resolves from the personal file plus **at most one** `.grove.kdl`
delta, found at the worktree root or failing that the repository root. Rework
`docs/adr/complete-session-configuration.md` in place and reconcile every
current-state claim the change falsifies. No code this session.

## Context

The brief carries the six requirements and the file pointers; this section
carries only what is specific to reworking *this* ADR.

`complete-session-configuration` is unusually hostile to what we are about to do,
and that is a feature — it means the rework has to be argued rather than
asserted. Two of its rejected options are close neighbours of this change:

- *"Keep a primary harness and layer kind/family overrides over it"* — rejected
  because the result depends on **a precedence lattice** and still asks grove to
  understand harness flags. Requirement 6 is the answer: search order is not
  merge order, the two candidate paths are never merged with each other, so
  resolution is two deep and flat. Grove still understands nothing about the
  template it selects. Say this explicitly; a reader who skims will assume the
  lattice came back.
- *"Store launch policy in each task leaf"* — rejected because task trees should
  describe work and **remain portable**. Requirement 1 is the answer: the delta
  is uncommitted, so it is not part of any portable artifact and a clone carries
  none of it. This is also the whole security story — an untrusted repository
  cannot dictate which executable grove spawns, because a repository cannot ship
  a `.grove.kdl` at all.

The option that has to be revised rather than answered is *"Provide defaults,
families, inheritance, or profiles inside KDL"*, rejected because
*"deduplication makes one kind's target partial and allows a new kind to inherit
policy its owner never reviewed."* Half of that objection stands and half is
retired, and the ADR must be precise about which:

- **Retired**: the inheritance hazard. The personal file stays mandatorily
  complete, so a newly added kind resolves to a target its owner had to write by
  hand or fail validation. It can never inherit from a delta that does not
  mention it.
- **Stands**: partial targets are still forbidden *within* a kind. A delta
  overrides a kind's whole template or not at all; there is no merging of words,
  flags or fragments, and no way to say "same command but a different model".

The invariant to state as the ADR's load-bearing claim is the one the brief
names: any one kind's effective command is a single complete template string read
whole from a single file. `complete-session-configuration` was defending that,
not the file count, and the rework is coherent precisely because that claim
survives intact.

## Done when

- `docs/adr/complete-session-configuration.md` describes the design as it then
  stands — resolution order, the two-deep bound, fail-closed on an invalid delta,
  and the completeness rule that still binds the personal file. Reworked **in
  place**: edit the binding statement and the affected *Considered options*
  entries, including their reopen conditions. **No superseding ADR is appended**
  — the VCS holds the history.
- The ADR set is still a minimum coherent set. If the rework leaves
  `complete-session-configuration` carrying two separable decisions, splitting is
  legitimate; adding a second overlapping record is not.
- Every citation the rework leaves dangling is fixed — in `docs/`, in
  `content/`, in other ADRs, and in `.grove/BRIEF.md` if it now misdescribes the
  record.
- `content/references/driver.md`'s §"What the one configuration carries" no
  longer claims that **no repository stamp** routes a session, since a
  `.grove.kdl` at a repository or worktree root is exactly that. Re-derive the
  current wording from this checkout before editing — see the brief's Notes on
  the three disagreeing corpora.
- `cargo test` is green, `tests/rule_ownership.rs` and
  `tests/loaded_path_budgets.rs` included. A corpus edit can trip the per-kind
  word budgets; if it does, that is a real signal about the edit's size, not an
  obstacle to route around by raising a budget.
- The last act of this session is to decide whether the reworked ADR warrants a
  `review-design` leaf. It is a decision the project builds on for months, which
  is the case `decompose.md` says to decide *for* — so argue yourself out of it
  rather than into it, and if you cut it, write the specific doubt into its body.

## Notes

Do not restate the six requirements in the ADR as requirements. An ADR records a
decision and why it binds; the brief records what the human settled. Reproducing
one inside the other creates two sources of truth that drift.

`docs/CONFIGURATION.md` and `docs/ARCHITECTURE.md` are **not** this leaf's to
finish — they describe behaviour for users and land with the code in
`local-config-kdl-k3`. Touch them here only where a *citation* of the reworked
ADR has gone stale. Resist the pull to document the feature early; documentation
that describes unbuilt behaviour is the same lie as a glossary entry for it.
