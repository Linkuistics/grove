# supplemental-evaluation-k92

**Reviews:** supplemental-evaluation-k77

## Goal

Adversarially audit the committed replacement-campaign decomposition before any
instrument, apparatus, or live-model child runs.


## Context

- Producer artifact: `supplemental-evaluation-k77` and the complete
  `paired-acceptance-campaign-k80` subtree it creates.
- Requirements authority and access amendment:
  `acceptance-replication-authority-k76`.
- Claim boundary and historical constraints:
  `acceptance-contract-reconciliation-k75`.
- Protocol findings to re-derive rather than trust:
  `evaluation-recovery-k73` and `evaluation-recovery-k74`.

## Done when

- Try to disprove that the manifest boundary is complete and immutable before
  execution, that acceptance and supplemental namespaces cannot leak into one
  another, and that historical records cannot be relabelled, pooled, rescored,
  repaired, or erased.
- Try to disprove treatment delivery: every enabled context must receive the
  verified final bytes before the byte-identical user prompt, every control must
  receive no skill, preload failure must invalidate the apparatus, and delivered
  non-adherence must remain a valid outcome.
- Try to disprove each independent intake, source/fragment, and
  exposition/assurance verdict and their non-compensating AND endpoint,
  including comparative thresholds, absolute floors, exhaustive regression
  rows, mixed-control handling, and fail-closed incomplete evidence.
- Re-derive F1-F14 against the actual task order and ownership. In particular,
  verify the all-generation-before-any-adjudication barrier, deterministic
  replacement replay, bounded non-selective pair-atomic resumption, adjacent
  pairs, criterion-author separation, dual scoring of every scored bundle,
  scorer/resolver ownership, and post-score arm guesses.
- Challenge whether each leaf is an independently useful working increment and
  whether the documented human procedure plus small deterministic helpers is
  proportionate. Identify automation that lacks an owner and automation that
  adds cost without protecting acceptance.
- Challenge the decision not to commission a new transfer probe and the claim
  that ten new `fzf` repetitions do not earn their cost under the amended
  parent contract.
- Record severity-ordered findings against the producer commit without editing
  the producer task or campaign subtree. If actionable findings exist,
  commission a lazy `integrate-review-planning` step with this bare stem
  immediately before `paired-acceptance-campaign-k80`; if none exist, retire
  without creating one.
- Launch no evaluated treatment or control context.

## Notes

Assume the plan is overconfident. Prefer a smaller valid campaign, but reject a
simplification that removes an acceptance-critical seam or turns unavailable
evidence into attainment.

## Findings

Reviewed against the `supplemental-evaluation-k77` commit `bf832bb82203` and the
committed `paired-acceptance-campaign-k80` subtree it creates. No producer file
and no historical evaluation byte was edited. Sixteen findings, severity-ordered
and lettered `G1`–`G16` so they do not collide with the `F1`–`F14` of
`evaluation-recovery-k73`, which they cite.

The plan is substantially correct: the manifest boundary, the acceptance /
supplemental split, the generation-before-adjudication barrier, exhaustive
regression rows, fail-closed direction, dual scoring, adjacent pairs and the
non-compensating AND are all present and correctly placed. The findings below
are the seams where an execution session still has room to move, or where an
historical failure mode is re-imported by silence rather than by decision.

### G1 — "Preload" names two different mechanisms, and only one of them delivers the treatment (high)

The amended contract's whole premise is that treatment is assigned by
*guaranteed delivery* rather than by post-hoc discovery selection
(`acceptance-replication-authority-k76`: "before the model answers, the
execution procedure preloads the verified final skill bytes into every enabled
context"). The subtree implements that sentence twice, incompatibly:

- `campaign-apparatus-k82`: "Control homes contain no target skill; enabled
  homes preload the manifest-pinned bytes before the prompt. Preload manifests
  and prompt digests prove delivery."
- `treatment-verification-k83`: "Sealed control and enabled templates …
  Removing only the target skill subtree from the enabled template makes their
  declared content identical."

A *home*, a *template* and a *preload manifest* are filesystem objects. Bytes on
disk are not bytes in a context. On every harness this repository actually
drives, an installed skill's body reaches the model only when something reads
it — which is a tool call, and which the historical Case A prompt forbids.
That is exactly the structural conflict the frozen report diagnoses at
`docs/evaluations/writing-code-walkthroughs/README.md:247`-`251`: "the frozen
no-tool prompt conflicts structurally with observable file-based skill loading:
visible use invalidates an enabled attempt, selecting the retained arm against
the treatment it is meant to measure. Closing this gap requires a new
predeclared prompt or access rule, not another run under the frozen contract."

If the campaign runs under the filesystem reading, it re-imports that defect
wholesale and the amendment bought nothing. This repository has already recorded
that the receipt is unobtainable by observation:
`docs/adr/skill-delivers-the-methodology.md` rejects a machine-checked read
receipt "because Grove cannot observe what a session reads", and rejects
harness-specific injection as launch policy — both reopen conditions unmet.

Only two designs are coherent, and the plan must name one in the manifest:

(a) **In-context injection.** The verified bytes are included verbatim in the
    request payload that precedes the user prompt. The delivery receipt is then
    a digest of the *submitted payload*, not of a directory listing, and the
    per-surface tool-access rule becomes irrelevant to delivery.
(b) **On-disk installation.** The receipt is an observed read in the preserved
    raw event stream, and the instrument must then *permit* that read and must
    not classify it invalid — reversing the historical rule.

**Change:** `acceptance-instrument-k81` states the delivery channel as part of
the delivery contract; `campaign-apparatus-k82` defines the preload receipt as
an artifact of the chosen channel (payload digest under (a), event-stream
evidence under (b)); `campaign-freeze-k84` pins it. Recommended: (a) — it is the
only reading under which "a failed or mismatched preload invalidates the
apparatus" is a decidable test rather than an assertion about a filesystem.

### G2 — The phase-only replacement rule turns carrier failure into unattainable evidence (high)

`campaign-apparatus-k82` makes replacement "a deterministic function of
preserved exposure phase": pre-exposure failures consume the budget, "The first
post-exposure outcome is retained." Every generation leaf repeats it. Combined
with the fail-closed rule (F8 of the parent chain, applied at `k88`–`k90`),
a post-exposure event that yields no behavioral evidence — a transport reset, a
harness crash, a wall-clock kill, an apparatus-side interface breach — is
retained as a non-answer and then fails its surface closed.

This is stricter than the instrument it replaces and reproduces the exact deaths
already on the record. The frozen rubric made carrier failures replaceable
regardless of phase (`baseline/rubric.md:108`-`112`: replaceable when "the
process exits nonzero, emits an explicit failed/cancelled response status, emits
no final assistant message …"; and `:113`-`114` "A wall-clock termination emits no
final answer and is therefore replaceable"). Even so, historical Case B ended
`0/5` valid after 15 DNS/transport timeouts (`README.md:66`) and the transfer
probe ended with 20 of 21 attempts breaching the model-interface boundary
(`README.md:187`). Under the new phase-only rule none of those 35 attempts would
have been replaceable, and all three surfaces would be dead on a single flaky
afternoon.

The axis F2 chose (*when* the failure happened) is a proxy for the axis that
matters (*whose* failure it was). F2 chose it because attribution looked like a
judgement call — but attribution is machine-decidable from evidence the
apparatus already preserves.

**Change:** freeze a three-way outcome taxonomy keyed on preserved,
machine-checkable signals rather than on phase alone — *carrier failure* (no
assistant token emitted, or a harness-level nonzero exit / transport error with
no model output), *behavioral outcome* (any emitted model output, including
refusal, omission, non-adherence, and a model-initiated boundary violation), and
*apparatus failure* (preload absent or mismatched). Carrier failures are
replaceable under the same global budget and the same non-selective pair-atomic
resumption as pre-exposure failures; behavioral outcomes are retained
unconditionally. The auditor replays the taxonomy over the full history exactly
as it replays the phase rule today, so nothing is returned to operator
discretion.

### G3 — The freeze is immutable and also scheduled for review-and-integration; the two rules contradict (high)

`campaign-freeze-k84` states both:

- "No evaluated output predates the freeze commit. Any later change to a frozen
  byte invalidates this cycle and requires a new predeclared manifest."
- "commission a lazy `review-impl` sibling … Execution waits for that review and
  any earned integration."

An integration that acts on a finding necessarily changes a frozen byte, so by
the first rule it invalidates the cycle the second rule was added to protect.
Nothing states which governs. An execution session reading this has a live
incentive to read the review as advisory.

The reversal is also unexplained. `evaluation-recovery-k74` **declined** F14 as a
recorded trade-off ("A scheduled review leaf would add stronger temporal
separation, but … forcing every finding through a later integration session
before the manifest can be signed"). `supplemental-evaluation-k77` reverses it
("F14 earns a scheduled `review-impl`") without stating what changed. Reversing
an integrated disposition is legitimate; doing it silently is not, and here the
reversal is precisely what creates the contradiction.

**Change:** state the amendment rule explicitly — because no evaluated context
has run, a pre-execution amendment is not a cycle invalidation: it re-freezes,
producing a **new manifest digest**, and the no-evaluated-output-predates rule
binds against the *final* manifest commit rather than the first. Record why F14
is now taken where `k74` declined it.

### G4 — Nothing requires the endpoint to be shown attainable before it is frozen (medium-high)

`acceptance-instrument-k81` fixes, per surface, "sample size … binary scoring,
material comparative threshold, and absolute enabled-performance floor", and
`k91` applies the AND. That is six gates over three surfaces. No leaf requires
anyone to demonstrate, *before* the freeze, that the frozen sample can reach
them.

The historical instrument failed this test by construction, not by behavior: the
report's own verdict is that the primary endpoint "is unreachable under every
completion of the missing fifth enabled scope-elicitation sample" and the `A14`
guard "fails under every completion" (`README.md:5`-`10`). `evaluation-recovery-k73`
F6 raised the same gap as "the plan states no minimum-detectable-change", and
`k74`'s F6 disposition answered only the mixed-row half. The unanswered half is
inherited here verbatim.

The total cost is also invisible. Three surfaces × n pairs × 2 arms is the whole
budget of the campaign and no leaf states n, so neither this review nor the
freeze reviewer can judge proportionality against the alternative the parent
requires (`acceptance-replication-authority-k76`: "When a bounded action is
simpler … ask the human to perform that action"). The in-repo benchmark is
`docs/research/wording-micro-test.md` — 30 sessions, three arms, two targets, a
documented human procedure, no reusable harness — which is the shape `k76`
describes and which the plan never compares itself to.

**Change:** `acceptance-instrument-k81` publishes, as part of the draft manifest,
(i) an attainability statement per surface showing the frozen n, threshold and
floor admit a pass under a stated best case, and (ii) the total evaluated-context
count for the campaign. A threshold that cannot be met by any completion of the
frozen sample is a design defect to fix before freezing, not a result to
discover afterwards.

### G5 — The scoring prompt, the scorer runtime and the disagreement rule have no author (medium-high)

`campaign-freeze-k84` pins "scoring prompts" into the manifest.
`campaign-apparatus-k82` invokes scorers and says "A separately owned blind
resolver applies only the frozen disagreement rule". `acceptance-instrument-k81`
authors rows, samples, thresholds and floors — and does not mention a scoring
prompt, a scorer runtime, or a disagreement-resolution procedure. Nothing
authors them.

This is F9's shape one level in. F9 named the missing *invocation harness* and
`k77` assigned it to `k82`; but the old plan's `k55` also owned "the scorer
instrument, blind tie-resolution procedure, scorer prompt and runtime"
(`evaluation-recovery-k73` F9), and that ownership was dropped rather than
reassigned. It matters more than an ordinary handoff gap because the scoring
prompt is where the frozen rows become operative criteria: whoever writes it
writes the instrument. If it is written by `campaign-apparatus-k82`, which
carries none of `k81`'s input restrictions, F5's role separation is defeated at
the only point where it changes a score.

**Change:** assign the scoring prompt, the scorer runtime declaration and the
blind disagreement-resolution procedure to `acceptance-instrument-k81`, under
the same restricted-input, preserved-chronology regime as the rows. `k82` keeps
invocation and `k84` keeps pinning.

### G6 — `k74`'s replenishment disposition is not carried into this subtree (medium-high)

`campaign-apparatus-k82` says pre-exposure failures "consume the manifest's
global resource budget" and describes resumption *order*. It never says what
happens when the budget is exhausted with assignments still incomplete.

`evaluation-recovery-k74` settled this as Doubt D3 — "Resource replenishment is
automatic, globally bounded, and pair-atomic; it occurs only between pairs, and
a mid-pair exhaustion becomes unavailable rather than separating the arms." Only
the pair-atomic and ordering halves survive into `k80`. Without replenishment, a
"global resource budget" is a hard ceiling on infrastructure flakiness — which
is the thing F3 removed, for the reason F3 gave: "a ceiling on it cannot prevent
selection — it can only convert infrastructure flakiness into an unreachable
endpoint."

**Change:** restate the exhaustion rule in `k81`/`k82` — automatic, globally
bounded replenishment between whole pairs, with a stated terminal condition —
so exhaustion is a predeclared outcome rather than an unstated default.

### G7 — Criterion-author isolation is still asserted rather than checkable (medium-high)

`acceptance-instrument-k81` requires that the criterion author "receives no skill
bytes, historical rubric rows, historical outputs, or new campaign outcomes",
and preserves "exact inputs, raw output, runtime identity, chronology, and
digests". Preserving inputs proves what was *handed over*; it does not bound
what was *reachable*. The criterion-author session runs in a checkout that
contains `plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md` and
`docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

That is verbatim the defect F5 raised against `measurement-design-k55` — "an
unverifiable assertion by one session that already has the deployed `SKILL.md`
in its checkout" — and F5's own standard was that the information flow be
"checkable rather than asserted". `k81` improves the preservation half and
leaves the exclusion half asserted.

The fix costs nothing new: `treatment-verification-k83` already builds sealed,
manifest-verified templates whose declared content is enumerated, and `k82`
already builds an access audit over raw events. Run the criterion author in one.

**Change:** `acceptance-instrument-k81` runs its criterion author in a sealed
workspace containing only the permitted inputs, with the same access audit the
evaluated contexts get, and preserves that audit as the isolation evidence.

### G8 — No stop-or-continue rule after a terminal protocol failure in generation (medium)

Each generation leaf "publishes a terminal complete, protocol-failed, or
unavailable raw surface record". Under the non-compensating AND, a
protocol-failed intake surface already determines the campaign result. Nothing
says whether `source-fragment-generation-k86` and
`exposition-assurance-generation-k87` then run.

Silence here is not merely a cost question. It hands the operator a choice —
continue or stop — to be exercised *with knowledge of an adverse outcome on a
previous surface*, which is the class of channel F1 and F2 exist to close. F1's
own words: the freeze "is complete for *artifacts* and empty for *discretion*".
There is a defensible answer either way (continuing yields records useful to a
later cycle; stopping saves the spend), but it must be frozen, not chosen in
flight.

**Change:** freeze the rule in the manifest — recommended: continue, because the
remaining raw records retain value for a later predeclared cycle and continuing
removes the discretionary branch entirely.

### G9 — Arm labels are revealed to scorers before two of the three surfaces are scored (medium)

`intake-adjudication-k88`: "Guess accuracy is revealed and reported as a
supplemental limitation after score records are sealed." Sealing is per surface,
and `k88` precedes `k89` and `k90`. Nothing requires scorer contexts to be fresh
per surface or forbids reuse. If any scorer identity persists across
adjudications, `k88`'s reveal trains it on the arm signature before it scores the
next two surfaces — converting the blind that F10 was added to *measure* into one
that later surfaces no longer have.

**Change:** either require a fresh scorer and resolver context per surface, or
defer every arm-label reveal until after `exposition-assurance-adjudication-k90`
seals its score records. Recommended: both — they are independently cheap.

### G10 — New campaign artifacts have no declared home, and historical immutability is never mechanically checked (medium)

Immutability of the historical record is asserted in six places across `k77`,
the `k80` brief, `k84` and `k91` and enforced nowhere. No leaf names where the
new campaign's manifest, raw records, score records and report are written, and
the obvious destination — `docs/evaluations/writing-code-walkthroughs/` — is the
historical tree itself, whose `README.md` carries the failed verdict the new
verdict must not overwrite or appear to amend.

**Change:** name a distinct destination path for every new campaign artifact,
and add to `campaign-freeze-k84` and `three-surface-verdict-k91` a recorded
digest sweep over `docs/evaluations/writing-code-walkthroughs/**` — captured at
freeze, reverified at the verdict — so "the historical bytes are unchanged" is
an executed check rather than a promise. This is one deterministic helper and it
protects the single property the whole `k75`/`k76` chain was cut to preserve.

### G11 — The external non-Rust fixture has no vendoring owner (medium)

`acceptance-instrument-k81` freezes the fixture and
`source-fragment-generation-k86` verifies its digest before launch. No leaf owns
*obtaining* it. The historical analogue was `targets/ocaml/check_floor.ml` from a
different repository, and the historical failure on that surface was network:
`0/5` valid after 15 DNS/transport timeouts (`README.md:66`). A fixture fetched
at run time re-imports precisely that dependency into the surface it already
killed.

**Change:** name the owner — `acceptance-instrument-k81` — and require the
external fixture to be vendored into the repository and digest-pinned before the
freeze, so no evaluated run performs a network retrieval.

### G12 — A failed verdict silently authorizes an unbounded next cycle (medium)

`three-surface-verdict-k91`: "If any does not [pass], the parent remains open."
`campaign-freeze-k84`: a frozen-byte change "invalidates this cycle and requires
a new predeclared manifest." Together these permit campaign after campaign at
the same treatment digest until one passes, with no multiplicity accounting —
which is the hazard the entire `acceptance-contract-reconciliation-k75` /
`acceptance-replication-authority-k76` chain exists to control, since this
campaign is *already* the retry of a failed one and needed an explicit
requirements decision to be legitimate.

`k76` authorized "one separately frozen, requirement-derived paired instrument".
The subtree never states that this is that one instrument, nor what a fail means
for a successor.

**Change:** the manifest states the retry contract for this cycle: one accepted
campaign per treatment digest, and any further campaign at the same digest is
supplemental unless a new requirements decision authorizes it. A treatment
revision produces a new digest and is a different question.

### G13 — `campaign-apparatus-k82` is not one focused session, and one of its helpers protects nothing (medium-low)

`k77` calls the apparatus "deliberately bounded" and then enumerates nine
deterministic subsystems — "digest and prompt equality, delivery checks,
assignment, exposure classification, replacement replay, resumption order,
record validation, bundle formation, and verdict calculation" — plus a
synthesized fixture suite covering preload mismatch, every exposure and
replacement branch, resource exhaustion and resumption, pair-atomic scheduling,
prohibited access, missing/truncated finals, bundle redaction, two-scorer
invocation and blind resolution, plus scorer and resolver invocation, plus an
access-audit path, plus a bounded operator procedure. That is a comprehensive
reusable harness described as its opposite, in one leaf. It is a `leaf-decompose`
candidate and is not marked as one.

Separately, the charter asks which automation adds cost without protecting
acceptance. It is **verdict calculation**. Every other helper on that list
protects a property that is unobservable after the fact — that assignment was
precommitted, that resumption did not consult outcomes, that redaction was
deterministic, that both scorers were invoked. A verdict is arithmetic over
per-row counts that `k88`–`k91` must publish in full anyway
(`k88`: "publishes all per-row arm counts"), so any reader re-derives it from
the published table and the frozen formula. Automating it protects nothing and
adds a component that must itself be trusted.

**Change:** mark `k82` as an expected `leaf-decompose` candidate with the seam
named — deterministic execution core (delivery receipt, assignment, outcome
classification, replacement replay, resumption, record validation) versus
adjudication support (bundle formation and redaction, scorer/resolver
invocation, access audit) — and drop the verdict-calculation helper in favour of
published per-row counts plus the frozen formula.

### G14 — The frozen `fzf` probe is a null result and is labelled as evidence (low-medium)

The `k80` brief and `three-surface-verdict-k91` both carry the frozen `fzf`
probe forward as "supplemental generality evidence". The record says otherwise:
"Twenty of 21 attempts violated the declared model-interface boundary. The
remaining enabled attempt was a valid refusal. With `0/5` valid controls and
`1/5` valid enabled samples, atomic comparative scoring and a transfer verdict
are undefined. The transfer claim is **not established**" (`README.md:187`-`191`).

The probe is evidence of an apparatus failure, not of generality. This matters
for the charter's question about ten new repetitions: the decision **not** to
commission a new transfer probe is correct, but only on `k77`'s *second* reason —
transfer is not an acceptance conjunct under `k76`, and no result could change
the frozen skill during this campaign. The first reason, that the existing probe
already supplies generality evidence, is false, and if it stands unqualified
`k91` will report a null result as a finding.

**Change:** `k91` reports the `fzf` probe as an undefined transfer verdict with
its 20/21 interface-breach rate, not as generality evidence, and rests the
no-new-probe decision on the acceptance-conjunct reason alone.

### G15 — F11's "irregular or incomplete" clause survives in one adjudication leaf of three (low)

`intake-adjudication-k88` requires two blind scores for "Every scored bundle,
including an irregular or incomplete one". `source-fragment-adjudication-k89`
and `exposition-assurance-adjudication-k90` say only "Every scored bundle". The
dropped clause is the whole of F11: historically the single-scored arms were
exactly the irregular ones — the enabled Case A arm and the Case B contemporary
control.

**Change:** restore the clause in `k89` and `k90`.

### G16 — There are two namespaces where the campaign needs three (low)

Acceptance and `supplemental` are both frozen at `k84`. Post-hoc analyses —
which by definition arrive after outcomes are known — have no third home, so
they will be written into the frozen `supplemental` namespace and inherit its
pre-registration credibility. Freezing is the property that makes `supplemental`
readable as "declared in advance, just not acceptance-critical"; a post-hoc
analysis filed there is neither.

**Change:** name a third, explicitly post-hoc `exploratory` namespace, opened
after the verdict, so a favourable after-the-fact analysis cannot present itself
as pre-registered.

## Decisions (running log)

The review produced actionable findings, so the lazy `integrate-review-planning`
step is earned. Per `references/decompose.md` the target is the first sibling
entry after this leaf whose subtree still holds live work: that is the node
`paired-acceptance-campaign-k80`, not a leaf inside it. Inserting at the node's
slot is what keeps every campaign child — starting with
`acceptance-instrument-k81` — from running before the findings land.

No evaluated treatment or control context was launched, and no in-session
reviewer was materialised: a `review-*` session spends none of the allowance
because it is itself the adversarial read.

Findings are recorded against the committed producer artifact only. No file in
`paired-acceptance-campaign-k80`, no historical evaluation record, and no frozen
rubric byte was edited by this session.

G1 is the finding that decides whether the campaign is worth running at all: if
delivery is filesystem installation, the amended contract's premise fails and
the campaign measures discovery a second time. G2, G6 and G8 are the three
places where an historical failure mode returns through silence rather than
through a decision. G3 must be settled before the freeze leaf is written, since
it governs what that leaf's own review can do. The transfer question in the
charter is answered in G14: the no-new-probe decision stands, its stated reason
does not.
