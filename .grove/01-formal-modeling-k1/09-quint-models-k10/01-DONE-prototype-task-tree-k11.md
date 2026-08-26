# task-tree-k11


## Goal

Model Grove task-tree actions and refusals in Quint against the shared task-tree claims.



## Context

The model belongs with `grove-task-tree`; generic ordinal ordering is assumed through the `ordinal-fs-tree` contract. Build without reading Alloy's task-tree model.

## Done when

- Typed state and actions cover current-format initialization, selection, decomposition, insertion/addition, retirement, invalid/foreign roots, opaque entries, and terminality.
- Refused operations leave protected state unchanged and return a classified observation; they are exercised rather than filtered out of traces.
- Invariants cover identity/name/format validity, sibling order assumptions, stable selection, unrelated-entry preservation, legal mutation, and correct exhaustion.
- Tests and simulation witnesses cover each transition family; verification commands, seeds, trace limits, abstractions, and ordinal assumptions are documented.
- Material Quint-specific catches, misses, costs, and derived tests are appended to Experiment 2.

## Notes

Do not grow a second filesystem model. If a Grove claim cannot be expressed above the ordinal seam, report that seam problem explicitly.

## Decisions (running log)

**The model sits above the ordinal seam and states `TT-09` as the seam itself.**
The leaf's own instruction is not to grow a second filesystem model, and
`docs/ordinal-fs-tree/models/operations.qnt` already checks the ordered-tree
algebra. So append, insert, promotion and rewrite are *assumed* here, and what
the model adds is grove's own vocabulary on top — session kinds, outcome
infixes, the format witness, reserved witnesses, foreign entries, selection,
fail-closed ownership. `TT-09` is then not a restatement of the library's
properties but the claim that every mutation grove makes is one of the four plus
a domain precondition, which is checkable over the effects an operation applied.
No seam problem to report: every `TT-` claim was expressible above the boundary.

**Every action is total, and refusals are transitions.** Quint's own idiom is a
guarded action that simply does not fire, which makes a refusal an *absent*
transition and every refusal claim in the catalogue true by construction. Each
action here computes a `Decision` from its snapshot and transitions in every
case. This is the same choice entry 003 recorded for the library, reached again
independently for a different subject, and it is what the whole `TT-24`/`TT-19`/
`TT-15` group rests on.

**`TT-21` gets a model-mutation dial, because it is otherwise vacuous.** An
executable model classifies from `op.snap` because that is how one writes it, so
"every classification comes from one listing" holds by construction and the green
tick means nothing. `ONE_SNAPSHOT` plus the `mutant_two_listings` instance makes
later steps classify from the live tree and the claim dies. Recorded as a
decision rather than a detail because it generalises: the pre-registration's
*vacuous invariant* hazard has an executable-model-specific form — not "the
antecedent is never reached" but "the model cannot express the defect" — and the
control for it is a dial on the model, not on the world.

**A guard wait is modelled as membership of `pend`, not as an outcome.** The
catalogue anticipates that a model needing the waiting state observable will add
an abstraction to the closed outcome set, and names `Deferred` as the Alloy
column's. This model does not need one: a caller that cannot take the guard
produces no outcome and joins `pend`, which is a real transition (so `TT-22`
stays falsifiable) and is also truer to the catalogue's own position that
grove's tree lock blocks and a wait is not a return. `Deferred` was written into
the outcome type first and then removed.

**`hand-edit` installs one of an enumerated family of 25 well-formed trees.**
Composing single-object edits is faithful and useless: the situations the claims
are about sit four or five specific edits deep, and simulation reaches them at a
rate that makes the suite flaky rather than slow. `EN-11` grants that *any*
well-formed tree is reachable by hand edit, so this is a search strategy over
exactly the granted space, not a second assumption — and the test that it is
faithful in the direction that matters is that `relax_EN_11` removes the whole
family with it, and its controls still behave.

**Four `scenario_` instances exist for sampling and nothing else.** `TT-07`,
`TT-08`, `TT-09.c` and `TT-04`'s renumbering witness land in 0.03 – 0.08% of
unfocused traces, which at any affordable budget is flaky. Each gets an instance
narrowing the action menu onto its own situation. The runner's rule — an
instance prefixed `relax_`, `mutant_` or `scenario_` carries only its own
commands, every other instance inherits the library's — is what keeps that from
weakening `base`, where every claim is still checked unfocused.

**Catalogue defects are recorded, never fixed.** Five were found (entry 044).
The independence barrier freezes `docs/specs/semantic-contract.md`, and the
Alloy column recorded rather than fixed its own, so this column does the same:
each narrowing is declared in the model, in the README and in the entry, and
`formal-synthesis-k16` owns the disposition. Two obligations — `TT-17` and
`TT-20` — are consequently checked over less than their literal text, with the
gap stated in all three places rather than in the model alone.

**The Quint driver's module rule is derived from the file, not transcribed.**
`models/run.sh` needed to know which module each command runs in. A module
carrying `const` declarations is a library and is not runnable; every other
module is an instance; a command runs in the module it is textually defined in.
That keeps the catalogue-is-the-manifest principle intact one level down — the
runner reads the model rather than a hand-maintained list beside it — and the
three control prefixes are the only convention added.

**Model checking is pursued to a measurement, not abandoned at the first
error.** The first attempt failed with `RangeError: Invalid string length` in
quint 0.32.0's Apalache result reporter, which reads as "Quint cannot check this
subject" and is not what it is. Put to the human, who chose to split the model
until the reporter survived rather than accept the limit — which was right:
moving the controls into `task-tree-controls.qnt` removes the crash entirely and
Apalache runs. Two further barriers were behind it. `gapless` spelled
`ps == 1.to(n)` — a non-constant integer range Apalache refuses outright — and
is now a cardinality plus a bound, which is equivalent and checkable. Past both,
model checking is **reachable and not affordable**: `base` exhausts a 4 GB heap
at depth 3, and a deliberately tiny `verify_small` does not finish depth 3 in 25
minutes with a 24 GB heap, because every transition quantifies over `reached` —
an unrolled fixpoint a simulator skips and a symbolic backend encodes in full.
Both barriers removed are durable artifacts that survive the negative result.

**The `verify_` prefix is a fourth instance class, and verification is off by
default.** A module named `verify_<x>` inherits the library's PROPERTY commands
only — a witness is a reachability question and a reduced world is the wrong
place to ask one — and is model-checked rather than simulated. `QUINT_VERIFY=0`
is the default because of the measurement above, and the runner then prints a
`SKIP` line per property. It also reads a `- **VERIFY** quint (…)` line out of
the scope README, prints it on every run, and *fails* a scope whose Quint models
exist and which declares nothing — so a limit on model checking names itself
instead of passing as silence. An out-of-heap, a backend that never started and
a crashed reporter abort the run as a dead tool rather than being recorded as
verdicts, which is the runner's first obligation applied to a second tool.

**A `review-prototype` chain was cut, and inserted ahead of the sibling model
leaves rather than appended.** `cross-model-replay-k15` already reads this model
adversarially and re-derives every finding, so the model has a scheduled read
and a review leaf for it alone would duplicate that. Two things replay will not
read: the Quint driver in `models/run.sh`, which is not a model and which
`prototype-finish-k12` and `prototype-system-k13` both inherit — a defect in it
makes three columns green on nothing; and the three narrowings (`TT-17`,
`TT-20`, `TT-15a`) that let this model reach green, where "the catalogue is
wrong" and "I narrowed until it passed" produce identical output and the
producing session is the wrong context to tell them apart. Put to the human, who
chose it over leaving both to replay and over a narrower review of the runner
alone. Inserted rather than appended for the same timing reason
`model-contract-k5`'s review was: appending would put the review after the two
leaves that inherit the instrument under review.

