# grove.lazily-create-review-and-integrate-steps — brief

## Goal

Make review composition **flat and lazy**. Two changes, one rule:

1. **No chain node.** A review's steps are ordinary flat siblings in their
   parent directory. The `<stem>-chain/` node directory species goes away; the
   hierarchy it bought is not worth the navigation cost.
2. **Each step creates the next, only when it is required.** The *last act of a
   producer session* is to decide whether review is required and, if so,
   `leaf-add` the `review-<producer>` leaf itself. The *last act of a review
   session* is to `leaf-add` the `integrate-review-<producer>` leaf — **only if
   it has findings worth acting on**. A review that finds nothing creates
   nothing and simply retires.

The payoff of laziness is not only the saved session. Because the creating
session writes the new leaf's body, it can put **specific instructions,
findings, and data** into it — a review leaf that names the exact case its
producer could not cover, an integrate leaf carrying the findings verbatim.
That is strictly more than the generic template a constructor can write up
front, and it is why the creating session is the right author.

## Done when

- `grove-llm leaf-add-chain` and `grove-llm leaf-promote-chain` no longer exist,
  and `tests/removed_surface.rs` classifies both as removed.
- `src/tree_promotion.rs` and the `PROMOTING-*/` transaction (witness, recovery,
  and every refusal that names it) are gone. The `FINISHING-*/` finish
  transaction is **untouched**.
- `src/task_relationship.rs` is gone. The `**Reviews:**` / `**Integrates:**`
  lines survive as a **documented convention in `content/TASK-FORMAT.md`**,
  written by hand by the session authoring the body.
- `grove-llm leaf-add-pair` still emits its three steps **atomically**, but as
  flat siblings with no `<stem>-pair/` node.
- `cargo test` passes; `cargo clippy` clean.
- The glossary, architecture, ADR set and provisioned methodology describe the
  new design **as the current state** — reworked in place, with no superseding
  entry appended and no dangling citation.

## Decomposition

One leaf: `flat-lazy-review-k2`, covering code *and* methodology together.

They are coupled and must not be split. A binary without `leaf-promote-chain`
whose embedded `content/SKILL.md` still instructs it is a broken *build*: the
methodology is compiled in, so the pair ships as one artifact and a split leaf
would cut a release that hands sessions a verb it lacks. `tests/provision.rs`
now enforces exactly that. The work is deletion-heavy (~1150 lines of Rust out),
which is what makes one session tractable.

**Corrected by `provisioned-skill-refresh-k9`:** this paragraph originally
argued the coupling from "the very next session would have that broken skill
provisioned to it." That premise is false — `include_dir!` fixes the embed at
build time, so *no* session in this loop reads this grove's committed
`content/`; every one of them runs the installed v17.0.0 build. The conclusion
survives, at the release boundary rather than the session boundary.

`flat-lazy-review-k2` is the first user of the rule it implements: its own last
act is to decide whether a `review-impl` leaf is required, and to write that
leaf's instructions if so. If the work proves bigger than one session,
`leaf-decompose` it rather than letting it sprawl.

## Pointers

**Rust — delete:**
- `src/tree_promotion.rs` (788 lines) and `tests/leaf_promote_chain.rs`
- `src/task_relationship.rs` (359 lines) and `tests/task_marker_surface.rs`
- `leaf_add_chain_unlocked` in `src/tree_grow.rs`; the `LeafAddChain` /
  `LeafPromoteChain` commands in `src/llm_cli.rs`

**Rust — change:**
- `src/tree_grow.rs` — `add_run` survives for `leaf-add-pair` only. It no longer
  creates a node directory, so it appends N flat siblings at consecutive
  positions; its `Step.relationship` field and the `StepRelationship` struct
  have no remaining user (the pair declares none) and go with it. Keep the
  one-snapshot allocation and all-or-nothing rollback — that contract is why a
  pair is one call.
- `src/tree_access.rs`, `src/tree_read.rs`, `src/tree_id.rs`, `src/tree_rename.rs`,
  `src/lib.rs`, `src/leaf.rs` — remove the `PROMOTING-*` reserved-prefix refusals
  and the chain-node vocabulary. `Kind::review_steps` / `review_steps_or_refuse`
  in `src/leaf.rs` still derive the review and integrate kinds; that derivation
  is now guidance a session applies, not a constructor input, so check whether it
  keeps a caller.
- `tests/removed_surface.rs` — add both verbs to the classification table.
- Also touched: `tests/composition_guidance.rs`, `tests/finish_lifecycle.rs`,
  `tests/jj_tree_verbs.rs`, `tests/leaf_chain.rs`,
  `tests/reviewed_producer_lifecycle.rs`, `tests/session_kind_guidance.rs`,
  `tests/session_kind_tree.rs`.

**Methodology and docs — rework in place:**
- `content/SKILL.md` — the *Compose, don't just append* section, the Decompose
  step, the review-ownership predicate, and the Retire cascade's brief-less-node
  discriminator (with no chain nodes, the two node species collapse back to one:
  **every node carries a `BRIEF.md`**, and the discriminator disappears).
- `content/TASK-FORMAT.md` — document the `**Reviews:**` / `**Integrates:**`
  convention as hand-written; describe the producer's and reviewer's closing
  obligation.
- `content/driving.md`, `content/BRIEF-FORMAT.md`
- `CONTEXT.md` — rework **Review chain / vendor pair**, **Node directory**,
  **Promotion transaction** (delete), **Pick**, **Tree access lock**,
  **Confirmation boundary** and the *"chain"* flagged-ambiguity entry. Several
  carry `_Avoid_` lines that argue *for* chain nodes; those invert.
- `docs/ARCHITECTURE.md` §Task kinds and composition, §Tree access lock and
  promotion transaction.
- `docs/adr/grove-owns-escalated-review.md` — subject survives, mechanism
  changes: escalation now resolves to `leaf-add`, not promotion.
- `docs/adr/task-tree-transactions-fail-closed.md` — loses `PROMOTING-*`, keeps
  `FINISHING-*`.
- `docs/specs/doubt-grove-review-mechanics.md`, `docs/USAGE.md`, `CHANGELOG.md`.

## Notes

**Decisions from the `plan-k1` grilling, with what was rejected:**

- **No migration.** A chain node is just a brief-less node *directory*, and every
  reader already handles node directories generically — the `-chain` token is
  ordinary slug text nothing keys on. Existing trees keep working untouched. The
  change is forward-only; do not write a migration.
- **`leaf-promote-chain` dies entirely**, not just its verb. It existed solely to
  retrofit a chain node around a picked producer without changing its handle,
  which is why it needed a fail-closed transaction with interruption recovery.
  With review leaves flat and on demand, "promotion" is one `leaf-add`. Rejected:
  keeping the transaction scaffolding for a future caller — infrastructure with
  no caller is what gets pruned later anyway.
- **`task_relationship.rs` dies with it.** Its only *reader*, `declaring()`, had
  exactly one caller (`tree_promotion.rs:79`). Its module doc justifies its
  existence as "the two writers and the one reader agreeing by construction" — a
  justification that is load-bearing on promotion existing. Rejected: adding
  `--reviews <handle>` to `leaf-add` to keep one writer; that leaves a module
  agreeing only with itself. The markers stay as freeform convention, which is
  constraint 3 (task files are freeform markdown; nothing validates them).
- **`leaf-add-chain` dies.** It would now emit only a producer, which is
  `leaf-add`. Rejected: keeping it as a producer+review constructor for
  known-load-bearing artifacts — that is the eager behaviour by another name and
  gives two doctrines about when a review leaf exists.
- **The research pair stays eager**, and that asymmetry is deliberate. Lazy
  creation is *wrong* for a vendor pair: if `research-a` created `research-b`,
  `b` would inherit `a`'s framing and corpus, destroying the independence the
  pair is run for. Eagerness is the point for a pair; it is not for a chain.
  Only the node directory goes.
- **Integrate creation is conditional on findings.** A review with nothing worth
  acting on creates nothing. Rejected: always creating it — that keeps the empty
  session this workstream exists to remove.
- **No new ADR.** The chain-node decision was never in `docs/adr/`; it lives in
  `docs/ARCHITECTURE.md` and `CONTEXT.md`, and both are reworked in place. The
  two existing ADRs that touch this are edited, never superseded.

**Known consequence, accepted:** the chain node's one surviving benefit was that
a sibling-level `leaf-insert` could not split a chain. Flat steps can be split by
an insert. Record it where it belongs (`CONTEXT.md`'s **Pick** / **Position**
entries) rather than defending against it — grove validates no cross-leaf
grammar, and contiguity was always a convention rather than an enforced unit.

**Narrowed by `chain-contiguity-k6`**, which this grove's own tree provoked: the
mechanism stays exactly as decided — nothing is enforced, and no chain node
returns — but the *guidance* no longer accepts the cost uniformly. The two hops
differ in what the next step consumes. A `review-*` step **re-derives** from the
producer's commit, so a gap before it is free and `leaf-add` is right wherever it
lands. An `integrate-review-*` step **consumes** `path:line` citations its review
already froze, against a working tree that has since moved, and the drift is
silent — so it is cut adjacent to its review by default, with `leaf-insert`, and
departing needs the intervening work to provably touch no file the findings cite.
