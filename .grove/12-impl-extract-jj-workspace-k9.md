# extract-jj-workspace-k9

## Goal

Extract the VCS seam as `crates/jj-workspace`, a **fully domain-free** crate
whose whole surface is *resolve a jj workspace, refuse a tree that is not one,
take a path-scoped commit*.

## Context

`docs/specs/module-decomposition.md`, decision 8, states the interface verbatim —
`Workspace::{resolve, root, main_repo, control_dir, is_tracked, commit}`,
`Commit { change_id }`, `Refusal`. Implement against it; it is not to be
redesigned.

`decomposition-k2`'s running log, **"Workspace resolution belongs to the VCS
seam, not the loop"**, corrects `minimalism-k1`'s earlier split: the seam's own
guarantee is now *a non-jj working tree is refused before any mutation, with the
command that fixes it*, and that refusal **is** `vcs_of`. Splitting them would
leave the loop shelling out to `jj` and the seam unable to state its own
precondition.

## Done when

- `crates/jj-workspace` exists as a workspace member with `[lints] workspace =
  true`, and the root crate depends on it.
- Its public surface is exactly decision 8's. `Refusal` is opaque,
  `Error + Display`, and every variant names what is wrong, where, and what fixes
  it — jj's remedy (`jj git init --colocate`), not grove's.
- **`control_dir(namespace)` takes the namespace.** This is finding 6 of the
  design review and is the crate's domain-freedom test at the method rather than
  in the sentence claiming it: the implementation being moved reaches its answer
  by hard-coding a grove-named directory inside jj's administrative one, and
  *where a lease file may live* cannot be stated without naming the consumer.
  Returning the administrative directory raw is **rejected** — it puts the
  consumer's generic filenames into a namespace the version control system owns
  and may extend.
- The crate has tests exercising its public interface **without grove** — test
  seam 1. That is the compiler-enforced form of the brief's done-when.
- `docs/adr/one-live-driver-per-working-tree.md` is **reworked**: the lease
  survives; independent provisioning, the Git lane, the Git-or-jj
  control-directory derivation, the same-device gate and the Git-or-jj
  lost-result path do not. The control directory becomes the namespace the seam
  hands back.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green.** Expand → migrate → contract inside one leaf, which is
affordable here because the blast radius is one module: `repo.rs`'s external
surface was twelve functions and `drop-git-lane-k7` plus
`delete-finish-transaction-k8` have already removed the commit-boundary half and
the lane branching. What is left to move is workspace resolution.

**The lease itself does not move here.** It stays in grove until
`loop-crate-driver-k22`; what changes for it now is that it asks the seam for its
control directory by namespace instead of deriving one.

**Depends on `drop-git-lane-k7`.** The crate cannot claim *fully domain-free* —
or state its own precondition as a refusal — while a git lane exists behind it.
