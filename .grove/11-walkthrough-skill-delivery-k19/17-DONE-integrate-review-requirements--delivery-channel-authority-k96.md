# delivery-channel-authority-k96

**Integrates:** delivery-channel-authority-k95

## Goal

Settle the `delivery-channel-authority-k95` findings against the
`delivery-channel-authority-k94` decision, so that the value
`authorization = none` is either re-established on evidence that supports it or
replaced, and so that whichever it becomes, the tree carries a path forward from
it rather than a leaf that can only escalate.

## Context

- Findings, verbatim and severity-ordered: the `## Findings` section of
  `delivery-channel-authority-k95`, read from its own commit. They are
  deliberately not restated here.
- Decision being corrected: `delivery-channel-authority-k94`, whose running log,
  reopening contract, treatment transport, and model-interface and freshness
  seams are the artifact under repair.
- Requirements authority and amended acceptance wording:
  `acceptance-replication-authority-k76` and `walkthrough-skill-delivery-k19`.
- Upstream findings the review cites rather than re-derives:
  `supplemental-evaluation-k92` `G1`, `G4` and `G7`, and their dispositions in
  `supplemental-evaluation-k93`.
- Blocked consumer: `paired-acceptance-campaign-k80` and its whole subtree,
  which must not run while the authority value stands at none.
- Product decision that constrains delivery reasoning without settling it:
  `docs/adr/skill-delivers-the-methodology.md`.

## Done when

- Every finding is classified as applied, applied differently, or declined with
  a reason recorded against the parent contract. Declining one is a legitimate
  outcome that the running log must argue rather than assert.
- The unperformed checks are performed before any finding that depends on them
  is dispositioned. Two are one command each and neither is a transport probe:
  the provisioned multi-provider client's own credential-readiness check, and a
  bounded non-evaluated mechanics probe through an already-authenticated local
  harness. The reviewing session was denied both by its sandbox and recorded
  them as unperformed rather than as results. If this session is denied them
  too, say so and hand them to the human as a bounded action rather than
  reasoning past them a second time.
- The reopening contract states what evidence is **sufficient**, not only what
  is disqualifying, or else records that no runtime class available here can
  satisfy it and that the parent behavioral conjunct is closed rather than
  pending. A bar with no attainability statement is the defect
  `supplemental-evaluation-k92` `G4` raised against the campaign endpoint and
  `supplemental-evaluation-k93` applied; it must not survive one level up.
- Whatever the authority value becomes, the tree carries a live path from it.
  If it stays none, the question that could change it is put to the human by a
  leaf that exists, positioned so the walk reaches it before
  `paired-acceptance-campaign-k80`. If it becomes an authorized channel, the
  campaign's own consumption of it is exact enough that
  `acceptance-instrument-k81` makes no new requirements choice.
- The disposition of `supplemental-evaluation-k92` `G7` is restated explicitly,
  because this decision withdrew the seam that `supplemental-evaluation-k93`
  applied it through. Either name a mechanism that isolates a non-evaluated
  criterion-author context without a provider receipt, or record `G7` as
  unapplied and name where it is re-owned.
- Findings that repair transport, confinement, or freshness rules land as edits
  to the requirements decision, not as campaign wording. Any finding whose
  repair would change what the campaign measures is externalized as a new
  producer leaf with its own review chain rather than absorbed here.
- No historical evaluation record, no frozen rubric byte, and no file inside
  `paired-acceptance-campaign-k80` is edited by this session, and no evaluated
  treatment, control, scorer, or resolver context runs.

## Notes

Read the findings from the review's own commit. They are the reviewer's list and
this session's charter is the decision's correctness, not the list's completion.

The findings interact and should be judged together rather than in file order.
Three of them are one question: whether `authorization = none` is a fact about
this machine, a fact about hosted providers, or a fact about a bar written
without checking that anything could clear it. Answer that first; the transport,
confinement and freshness findings are repairs to a contract whose scope that
answer decides.

Two findings pull against each other on purpose. One argues that prefix caching
is the only provider-side equality oracle over effective request bytes and that
the freshness clause bans it as a side effect; the other argues the single
confined fixture tool costs the whole confinement axis for one observable bit.
Both trade a stated protection for stated evidence, and both are places where
this chain has previously preferred the protection without pricing the evidence.
Price it this time.

The reviewing session did not fetch provider documentation, by design — a review
that established the mechanics would have been doing this session's work. Confirm
the caching and acknowledgement mechanics against current provider documentation
before acting on any finding that turns on them.

Delegation from the human covered choosing among the options as they stood. It
did not cover whether to provision a channel that does not yet stand, and that
is the only question whose answer can change the value. If this session cannot
answer it, it belongs back with the human as a named leaf, not as a sentence in
a running log.

## Decisions (running log)

`authorization = none` remains the current value. Credential readiness and
channel eligibility are separate from campaign authority: the provisioned
multi-provider client reports direct Anthropic, OpenAI, and Google providers not
ready, while `openai-codex` is OAuth-ready; no listed credential is authorized
for this campaign.

The bounded Claude mechanics probe succeeded with an explicit empty tool list,
no MCP servers, a replacement system prompt, safe mode, no settings sources, and
no session persistence. Its stream carried a provider request ID, message ID,
model, fresh session ID, and usage, but no effective request body or provider
digest. The unexpectedly large effective input-token count demonstrates hidden
or transformed input rather than proving the explicit prompt's identity.

Current Anthropic and OpenAI documentation confirms that request IDs are
correlation and support identifiers, not content acknowledgements. Anthropic's
usage counts are not one-to-one with visible request content, its token count is
an estimate that may include provider-added tokens, and its prompt cache requires
identical prefix segments but binds only equality with a prior cached prefix.
None of those quantities proves that the prefix is the frozen treatment.

The parent contract now states two sufficient final-boundary evidence classes: a
provider attestation over independently recomputable canonical input bytes and
token IDs immediately before inference, or an independently captured canonical
byte stream and token sequence from a pinned self-hosted runtime. Only the
self-hosted class is asserted operationally attainable, because its operator
owns the complete transformation and inference boundary; it is not provisioned
here. The provider class is sufficient if a provider later exposes it, not an
existence claim. Hosted request IDs, usage, cache hits, upstream gateway
receipts, client digests, argv, and local reads remain supporting evidence only.

H1–H13 are classified in `delivery-channel-authority-k94`'s corrected running
log. Transport, fixture, sampling, confinement, and freshness repairs that do
not change the intervention land there. The two measurement choices—fixture
inlining versus a one-use tool, and raw frontmatter versus deployed loading
semantics—are re-owned by the new requirements producer
`delivery-channel-provisioning-authority-k97`, with its own lazy review chain.

`supplemental-evaluation-k92` G7 remains applied differently through a
planning-only ephemeral VM, container, or enforced OS sandbox. Its filesystem
namespace mounts only frozen permitted inputs and enumerated read-only runtime
dependencies, denies host and undeclared mounts, and restricts network access to
one model endpoint. The harness has a replacement system prompt, no tools or MCP
servers, no settings or customizations, and no resumable session; the sandbox
configuration, mount/input manifests, network policy, init event, raw output,
runtime, and chronology are preserved. This seam is sufficient to bound local
criterion-author reachability; it is explicitly not a delivery receipt for
evaluated contexts.

The in-session doubt reviewer found six substantive defects in the first repair:
the provider receipt was too early, a digest over undisclosed additions was
circular, the self-hosted class conflated bytes with token IDs, an upstream
gateway could not attest the inference boundary, the isolation seam named only a
directory, and the text overstated two-class attainability. All six were valid
and actionable. The corrected contract now requires final-boundary timing,
independently recomputable canonical bytes and token sequences, pinned
tokenizer/template/special-token behavior, and an enforced filesystem/network
boundary; it claims current attainability only for controlled self-hosting.

The human provisioning-or-closure decision is now live as
`delivery-channel-provisioning-authority-k97`, immediately before
`paired-acceptance-campaign-k80`. The campaign subtree remains byte-unchanged and
must not run while `authorization = none`.
