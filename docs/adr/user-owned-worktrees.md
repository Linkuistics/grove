# Groves do not create worktrees — git topology is user-owned

grove never creates, integrates, or tears down git topology. The workflow's one
precondition is *a git working tree* — user-provided, on any branch, anywhere on
disk; linked worktree and main checkout are equally valid. The grove's name is the
working tree's directory basename, and grove reads no branch, ever. There are no
topology convenience verbs either: worktree creation, branch integration (merge or
PR), and worktree/branch teardown all belong to the user's own tooling (plain
git/gh, or a dedicated worktree manager such as
[worktrunk](https://github.com/max-sixty/worktrunk)).

## Why it binds

Worktree-management tools work best when *they* create the worktree and run the
grove process from inside it. A grove-owned canonical layout —
`<repo>/.grove-worktrees/<name>/` on a same-named branch, created by `grove do` and
destroyed by the finish cycle — fights any such tool: grove and the tool each
assume they own placement, naming, and teardown. Ceding the whole axis is strictly
more general: grove keeps everything it is actually about (the `.grove/` tree, the
loop, the methodology) and drops everything git already does better elsewhere.
Symmetry is the guard-rail: because grove creates no topology, it also merges none
and deletes none — the complete finish cycle ends at deleting `.grove/` and
signalling done (see *in-session-finish-cycle*).

## Considered options

- **Grove-owned topology** (the pre-v11 scheme: `do <name>` creates
  `.grove-worktrees/<name>` + branch, re-attaches orphans, finish merges and
  deletes both). Rejected: the canonical layout blocks worktree-tool interop, and
  every derived path (`retire <name>/<node-path>`, orphan recovery, harness-stamp
  addressing) hard-codes it further. Nothing would reopen this — running in place
  subsumes the old scheme (a user who wants the old layout can simply create it).
- **Topology utilities** (`grove create <name>` / `grove remove <name|path>` as
  conveniences outside the workflow). Considered and eliminated in the same
  grilling: they re-entrench a default layout, duplicate what dedicated tools do
  better, and put grove back in the ownership business through the side door.
  Reopened if a real workflow emerges with no external tooling where the
  convenience is demonstrably missed.
- **Merge stays in the finish cycle** (integration in-session, only teardown
  external). Rejected: integration style — local merge vs PR — is a property of
  the user's flow, not of grove; the tools that own worktree creation typically
  own integration too.

## Consequences

- `grove do` is argument-less and run from inside the working tree; state dispatch
  (bootstrap / continue / propose-finish) is unchanged (*do-is-sole-lifecycle-verb*).
- `grove retire` addresses its node as `grove retire <node-path>`, in-worktree.
- The grove name (root-brief title, session name `<repo-basename>: <name> grove`,
  harness-stamp key) derives from `git rev-parse --show-toplevel`'s basename.
- "The default branch never carries grove state" is no longer grove's guarantee to
  make — whoever integrates the branch does so after the finish cycle has deleted
  `.grove/`.
