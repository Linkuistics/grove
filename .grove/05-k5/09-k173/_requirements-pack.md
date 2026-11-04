# requirements-pack-k173 — brief

## Goal

Create `~/Development/AgenticOrchestration/RequirementsSkills` and move the
requirements chain into it: `grove-requirements` with its interview
procedure, `grove-review-requirements` and
`grove-integrate-review-requirements`. This is the first move of an external
member of the cross-pack review and integration families.

## Done when

- RequirementsSkills has the layout of ResearchSkills in the contract's
  "Proven facts". That layout is colocated Git and jj, Apache-2.0 and one
  source directory with the root `skills`. It also has a manifest with
  `dependencies: ["grove"]`, its own `conformance/rules.tsv` and the copied
  pack tasks. It has no `craft/` directory.
- The moved kinds open with the bare-name text of the contract. They load
  `grove`, and the review and integration kinds name the spine's
  `references/review.md` and `references/integrate-review.md` as external
  resources. Their other bytes are unchanged, or the record names each change.
- The pack's rows move from core's manifest by the contract's move order. The
  first pack commit records a Grove commit that still ships the moving files.
  Grove removes its copies and pins that commit. A later pack commit records
  the removal commit.
- The pack passes alone with the external resources listed. The aggregate
  passes in Grove's `task check:packs` and in the pack's own `task check`.
- The kind table of `TASK-FORMAT.md` names `RequirementsSkills` for the three
  kinds. Grove's label tests, installer, trial cases and documents agree.
- The spine's rule for the ADR and spec formats follows
  pack-membership-k171. Each reference of the moved text to those formats
  resolves on every route.
- Both pack commits are on the public remote, and the Grove line under the
  bookmark `gh-issue-25` holds each Grove commit that a pack commit records.
- The proofs that pack-membership-k171 assigns to this move pass.
- `docs/repository-moves-record.md` records the source root, the commits, the
  tree ID, the `resourceDigest` and the external resources.

## Decomposition

The first child plans this node from the membership decisions. Copy the
tooling of ResearchSkills. Do not repeat the research tooling leaves.

requirements-pack-plan-k174 cut four leaves in the contract's move order.
Each leaf leaves Grove's `bash scripts/check.sh` green.

1. `requirements-pack-repository-k191`: the expand step. It publishes the
   first pack commit. Grove still ships its copies, so no Grove consumer
   changes.
2. `requirements-removal-k192`: the contraction step. It removes Grove's
   copies with every consumer in one change. It also publishes the second
   pack commit.
3. `requirements-directory-trials-k193`: the boundary's directory trial in
   both harnesses.
4. `requirements-native-namespace-k194`: the boundary's native Claude Code
   trial.

The two pack commits are not separate increments. Between them the aggregate
fails on duplicates by design, so the second pack commit belongs to the
removal. The two trials are separate leaves because each measures one route
and needs its own setup. A failed trial on one route then does not block the
other. Neither trial changes the pack or Grove's skills.

No leaf repeats the delegate, working-copy, refusal or offline proofs of the
research move. The membership specification assigns this move only the
cross-pack family boundary.

## Pointers

- `docs/specs/external-pack-contract.md`, sections "Rule placement",
  "Conformance" and "Proven facts".
- `docs/adr/corpus-rules-have-one-owner.md`.

## Notes

`grove-requirements/grilling.md` is a resource of its own skill and moves
with it. The root brief names it "its interview procedure".

`grove-requirements` adapts and bundles MIT-licensed passages. At planning,
its licence pointer resolved only in the spine's `LICENSES/`. The pack
therefore adds that licence file inside the moved skill, as the contract's
"Provenance and licences" directs for an adapted passage.
