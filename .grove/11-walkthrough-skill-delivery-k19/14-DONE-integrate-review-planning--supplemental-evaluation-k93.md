# supplemental-evaluation-k93

**Integrates:** supplemental-evaluation-k92

## Goal

Apply the `supplemental-evaluation-k92` findings to the
`paired-acceptance-campaign-k80` decomposition before `acceptance-instrument-k81`
runs, so the campaign that executes cannot deliver an unverifiable treatment,
cannot convert infrastructure failure into an unattainable endpoint, and cannot
be amended or repeated into a favorable result.

## Context

- Findings, verbatim and severity-ordered: the `## Findings` section of
  `supplemental-evaluation-k92`, read from its own commit.
- Producer plan being corrected: `supplemental-evaluation-k77` and the
  `paired-acceptance-campaign-k80` subtree it created.
- Requirements authority and amended acceptance wording:
  `acceptance-replication-authority-k76` and `walkthrough-skill-delivery-k19`.
- Claim boundary: `acceptance-contract-reconciliation-k75`.
- Prior findings the review re-derives rather than trusts: F1-F14 in
  `evaluation-recovery-k73` and their dispositions in `evaluation-recovery-k74`.
- Immutable evidence the findings are grounded in:
  `docs/evaluations/writing-code-walkthroughs/README.md` and
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

## Done when

- Every finding is classified as applied, applied differently, or declined with
  a reason recorded against the parent contract. A finding is not obligatory
  because it was written down; declining one is a legitimate outcome that the
  running log must argue rather than assert.
- The delivery-channel finding is settled first and explicitly, because the
  remaining shape depends on it: name the mechanism by which verified treatment
  bytes reach an enabled context, name the artifact that evidences delivery, and
  state the per-surface tool-access rule that mechanism implies. If no available
  channel makes delivery machine-checkable, say so and externalize the
  requirements question rather than weakening the delivery claim.
- Any finding whose repair changes what the campaign measures — rather than how
  a task file states it — is externalized as a new producer leaf beside this one
  with its own review chain, not absorbed here.
- Fixes land in the `.grove/` task files and briefs of the campaign subtree. No
  historical evaluation record, no frozen rubric byte, and no committed campaign
  result is edited, and no evaluated treatment or control context runs.
- Where a finding restores a disposition that `evaluation-recovery-k74` settled
  and this subtree dropped, the restoration cites that disposition so the two
  cannot diverge again.
- Ownership gaps are closed by naming an owning leaf, never by adding the work
  to whichever leaf is nearest.
- Any leaf the findings show to be more than one focused session is decomposed
  or marked as an expected `leaf-decompose` candidate with its seam named.
- After integration, `acceptance-instrument-k81` remains the first live leaf of
  the campaign and no execution child has run.

## Notes

Read the findings from the review's own commit; they are deliberately not
restated here, so that rejecting one is not rejecting this leaf's charter.

The review states which findings it considers load-bearing and why in its
running log. Treat that ordering as the reviewer's argument, not as a verdict:
the campaign's shape is this session's call against the parent contract.

Several findings interact and should be judged together rather than in file
order. The delivery mechanism decides what a delivery receipt can be and what
the tool-access rule must say. The outcome taxonomy, the resource-exhaustion
rule, and the stop-or-continue rule are three faces of one question — which
failures the campaign is allowed to retry, and who decides — so applying one
without the others leaves the channel open. The freeze-immutability and
amendment rules govern what the freeze candidate's own scheduled review is able
to do, so they must be settled before `campaign-freeze-k84` is rewritten.

## Decisions (running log)

- **G1 — applied differently by externalizing the requirements question.** A
  narrow fresh-context challenge showed that preserved CLI arguments prove
  command construction, not the effective model request, and leave prompt
  normalization, truncation, tool confinement, freshness, and transport framing
  asserted. This environment exposes no direct provider credential with which
  to replace that proxy. `delivery-channel-authority-k94` is therefore inserted
  before the campaign to choose a request channel with an authoritative receipt
  or leave behavioral acceptance open; the campaign cannot reinterpret an
  installed directory, file read, or CLI argument as delivery evidence.
- **G2 — applied.** The phase proxy is replaced by a machine-signal taxonomy:
  carrier failure has no model output and is replaceable; any model output is a
  retained behavioral outcome; authoritative delivery-receipt or manifest mismatch is
  a campaign-wide apparatus failure. The full history is replayed against that
  taxonomy before scoring.
- **G3 — applied.** A pre-execution review integration may repair the candidate
  only by emitting a complete replacement manifest with a new digest, and
  execution binds to the final reviewed manifest commit. After that boundary a
  frozen-byte change invalidates the cycle. F14 is deliberately reversed from
  `evaluation-recovery-k74`: this is the sole acceptance-authorized campaign per
  treatment digest, and its new delivery seam makes temporal review worth the
  extra review chain.
- **G4 — applied.** The draft instrument must publish each surface's sample
  size, a best-case calculation showing both gates are attainable, and the total
  evaluated control/treatment context count before freeze.
- **G5 — applied.** `acceptance-instrument-k81` owns the scoring prompt, scorer
  runtime declaration, and blind disagreement rule under its restricted-input
  chronology. `campaign-apparatus-k82` owns invocation only; the freeze pins the
  authored bytes.
- **G6 — applied.** The plan restores `evaluation-recovery-k74` Doubt D3:
  globally bounded replenishment is automatic, predeclared, and pair-atomic,
  occurs only between pairs, and has a final-window terminal condition. Budget
  is reserved by surface so one unavailable surface cannot create a choice
  about whether later surfaces run.
- **G7 — applied differently.** Reusing `treatment-verification-k83`'s later
  templates would invert the tree dependency. Instead the criterion author must
  use the sealed tool-free fresh-request seam authorized by
  `delivery-channel-authority-k94`; its effective request, empty tool declaration,
  provider acknowledgement, raw output, runtime, chronology, and digests make
  both what was supplied and what was reachable checkable.
- **G8 — applied.** Generation continues through every scheduled surface after
  a preceding surface is protocol-failed or unavailable. Only shared apparatus
  invalidity stops later submission, removing the outcome-aware operator branch.
- **G9 — applied.** Each surface uses fresh scorer and resolver contexts, and no
  labels or arm-guess accuracy are revealed until all three surface score
  records are sealed.
- **G10 — applied.** Every new artifact has an owning path under
  `docs/evaluations/writing-code-walkthroughs-paired-acceptance/`. Freeze records
  a recursive path-and-digest manifest of the historical evaluation tree and
  the final verdict replays it before synthesis.
- **G11 — applied.** `acceptance-instrument-k81` owns obtaining and vendoring the
  external non-Rust fixture with origin, revision, path, and digest. Evaluated
  runs use only the vendored copy and perform no network retrieval.
- **G12 — applied.** The manifest and final report state that this is the one
  acceptance-authorized campaign for its treatment digest. A further same-
  digest campaign is supplemental unless a new requirements decision grants it
  acceptance authority; changing treatment digest opens a new requirements
  question rather than an automatic retry.
- **G13 — applied differently.** `campaign-apparatus-k82` is marked as an
  expected `leaf-decompose` candidate at the execution-core versus adjudication-
  support seam, which lets its own session decompose only when the implementation
  proves too large. Verdict calculation is removed from helper ownership;
  published per-row counts plus frozen formulas keep the arithmetic re-derivable.
- **G14 — applied.** The historical `fzf` probe is an undefined transfer verdict
  after 20 of 21 interface breaches, not generality evidence. No new probe is
  commissioned because transfer is not an acceptance conjunct and cannot change
  this campaign's frozen treatment.
- **G15 — applied.** `source-fragment-adjudication-k89` and
  `exposition-assurance-adjudication-k90` now repeat F11's load-bearing rule that
  every irregular or incomplete scored bundle receives two blind scores.
- **G16 — applied.** The manifest now distinguishes frozen `acceptance`, frozen
  predeclared `supplemental`, and post-verdict `exploratory` namespaces; the last
  carries neither pre-registration credibility nor verdict authority.

The one permitted narrow doubt cycle challenged only the proposed delivery
seam. Every one of its nine findings was valid and actionable against that
proposal: argv capture did not prove the effective prompt; visible framing could
change treatment identity; manifest exhaustiveness was undefined; argv could
not carry arbitrary or unbounded bytes; an empty-tools flag was unverified; the
single-fixture boundary was asserted rather than enforced; safe-mode precedence
over prompt appending was unknown; CLI identity did not pin request semantics;
and context freshness lacked a receipt. `delivery-channel-authority-k94` now
owns the corresponding nine requirements: authoritative effective-request
evidence, framing status, exhaustive manifest rules, encoding/size limits,
explicit tool declarations, a pathless fixture operation, channel precedence,
provider/runtime/schema identity, and a no-history fresh-request proof. No
second doubt cycle is needed because the challenged CLI proposal was removed
rather than mechanically revised.

G1 cannot be repaired as campaign wording without weakening the parent delivery
claim, so its precise requirements question is externalized to
`delivery-channel-authority-k94` with its own required review chain. Every other
finding is a repair to evidence handling, ownership, or campaign wording and
does not change the three behavioral surfaces or their parent-authorized
conjunctive endpoint. No evaluated treatment, control, scorer, or resolver
context ran in this integration.
