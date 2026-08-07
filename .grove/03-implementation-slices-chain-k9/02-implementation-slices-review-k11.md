# implementation-slices-review-k11

**Kind:** review-planning
**Reviews:** implementation-slices-k10
**Producer launch:** {"producer":"implementation-slices-k10","session":"implementation-slices-k10","generation":"k10","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `implementation-slices-k10` and record concrete findings for its integration step.

## Context

- Review the actual generated tree against the integrated design, not a prose
  restatement of it.

## Done when

- Findings cover missing work, horizontal or oversized slices, invalid ordering,
  non-green intermediate states, under-specified acceptance criteria, and
  review-chain misuse; no tree fixes are applied.

## Findings

Reviewed artifact: the twelve implementation slices `implementation-slices-k10`
grew — seven review chains and five lone leaves, 27 task files across positions
`03/04` and `04`–`15` — read against
`docs/specs/config-driven-sessions.md`, the four `docs/adr/` records,
`.grove/BRIEF.md`, `CONTEXT.md`, and the code the sequence rewrites. Cite these
as `implementation-slices-review-k11 P<n>`. No fixes applied.

Baseline established for this review: `cargo test --locked --no-fail-fast`
fails exactly one target, `--test composition_guidance`, one test,
`canonical_guidance_explains_decomposed_receipts_and_pruning_scope`. Everything
else is green. So "each slice lands green" is a checkable claim throughout.

Verdict: the expand → cut over → contract shape is right, the dependency
graph is stated per leaf rather than implied, and the removal list is
traceable end to end. Three defects block: the first code slice requires a
format witness that nothing in its own boundary can write and whose producer
it explicitly disclaims (P1); migration is ordered two chains ahead of the
lock it says it uses (P2); and the epoch slice carries two acceptance criteria
that only become true four slices later (P3). P1–P3 are the blocking set.
P4–P7 are slice-shape defects worth fixing before execution starts; P8–P11 are
under-specified criteria that will each cost a session.

### Blocking

**P1 — `session-kind-tree-k23` requires `.grove/FORMAT` to exist and forbids
itself from writing one; nothing else in its boundary can.**
`session-kind-tree-k23:33` requires current readers to use "the filename
grammar and known format witness", and `:18` fixes the witness as
`.grove/FORMAT` containing `session-kinds-v1`. But `:46` forbids the slice from
adding "driver-side root/finish allocation", and
`session-kind-migration-k27:34-35` claims fresh-tree creation — "the exact root
brief, requirements `plan-k1` filename/body, **and marker**". Today no code
writes the witness at all: `grep -rn FORMAT src/*.rs` returns only three
`TASK-FORMAT.md` prose references (`src/tree_read.rs:250,299,1117`), and
`.grove/FORMAT` does not exist in this working tree.

`root_init` is not driver-side and is not deferred anywhere: it lives at
`src/tree_lifecycle.rs:54`, inside a k23 primary code surface, and writes a
`**Kind:** requirements` body (`:72`, asserted at `:799`). So at k23's own
boundary `grove-llm root-init` produces a tree k23's readers must reject —
no witness, no filename kind — and `tests/root_init.rs` (9 tests, currently
green, including `after_root_init_pick_returns_the_new_leaf_not_done`) fails.
k23 cannot satisfy "`cargo test --locked` passes" and its own reader contract
simultaneously.

Correction: give k23 explicit ownership of current-format *writing* on the
`grove-llm` side — `root-init` emits `FORMAT` and
`01-requirements-plan-k1.md`, grow verbs emit filename kinds — and narrow k27
to legacy conversion plus exact-partial-scaffold recovery. The alternative
(readers tolerate an absent witness until k27) contradicts the spec's
"positive discriminator" (`config-driven-sessions.md:223-240`) and must be
rejected, not left implicit.

**P2 — `session-kind-migration-k27` is ordered before the universal
working-tree lock it says it uses.**
`session-kind-migration-k27:34-35` requires fresh-tree creation "under the
universal tree mutation seam", but that seam is created by
`session-epoch-k35:36-37`: "Tree readers/mutators lock the open working-tree
root across root-init, migration, ordinary verbs, promotion, and finish
deletion." k26 is position 06; k34 is position 08.

This is not a naming quibble — the current lock cannot do the job.
`src/tree_access.rs:58-61` bails `grove root not found` unless `.grove/` is
already a directory, and locks that directory's descriptor, so it cannot
serialize `.grove/`'s own creation. The spec says exactly this
(`config-driven-sessions.md:502-514`): the working-tree root "exists before
`.grove/` and survives its deletion", which is why the lock had to move.

Correction: cut the universal-lock migration out of k35 into its own leaf
sequenced before `session-kind-migration-chain-k26` (it is independently
demoable: every existing verb keeps working, root-init becomes lockable), or
restate k27 against the existing `.grove/`-descriptor lock and make k35's
retrofit of root-init/migration/finish explicit. Leaving the phrase as-is
gives the k27 session a seam that does not exist.

**P3 — `session-epoch-k35` carries two acceptance criteria that only
`finish-lifecycle-k43` can make true.**
`session-epoch-k35:36-37` lists "finish deletion" among the operations that
must hold the working-tree lock, and `:44` requires tests covering
"finish-delete/root-recreate handle reuse". Driver-owned finish allocation and
`grove-llm finish-commit` — the only things that delete `.grove/` — are
`finish-lifecycle-k43`'s Done-when. k43 is position 10; k34 is position 08.
Neither bullet is checkable at k35's boundary, so a k35 session either
hand-rolls a stand-in deletion (proving nothing about the real path) or
declares the criterion vacuous.

Correction: move the finish-related lock coverage and the
finish-delete/root-recreate reuse test into k43's Done-when, and restate k35's
handle-reuse criterion against an operation that exists at k35 — deleting the
tree directly and re-running `root-init` exercises the same epoch/nonce
property without depending on the helper.

### Slice shape and review coverage

**P4 — `session-epoch-k35` is four independently demoable changes in one
leaf.**
Its Goal (`:7-9`) already names three: the epoch binding, "move all tree
operations onto the universal working-tree lock", and "make the loop signal
path independently random per launch"; its Done-when adds a fourth, the
meta-grove env-hygiene guard (`:42-43`). The lock migration touches every
verb and no epoch code; the random per-launch signal path plus
abandoned-signal cleanup is a separate contract with its own stated collision
bound. This is the largest leaf in the sequence against the longest section of
the spec (`config-driven-sessions.md:347-520`), and P2 shows the bundling has
already produced a real ordering defect. Correction: extract the lock
migration (per P2) and, if the remainder still reads long, the signal-path
draw/cleanup.

**P5 — `methodology-and-viewer-k48` is a horizontal slice: it groups three
artifacts by presentation layer, across two bounded contexts.**
`implementation-slices-k10:52-54` states the grouping rule out loud —
`methodology-and-viewer-k48` and `durable-docs-reconciliation-k49` "reconcile
the two durable presentation layers". Grouping by layer is the definition of a
horizontal slice. Concretely, k48 (`:14-18`) bundles: `content/` (the
methodology the binary provisions), `plugins/linkuistics/skills/
doubt-driven-development/` — which `CONTEXT-MAP.md` assigns to the **skills**
bounded context, not grove — and `herdr-plugin/` (a separately installed,
separately versioned `python3` renderer; CONTEXT.md's **Tree viewer plugin**
entry makes independent versioning the property that keeps "optimised-for"
real).

It also serializes the viewer needlessly. The viewer's only contract is the
filename grammar, so it can land immediately after
`session-kind-tree-integrate-k25` and be demoed on its own ("a nineteen-kind
tree renders"); k48's `:12` dependency on `legacy-review-removal-k47` holds it
back five slices for a reason that applies to the methodology text, not to
`grove_tree.py`. Correction: cut the viewer as its own leaf after k25.

**P6 — Three load-bearing artifacts land unreviewed on the strength of a
"mechanical" label that two of them do not fit.**
`implementation-slices-k10:56-58` justifies the five trailing lone leaves as
"mechanical, documentation, and acceptance". That holds for
`legacy-launch-removal-k46` (a deletion sweep) and for
`acceptance-verification-k50`. It does not hold for:

- `legacy-review-removal-k47` — it reshapes retirement side effects, promotion
  authority, and the one-review-ownership rule, all governed by the live
  `docs/adr/grove-owns-escalated-review.md`, and its own Notes (`:42-44`)
  defer a module-splitting *judgement* into it. That is design work sitting
  alone between two reviewed neighbours.
- `methodology-and-viewer-k48` — it rewrites the provisioned methodology, the
  artifact every future grove session reads and the thing the binary exists to
  deliver.
- `durable-docs-reconciliation-k49` — it reworks the minimum coherent ADR/spec
  set in place, which grove's own rule names as the canonical
  cut-a-chain-proactively case.

Correction: promote k48 and k49 to review chains; for k47 either cut a chain
or externalise the `task_relationship.rs` split as its own leaf so the
deletion stays genuinely mechanical.

**P7 — `receipt-guidance-test-cleanup-k17` is parked inside a brief-less chain
node, contradicting its own producer's Done-when.**
It sits at `.grove/03-implementation-slices-chain-k9/04-…`, i.e. as a fourth
child of the review chain composing the *planning* artifact, while
`implementation-slices-k10:20` requires that "the **root tree** contains the
full implementation sequence". CONTEXT.md's **Node directory** entry defines a
chain node as meaning "these steps compose one artifact"; an unrelated `impl`
leaf is not one of those steps. Cost: the viewer collapses a finished chain to
one counted line, so the first implementation slice disappears from the tree
view, and the chain node stops being a three-step composition.

The placement also buys nothing — pick order is identical either way, since
the chain's own steps precede any root sibling at position 04. Correction:
`leaf-insert` it at root position 04, ahead of `session-config-chain-k18`.

### Under-specified acceptance criteria

**P8 — k17's stated evidence understates the failure set, in a file k17 does
not own.**
`receipt-guidance-test-cleanup-k17:13-22` names only the two `CONTEXT.md` rows.
Measured now, three of the test's fifteen rows fail: `CONTEXT.md` ×
"factual source session", `CONTEXT.md` × "producer generation", and — the one
not named — the `SPEC` constant (`tests/composition_guidance.rs:12` =
`docs/specs/doubt-grove-review-mechanics.md`) × "legacy node receipts are
uncheckable". The assertion normalises whitespace
(`tests/composition_guidance.rs:14-21`), so the other twelve rows genuinely
still pass. A session following the brief literally fixes two rows and trips
the third, in a spec `durable-docs-reconciliation-k49:32-34` owns.
Correction: name all three rows and state whether k17 may edit the
review-mechanics spec or must relax that row and leave the spec to k49.

**P9 — The generated `finish` leaf has no specified body.**
`finish-lifecycle-k43` specifies the filename, position, key, handle, reuse,
reservation and commit behaviour, and never says what is *in* the file. The
spec has the same hole: `config-driven-sessions.md:547-557` gives
`NN-finish-finish-k<key>.md` and the handle only, while the sibling case at
`:526-527` is explicit — "The leaf body is the ordinary `plan-k1` requirements
task with no body kind marker." Bootstrap reads the task file, so this is a
real artifact, not a formality: an empty body makes the HITL finish session
depend wholly on provisioned methodology that `methodology-and-viewer-k48`
rewrites five slices earlier. Correction: give k43 an acceptance criterion for
the generated body (Goal plus a Done-when naming promote → `finish-commit` →
`complete --done` in order), or state that it is deliberately empty and why.

**P10 — Nothing owns "the `grove-llm` path in the Herdr hook JSON is the
version-checked one".**
`config-driven-sessions.md:344-345` makes this an explicit contract — "The same
resolved absolute path is embedded in Herdr turn-hook JSON, so the hooks and
the version-checked agent interface cannot drift" — and the test-seam list
(`:816`) requires "the exact path reused by Herdr hooks".
`lifecycle-cutover-k39` covers sibling resolution and the version check
(`:29-31`) and lists `src/herdr.rs` among its surfaces (`:20`), but no
Done-when bullet in k39, k46 or k50 asserts the two are the same path.
Correction: add the clause to k39's Done-when.

**P11 — The meta-grove consequence of this sequence is unnamed.**
`legacy-launch-removal-k46` deletes `grove do`, the verb currently driving this
workstream, and `session-kind-migration-k27` will rewrite this repo's own
`.grove/` — which has no `FORMAT` and carries `**Kind:**` bodies. The
resolution is presumably "the installed 16.5.0 binary keeps driving; do not
install the branch until the grove finishes", consistent with
`acceptance-verification-k50:46-47`. CONTEXT.md keeps a whole **Meta-grove**
entry because this class of hazard is invisible everywhere else and routine
here, so leaving it implicit across a twelve-slice sequence is the omission.
Correction: record the constraint once, in k46 or k50.

### Checked and clear

Recorded so `implementation-slices-integrate-k12` does not re-derive them:
every code surface named across the twelve leaves exists (`src/`, `tests/`,
`content/`, `scripts/` all verified); the "positive and cross-tree controls"
phrase used by k46/k47/k49/k50 is grounded in `content/driving.md:375`, not
invented jargon; `src/json.rs` survives k47's removal of `--json` routing
evidence because `json::escape` has three other callers
(`src/herdr.rs:361`, `src/launch.rs:661`, `src/llm_cli.rs:717`); the spec's
removal list maps onto k46/k47/k48/k49 with no unassigned entry; and the
review-mechanics spec split that `implementation-slices-k10:33-34` demanded is
carried by `durable-docs-reconciliation-k49:32-34`.

## Notes
