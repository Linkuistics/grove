# delete-finish-models-k30

## Goal

Delete `crates/grove-finish/models/` — the last of the formal-methods campaign's
apparatus — and rework in place the nine decision records that cite it, so each
finding survives its instrument.

## Context

`delete-formal-models-k29` deleted the campaign's apparatus and the semantic
contract it checked. Its `## Done when` enumerated `models/`,
`crates/grove-task-tree/models/`, the two `scripts/` reporters and
`docs/specs/semantic-contract.md`, and omitted this directory.

k29 read the omission as an oversight rather than as the boundary, and its
`## Decisions (running log)` records the evidence: k29's `## Context` puts the
material the rebase surfaced at ~38,000 lines, the enumerated set measures
23,757, and these four files are 16,392 — the shortfall, almost exactly. The
same list's stated line counts run low against the files they name, and its one
note about `crates/` asserted a live crate that does not exist. Both are the
signature of a list written from the campaign's records rather than from a walk
of the tree.

So the boundary the human settled — *the apparatus goes, the records stay* —
puts this directory on the deleted side, and k29 externalised it rather than
absorbing it because the cost is not clerical. Nine records under `docs/adr/`
bind their arguments to these files as evidence, and each needs the in-place
rework k29's `## Notes` describes rather than a link repair.

**This leaf sits immediately before `delete-finish-transaction-k8`** because
that is the leaf which reworks the same records for the same subject. Running
them adjacently means the finish records are opened once, not twice; running
this one first means k8 never owes a reconciliation to models already scheduled
for deletion, which is the argument k29 itself was ordered on.

After k29 this directory is already stranded: `models/run.sh` was its only
runner (`run.sh:171` mapped scope `finish` here) and went with k29.

## Done when

- **Deleted:** `crates/grove-finish/models/` — `finish.qnt`, `finish.als`,
  `finish-controls.qnt`, `README.md` (16,392 lines). The directory holds nothing
  else, and `crates/grove-finish/` goes with it: it is not a crate, it is a
  model-only directory naming a context (`Cargo.toml`'s `members` lists only
  `crates/ordinal-fs-tree`).
- **Each citing record reworked in place, its finding kept.** The shape is
  `docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md`, reworked
  by `delete-migration-k6`: the decision outlives the instrument, and
  `## What enforces it` names whatever checks it now, saying plainly that the
  model controls were retired with the campaign. The nine:
  `finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`,
  `success-is-proved-by-the-ticket-not-the-tree.md`,
  `root-lifecycle-stays-with-its-receipt.md`,
  `a-shared-safety-claim-names-the-role-not-the-artifact.md`,
  `a-refusal-leaves-nothing-standing.md`,
  `a-closed-partition-is-over-outcomes-not-states.md`,
  `obligations-follow-context-not-artifact.md`, and the two others a sweep
  finds — enumerate rather than trust this list.
- **Kept, and their citations still resolve:** the campaign's records under
  `docs/` that k29 kept, in particular `docs/formalism-findings.md`,
  `docs/candidate-lessons.md` and `docs/driving-a-checkable-loop.md`, each of
  which links into these files.
- `CONTEXT.md`'s `crates/grove-finish/models/` reference reconciled.
- `cargo test` and `cargo clippy --all-targets` clean;
  `tests/reference_navigation.rs` is the completeness check, not a grep.
- `CHANGELOG.md`'s `## Unreleased` records the removal beside k29's.

## Notes

**Not a cutover leaf.** No tree shape, no filename grammar, no verb surface — it
neither rebuilds nor reinstalls, and it signals completion normally.

**`obligations-follow-context-not-artifact.md` is the one to read first.** Its
whole argument is about where an obligation family *lives*, stated over the
`FN-` / `TT-` / `SY-` split that this deletion completes the removal of. It may
turn out to be a record whose subject is wholly gone rather than one whose
citation moved; `ADR-FORMAT.md`'s rework guidance owns that call, and deleting a
record is a legitimate outcome there.

**Do not reopen the boundary.** Whether the campaign's records under `docs/`
survive was settled by the human, against a stated alternative that took them
too; k29's `## Context` carries the reasoning. This leaf moves apparatus only.
