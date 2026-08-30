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

## Decisions (running log)

- **The lint's lock is shared, not exclusive.** The lint only reads. What the
  recorded argument establishes — *nothing renames underneath a hit* — needs
  writers excluded, which `LOCK_SH` does; excluding *readers* buys nothing and
  blocks `pick`, `kind` and `brief-chain` for the length of a whole-tree content
  scan. "None" is not on the table: the store's own reader takes `LOCK_SH` to
  take a snapshot at all, so the scan would have to bypass the reader to hold
  nothing, and a torn walk is the only thing that buys.
- **The blocking-sink hazard is closed structurally, not by comment.** The scan
  collects its hits, the guard is consumed by the scan and dropped with it, and
  the verb returns a `Vec<String>`. The verb crate no longer writes to a caller's
  sink at all, so *no lock is held while printing* is a property of the types
  rather than of the order of two statements someone could later swap.
- **The claim the test makes is weakened deliberately, and this is the argument.**
  It asserted the hits were *printed* while the tree was held. That property was
  never worth what it cost, because it does not survive the function's own
  return: the operator reads stderr long after the guard is gone, and any grove
  process may rename in that window. What is actually load-bearing is that the
  hits come from **one consistent snapshot** — no half-walked tree, no hit read
  out of a file another writer was renaming — and a shared guard over the scan
  gives exactly that. The test now asserts the snapshot property, plus the new
  one: that the tree is unlocked at the moment a hit is printed.
- **The write errors stay swallowed, and move to where the stream semantics
  live.** `report_insert` prints the hits with `writeln!(stderr).ok()` rather
  than `eprintln!`, keeping today's behaviour — the insert has landed, so a lint
  that cannot print must not turn a reported mutation into a failure — at the one
  site whose doc comment already states that rule.

- **A self-deadlock the old shape did not have, found and closed while
  implementing.** The exclusive version reached the tree through
  `TreeWrite::guard()`, which *consumes* the guard the value was opened with —
  so it could never contend with it. Opening a shared lock instead is a **second
  file description**, and two of those on one directory do not share an `flock`:
  a caller holding an unspent write guard would have blocked the lint against
  its own process, forever. `TreeWrite::relinquish()` gives the guard up before
  the read is taken, which makes the hazard unexpressible rather than a rule in
  a header, and `the_cross_reference_lint_answers_through_an_unspent_write_guard`
  pins it — bounded by a channel timeout, because the regression is a hang and
  an unbounded test would report nothing about why.

## Why this leaf does not install anything

Re-derived rather than inherited, on the test the root brief names: **is there a
cell where the installed build meets the tree this leaf leaves and fails?**

The tree this leaf leaves is the tree it found. Nothing under `.grove/` is
added, removed or renamed by the change; the filename grammar is untouched; the
verb set is the same twelve; and `grove-llm leaf-insert`'s three output streams
are byte-identical in shape — the same path on stdout, the same renumber
summary, the same `path:line: <old-name> (context)` lint lines on stderr, in the
same path order. What moved is a library-internal signature (`Vec<String>`
returned instead of an `impl Write` taken) and which `flock` mode one opening
asks for, neither of which is visible to a reader of the tree or of the command.

| | old tree | new tree |
|---|---|---|
| **installed build (19.6.0)** | drives (today) | *same tree* — drives |
| **new build** | *same tree* — drives | drives |

Two of the four cells are the same observation because the two trees are the
same bytes. No cell fails, so there is no cutover, no release and no step-5 exit:
this session signals `complete` like any other.
