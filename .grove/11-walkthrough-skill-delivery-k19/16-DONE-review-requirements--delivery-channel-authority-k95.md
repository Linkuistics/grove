# delivery-channel-authority-k95

**Reviews:** delivery-channel-authority-k94

## Goal

Adversarially try to disprove the `delivery-channel-authority-k94` requirements
decision before any replacement-campaign child is allowed to run.

## Context

- Producer artifact and stable handoff: `delivery-channel-authority-k94`, read
  from its own focused commit.
- Authority being preserved: `acceptance-replication-authority-k76` and the
  amended `walkthrough-skill-delivery-k19` brief chain.
- Finding that opened the gate: `supplemental-evaluation-k92` G1 and its
  disposition in `supplemental-evaluation-k93`.
- Blocked consumer: `paired-acceptance-campaign-k80`, which remains unable to
  run while the producer says `authorization = none`.
- Product decision that does not itself establish evaluation delivery:
  `docs/adr/skill-delivers-the-methodology.md`.

## Done when

- Re-derive the producer decision from the committed diff and available channel
  evidence. Try to find a currently provisioned direct request channel or
  authoritative harness receipt that the producer missed; reject local argv,
  filesystem, client-side digest, or asserted file-read evidence unless it
  demonstrably crosses the provider acceptance boundary.
- Try to disprove receipt authority in both directions: identify any false
  positive that could admit ignored, normalized, truncated, rejected, partial,
  or differently tooled treatment, and any requirement so strong that it rules
  out a channel that actually makes effective delivery checkable. Challenge the
  correlation, redaction, chronology, provider/runtime identity, and error
  taxonomy explicitly.
- Try to disprove treatment identity: recursively enumerate what the deployed
  skill can contain; challenge manifest exhaustiveness, deterministic ordering,
  UTF-8 and path rules, symlink and hard-link handling, byte preservation,
  framing visibility, control-arm delta, and the 65,536-byte cap. Reject any
  framing that silently turns the deployable skill into evaluation-specific
  semantic instructions.
- Try to disprove model-interface confinement. For intake and
  exposition/assurance, establish whether an explicit empty tool declaration is
  observable at the authoritative seam. For source/fragment, challenge whether
  one zero-argument `read_fixture` operation can be bound to the vendored
  manifest, called at most once, and prevented from accepting a path, traversing,
  following symlinks, mutating, or retrieving over a network.
- Try to disprove freshness: look for prior messages, response/session IDs,
  implicit harness state, caching, hidden tools, or a fixture-tool continuation
  that turns one fresh top-level request into a resumed conversation. Require
  the preserved request and receipt to enumerate every model-visible system,
  user, treatment, tool, fixture, and framing input.
- Check that leaving authority open changes no parent acceptance meaning and
  that the handoff is exact enough for `paired-acceptance-campaign-k80` to stop
  without making a new requirements choice.
- Record severity-ordered findings against the producer commit without editing
  the producer task. If actionable findings exist, commission a lazy
  `integrate-review-requirements` step with this bare stem where the walk reaches
  it before `paired-acceptance-campaign-k80`; if none exist, retire without
  creating one.
- Launch no evaluated treatment, control, scorer, or resolver context. Do not
  run a transport probe without separately authorized credentials and a channel.

## Notes

Assume the negative authorization is overconfident and the reopening contract
is internally inconsistent. A valid review may still conclude that no current
channel qualifies, but only after trying to produce a counterexample against
each of receipt authority, treatment identity, tool confinement, and freshness.

## Findings

Reviewed against the `delivery-channel-authority-k94` commit `b59479229187`,
whose diff contains the decision's `## Decisions (running log)`, the reopening
contract, the treatment-transport rules, the model-interface and freshness
seams, this review leaf, and the position shift of
`paired-acceptance-campaign-k80` from 16 to 17. No file inside that subtree
changed content. No producer file was edited by this session, no historical
evaluation byte was touched, and no evaluated treatment, control, scorer, or
resolver context ran.

Thirteen findings, severity-ordered and lettered `H1`–`H13` so they do not
collide with `F1`–`F14` of `evaluation-recovery-k73` or `G1`–`G16` of
`supplemental-evaluation-k92`, both of which they cite.

The decision's direction is right and its fail-closed instinct is correct: bytes
on disk are not bytes in a context, argv is not the effective request, and a
client-computed digest attests only to the client. What the findings below
attack is not that conclusion but the three things holding it up — an
eligibility bar written with no existence proof, an evidence base narrower than
the claim drawn from it, and a handoff that blocks the campaign without leaving
anything in the tree that could ever unblock it.

### H1 — The reopening contract states a necessary condition and never shows it is satisfiable, so a decision presented as contingent is in fact terminal (high)

The contract requires "a provider-issued acknowledgement, correlated by a
server-generated request identifier, that binds the accepted effective request
or its provider-computed digest", an acknowledgement status that "distinguishes
accepted-exactly from ignored, normalized, truncated, rejected, and partially
delivered input", and rejects any runtime with "undocumented request
transformation".

Nothing in the decision demonstrates that any channel can clear that bar. The
request/response shapes of the hosted chat APIs a later session would reach for
do not echo the accepted request and return no digest of it, and no hosted
provider documents the absence of request transformation — which is what the
third clause demands be documented away. Under a strict reading the only
eligible runtime class is a self-hosted open-weights server, where the request
bytes, the tokenization, the tool declaration and the absence of history are all
locally observable. That class is never enumerated among the options; the
decision searched only for hosted channels while writing a bar that appears to
exclude them all.

This is `G4` one level up. `G4` said nothing required the campaign's endpoint to
be shown attainable before it was frozen, and `supplemental-evaluation-k93`
applied it. The same defect is now in the gate that governs whether the campaign
exists: a bar with no attainability statement. The consequence is worse here,
because the decision's own wording — "Until a later requirements decision
replaces that value" — reads as *pending* to every downstream reader, while the
contract as written may admit nothing at all.

**Change:** the decision must either (a) name at least one concrete channel that
clears the bar, with its acknowledgement field identified, or (b) state the
**sufficient** evidence set — what a receipt must show, not only what it must
not be — and record that a hosted provider can meet it only through indirect
provider-computed quantities, or (c) record explicitly that the eligible runtime
class is self-hosted and that the parent behavioral conjunct is closed against
hosted providers. Any of the three is a legitimate answer; the current text is
none of them, and its silence is what makes a permanent state read as temporary.

### H2 — `G7`'s repair was delegated to a seam this decision then refused to authorize, so the criterion-author isolation defect is silently reinstated (high)

`supplemental-evaluation-k92` `G7` found criterion-author isolation asserted
rather than checkable — an unverifiable exclusion claim by a session whose
checkout contains `plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md`
and `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
`supplemental-evaluation-k93` recorded `G7` as **applied differently**: "the
criterion author must use the sealed tool-free fresh-request seam authorized by
`delivery-channel-authority-k94`".

This decision authorizes no such seam. `G7`'s repair therefore evaporates, and
the original `F5`/`G7` defect returns with nothing in the tree recording that it
did. The `k80` brief still describes `acceptance-instrument-k81` as deriving its
rules "in a sealed no-tool criterion-author context", which no longer has a
mechanism behind it.

The handoff hides this rather than resolving it. "`paired-acceptance-campaign-k80`
and all of its children must not run" is true, and it makes the reinstated defect
invisible for exactly as long as the campaign stays blocked — which is precisely
when a later session, unblocking the campaign, would be least likely to notice.

**Change:** record in the decision that `G7` reverts to unapplied, and name where
criterion-author isolation is re-owned in the event a channel is later
authorized. Isolation of a **non-evaluated planning context** does not obviously
need the same provider receipt an evaluated context does; if a weaker mechanism
suffices for it, say which, because that would let instrument authoring proceed
while delivery stays open.

### H3 — The decision blocks the campaign and leaves nothing in the tree that can unblock it (high)

The producer's `Done when` opened with "Ask the human to choose between
provisioning a direct authenticated request channel …, identifying another
harness channel …, or leaving the amended behavioral claim open", and
recommended the first. The running log records that "the human delegated the
choice to this requirements session", and the session then took the third branch
on the strength of facts about the current environment.

Delegation answers *who decides among the options as they stand*. It does not
answer *whether to provision a channel that does not yet stand*, which is the
one option the producer was told to recommend and the only one that can change
the value. That question was never put back to the human, and after retirement
there is no leaf that will put it.

The tree now reads: once this review chain settles, the next entry the walk
reaches is
`18-paired-acceptance-campaign-k80/01-design--acceptance-instrument-k81.md`, a
leaf chartered to consume a decision that forbids it to run. Its session can only
escalate. Later, `walkthrough-skill-delivery-k19`'s node close will check a
`Done when` carrying four behavioral conjuncts that no live leaf can discharge,
and fail it with a gap that cannot be named as work — the escalation branch of
`references/retire.md`, reached by construction rather than by discovery.

Grove has an exact idiom for *not now, but still ours*: leave the leaf live and
insert something ahead of it. The producer inserted this review and nothing else.

**Change:** insert, ahead of `paired-acceptance-campaign-k80`, a leaf whose whole
job is to put the provisioning question to the human with the options priced —
what a direct channel would cost to provision, what evidence it would then yield
against `H1`'s sufficient set, and what the alternative closure of the parent
conjunct would mean. That leaf, not `acceptance-instrument-k81`, is what the walk
should reach next.

### H4 — The evidence base for "no direct provider channel" is environment variables only, and a provisioned multi-provider client was never checked (high)

The producer records: "No direct provider API credential is present in this
session's environment", and the running log generalizes it to "The
direct-provider option has no provisioned credential or provider receipt to
inspect."

The narrow claim holds. This session confirmed that no `ANTHROPIC_*`,
`OPENAI_*`, `GEMINI_*`, `GOOGLE_*`, `AZURE_*`, `BEDROCK_*` or `VERTEX_*` API-key
variable is exported into it.

The generalization does not follow, because environment variables are not where
these clients keep credentials. `/opt/homebrew/bin/pi` is provisioned on this
machine and is a **direct multi-provider request client**: `--provider`,
`--model`, `--api-key`, `--system-prompt` (replacement, not append),
`--append-system-prompt`, `--mode json`, `--no-session`. It carries a first-class
readiness command that answers the producer's exact question without emitting a
secret and without any network transport probe:

    pi auth check --provider <provider> --json --no-refresh

Neither session ran it. This session was denied the attempt by its own sandbox
classifier, along with any inspection of `~/.claude/.credentials.json` or the
login keychain — so the question of what credentials this machine actually holds
remains open, and I record that as a limit of this review rather than as a
result. `/Users/antony/.local/bin/claude` and `/opt/homebrew/bin/codex` are also
provisioned and authenticated.

This does not overturn `authorization = none`. Readiness is not eligibility, and
eligibility is not authority — a credential the human has not authorized for this
campaign remains unauthorized however present it is. But the decision draws a
*factual* conclusion ("no provisioned credential to inspect") from an evidence
base that could not have supported it, and that conclusion is load-bearing for
the fail-closed branch.

**Change:** run the readiness check — or have the human run it — and record its
actual output before the fail-closed branch is treated as settled. Restate the
producer's finding at the width its evidence supports: no credential is
**authorized** for this campaign, which is a decision the human owns, rather than
no credential **exists**, which was never established.

### H5 — The stated reason for not running a bounded mechanics probe is contestable, and the probe needs no new credential (medium-high)

"A bounded transport probe was not run because there is no authorized credential
or channel to probe." Three authenticated harnesses are on `PATH`, and the
producer's own `Done when` permits a bounded non-evaluated probe to "establish
interface mechanics".

The mechanics the probe would settle are exactly the ones the decision leaves
asserted. `claude --print --output-format stream-json` surfaces
provider-generated message identifiers and provider-computed `usage`
token counts over the accepted effective request; `--tools ""` ("use `""` to
disable all tools") is the candidate empty-tool declaration the charter asks
about; `--system-prompt` replaces rather than appends, which the producer's
Notes — which mention only "a system-prompt append flag" — do not reflect; and
`--strict-mcp-config`, `--setting-sources`, `--safe-mode` and `--bare` exist to
strip the client's own additions from the request. Whether the resulting
provider-computed input-token count is stable and predictable enough to serve as
the "provider-computed digest" that the reopening contract **already admits** is
a question one non-evaluated probe answers.

One caveat the probe must respect: `--bare`'s own help states that under it
"Anthropic auth is strictly `ANTHROPIC_API_KEY` or `apiKeyHelper`", so on an
OAuth-only machine the stripping composition must be built from `--safe-mode`,
`--setting-sources`, `--strict-mcp-config` and `--system-prompt`, and what
remains in the system prompt must be measured rather than assumed.

**Change:** run the bounded mechanics probe before declaring the harness channel
class ineligible, and record what the emitted stream does and does not bind. A
class ruled out by argument where a one-command measurement was available is the
same overconfidence this review chain exists to catch.

### H6 — Prefix caching is the one provider-side equality oracle over effective request bytes, and the freshness rule bans it without argument (medium-high)

The receipt problem is that no provider echoes the request. The nearest thing to
an exception is prefix caching: a `cache_control` breakpoint placed at the end of
the treatment block makes the provider return separate cache-creation and
cache-read input-token counts, and a cache **read** is a provider-side assertion
that the effective prefix of this request matched a previously accepted prefix,
byte for byte, at an attested token length. Paired with a control whose treatment
field is absent — a guaranteed miss — that is provider-computed arm-delta
evidence rather than client-computed. Combined with a provider token-count
endpoint it bounds silent truncation and silent field-dropping, the two failure
modes the producer names first. The treatment is 13,940 bytes, comfortably above
any minimum cacheable prefix.

The freshness seam forbids it in passing: each request must contain no "cache
reference". There is a real argument for that ban — a cache hit means the
provider serves a stored prefix, so what was *processed* and what was *submitted*
are attested equal rather than identical, and a stored prefix is state carried
between requests. That argument is not made. The clause bans, as a freshness side
effect, the only instrument that could partly discharge the receipt requirement
sitting two paragraphs above it. That is the internal inconsistency this review
was chartered to look for.

The honest limit, which the integration must carry rather than lose: a cache hit
proves two client-constructed requests shared a prefix and how long that prefix
was. It does not prove the bytes were the treatment. It upgrades the evidence
from *the client asserts what it sent* to *the provider attests two requests were
prefix-identical at N tokens*; it does not close the regress. It is a partial
instrument, and `H1`'s sufficient-evidence set is where it belongs.

**Change:** decide the caching question explicitly and on its merits — as a
receipt question first and a freshness question second — instead of closing it by
a clause written for a different purpose. Confirm the mechanics against current
provider documentation during integration; this review is inspection-only and
did not fetch them.

### H7 — The treatment manifest is directory-scoped and the deployable skill points outside its directory (medium)

The transport rule is "recursively enumerate the verified final skill
directory". The verified final skill is
`plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md`, and at lines
19–22 it directs its reader to `../../PROVENANCE.md` — outside the enumerated
directory, and named as the record of "the adopted sources and limits".

So the delivered treatment is the deployable artifact with a dangling reference.
In deployment that file is on disk beside the plugin and reachable; on the two
surfaces the producer confines to zero tools it is unreachable by construction,
and on the third the only permitted operation is a fixture read that cannot
retrieve it. This is not an accident of one skill: the parent brief requires the
skill to follow "plugin layout" and progressive disclosure, and
`plugins/linkuistics/PROVENANCE.md` is a plugin-level shared file by design.

The producer's own principle — "the deployable skill remains the treatment; the
evaluation must not rewrite it into campaign-specific instructions" — is
violated in the direction nobody checks. A directory-scoped manifest does not
rewrite the treatment; it silently *narrows* it.

**Change:** the manifest rule must state its closure over the skill's own
outbound references. Either vendor the referenced plugin files into the treatment
and record them as treatment bytes with their own digests, or record the dangling
reference as a declared, frozen difference between the deployed artifact and the
delivered treatment, so the arm-delta comparison and the final report both carry
it.

### H8 — The one-fixture, zero-argument `read_fixture` operation carries no degree of freedom and re-imports the boundary it exists to police (medium)

`read_fixture` accepts no path or locator, returns the exact set pinned by the
vendored manifest, and may succeed at most once. The only thing the model chooses
is *whether* to call it — a binary that is observable in the output stream under
either design.

Against that single bit, the tool costs the whole confinement axis. A tool
declaration is the hardest element to bind to a provider receipt: the producer
requires that "the provider receipt must bind this declaration and the fixture
manifest to the same accepted request as the treatment", and no hosted provider
echoes a tools array. Every clause about traversal, symlink escape, mutation,
process, network and at-most-once succeeds is enforced by the client — the party
whose word this decision spent nine paragraphs declining to take.

Inlining the fixture into the user message collapses source/fragment onto the
same zero-tool seam as the other two surfaces, deletes the axis outright, and
makes the historical failure mode structurally impossible rather than merely
detected: the frozen probe recorded 20 of 21 attempts breaching the declared
model-interface boundary, and a request that declares no tools cannot produce a
tool call at all.

**Change:** if "did the model choose to look before planning" is a scored
behavior on this surface, say so in the instrument and keep the tool with that
justification recorded. If it is not, inline the fixture and declare zero tools
on all three surfaces.

### H9 — The fixture is model-visible input with none of the treatment's byte rules (medium)

The treatment gets exhaustive recursive enumeration, path and encoding rules,
non-regular-file rules, byte preservation, explicit framing status, and a
65,536-byte cap. The vendored fixture gets one clause — "the exact regular-file
set pinned by the vendored fixture manifest" — and nothing else: no encoding
rule, no size cap, no framing status, no statement that it is byte-identical
across arms.

It is model-visible input on one surface in both arms, it competes for the same
context budget the treatment's cap exists to defend, and under `H8`'s
recommendation it would become part of the user message and therefore part of the
frozen arm-delta comparison. An asymmetry this large between two model-visible
byte streams in the same request is not a proportionality saving; it is the same
gap in a place nobody looked.

**Change:** apply the treatment's transport rules to the fixture, or state which
of them do not apply and why.

### H10 — Delivering raw skill bytes delivers frontmatter that no deployed context sees as instruction (medium)

The treatment's first bytes are a YAML block carrying `name`, `description` and
`harnesses`. In deployment a harness consumes these as metadata — an index entry
and a load decision — not as instruction text placed before a user message. Under
"File bytes are carried unchanged: no newline, Unicode, encoding, or whitespace
normalization is permitted", they are delivered verbatim as model-visible
instruction.

The framing rules cover channel framing and are silent on this. Frontmatter is
either part of the declared enabled intervention or an artifact of the transport,
and which one it is changes what the campaign measures: a description written to
make a harness *select* a skill is not neutral text when it arrives as the
opening of a system prompt.

**Change:** state frontmatter's status explicitly in the manifest and reflect it
in the arm-delta comparison, on the same footing as model-visible framing bytes.

### H11 — Two admissibility rules can fail a treatment for reasons that cannot affect a model-visible byte (low-medium)

Hard links: "hard-link aliases … make the treatment ineligible". A link count
above one is set by filesystem-level deduplication and by checkout
optimizations, has no bearing on file contents, and is invisible to any byte the
model sees. The rule is a false positive with no threat model behind it.

Encoding: "Only regular files whose complete contents are valid UTF-8 without NUL
bytes are admissible." A skill may legitimately carry a diagram, an image, or a
PDF under progressive disclosure. Declaring such a treatment ineligible
constrains the *product* to suit the *instrument* — the inversion the producer's
own rule against rewriting the deployable skill forbids, arriving through the
transport rules instead of through the prose.

Both are currently moot: the treatment is one 13,940-byte UTF-8 file. Both are
latent, and a latent rule in a frozen manifest is a rule nobody re-derives.

**Change:** replace the hard-link clause with a rule over content as delivered,
and state what happens to a non-text asset — carried out of band, declared
excluded with the exclusion recorded, or reopening requirements — rather than
silently disqualifying the artifact.

### H12 — Sampling parameters are absent from the pinned identity fields (low)

Pinned: provider, endpoint, model snapshot, runtime, API/schema version,
authenticated client identity. Not pinned: temperature, top-p, top-k, stop
sequences, any thinking or reasoning budget, and `max_tokens` — which appears
only obliquely as "the frozen output allowance" inside a context-limit clause.

These are request fields, not identity fields, so the arm-delta rule catches a
difference *between arms* but pins no *value across the campaign*. A resumed
campaign, a replacement pair run days later, or a second surface configured by
hand can vary them without tripping anything, and the exposition/assurance
surface is the one most sensitive to output allowance.

**Change:** pin the full sampling configuration in the manifest alongside the
model snapshot, and make it part of what the preserved request body enumerates.

### H13 — The transport apparatus is specified for a corpus the treatment does not have (low)

The verified final skill is a single file of 13,940 bytes. The transport
specifies recursive enumeration, `SKILL.md` first with remaining paths in
unsigned-byte lexical order, dot/dot-dot/absolute path rules, symlink, hard-link,
device, socket and FIFO handling, per-file structured or length-prefixed framing,
and a 65,536-byte cap — rules written for a directory tree that does not exist.

`acceptance-replication-authority-k76` requires apparatus to stay proportionate
and to prefer a bounded human action to a built one. None of these rules is
wrong, and the skill may well grow; the objection is that the manifest describes
a shape without recording the actual one, so no reader can tell which clauses are
live and which are contingent.

**Change:** record the treatment's actual shape and byte count in the manifest,
and mark the multi-file clauses as conditional on that shape changing.

## What survived the attempt

Recorded so the integration does not re-litigate ground this review tried and
failed to take.

- **Parent acceptance meaning is unchanged, and the producer is right that it is.**
  The `walkthrough-skill-delivery-k19` `Done when` conjuncts still say what they
  said; leaving authority open changes their *attainability*, not their meaning,
  and attainability was already recorded by `supplemental-evaluation-k93` in the
  brief's Decomposition. No amendment was owed. `H3` is about the tree's shape
  under that unchanged wording, not about the wording.
- **The handoff string is exact and pinnable.** `delivery-channel-authority-k94:
  authorization = none` is a value a later session can cite without re-deciding
  anything, and the enumerated prohibitions — installed directory, asserted or
  observed file read, captured CLI argument, client-computed prompt digest,
  unauthenticated local transcript — are specific enough that
  `acceptance-instrument-k81` cannot reinterpret its way past them. `H2` is the
  one thing the handoff omits.
- **The core reasoning is sound and is not what the findings attack.** Bytes on
  disk are not bytes in a context; argv proves command construction and not the
  effective request; a client-side digest attests only to the client; a harness
  assertion that it attempted a preload does not cross the provider acceptance
  boundary. Every one of these survives, and `G1`'s option (b) — on-disk
  installation with an observed read as the receipt — remains correctly rejected.
- **The review-leaf commission is correctly placed.** The producer cut this leaf
  last, with the bare stem `delivery-channel-authority`, at position 16
  immediately before `paired-acceptance-campaign-k80`, and shifted that node to
  17 without altering a byte inside it.
- **The 65,536-byte cap does not bind.** It was worth checking whether the
  producer had frozen a cap the treatment already exceeded, which would have made
  the transport ineligible by construction. It has not: 13,940 bytes against a
  65,536-byte cap, a factor of 4.7 in hand.

## Limits of this review

- Credential discovery was attempted and denied by this session's own sandbox:
  `pi auth check`, keychain enumeration, and `~/.claude/.credentials.json` were
  all blocked. `H4` therefore reports an **unperformed check**, not a credential.
  No attempt was made to work around the denial.
- No network request of any kind was made. `H6`'s caching mechanics and `H1`'s
  characterization of hosted provider response shapes rest on knowledge, not on
  fetched documentation, and the integration must confirm both before acting on
  them. A review that fetched them would be doing the integration's work.
- No transport probe was run, no credential was read, and no evaluated treatment,
  control, scorer, or resolver context was launched.

## Decisions (running log)

The review produced actionable findings, so the lazy `integrate-review-requirements`
step is earned. Per `references/decompose.md` the target is the first sibling
entry after this leaf whose subtree still holds live work: that is the node
`paired-acceptance-campaign-k80`, not a leaf inside it. Inserting at the node's
slot is what keeps `acceptance-instrument-k81` from running before the findings
land.

No in-session reviewer was materialised: a `review-*` session spends none of the
allowance because it is itself the adversarial read.

Findings are recorded against the committed producer artifact only. The
`delivery-channel-authority-k94` file, the `paired-acceptance-campaign-k80`
subtree, every historical evaluation record, and every frozen rubric byte are
unchanged by this session.

`H1`, `H3` and `H4` are the three that decide whether this decision is a gate or
a grave. `H1` is a bar with no existence proof, `H4` is a conclusion wider than
its evidence, and `H3` is the missing leaf that would let a human answer either.
`H2` is the one thing the handoff loses on the way past: a repair that
`supplemental-evaluation-k93` recorded as applied is, after this decision,
unapplied and unrecorded. `H6` and `H8` are the two places where the contract
works against itself — one banning the instrument that could partly discharge it,
the other paying the whole confinement axis for a single observable bit. The
remainder are transport rules that describe a corpus the treatment does not have,
or omit inputs the model does see.

The conclusion `authorization = none` is not disproved. It is under-evidenced,
stated with more permanence than its author intended and less than its own
contract implies, and left in a tree with no path back.
