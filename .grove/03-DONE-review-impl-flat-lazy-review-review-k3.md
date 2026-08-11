# flat-lazy-review-review-k3

**Reviews:** flat-lazy-review-k2

## Goal

Adversarially review the `flat-lazy-review-k2` commit. It is deletion-dominant
(~1400 lines of Rust out, three source files and two test files gone) and
rewrites the provisioned methodology in the same commit, so the two risks are
**a deletion that took a live guarantee with it** and **a doc that now describes
something the binary does not do**. This is a meta-grove: whatever `content/`
says at that commit is what drove *you*.

Findings only — do not edit code, do not run `cargo test`/`clippy`/`fmt`. The
paired `integrate-review-impl` leaf owns every fix and all post-fix
verification, and it exists only if you find something worth acting on.

## Context

Verification the producer already ran, so you need not repeat it:

- `cargo test --no-fail-fast`: 39 binaries, **932 passed, 0 failed**.
- `cargo fmt --check`: clean.
- `cargo clippy --all-targets`: 8 warnings, **all in files this commit does not
  touch** (`tree_migration_transaction.rs`, `driver_lease.rs`,
  `finish_cleanup.rs`, `finish_cleanup/auxiliary.rs`, `repo/finish_commit.rs`).
  Confirmed pre-existing by `jj st` — none of those paths is modified. They are
  externalized as `clippy-baseline-k4`, not fixed here.
- Every `grove-llm <verb>` string in `content/` resolves to a live subcommand
  (checked by diffing the names in `content/` against `grove-llm --help`).

Read `.grove/BRIEF.md` for the decision record and the rejected alternatives —
several findings you might raise were considered and refused there, and a
finding that re-litigates one is noise unless it brings a new argument.

## Done when

Every item below has been inspected and either cleared or written up as a
concrete finding with a file, a line, and a failure scenario.

### 1. `add_run`'s destination sweep — an untested guard (`src/tree_grow.rs`)

The flattened `add_run` sweeps **every** planned destination with
`Path::exists()` before the first write. I could not construct a test that
reaches it: a *valid* task-shaped squatter is counted by `read_level`, so
numbering allocates past it, and a *malformed* one makes the read itself refuse
before allocation. So the sweep is reachable only by a writer that never took
the tree lock, and I documented it as exactly that.

- Is that justification sound, or did I miss a reachable path?
- The hazard it guards is real and specific: `fs::write` **truncates**, so
  without the sweep a racing writer's file is silently clobbered — where the old
  node shape got the same protection free from `create_dir`. Confirm that
  reasoning, or say why the guard should go.
- **The case I did not test and suspect is broken:** a **dangling symlink** at a
  planned destination. `Path::exists()` follows the link and reports `false`, so
  the sweep passes; `fs::write` then writes *through* the symlink to its target,
  outside `.grove/`; and if a later step fails, `roll_back`'s `fs::remove_file`
  removes the *symlink*, leaving the written file behind. Verify by reading, and
  say whether `symlink_metadata` is the correct sweep primitive.

### 2. `roll_back`'s unwind (`src/tree_grow.rs`)

It removes exactly the paths it recorded as created, newest first, and names any
it could not remove. Check that `created` cannot contain a path that is not a
regular file this run wrote, and that a partial `fs::write` (file created, bytes
truncated) is still removed rather than left as a valid-looking empty leaf.

### 3. Deleting `Kind::review_steps` — a real loss of compile-time force

The exhaustive `match` forced a *twentieth* kind to declare whether it heads a
review chain. Deleting it (no callers remained) trades that for tests over the
label set in `src/leaf.rs`'s `inline_tests`. Judge the trade:

- Do `every_producer_s_chain_steps_are_its_own_labels_prefixed` and
  `exactly_the_five_producers_have_chain_steps` actually catch what the `match`
  caught — a new producer added without its two steps?
- The `PRODUCERS` const duplicates five variants already in `Kind::ALL`. Is that
  redundancy load-bearing or just a second place to forget?
- Counter-case worth arguing: keeping a caller-less derivation would have been
  dead API the crate deletes on sight (`src/lib.rs`'s header states the rule).

### 4. `resolve_parent_node` — a guard I deliberately did **not** reinstate

With chain nodes gone, every node grove writes carries a `BRIEF.md`, so the old
"parent must have a charter" guard could come back. I argued it should not:
node-ness is structural, a charter is content, and grove validates content
nowhere (constraints 3 and 5). Test that argument — in particular whether any
*reader* now assumes a charter is present and would misbehave without one.

### 5. The two-species collapse, swept for survivors

`BRIEF.md`-presence was the Retire cascade's discriminator. I removed it from
`content/SKILL.md`, `content/BRIEF-FORMAT.md`, `CONTEXT.md`'s **Node directory**
entry, `src/tree_id.rs` and `src/tree_read.rs`. Sweep for any surviving reader,
test, or sentence that still branches on it — a close that skips its `Done when`
rollup because a node "is the other kind" silently drops a real check.

### 6. Doc/binary agreement, and the ADR/spec set

- `docs/specs/doubt-grove-review-mechanics.md` was rewritten heavily: the
  promotion transaction sections are gone and the escalation mechanism replaced.
  Is what remains a coherent *current-state* spec, or a document with a hole
  where its largest section was?
- `docs/adr/grove-owns-escalated-review.md` and
  `docs/adr/task-tree-transactions-fail-closed.md` were edited in place, never
  superseded. Check for dangling citations in either direction.
- The `Compatibility` claim in the spec — that an existing `<stem>-chain/` node
  keeps working untouched and needs no migration — is asserted, not tested.
  Verify it by reading the tree readers, and say whether it deserves a test.

### 7. The `CHANGELOG.md` `## Unreleased` entry

It calls the change **Breaking**. Confirm that is right (two verbs removed, no
tree format change) and that the entry does not overclaim: in particular the
sentence about `PROMOTING-*` no longer being reserved, and what actually happens
to a directory left by an interrupted promotion under an older binary.

## Notes

**Where I am least confident**, in order: item 1's dangling-symlink case (I
believe it is a real defect and chose not to fix it in a leaf that was already
this large), item 3's trade, and item 6's spec coherence. Items 2, 4, 5 and 7 I
inspected and believe are right — disprove them.

**Not findings**, because the brief settled them: no migration; the pair staying
eager while the chain went lazy; `leaf-add-chain` and `leaf-promote-chain` dying
entirely rather than one surviving; `Reviews`/`Integrates` surviving as unparsed
convention; and flat steps being splittable by `leaf-insert`. Re-open one only
with an argument the brief did not consider.
