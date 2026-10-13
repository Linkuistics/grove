# Grove provider evidence through Ravel

Grove's optional `assets/ravel-policy/provenance.ts` wraps the owner's final
policy using Ravel 0.1.0's public `Policy`/`Selected` API. It uses no Ravel store,
record verb, private protocol or run lookup. Ravel supplies only routing and
process supervision. The readable Grove review adapter consumes this contract for creator-aware routing.

```typescript
import { policy as owner } from "./owner-policy.ts";
import { withProvenance } from "/absolute/path/to/grove/assets/ravel-policy/provenance.ts";
export const policy = withProvenance(owner);
```

Opt in once around the final policy, after all route selection and modifiers.
The wrapper preserves its version, context loader, selected labels/reason and
refusals. It awaits its selection and constructs evidence from the returned
`provider`, never from a kind table, executable name or model string. It launches
`/bin/bash --noprofile --norc -p -c` with a fixed script and literal argv. The
script exports fresh `GROVE_PROVIDER_EVIDENCE` and `exec --`s the original program
and arguments in the same process group, with Ravel's ending authority. `-p`
prevents ambient startup files, imported functions and shell options from running
before exec. No prompt, original argument, working directory or environment grant
is rewritten. A refusal produces no managed process or evidence.

The wrapper changes the selected executable to Bash. Confined callers must
explicitly supply `--runtime-read` for the original executable and any existing
runtime files it needs. Bash must exist at `/bin/bash`. Settings, bounds and
owner environment grants remain in Ravel's configuration; the wrapper raises
none. Its extra argv consumes the existing message/OS argument budget, so a
selection already near those limits can refuse. Receipts have a separate 8192
UTF-8-byte bound. Nonblank labels must be single-line and NUL-free. These are
explicit costs of the opt-in wrapper; the owner's personal routes and grants
are not automatically edited. Nonwrapped policies supply no recorded evidence.

## The record and its lifetime

The managed process receives one canonical, compact, single-line JSON record:

```json
{"schemaVersion":1,"attemptId":"12345678-1234-4123-8123-123456789abc","provider":"origin-a","policyVersion":"owner-version","kind":"impl","taskId":"child-k3"}
```

Fields appear in this order; `taskId` is absent when the public request has none.
`attemptId` is a fresh UUID v4 for each successful policy selection. The provider
is the owner's model-origin label at that selection. The receipt attests selection
of the managed command; it does not establish a verified backend, successful work
or a naturally completed session. Its availability in the managed process comes
from launch. The UUID distinguishes attempts; it is neither ending authority nor
a Ravel identity. The request identity and policy version provide attribution,
without including executable data in a generic context.

At retirement the finishing session copies its own receipt byte for byte onto
every live review of that producer, directly under `**Reviews:**`:

```markdown
**Reviews:** parent-k1
**Creator:** recorded {"schemaVersion":1,"attemptId":"12345678-1234-4123-8123-123456789abc","provider":"origin-a","policyVersion":"owner-version","kind":"impl","taskId":"child-k3"}
```

The task commit pins the inline record; there is no separately stored receipt or
path to resolve. Review retries read the same committed bytes, independent of
subsequent changes to producer routing. A producer retry which actually finishes
replaces earlier evidence with that attempt's record. An attempt leaving its leaf
live or a node open writes nothing. For a node close, the closing child's record
stands for the whole node, so its `taskId` need not equal the reviewed node's
handle. One close cascade copies the same record to each producer it finishes.
Terminal reviews and unrelated/prefix-similar handles are untouched.

The owner policy and finishing session are trusted same-user participants. The
provider label and association with an artifact are attestations, not signatures.
A session can edit its environment or a review file; measured reads pin bytes,
not truth. There is no attempt lookup to authenticate the association. Do not
reconstruct it from current routing, other tasks, version history or an old store.

## Consumption contract for the Grove review adapter

Use public `loadContext(request, host)` and `host.readText(request.taskFile)`.
Parse exactly one whole-line `**Reviews:**` relationship and at most one Creator
line directly beneath it. For `recorded`, call `parseEvidence` on the suffix.
It accepts only the canonical record above, required nonblank fields, UUID v4,
optional nonblank `taskId`, no unknown fields, duplicate keys, newlines or NULs,
and at most 8192 bytes. Never compare the receipt's `kind` or `taskId` with the
producer's current mapping/handle to guess its provider: node-close attribution
is deliberately a child's selection.

Deliver a schemaVersion-1 context with `facts.creator` containing
`{ evidence: "recorded", provider, receipt }`. Attribute the task read in
`sources`; Ravel adds the canonical source path, bytes and SHA-256 to `measured`.
For the owner's `**Creator:** declared <provider>`, deliver
`{ evidence: "declared", provider }`, retaining that evidence label. The suffix
is a nonblank single-line origin, not inferred from a command. `assets/ravel-policy/grove.ts` implements this loader and
`examples/review.ts` validates these facts before selecting a reviewer.
`facts.reviewedArtifact` contains `{ id }`, the reviewed handle (or an external
artifact ID for generic callers). Grove refuses caller-supplied review facts
that conflict with its task. Other facts/assessments/sources are retained.
The provider must be an exact own key in the review kind's origin table and the
selected reviewer must have another provider. A gateway keeps its model origin.
Unknown origins refuse without guessing or choosing a fallback.

Missing Creator is unknown: refuse with an owner-declaration remedy. Duplicate,
malformed, noncanonical, unsupported or oversized records refuse with a repair
remedy; never silently reinterpret them as declared or use stale evidence.
`**Creator:** run <id>` is explicitly legacy and unsupported by the new consumer:
refuse with a remedy to have the owner declare the provider or finish again
under a receipt-producing policy. Never open the original dispatch store; current Ravel has none.

## Original installed-v22-driver sessions

This grove's original launch prompt still names `harness-dispatch exit`, and its
installed Grove 22.1.0 driver retains its original admission/teardown authority.
Only a session explicitly launched under that original contract writes
`**Creator:** run <HARNESS_DISPATCH_RUN_ID>` from its own environment. That form
is consumed only by the original legacy policy/store, not the new adapter.
A Ravel session uses its fresh receipt; absent evidence clears an earlier Creator
and leaves the owner declaration pending. Invalid evidence stops for repair
before retirement is committed. It must not fall back to legacy or stale data.
Nothing in this document changes a running session's ending binary/channel.

## Using the review policy

Import `assets/ravel-policy/examples/grove-review.ts` normally. Its final policy
uses `withProvenance`, so creator receipt production and review consumption work
together. Owners can compose `groveReviewSelector({ routes, reviews })` with
custom routes and wrap their final policy once. The illustrative routes/models
remain owner-edited starting points; static sample choices stay unchanged.
The generic `examples/review.ts` selector is also usable without Grove, from
explicit JSON context. Its receipt object must have the exact schema/values
above and agree with the separate `facts.creator.provider`; missing or invalid
facts refuse. JSON transport may reorder object keys, so this path reconstructs
canonical order before shared validation. Canonical input encoding and duplicate
JSON keys are checked on raw Grove task lines, not already-parsed generic objects. No executable data is accepted in this context.

## Verification

`task ravel:review RAVEL=/absolute/installed/ravel` checks strict installed-SDK
types, real installed-worker production/parser fixtures, actual Grove lifecycle
sessions with fake agents, and inherited-evidence isolation. Save complete logs
with `set -o pipefail; task ... 2>&1 | tee target/k42/review.log`.
The old bundled dispatch package and its compatibility task were removed at
Grove's delivery contraction. Current checks consume the independent installed
Ravel pair. No test launches a model or edits the owner policy.
