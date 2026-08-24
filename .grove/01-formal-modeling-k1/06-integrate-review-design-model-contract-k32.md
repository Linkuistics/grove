# model-contract-k32

**Integrates:** model-contract-k31

## Goal

Correct the semantic catalogue's coverage controls, state/outcome partition and
finish-question decision seams before either model family is written from it.

## Context

`model-contract-k31` reviewed producer commit `4c799827` against the formal-node
brief, `TODO.finish_process.md`, the Experiment 2 pre-registration, the current
ADR set and the current-state architecture. The verdict is **not fit as the sole
input to Alloy and Quint as written**. Apply the six findings below to
`docs/specs/semantic-contract.md`; line references are to the reviewed commit.

### model-contract-k31 B1 — coverage is not independent by family

The node requires independently constructed Alloy and Quint models to cover all
three scopes, but the runner accepts each identifier when a command or declared
gap exists in **at least one** family (`semantic-contract.md:145-149`). One
family can therefore omit a claim entirely while the repository runner remains
green.

Change the coverage unit to `(family, claim)`: every claim must have a property
command plus its required witnesses, or an explicit declared gap, in **each**
family. Apply the reverse check per family too, and record evidence per pair.

### model-contract-k31 B2 — one identifier does not cover several obligations

The runner recognises only the claim prefix (`semantic-contract.md:93-100`),
while many claims contain several independently reachable cases: `TT-02` has
two species mismatches (`:441-444`), `FN-22` has multiple post-handoff paths
(`:701-706`), and `SY-09` has three session endings (`:813-816`). One witness
can make the identifier look covered while the other required cases remain
unmodelled.

Give every separately enumerated property/witness obligation a stable,
machine-checkable sub-identity (or an equivalent checked manifest). Require the
runner to account for every sub-obligation in each family; a freeform mnemonic
alone is insufficient.

### model-contract-k31 B3 — the assumption mutation control is contradictory

The rule says every relaxation must break a claim that names the assumption and
that a no-effect relaxation is decorative (`semantic-contract.md:368-374`), but
the table names no dependent claims. It also mixes three different controls:

- `EN-01` is a premise-breaking mutation;
- `EN-08`/`EN-11`/`EN-16` remove an exercised dimension and should invalidate
  required witnesses; and
- `EN-03`/`EN-05` are stronger counterfactual capabilities intended to permit a
  cheaper protocol, while `EN-15` explicitly says its relaxation should make no
  claim stronger (`:380-392`).

For every `EN-` row, add the controlled claim IDs, classify the control as
premise-break / exercise-removal / counterfactual-capability, and state the
expected result (which exact check or witness fails, or which incumbent claim is
deliberately replaced while shared safety claims remain green). Do not require a
counterfactual capability to falsify the safety property it is meant to retain.

Also make explicit the premises currently smuggled into the three claims named
by the review brief:

- `FN-24` assumes the model's step list is a complete set of crash boundaries
  and that every persistent effect hidden inside one step is atomic or separately
  represented; `EN-01` only grants atomicity to a same-directory rename.
- `TT-21` excludes only a **cooperating** writer. State whether `hand-edit` and
  `foreign-write` may interleave during an operation; advisory locking does not
  exclude them, despite the current witness saying simply “concurrent writer”
  (`:541-544`).
- `SY-13` needs `terminal disposition` defined (including whether a block is
  terminal for this property) and either an explicit fairness/admission premise
  or wording that makes it purely existential reachability (`:832-835`).

### model-contract-k31 B4 — reachable root and ownership states are not sorted

The root table maps every present root with no format witness to `Legacy`
(`semantic-contract.md:259-270`), but `TT-20` and `SY-06` require an interrupted
fresh scaffold to be recognised as a recoverable partial scaffold
(`:535-539`, `:796-800`; current architecture `:895-917`). Add an ordered
`PartialScaffold` state defined by the exact known subset, before `Legacy`, so
legacy work cannot be completed as though Grove owned it.

The table's `Malformed(entry)` covers only a task-shaped entry that breaks the
grammar, while `TT-13` says two individually valid live finish leaves make the
whole tree malformed (`:497-501`). Widen it to `Malformed(reason)` and enumerate
the global invariant failures the claims can reach, beginning with
`MultipleLiveFinish`.

Finally, decide the foreign-reserved-name outcome rather than letting each model
choose. `TT-24` currently permits “a refusal or a block” but its witness says
refused (`:558-561`); `OwnershipConflict` explicitly includes an artifact at a
reserved name (`:338-349`); and `FN-21` says a reaper declines such an entry
without assigning an outcome (`:692-699`). State the exact outcome for an
ordinary tree operation, matching recovery and quarantine reaping, adding a
refusal reason if the pre-transaction case remains a refusal. Keep `FN-25`'s
partition scoped to actual `Blocked` outcomes.

### model-contract-k31 B5 — Q1, Q2 and Q4 are not decidable as mapped

The table promises a keep / delete-replace / defer verdict from the claims
(`semantic-contract.md:176-193`), but three mappings cannot produce it:

- Q1's deciding `FN-19` **requires** quarantine, so a disposal-in-place
  candidate necessarily violates the catalogue even if it preserves `FN-24`.
  Separate strategy-neutral safety properties from incumbent quarantine
  mechanics and require both candidate strategies to be checked against the
  shared properties.
- Q2 says a witness that never lands means delete/replace (`:187`), then says an
  unreached deciding witness means defer (`:191-193`); `FN-25` also requires both
  blocked diagnoses to be reachable on every lane. Map Q2 directly to explicit
  disposition reachability/unreachability checks and state what a bounded proof
  can decide; absence of a witness alone cannot classify deletion.
- Q4's deciding claims constrain unrelated mutation, successful exit and fresh
  root classification, but none classifies which transition exists only to
  protect an intermediate artifact. Require an artifact/transition removal
  mutation matrix naming the first shared safety claim each removal breaks (or
  none); that matrix is the evidence Q4 asks for.

### model-contract-k31 B6 — Q3's replace transition is prose-only

`FN-21` requires resumable disposal and ownership checks, while `FN-22` covers
disposition revalidation around filesystem handoffs. Neither requires a cleanup
marker replace state or transition (`semantic-contract.md:692-706`). The
deliberate-omission row says the transition must exist (`:852-865`), but the
runner only checks claim IDs, so a model can omit it and remain green.

Add the replace transition to the action/state vocabulary and give it its own
claim with a source-state reachability witness, interruption/resumption
obligations and coverage in both families. Map Q3 to that claim.

While correcting `FN-22`, enumerate the disposition changes on both sides of
both handoffs and specify the stable state/outcome after a **successful**
corrective return. The current three cases name post-restore change,
post-quarantine change and failed return, but never say what a successful return
settles to; distinguish at least `Committed -> NotCommitted` (rollback/refusal)
from `Committed -> Indeterminate` (block).

## Done when

- B1–B6 are corrected in the catalogue with checkable claim, witness,
  assumption and outcome wording rather than explanatory prose alone.
- The Q1–Q4 table can yield keep, delete/replace or defer without contradicting
  another row or requiring a candidate to violate the shared safety contract by
  construction.
- Both model families are required to cover every claim and every enumerated
  witness obligation independently, or declare the specific gap.
- `alloy-models-k6` can consume the revised catalogue without inventing a state,
  outcome, fairness premise or decision rule.

## Notes

This is an integration of design-review findings, not model implementation.
Do not write either model here. The reviewed ADR set remains coherent: these are
corrections to an unsettled specification and its test seam, not new settled
trade-offs. Rework an ADR only if the correction changes a decision it already
owns.
