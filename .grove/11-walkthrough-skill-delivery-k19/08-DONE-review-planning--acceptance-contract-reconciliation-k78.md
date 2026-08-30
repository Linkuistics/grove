# acceptance-contract-reconciliation-k78

**Reviews:** acceptance-contract-reconciliation-k75

## Goal

Try to disprove the reconciled clause-to-evidence trace and the dependency
boundaries it imposes before any recovery campaign execution.

## Context

- Producer commit: `acceptance-contract-reconciliation-k75`.
- Review the producer task's clause table, the reshaped
  `recovery-campaign-k54` subtree, `acceptance-replication-authority`, and
  `supplemental-evaluation` against the parent and root briefs.
- Evidence authority:
  `docs/evaluations/writing-code-walkthroughs/README.md`,
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`, and the
  frozen artifacts under `enabled/transfer-probe/`.
- Prior plan and findings: `evaluation-recovery-k53`,
  `evaluation-recovery-k73`, and `evaluation-recovery-k74`.

## Done when

- Re-derive every parent acceptance clause and try to falsify which of the
  historical pre-skill records, frozen rubric, contemporaneous comparators, new
  enabled runs, transfer records, and deterministic checks may discharge it.
- Check the conclusion that valid evidence cannot authorize a post-failure
  replication by itself and that the precise unresolved fact belongs in a
  `requirements` leaf.
- Check that every accepted Case A/B/C prompt, fixture, row, sampling rule,
  invalid-run rule, scoring rule, set, threshold, endpoint, and regression rule
  remains exact, including the historically problematic no-tool and replacement
  rules.
- Check that a new no-skill arm is never called the pre-skill baseline, the
  original failed enabled result can never be dropped or pooled away, and the
  report cannot claim an altered instrument met the unchanged rubric.
- Check that the exact-instrument recovery and the F1–F14 supplemental campaign
  are genuinely independent working increments and that neither can rescue,
  weaken, or rewrite the other's verdict.
- Check the transfer boundary: only the already frozen fzf probe can close its
  historical shortfall absent an explicit requirements amendment; a fresh
  target is supplemental.
- Check ordering: no evaluated context can run before this review and any earned
  integration settle, then `acceptance-replication-authority` decides, then the
  exact campaign runs or dispositions, then supplemental planning begins.
- Record severity-ordered findings against the producer commit without editing
  its plan or campaign tree. If actionable findings exist, insert a lazy
  `integrate-review-planning` step where the walk reaches it before
  `acceptance-replication-authority`.

## Notes

Assume the reconciliation is subtly overclaiming. In particular, try to find a
path by which a post-skill control acquires pre-skill authority, a new treatment
sample silently replaces the failed one, an F1–F14 safeguard changes historical
semantics, or a supplemental row reaches `campaign-report-k72` as an acceptance
input.

## Findings

Reviewed against the `acceptance-contract-reconciliation-k75` commit, the
reshaped `recovery-campaign-k54` subtree, `acceptance-replication-authority-k76`,
and `supplemental-evaluation-k77`. No producer file, campaign task file,
historical evaluation record, or frozen rubric byte was edited by this session.
Eight findings, severity-ordered.

The reconciliation's core moves survive the attempt to disprove them. The
clause-to-evidence trace is right that only the historical records are a
pre-skill baseline, right that a recovery no-skill arm is a contemporaneous
comparator, right that the frozen materiality test needs *both* operands
(`baseline/rubric.md:168`-`172`), right that `R` and `G` are fixed by the
historical baseline report rather than recomputable (`rubric.md:174`-`184`,
`README.md:42`-`48`), and right that the post-failure replication question is a
requirements decision rather than a planning inference. The rubric digest it
pins verifies. The exclusion of every F1–F14 repair that alters an accepted rule
is correct and was applied consistently across all twenty campaign task files —
no leaf still carries a reverted exposure-phase, resource-window, arm-guess,
expanded-regression, or fail-closed rule.

What the trace does not survive is the question of whether the exact-instrument
campaign it authorizes can produce anything. F1 and F2 are the load-bearing
findings and are jointly sufficient to reopen the recommendation in k75's
"Requirements decision required before execution".

### F1 — The recommended contract is contradicted by the evidence it cites (high)

k75 recommends option 1: "authorize one exact-instrument replication" to
discharge the behavioral clause if it "independently meets the unchanged
historical endpoint" (`k75:105`-`112`), and `k76` carries that recommendation
forward as the smallest coherent change.

The primary endpoint is decided by Case A alone: `R` is fifteen Case A rows and
`G` is four (`README.md:43`-`48`), because Cases B and C produced no valid
historical sample and so contributed no repeated-gap classification. Case A is
also the one surface the frozen report diagnoses as structurally unable to
measure the treatment:

- `README.md:113`-`119` — "Seven enabled attempts visibly read `SKILL.md` and
  were invalid because Case A forbids every tool call. The four valid enabled
  streams contain no observable skill-file read or use. … The invalidation rule
  is treatment-correlated: it discarded every attempt with visible skill use and
  retained exactly the attempts without it, so the enabled sample is selected
  against observable treatment use."
- `README.md:247`-`251` — "In Case A the frozen no-tool prompt conflicts
  structurally with observable file-based skill loading … **Closing this gap
  requires a new predeclared prompt or access rule, not another run under the
  frozen contract.**"

So the authority k75 treats as governing says, in terms, that another run under
the frozen contract cannot close the defect that decides the endpoint. Two
consequences the trace never states:

1. A *failing* replication adds nothing the original failure did not already
   establish, because the same treatment-correlated selection is reproduced.
2. A *passing* replication would discharge the parent clause textually while
   resting on an enabled arm from which every observable use of the treatment
   was removed by the invalid-run rule. `k67:31`-`33` and `k72` forbid claiming
   that a visible skill read proves use; nothing anywhere requires the report to
   state the *converse* — that the retained sample is selected against visible
   use, so an improvement on it cannot be attributed to the skill.

This is the precise shape the charter asked for: not a relabelled control, but
an acceptance claim that would be true of the instrument and false of the world.

**Change (k76 owns it, this is a requirements question).** Add to `k76`'s
`Done when`: the decision must confront `README.md:247`-`251` explicitly and
record which of these it is choosing — (a) option 1 with the limitation carried
as a named, non-removable clause of the discharge (Case A's enabled arm is
selected against observable treatment use, so a pass discharges the rubric's
comparison and not a causal claim); (b) option 2, recording behavioral
acceptance as failed, which the historical evidence most directly supports;
(c) an explicit requirements amendment to the Case A access rule, which the
frozen report says is what the defect actually needs, and which by k75's own
boundary is a new instrument and therefore supplemental. Silence between (a)
and (c) is the failure mode: an exact rerun is (a) wearing (c)'s justification.
If (a) is chosen, add the same limitation to `k72`'s `Done when` beside the
superseded-digest and shortfall limitations it already preserves.

### F2 — The campaign reproduces the apparatus that caused every historical invalidation, and no leaf owns the in-boundary fix (high)

Every historical arm that died, died of the apparatus, not the instrument, and
the causes are systematic rather than stochastic:

- Transfer probe — `enabled/transfer-probe/README.md:11`-`16`: "Twenty attempts
  are invalid because their raw events accessed undeclared model-interface
  surfaces: **every invalid attempt called an MCP resource surface**, seven also
  called web search, and one also called an external document connector."
- Case B enabled — `enabled/source-fragments/README.md:13`-`18`: attempts 1 and
  2 called `list_mcp_resources` and `list_mcp_resource_templates`, attempt 3
  issued a web search; the frozen third-invalid rule stopped the leaf and "the
  remaining eight schedule positions were not executed."
- Case B historical — `baseline/source-fragments/README.md:7`-`8`, `94`: DNS
  resolution failure for both transports, no final assistant message, `0/5`
  valid after fifteen attempts.
- Case A enabled — the `SKILL.md` reads in F1.

The frozen rubric already forbids all of it. `rubric.md:66`-`69` seals the
template with "no `skills/`, `AGENTS.md`, `AGENTS.override.md`, `hooks.json`,
plugins, **MCP configuration**, or `config.toml`", and `rubric.md:89`-`93`
declares the access boundary the calls violated. The MCP catalogue and web
search were reachable anyway, under `--ignore-user-config --ignore-rules
--ephemeral --sandbox read-only` and a fresh `CODEX_HOME`. Reproducing the
rubric's sealing specification exactly is therefore *proven insufficient*, and
`campaign-runner-k56` specifies exactly that reproduction (`k56:20`-`27`).

k75 anticipates the category — "A compatible F1–F14 safeguard may strengthen
apparatus without changing an accepted-experiment rule" (`k75:76`-`78`) — and
this is the paradigm case of it: removing tool surfaces the frozen boundary
already declares out of bounds changes no prompt, row, threshold, endpoint, or
sampling rule. But the clause is inert. `MCP`, `web search`, `connector`, `DNS`,
`transport`, and `network` appear nowhere in the twenty task files and two
briefs of `recovery-campaign-k54`; no leaf names the root cause, owns a fix, or
tests that the surface is absent before an evaluated context runs.

As written the campaign spends up to 120 evaluated contexts to reproduce four
known apparatus failures. Note the interaction with the two-replacement ceiling
(`rubric.md:116`-`120`): one exhausted repetition stops the whole leaf, which is
exactly how the transfer probe ended at repetition 1 with nine positions
unexecuted. With a cause present in every attempt, the exact rerun is predicted
to terminate the same way, not merely at risk of it.

**Change.** Give `campaign-runner-k56` (and `campaign-freeze-k59`'s
pre-execution tests) an explicit `Done when`: enumerate every model-interface
surface reachable from the sealed control and enabled templates under the frozen
command shape, prove that the surfaces the frozen access boundary forbids —
MCP resource listing, web search, external document connectors — are absent
before any attempt, and record the enumeration in the freeze. State that this is
apparatus conformance with `rubric.md:66`-`69` and `rubric.md:89`-`93`, not a
new rule. Separately decide and record what a `codex-cli 0.150.1` that is no
longer obtainable means for the replication: the version, executable digest,
model alias, reasoning effort, and timeouts in `rubric.md:41`-`55` are part of
the instrument, and `k55`'s conformance table enumerates prompts, fixtures,
rows, sets, thresholds, and rules but not the execution controls.

### F3 — The transfer probe is carried as an acceptance conjunct the parent brief does not contain (medium-high)

`k70:29`-`31`: the transfer result "is **required for** the parent's
cross-codebase and cross-language applicability clause". `k54`'s brief and
k75's clause table give it a table row of its own — "The skill applies across
codebases and languages" (`k75:58`).

There is no such clause. `walkthrough-skill-delivery-k19`'s `Done when`
(`BRIEF.md:15`-`29`) has six bullets and none of them is about transfer or
generality; "It must apply across codebases and languages" is a sentence in the
brief's **Context** (`BRIEF.md:11`-`13`). The root brief's `Done when` has no
transfer conjunct either. `evaluation-recovery-k73`'s F4 said this in as many
words — "Neither the root brief's nor `walkthrough-skill-delivery-k19`'s
`Done when` contains a transfer conjunct" — and `k74` resolved F4 "applied
differently" by declaring the probe "a required, separately reported conjunct of
the parent brief's requirement". That promotion of a Context sentence to an
acceptance clause was never an amendment to any brief.

k75 was the leaf chartered to re-derive the acceptance mapping from the parent
contract, and it imported k74's promotion instead of re-deriving it. The result
widens the acceptance surface in both directions: an unavailable transfer
verdict leaves a "parent conjunct" open that the parent never stated, and a
passing one is reported as discharging an acceptance clause. This is a
supplemental-shaped row reaching `campaign-report-k72` as an acceptance input —
by relabelling rather than by pooling.

**Change.** Either add the transfer conjunct to `k19`'s `Done when` as an
explicit amendment through `k76` (which already owns "Confirm which transfer
contract applies"), or demote the probe to separately reported generality
evidence in `k54`, `k68`, `k70`, and `k72`, and reconsider whether ten evaluated
repetitions of a probe that gates nothing survive the F4 cost test that k73
already applied to it once. Do not leave the promotion implicit.

### F4 — Cases B and C can never yield a materiality or regression verdict, and the plan directs publishing one (medium-high)

`rubric.md:168`-`170` requires an enabled count to exceed **both** the
historical baseline and the contemporaneous count by `2/5`. Cases B and C have
no valid historical arm, and by the parent's own "before the skill exists"
requirement and k75's own rule, one can never be created — a new no-skill arm is
a contemporaneous comparator, not a pre-skill baseline. `README.md:50`-`52` and
`README.md:136`-`139` state the consequence directly: an absent historical arm
"prevents the corresponding comparison rather than supplying a zero", and Case C
counts "cannot satisfy the rubric's historical-plus-contemporary materiality
test". `R` and `G` are Case A rows only, so B and C also cannot touch the
endpoint or the regression guard.

Neither k75's trace nor the campaign says so. `k60`'s brief instead directs:
"Each case publishes historical-rubric row counts **and material/regression
classifications** without changing any row or denominator", and `k64:23`-`25`
and `k66:20`-`22` repeat it per case. For B and C those classifications are
undefined, and the instruction to publish one invites precisely the
zero-substitution `README.md:50`-`52` forbids.

The cost consequence is worth stating alongside the correctness one: of forty
planned repetitions (three same-case surfaces at five plus five, plus ten
transfer), thirty cannot contribute to the primary endpoint. Running B and C is
still required by the parent's "same scenarios run in fresh contexts" clause, so
this is not an argument to cut them — it is an argument that what they can
deliver must be stated up front rather than discovered at adjudication.

Related, and for `k76` to weigh rather than for the campaign to fix: k74 applied
k73's F6 (every requirement-derived row outside the target set becomes a
regression row) and k75 correctly reverted it to the frozen `G`. That reopens
the hole F6 identified — the one observed enabled regression in the historical
campaign, `C23` at `-2` (`README.md:165`, `168`-`169`), fell outside every
guard. Under the exact instrument it would again be visible in the tables and
unable to block the verdict. That is a property of the instrument the
requirements decision is choosing, not a defect of this plan, but it belongs in
the record of what option 1 buys.

**Change.** State in `k55`'s conformance work and in the `k60` brief that Cases
B and C can produce only descriptive per-row counts and terminal
shortfall/invalidity records under the frozen rubric, that material and
regression classification is undefined for them, and that "undefined" is
published as such rather than as a zero, a miss, or an omission. Add the same to
`k67` and `k72` so the synthesis cannot read a missing B/C classification as a
neutral or adverse result.

### F5 — The rubric-mandated second blind scoring is unassigned and its case selection is unfrozen (medium)

`rubric.md:128`-`130`: "One complete case is scored a second time by another
independent blind context; report the criterion-level disagreement count and
both citations before resolving disagreements under the same rules." That is
part of the exact instrument, not a strengthening.

No leaf in `recovery-campaign-k54` requires it or owns it. It appears only in
exclusion lists. Most are correctly qualified — `k55:46`-`47` says "dual scoring
**beyond the historical requirement**", `k62:19`-`20` and `k66:18`-`19` say
"absent from the historical rubric" / "New" — and k75's own boundary bullet
(`k75:83`-`85`) carries the "absent from the historical instrument" qualifier,
so the authority document is sound here. But `k64:18`-`19` is unqualified:
"Stronger exposure classification, **two-scorer coverage**, forced arm guesses,
and new rows are excluded and cannot fill a shortfall." If the freeze selects
Case B as the double-scored case, `k64` refuses the rubric's own requirement.
`k59:21`-`23` builds a *machine-checkable* exclusion list from this same family
of statements, which is where an unqualified phrasing turns into a hard refusal.

Beyond the wording: which case receives the second scoring is a free choice the
freeze does not pin. k73's F11 showed why that matters — historically the
single-scored arms were exactly the irregular ones (`README.md:210`-`212`), and
a post-hoc choice of which case to double-score is a selection channel the joint
freeze exists to close.

**Change.** Add the second blind scoring to `k57`'s owned scoring contract as a
positive requirement; name the case that receives it in `campaign-freeze-k59`,
before execution, with a rule that survives that case ending in shortfall; and
qualify `k64:18` the way its three sibling leaves are already qualified.

### F6 — `transfer-freeze-k58`'s `Done when` is internally unsatisfiable (medium-low)

`k58` must byte-verify "the historical `enabled/transfer-probe/` prompt, fzf
fixture/revision, criteria, thresholds, regression rule, schedule, and role
chronology" and "selects no new target" — and then, three lines later, must
establish that "The target is outside this repository, **absent from the
historical campaign**, and worded without `ordinal-fs-tree` domain assumptions."
The target is the historical campaign's target; the two clauses cannot both
hold.

This is residue from the pre-reconciliation shape, where `k58` selected a new
probe and the rubric's constraint (`rubric.md:21`-`23`) was that the selection
be absent from the skill bytes, completed book, baseline outcomes, and selected
codebase — that is, absent from the *same-case* campaign, not from the transfer
probe itself. k75's rewrite updated the first two clauses and left the third.

**Change.** Restate the third clause as the rubric's actual constraint, already
satisfied and re-verifiable from `enabled/transfer-probe/selection/`: the target
is outside this repository, absent from the same-case corpus and the completed
book, and was selected before the criteria were frozen by a role that had seen
neither the skill bytes nor the source.

### F7 — The clause table is not clause-by-clause against the parent `Done when` (medium-low)

k75's deliverable is "A clause-by-clause trace" of "every parent acceptance
claim". Mapped against `BRIEF.md:17`-`29`, the six-row table is not a bijection
and never says so: bullet 5 ("run … meet the rubric, and show a material
improvement") is split across rows 2 and 3, correctly; bullets 2 and 3 are
merged into row 6; bullet 4 — "Skill frontmatter, progressive disclosure,
citations, harness declaration, plugin layout, and description follow the
Linkuistics authoring conventions" — has no row of its own and is only
implicitly inside row 5; and row 4 has no bullet at all (F3).

The work bullet 4 names is in fact covered operationally by
`deployment-verification-k71:23`-`26`, so nothing is unowned. The defect is that
a later session cannot check the trace's coverage against the parent, which is
the one property a clause-by-clause trace exists to have. `k72` is required to
"reproduce the clause-to-evidence trace" and would reproduce the gap.

**Change.** Add the parent-bullet citation to each row, split or annotate the
merged rows, and state the two deliberate merges explicitly.

### F8 — `recovery-campaign-k54`'s brief claims a child it does not have (low)

`k54`'s `Decomposition` opens with "`acceptance-replication-authority` first
decides whether a post-failure replication may discharge the parent claim and
which contract governs it", listing it alongside `measurement-design-k55` and
the other members of the subtree. The leaf is not in the subtree: it is the
`requirements` leaf `acceptance-replication-authority-k76`, a sibling of the
`recovery-campaign-k54` node at the `k19` level.

The ordering is correct and is the reason this is low rather than material — the
k76 leaf sits at an earlier position than the k54 node in the same directory, so
the walk reaches the decision before it descends into the campaign, which is
exactly the boundary k75 intended. But the
brief describes a decision made *inside* the campaign it constrains, which is
the opposite of the independence being claimed, and a session reading `k54`'s
brief alone would look for the leaf in the wrong directory.

**Change.** In `k54`'s `Decomposition`, name `acceptance-replication-authority`
as an external precondition settled before this node is entered rather than as
its first child.

## Checks that passed

Recorded so the integration does not re-derive them.

- Rubric digest: `baseline/rubric.md` hashes to
  `54cc097463616207c7be98ca072256ee81405294b1926844961a9cf65282fea6`, matching
  `k75:72`, `README.md:39`-`40`, and `enabled/transfer-probe/README.md`.
- `R` has fifteen members and the endpoint is `ceil(2 × 15 / 3) = 10`
  (`rubric.md:174`-`184`, `README.md:42`-`48`); `G = {A02, A03, A14, A15}`; the
  `A14` guard breach is `5/5` historical against at most `3/5` enabled
  (`README.md:104`).
- Transfer criteria are `T01`–`T20` with sixteen non-compliance rows and an
  eight-row threshold, exactly as `k70:17` states
  (`enabled/transfer-probe/criteria.md`, `README.md:38`).
- Case B's fixture is the external OCaml `targets/ocaml/check_floor.ml` at its
  frozen digest (`rubric.md:235`-`243`), so the same-case set does contain a
  non-Rust, cross-repository surface as `k55` and k74's D2 claim.
- The transfer materiality test is measured against the contemporaneous control
  alone (`rubric.md:26`-`29`), so — unlike Cases B and C — the frozen probe can
  in principle reach a verdict on rerun. F2, not the instrument, is what blocks
  it.
- Every F1–F14 repair that alters an accepted rule is excluded consistently:
  `arm-guess`, `resource window`, `pre-exposure`, `fail-closed`, `mixed-control`,
  `pair-atomic`, and `requirement-derived` appear in the campaign tree only
  inside exclusions. No leaf still carries a k74 fix that k75 reverted.
- Ordering holds. `01-skill-baseline-k20` and `03-skill-evaluation-k22` are
  wholly terminal; the live sequence is this review → `k76` → `k54` → `k77`.
  `k76:38`-`40` and `k55:53`-`56` both handle the non-authorizing outcome, so a
  strict contract does not leave `k55` runnable merely by being present.
- No path found by which a new no-skill arm is labelled the pre-skill baseline,
  or the original failed enabled arm is dropped or pooled: `k67:16`-`18`,
  `k67:25`-`27`, and `k72:19`-`22` each block it independently.
- The two workstreams are ordered so influence can only run one way: `k72`
  publishes inside `k54` at position 10, before `k77` at position 11, so no
  supplemental score can reach the acceptance synthesis.

## Decisions (running log)

- Findings are recorded against the committed producer artifact only. No file in
  `recovery-campaign-k54`, no `docs/evaluations/` record, and no frozen rubric
  byte was edited by this session, per the `review-*` discipline.
- The review produced actionable findings, so the lazy `integrate-review-planning`
  step is earned. Per `references/decompose.md` the target is the first sibling
  entry after this leaf whose subtree still holds live work: that is the leaf
  `acceptance-replication-authority-k76` at position 09, not the
  `recovery-campaign-k54` node behind it. Inserting at k76's slot is what keeps
  the requirements decision — the leaf F1 and F3 most directly change — from
  running before the findings land.
- F1 and F3 are addressed to `k76` rather than to the campaign, because both are
  requirements-shaped: one asks what a replication may claim, the other asks
  whether a clause exists at all. The integration should reshape `k76`'s
  `Done when` and leave the choice itself to that leaf and its human.
- No in-session reviewer was materialised: a `review-*` session spends none
  (`references/execute.md`).
