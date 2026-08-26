# system-k13


## Goal

Model Grove's end-to-end lifecycle in Quint at component boundaries.



## Context

Compose task-tree observations with finish/recovery observations in `models/system/`. Build before reading Alloy's system model and avoid duplicating component internals.

## Done when

- State/actions connect confirmation intent, session completion, selectable work, exhaustion, finish entry, interruption, restart, blocked recovery, ownership conflict, preserve/merge success, and root absence.
- Explicit refusals prevent early finish and ordinary work during correlated recovery; interruption remains possible at every transient boundary.
- Invariants and temporal/scenario checks cover safe absence, stable terminal outcomes, recovery availability under stated assumptions, and absence of unowned mutation.
- Tests, simulations, seeds, trace limits, backend limitations, and witnesses for normal and hostile paths are documented and run by the common model command.
- Cross-component findings are appended to Experiment 2.

## Notes

Do not claim liveness from a few successful simulations. State exactly which temporal claim, scheduler/fairness assumption, and finite limit were checked.

## Decisions (running log)

**The model composes the two components' OBSERVATIONS and re-derives neither.**
The leaf's own instruction is to avoid duplicating component internals, and both
sibling models already own their subjects. So the task tree is a ten-field
summary with `TT-18`'s classification order over it, and the finish transaction
is a five-phase cursor plus the two facts the lifecycle actually reads off it —
whether the deletion is proven and whether the ticket is persistent. No
positions, no keys, no walk, no manifest, no quarantine, no lanes; `SY-` claims
quantify over none of them. `SY-05` is the justification made visible: `FN-11`
and `FN-19` are checked next door, and `SY-05.b` is the claim that they COMPOSE
into a sound inference, with a dial (`DELETION_PROVEN_FIRST`) that kills both
obligations together.

**The search dial for this scope is the determinism of the subject, not a
budget.** `task-tree-k11` narrowed the action menu with `scenario_` instances and
`finish-k12` capped an environment budget over a twenty-step transaction. Neither
shape fits a DRIVER LOOP, which is deterministic by construction: given a lease,
a validated configuration, an open epoch and a classified root, exactly one move
is next. Modelling it as a uniform choice over its whole action vocabulary does
not model a driver — it models a random walk that shares the vocabulary.
Measured at 2000 samples: a flat 27-disjunct menu landed **5 of 25 witnesses**
while all 25 properties reported green; `driverStep`, written as the chain of
conditionals a loop is, lands **23 of 25**, and the two that do not have
scenarios. Nothing was removed from the model. The world's share is capped three
ways instead — `ENV_BUDGET`, `ENV_KINDS`, `ENV_PHASES` — and `base` grants all
three wide.

**Eighteen model mutations, and the count is the observation rather than an
accident.** `task-tree-k11` needed two and `finish-k12` eleven. A deterministic
loop satisfies almost every ordering claim it is written to satisfy: "the layout
is proved at lease acquisition", "validation precedes every transition", "at most
one transition per iteration", "selection is not recomputed" are each true of
`driverStep` *because `driverStep` is written in that order*, and each would
carry a green tick over nothing. The honest count is lower than eighteen — four
obligations die only inside a bundle control, and the README says which rather
than letting a reader read them as separately evidenced.

**`SY-13.a` is existential reachability, and an executable model cannot search
for it.** "There EXISTS a bounded sequence of admitted actions reaching…" is not
an invariant. The instrument is two halves worthless apart: a DECLARED escape map
over the closed stable-class set, iterated exhaustively to the bound, and a
CORROBORATION witness that the model's own transitions walked the longest
declared chain edge by edge. The map alone is fiction with a green tick; the walk
alone is one path. Recorded as observation B in entry 046 because it generalises
to any `SHALL exist` claim the catalogue carries.

**Catalogue defects are recorded, never fixed.** Five were found (entry 046). The
independence barrier freezes `docs/specs/semantic-contract.md` and both sibling
columns recorded rather than fixed their own, so this one does the same. Three
obligations are consequently checked over less than their literal text —
`SY-13`'s sweep, `SY-14`'s quantifier, `SY-04.a`'s cap — and each gap is declared
in the model, in the README and in the entry rather than in the model alone.
`mutant_literal_sy13` goes further and *fires* the first of the three, which is
what turns "the catalogue is wrong here" from a remark into a control.

**The independence disclosure is sharper than `finish-k12`'s and is recorded as
such.** Locating the next free entry number was done with a heading grep, which
printed the titles of the Alloy column's four LIFECYCLE entries (040 – 043) —
entries about this very scope. Findings 1, 2 and 5 are in the model's text from
before that grep; findings 3 and 4 were reached after it. Entry 046 states which
is which and asks `cross-model-replay-k15` to treat 3 and 4 as contaminated
overlap candidates rather than clean `quint-only` ones.

**A `review-prototype` chain was cut, and cut by this session rather than by a
human.** `task-tree-k11` and `finish-k12` each put the same question to a human
and each chose review; this session ran unattended, so it made the call on the
evidence and records that it did. `cross-model-replay-k15` already reads this
model adversarially and re-derives every finding, so a review that only re-read
the claims would duplicate it. **What replay will not read is the instrument**,
and this column's instrument carries a hazard neither sibling's did:
`driverStep` is a search narrowing that lives in `base` itself, not in a focused
instance, so every one of the 25 properties is checked only over orderings the
driver's own conditional chain admits. A branch omitted there is invisible from
the runner and indistinguishable from a green. Beside it sit the three
narrowings, where "the catalogue is wrong" and "I narrowed until it passed"
produce identical output; the four bundle-only obligations; and two witnesses
landing in 16 and 18 traces of 8000. Appended rather than inserted, because
`prototype-system-k13` is the last entry in `quint-models-k10` and no sibling
entry after it holds live work — so `pick` reaches the review next either way.
