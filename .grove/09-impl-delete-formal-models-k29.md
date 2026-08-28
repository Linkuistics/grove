# delete-formal-models-k29

## Goal

Delete the formal-methods experiment's **apparatus** and the semantic contract it
checked. The campaign's value is the lessons it produced and the `linkuistics`
skills those lessons became; the runnable machinery has no consumer left and the
contract specifies a system this grove is actively dismantling.

## Context

This leaf was cut mid-`delete-migration-k6`, when rebasing that leaf onto `main`
surfaced ~38,000 lines of formal-modelling material that had landed there while
the refactor branch was off to the side (`finish-k93`, `model-led-development-k94`
through `-k98`). The human's decision, recorded at the moment it was taken: *"those
formal models were an experiment that can in fact be deleted — all that matters is
the lessons that were learnt whilst building them, and the skills generated
therefore."*

The boundary was settled the same way, against a stated alternative that took the
campaign's records too. It stands where it does because **the skills cite the
records for their authority**: `plugins/linkuistics/PROVENANCE.md` and four files
under `model-led-development/` and `doubt-driven-development/` bind their rules to
`docs/formalism-findings.md`, `docs/candidate-lessons.md` and
`docs/review-yield.md` — one through a public GitHub URL to `main`. A rule whose
evidence has been deleted is a rule nothing can falsify, which is the failure the
`doubt-driven-development` skill exists to prevent. So the evidence stays and the
machinery goes.

## Done when

- **Deleted**, with nothing left importing, running or citing them:
  - `models/` — `system/lifecycle.qnt`, `system/lifecycle.als`,
    `system/lifecycle-controls.qnt`, `system/README.md`, `run.sh`,
    `run-controls.sh`, `README.md` (~8,600 lines).
  - `crates/grove-task-tree/models/` — `task-tree.qnt`, `task-tree.als`,
    `task-tree-controls.qnt`, `README.md` (~7,600 lines).
  - `scripts/loop-record.py` and `scripts/review-yield.py` (~1,500 lines).
  - `docs/specs/semantic-contract.md` (~2,900 lines).
- **Kept, and their citations still resolve:** `docs/formalism-findings.md`,
  `docs/candidate-lessons.md`, `docs/review-yield.md`, `docs/loop-record.md`,
  `docs/preservation-baseline.md`, `docs/driving-a-checkable-loop.md`,
  `docs/results-of-formal-methods-trial.md`.
- Every reference into the deleted set is reconciled — `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table, `CONTEXT.md`, `CONTEXT-MAP.md` (which lists
  `semantic-contract` among the contexts' specs), the two `docs/` records that
  cite model files, and the skills. `tests/reference_navigation.rs` is the check
  that this is complete; run it rather than trusting a grep.
- `docs/results-of-formal-methods-trial.md` still reads truthfully once its
  subject is gone — it is the trial's *conclusion*, and a conclusion that points
  at deleted models needs its pointers turned into description.
- `cargo test` and `cargo clippy --all-targets` are clean.
- `CHANGELOG.md`'s `## Unreleased` records the removal, naming what was kept and
  why.

## Notes

**This is not a cutover leaf.** It changes no tree shape, no filename grammar and
no verb surface, so it neither rebuilds nor reinstalls, and it signals completion
normally. The root brief names the five leaves that may reinstall and this is not
one of them.

**Ordering: it runs after `delete-migration-k6` and before everything else.**
After k6 because k6 is already written and blocked only on a human-run install;
before the rest because the semantic contract describes the witness lifecycle,
the git lane, the finish transaction and the module boundaries that k7, k8, k9
and the store node all change — every one of those leaves would otherwise owe a
reconciliation to a document scheduled for deletion.

**The one file that needs judgement rather than deletion.**
`docs/specs/semantic-contract.md` is not apparatus: it is a specification of
grove's semantics, written to be model-checked. It goes because its checker goes
and because k6 already contradicts it in roughly 285 places, all of them the
format witness. Do not attempt to salvage it into a witness-free contract — that
is a design session's work, not this one's, and `spec-to-current-state-k23`
already owns what the surviving specs must say. If something in it turns out to
be load-bearing prose rather than a checkable claim, promote that paragraph into
`docs/ARCHITECTURE.md` where the current-state documentation lives, and say so.

**`crates/grove-task-tree/models/` is inside a live crate.** Deleting the
directory must not disturb the crate; check `Cargo.toml` and the crate's own
`README` for references before removing it.

**One ADR already survived this deletion and is the model for the rest.**
`docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md` was reworked
in place by `delete-migration-k6`: its *decision* outlived the instrument that
found it, and its `## What enforces it` section now names product tests and says
plainly that the model controls were retired with the campaign. That is the shape
to reach for wherever a record cites a model — the finding stands, the citation
moves to whatever checks it now.
