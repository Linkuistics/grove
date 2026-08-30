# loop-crate-verbs-k21

## Goal

Create `crates/grove-loop` and move the tree layer and the twelve verbs into it,
with the signatures decision 9 fixes. `grove-llm` becomes a thin binary crate
over it.

## Context

`docs/specs/module-decomposition.md`, decision 9 — `read` / `write` /
`Reading` / `Writing`, `Reference`, `Selection`, `Error`, and the whole of
`verbs::`, stated verbatim and not to be redesigned.

**Why the verbs live here rather than with the store.** Ten of the twelve touch
the tree and every one is stated in **grove's** vocabulary — brief chains, kinds,
outcomes, handles, finishing — none of which the store has a word for.
Co-locating them gives the handle grammar one owner and puts the driver and the
verbs on one definition of a kind. The two that reach outward reach the runner
(`complete`) and the VCS seam (`finish-commit`).

**Three shapes recur across the surface and are deliberate**:

- A verb that **reads** takes a `Tree` and one that **writes** takes a
  `TreeWrite`, so the lock a verb needs is visible in its signature rather than
  acquired inside it.
- A search that matched nothing answers **`Sought`** — the store's word, from
  `sought-k24` — rather than an option each verb re-interprets. A loop that
  reintroduced `Option` here would have moved the problem rather than solved it.
- **Every verb returns the paths it wrote**, because its caller is a session that
  has to name them in a commit message it writes by hand.

`grove-loop::read` / `write` **mirror the store's, one level up, and for the same
reason**: a caller cannot scaffold over a live grove or read one that is not
there, because the types do not offer it. That is what lets `root_init` take the
`Vacancy` and be unable to run over a live grove.

## Done when

- `crates/grove-loop` exists as a workspace member and carries `src/task_tree.rs`,
  `src/task_grow*`, `src/task_name.rs`, `src/leaf.rs`, whatever survives of
  `src/tree_lifecycle.rs` and `src/tree_format.rs`, and the twelve verbs.
- `src/bin/grove-llm.rs` becomes its own crate under `crates/`, thin over
  `grove-loop`. It is a **separate crate, not a `[[bin]]` target**, for the same
  reason a module is a crate: a binary target can reach its own library's private
  items, so *the binary is thin* stops being compiler-enforced the moment it is a
  target.
- `Reference::parse` covers `.` for the root, a key, a handle, and a path.
  `resolve` answers `Sought<Resolution>` where **ambiguity is an answer, not an
  error** — the caller is a session that can re-ask with a narrower reference.
- `finish_commit` reaches the VCS seam and `complete` reaches the runner's
  channel; nothing else in the verbs crosses a module boundary.
- `Error` is **one opaque type for the whole crate**, `Error + Display`, under
  the same obligation as the runner's: every one names what is wrong and what
  fixes it.
- The crate's tests exercise the verbs through the public interface — test seam 1
  — and `tests/` at the root shrinks accordingly.
- `docs/adr/bulk-marks-are-not-atomic.md` **re-checked, expected unchanged**: a
  subtree prune is still *N* rewrites under *N* guards. Its implementation
  pointer moves into the loop crate. Re-check it; do not assume it.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green.** This is a wide but mechanical move: in Rust a module relocation
is a path rewrite, and the suite stays green because it all happens in one
commit. It is split from `loop-crate-driver-k22` by **layer**, not by call site —
the tree and its verbs here, the driver and its lease there — because that is the
seam where the two halves stop referring to each other.

**Depends on `open-kind-k20`**, so the verb signatures move once, already
carrying `&[Kind]` and a required kind, rather than being written twice.

**Reinstall in this session.** The verb surface is the same, but the binaries are
rebuilt from a different crate layout and the installed pair must match what the
tree and the corpus expect. Follow `grammar-separator-k15`'s install sequence.

**`CONTEXT-MAP.md` is the discipline to follow, not a document to update
afterwards.** It records the vocabulary-boundary work the first extraction did
and is the model for this one; its collision table is what keeps *session*,
*key*, *entry* and *leaf* from meaning two things across four crates.

## Decisions (running log)

**`grove-loop`'s `Error` wraps `anyhow` and stops there.** The modules moving
across carry stacked `with_context` prose, which is what makes them satisfy
principle 2 at all; re-expressing nineteen error sites as an enum would have
been a rewrite, not a move. `Error` is an opaque newtype whose `source()`
delegates, so a consumer walking the chain gets what it always got and takes on
no error library of ours — the rule `jj-workspace` and `keyed-launch` already
state.

**`TreeWrite` is grove-loop's own wrapper, not the store's guard.** Decision 9
writes every mutating verb as taking `&TreeWrite`, and a store mutation
*consumes* its guard — the two cannot both be true of one type. The wrapper
holds *the right to be the writer*: it hands out the guard it opened with and
reopens for the next verb. That is not a way around the store's rule, it is the
rule made explicit, and the gap between one guard and the next is exactly the
one `bulk-marks-are-not-atomic` already records.

**`Located` keeps an `outcome` field the spec's listing does not have.**
`resolve` must not let a retired or abandoned dead end look live, and the note
that says so is the caller's to print. The alternative was for the caller to
read the outcome off the filename, which is the one thing principle 3 forbids
anything but `TaskName` to do.

**`Renumber` is `from`/`to` paths plus the two positions.** Decision 9 writes
`{ from, to }`; the caller's line is `NN -> NN (name)`, and deriving the
positions from a path pair would have it re-derive both halves of a rename the
verb already knows. The two names come off the paths through accessors.

**`finish_commit` takes a parsed `Handle`, so the "the live finish leaf is X"
hint on a malformed handle is gone.** Decision 9 fixes the signature and the
parse therefore happens at the CLI. `HandleError`'s own message names what is
wrong and what a handle looks like, so principle 2 still holds; what is lost is
a convenience, and it is recorded here rather than argued away.

**The teardown deletes through `WriteGuard::delete` rather than
`remove_dir_all`.** `root-lifecycle-belongs-to-the-store` moved both halves of a
root's lifetime across, and this was the last caller that had kept its own
spelling. The guard is consumed by the deletion, so the lock is held right up to
the unlink. The commit message is unchanged: nothing downstream has anywhere to
put the removed-path list, and widening the return past decision 9's `Commit`
would have been a redesign.

**Three unit-test claims changed subject rather than being deleted**, each
because the thing they guarded is now a fact about the types: *a bad slug leaves
no tree behind* (the verbs take `Slug`), *a missing root errors in the verb* (the
caller opens the tree, and a missing root yields a `Vacancy`, which offers only
`root_init`), and *finish is refused before the tree is read* (there is no
reachable call with an unopened tree). Each rewritten test says in its header
what moved.

**`grove-llm` is a crate with a lib target as well as its bin.** The bin is three
lines over the lib. The lib exists so the CLI surface can be inspected by its own
tests — the clap-completeness and verb-set assertions that used to reach
`grove::llm_cli::Cli` — and thinness is still compiler-enforced, because the
logic it must not reimplement is in a *different* crate.

## Why this leaf does not install anything

**The `## Notes` above say *Reinstall in this session*. That label is not
inheritable**, and the root brief says so: *"each remaining leaf must re-derive
whether it is a cutover leaf rather than inherit the label, and the test is the
matrix k6 ran: is there a cell where the *installed* build meets the tree this
leaf leaves and fails?"* This leaf ran it, and there is no such cell.

The note's premise was that *the binaries are rebuilt from a different crate
layout and the installed pair must match what the tree and the corpus expect*. A
crate layout is not something a tree expects: what a tree can expect is a
**grammar**, a set of entries, and a set of verbs. This leaf moves none of them.
It adds no file to `.grove/`, removes none, renames none, and changes no filename
grammar — the whole change is which package a module compiles in.

Measured against both builds, from this worktree, on the tree this leaf leaves:

| verb | installed 19.6.0 | this build |
|---|---|---|
| `pick` | `…/25-impl--loop-crate-verbs-k21.md` | same |
| `kind` | `impl` | same |
| `brief-chain` | the root brief | same |
| `resolve loop-crate-verbs-k21` | the leaf's path | same |
| `resolve .` | `resolve: no entry matches reference "."`, exit 0 | the grove root |

Four rows identical; the fifth is a **new answer**, not a broken cell — the old
build's is a diagnostic and a zero exit, which is what `resolve` does with a
reference it cannot match. There is nothing here the installed build meets and
fails, so the four-cell matrix comes back clean and the install is not run.

**And the harm the install would do is the same harm k6 measured.** A release
meets every live grove on this machine, and this leaf has no tree-visible change
to justify the risk — the root brief's *if the tree-visible change can simply be
deferred, defer it and do not deploy* has nothing to defer here, because there is
no tree-visible change at all. `loop-crate-driver-k22` inherits the same question
and owes its own matrix; this leaf's finding is not transferable, for the reason
k18's was not.

Consequently this session ends **with** `grove-llm complete`, like an ordinary
leaf. The non-signalling exit belongs to a cutover, and this is not one.

## The doubt pass, and what it found

This leaf spent its **one** in-session reviewer on the single claim the compiler
could not establish: *the `TreeWrite` wrapper does not weaken the locking
contract relative to the previous design, in which each verb opened its own guard
internally from a `&Path`*. A fresh context got the wrapper, the store's locking
rules and the call sites, with the conclusion stripped, and was asked to break
it.

**The claim was false as stated, and four of its findings were acted on.**

1. **`TreeWrite` holds no lock after the first verb, and two doc comments said
   otherwise.** The type's header called itself *the task tree under the store's
   exclusive lock*, and the crate header said a `TreeWrite` *cannot run over a
   tree that is not there* — true of the first verb and of nothing after it. Both
   are corrected in place, and the type now states the three consequences: a
   second verb can find the tree gone, it waits on a new lock, and holding one
   across a call that opens the tree deadlocks the process against itself.
2. **A second verb waited silently.** `guard()`'s reopen went through
   `reopen_write`, which skips the contention diagnostic on the argument that the
   wait was already announced. That argument holds for one verb's later guards
   and not across two verbs, because the first lock has been *released* — so a
   contender arriving in the gap blocked the caller with nothing said. It now
   reopens through `task_tree::write`, which announces.
3. **A prune that stopped partway named nothing it had marked.**
   `bulk-marks-are-not-atomic` accepts *N* guards on the argument that re-running
   converges, and that argument is only available to an operator who can see the
   residue. `apply_prune` now carries the marked list into the refusal and says
   to rerun. Covered by `a_prune_that_stops_partway_names_what_it_already_marked`,
   which induces the failure with a read-only directory at the second position.
4. **Two latent traps, both cheap.** `cmd_root_init`'s `let … else` held the
   unmatched `TreeWrite` — and its lock — across the `else` block, so a later
   improvement to that message that read the tree would have deadlocked; it is a
   `match` now. And `cmd_leaf_decompose`'s read-then-open ordering is
   load-bearing for the same reason and now says so.

**Two findings were classified as a contract I had stated unclearly**, and the
prose was fixed rather than the code: `surface_cross_refs`'s `# Errors` claimed
it returns a failed write to `out`, which it deliberately does not, and the
`RefCell` comment claimed a borrow ended where under edition 2021 it does not —
`guard()` now takes the value out in its own statement so the claim is true.

**One is a visible trade-off, stated where it lives**: `TreeWrite` is `!Sync`,
which blocks the harmless case (sharing one across threads) and not the fatal one
(two threads each opening their own). Nothing in this workspace shares one, and
making it `Sync` would let two threads race for the same guard.

**One was externalised as `lint-lock-scope-k32`**: the cross-reference lint holds
the *exclusive* guard across a whole-tree content read and across blocking writes
to a caller-supplied sink. That code is unchanged by this leaf — it moved between
crates and nothing else — and re-deciding its locking inline is the failure the
externalisation rule exists to prevent. It is inserted **before**
`spec-to-current-state-k23` so the spec's rewrite describes the lock that leaf
settles on.

**Two were pre-existing and not made worse here**: `announce_contention` reports
a nonexistent contender during a self-deadlock (a property of `flock` probing
one's own lock, true before the wrapper existed), and `root()` is a spelling
rather than an inode, so two guards of one `TreeWrite` can in principle lock two
directories if the worktree is replaced between them — which `apply_prune`'s own
reopen loop could already do. Both are named in the type's header rather than
argued away.

**No second reviewer was materialised.** Every fix above is a doc correction, a
one-line ordering change, or a behaviour fix covered by an executable test —
none is the substantive non-mechanical redesign that would make review
tree-sized.
