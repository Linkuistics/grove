# flat-lazy-review-k2

## Goal

Implement flat, lazily-created review composition — code and provisioned
methodology together, in one commit. The root brief holds the full decision
record, the rejected alternatives, and the file-by-file blast radius; read it
rather than rediscovering them.

Two behaviours to land:

- A review's steps are **flat siblings**, never a `<stem>-chain/` node.
- A producer's last act is to `leaf-add` its `review-<producer>` leaf **if review
  is required**; a review's last act is to `leaf-add` its
  `integrate-review-<producer>` leaf **if it has findings**. The creating session
  writes the new leaf's body, putting the specific instructions, findings and
  data into it.

## Context

The change is deletion-dominant. Roughly 1150 lines of Rust come out
(`src/tree_promotion.rs`, `src/task_relationship.rs`) plus two verbs and their
tests. Nothing new is added to the CLI surface: the lazy pattern is expressed
entirely with the existing `grove-llm leaf-add <parent> <slug> --kind <kind>`.

Suggested order — it keeps the tree readable at every step and puts the
mechanical deletions first:

1. Delete `leaf-promote-chain` + `src/tree_promotion.rs` + the `PROMOTING-*`
   witness, recovery, and every reserved-prefix refusal that names it.
2. Delete `leaf-add-chain` + `leaf_add_chain_unlocked`.
3. Delete `src/task_relationship.rs`; drop `Step.relationship` and
   `StepRelationship` from `tree_grow` (the pair declares none).
4. Flatten `add_run` so `leaf-add-pair` appends three flat siblings at
   consecutive positions instead of creating a node. **Keep** the one-snapshot
   key allocation and all-or-nothing rollback.
5. Update `tests/removed_surface.rs`'s classification table and the other
   affected tests.
6. Rework `content/`, `CONTEXT.md`, `docs/` in place, as the current state.

## Done when

The root brief's `Done when` holds. In addition:

- `cargo test` passes and `cargo clippy` is clean — run them and paste the real
  output, do not assert from expectation.
- No provisioned `content/` file instructs a verb that no longer exists. This is
  a meta-grove: whatever `content/` says at this commit is what the *next*
  session is driven by.

## Notes

**Watch for a collapse the brief only flags.** With chain nodes gone, the two
node species merge back into one, so **every node carries a `BRIEF.md`** again.
That deletes the `BRIEF.md`-presence discriminator from the Retire cascade in
`content/SKILL.md` and from `CONTEXT.md`'s **Node directory** entry — several
`_Avoid_` lines in both argue *for* two species and must invert, not merely be
trimmed.

**Check whether `Kind::review_steps` keeps a caller.** It derived the two step
kinds for the constructors being deleted. The derivation is still true and still
useful as guidance, but if nothing calls it, it goes.

**This leaf is the first user of the rule it implements.** Its own last act is to
decide whether a `review-impl` leaf is required. Given the size and the
doc/code coupling, one is likely warranted — if so, write that leaf's body with
the *specific* things this session could not verify, not a generic "review the
change".

**If it proves bigger than one session**, `leaf-decompose` at the genuine seam
and do only the first child. Do not let it sprawl.
