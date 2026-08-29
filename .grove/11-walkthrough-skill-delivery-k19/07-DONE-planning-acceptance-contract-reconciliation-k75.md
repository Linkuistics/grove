# acceptance-contract-reconciliation-k75

## Goal

Reconcile the recovery campaign with the parent brief's requirement for a
pre-skill control followed by the same scenarios and unchanged rubric, so the
campaign report cannot substitute a new contemporaneous comparison for the
accepted experiment.

## Context

- Parent contract: `walkthrough-skill-delivery-k19` and the root brief.
- Existing recovery plan: `evaluation-recovery-k53`,
  `evaluation-recovery-k73`, `evaluation-recovery-k74`, and the
  `recovery-campaign-k54` subtree.
- Frozen historical evidence and instrument:
  `docs/evaluations/writing-code-walkthroughs/README.md` and
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

## Done when

- A clause-by-clause trace states exactly which parent acceptance claims the
  historical pre-skill records, unchanged historical rubric, recovery
  contemporaneous controls, and new enabled runs may each discharge.
- The plan fixes whether every recovery prompt, fixture, atomic row, threshold,
  endpoint, and regression rule must remain byte-identical to the historical
  instrument. Any new requirement-derived row is explicitly supplemental and
  cannot replace an unchanged-rubric comparison without a separately authorized
  requirements change.
- The recovery subtree is reshaped so `campaign-report-k72` cannot call a new
  control arm a pre-skill baseline, cannot claim an altered instrument met the
  unchanged rubric, and cannot silently drop a parent-required comparison.
- Historical rubric bytes and records remain untouched. If the parent contract
  is impossible to discharge from valid evidence, the plan names the precise
  requirements decision needed and externalizes it rather than weakening the
  acceptance claim.
- No evaluated treatment or control context runs. `measurement-design-k55`
  remains sequenced behind this reconciliation and its review.
- As the final planning action, commission a lazy `review-planning` sibling with
  this leaf's bare stem and a charter to try to disprove the reconciled
  clause-to-evidence trace before campaign execution.

## Notes

This is the substantial redesign surfaced by the integration leaf's one
fresh-context doubt review. It is separate from applying F1–F14: those repairs
remain valid constraints on whatever recovery shape survives reconciliation.

## Decisions (running log)

### Clause-to-evidence trace

| Parent acceptance claim | Historical pre-skill records | Frozen historical rubric | Recovery contemporaneous controls | New enabled runs | Deterministic checks | Status before recovery |
|---|---|---|---|---|---|---|
| A committed no-skill control exists before skill authoring and exposes the relevant failures. | Sole evidence. Case A has five valid records and exposes the accepted scope-intake gaps. Cases B and C have no valid historical sample; their shortfalls must remain explicit and cannot be repaired after skill authoring or called a pre-skill baseline. | Proves that prompts, fixtures, rows, sampling, scoring, `R`, `G`, and the endpoint were predeclared before evaluated work. | Cannot discharge this clause: they run after the skill exists and are only the historical rubric's service-drift comparator. | Cannot discharge this clause. | Cannot establish model behavior. | Discharged only for Case A; the parent brief already requires the Case B/C historical shortfalls to be reported rather than converted into inferred failures. |
| The same scenarios run in fresh skill-enabled contexts against the unchanged rubric. | Supplies the accepted Case A baseline counts and the preserved Case B/C shortfalls. | Sole authority for the acceptance comparison. Its Case A/B/C prompt, fixture, atomic-row, sample, invalid-run, scoring, threshold, endpoint, and regression semantics must govern byte-for-byte and without reinterpretation. | May supply the rubric-required contemporaneous comparison, but is never the pre-skill baseline. | May supply a new final-digest replication only after the requirements gate defines the evidential relationship between that replication and the already failed enabled campaign. | May prove which skill bytes and apparatus ran, not that the unchanged behavioral clause passed. | Open. The historical enabled campaign used mostly superseded skill bytes, is incomplete, and cannot be silently replaced. |
| Enabled behavior meets the predeclared rubric and materially improves over the no-skill baseline. | Supplies only the historical baseline operand. | Fixes `R`, `G`, the `2/5` row delta, the `10/15` primary endpoint, and all exclusions. | Supplies the second comparator required by the frozen endpoint, not a replacement endpoint or a newly selectable baseline. | Can be scored under the frozen endpoint only under an authorized replication rule. New requirement-derived rows cannot enter this verdict. | Cannot discharge judged improvement. | Not discharged. The durable report proves the original endpoint unreachable and the `A14` guard failed under every completion of its missing enabled sample. |
| The skill applies across codebases and languages. | The Case B fixture is external OCaml evidence, but its historical arm has no valid sample. | Fixes how Case B is reported and defines the separately reported transfer-probe procedure. | Can expose current runtime drift only. | Exact Case B and exact previously frozen transfer artifacts can add bounded evidence after authorization; a newly selected transfer target or new criteria are supplemental. | Structure checks can establish domain-neutral wording, not behavioral transfer. | Open: the historical transfer probe has no valid control arm and one enabled refusal. |
| Skill/plugin deployment and deterministic guarantees pass. | Historical checks describe their own digests only. | Separates deterministic properties from judged output. | No special role. | No special role beyond producing the executed digest. | Current-digest structure, installation, links, runner/auditor tests, and repository checks may discharge this conjunct. | Open until current-digest verification runs. |
| The skill elicits the named intake fields and embodies the proven generic method. | Case A records establish the accepted unguided gaps. | Supplies the accepted behavioral observations and keeps deterministic rows secondary. | May provide the contemporaneous comparison required by the endpoint. | Exact-rubric scoring may establish bounded behavior after authorization. | Static inspection may establish that the instructions contain the required method, but not that a context discovered or followed it. | Static structure is partly established; the accepted behavioral claim remains open. |

### Instrument identity and claim boundaries

- The historical records are immutable evidence, not a pool from which recovery
  may replace, rescore, or drop an inconvenient arm. The original failed enabled
  result remains in every synthesis.
- The historical pre-skill records are the only pre-skill baseline. A recovery
  no-skill arm is a **contemporaneous comparator** required to expose runtime
  drift; `campaign-report-k72` may not label it a baseline without that qualifier
  and may never use it as the parent clause's pre-skill control.
- The acceptance lane takes the byte-frozen
  `baseline/rubric.md` (`54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`)
  as its sole instrument. Every Case A/B/C prompt, fixture, atomic row, sample
  rule, invalid-run rule, scoring rule, threshold, endpoint, and regression rule
  remains byte-identical or is referenced directly from those frozen bytes.
  A compatible F1–F14 safeguard may strengthen apparatus without changing an
  accepted-experiment rule; every incompatible safeguard belongs to the later
  supplemental workstream.
- Closing the historical transfer shortfall uses the already frozen transfer
  prompt, fixture, criteria, thresholds, and regression rule under
  `enabled/transfer-probe/`. A newly selected target or any newly derived transfer
  row is supplemental generality evidence, not the historical transfer verdict.
- Any requirement-derived row, absolute gate, mixed-row rule, exposure-phase
  replacement rule, resource-window rule, dual-scoring rule, or arm-guess probe
  absent from the historical instrument is **supplemental**. It lives in the
  separate `supplemental-evaluation` workstream with its own manifest, score
  table, and verdict, and cannot replace, pool with, weaken, or rescue the
  unchanged-rubric comparison.
- Missing, invalid, or unavailable accepted-lane evidence remains missing,
  invalid, or unavailable under the historical rule. A supplemental fail-closed
  rule may describe its own lane but cannot rewrite the historical denominator or
  turn a historical shortfall into a failure or success.

### Requirements decision required before execution

The current parent contract does not state whether a post-failure replication
may independently satisfy an acceptance claim after the accepted treatment arm
has already failed. That is a requirements decision, not a planning inference.
`acceptance-replication-authority` must choose and record one of these coherent
contracts before any evaluated context runs:

1. **Recommended — authorize one exact-instrument replication.** Keep the
   historical pre-skill records as the sole baseline, preserve and report the
   failed original campaign, and permit one separately frozen final-digest
   replication to discharge the behavioral clause if it independently meets the
   unchanged historical endpoint. State explicitly that this is a requirements
   amendment defining retry authority, not a reinterpretation of old evidence.
2. **Keep the current contract strict.** Record the behavioral acceptance claim
   as failed and prohibit the recovery campaign from claiming discharge. Cut
   subsequent skill-revision and evaluation work rather than rerunning until a
   retry contract exists.
3. **Authorize a new paired instrument.** Amend the parent brief explicitly so a
   contemporaneous-control, requirement-derived campaign replaces the unchanged-
   rubric clause. This is the broadest change; every historical comparison then
   remains a separately reported limitation, and the new campaign may not call
   itself the accepted pre-skill experiment.

Until that leaf settles the choice, `measurement-design-k55` and every live-model
descendant of `recovery-campaign-k54` are blocked. That campaign now carries only
the exact historical acceptance instrument. A later sibling workstream owns all
supplemental measurement, so none of the three answers can be implemented by
sharing a freeze, score table, or label in the final report.

The working increments are therefore ordered: review this reconciliation;
settle replication authority; execute or disposition the exact-instrument
recovery; then plan any supplemental campaign. Each boundary leaves a complete,
independently legible claim before its successor begins.
