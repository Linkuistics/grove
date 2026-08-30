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
human delegated the choice to this requirements session, and the session takes
the fail-closed branch. The direct-provider option has no provisioned credential
or provider receipt to inspect. The identified local CLI can preserve installed
files and argv, but those artifacts prove only filesystem and command
construction state. No identified harness emits an authoritative receipt that
binds the exact effective model request, treatment bytes, declared tools, and
freshness to the provider-accepted request. A bounded transport probe was not
run because there is no authorized credential or channel to probe.

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

### Reopening contract

A later requirements leaf may authorize a channel only after it provisions and
pins all of the following as one checkable interface:

- The provider, endpoint, model snapshot, runtime, API/schema version, and
  authenticated client identity. A moving model alias or undocumented request
  transformation is insufficient.
- The exact serialized request bytes and a provider-issued acknowledgement,
  correlated by a server-generated request identifier, that binds the accepted
  effective request or its provider-computed digest. Client-side argv, object,
  or digest capture is not the acknowledgement.
- A treatment-placement field whose provider contract makes it model-visible
  before the user message, plus an acknowledgement status that distinguishes
  accepted-exactly from ignored, normalized, truncated, rejected, and partially
  delivered input. Normalization is admissible only when the provider returns
  the effective bytes or their digest and they match the frozen expectation.
  Truncation must be disabled; an endpoint that silently truncates or ignores
  the field is ineligible.
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

An alternative harness channel is eligible under the same contract only if its
receipt is authoritative for instructions actually loaded into the model
request. A harness assertion that it attempted a preload, or an event showing a
local file open, does not cross that boundary.

### Treatment transport

The deployable skill remains the treatment; the evaluation must not rewrite it
into campaign-specific instructions. At freeze, recursively enumerate the
verified final skill directory. The treatment manifest contains every relative
path, byte length, and SHA-256 digest. `SKILL.md` is first and all remaining
paths follow in unsigned-byte lexical order. Missing or additional paths fail
the manifest check.

Only regular files whose complete contents are valid UTF-8 without NUL bytes are
admissible. Paths are relative UTF-8 POSIX paths with no empty, dot, dot-dot, or
absolute component. Symlinks, hard-link aliases, devices, sockets, FIFOs, and
other non-regular entries make the treatment ineligible. File bytes are carried
unchanged: no newline, Unicode, encoding, or whitespace normalization is
permitted. The total unframed treatment is capped at 65,536 bytes; exceeding the
cap reopens requirements rather than inviting truncation. The complete request
must also reserve the frozen output allowance within the pinned model's context
limit.

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
The source/fragment request declares exactly one strict, zero-argument operation,
`read_fixture`, whose result is the exact regular-file set pinned by the vendored
fixture manifest. The operation accepts no path or locator, may succeed at most
once, cannot traverse or follow symlinks, and exposes no mutation, process,
filesystem, network, or caller-selected retrieval surface. The provider receipt
must bind this declaration and the fixture manifest to the same accepted request
as the treatment.

Each arm begins with one new top-level request. It contains no prior assistant or
user message, previous-response identifier, conversation identifier, resumable
session identifier, cache reference, or hidden harness history. Its preserved
body and attached manifests enumerate the complete system, user, treatment,
tool, and fixture inputs. A channel whose fixture operation requires a second
client request carrying prior messages or a resumable identifier does not meet
this freshness contract.

The agreed test seams for a future implementation are: recursive manifest
enumeration and byte-for-byte revalidation; exact treatment framing and arm-delta
comparison; provider acknowledgement correlation and effective-request digest
verification; explicit empty-tool and pathless-fixture enforcement; freshness
validation over the complete request; and deterministic replay of the receipt
error taxonomy. These seams match the expectation that delivery, confinement,
and freshness fail before behavioral evidence is admitted, while post-receipt
non-adherence remains measured behavior.
