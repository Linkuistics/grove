# harness-selection-and-execution-k42

**Reviews:** harness-selection-and-execution-k6

## Goal

Review the implementation tree that `harness-selection-and-execution-k6` cut
for `harness-dispatch`'s first release, before any increment runs. Produce
findings, not fixes.

## Context

Read the planning leaf's running log and its commit, which grew the tree. Read
the root brief's `Implementation plan`, every node brief, and every leaf body
under the ten increments. Judge them against
`docs/specs/harness-selection-and-execution.md`, its ADRs and the root brief's
accepted requirements. The design is reviewed and settled. Challenge the
decomposition, not the design.

## Done when

The findings are committed, and each is classed as blocking or advisory. The
doubts the producer could not settle alone are these:

- **Independence.** Can each increment be demonstrated at its boundary without a
  later sibling? Is any leaf a horizontal layer that sits dead until its
  successor lands? Suspects are `choice-and-refusals-k15`, which bundles
  diagnostics, and `dispatch-documentation-k41`.
- **Order.** Delivery runs third to surface the supported-target floor early.
  Is the cost worth it? The archive assertions and smoke churn with each later
  shipped file. Does computed-before-records, or records-before-boundary,
  hide a dependency the log does not state?
- **Coverage.** Repeat the coverage walk. Every cell of the spec's `#test-seams`
  table, every root acceptance case, and the planning leaf's `Done when` should
  map to a leaf that owns it. Look for obligations no leaf names: a
  documentation claim, a Taskfile task, an archive assertion, a book update.
- **Size.** Which leaves will not fit one session? `routed-inspection-k13`
  carries the crate, the worker, the protocol, authority and inspection.
  `ambient-authority-k30` carries five hostile classes with probe builds.
- **Review placement.** Five node reviews are expected. Is that too many or too
  few, and is each one's stated doubt the right one?
- **Methodology fit.** Each increment is a node of this grove, not a separately
  created grove. Does that reading of the planning rule hold, given the
  repository's finish-and-release sequence?

## Notes

A `review-planning` session spends no in-session reviewer. If the findings
warrant action, cut `integrate-review-planning` with the bare stem
`harness-selection-and-execution`. Place it where `pick` reaches it next,
which is before `grove-task-slots-k11`.

## Decisions (running log)

Reviewed planning commit `d45c35224863b87dbc0004c4237a8b58c25c60e9`, its running
log, the root brief, every brief and leaf under the ten increments, and the
settled specification and ADRs. This is an inspection-only review: no tests,
builds, lint or format commands ran, and no implementation or planning artifact
was changed. No in-session reviewer was used.

The coverage walk finds owners for the agreed acceptance surface. Its defects
are dependency and lifecycle defects, rather than a missing eleventh feature:
three blocking findings and two advisory findings follow. Integration belongs
before `grove-task-slots-k11`; its body will name this review and leave triage to
the integrating session.

## Findings

### F1 — Blocking: a review inside its producer node cannot receive that node's creator reference

Locations: `.grove/14-k12/03-impl--choice-and-refusals-k15.md:44`,
`.grove/17-k23/03-impl--run-lookup-k26.md:33`,
`.grove/18-k27/03-impl--ambient-authority-k30.md:48`,
`.grove/20-k35/02-impl--grove-review-adapter-k37.md:46`, and
`.grove/21-k38/_creator-reference.md:60`.
Contract: `docs/specs/harness-selection-and-execution.md:364` and
`docs/adr/a-review-carries-its-creator-reference.md`.

All five expected implementation reviews are to be created **inside** the node
they name in `**Reviews:**`. The planning log expressly says this prevents the
node closing before review. But the settled creator convention names the run
whose retirement **closed** that node, and a node that stays open writes
nothing. These two conditions cannot both be satisfied before review selection.

For example, k40 adds a live review inside `creator-reference-k38` and retires.
k38 still has a live descendant, so k40 does not close it and cannot attest its
creator. The supplied Grove policy next reads a review of k38 with no current
creator reference and refuses. Only retiring that review could close k38, which
would make the review's own run the node's creator after the selection that
needed it. Writing k40's run early would change the accepted convention; an
owner declaration would turn an otherwise dispatched producer into a manual
adoption case. Neither is the planned unattended mechanism.

This also conflicts with the existing flat review-chain rule: the review of a
decomposed producer is a sibling of that producer, rather than a child keeping
it live. It is a planning defect, not a reason to redesign creator identity.
Give the reviewed producer a completion boundary before its review needs its
provenance. Preserve the intended review-before-dependent-increment order when
repairing placement; appending all reviews at the root's end would lose the
stated reason for reviewing the static foundation early. The paired integration
owns the concrete tree repair.

### F2 — Blocking: computed-policy completion depends on the later selection deadline

Locations: `.grove/16-k20/01-impl--computed-selection-k21.md:20`,
`.grove/16-k20/_computed-policy.md:13` and `:54`, and
`.grove/18-k27/01-impl--selection-cancellation-k28.md:17`.
Contract: `docs/specs/harness-selection-and-execution.md:250` and `:280`.

The computed increment requires unresolved promises to refuse, while assigning
the whole-selection deadline to k28, after the entire records increment. k21
narrows its own obligation to a never-settling promise "the worker can detect",
but the node's `Done when` contains no such exception. A pending promise kept
alive by ordinary asynchronous work need not terminate the worker or provide an
event the worker can classify as permanently unresolved. A synchronous loop in
the callback cannot run a worker-side timeout callback at all.

Thus a command-seam fixture for the node's unresolved-promise case can wait
forever, and the increment cannot demonstrate its promised refusal without a
later sibling. Its byte bounds do not establish a finite selection-time bound.
Static policy imports already execute TypeScript, so import hangs expose the
same prerequisite in the first worker slice.

The minimum externally enforced deadline and worker cleanup must exist when
arbitrary imports/callbacks are admitted, or those capabilities must remain
explicitly unavailable until that boundary exists. The later boundary increment
can still own the signal-race and hostile-fixture work; records-before-final-
linearization does not require deadline-after-records. Include a pending promise
with live asynchronous work and a synchronous spin in the owning command-seam
acceptance, rather than only a promise for which Bun happens to exit.

### F3 — Blocking: the first releasable run bypasses the required handoff record

Locations: `.grove/14-k12/02-impl--harness-exec-k14.md:12` and `:37`,
`.grove/14-k12/_static-dispatch.md:11`, and `.grove/_BRIEF.md:299`.
Contract: `docs/specs/harness-selection-and-execution.md:444` and the root
acceptance case beginning "Before launch, persist a run identity".

k14 explicitly delivers `run` by plain exec while deferring the handoff record
and run identity to k23. The static node documents this command as delivered;
delivery then installs and exercises it on all targets before records land.
The running log also says a human could release at any increment boundary.

The required record is a precondition of **every run**, rather than an optional
data command or an argument form an early increment can refuse. A successful
static run at these boundaries executes with no committed attempt, no exported
run identity and no required-record failure check. Merely refusing the `runId`
slot or narrowing the feature notice does not satisfy that precondition. This
is a missing dependency of the first runnable slice; all record work does have
an eventual owner.

Separate the minimum durable handoff from later observation import and policy
run lookup, or keep the early product boundary at useful inspection until
recorded execution is available. Retain the required write-failure-prevents-
exec case at the first boundary that exposes `run`. This preserves the reviewed
design instead of silently introducing a release with weaker execution semantics.

### F4 — Advisory: the first worker leaf carries several unresolved implementation seams

Locations: `.grove/14-k12/01-impl--routed-inspection-k13.md:10` and `:21`, and
`.grove/18-k27/03-impl--ambient-authority-k30.md:30`.

k13 owns a new Rust package, Bun build/install integration, private framed
protocol, build identity, runtime location, environment admission, static
evaluation, human/JSON inspection and deterministic test-worker acquisition.
The latter acquisition decision is still explicitly open. The native experiment
establishes only narrower runtime facts and supplies no production protocol.
F2 adds a real lifecycle prerequisite to this already large first slice.

This is the leaf most likely to need decomposition before it reaches a coherent
green boundary. Keep any further cuts behaviorally demonstrable; a crate-only
or protocol-only sibling would recreate the horizontal split the plan avoids.
k30 also carries multiple probe variants and five hostile classes, but it builds
on known probe evidence and an existing launcher, so its size risk is lower.
No inspection alone proves that either leaf cannot fit a session; this is an
advisory sizing warning, not a demand for a speculative extra node.

### F5 — Advisory: the node-based interpretation does not match the planning rule literally

Locations: `.grove/10-DONE-planning--harness-selection-and-execution-k6.md:47`
and `/Users/antony/.codex/skills/grove-planning/SKILL.md`, "Find the working
increments before cutting any leaf".

The planning rule says to create a separate grove for every obvious working
stage and only then cut the current increment's leaves. The producer instead
cut all ten stages as nodes/leaves of one grove, with a recorded rationale.
Existing precedent and the absence of a creation verb do not themselves make
those two instructions equivalent.

The repository-specific case is persuasive: the root charters one first release,
the confirmed finish cycle tears down this tree and integrates/releases once,
and no accepted requirement asks for ten separate releases. The dependency-
ordered nodes keep that scope legible. This is consequently advisory rather
than a request to split the release or reopen the requirements interview. Keep
the deviation explicit and distinguish it from literal compliance with the
generic rule. F1–F3 still require coherent boundaries even under this reading.

## Coverage walk and dispositions

These are **planned owners**, not reports of passing implementation tests. The
walk covered the complete current root acceptance list and all six seam rows;
no conclusion below is inferred from absence in the code graph.

| Root acceptance case, in brief order | Owning leaves and expected instrument |
|---|---|
| Independent caller, optional task, generic review association | k13–k15, k21–k22, k36; standalone command seam without Grove |
| Static/computed choice and complete inspection | k13–k15, k21–k22, k24–k26, k37; command inspection and actual examples |
| Invalid/missing input, context overflow and explicit-choice refusals | k13–k15, k21–k22, k28; command refusal cases; F2 corrects deadline dependency |
| Personal authority, explicit relative selection, ambient inputs inert | k13, k30; public launcher with the named firing probe builds |
| Original provider, declaration, associations and provider membership | k26, k36–k37; shipped selector/adapter seam |
| Stable identity through retirement/reorder, observations after teardown | k24–k26, k40; run-ID lookup, observation import and Grove lifecycle seam |
| Authoritative Grove slots and literal unchanged prompt | k11, k14, k32; existing Grove launch-boundary seam |
| Authoring succeeds and bad delegated policy refuses only at launch | k32, k34; launch boundary and configure-grove/usage guidance |
| Final cwd/terminal/exits/channel, helper authority and fixed choice | k14, k28–k30, k32–k33; command seam plus controlling PTY |
| Interruption/timeout, null stdin, structured output and refusal exits | k13–k15, k22, k28–k30, k33; command/PTY seams; F2 moves the prerequisite earlier |
| Required record, distinct evidence, later observations and unknowns | k24–k26, k29, k36–k37; store failures/imports and post-commit cancellation; F3 corrects first-run dependency |
| Release/install, package workflows, all targets/floors, usage acceptance | k13, k17–k19, k21, k24, k34, k37, k39–k41; Taskfile, archive assertions, installed smoke and final documentation review |

| Specification seam row | Owners |
|---|---|
| Temporary policies and fake harnesses | k13–k15, k21–k22, k29 |
| Actual shipped examples | k15, k21, k36–k37 |
| Authority and lifecycle fixtures | k13, k22, k28–k30 |
| Records and observations | k24–k26, k29, k37, k40 |
| Existing Grove launch boundary | k11, k32, k40 |
| Existing Grove launch boundary, controlling PTY | k33, using k28–k30's implementation |

The planning leaf's remaining `Done when` obligations have owners too: creator
methodology/conformance/pins/provisioned skills are k39–k40; original-creator
selection is k36–k37; outcomes are k25; bounded authority/cancellation is
k22/k28–k30; direct-route compatibility is k11/k32; installed layout/floors are
k17–k19 with k21/k24/k37 regression changes; documentation/configure-grove
acceptance is k34/k40/k41 and k41's lazy review. Package Taskfile/check/build/
install work is k13, target smoke work is k18–k19, and archive assertions are
k17 with each added shipped example/adapter's owning leaf. The root's common
obligation carries source-exact updates to touched walkthrough books; no new
book campaign is a first-release requirement. F1 concerns the five reviews'
placement, not a missing review count.

The remaining decomposition is defensible. k11 has standalone wrapper value;
inspection, explicit choice and actionable diagnostics each change usable
behavior; computed context, observation entry and read-only run lookup can each
be demonstrated through the command seam once their prerequisites hold. k15's
diagnostics bundle is cohesive. k41 consolidates delivered documentation,
architecture and release notes and supplies the mandatory acceptance walk; it
does not sit dead waiting to make earlier code useful.

Early delivery is worth its repeated smoke cost because failure at the Linux
floor reopens the runtime decision before the larger SDK/adapter investment.
Keep the existing per-target regression obligation, including bundled SQLite.
Computed-before-records is sound for SDK run lookup, and records-before-final-
handoff is sound for the cancellation append; neither requires deferring the
minimum record or deadline as in F2–F3. The five implementation review doubts
are material; no additional review is justified solely by node count. Their
placement needs F1's repair. The final documentation review remains separate.

## Evidence limits

Tier 2 graph project:
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`,
generation `2026-09-29T11:18:53Z`, ready. `SessionConfig.expand` was discovered,
traced in both directions at depth one (seven callers, no pagination), and read
as an exact snippet. It supplies the existing four slots; this corroborates the
first slot increment's existing integration seam and is not evidence that the
new slots exist. Coverage reports matching metadata and no recorded gaps for
that file. The graph reports no callees there despite the snippet's template
call, so no dependency completeness claim uses that edge absence.

Coverage was checked for every existing task/brief path and the source/config/
contract paths used here. Task files have changed or untracked graph metadata;
docs and scripts are excluded; the current glossary and Taskfile metadata have
changed. The reviewed planning subtree, contracts, relevant glossary entries,
Taskfile and release/check scripts were therefore read directly. The complete
planning artifact was read from its committed bodies and root diff, not inferred
from graph coverage. These are bounded planning findings. No runtime result,
current external Bun claim or passing implementation verification is asserted.
