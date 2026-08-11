# flat-lazy-review-integrate-k5

**Integrates:** flat-lazy-review-review-k3

## Goal

Apply the actionable findings from the adversarial review of
`flat-lazy-review-k2`. Preserve the flat-and-lazy composition decision while
restoring the filesystem guarantees and making every current-state document
describe exactly what the binary does.

## Context

The producer commit is `ae9c04c3` (`flat-lazy-review-k2`). It recorded clean
`cargo test --no-fail-fast` and `cargo fmt --check` runs; its clippy baseline is
already externalized as `clippy-baseline-k4`. The review itself was
inspection-only and ran no verification commands.

### Findings

1. **[P1] Make destination creation collision-safe, not merely pre-swept**
   (`src/tree_grow.rs:166`). `Path::exists()` follows symlinks and reports false
   for a dangling one. A writer that ignores Grove's lock can install a dangling
   symlink at a planned destination after the parent snapshot; the sweep passes,
   `fs::write` follows it and creates/truncates the target outside `.grove/`, and
   a later rollback removes the symlink rather than the target. The same TOCTOU
   lets a regular file created after the sweep be truncated and then removed as
   though Grove owned it. A pre-existing task-shaped symlink is correctly refused
   earlier by `read_level`, so this is precisely the unlocked-racer case the
   destination guard claims to cover. `symlink_metadata` is the correct
   no-follow primitive for the up-front sweep (treat only `NotFound` as absent),
   but it is not sufficient across the sweep/write gap: acquire each destination
   with an atomic non-clobbering create such as `OpenOptions::create_new(true)`.

2. **[P1] Include the failing write's newly created path in rollback**
   (`src/tree_grow.rs:175`, `src/tree_grow.rs:610`). `fs::write` can successfully
   create/truncate a file and then return an error from `write_all`; `add_run`
   pushes the path into `created` only after the whole helper succeeds. In that
   failure mode `roll_back` removes the earlier leaves but strands an empty or
   partial, well-formed leaf at the failing path. Record ownership immediately
   after the atomic create and before writing bytes, then unwind that path too.
   Add a deterministic failure seam that errors after creation to pin the
   all-or-nothing-on-reported-error contract.

3. **[P2] Restore exhaustive classification of new session kinds in tests**
   (`src/leaf.rs:258`, `src/leaf.rs:292`). The new label tests prove that the five
   currently hand-listed producers have their two prefixed labels, but they do
   not catch a twentieth kind being introduced as a producer without those
   steps: update `Kind::ALL`, its label/count tests, and the taxonomy, leave
   `PRODUCERS` and the expected five-label vector unchanged, and both new chain
   tests still pass. `PRODUCERS` is therefore a second manual roster, not the
   compile-time force the deleted exhaustive match supplied. Keep the production
   API deleted, but use a test-only exhaustive `match` (or an equivalently
   exhaustive classifier) so every new variant must explicitly be classified as
   producer or non-producer.

4. **[P1] Put retirement before the task commit in every handoff document**
   (`docs/specs/doubt-grove-review-mechanics.md:155`,
   `plugins/linkuistics/skills/doubt-driven-development/SKILL.md:99`). Both
   sequences still say commit, then retire, contradicting `content/SKILL.md`'s
   task boundary. In jj, following that order seals the artifact/review leaf and
   puts the producer's `DONE` rename into the next task's working-copy commit; in
   Git it leaves the rename uncommitted or forces a second commit. Rewrite both
   sequences to cut the next leaf, retire the producer, commit the artifact + new
   leaf + retirement, then complete, and pin the order in the guidance tests.

5. **[P1] Remove or implement the new process-interruption atomicity promises**
   (`docs/ARCHITECTURE.md:378`,
   `docs/specs/doubt-grove-review-mechanics.md:147`). `add_run` rolls back only
   when control returns through its `Err` branch. SIGKILL/process death after the
   first pair leaf is written bypasses `roll_back` and leaves a valid-looking
   partial pair, directly contradicting “Nothing survives an interruption”. A
   killed `leaf-add` can likewise leave a partial file, so “either wrote the leaf
   or did not” overstates `fs::write`. This also contradicts the immediately
   preceding architecture statement that only finish and migration promise
   process-interruption recovery. Either narrow these passages to reported I/O
   failures, or add a real fail-closed transaction and interruption tests.

6. **[P2] Make interrupted `PROMOTING-*` upgrade guidance phase-correct**
   (`CHANGELOG.md:67`,
   `docs/specs/doubt-grove-review-mechanics.md:212`). The old transaction created
   the witness and generated steps before moving the producer, then could prepare
   a tracked Git index entry before landing. Therefore a stranded witness may
   have the original producer still outside, may hold it inside, or may hold it
   while Git's index already names the never-landed final child. “The producer
   copy inside it must be moved back by hand” is false for the first state and
   incomplete for the last. The new reader does correctly treat the unpositioned
   witness as foreign, but recovery instructions must distinguish these states
   (prefer recovery with the old binary before upgrading, or document the exact
   filesystem and Git-index repair).

7. **[P2] Remove surviving current-state claims about a second node species**
   (`docs/ARCHITECTURE.md:214`, `src/tree_grow.rs:1008`). The architecture still
   labels `BRIEF.md` “optional for composition-only nodes”, and the test comment
   still says a charter distinguishes “the two node species”. No constructor now
   creates a composition node; a missing charter is tolerated only as a
   hand-authored lapse/legacy shape. These statements can send a future reader
   back toward the deleted `BRIEF.md` discriminator and silently skip a real
   `Done when` rollup. Describe one structural node species and the reader's
   deliberate tolerance separately.

8. **[P2] Pin the no-migration compatibility promise with one legacy-chain
   fixture** (`docs/specs/doubt-grove-review-mechanics.md:198`). The current
   readers do support the claim: node parsing ignores the `-chain` slug token,
   `read_level` descends directories structurally, `brief_chain` skips a missing
   charter, and recursive resolution finds the children. But no source or test
   fixture contains `-chain-k`, so the release's central compatibility promise
   can regress while all current-shape tests pass. Add one current-format,
   brief-less legacy chain node and assert pick, handle resolution, and
   brief-chain behavior without migration.

## Done when

- Every finding above is classified against the current contract and either
  fixed or recorded as an explicit accepted trade-off with its rationale.
- Filesystem fixes have focused tests for dangling symlinks, post-sweep
  collisions, failure after file creation, and any process-interruption guarantee
  retained in the docs.
- The legacy chain compatibility fixture exercises pick, resolve, and
  brief-chain on one unchanged brief-less `-chain` node.
- `cargo fmt --check`, `cargo test --no-fail-fast`, and
  `cargo clippy --all-targets` are run after the fixes; pre-existing clippy
  findings remain owned by `clippy-baseline-k4`.

## Notes

Cleared during review: the non-testable destination sweep is reachable only for
a writer that ignored the tree lock; reverse-order rollback is correct for paths
it truly owns; no production reader branches on `BRIEF.md` presence; the two
edited ADRs remain a coherent current-state set with live citations; removing
the two public verbs is correctly classified as breaking; and unchanged legacy
chain directories parse and walk without migration.
