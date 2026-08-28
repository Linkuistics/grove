# spec-to-current-state-k23

## Goal

Rewrite `docs/specs/module-decomposition.md` to describe the design's **current
state**, and close out the whole decomposition: the ADR set, the glossary, the
vocabulary map and the release process.

## Context

The spec says so itself: *"This spec is not one of the retirements. Its
`## Problem` and its *what changes* framing are transient, but what it
describes — how the module boundaries work — is a spec's own grain, and four
artifacts already link into this area. The leaf that lands the last decision
rewrites it to current state rather than deleting it."*

`SPEC-FORMAT.md` governs the rewrite; `ADR-FORMAT.md` governs the set check.

## Done when

- `docs/specs/module-decomposition.md` describes five modules that exist, with no
  `## Problem`, no *what changes*, no ADR-reconciliation table and no line-count
  arithmetic about a codebase that no longer looks like that. The two records
  added along the way — `jj-is-the-only-lane` and `a-kind-is-an-open-token` —
  carry the trade-offs the rewrite drops, which is why they were added.
- **Every one of the seventeen original records is confirmed to have landed its
  disposition**, walking the spec's own `## ADR reconciliation` table as a
  checklist: four retired, two reworked, seven amended, two unchanged, two added.
  A record whose disposition did not land is either done here or named as an open
  leaf — it is not left quietly false.
- `docs/ARCHITECTURE.md`, `docs/USAGE.md`, `docs/CONFIGURATION.md` and
  `README.md` describe the built thing.
- `CONTEXT.md` and `CONTEXT-MAP.md` are reconciled: the collision table now spans
  four crates plus a plugin, and every term a module owns is attributed to it.
  `CONTEXT-FORMAT.md` governs.
- **The manifest exclusion that kept `ordinal-fs-tree` out of the release cut is
  removed**, so one release process answers *is this crate published on its own*
  deliberately rather than by accident. `docs/RELEASING.md` reflects the answer.
- The brief's five `Done when` conditions are checked, one by one, against the
  built code and the answer written into the commit message:
  1. Each module is testable through its own interface without the other four.
  2. The tree store is the only thing that touches the task tree.
  3. The runner is the only thing that spawns or supervises a process.
  4. Adding a task type is authoring content, not editing and rebuilding a binary.
  5. No module reimplements a guarantee the VCS already provides.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is documentation only — no code changes unless a
done-when check fails, in which case the gap is a **new leaf**, not work absorbed
here. That is the whole discipline: this leaf's job is to find out whether the
decomposition is done, not to finish it.

**Condition 4 is the one with no compiler behind it.** The other four are
structural and a reviewer can check them by reading `Cargo.toml` and the crate
boundaries. *Adding a task type is authoring content* is checked by **doing it**:
write a twentieth skill, add its configuration key, cut a leaf of that kind, and
watch it launch — without touching or rebuilding a binary. Do that as the
verification, and delete the throwaway afterwards.

**Do not tear the grove down here.** The finish cycle is the driver's to trigger
and the `finish` kind's to perform, on explicit human confirmation. Retiring this
leaf is an ordinary retirement.
