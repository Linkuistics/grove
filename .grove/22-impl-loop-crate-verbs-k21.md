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
