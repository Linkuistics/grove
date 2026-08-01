# doubt-grove-implementation-review-k8

**Kind:** review-impl

## Goal

Adversarially review the implemented doubt/Grove composition for correctness,
safety, compatibility, and fidelity to the integrated design.

## Context

Review the diff and tests from `doubt-grove-implementation-k7` against the root
brief and integrated design. Produce findings only. Exercise public CLI behavior
at the highest available test seam; inspect both jj and git paths.

## Done when

- Atomicity, key preservation, tree ordering, current-leaf continuation, routing
  comparison, non-blocking warnings, and restart behavior are challenged.
- Every task-kind branch and the one-review budget are checked across canonical
  Grove and doubt skill text.
- Backward compatibility and standalone doubt behavior are checked.
- Findings are severity-ranked, reproducible, and recorded in this leaf for
  `doubt-grove-implementation-integrate-k9`.

## Notes

Assume passing tests are incomplete evidence; seek missing assertions and
contracts rather than summarizing the change.

## Findings

Baseline verified before reviewing: `cargo test --all` 674 passed / 0 failed,
`cargo fmt --check` clean, `cargo clippy --all-targets --all-features` silent.
The behaviours behind F3–F5 were each exercised by hand and found **correct** —
they are missing assertions, not defects. F1, F2, F6 and F7 are shortfalls in
what the code or tests actually deliver.

Cite these as `doubt-grove-implementation-review-k8 F<n>` (task-tree-scheme §5:
an index is scoped by its review's handle, never quoted bare).

### F1 — the guidance tests pass with two mandated rules deleted (highest)

`tests/composition_guidance.rs` is the *only* guard on the doubt/Grove contract
text, and its `grove_guidance_replaces_the_old_in_session_review_loop` asserts
the tokens `"research"` and `"integrate-review-*"` against `content/SKILL.md`,
`content/driving.md` and `content/TASK-FORMAT.md`. Both tokens occur throughout
those files as part of the ordinary task-kind taxonomy, so the assertions are
satisfied by prose unrelated to the composition rules.

Reproduced: delete this sentence from `content/SKILL.md`'s new "Review ownership
inside a picked leaf" block —

> A chained producer, `review-*`, and every research-pair leaf spawn none;
> `integrate-review-*` may spend one narrow reviewer, then externalises
> substantial redesign inside the owning chain node.

— and `cargo test --test composition_guidance` still reports **4 passed**. That
removes two of the six current-state facts the spec requires be *present*
(research exclusions, integration placement) with nothing failing.

This is the exact substitution `docs/specs/doubt-grove-review-mechanics.md`
forbids in its last test seam: "Do not substitute a row-coverage assertion that
can pass while both old and new rules coexist." The absence assertions
(`after three rounds`, `big enough that a subagent cannot hold it`, the two
retrofit strings) are correctly contradiction-shaped; the presence assertions
are not. Replace the vacuous tokens with phrases that only the new rules
contain — e.g. the research-pair exclusion and the "inside the owning chain
node" placement — for all three Grove surfaces and for `DOUBT_SKILL`, whose
`"research"` assertion has the same defect.

### F2 — promotion's kind refusal names a remedy the verb does not have

`Kind::review_steps_or_refuse()` (`src/leaf.rs`) is shared by
`tree_grow::leaf_add_chain` and `tree_promotion` (both `promote_new` and
`recover_pending`), and its message is written for `leaf-add-chain`'s `--kind`
flag. Promotion has no such flag — the spec is explicit that "the operation
takes no parent, stem, kind, or harness flags: all four are facts already
carried by the picked producer" — so the refusal instructs the session to do
something impossible.

Reproduced (a `review-impl` leaf as the current pick):

```
$ grove-llm leaf-promote-chain "$(grove-llm pick)"
Error: promotion-failed: `review-impl` is already a review-chain *step*, not the
producer that heads one. Pass the producer kind — one of `requirements`,
`design`, `planning`, `prototype`, `impl` — and the verb derives
`review-<producer>` and `integrate-review-<producer>` itself.
```

The `research` arm has the same shape ("Use `leaf-add-pair <parent> <stem>
--harness-a … --harness-b …`"), which is at least a real verb but still answers
a question promotion's caller did not ask. The spec's contract is "A refused run
changes nothing, consumes no key, and **names the appropriate Grove action**";
for promotion the appropriate action is *this leaf is already a review step —
run it, or promote the producer it reviews*. Give the two call sites distinct
remedies rather than one shared string.

Two neighbouring refusals name no action at all and are worth the same pass:
`producer <handle> already has scheduled review work: <paths>` and `producer
<handle> is already inside a brief-less composition-managed node: <path>`.

### F3 — every promotion test uses a lone root-level producer

All fifteen tests in `tests/leaf_promote_chain.rs` build their tree with the
`grove()` helper, which writes exactly one leaf, `.grove/01-sync-k1.md`, at the
grove root. Consequences, each a seam the spec named and no test touches:

- **"unchanged sibling positions"** is never asserted — no test has siblings
  around the producer at promotion time.
- **Position reuse** at anything other than `01` is never asserted, though the
  spec's worked example is deliberately `05`.
- **"decomposition-node parents"** (a producer inside a `BRIEF.md`-carrying
  node) is never promoted. Only the root-without-a-brief case
  (`root_without_a_brief_is_not_misclassified_as_composition_managed`) and the
  brief-less refusal exist.

Verified correct by hand — a tree of `01-DONE-first-k1.md`, `02-mid-k2.md`,
`03-last-k3.md` promotes `mid-k2` into `02-mid-chain-k4/` with `01-DONE-` and
`03-` untouched and `pick` returning `02-mid-chain-k4/01-mid-k2.md`. So this is
an assertion gap, not a bug; add a nested/positioned fixture beside `grove()`.

### F4 — no promotion-versus-other-mutator concurrency test

The spec's seam is "promotion versus promotion **and** promotion versus every
existing mutator". Only `a_serialized_second_promoter_waits_then_returns_the_
completed_shape` exists; `leaf-add`, `leaf-insert`, `leaf-decompose`,
`leaf-retire` and `leaf-prune` are never driven against a paused promotion.

Verified correct by hand: with a promotion paused at `after-producer-move`, a
concurrent `grove-llm leaf-add . later` blocks, emits exactly one `waiting for
active Grove tree operation`, and lands `02-later-k5.md` with a fresh key after
the promotion completes. Again an assertion gap — but it is the assertion that
would catch a future mutator added without a `tree_access` guard, which is the
whole point of the seam.

### F5 — failure injection barely reaches the Git-index and jj paths

`reported_failures_roll_back_without_consuming_a_key` loops all four
checkpoints, but its fixture never runs `git add`, so `capture_git_index_entry`
returns `None` throughout: `prepare_promotion_index` and
`restore_promotion_index` are both dead code in that test, and
`after-index-prepare` is a no-op position. The only tracked-Git failure test,
`tracked_git_reported_failure_restores_the_original_index_path`, injects at
`after-index-prepare` alone — so `restore_promotion_index`'s *other* two
branches (index at the staging path, index still at the original) are never
exercised against a real index entry.

Both Jujutsu configurations are happy-path only. The spec asks for interruption
coverage across "tracked Git, untracked Git, native Jujutsu, and colocated
Jujutsu"; neither jj test injects a failure or a kill.

Verified correct by hand in the two uncovered tracked-Git checkpoints
(`after-generated-steps`, `after-producer-move`: file restored, index back to
`.grove/01-sync-k1.md`, no residue) and in colocated jj (SIGKILL at
`after-producer-move` leaves a blocking `PROMOTING-` witness, `pick` refuses with
the recovery command, the Git index is untouched, and recovery lands the chain
with `pick` returning the producer). Assertion gap.

### F6 — the idempotent-retry contract breaks once the review leaf is decomposed

`completed_shape` requires exactly one *file* at the producer's sibling level
declaring `**Reviews:** <handle>`. Decomposing the review turns it into a
directory, `declarations` skips directories, the completed shape is no longer
recognised, and the call falls through to `promote_new` — whose
`resolve_producer` then re-tries the stale absolute path as a key/slug
reference and fails.

Reproduced:

```
$ grove-llm leaf-promote-chain .../.grove/01-sync-k1.md      # ok
$ grove-llm leaf-retire .../01-sync-chain-k2/01-sync-k1.md   # ok
$ grove-llm leaf-decompose .../01-sync-chain-k2/02-sync-review-k3.md first-pass
$ grove-llm leaf-promote-chain .../.grove/01-sync-k1.md --json
Error: promotion-failed: producer reference not found: .../.grove/01-sync-k1.md
```

The producer plainly exists as `01-sync-chain-k2/01-DONE-sync-k1.md`, so the
diagnostic is wrong about *why* it refused. The sequence is a documented one —
the spec's Compatibility section explicitly contemplates decomposing a `review-*`
leaf — and the failure is safe (nothing is written, no key consumed), so this is
low severity. It is adjacent to but distinct from `decomposed-producer-receipt-k20`,
which concerns a decomposed *producer*. Either recognise a node child's `BRIEF.md`
as the relationship carrier in `completed_shape`, or make the fall-through
diagnostic name the terminal producer it actually found.

### F7 — nothing end-to-end asserts the silent (diverse) path

`a_review_diversity_warning_is_emitted_once_and_prepended_to_the_prompt` and
`uncheckable_review_metadata_warns_without_inventing_a_producer` between them
cover every *warning* case through the driver, and the unit table in
`src/task_relationship.rs` covers `Diverse` at the comparison layer. No test
drives a review launch whose harness *and* exact model both differ from the
receipt and asserts that stderr carries no warning and the launched prompt is
**not** prefixed. A regression that rendered `Diverse` as a notice — or that
prepended unconditionally — would pass the whole suite bar the unit table. The
spec's "If both differ, stay silent" is the one comparison outcome with no
end-to-end evidence; one more case in the existing loop-driver test closes it.

### Observation (not a finding)

`render_launch_target` formats an explicit model with `{model:?}`, so notices
read `producer-target=claude/"opus"` — Rust `Debug` quoting in operator-facing
text. It is deliberate (`review_warning_names_only_a_valid_relationship_and_
renders_defaults` asserts a newline is escaped rather than breaking the one-line
notice) and it disambiguates a literal model named `default(claude)`, so it is a
defensible trade-off rather than a defect. Noting it only so the integration
leaf does not rediscover it as a typo.
