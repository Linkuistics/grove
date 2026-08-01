# decomposed-producer-receipt-implementation-review-k26

**Kind:** review-impl

**Reviews:** decomposed-producer-receipt-implementation-k25

## Goal

Adversarially review the decomposed-producer receipt implementation for
lifecycle correctness, stale-generation safety, compatibility, and fidelity to
the design.

## Context

Review `decomposed-producer-receipt-implementation-k25` against
`decomposed-producer-receipt-k20`, the updated `review-target-receipts` ADR, and
the receipt contract in `docs/specs/doubt-grove-review-mechanics.md`. Produce
findings only.

## Done when

- Direct leaves, one-level and nested node-close cascades, supported reopen,
  kind-agnostic closing descendants, legacy and unknown-field receipts, and
  advisory failure paths are challenged.
- Factual-pick/worktree/routed-session checks and DONE-first ordering are
  verified at public seams. Source-session/generation divergence and the
  at-most-one-live-review cascade invariant are disproved if possible.
- Terminal reviews remain byte-identical with `review-terminal`; pruning scope,
  distinct checkable source-session warnings, nested `producer-target` evidence,
  and launch-window notice discard behavior are exercised end to end.
- Documentation and code use producer identity, source session, and producer
  generation consistently.
- Findings are severity-ranked, reproducible, and recorded here for
  `decomposed-producer-receipt-implementation-integrate-k27`.

## Notes

Do not broaden this into the distinct decomposed-review relationship-carrier
finding.

## Findings

Artifacts reviewed: commit `ea711970a001`
(`decomposed-producer-receipt-implementation-k25`) in full — `src/task_relationship.rs`,
`src/tree_lifecycle.rs`, `src/tree_read.rs`, `src/llm_cli.rs`, `src/loop_driver.rs`,
`tests/producer_receipt.rs`, `tests/kind.rs`, `tests/loop_driver.rs`,
`tests/composition_guidance.rs` — against `docs/adr/review-target-receipts.md`
(whole file), `docs/specs/doubt-grove-review-mechanics.md` *Producer handoff*
(243-268), *Advisory target diversity* (270-372), *Module interfaces* (374-420),
*Test seams* (422-508), *Canonical surfaces* (510-537) and *Compatibility*
(539-573), and the reconciled documentation surfaces.

Claims marked **[measured]** were produced by driving the built
`target/debug/grove-llm` against throwaway groves outside the repository; claims
marked **[read]** cite current source. Baseline: `cargo test --all` is green (30
binaries, 707 tests, 0 failures), `cargo clippy --all-targets` and
`cargo fmt --check` are clean. The working copy carries no source changes.

### Done-when trace

| Clause (from `decomposed-producer-receipt-implementation-k25`) | Verdict | Where |
|---|---|---|
| direct leaves, nested cascades, reopen, kind-agnostic closing descendants, legacy/unknown-field receipts | **met** — happy paths verified end to end [measured] | non-findings |
| advisory failure paths | **not met** — candidate computation is fail-closed and aborts retirement before `DONE` | I1 |
| factual-pick / worktree / routed-session checks, `DONE`-first ordering | **met** at public seams | non-findings |
| source-session/generation divergence, at-most-one-live-review cascade invariant | **not disproved** — both hold; the cascade invariant is emergent, not enforced, and the emergence argument is sound | non-findings |
| terminal reviews byte-identical, pruning scope, source-session warning, nested `producer-target`, launch-window notice | **met** | non-findings |
| documentation and code consistent on producer / source session / generation | **met for the happy path; the advisory-failure contract is now documented but not implemented** | I1 |

### I1 — advisory receipt preparation is fail-closed and can abort `leaf-retire` before `DONE` (severity: high)

`src/tree_lifecycle.rs:236-241` consumes the new candidate walk with `?`:

```rust
let receipt_candidates = producer_receipt_candidates(
    &grove_abs, &producer_path, &factual_leaf_handle, factual_leaf_key,
)?;
```

That contradicts the comment three lines above it (`src/tree_lifecycle.rs:231-234`,
"Every failure here is retained as a diagnostic plan rather than returned:
metadata must never become lifecycle-critical"), the ADR ("Grove still does not
block retirement on advisory metadata"; rejected option *Make a missing receipt
block retirement or review* — "lifecycle correctness must not depend on metadata
that can be absent from legacy or hand-edited chains"), and the spec test seam
"Metadata failures produce `uncheckable` and never block retirement or launch".

Two reproducible triggers, both **[measured]**:

1. A foreign extension-less file whose name parses as a node
   (`.grove/01-node-k1/03-foreign-k99`) inside a brief-carrying node.
   `collect_live_leaves` (`src/tree_lifecycle.rs:301-317`) matches it as
   `Entry::Node` without a `file_type` check and calls `fs::read_dir` on a
   regular file:

   ```
   $ grove-llm pick
   …/.grove/01-node-k1/01-leaf-k2.md
   $ grove-llm leaf-retire .grove/01-node-k1/01-leaf-k2.md
   Error: reading …/.grove/01-node-k1/03-foreign-k99
   Caused by: Not a directory (os error 20)        # exit 1, leaf still live
   ```

2. A foreign (unparseable) ancestor directory that carries a `BRIEF.md`
   (`.grove/scratch/BRIEF.md` + `.grove/scratch/01-leaf-k2.md`).
   `producer_receipt_candidates` gates on `BRIEF.md` presence *before* parsing
   the directory name (`src/tree_lifecycle.rs:273-280`):

   ```
   $ grove-llm leaf-retire .grove/scratch/01-leaf-k2.md
   Error: invalid decomposition node …/.grove/scratch      # exit 1
   ```

   This one is a plain regression: before `ea711970a001` the same call renamed
   the leaf to `DONE`, because the pre-change `prepare_producer_receipt` returned
   a plan rather than a `Result` and no ancestor walk existed. Third trigger of
   the same shape: any `fs::read_dir`/`collect_all` failure reached through
   `producer_generation_unlocked(node)?` (`src/tree_lifecycle.rs:287`).

Consequence, and why this is ranked above the rest: `pick` returns the leaf and
`leaf-retire` refuses it, so the session can neither retire nor legitimately
`complete`. Under `grove do` every relaunch re-derives the same pick and the
self-driving loop wedges on a leaf it cannot close — the one failure mode
"guides, does not gate" exists to prevent, reached through metadata the design
declares advisory.

The asymmetry is internal to the feature, which makes the fix unambiguous: every
*other* failure in the same path already degrades correctly — `unique_review_sibling`
errors become `ReceiptDiagnostic` via `diagnostic()`
(`src/task_relationship.rs:535-539`), and `PreparedReceipt::new` errors become
`ReceiptPlan::Uncheckable` (`src/task_relationship.rs:561-564`). Only the
candidate walk propagates. Fix shape: make `producer_receipt_candidates`
infallible — keep the direct-leaf candidate unconditionally, and on any error
walking or naming an ancestor drop that ancestor and emit an
`uncheckable(reason=…)` diagnostic instead of returning `Err`.

### I2 — the two new live-leaf walkers classify foreign entries differently from `read_level`, so node closure is decided from a different tree than `pick` walks (severity: medium)

`src/tree_lifecycle.rs:301-317` (`collect_live_leaves`, writer) and
`src/task_relationship.rs:316-332` (`contains_live_leaf`, reader) both decide
`Entry::Leaf` / `Entry::Node` from the filename alone. `tree_read::read_level`
(`src/tree_read.rs:558-582`) — the walk behind `pick`, `resolve`, `collect_all`
and therefore `producer_generation_unlocked` — additionally stats the entry and
treats a kind mismatch as foreign: "A node is a directory; a brief and a leaf are
files. A kind mismatch (e.g. a directory named like a leaf) is foreign — never a
task." The spec's Compatibility section holds that line explicitly: "all other
foreign files retain the existing lenient behavior", and "Pick, brief-chain, node
retirement, terminal infixes, the brief-less-node discriminator … continue to
read filenames or `BRIEF.md` presence exactly as before."

Both directions are observable **[measured]**, using a directory named like a
live leaf (`.grove/01-node-k1/09-decoy-k98.md/`) in an otherwise-correct
decomposed chain that produces a `checkable` receipt without it:

- Writer: retirement of the node's last real live leaf succeeds (exit 0) but the
  node is judged still-open, so **no receipt is written and nothing is printed** —
  a silent downgrade of a checkable comparison to `producer-receipt-missing` at
  the review's launch. `pick` had already skipped the decoy.
- Reader: with the receipt already in place, the same decoy flips the guarded
  peek from
  `{"status":"checkable","producer":"node-k1","session":"leaf-k2","generation":"k2",…}`
  to `{"status":"uncheckable","producer":"node-k1","reason":"reviewed-producer-live"}`.
  A node-shaped regular file yields `reason=reviewed-producer-unreadable`.

The reader half degrades correctly (advisory, non-blocking) and is only a fidelity
bug; the writer half is silent, and its `Entry::Node` arm is also trigger 1 of I1.
Fix shape: give both a single lenient walker — either call `tree_read::read_level`
or hoist one shared helper — so "has a live leaf" means the same thing to
retirement, to the receipt reader, and to `pick`.

### I3 — receipt diagnostic reason codes are derived by substring-matching a human-readable error message (severity: low)

`src/task_relationship.rs:753-767`:

```rust
let reason = if detail.contains("more than one sibling") {
    "review-relationship-ambiguous"
} else if detail.contains("Reviews") {
    "review-relationship-malformed"
} else { "receipt-preparation-failed" };
```

Reason codes are an observable contract — `CONTEXT.md`, `docs/ARCHITECTURE.md`
and `docs/CONFIGURATION.md` enumerate them, and `tests/producer_receipt.rs`
asserts on them — but they are reconstructed here from prose that
`anyhow::Context` composed. Rewording either `bail!` in `unique_review_sibling`
silently reclassifies the diagnostic with no test failure, and any I/O error
whose path happens to contain the literal `Reviews` is misreported as
`review-relationship-malformed`. Fix shape: return a typed error (or a
`(reason, detail)` pair) from `unique_review_sibling` and delete the classifier.

Related nit, same file: `ReviewEvidence::Uncheckable::producer` is deserialized
with `deserialize_nullable_model` (`src/task_relationship.rs:179`). The helper is
correct — it forces the field to be *present* while allowing `null`, which
`tests/…::review_evidence_wire_requires_nullable_producer_and_rejects_unknown_fields`
depends on — but its name asserts something about models, so the load-bearing
intent is invisible at all three call sites. Rename to something like
`deserialize_required_nullable`.

### I4 — the reader admits an `ABANDONED` producer as a valid terminal producer (severity: low)

`is_terminal_leaf` (`src/task_relationship.rs:304-314`) matches
`Outcome::Done | Outcome::Abandoned`, and `review_evidence_unlocked` uses it both
for the direct-leaf producer gate (`:244`) and for the node's source-session gate
(`:275`). The ADR is explicit the other way: "A producer entity closed only by
pruning remains deliberately uncheckable: an `ABANDONED` transition records a
human decision against work, not a session that produced the artifact for review."

Not reachable through Grove's own writes — pruning writes no receipt, so a pruned
producer reaches `producer-receipt-missing` first, which is what
`pruning_the_producer_and_pruning_the_enclosing_chain_have_distinct_scope`
observes; and a `DONE` leaf cannot subsequently be pruned. So this is a
defence-in-depth gap against hand-edited trees rather than a live defect, which
is why it sits below I3's contract fragility. Fix shape: accept only
`Outcome::Done` for the producer gate and add the `ABANDONED`-producer-with-receipt
case to the wire tests; keep `Abandoned` accepted for the source-session gate only
if a rationale is recorded.

### I5 — no test asserts the advisory guarantee in the negative direction (severity: low)

`tests/producer_receipt.rs` covers a failed `DONE` rename writing no receipt
(`a_failed_done_rename_writes_no_new_receipt`), a post-`DONE` write failure
(`post_done_write_failure_stays_advisory_and_marks_old_receipt_stale`), and every
uncheckable *relationship* shape — but nothing exercises a failure inside
**candidate computation**, which is the one step that runs before `DONE` and now
returns `Result`. That is exactly the hole I1 occupies, and it is why a green
707-test suite says nothing about it. Add a test that plants a foreign entry
(either shape from I1/I2) in a brief-carrying ancestor's subtree and asserts
`leaf-retire` still exits 0, still applies `DONE`, and reports `uncheckable`.

Secondary, from the spec's own seam list (`docs/specs/…:478-479`): "prove
reopening a producer whose linked review is already terminal does not reactivate
or rewrite the review" is only established compositionally — by
`a_terminal_linked_review_is_preserved_and_diagnosed` (terminal review, no
reopen) plus `producer_generation_survives_reorder_and_changes_after_supported_reopen`
(reopen, live review). The stated combination has no direct test.

### Challenged and found correct

Recorded so `…-integrate-k27` does not re-derive them:

- **At-most-one-live-review cascade invariant.** Not enforced anywhere in code;
  it emerges because a producer's review is always a *sibling*, hence a
  descendant of every outer producer, so a live review keeps every enclosing node
  open. Traced against `node_closes_when_leaf_retires` for the direct-leaf,
  one-level and nested cases; `a_close_cascade_materialises_at_most_one_live_linked_review`
  and `one_done_transition_can_close_nested_reviewed_producers` pin both
  directions. No counterexample found.
- **Source-session / generation divergence.** `generation` is the max key at or
  below the producer and `session` is the closing leaf's handle; the fixtures
  genuinely separate them (receipt `"session":"finish-k6","generation":"k9"`).
  Reorder-stability and reopen-staleness verified end to end.
- **`DONE`-first ordering.** `rename_entry` precedes `receipts.materialize()`
  unconditionally (`src/tree_lifecycle.rs:250-253`); `PreparedReceipt::write`
  re-reads after `DONE` and replaces only the marker line, preserving a late edit.
- **One guarded read, no second metadata open.** `review_evidence_unlocked` is
  called only from `launch_peek` under `tree_access::read`
  (`src/tree_read.rs:205-224`); the driver consumes `leaf.review` and never
  re-reads (`src/loop_driver.rs:392-403`). Version skew is caught both ways by
  `deny_unknown_fields` plus the `review_kind != wire.review.is_some()` check,
  and `KindPeek::Degraded` bails before launch (`src/loop_driver.rs:937`).
- **Model equality.** All ten cases in the ADR's table behave as specified,
  including same-harness `None`/`None` matching and cross-harness `None`/`None`
  diverging; explicit selectors render quoted so a model literally named
  `default(claude)` cannot be confused with a harness default.
- **Wire compatibility.** Unknown keys ignored; `session`/`generation` all-or-nothing;
  legacy direct-leaf receipts derive both facts; legacy node receipts are
  `producer-receipt-legacy-node`; a pre-rule strict reader rejects the new fields.
- **Documentation.** `CONTEXT.md`, `content/SKILL.md`, `content/TASK-FORMAT.md`,
  `content/driving.md`, `docs/ARCHITECTURE.md`, `docs/USAGE.md`,
  `docs/CONFIGURATION.md`, the doubt-driven skill and the three `--help` surfaces
  tell one consistent story about producer identity, source session, generation,
  the terminal-review skip and pruning scope. The single inconsistency is that
  four of them now assert the advisory-failure guarantee that I1 breaks.
