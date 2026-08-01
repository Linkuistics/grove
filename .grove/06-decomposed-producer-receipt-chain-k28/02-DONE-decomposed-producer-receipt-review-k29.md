# decomposed-producer-receipt-review-k29

**Kind:** review-design
**Reviews:** decomposed-producer-receipt-k20

## Goal

Adversarially review `decomposed-producer-receipt-k20` and record concrete
findings for its integration step.

## Context

Review the updated `review-target-receipts` ADR and the producer-handoff,
advisory-diversity, module-interface, test-seam, compatibility, and out-of-scope
sections of `docs/specs/doubt-grove-review-mechanics.md`. Challenge whether the
node-closing handoff target and permanent-key generation preserve factual pick,
node reopen, restart, non-blocking metadata, and direct-producer compatibility.

## Done when

- The source-session and generation invariants are tested against nested close
  cascades, supported reopen, reordered children, and stale or hand-edited
  receipts.
- Guard scope and launch-window forecast semantics are checked for races.
- Receipt materialization is checked for lost Markdown edits around `DONE`.
- The choice of one handoff target, rather than every contributing target, is
  challenged as an explicit advisory trade-off.
- Findings are severity-ranked and recorded here for
  `decomposed-producer-receipt-integrate-k30`.

## Notes

Produce findings only; do not implement fixes.

## Findings

Artifacts reviewed: `docs/adr/review-target-receipts.md` (whole file, reworked by
`decomposed-producer-receipt-k20`) and `docs/specs/doubt-grove-review-mechanics.md`
sections *Producer handoff* (243-310), *Advisory target diversity* (313-484),
*Module interfaces* (486-526), *Test seams* (528-599), *Compatibility* (627-660)
and *Out of scope* (662-679), against that leaf's three `Done when` clauses and
against the current implementation seams the design constrains.

Claims marked **[measured]** were produced by compiling the receipt struct shape
against `serde` 1.0 in a scratch crate outside the repository; claims marked
**[read]** cite current source. The working copy is unchanged.

### Done-when trace

| Clause (from `decomposed-producer-receipt-k20`) | Verdict | Where |
|---|---|---|
| decides uncheckable vs. a well-defined target, and names whose target that is | **met, but the named target can be a review-family session, which the design never considers** | R1 |
| preserves factual-pick checks, node-close contract, task-tree-only state, restart | **factual-pick and restart hold; the node-close contract is silently changed** | R3, counterexamples |
| implementation/test/doc work decomposed into later leaves, not absorbed | **met** — `…-implementation-chain-k24` exists and the design session wrote no code | — |

### R1 — the handoff session can be a `review-*`/`integrate-review-*` leaf, so the comparison becomes reviewer-vs-reviewer (severity: high)

`docs/adr/review-target-receipts.md:5-9` defines the decomposed producer's source
session as "the factual picked leaf whose successful retirement leaves the node
with no live descendant" — i.e. the **last leaf in depth-first order** inside the
producer node. Nothing constrains that leaf's kind.

Grove's own composition guidance makes a review-family leaf a likely occupant of
that slot. `content/SKILL.md`'s *Compose, don't just append* tells a session to
reach for a review chain by default for a load-bearing artifact and to argue
itself *out* of one; a decomposed producer whose final child is a chain node
therefore closes on that chain's `integrate-review-*` leaf. A node decomposed
into `01-x-chain-k40/{x, x-review, x-integrate}` records `x-integrate`'s target.

Two consequences, and they point in opposite directions, so neither is a tuning
question:

- **Config-independent:** the recorded target was chosen by *review-family*
  routing policy, not producer policy. The receipt then answers "did the reviewer
  differ from the integrator?" — a question the design never asked — and says
  nothing about the sessions that wrote the artifact under review. The receipt's
  stated purpose (`:3-5`, "the effective harness and model of the session that
  hands a reviewed artifact to its review") silently stops being served.
- **Config-dependent:** under `GROVE_INTEGRATE_REVIEW_HARNESS` set equal to
  `GROVE_REVIEW_HARNESS` — a natural configuration, since the integrator applies
  the reviewer's findings — the harness axis matches by construction and every
  such review warns. Under the documented example (`docs/CONFIGURATION.md:130-147`:
  reviews to `codex`, integration back to Claude) the axes differ by construction
  and the review is silently "diverse" no matter what the producing sessions ran.
  The notice is noise in one configuration and vacuous in the other.

`Family::Review` and `Family::IntegrateReview` are distinct routing families
(**[read]** `src/leaf.rs:138-158`), so this is not an accident of one variable.

*What would satisfy this:* either restrict the handoff target to a closing
descendant of a **producer** kind (falling back to `uncheckable(reason=
handoff-session-not-a-producer)` when the closing leaf is review-family), or
record the closing leaf's kind in the receipt and render it in the notice so the
operator can see what the comparison is actually against. Either way add the case
to the test seams — it is absent from `docs/specs/doubt-grove-review-mechanics.md:560-576`.

### R2 — the "one of N" trade-off is invisible at the only place it is consumed, and the cheap middle option was never considered (severity: high)

The ADR calls the choice "deliberately the aggregate artifact's **handoff
target**, not a claim to represent every session that contributed to it"
(`:12-14`), and *Out of scope* (spec `:668-670`) repeats it. Both statements live
in documents the operator is not reading at 02:00 when a warning scrolls past.

The rendered notice (spec `:434-453`) names the review handle, the producer
handle, the matching axis, both targets and the routing configuration. It does
**not** name `session`, and it does not indicate that a decomposed producer's
comparison covered one of several contributing sessions. So a *silent* result —
the design's "both differ, stay silent" case — reads as "this review is diverse
from the producer", when in a four-child node it means "diverse from child four".
The reader has no way to tell a leaf producer's full-coverage silence from a node
producer's one-of-N silence, because the two receipts are rendered identically.

The *Considered options* list jumps straight from the accepted design to
"**Aggregate every contributing child target**" (`:117-121`), rejected on set
ownership, partial-write and cleanup grounds. Those grounds are real, and they do
not apply to the cheaper option in between: keep exactly one receipt written at
close, and add a scalar — `contributors: <count of terminal descendant leaves>`,
or simply always rendering `session=<handle>` — so the notice can say what it
compared. That option carries no set semantics, no partial writes, no state
beside a live producer, and no second write path. Its absence makes the
rejection of aggregation look stronger than it is: the alternative that was
actually available was never priced.

*What would satisfy this:* render `session=` in the warning whenever it differs
from `producer=`, add the middle option to *Considered options* with its own
reopen condition, and add a seam asserting a decomposed-producer notice is
distinguishable from a direct-producer notice.

### R3 — a reviewed node close now writes bytes, invalidating the stated premise of the confirmation-boundary decision (severity: medium-high)

Two canonical surfaces assert that a node close writes nothing:

- `CONTEXT.md:204` — "a **node close** (writes nothing at all) ask[s] nothing".
- `content/SKILL.md:349-352` — "A node is never marked, so whatever a human
  answered the tree was byte-identical afterwards — and a node closed in error is
  reopened by one `leaf-add`, with nothing to undo."

That is the *first* of the confirmation boundary's two ordered tests, and it is
the test the node close was decided on. Under this design, closing a **reviewed**
decomposed producer writes a `**Producer launch:**` line into a sibling review
task (spec `:255-266`). Both quoted claims are now false for that case: the tree
is not byte-identical afterwards, and a node closed in error leaves a receipt
behind — precisely the residue the `generation` field exists to detect
(ADR `:44-51`).

The *conclusion* survives, via the second test: what the session establishes is
what it did, not what is worth doing, so it still asks nothing. But the design
changed the input to a decision recorded elsewhere and did not reconcile it,
which is the "rework the set in place, and reconcile the briefs that cite it"
obligation a `design` leaf carries. Left as is, the next session to reason about
the confirmation boundary reads a premise the code contradicts.

*What would satisfy this:* restate the boundary in `CONTEXT.md` and
`content/SKILL.md` as "a node close writes no *lifecycle* mark; a reviewed node
close writes advisory metadata whose staleness is self-detecting", and say
explicitly that test (2) is what now carries the no-ask conclusion.

### R4 — the compatibility claim about old binaries is false, and the wire rule that would make it true is unstated (severity: medium-high)

`docs/specs/doubt-grove-review-mechanics.md:629-630` states: "Old binaries and
hand-edited task files ignore the new freeform relationship and receipt fields."

True for `**Reviews:**`/`**Integrates:**` and for hand edits. False for the
receipt. `ProducerLaunchReceipt` is `#[serde(deny_unknown_fields)]` with a
flattened `LaunchTarget` (**[read]** `src/task_relationship.rs:79-85`, `:25-31`),
and that combination denies rather than ignores:

```text
[measured] old reader / new receipt  -> Err("unknown field `session` at line 1 column 88")
[measured] new reader / old receipt  -> Ok(session: None, generation: None)
```

So an old binary reading a new receipt takes the `Metadata::Malformed` path
(**[read]** `src/task_relationship.rs:168-209`) and reports
`producer-receipt-malformed` — a *wrong* diagnostic, not a benign one, since it
accuses the receipt of being garbled when it is merely newer. Nothing blocks, so
severity is bounded by the advisory rule; the defect is that the compatibility
section promises a property the type does not have, and the next field addition
repeats it.

This is not hypothetical in this repository: the grove is driven by the
Homebrew-installed `grove-llm` (16.4.0 today) while the checkout evolves, so the
skew window is every interval between merging and upgrading.

*What would satisfy this:* correct the sentence, state the wire rule ("receipt
readers must ignore unknown keys") as part of the receipt contract so it binds
future fields, and add the missing legacy-receipt seam — `:560-576` currently
tests no receipt lacking `session`/`generation`, in either the leaf-producer
(derives, stays checkable) or node-producer (uncheckable) direction, although
`docs/adr/review-target-receipts.md:49-51` promises both behaviours.

### R5 — unconditional replacement writes into terminal review tasks, and every multi-ancestor cascade contains at least one such write (severity: medium)

The ADR makes replacement unconditional (`:22-25`) and separately establishes
that a receipt has no scheduling authority: reopening a producer whose review is
already terminal neither reopens nor duplicates it (`:86-90`). Together they mean
Grove rewrites a *completed* leaf's bytes to install evidence nothing will ever
read. The freshness argument the unconditional rule rests on — a stale receipt
must be replaceable — does not reach this case, because a terminal review never
launches and never compares.

The cost is not only wasted I/O. The write lands in the working copy of a session
whose commit is about a different work item, so a finished artifact acquires a
diff hunk attributed to unrelated work — against the one-task-one-focused-commit
rule.

This also bounds the multi-ancestor case the design leans on. Spec `:258-261`
says one factual leaf "may therefore supply the handoff target for several newly
closing reviewed ancestors". Structurally, **at most one closing reviewed
ancestor can have a live linked review**: an inner reviewed node `N1` keeps its
review `R1` as a *child of* `N2`, so while `R1` is live `N2` has a live
descendant and cannot close. Two reviewed ancestors therefore close together only
when the inner one's review is already terminal — i.e. only on the
reopen-after-review path, where the inner write is exactly the useless one above.

*What would satisfy this:* skip materialisation when the linked review is
already terminal and emit `uncheckable(reason=review-terminal)` instead; state
the at-most-one-live-review invariant where the cascade is described, so the
multi-ancestor sentence is read as a rare reopen case rather than routine.

### R6 — a prune that closes a reviewed producer leaves its review and integration live and next in pick order (severity: medium)

Spec `:307-310` covers exactly one consequence of `leaf-prune`: no handoff target
is supplied, and the linked review stays uncheckable. It does not cover what the
loop does next.

`leaf-prune` given a node marks every live leaf in *that subtree* and nothing
outside it (**[read]** `src/tree_lifecycle.rs:261-277`). The linked review and
integration are siblings of the producer, not descendants, so they survive. The
depth-first walk then returns `…-review` as the very next task: an unattended
relaunch opens a fresh adversarial session over an artifact the human just
decided against, warns `producer-receipt-missing`, and produces findings for an
integration leaf that will run after it.

The operator's correct move is to prune the enclosing **chain node** rather than
the producer, which marks all three steps — but that is nowhere stated, and the
producer is the entity a session naturally names when it concludes a path is
dead.

*What would satisfy this:* say in the handoff section that abandoning a reviewed
producer means pruning the chain node, and add the pick-order consequence to the
prune paragraph so the design records the shape, not just the receipt.

### R7 — the spec restates the ADR nearly verbatim, and the two copies have already drifted (severity: medium)

`CONTEXT.md:344-347` states the grain rule: a spec "**cites** the ADRs in its area
rather than restating them (restate one and the two sets will disagree, after
which neither binds)". The reworked pair breaks it in at least four places, and
the restatements are not summaries but paraphrases already differing in wording:

| Fact | ADR | Spec | Divergence |
|---|---|---|---|
| plan stores facts; re-read after `DONE` | `:79-84` | `:270-275` | "The tree lock serializes Grove commands" vs. "Grove's tree lock excludes cooperating commands" |
| producer generation definition | `:40-51` | `:277-285` | duplicated in full, including the reorder and hand-edit carve-outs |
| receipt has no scheduling authority | `:86-90` | `:287-291` | duplicated |
| exactly-one claimant / no positional inference | `:26-28`, `:92-99` | `:418-427`, `:444-453` | duplicated across three sites |

The ADR grew from 60 to 130 lines in this session; most of the growth is content
the spec also carries. The failure mode is not redundancy, it is that a future
edit to one copy is invisible from the other.

*What would satisfy this:* cut the spec's restatements to one-line citations of
the decision, keeping in the spec only what the ADR does not fix — the JSON
shapes, the module boundaries, the seams.

### R8 — both worked examples conflate `session`'s key with `generation`, and no seam separates them (severity: medium-low)

Spec `:343` and `:383` both render `"session":"sync-docs-k27","generation":"k27"`.
The two are independent facts: `session` is the *depth-first last* live leaf,
`generation` is the *greatest permanent key* at or below the producer. A key is
allocated as tree-wide max + 1 while a position is rewritten on insert, so a
`leaf-insert` early in the producer's subtree carries the highest key while the
closing leaf sits last — the two values then disagree, and every example a reader
or test author copies says they agree.

An implementation that derives `generation` from `session`'s handle passes every
example in the spec and every case listed at `:560-576`; the nearest listed case,
"generation stability under reorder versus change after supported reopen", does
not separate them either, because a reorder leaves both unchanged.

*What would satisfy this:* change one example so the two keys differ, and add the
seam — `leaf-insert` into the producer subtree after later children exist, then
close, and assert `generation` names the inserted leaf while `session` names the
closing one.

### R9 — the retained review evidence and its prompt-prepended notice can head a session that runs a different leaf (severity: medium-low)

The design settles the routing half of the launch-window race (spec `:466-470`:
the peek is a forecast, the session's factual pick wins) and leaves the rendering
half unstated. The notice is prepended to the launched prompt and printed to
stderr before spawn (**[read]** `src/loop_driver.rs:321-325`, `:384-386`); a
`leaf-insert` or a producer reopen inside the window then leaves a session whose
first lines are "review target diversity warning (review=…)" while its own
`pick` hands it a producer.

For an LLM-driven loop this is more than cosmetic: the ownership discriminator is
the leaf `pick` returns, and the prompt now opens with evidence about a different
leaf. The stderr copy is transient; the prompt copy is the one that persists in
the transcript, which is the property `:455-459` cites in its favour.

*What would satisfy this:* state that the notice may name a leaf the session does
not execute, and word it so the session discards it when its own pick disagrees —
it already names the review handle, so the check is available to the reader.

### R10 — the glossary was not updated inline, and its receipt entry is now wrong for the case this design added (severity: low)

`CONTEXT.md:298-329` still defines a receipt as "the effective harness and model
of the producer session that retires a reviewed artifact" — which is exactly what
this design changed: for a decomposed producer the retiring session is a
*descendant*, and the entity is a node that never launched. The entry also uses
"generation" loosely (`:316`, inherited from the hardening design) while the field
`generation` now has a precise definition, and never mentions `session`.

Grove's rule is to append the glossary *inline* as terms resolve; the spec's
*Canonical surfaces* (`:606-608`) defers it to implementation instead. Two new
terms with exact meanings — the **handoff/source session** and the **producer
generation** — are therefore undefined in the one file every future session
reads, across however many sessions the implementation chain takes. That is the
drift the glossary exists to prevent.

*What would satisfy this:* add the two terms to the *Review target receipt*
entry now, and correct its opening definition to cover both producer species.

### R11 — the routing peek verb no longer describes its payload (severity: low)

`grove-llm kind --with-harness --json` now returns path, handle, kind, declared
harness *and* validated review evidence including the producer's own harness and
model (spec `:326-348`). The verb is named for one of five fields and the flag
for another; a reader of `--help` cannot guess that review-receipt validation
happens here, and `harness` means two different things at two nesting levels (the
leaf's declared harness at the top, the producer's effective harness inside
`review`). `generation` is additionally encoded as the string `"k27"` rather than
an integer, so every consumer re-parses the prefix.

The one-guarded-read justification is sound; the naming is what has fallen behind
it, and the design's own *Canonical surfaces* list already commits to reconciling
`grove-llm kind --help`.

*What would satisfy this:* either rename the peek to match its contract or record
in *Module interfaces* that the verb is deliberately the single guarded-read
entry point, and disambiguate the inner `harness`/`model` keys as
`producer-harness`/`producer-model` (or nest them under `producer-target`).

### Counterexamples attempted and not found

Recorded so `decomposed-producer-receipt-integrate-k30` does not re-derive them:

- **Post-`DONE` receipt materialisation racing another Grove command.** The
  re-read-then-rename window is inside the exclusive guard: `leaf_retire` takes
  `tree_access::write` and calls `leaf_retire_unlocked`, which prepares, renames
  and materialises before returning (**[read]** `src/tree_lifecycle.rs:187-247`).
  The ADR's carve-out for "a direct editor racing between that final read and
  rename" (`:82-84`) is therefore the whole exposure, and it is stated.
- **Generation failing to detect a reopen from a deep subtree.** Keys are
  allocated tree-wide as max + 1 and no terminal entry is ever removed, so any
  reopen anywhere at or below the producer yields a key strictly greater than the
  recorded generation, however deep the previous maximum sat. Monotonicity holds.
- **A slug rename breaking receipt validation.** `resolve` matches the terminal
  `-k<key>` and treats the slug as decorative (**[read]**
  `src/tree_read.rs:385-399`), and both `producer` and the `**Reviews:**` line
  carry the same handle text, so a rename leaves the pair mutually consistent and
  still resolvable.
- **The routed-handle check breaking for a node close.** Correctly re-anchored:
  spec `:364-369` requires the routed handle to equal the *retiring source
  session*, not the producer entity, which is what the driver exports
  (**[read]** `src/loop_driver.rs:365-376`). The current implementation's
  `session.handle != producer_handle` test (**[read]**
  `src/task_relationship.rs:398-404`) is the code this design correctly changes.
- **A receipt appearing beside a live producer.** Prevented by construction: the
  candidate set contains only entities the `DONE` transition itself closes
  (spec `:255-261`), and the plan is prepared while the retiring leaf is still
  the factual pick.
- **New optional fields breaking legacy reads.** **[measured]** `Option<String>`
  fields alongside a flattened target parse a legacy receipt as `None`/`None`
  without `#[serde(default)]`; the forward direction is the broken one (R4).
  Note for implementation: re-serialising then emits explicit `"session":null`,
  which an old binary also rejects.
