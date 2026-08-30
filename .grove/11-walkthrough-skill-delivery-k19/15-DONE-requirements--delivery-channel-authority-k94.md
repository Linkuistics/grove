# delivery-channel-authority-k94

## Goal

Decide whether an available execution channel can make guaranteed treatment
delivery machine-checkable for the replacement paired campaign, and either
authorize that exact channel or leave behavioral acceptance open rather than
substituting a filesystem or command-line proxy for delivery.

## Context

- Parent delivery amendment: `acceptance-replication-authority-k76` and the
  amended `walkthrough-skill-delivery-k19` brief chain.
- Finding that exposed the unresolved mechanism: `supplemental-evaluation-k92`
  G1 and its integration at `supplemental-evaluation-k93`.
- Campaign consumer: `paired-acceptance-campaign-k80`, which must not begin
  `acceptance-instrument-k81` until this decision and its review chain settle.
- Existing product decision: `docs/adr/skill-delivers-the-methodology.md`, whose
  rejection of an observable skill-read receipt and harness-specific Grove
  launch policy does not itself authorize a weaker evaluation receipt.

## Done when

- Ask the human to choose between provisioning a direct authenticated request
  channel whose exact effective request body and provider acknowledgement can
  be preserved, identifying another harness channel that emits an authoritative
  loaded-instruction receipt, or leaving the amended behavioral claim open.
  Recommend the direct request channel; a captured CLI argument, installed
  directory, or asserted file read is not sufficient evidence.
- For any authorized channel, pin the provider/runtime identity, request schema,
  exact treatment-placement field, acknowledgement/stream event that proves
  acceptance, freshness rule, error taxonomy, and receipt artifacts. State how
  the channel detects ignored, normalized, truncated, rejected, or partially
  delivered treatment before an outcome can bear acceptance authority.
- Define the treatment transport without rewriting the deployable skill into
  evaluation-specific instructions. Specify exhaustive recursive manifest
  coverage, treatment-file ordering and boundaries, admissible file types and
  encoding, symlink/non-regular handling, maximum payload size, and the exact
  status of any framing bytes visible to the model.
- Fix per-surface model-interface access at the same seam: intake and
  exposition/assurance expose no tools; source/fragment exposes exactly one
  manifest-bound read operation for the vendored fixture, with no caller-
  supplied path, traversal, symlink escape, mutation, or network retrieval.
- Make freshness checkable: each arm is one new request containing no prior
  message or resumable session identifier, and its preserved body enumerates
  the complete system, user, tool, fixture, and treatment inputs.
- If no available channel meets those conditions, amend no delivery claim and
  run no campaign. Record that the behavioral conjunct remains open until a
  checkable channel is provisioned; do not let `paired-acceptance-campaign-k80`
  reinterpret command construction or filesystem state as receipt evidence.
- Update the parent acceptance wording if and only if the chosen answer changes
  what guaranteed delivery means. Give `paired-acceptance-campaign-k80` an
  exact stable-handle handoff it can pin without making a new requirements
  choice.
- Launch no evaluated treatment or control context. A bounded non-evaluated
  transport probe may establish interface mechanics only if the human
  authorizes the required credential or channel.
- As the final planning action, commission a lazy `review-requirements` sibling
  with bare stem `delivery-channel-authority`, inserted immediately before
  `paired-acceptance-campaign-k80`, chartered to try to disprove receipt
  authority, treatment identity, tool confinement, and freshness. Any findings
  must settle through an earned `integrate-review-requirements` step before the
  campaign runs.

## Notes

The current local CLI exposes a system-prompt append flag, but preserving argv
does not prove the effective request received by the model and does not detect
CLI normalization, truncation, or an ignored flag. No direct provider API
credential is present in this session's environment. Those observations justify
the requirements gate; they do not decide which credential or provider the
human should authorize.

## Decisions (running log)

No execution channel is authorized for the replacement paired campaign. The
human delegated the choice among channels already available to the producing
session, and that session took the fail-closed branch. Review integration
corrects the evidence supporting that branch without changing its value.

The multi-provider client's non-refreshing readiness check reports
`credentials_not_configured` for the direct Anthropic, OpenAI, and Google
providers and reports OAuth readiness for `openai-codex`. Readiness establishes
that a credential exists and nothing about campaign authority or receipt
quality. No credential, including the Codex OAuth credential, has been
authorized for this campaign.

A bounded non-evaluated Claude CLI probe also ran. With a replacement system
prompt, a fresh non-persistent request, `tools = []`, no MCP servers, safe mode,
and no settings sources, its stream recorded a fresh session ID, an Anthropic
message ID and request ID, the resolved model, empty tool and MCP declarations,
and provider-computed usage. It did not record the accepted request body or a
provider digest of it. The response charged 317 input tokens for a tiny explicit
system and user prompt, which is direct evidence that the harness stream does
not enumerate the whole effective model input. The stream is useful mechanics
evidence and is not a delivery receipt.

Current provider documentation matches the measurement. Anthropic documents a
unique request ID for correlation, but not an echo or digest of the effective
request; it also documents that API request transformation makes usage counts
non-isomorphic to visible request content. Its token-count endpoint is an
estimate that may include provider-added tokens. OpenAI likewise documents
`x-request-id` as a troubleshooting identifier, not as an acknowledgement of
request contents. Consequently, no inspected hosted channel satisfies the
sufficient-evidence classes below.

Primary-source record: [Anthropic request IDs](https://platform.claude.com/docs/en/api/errors),
[Anthropic Messages usage semantics](https://platform.claude.com/docs/en/api/typescript/messages),
[Anthropic token counting](https://platform.claude.com/docs/en/build-with-claude/token-counting),
[Anthropic prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching),
and [OpenAI request IDs](https://platform.openai.com/docs/api-reference/authentication).

This leaves the amended behavioral-acceptance conjunct open. The stable handoff
is `delivery-channel-authority-k94: authorization = none`. Until a later
requirements decision replaces that value, `paired-acceptance-campaign-k80` and
all of its children must not run. In particular, `acceptance-instrument-k81`
must not reinterpret an installed directory, an asserted or observed file read,
a captured CLI argument, a client-computed prompt digest, or an unauthenticated
local transcript as delivery evidence.

The parent acceptance wording is unchanged. This decision does not weaken or
redefine guaranteed delivery; it records that the currently identified channels
cannot evidence the definition already in force. The historical failed campaign
and its Case A access limitation remain contrary evidence, and no new treatment,
control, scorer, or resolver context has run.

The unresolved provisioning-or-closure choice now has a live owner:
`delivery-channel-provisioning-authority-k97`, positioned before
`paired-acceptance-campaign-k80`. That requirements producer must obtain the
human's choice; the blocked campaign is no longer the next session forced to
escalate a decision outside its charter.

Criterion-author isolation does not require an evaluated-context delivery
receipt. For that non-evaluated planning context, a sufficient seam is an
ephemeral VM, container, or enforced OS sandbox whose filesystem namespace
mounts only the frozen permitted inputs plus enumerated read-only runtime
dependencies, denies host-root, parent-path, and undeclared mount access, and
limits network access to the one model endpoint. Inside that boundary the
harness uses a replacement system prompt, no declared tools, no MCP servers,
safe/no-customization mode, no settings sources, and no persistent or resumed
session. Preserve the sandbox configuration, mount and permitted-input
manifests, network policy, invocation, fresh session identifier, init event
showing the empty client-visible tool and MCP sets, raw output, runtime identity,
and chronology. This makes repository bytes outside the namespace unreachable
and makes any output-side tool use visible. It does not claim that the init
event is a provider receipt. `supplemental-evaluation-k92` G7 therefore remains
**applied differently**, through this planning-only isolation seam; it must not
be reused as evidence for an evaluated treatment or control.

### Review disposition

- **H1 — applied.** The reopening contract below states a sufficient evidence
  set and one operationally attainable class: a controlled self-hosted runtime
  in which the operator owns and can independently observe the complete
  bytes-to-tokens-to-inference boundary. A hosted final-boundary attestation is
  sufficient only if a provider later exposes it; no inspected provider does.
  The parent conjunct remains pending until
  `delivery-channel-provisioning-authority-k97` chooses self-hosted provisioning
  or explicit closure.
- **H2 — applied differently.** G7 stays applied through the weaker,
  planning-only isolation seam above. It no longer depends on a delivery channel
  that this decision did not authorize.
- **H3 — applied.** `delivery-channel-provisioning-authority-k97` is the live
  human-authority path immediately before the blocked campaign.
- **H4 — applied differently.** The readiness checks replace the unsupported
  absence claim with the narrower facts above: three direct-provider entries are
  not ready, the Codex OAuth entry is ready, and none is campaign-authorized.
- **H5 — applied.** The bounded mechanics probe ran and established both the
  empty client-visible interface and the absence of an effective-request
  receipt.
- **H6 — applied differently.** Anthropic documents byte-identical prefix
  matching and cache-read token reporting. A hit proves equality to a prior
  provider-accepted prefix, not that either prefix equals the frozen treatment.
  Caching is priced as supporting mechanics evidence and declined as acceptance
  authority; evaluated requests remain cache-free and fresh.
- **H7 — applied differently.** The manifest now closes over local outbound
  references by classification. The current plugin provenance link is a frozen,
  declared external reference rather than an accidentally omitted treatment
  file; whether deployed semantics require delivering it is re-owned by
  `delivery-channel-provisioning-authority-k97` with the frontmatter decision.
- **H8 — applied differently by externalization.** Whether source/fragment
  receives an inline fixture with zero tools or a pathless one-use tool changes
  the intervention. `delivery-channel-provisioning-authority-k97` owns that
  choice and its review chain; this integration does not decide it silently.
- **H9 — applied.** The fixture receives the same byte, path, framing, size,
  digest, arm-equality, and preserved-input rules as every other model-visible
  input, regardless of the delivery choice for H8.
- **H10 — applied differently by externalization.** Raw YAML frontmatter is not
  instruction text under ordinary deployed loading. Choosing raw-file delivery
  or deployed body semantics changes the intervention, so
  `delivery-channel-provisioning-authority-k97` owns it and its review.
- **H11 — applied.** A hard-link alias is not an admissibility failure when the
  logical path and delivered bytes are fixed. A non-text asset is declared
  excluded, carried through a separately specified model-visible part, or
  reopens requirements; it never silently disqualifies the product.
- **H12 — applied.** The pinned request identity now includes the complete
  sampling and output configuration.
- **H13 — applied.** The manifest records the actual frozen shape and byte count;
  multi-file rules apply only when that shape exists.

### Reopening contract

A later requirements leaf may authorize a channel only when it demonstrates one
of these sufficient evidence classes:

1. **Provider final-boundary attestation.** The service operating the actual
   inference boundary issues an attestation after every input transformation and
   immediately before inference, correlated with that inference's output by a
   server-generated request identifier. It returns the complete canonical
   pre-tokenization input bytes and final token-ID sequence, or digests of both
   plus the fully disclosed deterministic normalization, chat-template,
   tokenizer, and special-token rules needed to derive and independently
   recompute both expected digests from the frozen request. An upstream client,
   proxy, or gateway receipt is insufficient unless that component also operates
   and observes the final inference boundary.
2. **Self-hosted observed model boundary.** A pinned self-hosted runtime exposes
   an independently captured canonical pre-tokenization byte stream and final
   token-ID sequence at the last boundary before inference, correlated with the
   request and output by an authenticated runtime receipt. The model weights,
   runtime build, normalization, chat template, tokenizer build and digest,
   special-token rules, and sampling configuration are pinned. The verifier
   independently derives the expected canonical bytes and token sequence from
   the frozen request and matches both before admitting output.

The self-hosted class is operationally attainable because the operator controls
the complete transformation and inference boundary, although no qualifying
runtime is provisioned here. The provider class is a sufficient specification,
not an existence claim: no inspected hosted provider exposes it. A hosted
request ID, message ID, response body, usage count, token-count estimate,
cache-hit count, client-captured request or digest, upstream gateway receipt,
CLI argument, local file-read event, or harness init event may support diagnosis
but is not sufficient alone or in aggregate without one of the two final-boundary
bindings above. If the human declines self-hosted provisioning and cannot name a
provider that meets the first class, a requirements decision must close the
parent behavioral conjunct explicitly rather than leave it indefinitely pending.

The authorized interface also pins all of the following:

- The provider, endpoint, model snapshot, runtime, API/schema version, and
  authenticated client identity. Every request transformation is either included
  in the bound effective request or makes the channel ineligible.
- The exact serialized client request, its digest, and the content-bound receipt
  from one sufficient evidence class above.
- A treatment-placement field whose provider contract makes it model-visible
  before the user message. Normalization is admissible only when the bound
  effective bytes or digest match the frozen expectation. Truncation must be
  disabled; an endpoint that silently truncates or ignores the field is
  ineligible.
- The complete sampling and output configuration: temperature, top-p, top-k
  where supported, stop sequences, reasoning/thinking mode and budget, seed or
  determinism controls where supported, maximum output tokens, service tier,
  fallback policy, and every provider-specific generation option. Values are
  frozen across the whole campaign as well as equal between arms.
- A receipt verifier that classifies schema rejection, authentication failure,
  rate or transport failure, provider refusal, incomplete delivery, stream
  interruption, treatment mismatch, and tool-boundary mismatch before an
  outcome receives acceptance authority. A valid delivery receipt precedes the
  behavioral boundary; after it, every model output remains a behavioral
  outcome rather than a replaceable delivery failure.
- Preserved artifacts containing the exact redacted request body, its digest,
  the treatment and fixture manifests, the explicit tool declaration, provider
  response headers, acknowledgement or raw stream event, raw output events,
  runtime identity, chronology, and the verifier's machine-readable verdict.
  Redaction may remove only authentication secrets and must not change any
  model-visible byte.

An alternative harness channel is eligible only when its underlying provider or
self-hosted runtime supplies one of the two sufficient bindings. Harness events
may preserve mechanics and chronology; they cannot promote themselves into an
authoritative receipt for instructions actually loaded into the model request.

### Treatment transport

The deployable skill remains the treatment; the evaluation must not rewrite it
into campaign-specific instructions. At the producer and review commits the
skill directory contains one regular file, `SKILL.md`, of 13,940 bytes, plus a
local reference from that file to the plugin-level `../../PROVENANCE.md`
outside the directory. Freeze remeasures rather than trusting those historical
values.

The treatment manifest records the actual recursive file shape, every relative
path, byte length, and SHA-256 digest, and the total byte count. `SKILL.md` is
first and any remaining paths follow in unsigned-byte lexical order. Multi-file
ordering and boundary rules are conditional on more than one delivered file.
Every local outbound reference is classified as delivered, deliberately
external, or non-load-bearing. A load-bearing reference may be deliberately
external only when the manifest names the resulting deployed-versus-delivered
difference; no reference disappears merely because it crosses the skill
directory boundary. The current provenance link is such a declared external
reference until `delivery-channel-provisioning-authority-k97` settles the
deployed-semantics treatment definition.

Text parts contain complete valid UTF-8 without NUL bytes. Paths are relative
UTF-8 POSIX paths with no empty, dot, dot-dot, or absolute component. Symlinks,
devices, sockets, FIFOs, and other non-regular entries reopen requirements. A
hard-link alias is admissible when each logical path and its delivered bytes are
manifested; link count is not model-visible evidence. A non-text asset is either
carried through a separately frozen model-visible part, declared excluded with
the deployed-versus-delivered difference recorded, or reopens requirements.
File bytes are carried unchanged within their declared part: no newline,
Unicode, encoding, or whitespace normalization is permitted. The total unframed
treatment is capped at 65,536 bytes; exceeding the cap reopens requirements
rather than inviting truncation. The complete request must also reserve the
frozen output allowance within the pinned model's context limit.

The authorized channel must define unambiguous per-file boundaries by structured
parts or length-prefixed framing. Every framing byte and its order are frozen.
If framing is model-visible, it is recorded separately from the deployable-skill
digest and declared part of the enabled intervention; it may identify paths and
boundaries but may not add semantic instructions. A channel that cannot state
whether framing is model-visible is ineligible. Controls omit the treatment
field and all treatment framing; the frozen effective-request comparison must
show that treatment assignment is the only arm difference outside declared
arm identifiers that are not model-visible.

### Model-interface and freshness seams

Intake and exposition/assurance requests declare an explicit empty tool list.
For source/fragment, `delivery-channel-provisioning-authority-k97` must choose
between an inline fixture with the same empty tool list and one strict,
zero-argument `read_fixture` operation whose availability is itself part of the
measured intervention. The instrument must state whether choosing to call that
operation is scored behavior. Until that choice and its review settle, no
channel is authorized. If retained, the operation accepts no path or locator,
may succeed at most once, cannot traverse or follow symlinks, and exposes no
mutation, process, filesystem, network, or caller-selected retrieval surface.
The authoritative receipt must bind the chosen declaration and the fixture
manifest to the same effective request as the treatment.

Every fixture file follows the treatment's byte, path, framing, encoding,
non-text, size, and digest rules. The fixture manifest and total request budget
are frozen, and the fixture bytes are identical between arms. Whether inline or
tool-returned, the receipt must enumerate the fixture as model-visible input.

The same producer must decide whether the enabled intervention carries raw
`SKILL.md` bytes including YAML frontmatter or reproduces deployed semantics in
which frontmatter is metadata and only the body is instruction text. The
manifest records the raw-file digest, the exact extraction or framing rule, and
every model-visible byte. This is a measurement choice, not a transport detail,
and therefore remains outside this integration.

Each arm begins with one new top-level request. It contains no prior assistant or
user message, previous-response identifier, conversation identifier, resumable
session identifier, provider cache reference, or hidden harness history. Prompt
caching is disabled for evaluated requests: an exact-prefix cache hit is useful
mechanics evidence but imports state from a prior request and does not establish
treatment identity. Its preserved
body and attached manifests enumerate the complete system, user, treatment,
tool, and fixture inputs. A channel whose fixture operation requires a second
client request carrying prior messages or a resumable identifier does not meet
this freshness contract.

The agreed test seams for a future implementation are: recursive manifest
enumeration and byte-for-byte revalidation; outbound-reference classification;
exact treatment and fixture framing and arm-delta comparison; sufficient-class
receipt correlation and effective-request verification; frontmatter-status and
fixture-access enforcement; explicit empty-tool or reviewed pathless-fixture
enforcement; freshness and no-cache validation over the complete request;
sampling-configuration equality; and deterministic replay of the receipt error
taxonomy. These seams match the expectation that delivery, confinement, and
freshness fail before behavioral evidence is admitted, while post-receipt
non-adherence remains measured behavior.
