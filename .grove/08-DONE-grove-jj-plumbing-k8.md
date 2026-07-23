# grove-jj-plumbing-k8

**Kind:** work

## Goal

Teach grove's Rust plumbing to work first-class in jj-enabled working trees
(native repos and secondary jj workspaces included), per the decisions in
`grove-jj-integration-k7`: jj-aware plumbing, jj-first detection.

## Context

- **House rule:** all edits and their commit land in Linkuistics/grove
  (`~/Development/grove`); this leaf only tracks. Retiring it here is a
  second, bookkeeping-only commit in this repo.
- Decisions (full log in `07-DONE-grove-jj-integration-k7.md`):
  - **jj-aware plumbing** — thin probe, no VCS trait.
  - **jj-first detection** — `.jj/` present → jj plumbing, even when
    colocated; git only in not-jj-enabled trees.
- The verified call sites (grove source, 2026-07-23):
  - `src/repo.rs` — `git_toplevel` → `jj workspace root`; `git_common_dir`'s
    main-repo derivation → `jj workspace root --name default`. Callers:
    `launch.rs` (×2 + codex grant), `llm_cli.rs:448`, `tree_migrate.rs`.
  - `src/tree_rename.rs` — jj-enabled → take the existing `plain_rename`
    path (jj has no index; it snapshots the working copy). `is_tracked`'s
    git probe must not run in jj trees.
  - `src/tree_migrate.rs` — `commit_migration` via `jj commit -m <msg>
    .grove` (fileset-scoped commit, verified available on jj 0.43).
- Session naming (`<repo-basename>: <name> grove`) rides on repo.rs: the
  rule itself is VCS-neutral, only the probes change.
- Tests: the existing test helpers spin up git scratch repos (`run_git` in
  `tree_grow.rs`/`tree_lifecycle.rs`/`tree_migrate.rs`); add jj-native and
  colocated scratch coverage for root resolution, rename, and migrate.

## Done when

- `grove do` and every `grove-llm` verb work in a jj-native working tree, a
  secondary jj workspace, and a colocated repo (jj-first).
- Tests cover the three repo shapes; `cargo test` green.
- One focused commit in Linkuistics/grove; this leaf retired with its own
  bookkeeping commit here naming `grove-jj-plumbing-k8`.

## Notes

jj 0.43.0 is installed locally. The codex sandbox question is deliberately
NOT here — it is `grove-codex-jj-sandbox-k9`.

## Outcome

Shipped as Linkuistics/grove `b6e45dd` (main). `repo.rs` grew the jj-first
probe `vcs_of` (nearest `.jj/` beats a `.git` beside it; closest marker
wins); `git_toplevel` → `toplevel` (jj workspace root straight from the
walk, no jj binary needed); main-repo derivation → `main_repo_of` via
`jj workspace root --name default --ignore-working-copy`. Rename goes plain
in jj-enabled trees; the adoption migration commits via fileset-scoped
`jj commit -m <msg> .grove` (jj-authored: change-id, op-undoable). Tests
cover all three shapes; 471 pass. Facts for k9/k10: the ambient jj config
may default `git.colocate` **on** (fixtures must force it off), and the
codex `--add-dir` grant still calls `git_common_dir` — it errors in a
jj-native tree, so codex launches there stay broken until k9.
