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

## Decisions (running log)

**I1 — the sample grants the repository that holds the named store, not the
store directory alone.** A secondary workspace's `.jj/repo` names the store at
`<main>/.jj/repo`; the sample grants `<main>`. In a colocated repository the
store's git backend lives outside the store (`.jj/repo/store/git_target` is
`../../../.git` in this grove), so a grant of the store alone would leave a
confined harness unable to write the commits it makes. `<main>` is also exactly
the value the `repo` parameter carried (the default workspace's root), so the
parity fixture's `${repo}` slot keeps its recorded meaning and the sample stays
argument-for-argument equal to it in a secondary workspace. The spec's
*The sample policy and the choice file* says so. The derivation is lexical
(`path.resolve`), from the file alone: no jj, nothing in dispatch.

**I2 — the parity fixture stays the record it is.** Its README says it is not
edited, so the sample test maps it: `-n ${session_name}` is dropped (the sample
names no session), and `--add-dir ${repo}` is filled with the main repository
from a secondary workspace and dropped from a primary one.

**I3 — dispatch's generic `--param` feature is untouched.** Its own tests that
use `repo` and `session_name` as arbitrary parameter names test the feature, not
Grove's launch, and stay. What changes is every surface that states what Grove
passes: dispatch's help, `init`'s report, the README, USAGE, configure-grove and
the books.

**I4 — every surface that states Grove's invocation moved with it, not only
the ones a test ties.** Besides the help, `init`'s report, the dispatch README,
USAGE, configure-grove and the two books, the root README's inspect example and
two `docs/ARCHITECTURE.md` sentences named the three parameters and were false
the moment the driver stopped passing them, so they changed here.
`current-state-docs-k20` keeps the rest of the current-state work. Dispatch's
generic help examples now pass `profile`/`label`, so no example a reader copies
suggests Grove's old `repo`.
