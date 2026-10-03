# no-selection-parameter-k6

## Goal

A launch carries no selection parameter (D1, D2). Neither of Grove's two calls
to dispatch passes a `--param`: not the lifecycle launch, and not `grove run`'s
selection, which still goes through `inspect` until `confined-run`. The sample
policy selects without one. In a secondary jj workspace it grants the harness
the store derived from `<cwd>/.jj/repo`, in a primary one nothing more, and it
names no session.

## Context

- The spec's *The sample policy and the choice file* and the lifecycle-launch
  table under *Grove integration*. `plan-k1`'s D1 and D2 give the owner's words
  and the accepted cost.
- The sample policy is `crates/harness-dispatch/worker/sample/policy.ts`.
  Today it requires `repo` (as `--add-dir`) and `session_name` (as `claude -n`)
  and refuses without them. The typecheck fixtures under
  `crates/harness-dispatch/worker/typecheck/` use `repo` too. The front carries
  the sample's text, which is what `init` writes.
- The callers that pass parameters today: the driver's argv builder in
  `crates/grove-loop/src/loop_driver.rs` (`session_name`, `worktree`, `repo`),
  and `grove run`'s `inspect` call in `crates/grove/src/standalone.rs`
  (`session_name=standalone:<kind>`, `worktree`, `repo`).
- The seams to extend: the sample cases in `crates/harness-dispatch/tests/`,
  and Grove's launch-boundary suite in `crates/grove/tests/loop_driver.rs`.

## Done when

- Seam 1: the installed sample selects for every kind, under every selection
  it offers, with no parameter passed. Run from a secondary jj workspace it
  grants the store that the workspace's `.jj/repo` names, and from a primary
  one nothing more. It names no session.
- Seam 3: the mandate, kind, task file and handle reach `select`, and no
  parameter does. Its `cwd` is the working-tree root. `grove run`'s selection
  passes no parameter either.
- `task dispatch:typecheck` passes with fixtures that read no `repo`.
- The `grove-loop` and `overview` books follow their source (P2), and
  `## Unreleased` records that an owner policy reading `repo` or
  `session_name` refuses until it is edited.
- `bash scripts/check.sh` passes.

## Notes

- The owner's installed policy is not this leaf's. It reads both parameters,
  and this grove's own sessions run v22, which still passes them.
  `major-release` handles the owner's edit.
- A secondary workspace's `.jj/repo` is a one-line file holding the store's path
  relative to `.jj/` (`../../grove/.jj/repo` in this grove). A primary
  workspace's `.jj/repo` is a directory. The sample's jj knowledge stays in the
  sample: nothing jj-specific enters dispatch's core, SDK or adapter (D1).
- Launch-time naming becomes a policy's own derivation from its cwd, if an
  owner wants one. Otherwise the skill's `/rename` suggestion is the mechanism
  (D2).
