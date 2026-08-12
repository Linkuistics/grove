# increments-integrate-k12

**Integrates:** `increments-review-k11`
**Reviewed producer:** `increments-k4` (`177255c4cef2`)

## Goal

Repair the `increments-k4` decomposition before any implementation leaf runs.
This leaf owns the task tree and root brief: verify each finding below, then
reshape the future work. Do not implement the methodology parser, CLI verb, or
classification here.

The two-grove boundary remains settled unless triage finds evidence not present
in this review. The surviving findings are inside this grove's leaf boundaries,
contracts, and order.

## Context

The review inspected the producer commit, all five future leaf bodies, the
settled spec and ADR set, the current command-dispatch and methodology-identity
source, the release scan, and the embedded corpus (139,136 bytes across nine
markdown files). Six findings survived.

### B1 — `unit-grammar-k7` is a horizontal, inert slice

The root brief rejects stage 1 as a separate increment because it has no usable
behaviour (`.grove/BRIEF.md:55-58`), but then cuts that exact stage as
`unit-grammar-k7`. The leaf confirms that nothing reads a unit and that no
session-visible behaviour changes (`.grove/09-impl-unit-grammar-k7.md:86-93`).
That conflicts with the planning contract: child leaves are vertical and each
lands useful, verifiable behaviour without waiting on a sibling
(`content/TASK-FORMAT.md:83-93`, `:131-132`). Green scaffolding is not a vertical
slice.

Redraw `unit-grammar-k7` and `methodology-verb-k8` so the first implementation
boundary reaches an observable `grove-llm methodology` listing/fetch against the
trivial marking. Merging the two leaves is the simplest answer. If that no longer
fits one focused session, decompose the combined work into end-to-end
grammar-to-CLI capabilities; do not retain a parser/build-gate-only child.
Keeping classification judgement separate remains sound.

### B2 — methodology identity cleanup is assigned to the wrong grove

The settled spec couples identity simplification to the moment `grove-llm`
starts linking the embed: both binaries then hash the embed directly, and the
`build.rs` hash traversal, `GROVE_CONTENT_HASH`, hash dependency, and equality
test retire (`docs/specs/mandate-delivered-methodology.md:360-372`). The prior
design integration explicitly recorded this as a confirmed source claim.

The plan instead leaves the compile-time identity in `unit-grammar-k7`
(`.grove/09-impl-unit-grammar-k7.md:100-102`), omits its removal from
`methodology-verb-k8`, and assigns it to successor stage 4
(`.grove/BRIEF.md:108-115`). That preserves the duplicate traversal after its
only justification ends and contradicts the settled design.

Move the identity cutover into the implementation leaf that first links the
embed into `grove-llm` (whether that remains `methodology-verb-k8` or is merged
under B1). The direct embed hash must serve `--content-hash`, the driver's pairing
report, and the still-live provisioning stamp/warnings; remove the build-time
constant and equality test there. Remove this work from the successor charter;
stage 4 should delete provisioning, not perform a delayed identity migration.

### B3 — the planned inspection verb would be trapped behind session admission

`methodology-verb-k8` makes the installed binary a human inspection tool
(`.grove/10-impl-methodology-verb-k8.md:60-66`), but the current dispatcher admits
every subcommand through a working-tree/session-epoch guard
(`src/llm_cli.rs:438-449`). Only `--content-hash` is handled before that guard as
tree-free metadata (`src/llm_cli.rs:420-437`). A straightforward new match arm
would therefore refuse the promised audit from an ordinary shell outside a live
Grove session.

Add the missing command contract and acceptance seam to the verb leaf:
`grove-llm methodology`, in listing and fetch modes, reads only the binary's
embed and works without a repository, `.grove/`, driver lease, or session epoch.
Dispatch it before ambient-session admission, like `--content-hash`, and pin that
with a command-level test from a non-repository temporary directory.

### B4 — `unit-grammar-k7` pre-decides the ordering-key result

`ordering-key-placement-k6` legitimately considers deferring the ordering
carrier and its validation until composition (`.grove/08-design-ordering-key-placement-k6.md:56-63`).
Yet `unit-grammar-k7` unconditionally requires duplicate ordering keys to fail
the first build gate (`.grove/09-impl-unit-grammar-k7.md:70-76`). If the design
leaf selects its option 3, the next leaf begins with a contradictory Done-when.

Make the implementation contract derive its file-level validation cases from
the spec as settled by `ordering-key-placement-k6`; do not hard-code a duplicate
ordering-key error before that decision runs. Equivalently, require the design
leaf to reconcile the dependent task contract explicitly as part of its output.

### B5 — `classification-k9` is larger than one focused session

The leaf asks one context to classify 139,136 bytes across nine files, make every
unit-boundary and scope/deferral judgement, preserve cross-file reachability,
update the pinned complete id set, verify the installed listing, and author the
aggregate review handoff (`.grove/11-impl-classification-k9.md:5-11`, `:32-40`,
`:62-84`). The three largest files alone are 51,524, 41,954, and 24,894 bytes.
This is exactly the runaway, judgement-heavy work for which `leaf-decompose`
exists.

Decompose `classification-k9`. Its first child should be a `planning` leaf that
maps dependency-ordered classification batches before implementation children
are cut. A blind one-file-per-child split is not automatically green: a newly
classified triggering unit may defer across files to a procedure whose file is
still represented by one trivial triggering unit, which the build gate must
reject as a target of the wrong class. Batch by deferral closure and session
size, not filename alone. The node brief must preserve an aggregate
`review-impl` handoff after the final batch, naming the baseline and all child
handles so the reviewer can inspect the whole classification rather than only
the closing commit.

### B6 — `step-suffix-redundancy-k10` is neither last nor design-only

Keeping the human-raised concern in this grove is correct, but its claimed order
is not. The brief says `classification-k9` is last so its lazy review chain stays
contiguous, then places the suffix leaf after it and says that leaf preempts
nothing (`.grove/BRIEF.md:81-85`). A review leaf is added only when its producer
finishes and therefore lands after already-live siblings
(`content/TASK-FORMAT.md:220-245`). The suffix leaf will run before the
classification review, and its stated surface edits `content/SKILL.md` and
`content/TASK-FORMAT.md` — the classification artifact itself
(`.grove/12-design-step-suffix-redundancy-k10.md:68-83`).

The leaf also uses the `design` kind while requiring production/test changes to
`src/tree_grow.rs` and guidance tests if removal wins
(`.grove/12-design-step-suffix-redundancy-k10.md:74-93`). A design leaf's
deliverable is a spec/ADR decision; implementation belongs in an `impl` leaf
(`content/TASK-FORMAT.md:79-82`, `:97-100`).

Keep the concern in this grove, but put its decision before the real
classification and separate decision from execution. The design leaf should cut
an `impl` leaf carrying the chosen change; that implementation must also run
before classification so `classification-k9` classifies the final prose and its
review sees the final marked corpus. Reconcile the root brief's false
"contiguous" / "preempts nothing" rationale with the actual lazy-chain order.

## Confirmed non-findings

- **The two-grove split is sound.** Parser/gate plus an externally addressable
  embed is the first independently useful release; mandate composition plus
  provisioning retirement must share the next grove so the both-paths state is
  never a release boundary.
- **Narrowing the root brief was within `increments-k4`'s authority.** The prior
  brief expressly delegated grove-vs-leaf boundaries to planning, and the new
  Done-when preserves the first increment's useful outcome.
- **The trivial marking correction is right.** An all-procedural corpus violates
  reachability; one triggering `kinds=*` unit per file has no procedural unit to
  make unreachable and remains legal until composition and its size alarm exist.
- **`ordering-key-placement-k6` fits one design session.** It is one bounded
  placement/gate question. If its resulting spec edit proves load-bearing, that
  producer can decide on `review-design` at its normal end boundary; the question
  does not need decomposition now.
- **The release-scan inversion and `INSTRUCTED_VERBS` timing are correctly
  placed.** The scan changes when `grove-llm` links the embed. The pinned verb set
  does not gain `methodology` until `content/MANDATE.md` actually instructs it in
  the successor grove.
- **The successor charter may remain in the root brief until finish.** It is
  explicitly marked "must not lose", and the finish cycle must promote durable
  brief material before teardown. Promoting at finish avoids a second plan that
  can drift while this grove is still changing; the finish session should create
  the durable handoff then.
- **The suffix concern belongs in this tree.** Grove's externalization rule says
  a surfaced concern becomes a leaf in the current tree. B6 is about its shape
  and dependency order, not about moving it to an untracked future grove.

## Done when

- Every B finding is verified and either applied to the tree/briefs or rejected
  with concrete contrary evidence recorded here.
- No future `impl` leaf is inert or waits on a sibling to become useful.
- `classification-k9` is a node whose first live child is the planning needed to
  derive green dependency batches; no classification implementation is absorbed
  into this integration session.
- The suffix decision and any resulting implementation are ordered before the
  real classification, while remaining separate session kinds.
- The methodology-verb contract includes direct identity hashing and tree-free,
  epoch-free inspection, and the successor charter no longer owns identity
  migration.
- The root brief and future task bodies agree on the ordering carrier without
  pre-deciding `ordering-key-placement-k6`.

## Notes

- This is `integrate-review-planning`: reshape the tree and its contracts, then
  verify the resulting order by inspection. Implementation, builds, and tests
  belong to the leaves this session repairs.
