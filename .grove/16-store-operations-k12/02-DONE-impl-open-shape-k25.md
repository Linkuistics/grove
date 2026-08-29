# open-shape-k25

## Goal

Answer *is there a tree here* **as a shape rather than as a predicate**, and give
the resulting vacancy the ability to create the tree under the lock it already
holds.

## Context

`docs/specs/module-decomposition.md`, decision 2, stated verbatim:

```rust
pub fn read<N: EntryName>(root: &Path)  -> Result<Reading<N>, Error<N>>;
pub fn write<N: EntryName>(root: &Path) -> Result<Writing<N>, Error<N>>;
pub enum Reading<N> { Tree(ReadGuard<N>), Vacant }
pub enum Writing<N> { Tree(WriteGuard<N>), Vacancy(Vacancy<N>) }

impl<N: EntryName> Vacancy<N> {
    pub fn initialize(
        self,
        distinguished: Option<Vec<u8>>,
        entries: Vec<NewEntry<N::Parts>>,
    ) -> Result<Report<N>, Error<N>>;
}
```

Two warrants, both settled:

- **Why a shape and not `exists()`.** `decomposition-k2`'s running log: a
  separate predicate is a check-then-act split, and check-then-act over a tree
  under a lock is exactly the disease grove's two-phase
  `Classification`/`settle` dance exists to paper over. One lock acquisition, and
  the answer hands back the only operation valid for it — initializing over a
  live tree and deleting a vacancy are **not expressible**.
- **Why `initialize` takes bytes.** Design-review finding 2. `NewEntry` can
  create only a positioned entry, and the distinguished child is the one entry it
  cannot express: it carries no parts and its name is `N::distinguished()`. The
  library **already writes one this way** when a promotion moves a leaf's bytes
  into a new node, so `entry-name-is-the-only-seam` holds with **no new trait
  method**. Without it the consumer would write the charter itself, outside the
  lock and outside the store, and *the store is the only thing that touches the
  task tree* would fail at the first operation of every fresh tree.

## Done when

- `read` and `write` return the two enums; the guards keep their names and
  behaviour. `Vacancy` is a guard: it holds the exclusive lock, so there is **no
  window** between deciding a tree is absent and creating it.
- `initialize` creates the root, writes the distinguished child, and places the
  first entries, under the lock already held. `None` creates a root without a
  distinguished child; `Some` in a domain whose `distinguished()` is `None` is
  the **same refusal a promotion gives for the same reason** — reuse it rather
  than inventing a second.
- Something at the root that is neither a tree nor nothing — a regular file, a
  symlink — is an `Error` **carrying what was found**, not a third variant.
- **Every consumer that must still compile is migrated here, mechanically.**
  Outside the crate that is exactly three call sites, all inside one module:
  `src/task_tree.rs:77`, `:117` and `:144` (`Tree` and `TreeWrite` alias the two
  guards at `:60` and `:94`). Each learns to match the new enum and to raise
  Grove's existing *absent tree* diagnostic on the vacant arm — the same message
  Grove's own lock layer produces today, moved, not redesigned. Nothing else in
  Grove changes shape.
- The crate's tests cover: vacancy → initialize → read back; initialize refused
  over a live tree by construction (a compile-fail or doc test is the honest form
  of *not expressible*); the non-tree root error.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, `CONTEXT.md` and `CLI.md` reconciled;
  `models/operations.qnt` gains root creation as a transition and
  `models/structure.als` gains the vacancy/tree/neither trichotomy, or each
  states why it does not.
- `docs/adr/grove-does-not-stage-its-own-renames.md` and
  `docs/adr/bulk-marks-are-not-atomic.md` **re-checked** against a store that now
  owns root creation — re-checked, not assumed. Record the outcome either way.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green.** This is the widest of the three inside the crate — every
existing caller of `fs::read` / `fs::write` sees a new return shape — but the
crate is small and its consumers are in-tree, so expand → migrate → contract fits
one session.

**Grove is *adapted* here; it is not *migrated* here.** The distinction is the
whole of what keeps this leaf green, and `module-split-k4` collapsed the two.
Adapting means the three call sites above learn the new return shape so the
workspace compiles — the mechanical half, which cannot be deferred to a later
sibling without leaving `cargo test` red for the sessions in between. Migrating
means Grove's own `read`/`write` mirror of this shape (decision 9,
`loop-crate-verbs-k21`) and the deletion of Grove's second lock layer
(`collapse-tree-access-k13`): both stay outside this node, and both are cheaper
*because* the adaptation already happened.

**Lands green as a whole workspace, not as a crate.** `cargo test` at the
workspace root is the check, not `cargo test -p ordinal-fs-tree` — a leaf that
leaves the root package uncompilable has not landed, whatever the crate's own
suite says.

**The type is the guarantee.** If a reviewer can write a program that calls
`initialize` on a live tree, the leaf is not done — the point of the shape is
that the ill-formed call does not typecheck.

## Decisions (running log)

**The trichotomy's third answer is decided by following symbolic links, and its
"nothing there" answer is decided without.** Two questions of the filesystem
rather than one. `symlink_metadata` separates *nothing is here* from *something
is here*; `metadata` then classifies what is there, following links, because a
link naming a directory is an accepted spelling of a root and always has been
(`fs/read.rs`, *Paths come back the way they went in*). The case that forces the
pair is a **dangling** link: `metadata` alone calls it `NotFound`, which would
make it a vacancy and send `initialize` at a name that is plainly occupied. It is
`Error::RootIsNotATree { found: Found::Other }` instead.

**`Found` carries what was found, rather than a new type.** It is the crate's
existing word for *what a listing found under a name*, its `Display` already
reads as a sentence fragment (*a regular file*), and the message needs nothing
finer. A dangling link and a socket are both `Other`, which the message's advice
— move it aside — does not need to separate.

**`containing_directory` falls back to the lexical parent whenever `metadata`
does not report a directory at the root, not merely when the root is absent.**
The first draft keyed on absence, and a regular file at the root then made the
lock acquisition itself fail with `ENOTDIR` on `<root>/..` — before anything
could classify it — so the third answer was unreachable through `write`. The
test `a_root_that_is_a_regular_file_is_neither_a_tree_nor_a_vacancy` is what
caught it. The fallback is exact rather than approximate for a stated reason:
both spellings that made a lexical parent wrong (a final `..`, a followed final
symbolic link) require a **directory** at the root, so wherever the fallback
runs, the two routes cannot disagree.

**The refusal is `promote`'s, renamed rather than duplicated.**
`Refusal::PromoteNoDistinguished { key }` became
`Refusal::NoDistinguishedChild { promoting: Option<Key> }`. One condition — this
domain has no distinguished child and these bytes have nowhere to go — reached
from two operations, and `None` is the root initialization, which names no entry
because the root is not one. The alternative was a second variant with the same
message, which is `docs/formalism-findings.md` entry 017's failure by
construction.

**The root directory is created by the filesystem layer, not by an effect.** A
plan places names and the root has none, so no `Effect::Create` can make it and
no `Report` row can describe it. What that costs is one ordering decision: the
seventh obligation's pre-check moved out of `apply` into
`apply::names_are_one_component`, so `initialize` can run it *before* `create_dir`
— otherwise a domain that renders a name badly would leave an empty root behind
while returning an error whose whole promise is that nothing changed.

**`operations.qnt` gains `Initialize`; `structure.als` gains the trichotomy; and
the Quint action is *disabled* rather than refused.** The behavioural model's
initial state is already an empty tree, so an `Initialize` from it is exactly the
transition — and it is the only plan in the model that **creates** a
distinguished child rather than renaming an inode onto one, which is what makes
it worth having. It is guarded on `fs.objects` being empty rather than given a
`Refuse` branch, because a guard is the model's way of saying *not expressible*
where a refusal would say *expressible and answered*, and in the implementation
`initialize` on a live tree does not typecheck. The root's own existence stays
out of the behavioural model — it is a filesystem fact that model does not hold —
and the vacancy/tree/neither trichotomy went to `structure.als`, which holds
shape.

**Grove is adapted, not migrated.** The vacant arm of both call sites raises
`grove root not found`, the sentence Grove's own lock layer already produced for
the same condition — moved, not redesigned. Grove still creates its own tree root
outside the store (`tree_lifecycle::create_root_unlocked`: a `create_dir_all` and
a hand-written `BRIEF.md` under Grove's guard), and `Vacancy::initialize` is
deliberately left uncalled: closing that hole is `collapse-tree-access-k13`'s,
and a verb that created a tree on the way past would turn a mistyped root into a
second workstream.

**`bulk-marks-are-not-atomic` needed a correction; `grove-does-not-stage-its-own-renames`
did not.** Re-checked, not assumed. The first argued that a batched rewrite was
impossible partly because *the operation set was fixed by `library-k6` and closed
by `crate-k7`* — which this leaf falsified, so the record now rests on the
absence of a batched rewrite specifically rather than on a closed surface. The
second never turned on the effect being a rename: a `mkdir` is a working-copy
change Jujutsu snapshots exactly as it does a `rename(2)`, and a store that
creates a root without detecting a repository is one more operation with no
repository awareness to stage. Said so in place.

**The in-session reviewer found two real defects in the opening, and both were
the same mistake inverted.** The allowance was spent on one fresh context asked
to break the lock/vacancy claim. Classified:

*Valid and actionable, fixed.* **(a)** `containing_directory` chose the lexical
parent whenever the root did not *resolve* to a directory — which a **dangling
symbolic link** also satisfies, and a link's last component *is* followed. So a
caller through the link and a caller through the target path could take two
different locks over one tree the moment the target appeared: `reading-k19`'s
defect, re-entering through the door absence opened. The route now turns on
whether the kernel follows the last component, and a dangling link is refused
before any lock is taken. **(b)** `presence` inferred *dangling* from
`symlink_metadata` and `metadata` **disagreeing** — the identical pair an ordinary
directory removed between the two calls gives, so a deleted tree was reported as
a symbolic link occupying the root, with advice naming a file that does not
exist. Both now decide from the *first* observation's file type. **(c)** A root
below a regular file reached `File::open` on the lexical parent, taking the
advisory lock on a **file**; `ENOTDIR` is now the error it is. **(d)**
`remove_dir` answering `NotFound` in `initialize`'s unwind reported
`FailedPartiallyRolledBack` when the unwind's goal was achieved. Three tests
carry (a), (b) and (c).

*A contract stated unclearly, reworded.* `initialize`'s `Errors` doc said
`Failed` means "not even the root remains", which reads as a claim about the
root's state; it is a claim about **this call**, and a writer ignoring the
advisory lock can create the root under it. The distinction `claim_vacant`
already draws for every other operation, now drawn here.

*Visible trade-off, stated rather than fixed.* For a root spelled through a
symbolic link the lock is on the **target's** containing directory — which is
what makes every spelling converge, and equally why the lock does not cover the
link's own name. A hand re-pointing the link under a live guard is a writer
ignoring the advisory lock, which nothing path-based can defend against in a
module that deliberately never canonicalises. Written into
`containing_directory`.

*Noise for this leaf.* A latent staleness in `apply::entry_path` for a plan that
moves a node and then addresses a descendant of it — no operation emits such a
plan, and it predates this leaf. APFS firmlinks defeating any path-based lock
scheme: true, pre-existing, and not this leaf's to answer.
