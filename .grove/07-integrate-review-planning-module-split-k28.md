# module-split-k28

## Goal

Repair the decomposition cut by `module-split-k4` so every mapped decision has
an implementable leaf, every dependency points backward, every leaf can land
green, and the meta-grove cutovers hand the live loop to the installed build
without corrupting the task tree or the plugin installation.

**Integrates:** module-split-k27

## Context

**Reviewed artifact:** `module-split-k4` at producer commit `939cd044`.

The review was inspection-only. It read the producer diff, the complete spec,
the design and prior review logs, the requirements measurements and deletion
list, every planned leaf, the current source at each disputed seam, and the
machine's resolved installation. It ran no test, build, lint or format command.

## Findings

### 1. `open-shape-k25` changes a public return type and explicitly leaves its consumer broken

The leaf requires `ordinal_fs_tree::fs::{read, write}` to return `Reading` and
`Writing` (`14-store-operations-k12/02-impl-open-shape-k25.md:45-49`), then says
Grove is not migrated there and defers that work to `collapse-tree-access-k13`
and `loop-crate-verbs-k21` (lines 71-78). Today the store returns the guards
directly (`crates/ordinal-fs-tree/src/fs/mod.rs:78-104`), and Grove consumes that
exact shape in `src/task_tree.rs:72-103`: the `Ok(tree)` value becomes `Tree`,
and `TreeWrite` aliases `WriteGuard<TaskName>`.

There is no compatibility surface between the changed interface and those
later leaves. The workspace therefore cannot compile after k25 as written. This
violates `module-split-k4`'s contract that each leaf is a vertical slice landing
green, and the review's **Green** and **Ordering** conditions. Redraw this as an
expand/migrate/contract sequence or migrate every compiling consumer in k25;
do not leave a return-type break waiting on a later sibling.

### 2. `prompt-names-the-kind-k18` depends on the later provisioning deletion

K18 requires `src/methodology.rs`, `--content-hash` and the build-pairing report
to be gone, and retires `one-build-owns-a-session` and
`skill-delivers-the-methodology` (`20-impl-prompt-names-the-kind-k18.md:39-43`).
But provisioning remains until k19, and its live per-iteration path calls
`methodology::identity()` (`src/provision.rs:53-77`); the driver invokes that
path before every transition (`src/loop_driver.rs:116-130`). Deleting the module
at k18 therefore leaves a source dependency that only the later k19 removes.

The spec also assigns both retirements to *the leaf that deletes provisioning*
(`docs/specs/module-decomposition.md:776-777`), and the root brief says no ADR
is rewritten ahead of the code that makes it true. At k18 a build still writes
skill directories, so the proposed retirement text is false. This violates the
planning contract's **Ordering** and **Green** conditions and its standing ADR
rule. Keep the identity and current-state records through provisioning's last
live build, or redraw the cut so their consumers and retirements disappear in
the same green leaf.

### 3. The meta-grove handoff protocol is not executable on the live loop

The plan's central premise is false: `report_build_pairing` is not a stopping
guard. It returns `()` and only prints diagnostics on every mismatch
(`src/loop_driver.rs:550-576`); `docs/USAGE.md:164-177` explicitly says it
reports without refusing. It is also called *after*
`provision::reverify_installed` (`src/loop_driver.rs:120-128`), which restores
the running build's embedded methodology before the diagnostic. Consequently
the claims in `.grove/BRIEF.md:123-127`,
`grammar-separator-k15:76-81`, and `open-kind-k20:87-95` that reinstalling
mechanically stops (or later does not stop) the loop do not describe the code.

That false premise leaves three concrete cutovers without a safe handoff:

- `delete-migration-k6` removes `.grove/FORMAT` from this live tree before the
  first planned reinstall, while the installed driver still requires that file
  and classifies its absence as a legacy tree (`src/tree_format.rs:7-29`). The
  old process can refuse or run the very migration k6 just deleted rather than
  hand off to k7 cleanly.
- K18 changes prompt composition and purports to remove the live driver's guard
  but does not reinstall; a running process cannot acquire either source change.
  K19 then deletes provisioning and installs plugin symlinks but also does not
  reinstall, so the still-running old driver continues its per-iteration
  `reverify_installed` and can replace the new delivery before the next launch.
  K20's statement that the running driver has no guard is therefore unsupported.
- `plugins/install.sh` refuses an ordinary run from this mandated jj secondary
  workspace (`plugins/install.sh:113-150`); the resolved main workspace is
  `/Users/antony/Development/grove`. K19 says only to run the script. `--force`
  would deliberately point personal symlinks at this disposable workspace, while
  the main workspace does not yet contain the plugin being reviewed.

The Homebrew/PATH observation itself is correct: on this machine both commands
resolve through `/opt/homebrew/bin` to
`/opt/homebrew/Cellar/grove/19.3.0/bin`, ahead of `~/.cargo/bin`. But overwriting
those targets is not a handoff protocol. This violates the review's
**Meta-grove hazards** condition and the planning contract that every leaf can
land and hand the next leaf a usable toolchain. Specify an actual stop/restart
boundary, order it before any old-build re-verification can clobber the new
delivery, prove the installed executable by behaviour rather than the unchanged
`19.3.0` version string, and give k19 an installation route that works from this
workspace without leaving permanent links to it.

### 4. `plugin-kind-skills-k17` contradicts the spec's rule ownership seam

The leaf correctly says a review/research family's procedure is one file in the
spine and each member directs a named load
(`19-impl-plugin-kind-skills-k17.md:18-22`), but its own `Done when` immediately
requires every member to carry its family's procedure inline (lines 24-31).
Decision 11 requires the former and says *nowhere twice*
(`docs/specs/module-decomposition.md:678-695`). The brief nevertheless maps
decision 11 and test seam 4 to k16/k17 as fully covered.

This violates the review's **Coverage** condition: the mapped leaf's acceptance
criteria do not deliver the mapped decision unambiguously and invite duplicate
owners. Make the `Done when` distinguish kind-owned inline rules from the one
family procedure loaded from the spine, in the same terms as the spec and the
conformance assertion.

### 5. The final coverage checklist drops dispositions and rejected scope

`spec-to-current-state-k23` says it will check seventeen original ADRs, then
lists *four retired, two reworked, seven amended, two unchanged, two added*
(`25-impl-spec-to-current-state-k23.md:26-30`). The spec actually contains
seventeen original records **plus** two additions: four retired, two reworked,
eight amended, one re-checked, and two unchanged
(`docs/specs/module-decomposition.md:772-792`). The proposed checklist therefore
drops one amendment and the `bulk-marks-are-not-atomic` re-check while counting
the two additions as if they completed the seventeen originals.

The root mapping also claims to cover the spec's `## Out of scope` list but has
rows for only four of six entries (`.grove/BRIEF.md:80-83`): the explicit
rejections of serving methodology over MCP and adding harness-plugin invocation
are absent (`docs/specs/module-decomposition.md:826-845`). No implementation
leaf is needed for rejected work, but the mapping must say that explicitly or it
cannot demonstrate the requested walk of the whole list.

This violates `module-split-k4`'s written-coverage contract and the review's
**Coverage** condition. Correct the disposition arithmetic and make every
out-of-scope item visible as deleted, deferred, or rejected/no-work.

## Done when

- Every finding above is resolved in the root brief and planned leaf bodies, or
  rejected there with evidence satisfying the cited contract.
- The revised order has no source or runtime dependency on a later leaf, and
  every leaf can leave the workspace suite green.
- The meta-grove sequence names mechanically checkable installed-build and
  loop-handoff proofs for FORMAT removal, the grammar rename, prompt/plugin
  delivery, provisioning deletion and the open-kind verb change.
- The decision/requirement/test-seam/out-of-scope mapping and the final ADR
  checklist account for their complete source sets.
- No production or test implementation is absorbed here; this leaf repairs the
  planning artifact only.

## Notes

Codebase-memory project `grove-refactor-minimalism` was reviewed at full-index
generation `2026-08-28T11:59:35Z`. All cited source and artifact paths had no
recorded coverage gap except `plugins/install.sh:182-347`; that entire reported
range was read directly before relying on the script. A clean coverage result is
best-effort evidence, not proof of completeness.
