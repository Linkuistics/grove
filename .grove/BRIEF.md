# grove.refactor-for-modularity — brief

## Goal

Redesign Grove as a small application assembled from deep, concern-oriented Rust crates. The result must be easier for humans and LLMs to learn, reason about, change, and verify without weakening Grove's fail-closed task-tree or finish guarantees.

The work is deliberately evidence-led: build and compare executable Quint and Alloy 6 behavioural models first, make the user and maintainer documentation describe the chosen system second, and only then reshape the Rust implementation.

## Done when

- The formal phase has produced green Quint and Alloy 6 models for task-tree behaviour, the finish/recovery protocol, and the end-to-end lifecycle; their assumptions, claims, witnesses, bounds, and counterexamples are durable and reproducible.
- `docs/formalism-findings.md` records this bounded comparison with enough evidence to distinguish overlapping findings, unique findings, false confidence, modelling cost, runner/tooling cost, and implementation/test influence.
- Current-state documentation explains the system, the crate dependency direction, where each concern changes, the supported Git/jj layouts, the user workflow, and recovery from interrupted finish attempts.
- The Cargo workspace exposes deep semantic crates for methodology, task-tree semantics, workspace/repository identity, and finish/recovery. The root `grove` package is the application/runtime and the two shipped binaries, not a second copy of those concerns.
- Generic ordered-tree algebra and filesystem mechanics live in `ordinal-fs-tree` by default. Any Grove-owned filesystem operation is a documented semantic exception established by the formal work.
- Legacy tree migration is gone. Fresh roots use the current format; an absent, legacy, or foreign format fails closed and is never silently rewritten.
- The preserved external contract, release constraints, model checks, workspace tests, and real Git/native-jj/colocated-jj finish scenarios pass.
- `TODO.finish_process.md` has been resolved into models, current-state documentation, tests, and code, then removed. **The file is already gone**: `docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md` carries the four questions, their dispositions — Q2 and Q3 `keep`, Q1 and Q4's three cleanup rows `defer` — the cost table and the four binding constraints. What remains of this item is the documentation and implementation half.

## Decomposition

The top-level order is a hard gate:

1. `formal-modeling-k1` defines and checks the design. No user documentation or product-code redesign begins before it is retired.
2. `documentation-k2` turns the checked design into current-state architecture, decisions, component guidance, and a user guide. No implementation begins before it is retired.
3. `implementation-k3` changes the product against those models and documents.

The approved target dependency direction is:

```text
ordinal-fs-tree       grove-methodology
        │                    │
        └──────> grove-task-tree

grove-workspace ─────> grove-finish <──── grove-task-tree
        │                    │
        └──────────> grove (application/runtime + binaries)
                              ▲
grove-methodology ────────────┘
```

`grove-workspace` supplies discovered repository identity and supported VCS operations; it must not become a generic filesystem toolbox. Models are owned beside the semantic component they constrain, with cross-crate lifecycle models in `models/system/`.

The tree is intentionally lazy. `formal-synthesis-k16` creates an ordinal root-lifecycle implementation leaf only if the experiment proves that abstraction deep and domain-independent. It likewise creates individual finish-simplification leaves only for transformations justified by both models. Review chains are created by the producing sessions when a decision warrants fresh-context challenge.

## Pointers

- `docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md` — the required finish/recovery concerns and the design pressure as it now stands, replacing the deleted `TODO.finish_process.md`.
- `docs/formalism-findings.md` — findings from the previous formal-method experiment; append this experiment rather than overwriting it.
- `docs/ARCHITECTURE.md`, `docs/USAGE.md`, and `docs/CONFIGURATION.md` — current-state documentation to reconcile in place.
- `docs/adr/task-tree-transactions-fail-closed.md` and the rest of `docs/adr/` — current decision set to edit, merge, split, or delete in place.
- `docs/ordinal-fs-tree/ARCHITECTURE.md` — existing component boundary to reconcile with crate-local ownership.
- GitHub issue #13 — source context for the finish-process work; the repository files and models remain authoritative.

## Notes

### Preservation ledger

Preserve unless the checked design explicitly records an approved exception:

- CLI verb names, arguments, help shape, structured/human output fields, and exit-status meanings.
- Configuration keys, environment overrides, defaults, and the current `session-kinds-v1` `.grove` format.
- Abstract outcomes across Git, native jj, and colocated jj workspaces.
- Methodology embedding/provisioning, package and binary names, release/install behaviour, MSRV 1.85, and the Linux glibc 2.17 compatibility target.
- Fail-closed ownership: Grove never resets, merges, deletes, or rewrites work it cannot prove belongs to the current finish attempt.

Approved breaking change: remove every legacy migration command, compatibility path, and automatic format rewrite. Documentation and diagnostics must say how to start a current-format root instead.

Finish is a conservative recovery protocol, not generic cleanup. It must distinguish an incomplete Grove-owned attempt (`RecoveryPending`) from unrelated or ambiguous state (`OwnershipConflict`), persist a correlation artifact outside disposable state, evacuate durable evidence before deleting `.grove`, and make both success exits explicit: preserve the branch/bookmark, or merge and then remove only the proved-owned branch/worktree.

`.grove/` is process state, not the permanent home of design conclusions. Every durable finding must be promoted into models, tests, documentation, or decision records before its task retires.

Known launch precondition: the Grove CLI reported methodology hash `10db…` while the installed driving skill reported `8501…` when this tree was created. Re-provision or otherwise reconcile that skew before driving the first leaf, and record the exact resolution in `experiment-baseline-k4`.
