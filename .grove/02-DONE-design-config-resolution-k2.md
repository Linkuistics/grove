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

## Decisions (running log)

**The ADR is reworked in place under its existing slug, and not split.** The
rework leaves `complete-session-configuration` carrying what looks like two
decisions — the template is one opaque complete string Grove executes directly,
and that string resolves from at most two files — but they are one claim seen
from two sides: the resolution rule is safe *because* every kind's command is
still read whole from one file, and the completeness rule that makes a partial
delta safe is the same completeness the record already defended. Splitting would
produce two records neither of which can be understood without the other, which
is what `linkuistics:decision-records` calls one ADR pretending to be two. The
environment paragraph stays for the same reason it was already there: it is a
consequence of opacity, not a separate call. Keeping the slug also keeps every
citation valid — `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and
`one-build-owns-a-session` (twice) all cite by slug, and all four citations were
re-read and still hold, since what they lean on is opacity and the
whole-template rule rather than the file count.

**`content/references/driver.md` is edited subtractively, and describes no
`.grove.kdl`.** Two claims in §"What the one configuration carries" are
falsified on different clocks. *"No repository stamp"* routes a session is a
claim about the design's shape, so the reworked ADR falsifies it now; it is
deleted. *"Every session is launched by `~/.config/grove/config.kdl`"* is a
claim about behaviour and stays true until the code lands, so it is not replaced
by a description of the delta — it is weakened to "launched from personal
configuration — it lives at `~/.config/grove/config.kdl`", which is true both
before and after `local-config-kdl-k3` and states nothing unbuilt. The canonical
phrase **Nothing else routes a session** is untouched, as
`tests/rule_ownership.rs` requires of the `one-configuration` row.

**"No default, family or fallback" becomes "…or inheritance."** The clause was
rescued by its neighbour in the sense `references/execute.md` warns about: with
*repository stamp* removed, *fallback* is the word left carrying a falsehood,
because requirement 6's "worktree root, failing that the repository root" is
literally a fallback in search order and a kind the delta omits literally falls
back to the personal file. *Inheritance* is the word the ADR and
`docs/ARCHITECTURE.md` already use for the thing actually forbidden — deriving
one kind's target from another's — and it stays true after the delta lands.

**Two reconciliations nobody owned are handed to `local-config-kdl-k3` through
the brief.** `CONTEXT.md`'s **Grove configuration** entry ("the sole source",
"repository-local stamps … neither override nor supplement this file",
`_Avoid_: a fallback chain`, `_Avoid_: … user policy has one home`) and
`content/references/decompose.md`'s "a property of two entries in
`~/.config/grove/config.kdl`" are both falsified when the behaviour lands, not
now — so neither is this leaf's to write. But `k3`'s own `Done when` named only
`docs/CONFIGURATION.md` and `docs/ARCHITECTURE.md`, and the brief mentioned
`CONTEXT.md` only as a term to *add*, which understates the larger half. Both
are now named in the brief's `Done when` and `Pointers`, where the session that
lands the behaviour will read them. Nothing was written into `k3`'s body: a
sibling leaf's body belongs to the session that cut it.

**A `review-design` leaf is cut, and inserted ahead of the implementation.** The
leaf asked to be argued out of it, and the argument that came closest was that
the six requirements were already settled with a human and the task file
pre-argued three of the four revised options, leaving only prose quality at
risk — which `local-config-kdl-k3` would notice anyway, since its own body names
this record as its contract. That fails on who is reading and why: a session
implementing to a contract is the reader least likely to notice that the
contract's *rationale* is incoherent, because an incoherent rationale blocks no
code. And what is most at risk here is exactly the rationale — whether the
record honestly distinguishes what was rejected from what was adopted, which the
leaf was explicitly warned a skimming reader will get wrong. Two further calls
were made without a second reader and are the project's for months: not
splitting the record, and taking *Considered options* from six entries to ten.
`leaf-insert` rather than `leaf-add` because a review appended after
`local-config-kdl-k3` would arrive once code already depended on the record —
the expensive order the brief exists to avoid. No `integrate-review-design` leaf
is cut: a review that finds nothing creates nothing. No in-session reviewer was
spent, here or earlier in the session.
