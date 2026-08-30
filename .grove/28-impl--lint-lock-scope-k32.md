# lint-lock-scope-k32

## Goal

Decide, and land, what lock `leaf-insert`'s cross-reference lint should hold and
for how long. Today it holds the store's **exclusive** guard across a whole-tree
content read *and* across every write to a caller-supplied sink.

## Context

`grove_loop::verbs::surface_cross_refs` takes an exclusive `WriteGuard`, walks
every leaf and brief in the tree, reads each one's bytes, and `writeln!`s each
hit into `out` — which is `std::io::stderr()` for `grove-llm leaf-insert`
(`crates/grove-llm/src/cli.rs`, `report_insert`).

**The exclusivity is a recorded decision, not an oversight**, and this leaf is
not free to just relax it. `crates/grove-loop/src/task_grow/tests.rs`'s
`leaf_insert_lints_cross_references_under_an_exclusive_lock_of_its_own` asserts
that the output is written *while the tree is held*, on the argument that a hit
printed after the lock went would name a path anything else could already have
renamed. What that argument establishes is that the lint must hold **a** lock —
it does not establish which.

Two things it leaves unexamined, and both were raised by a fresh-context
adversarial read at `loop-crate-verbs-k21`:

- **Shared would satisfy the argument.** The lint only reads. A shared guard
  keeps the tree quiescent against writers — which is the whole of what "nothing
  renames underneath a hit" needs — while letting `pick`, `kind` and
  `brief-chain` run. As it stands, every reader on the worktree is blocked for
  the duration of a whole-tree content scan.
- **A blocking write is inside the critical section.** `writeln!` to a pipe
  whose reader has stopped draining blocks, with the exclusive lock held, and
  every grove process on that worktree wedges behind it. A harness that captures
  a session's stderr and stops reading is not a hypothetical reader.

**`surface_cross_refs` swallows its write errors** (`writeln!(…).ok()`), and that
is deliberate and probably right — the insert has already landed, so a lint that
cannot print must not turn a reported mutation into a failure. Whatever this leaf
decides about the lock, it should say whether that stays true; the two questions
meet at *what happens when the sink is bad*.

## Done when

- The lint's lock is **shared, exclusive, or none**, decided on the argument
  above rather than inherited, and the existing test's claim is restated to match
  what was decided (it currently asserts *exclusive*, by probing with
  `flock(LOCK_SH | LOCK_NB)` and requiring failure).
- The blocking-sink hazard is either **closed** — collect the hits under the
  lock, release it, then write — or recorded as accepted with the reason.
  Collecting first is the obvious route and costs one `Vec<String>`; what it
  gives up is that a hit is now printed from a snapshot rather than from a tree
  still held, which is a *weaker* claim than the one the test makes today and has
  to be argued rather than assumed.
- If the answer changes what the verb promises, `crates/grove-loop/src/verbs.rs`'s
  `# Errors` section on `surface_cross_refs` says so.
- `cargo test --locked --workspace` and `cargo clippy --workspace --all-targets`
  clean; `CHANGELOG.md` updated if anything observable moved.

## Notes

**Small, and none of it is new code.** The whole subject is one function, one
test and one call site. It is cut as its own leaf because it is a **decision**
about a recorded trade-off rather than work `loop-crate-verbs-k21` could absorb:
that leaf moved the function between crates without touching what it does, and
re-deciding its locking inline would have been the failure the brief's
externalisation rule exists to prevent.

**Placed before `spec-to-current-state-k23`** so the spec's rewrite to current
state describes the lock this leaf settles on, rather than describing one and
then being corrected.

**No cutover.** Nothing here is tree-visible: no file in `.grove/` moves, no
filename grammar changes, and the verb set is untouched. Re-derive it anyway —
the root brief requires the matrix rather than the label — but expect it clean.
