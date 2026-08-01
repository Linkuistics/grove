# Review target receipts

Grove records the effective harness and model of the session that hands a
reviewed artifact to its review, then recomputes the review target at launch and
compares the two. For a leaf producer, that source session is the producer
itself. For a producer decomposed into a brief-carrying node, it is the factual
picked leaf whose successful retirement leaves the node with no live
descendant: the same session that verifies the node's `Done when` and reports
its close. The source session is kind-agnostic: a nested producer, review, or
integration leaf may be responsible for that handoff. Its factual responsibility
matters; substituting producer-kind routing would turn the receipt into a guess
about authorship. One retirement that closes several reviewed decomposition
ancestors supplies the same handoff target to each. This is deliberately the
aggregate artifact's **handoff target**, not a claim to represent every session
that contributed to it. A producer entity closed only by pruning remains
deliberately uncheckable: an `ABANDONED` transition records a human decision
against work, not a session that produced the artifact for review.

The foreground launch exports one session-target value containing the worktree
and source-session handle from one structured routing peek as well as the
resolved target. The same guarded peek supplies routing, readiness, and launch
diagnostics; no later pick reconstructs the routed identity. While the factual
leaf is still live and the tree is exclusively locked, retirement validates the
worktree, routed handle, current pick, and which reviewed producer entities this
`DONE` transition will complete. It applies the `DONE` infix first and
best-effort replaces each prepared producer receipt only after that succeeds, so
a failed terminal rename writes no receipt and a normal failed receipt write
leaves diversity uncheckable without blocking retirement. Replacement is
unconditional only for a **live** linked review. A terminal review will never
launch to consume new evidence, so Grove skips that write and emits an advisory
`uncheckable(reason=review-terminal)` diagnostic instead of changing a completed
task in an unrelated work item. Consequently a close cascade can
identify several reviewed producer ancestors but at most one can have a live
linked review: a live inner review is itself a live descendant of every outer
producer.

Each producer entity must have exactly one leaf sibling declaring its explicit
`Reviews` relationship; zero, duplicate, malformed, or non-leaf claimants are
advisory-uncheckable and never chosen by position. A terminal claimant still
establishes the relationship, but is not a receipt-write target.

This binds because current routing configuration cannot reconstruct a historical
launch after the configuration changes, while a route ledger or signal payload
would put authoritative workflow state outside the task tree. Environment is
inherited rather than addressed, so every non-session harness spawn must scrub
the session-target value and all three identity checks remain mandatory even
after that structural guard. The routed handle is evidence about what target
actually launched; it never overrides Grove's later factual pick. A non-empty
structured peek without that handle is a launch-time routing failure, not a
target Grove guesses with a second read.

Every newly written receipt names both identities: `producer` is the entity in
the stable `Reviews` relationship, while `session` is the factual leaf whose
launch target was recorded. It also names the producer `generation`, defined as
the greatest permanent key at or below that producer entity. A leaf's generation
is its own key; a node's generation is the maximum key in its subtree. Review
launch resolves the explicit producer handle and recomputes this value before
trusting the receipt. Terminal entries remain in place and every supported node
reopen adds a fresh globally monotonic key, so a failed replacement after
reopen/reclose leaves an old receipt detectably stale; reordering does not alter
the generation. Existing receipts without `session` and `generation` remain
checkable only for direct leaf producers, where both facts derive unambiguously
from `producer`.

The receipt wire format is extensible advisory metadata. Readers accept unknown
JSON keys so adding evidence later does not turn a newer receipt into malformed
data for an older tolerant reader; required known keys and their values remain
strictly validated. Binaries shipped before this rule may reject the added
`session` and `generation` keys and warn that the receipt is malformed. That
version skew is bounded by the advisory launch rule and cannot be repaired by a
new writer.

The writer establishes the historical fact that static tree state cannot later
reconstruct: before `DONE`, `session` must be the routed factual pick; for a leaf
producer it is the producer itself, and for an ancestor node it is a descendant
whose removal from the live set closes that node. The reader validates every
fact still present: the explicit relationship resolves to a terminal leaf or a
brief-carrying node with no live descendant, the source is that leaf or a
terminal descendant, and the generation still matches. Hand-edited Markdown can
lie about the historical close just as it can lie about `Reviews`; the metadata
is advisory rather than authenticated.

For a routed review, relationship, receipt, source, and generation validation
comes from the same shared tree read as the structured routing peek. That result
is a forecast, not a reservation: a later mutation may change the factual pick
before the harness starts, and the session's own Bootstrap-and-pick fact still
wins. Grove never performs a second unlocked tree read and mistakes it for the
peek's state.

The freshness guarantee covers Grove's cooperative transitions: Grove writes a
receipt only after the leaf outcome that completes its producer entity, and a
successful later close replaces any existing line in a live linked review. A
terminal review is skipped because no later launch consumes the replacement.
Directly restoring a terminal leaf while leaving its old receipt remains a
generation ambiguity, because that unsupported edit adds no key. Grove still
does not block retirement on advisory metadata; if a post-`DONE` replacement
fails, review either rejects the prior generation as stale or, for that
hand-edited direct-leaf case, diagnoses that the retained value may be stale.

A prepared plan retains the review path and receipt facts, not a pre-`DONE`
rendering of the whole task file. Materialisation re-reads the review task after
`DONE`, replaces only the receipt line in that current text, and atomically
renames the result. The tree lock serializes Grove commands; a direct editor
racing between that final read and rename remains outside the cooperative
guarantee, as direct edits do for every task-tree mutation.

A receipt has no scheduling authority. Reopening a producer after its linked
review is already terminal neither reopens nor duplicates that review; if the
new generation needs adversarial review, ordinary tree work must name a new
review chain or step. This is the same guide-not-gate rule under which Grove does
not require a review after every producer.

Pruning retains its exact-target scope. A producer entity closed by `ABANDONED`
supplies no handoff receipt; pruning only that producer leaves a sibling review
and integration live, so depth-first pick schedules them next as uncheckable. To
abandon the whole reviewed path, the human prunes the enclosing review-chain
node.

The receipt is an advisory side effect of the terminal leaf transition, not a
node lifecycle mark. Closing a reviewed decomposition node can therefore change
the live linked review's bytes even though the node itself remains unmarked. The
no-confirmation rule still holds because the session establishes the factual
handoff; no human judgement about whether the path is worth doing is being
recorded. A close in error is reopened with `leaf-add`, and generation validation
makes the prior advisory receipt detectably stale.

Model equality is exact: equal non-null selector strings match, two harness
defaults match only under the same harness, and a default never matches an
explicit selector. An uncheckable warning always names its review; it names a
producer only from a valid stable relationship and otherwise says
`producer=unknown` with the reason, never inferring identity from tree position.
If a receipt's producer disagrees with a valid `Reviews` relationship, the
result is `uncheckable(reason=receipt-producer-mismatch)` and the relationship's
producer remains the only named producer.

Whenever a **checkable** receipt's `session` differs from `producer`, a rendered
warning names that validated source session so the operator can see the exact
handoff target being compared. An uncheckable receipt may carry a syntactically
valid session from stale or inconsistent evidence, so its warning never presents
that value as the factual handoff. A fully diverse launch remains silent by
design; that silence establishes diversity from the one recorded handoff target,
not from every contributor to a decomposed producer.

## Considered options

- **Write the receipt before applying `DONE`.** Rejected because a successful
  receipt write followed by a failed terminal rename leaves a live producer with
  a target that a later finisher may be unable to overwrite. Reopen only if the
  receipt and terminal outcome can be committed atomically as one portable
  operation.
- **Render the entire review task before applying `DONE`.** Rejected because a
  later materialisation can erase an edit made between preparation and the
  terminal transition. Reopen only if review tasks stop being ordinary editable
  Markdown or the two files gain a real multi-file transaction.
- **Leave every decomposed producer deliberately uncheckable.** Rejected because
  node closure already has one factual session responsible for verifying and
  handing off the aggregate result, and discarding that target would make a
  common review shape warn without using available evidence. Reopen if the
  node-close contract stops assigning the closing session that responsibility.
- **Aggregate every contributing child target.** Rejected because it would
  accumulate authoritative receipt state while the producer remains live and
  turn one advisory comparison into set ownership, partial-write, and cleanup
  semantics. Reopen if review must prove diversity from every contributor rather
  than guide diversity from the handoff context.
- **Restrict a decomposed producer's source session to a producer kind.**
  Rejected because the factual leaf responsible for checking and closing the
  aggregate may be a nested review or integration leaf; substituting an earlier
  producer target would reconstruct authorship rather than record the handoff.
  Reopen if diversity must target authoring provenance and Grove records that
  provenance explicitly.
- **Record a contributor count beside one target.** Rejected because a count
  changes no diversity claim and is either invisible in the required silent
  success case or forces a new success notice. The warning instead names the
  validated source session for a checkable receipt when it differs from the
  producer. Reopen if audit-visible successful comparisons become a requirement.
- **Withhold a routing notice unless the session will execute that review.**
  Rejected because the driver has only a forecast; another pre-spawn read narrows
  but cannot close the launch window, while binding the session to that result
  would override factual pick. The notice is scoped to its review handle instead.
  Reopen if a harness can accept context after the session performs its own pick.
- **Use only the closing session without recording a producer generation.**
  Rejected because `leaf-add` may legitimately reopen a completed node; if the
  next post-`DONE` replacement then fails, the old target would look current.
  Reopen only if reviewed producer nodes become immutable after close.
- **Automatically reopen a terminal review when its producer reopens.** Rejected
  because receipts are advisory evidence, not scheduling state, and Grove does
  not enforce a review after every producer. Reopen only if review relationships
  become an enforced lifecycle grammar rather than optional composition.
- **Replace a receipt in a terminal review.** Rejected because no future launch
  consumes it, while the write changes a completed task in the commit for a
  different work item. Reopen only if terminal review evidence gains a durable
  consumer outside the task tree.
- **Add a generation solely to validate manually restored leaf producers.**
  Rejected because a direct terminal-leaf edit adds no task-tree fact from which
  a new generation can be derived. Reopen if removing `DONE` becomes a supported
  lifecycle operation that also issues a fresh permanent key or explicit
  generation.
- **Recompute the producer's target from current configuration.** Rejected
  because kind, family, harness, and model configuration may change between the
  producer and review sessions, yielding a precise comparison against a target
  that never ran. Reopen only if routing becomes immutable for a grove's entire
  lifetime.
- **Keep a route ledger or add the target to the completion signal.** Rejected
  because the task tree is Grove's only workflow state, and the receipt is a
  fact about the review relationship that consumes it. Reopen only if Grove's
  artifact-only state constraint changes.
- **Make a missing receipt block retirement or review.** Rejected because target
  diversity is advisory: lifecycle correctness must not depend on metadata that
  can be absent from legacy or hand-edited chains. Reopen only if diversity
  becomes a launch correctness requirement rather than a warning.
- **Infer a missing producer from the previous sibling or chain suffix.**
  Rejected because positions move and Grove deliberately does not parse a
  cross-leaf grammar from filenames. Reopen only if producer identity becomes an
  explicit stable relation supplied by the tree rather than an ordering guess.
