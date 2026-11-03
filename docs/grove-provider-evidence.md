# Grove provider evidence through Ravel

Grove's optional `assets/ravel-policy/provenance.ts` wraps the owner's final
policy through Ravel 0.1.0's public `Policy`/`Selected` API. It uses no Ravel
store, record verb, private protocol or run lookup. Ravel supplies only routing
and process supervision. The readable Grove review adapter consumes this
contract for creator-aware routing. On Ravel 2.x, `provenance.rhai` wraps the
owner's final selection in the same way. [Runtimes](#runtimes) states what the
two sets share.

```typescript
import { policy as owner } from "./owner-policy.ts";
import { withProvenance } from "/absolute/path/to/grove/assets/ravel-policy/provenance.ts";
export const policy = withProvenance(owner);
```

Opt in once around the final policy, after all route selection and modifiers.
The wrapper preserves the version, the context loader, the selected labels and
reason, and the refusals of that policy. It awaits the selection of the policy.
It constructs the evidence from the returned `provider`, never from a kind
table, an executable name or a model string. It launches
`/bin/bash --noprofile --norc -p -c` with a constant script and a literal argv.

The script exports a fresh `GROVE_PROVIDER_EVIDENCE`. Then it runs `exec --` on
the original program and arguments in the same process group, with Ravel's
ending authority. `-p` prevents ambient startup files, imported functions and
shell options from running before the exec. The wrapper rewrites no prompt,
original argument, working directory or environment grant. A refusal produces
no managed process and no evidence.

The wrapper changes the selected executable to Bash. Confined callers must
explicitly supply `--runtime-read` for the original executable and any existing
runtime files it needs. Bash must exist at `/bin/bash`.

Settings, bounds and owner environment grants remain in Ravel's configuration.
The wrapper raises none of them. Its extra argv consumes the existing
message/OS argument budget, so a selection already near those limits can
refuse. Receipts have a separate bound of 8192 UTF-8 bytes. Nonblank labels must
be single-line and NUL-free, and [Runtimes](#runtimes) defines a blank label.

All of these are explicit costs of the opt-in wrapper. Nothing edits the
owner's personal routes and grants automatically. Nonwrapped policies supply no
recorded evidence.

## Runtimes

Grove ships the wrapper and the review loaders for both admitted majors of
Ravel. `provenance.ts`, `grove.ts` and the TypeScript examples run on Ravel 1.x.
`provenance.rhai`, `grove.rhai`, `examples/review.rhai` and
`examples/grove-review.rhai` run on Ravel 2.x. A receipt that one runtime's
wrapper records reaches a review that the other runtime may load. Both sets
therefore keep these rules:

- **Receipt bytes.** Both wrappers write the same bytes for a selection. The
  members keep the constant order below. The Rhai wrapper joins members that
  `ravel::json_encode` wrote, and that encoder writes the text of
  `JSON.stringify` for every string that Rhai can hold.
- **A blank label.** A label is blank when ECMAScript's
  `String.prototype.trim` empties it. That trim removes 25 characters: U+0009
  to U+000D, U+0020, U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F,
  U+205F, U+3000 and U+FEFF. U+0085 and U+180E are not in the set. The Rhai
  helpers test this set themselves, because the `trim` of Rhai removes U+0085
  and keeps U+FEFF.
- **The checks.** The receipt, label and command checks of the wrapper, the
  review loader and the review examples give the same outcome in both
  runtimes.

One difference remains. A JavaScript string can hold an unpaired surrogate, and
a Rhai string cannot. A receipt can escape one as `\ud800`. The TypeScript
loader accepts that receipt, and Ravel 1.x then refuses the delivered context
as `context_invalid`, because JSON cannot carry the string. The Rhai loader
refuses the receipt itself as `creator_line_malformed`. Neither runtime
launches a reviewer, and the owner's declaration of the provider is the
remedy.

Ravel's own checks apply after the wrapper. Ravel of both majors checks that an
owner version and a selected provider are nonblank by Rust's whitespace, which
counts U+0085. So no wrapper records a version or a provider of U+0085 alone.

`crates/grove/tests/ravel_asset_cases.rs` holds one case table for both sets.
Its wrapper cases state the receipt bytes, and its loader cases read the same
bytes, so each runtime reads what the other writes.

## The record and its lifetime

The managed process receives one canonical, compact, single-line JSON record:

```json
{"schemaVersion":1,"attemptId":"12345678-1234-4123-8123-123456789abc","provider":"origin-a","policyVersion":"owner-version","kind":"impl","taskId":"child-k3"}
```

The fields appear in this order. `taskId` is absent when the public request has
none. `attemptId` is a fresh UUID v4 for each successful policy selection. The
provider is the owner's model-origin label at that selection.

The receipt attests the selection of the managed command. It does not establish
a verified backend, successful work or a naturally completed session. The
launch makes the receipt available in the managed process. The UUID
distinguishes attempts. It is neither ending authority nor a Ravel identity.
The request identity and policy version provide attribution, without executable
data in a generic context.

At retirement the finishing session copies its own receipt byte for byte onto
every live review of that producer, directly under `**Reviews:**`:

```markdown
**Reviews:** parent-k1
**Creator:** recorded {"schemaVersion":1,"attemptId":"12345678-1234-4123-8123-123456789abc","provider":"origin-a","policyVersion":"owner-version","kind":"impl","taskId":"child-k3"}
```

The task commit pins the inline record. There is no separately stored receipt
and no path to resolve. Review retries read the same committed bytes,
independent of later changes to the producer's routing. A producer retry that
actually finishes replaces the earlier evidence with the record of that
attempt. An attempt that leaves its leaf live or a node open writes nothing.

For a node close, the closing child's record stands for the whole node, so its
`taskId` need not equal the reviewed node's handle. One close cascade copies the
same record to each producer that it finishes. The copy leaves terminal reviews
and unrelated or prefix-similar handles untouched.

The owner policy and the finishing session are trusted same-user participants.
The provider label and its association with an artifact are attestations, not
signatures. A session can edit its environment or a review file. Measured reads
pin bytes, not truth. There is no attempt lookup to authenticate the
association. Do not reconstruct it from current routing, other tasks, version
history or an old store.

## Consumption contract for the Grove review adapter

Use public `loadContext(request, host)` and `host.readText(request.taskFile)`
on Ravel 1.x. On Ravel 2.x, use `fn load_context(request)` and
`ravel::read_text(request.taskFile, request.limits.contextBytes)`.
Parse exactly one whole-line `**Reviews:**` relationship and at most one Creator
line directly beneath it. For `recorded`, call `parseEvidence` on the suffix.
It accepts only a record with all of these properties:

- It is the canonical record above.
- Its required fields are nonblank.
- Its UUID is a UUID v4.
- Its `taskId` is optional, and nonblank when present.
- It has no unknown fields, no duplicate keys, no newlines and no NULs.
- It has at most 8192 bytes.

Never compare the receipt's `kind` or `taskId` with the producer's current
mapping/handle to guess its provider. Node-close attribution is deliberately a
child's selection.

Deliver a schemaVersion-1 context with `facts.creator` that contains
`{ evidence: "recorded", provider, receipt }`. Attribute the task read in
`sources`. Ravel adds the canonical source path, bytes and SHA-256 to
`measured`. For the owner's `**Creator:** declared <provider>`, deliver
`{ evidence: "declared", provider }`, and keep that evidence label. The suffix
is a nonblank single-line origin, not an origin inferred from a command.

`assets/ravel-policy/grove.ts` implements this loader. `examples/review.ts`
checks these facts before it selects a reviewer. `facts.reviewedArtifact`
contains `{ id }`. The `id` is the reviewed handle, or an external artifact ID
for generic callers. Grove refuses caller-supplied review facts that conflict
with its task. It retains the other facts, assessments and sources.

The provider must be an exact own key in the review kind's origin table. The
selected reviewer must have another provider. A gateway keeps its model origin.
Unknown origins refuse without a guess and without a fallback choice.

On Ravel 2.x, `grove.rhai` implements the loader and `examples/review.rhai`
checks the facts.

Treat a missing Creator as unknown, and refuse with an owner-declaration remedy.
Duplicate, malformed, noncanonical, unsupported or oversized records refuse with
a repair remedy. Never silently reinterpret them as declared, and never use
stale evidence.

`**Creator:** run <id>` is explicitly legacy, and the new consumer does not
support it. Refuse it with a remedy: have the owner declare the provider, or
finish again under a receipt-producing policy. Never open the original dispatch
store. Current Ravel has none.

## Original installed-v22-driver sessions

The original launch prompt of this grove still names `harness-dispatch exit`.
Its installed Grove 22.1.0 driver retains its original admission/teardown
authority. Only a session explicitly launched under that original contract
writes `**Creator:** run <HARNESS_DISPATCH_RUN_ID>` from its own environment.
Only the original legacy policy/store consumes that form. The new adapter does
not consume it.

A Ravel session uses its fresh receipt. Absent evidence clears an earlier
Creator and leaves the owner declaration pending. Invalid evidence stops for
repair before the retirement commit. It must not fall back to legacy or stale
data. Nothing in this document changes a running session's ending
binary/channel.

## Using the review policy

On Ravel 1.x, import `assets/ravel-policy/examples/grove-review.ts` normally.
Its final policy uses `withProvenance`, so creator receipt production and
review consumption work together. Owners can compose
`groveReviewSelector({ routes, reviews })` with custom routes and wrap their
final policy once. The illustrative routes and models remain initial versions
that the owner edits. The static sample choices stay unchanged.

The generic `examples/review.ts` selector is also usable without Grove, from an
explicit JSON context. Its receipt object must have the exact schema and values
above, and it must agree with the separate `facts.creator.provider`. Missing
or invalid facts refuse. JSON transport may reorder object keys, so this path
reconstructs the canonical order before the shared checks. The checks for
canonical input encoding and duplicate JSON keys run on raw Grove task lines,
not on already-parsed generic objects. This context accepts no executable data.

On Ravel 2.x, import `examples/grove-review.rhai` by its path, and delegate
`load_context` and `select` to it. Owners pass custom tables to
`load_context_with(rules, request)` and `select_with(rules, request, context)`.
They wrap the final selection once with
`provenance::with_provenance(selection, VERSION, request)`. Ravel 2.x gives
Rhai a JSON number such as `1.0` as a float. The Rhai selector therefore reads
a schema version equal to 1 as the integer 1, as Ravel 1.x does.
`assets/ravel-policy/README.md` shows the form.

## Verification

`task ravel:review RAVEL=/absolute/installed/ravel RAVEL_RUNTIME=<1.1|2.0>`
checks strict installed-SDK types on 1.1 and the case table of both sets. It
also checks actual Grove lifecycle sessions with fake agents, and
inherited-evidence isolation. Grove's delivery contraction removed the old
bundled dispatch package and its compatibility task. Current checks consume the
independent installed Ravel pair. No test launches a model or edits the owner policy.

Save complete logs with
`set -o pipefail; task ... 2>&1 | tee target/k42/review.log`.
