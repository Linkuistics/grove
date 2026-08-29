# evaluation-recovery-k73

**Reviews:** evaluation-recovery-k53

## Goal

Adversarially audit the committed recovery-campaign decomposition before any
measurement-design or campaign child runs.

## Context

- Producer artifact: `evaluation-recovery-k53` and the full
  `recovery-campaign-k54` subtree it creates.
- Parent acceptance contract: `walkthrough-skill-delivery-k19` and the root
  brief.
- Historical hazards:
  `docs/evaluations/writing-code-walkthroughs/README.md` and its frozen rubric.

## Done when

- Try to disprove that the exact dependency order freezes a valid instrument
  before treatment execution and that every leaf is one independently useful,
  focused increment.
- Check that criteria are independent of treatment and new outcomes, every
  required behavior family has absolute and relative protection, regressions
  cannot hide in mixed rows, and empty or incomplete data cannot pass
  vacuously.
- Check that assignment, exposure, replacement, protocol-failure, access,
  treatment-delivery, scorer blinding, disagreement resolution, and bounded
  claim rules cannot select favorable enabled samples or overstate what a skill
  read proves.
- Check that runner, auditor, transfer roles, joint freeze, same-case evidence,
  transfer evidence, deterministic deployment verification, and final report
  each have one owner with no missing or circular handoff.
- Record findings against the producer commit without editing its plan or
  campaign subtree.

## Notes

Assume the corrected plan remains overconfident. Prefer a smaller valid campaign
when a leaf or criterion does not earn its cost. If findings warrant changes,
cut the lazy `integrate-review-planning` step immediately before the recovery
campaign node so no execution child can run first.

## Findings

Reviewed against the `evaluation-recovery-k53` commit and the committed
`recovery-campaign-k54` subtree. No producer file was edited. Fourteen findings,
severity-ordered. F1–F4 are the ones that change the campaign's shape.

### F1 — Serial case ordering leaks outcomes back into sampling (high)

`same-case-evidence-k60` interleaves generation and adjudication:
`scope-runs-k61` → `scope-adjudication-k62` → `source-fragment-runs-k63` → …
So a published case verdict exists before the next case executes. The
`recovery-campaign-k54` brief defends this as "resource ordering only" on the
ground that the joint freeze stops an early outcome changing a later prompt,
criterion, apparatus, or treatment byte. That defence is complete for
*artifacts* and empty for *discretion*: the freeze does not constrain the one
judgement each runs leaf still makes, namely whether a given failed attempt was
pre-exposure and therefore replaceable (F2). An operator who has just read an
adverse Case A verdict makes Case B's replacement calls under a live incentive.

**Change:** order all three generation leaves before any adjudication leaf —
`k61`, `k63`, `k65`, then `k62`, `k64`, `k66`, `k67`. Every dependency survives
and no leaf changes content; this is a pure reorder that closes the channel.

### F2 — Replacement legality is decided by the leaf that benefits and is never audited (high)

Replacement is restricted to "failures proven to precede prompt and treatment
exposure" (`k53`, `k55`, `k61`, `k63`, `k65`, `k69`). Exposure phase is recorded
by the runner (`k56`) and *classified* by the auditor — but the auditor runs
inside the adjudication leaves (`k62`: "classifies every access, exposure phase,
retained outcome"), which execute after the runs leaf already replaced. So the
replace/retain decision is taken in-session, on unaudited judgement, and nothing
in the subtree owns:

- retroactive verification that every replacement was genuinely pre-exposure; or
- a frozen consequence when an illegal replacement is found.

This is the precise mechanism that selected the historical enabled arm. The plan
correctly names the *rule* and leaves the *enforcement* to the party it binds.

**Change:** make the replacement gate a deterministic function of the recorded
exposure phase, owned by `k56`/`k57` and exercised by their stub tests, so the
runs leaf cannot replace an attempt the classifier would call post-exposure.
Add to each adjudication leaf's `Done when`: re-verify replacement legality over
the preserved attempt history before scoring, and apply the frozen consequence
(reinstate the wrongly replaced attempt, or mark the case protocol-failed) when
verification fails.

### F3 — Capping *pre-exposure* replacements manufactures unavailable verdicts and buys nothing (high)

The plan keeps a finite pre-exposure allowance ("exhausted pre-exposure
replacements … have explicit non-vacuous verdict semantics", `k55`; "An
exhausted pre-exposure allowance produces the frozen unavailable record",
`k69`). A proven pre-exposure failure carries no outcome information by
construction, so a ceiling on it cannot prevent selection — it can only convert
infrastructure flakiness into an unreachable endpoint. That is not hypothetical:
under the frozen rule "at most two replacement attempts for each planned
repetition" (`baseline/rubric.md`), historical Case B died at `0/5` valid after
15 DNS/transport timeouts and the transfer probe died at `0/5` control. The
recovery plan reproduces the failure mode it was cut to escape.

Note the tension with F2: a cap is only a defensible backstop if the
pre-exposure proof is weak. Either the proof is deterministic — then the cap is
unnecessary and harmful — or it is not, and the cap is the wrong mitigation for
that. Fixing F2 makes the cap removable.

**Change:** bound pre-exposure retries by a resource budget (wall clock or
attempt cost) with a predeclared, non-selective resumption rule, and reserve
hard ceilings for post-exposure outcomes, which are retained rather than
replaced anyway.

### F4 — The transfer sub-campaign does not earn its cost as specified (high)

By the plan's own rules the transfer result "contributes nothing to the primary
endpoint or skill wording" (`k54` brief), "cannot drive wording changes in this
cycle" (`k68` brief), and "cannot rescue or weaken the same-case verdict"
(`k67`). Neither the root brief's nor `walkthrough-skill-delivery-k19`'s
`Done when` contains a transfer conjunct. So as written it is a sub-campaign
that gates nothing, at a cost of: `transfer-freeze-k58` (a three-role,
information-flow-controlled selection apparatus), the whole `k68` subtree, ten
evaluated runs, and a second two-context blind adjudication.

The cross-codebase/cross-language evidence transfer is meant to supply may
already sit inside the same-case set: the frozen Case B fixture is
`targets/ocaml/check_floor.ml` from `Linkuistics/APIAnyware` — a different
repository *and* a different language. `k55` never says whether the recovery
cycle's source/fragment fixture stays external, and that omission is what makes
the transfer spend unjustifiable either way.

**Change:** pick one branch explicitly in `k55`/`k54`.
(a) State the parent clause transfer discharges and make it a named conjunct of
the verdict — then it earns `k58` and `k68`; or
(b) keep the same-case source/fragment fixture external and non-Rust, cut `k58`
and the `k68` subtree from this cycle, and record the residual generality
question as tree work after the primary verdict. Recommended: (b) — it is the
largest removable cost in the campaign and its removal cannot touch the primary
endpoint.

### F5 — The primary instrument has weaker contamination control than the secondary one (medium-high)

`transfer-freeze-k58` protects its criteria with role separation, restricted
inputs, preserved raw inputs/outputs and chronology, and requires the
information flow be "checkable rather than asserted". `measurement-design-k55`
protects the *primary* criterion set — the higher-stakes artifact — with the
single clause "before the skill is consulted": an unverifiable assertion by one
session that already has the deployed `SKILL.md` in its checkout and the
historical report (whose `A05`–`A21` rows are visibly skill-shaped) in its
context. The plan itself concedes the standard by demanding checkability of the
lesser artifact.

**Change:** give same-case criterion authoring the same shape `k58` mandates —
a criterion author receiving only the parent acceptance contract and generic
method, with exact inputs, raw outputs, runtime identity and chronology
preserved, and the skill-to-criterion coverage map produced afterwards by a
separate step that provably cannot add or weaken a row.

### F6 — Regression protection can leave requirement rows unguarded (medium)

`k55` names "primary target set, regression set" as two sets and `k67` guards
"every protected required behavior", but nothing requires target ∪ regression to
cover every requirement-derived row. A row in neither set can regress
invisibly. The historical instrument shows exactly this failure: its guard was
`G = {A02, A03, A14, A15}` — four of twenty-one Case A rows and none of Case C's
twenty-four — and the one observed enabled regression (`C23`, `-2`) fell outside
every guard, which the report had to note in prose because no rule caught it.

Related: the reviewer brief's "regressions cannot hide in mixed rows" is
unaddressed. A row whose control is mixed (say `3/5`) can absorb a real drop
inside sampling noise under a count-delta rule, and the plan states no
minimum-detectable-change or mixed-row handling.

**Change:** freeze the regression rule over *every* requirement-derived row not
in the target set, and state explicitly how a mixed-control row is judged.

### F7 — Nothing checks the new instrument against the frozen one (medium)

Deriving criteria from the requirement rather than the old rubric is right, and
preserving `baseline/rubric.md` byte-for-byte is right. But nothing prevents the
new instrument being quietly *easier* than the one the skill already failed. The
"cannot add or weaken a row" guard in `k55` binds the skill→criteria map, not
the historical→new relation.

**Change:** `k55` publishes a retained / reworded / dropped map against
`baseline/rubric.md` with a reason per drop. It is evidence of non-weakening,
consumed as an audit trail — never as a source of criteria.

### F8 — "Non-vacuous" is asserted without a direction (medium)

`k55` requires "explicit non-vacuous verdict semantics" for exhausted
replacements, empty sets, incomplete arms and unblindable samples, and `k67`
requires they "follow the exact non-vacuous frozen outcome". Neither states the
*direction*. A freeze that resolves "unavailable ⇒ row dropped from the
denominator" satisfies every word written here and is precisely the vacuous pass
the reviewer brief forbids. `k72`'s "A failed conjunct leaves the parent open"
covers *failed*, not *unavailable*.

**Change:** freeze fail-closed explicitly — missing, unavailable, protocol-failed
and unblindable data can never contribute to attainment, and any such conjunct
leaves the parent requirement open.

### F9 — Scorer-execution machinery has no owner (medium)

Handoff gap. `k55` *designs* the scorer instrument, blind tie-resolution
procedure, scorer prompt and runtime; `k57` builds the bundle seam; `k59` *pins*
the "scorer instrument"; `k62`/`k64`/`k66`/`k70` *apply* it. No leaf builds and
tests the harness that invokes the two blind scorer contexts and the blind
resolver. `k56` is explicitly scoped to control/enabled treatment templates and
its stub coverage says nothing about scorer contexts; `k59`'s stubbed end-to-end
tests reach bundle content, not scorer execution. This is the only genuine
missing-owner defect in the subtree (nothing is circular).

**Change:** name the owner — extend `k57`, or cut a sibling leaf — and add
scorer/resolver invocation to `k59`'s stubbed end-to-end coverage.

### F10 — Blinding is nominal on the rows that decide the verdict, and unblinding is not measured (medium)

A scorer applying rows that describe skill-prescribed behaviors can infer the
arm from the behavior it is scoring; deterministic redaction cannot remove that.
`k57`'s "residual treatment disclosure" is a redaction-residue check, not a
measure of scorer-inferred arm, yet "blind scorers" is carried as a control
throughout.

**Change:** predeclare an arm-guess probe — each scorer records a forced arm
guess per bundle — and report guess accuracy as a stated limitation of the
primary result rather than treating unblinding as an exceptional case.

### F11 — Dual scoring is promised only for complete cases (medium-low)

`k55` fixes "two independent blind scorers **per complete case**". The frozen
rubric said the same in effect ("One complete case is scored a second time"), and
the arms that ended up single-scored were exactly the irregular ones — the
enabled Case A arm and the Case B contemporary control. The wording reproduces
the historical hole.

**Change:** require dual scoring of every scored bundle, whatever the case's
completion state.

### F12 — Pair members are not required to execute adjacently (medium-low)

The plan pins "runtime identity", but the historical report already records that
"the model name is a mutable service alias". A precommitted counterbalanced
order does not imply the two arms of a pair run close in time, so service-side
drift can confound a pair — the one confound the paired design exists to remove.

**Change:** freeze back-to-back execution of a pair's two arms, and record each
arm's start/end timestamps against its pair id.

### F13 — `campaign-auditor-k57` lacks the pre-freeze no-live-execution guard its siblings carry (low)

`k55` ("No evaluated treatment or control context is launched in this leaf"),
`k56` ("without launching a live evaluated model") and `k58` ("without a live
treatment or control execution") each state the guard. `k57` does not, and its
`Done when` demands fixture tests that prohibited "direct, indirect, compound,
expanded, and symlinked accesses are detected" — the leaf most tempted to reach
for a real agent to generate them.

**Change:** add the same explicit guard to `k57`, and state that its prohibited-
access fixtures are synthesized event records.

### F14 — The freeze is irreversible but is protected only by the in-leaf allowance (low, risk judgement)

`campaign-freeze-k59` is the campaign's point of no return by its own rule: any
later change to a frozen byte "requires a new predeclared cycle rather than an
amendment in place". Its adversarial protection is one fresh in-session reviewer
over the whole freeze candidate — which is a correct use of the allowance in
`references/execute.md`, not a violation. But it means the campaign's highest-risk
artifact, the instrument built by `k55`, is reviewed only as one component of a
bundle, and `k55` itself carries an `impl` kind, so nothing routes a
design-shaped adversarial read at it.

**Change (judgement, not defect):** consider a scheduled `review-impl` beside
`k59` — the same shape this leaf is — given that the alternative to catching a
defect there is discarding an entire executed campaign.

## Decisions (running log)

The review produced actionable findings, so the lazy `integrate-review-planning`
step is earned. Per `references/decompose.md` the target is the first sibling
entry after this leaf whose subtree still holds live work: that is the node
`recovery-campaign-k54`, not a leaf inside it. Inserting at the node's slot is
what keeps every execution child — starting with `measurement-design-k55` — from
running before the findings land.

Findings are recorded against the committed producer artifact only. No file in
`recovery-campaign-k54`, no historical evaluation record, and no frozen rubric
byte was edited by this session, per the `review-*` discipline.

F4 is left as an explicit two-branch decision rather than a unilateral cut: the
recommendation is to drop the transfer sub-campaign, but which parent clause
transfer discharges is a scope call the integrating session should settle against
the parent brief, and (a) remains a coherent answer.
