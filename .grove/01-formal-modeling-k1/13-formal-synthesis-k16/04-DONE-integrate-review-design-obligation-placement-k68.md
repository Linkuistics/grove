# obligation-placement-k68

**Integrates:** obligation-placement-k67

## Goal

Integrate the three findings from `obligation-placement-k67` before
`catalogue-disposition-k64` applies the placement rule to the rest of the claim
catalogue.

## Context

Read the review's `## Findings` verbatim; its `path:line` citations are the
handoff. The producer is `obligation-placement-k63`, and its committed change is
the review baseline.

The findings are coupled at one seam:

1. `TT-24.a` makes the rule ambiguous because *action* spans task-tree, finish,
   lifecycle and environment scopes. If the quantifier is universal, the `TT-`
   placement and its coverage are wrong; if it is prefix-local, Q4-6 cannot use
   task-tree coverage as evidence for a finish reaper mutation.
2. `FN-32` names both the witness slot and cleanup marker, but mutation 63 and
   `mutant_unproven_ownership` do not independently falsify the cleanup-marker
   half, and the Alloy witness pairs the marker with a `Discard` step that cannot
   mutate it.
3. The contested-cell report calls a property-only, no-witness cell “answered,”
   and its new extractor and condition have no durable runner controls.

## Done when

- The ADR's direction/observation/joint rule is mechanically readable from the
  obligation text, including terms such as *action* that span groups. `TT-24.a`
  and Q4-6 are re-decided under the corrected rule, and the row cites evidence
  executed against the action set its mutation changes.
- Each of the original six placements is rechecked after that correction. Any
  changed placement is reconciled across the semantic contract, both model
  families, scope READMEs, Q4 rows and the runner manifest.
- Every artifact `FN-32` names has an independently reached antecedent and an
  isolating falsifier in both families, or the claim is narrowed to exactly what
  the existing controls establish. A witness beside an unrelated framed field
  does not count.
- The contested report distinguishes complete answers from property-only cells,
  preserves the chosen fatal/nonfatal policy without overstating its evidence,
  and `models/run-controls.sh` carries positive and negative controls for
  `control_ob` plus the contested-cell condition.
- All post-fix verification required by the producer's touched scopes and runner
  is rerun against final files and recorded durably. The integration session owns
  every fix and every verification command; the review ran none.

## Notes

This leaf was inserted at the first live sibling after the review so no catalogue
edit can stale its citations. Do not let `catalogue-disposition-k64` absorb these
fixes: it consumes the placement rule and must see the integrated version.

## Decisions (running log)

**All three findings are real, and each is a different kind of real.** They were
verified against the cited artifacts rather than against the review's summary of
them, and they classify as: F1 — **a contract stated unclearly**, with a real
consequence riding on the ambiguity; F2 — **a real issue in the artifact**; F3 —
**a real issue in the instrument**. None is noise, and none demanded that the
producer's decision be rethought: the rule survives, and what it needed was a
fourth clause it was always relying on without saying so.

### F1 — the rule was total for single-owner terms and silent for group-spanning ones

**The counter-instance is exact and it reproduces.** §*Actions* partitions
*action* across five groups — Observation and Tree mutation (`grove-task-tree`),
Finish (`grove-finish`), Lifecycle (the application joint) and Environment. The
ADR's procedure is *look each term up in the Vocabulary, take the highest owning
scope*. For `action` that lookup returns no owner, so the procedure **does not
terminate** on `TT-24.a` and the rule was not checkable there. The landed design
then used both readings at once: the catalogue said `TT-24.a` "reaches both
contexts wherever a model admits them", while `crates/grove-finish/models/README.md`
cited its task-tree coverage as shared-safety evidence for a mutation of the
**quarantine reaper**.

**The fix is clause 4, and it codifies what the corpus already did rather than
inventing a reading.** A group-spanning term is read **prefix-locally**: the
obligation ranges over exactly what its own scope admits, and its text says so.
`SY-14` already wrote *no **admitted** action*, and `models/system/README.md`
already declared "`SY-14.b`'s *every action* is read as every action ON THE
TREE". `TT-24.a` is the one that did not, and it is now in the same idiom. The
consequence clause is the load-bearing half: **a scope may not cite a lower
scope's prefix-local obligation as evidence about an action that scope does not
admit**, because clause 2 already says a citation carries the cited obligation's
narrowings and the prefix-local reading is one.

**The alternative was live and is recorded as rejected.** Read the quantifier
universally and clause 1 moves `TT-24.a` to `SY-`, because the partition reaches
the Lifecycle group. That is worse on cost and on truth: the lifecycle model
would owe fail-closed ownership over an action set it abstracts, the task-tree
column — the only model that can actually attempt a mutation against a foreign
entry — would carry nothing, and the resulting obligation would state something
**no command in the repository checks**, since no single model admits the whole
partition.

**Q4-6 is re-decided to `none`, and the evidence is the mutation the row already
had.** Row x1 strips `reapable` to *there is a quarantine* and kills `FN-21.b`
and `FN-21.c` — both incumbent mechanics — leaving all 26 neighbours green,
`FN-27.a` – `FN-27.c` included. That measurement **was** executed against the
action set it changes; what was wrong was reading past it to an obligation
checked over a different one. `FN-32` is no rescue: `Reap` is excluded from its
antecedent by design, precisely so a sweep mutation cannot kill it. So the row
has no shared-safety obligation and `none` is the honest answer — and Q4's own
text says a `none` **is** an answer.

**That is a finding for `finish-verdicts-k65` rather than a defect here, and it
is the sharpest thing this leaf produced. No shared-safety obligation in this
repository is stated over the quarantine reaper's actions.** `TT-24.a` covers the
task-tree scope's admitted set, `FN-32` covers a live transaction's steps, and
the reaper is `FN-21.b`/`FN-21.c`, both incumbent mechanics. The cleanup marker
therefore joins the quarantine (`Q4-5`) and the replace transition (`Q4-7`) as an
artifact this family's evidence says protects Grove's own machinery and not the
user — `TODO.finish_process.md` Q4's delete/replace criterion met a second and
third time. Three of ten Alloy rows now read `none`. Whether that licenses
removal is Q1's and Q4's question, and this leaf does not answer it.

**The other five placements are unchanged, and each was rechecked rather than
assumed.** Instances 1 and 2 (`TT-24.c` → `FN-32`, `TT-24.d` → `FN-21.c`) name
specific finish contexts, so clause 1 places them and clause 4 does not reach
them. Instances 4, 5 and 6 (`SY-06.b`'s ordering, `SY-05.b`'s other half,
`SY-14`'s operator exit) contain no group-spanning quantifier; `SY-06.b`'s
citation carries `TT-20`'s declared narrowing in its own text, verified at the
source. **The manifest is unchanged and that is measured, not asserted**:
`models/run.sh --list` before and after the catalogue edits is byte-identical at
128 obligations, so nothing here opened a `(family, obligation)` cell.

**Every other group-spanning quantifier in the catalogue was swept and none
contradicts the rule.** `TT-04`, `TT-09`, `TT-25` and `SY-14` each quantify over
*action* within their own scope's prose and none claims cross-scope reach.
`TT-24.a`'s paragraph was the only one that did, which is why it was the only one
edited. Nothing is left for `catalogue-disposition-k64` on this axis.

### F2 — `FN-32` named two artifacts and was controlled on one

**The hole is real and both families had it, in different shapes.** Alloy:
mutation 63 removes `slotSame` from `doDiscard`, so it kills the first conjunct
only, and `witness_FN_32` put both artifacts beside a `Discard` — whose
`markSame` is **unconditional**, so the marker stood there because no step in the
trace could move it. Quint was worse than the review knew: `inv_FN_32` read the
aggregate `hist.mutatedUnproven` and was therefore **character-for-character
`inv_FN_10b`**, and it additionally swept the quarantine name that the
catalogue's `FN-32` explicitly excludes. Its control flips one global
`OWNERSHIP_PROVEN` dial, and that module's environment
(`ENV_PHASES = Set(0, 1, 2)`, `ENV_FOREIGN = Set(0)`) **never reaches a foreign
cleanup marker at all** — so its recorded kill was always the slot's.

**Narrowing the claim to the slot was the review's own sanctioned alternative and
it was rejected, on a fact the review could not have seen.** In Quint, `FN-32`
narrowed to the slot is `inv_FN_10b` exactly — the claim would collapse into the
incumbent-mechanics obligation it was created to escape, which is the whole of
its reason to exist. So the claim stands and the evidence was brought up to it.

**Four repairs, and the second is a measurement rather than a fix.**

1. `inv_FN_32` now reads `not(mutatedUnprovenSlot) and not(mutatedUnprovenMarker)`
   — the catalogue's two artifacts and not the aggregate. `noteUnprovenSlot` and
   `noteUnprovenMarker` set those beside the existing flags, so nothing else
   moved.
2. `mutant_unproven_marker` is `scenario_foreign_marker` — the one environment
   that demonstrably reaches a foreign marker — with `OWNERSHIP_PROVEN = false`,
   and `inv_fail_MUT_FN_32_a_transaction_mutates_a_marker_it_cannot_prove` is
   **violated** in it, verified by running it. Had the marker half been vacuous
   the control would have HELD, and an `inv_fail_` that holds turns the runner
   red. That is the experiment, and it came back saying the gate is real.
   `mutant_unproven_ownership` keeps the slot's, now stated over the slot flag
   so it cannot borrow the marker's.
3. Alloy gains `witness_FN_32_…_meets_an_unprovable_marker_and_it_stands` at a
   `MarkerReplace`, the only `groveActs - Reap` member whose marker mutation is
   ownership-gated; Quint gains `wit_FN_32_…_an_unprovable_marker` in
   `scenario_foreign_marker`, witnessed in **1659 of 8000 traces (20.74%)**.
4. **Mutation row 64**: `doMarkerReplace`'s foreign branch stops framing the
   marker — still blocks, still carries `W17OwnershipConflict`, and supersedes
   the document on its way out. **KILLED `FN-32` and `FN-31.d`, left green all 60
   others**, swept over every `check` in the file rather than a named
   twenty-six, `FN-10.b`, `FN-21.b`, `FN-21.c` and `FN-27.a` – `FN-27.c`
   included. With row 63 that is the pair: 63 kills through the slot, 64 through
   the marker.

**The honest residue is recorded rather than papered over.** The only reachable
site where a non-`Reap` step reads the marker's ownership bit is the replacement,
which is `FN-31.d`'s — so `FN-32`'s marker half and `FN-31.d`'s second conjunct
have the same content at the same place, which is why one mutation kills both.
The difference is entirely the **class**. That was always the argument for
creating `FN-32`; it is now also, for this one artifact, the whole of it.

### F3 — the report that exists to expose false confidence was making a false statement

**Confirmed at the source.** The coverage matrix calls a cell complete only with
`covered_prop` **and** `covered_wit`, and prints `NO-WITNESS` otherwise
(`models/run.sh`); the contested block credited `covered_prop` alone and printed
*answered*. The line now reads *answered WITH A PROPERTY ONLY, no witness* and
the summary counts them. **The reported-never-fatal policy is preserved**, and
the ADR's *What enforces it* now states what the line can and cannot say, so the
policy is not resting on an overstatement.

**Three durable controls, and they are the first in `models/run-controls.sh` to
assert a LINE rather than a fatal exit — deliberately, and the file says so.**
A report that can never go red still has to be shown to fire and to fire
correctly; a report whose evidence sentence is false is worse than none, because
it is read as the counterweight.

- `contested-property-only` — `TT-19`'s witness deleted, Alloy declaring the gap.
- `contested-control-seen` — `TT-21.a`, which `inv_fail_MUT_TT_21a_…` names.
- `contested-control-unseen` — `TT-19` intact, which no control names.

The last two are `control_ob`'s positive and negative pair: without the negative
half an extractor that matched everything would pass the positive one and report
every cell controlled.

**They were shown to fail before they were trusted.** `contested-property-only`
was run against the **pre-fix** `models/run.sh` — restored from the parent commit
into the working tree for one run and put back — and it FAILED, pattern not
found. The same control passes against the fixed runner. A control that has never
been seen to fail is not a control, which is this corpus's own rule one level up.

**Two families were needed and neither a real Alloy run nor a real solver was.**
The contested block only fires with `${#families[@]} -gt 1`, so the control copy
carries an Alloy family: a one-command stand-in `.als` and a shimmed `java`, in
the same idiom as control 5's shimmed `quint`. Every input to the reporting logic
— which family gapped, which answered, with what kind of answer, whether a
control names the obligation — is read from the catalogue, the scope README and
the command names in the model files. None of it comes from a solver. A real
`task-tree.als` would have cost about two hours per control and changed no line
of the report.

### One process failure, recorded because it destroyed a measurement

**Two mutation-64 sweeps ran concurrently against one log file.** The first was
launched with a trailing `&` inside a tool invocation, reported as complete when
the invocation returned, and kept running; the second truncated and re-wrote the
same log beneath it. The result was a log that had shown `KILLED FN_31d` and then
did not — a reading that was not merely incomplete but self-contradictory. Both
were killed and the sweep re-run once, alone, from an empty log.

It is the same lesson the producer paid two hours for, arriving from the other
side: **an instrument you adjust mid-reading has not read anything**, and a
second copy of the instrument writing into the first one's output is an
adjustment. The rule that would have prevented it: one measurement, one writer,
and never infer that a background job is finished from the return of the thing
that launched it.

**The whole-repository run was also stopped two minutes in and relaunched**, for
one remaining comment edit to `models/run.sh`. A comment could not have changed
the result, and that is exactly why it was cheaper to restart than to record a
run against a file that had since changed.

### Not written to `docs/formalism-findings.md`, and the reason is the freeze

The numbered series ends at 048, and `experiment-synthesis-k62` froze it
deliberately: Experiment 2 compares **the two columns as independently built**,
and every disposition child after it edits the catalogue and cascades commands
into both families. An entry 049 written by a disposition child would report
figures for a third set of models no independence protocol governed. The durable
homes for what this leaf found are the ones the synthesis already cites — the
ADR, the catalogue, the two model READMEs, the mutation matrix and
`models/run-controls.sh` — and every finding above is landed in one of them.

### The verification, re-run by this session against final files rather than quoted

Every file the runner reads had reached its final state before this was
launched — which is the rule the producer's two invalidated hours bought, and
which cost this session one two-minute restart to keep exactly.

| run | result |
|---|---|
| `models/run.sh --list` | **128 obligations**, byte-identical to the pre-edit capture |
| `models/run.sh` (whole repository, both families, coverage asserted) | **exit 0** — 794 commands, `256 complete, 0 declared gaps, 0 empty, of 256`, Q4 matrix alloy 10 of 10 rows / **3 `none`** / 1 abstracted, quint 10 of 10 / 3 `none` / 1 abstracted. 2h 00m 35s wall / 11931s CPU on a 16-core host |
| `models/run-controls.sh` | **10 passed, 0 failed** — the seven earlier controls and the three new ones |
| mutation 64, all 62 `check` commands in `finish.als` | KILLED `FN-32` and `FN-31.d`; **60 green** |
| `mutant_unproven_marker` / `inv_fail_MUT_FN_32_…_marker_…` | **violated**, as an `inv_fail_` must be |
| `contested-property-only` against the **pre-fix** `models/run.sh` | **FAILED**, pattern not found — the control fires |

**Exit 0 established by enumeration rather than by the absence of the word FAIL.**
The runner's distinctive fatal diagnostics were searched for together —
`placement error:`, `runner error:`, `MISSING SCOPE`, `NO ROW`, *rows that do not
resolve*, *commands naming no obligation*, *declared gaps in BOTH families*,
*not reported by the run*, *failed to run*, *failed to complete*, *refusing to
guess*, and a `FAIL ` line — for **zero hits**, and the pattern was controlled
against a synthetic three-line input, which it matched three times. Both
delegated `docs/ordinal-fs-tree/models/` runners ran. The contested-cell section
printed nothing, which is what zero contested cells looks like.

**Three numbers moved against `experiment-synthesis-k62`'s frozen run and each
has a reason.** 258 cells → **256**, and the two declared gaps with them, because
`TT-24.c`/`TT-24.d` are retired. Alloy's Q4 column 2 `none` → **3**, which is
`Q4-6`. Commands 791 → **794**: an Alloy and a Quint witness for `FN-32`'s marker
half and the Quint control that isolates it, against one control replaced in
place. The frozen run is left exactly as it was measured; this one is recorded
beside it in `models/README.md`.

## What this leaf hands forward

- **`finish-verdicts-k65`** — `Q4-6` now reads `none`, so **three** of the ten
  Alloy rows do (the quarantine, the cleanup marker, the replace transition), and
  **no shared-safety obligation in this repository is stated over the quarantine
  reaper's actions**. That is Q4's delete/replace criterion met twice more and a
  declared limit of the catalogue, not a licence to remove anything.
- **`catalogue-disposition-k64`** — the placement rule it inherits now has four
  clauses, and the fourth is the one that decides any obligation quantified over
  a group-spanning term. Every such quantifier in the catalogue was swept and
  only `TT-24.a`'s prose contradicted the rule, so nothing is left on this axis.
  `TT-24.c`'s outcome question — Alloy refuses, Quint blocks, both green against
  `FN-10.b` — is still this child's, unchanged by anything here.
- **`handoff-audit-k66`** — the durable artifacts are the ADR, the catalogue's
  `TT-24`/`FN-32`/§Actions text, the two model READMEs, mutation row 64 and
  `models/run-controls.sh` controls 8 – 10. Nothing was left only in this file.
