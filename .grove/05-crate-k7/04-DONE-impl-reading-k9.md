# reading-k9

## Goal

The first leaf that touches a filesystem: the advisory lock, the snapshot that
turns a directory tree into an in-memory tree of names, and the five reading
operations over it. After this leaf the crate can be pointed at a real directory
and asked what is in it.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — *The parse trichotomy*, *What is not in the trait, and
  why* (locking), *Operations → Reading*, and the invariants *No recognised name
  is silently skipped* and *Species agreement*.
- The root brief's `Pointers`, which carry three facts about the lock that came
  from reading the working implementation and must survive the extraction:
  the lock is `flock` on the **parent** of the tree root, not the root; paths
  are deliberately **never canonicalised**, because on macOS `/var` and
  `/private/var` name the same inode and canonicalising would make the mere
  presence of a lock rewrite every path a read verb returns; and locking follows
  inode identity through the descriptor while output preserves the caller's
  spelling.
- `src/repo.rs` and `src/tree_access.rs` for prior art on the guards. Prior art
  only.
- `operations.qnt` — the `unparseable` instance, and the invariant that a
  `Malformed` or `Reserved` name anywhere a walk reaches halts every operation.

## Done when

- Shared and exclusive lock guards exist, taken on the directory containing the
  tree root, with the no-canonicalisation rule preserved and its reason recorded
  where the next reader will meet it.
- A snapshot reads a whole tree, classifies every name through the consumer's
  `parse` with what the listing found — unfollowed — and halts on `Malformed` or
  `Reserved` carrying the consumer's own error.
- `walk`, `find`, `by_key`, `ancestors` and `distinguished_chain` behave as the
  *Reading* table states, including `ancestors` ending at the root, which is a
  node and not an entry, so its element type is not the entry type.
- A test shows a foreign **directory** is skipped whole, and a malformed one
  halts rather than vanishing. That pair is the failure this design exists to
  prevent and it should be visible in the test names.
- Each test names the model claim it discharges, or says it has none.
- An entry in `docs/formalism-findings.md`.

## Notes

**Walk order is unmodelled, and this is the leaf that owns it.** `operations.qnt`
models reachability and not the depth-first, distinguished-child-first ordering,
so `by_key`'s documented tie-break on a duplicate-key tree — *the first in walk
order* — rests on prose alone. Either implement the order the document states and
test it against hand-built trees, or make the case unreachable. Do not implement
it and describe it as checked: entry 003 already warns that a model can satisfy
a property by construction and look exactly like one that verified it, and this
is the same confusion arriving from the other side.

**Occupancy is decided without following links.** A symbolic link carrying an
entry's name is `Malformed`, not occupying, because `parse` sees what the
listing found and halts at the snapshot before any destination is computed. That
is a snapshot-layer property, so it is this leaf's to get right even though it
only pays off in the mutation leaves.

**Snapshot scope is whole-tree, and that is a decision, not an accident.** It is
why one unparseable name anywhere freezes the whole tree. Narrowing it later is
an invisible refinement; widening the blast radius is not. Do not narrow it here.

## Decisions (running log)

**Both model suites re-run before anything was written, and both are green** —
Alloy 20/20, Quint every claim across all eight instances, witnesses reached in
non-zero traces. That last part is the control entry 003 bought: a suite that
did not run reports *no counterexamples* with the same bytes as one that did,
and only the must-be-reached claims tell the two apart.

**The filesystem lives in `src/fs/`, and the crate root is the one file that may
name it.** `tests/algebra_has_no_filesystem.rs` scans every source outside
`src/fs/` for the whole word `fs`, so `mod fs;` in `lib.rs` is a violation of the
rule the guard holds — the promised path could not be declared at all. The
exemption added is the *declaration shape* and nothing else
(`[pub[(scope)]] mod fs;`, matched whole-line), with both controls. It is
narrower than it looks and it closes a hole rather than opening one: a
re-export — `pub use fs::Tree;` in `lib.rs` — stays a violation, so an algebra
module cannot reach the filesystem through a crate-root alias the scan cannot
see. That path is named in the guard's own header as a known limit; it is now
unreachable.

**One dependency, `libc`, and the "no dependencies" note in `Cargo.toml` is
rewritten rather than quietly broken.** `flock` is not in `std`. The
alternative was a hand-written `extern "C"` declaration plus hard-coded
`LOCK_SH`/`LOCK_EX` constants, which builds on every platform and is silently
wrong on any whose values differ — a lock taken in the wrong mode reports
success. `libc` is already in this workspace's lock file, so grove's graph does
not change; locking is invisible in the interface, so no consumer signature
mentions it.

**A filename that is not UTF-8 halts, and neither model can see the case.**
`parse` takes `&str`, so the library cannot ask the domain about such a name at
all. Skipping it is the failure this whole design exists to prevent — a
hand-edit that mangles one byte of a real name is *almost one of yours*, and a
skipped directory takes its subtree with it — so it is `Error::NonUtf8Name`,
carrying the library's own recovery advice because there is no domain error
value to carry. Both models hold no strings by design, so a name that is not a
string is outside what either can state. The cost is a genuinely foreign file
with a non-UTF-8 name freezing the tree; that is the blast radius the document
already accepts for `Malformed`, arriving by one more road.

**Walk order is implemented as the document states it, tested against
hand-built trees, and is not checked by any model.** Within a level: the
distinguished child first, then children by ordinal, ties broken by key and then
by the rendered name. The tie-break exists because `read_dir` order is
arbitrary — without a total order over one level, `by_key`'s documented *first
in walk order* would differ between two filesystems holding identical trees.
`operations.qnt` models reachability only and picks the least id, so this rests
on prose and on the tests named for it, exactly as the leaf brief requires it be
said.

**The listing is sorted before it is classified**, so which of two broken names
halts the tree is deterministic. Otherwise the recovery advice a consumer sees
depends on `read_dir` order.

**The lock's containing directory is computed lexically, and that is correct
precisely because nothing is canonicalised.** `root.parent()`, with an empty
result meaning `.`; the kernel then resolves `..` and symlinks when the
directory is opened, so `/x/y` and `/x/y/../y` take a lock on one inode while
every path the read verbs return keeps the caller's own spelling. A root with no
containing directory — `/` — is refused rather than locked elsewhere.
