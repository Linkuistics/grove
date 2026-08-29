# Supported workspace layouts

Grove's teardown ends in one atomic same-filesystem rename of the whole `.grove/`
root into the workspace's VCS-administration control directory, and the
[task-tree transactions fail
closed](task-tree-transactions-fail-closed.md) decision deliberately provides no
copy or working-tree-sibling fallback. A control directory Grove cannot rename
into is therefore a property of the **workspace layout**, not a teardown-time
misfortune: it makes that workspace unfinishable for its whole life. Bare `grove`
validates it during driver-lease acquisition — the one moment where the control
directory has just been resolved and created and the working-tree root descriptor
is already open — before configuration validation and before any `.grove/`
observation, creation, migration, selection, or commit. An unsupported layout is
a resumable no-mutation stop, in the same class as an invalid session
configuration — both are properties the driver establishes directly, unlike a
`grove-llm` pairing mismatch, which it can only proxy and therefore only reports
([one build owns a session](one-build-owns-a-session.md)).

The two required properties are established differently, and only one is
measured. **Untracked** is structural: the resolver places controls exclusively
inside the workspace's own `.jj/`, never a working-tree sibling, so no ordinary
commit can reach them. **Same
device** is contingent on how the operator laid the workspace out, so it is
compared rather than assumed — the preflight stats the created control directory
and the pinned working-tree root and requires one device.

Which layouts qualify follows from a single fact about the resolver: whether
resolution stays inside the working tree.

| Layout | Marker | Control directory | Verdict |
| --- | --- | --- | --- |
| Native jj, default or secondary workspace | `.jj/` | `<workspace root>/.jj/grove/` | In-root; the deliberate non-following of `.jj/repo` keeps a secondary workspace's controls beside its own working copy even though the store is shared |
| Colocated jj, default or secondary workspace | `.jj/` (the `.git` beside it is jj's business) | `<workspace root>/.jj/grove/` | In-root |

Since [*jj is the only lane*](jj-is-the-only-lane.md), every admitted layout is
in-root by construction: `.jj/` sits at the root of the working tree, so the two
operands can differ only where `.jj/` has itself been put elsewhere. Grove still
measures every layout rather than trusting the table: a symlinked `.jj` marker,
or a control directory that is its own mount point, escapes the working tree
without changing the marker's kind. That is now the *whole* at-risk family, and
it is the reason the comparison survives the lane it was written for.

Acquiring the lease is the right gate because it is the unique chokepoint. Root
initialization, session-kind migration, ordinary selection, finish allocation,
and a later driver's transaction recovery all run behind it, so one check covers
every path that can create or drive a task tree — without a second lifecycle
command, a durable capability marker, or a user-visible flag.

The preflight is an early warning and never a license. It does not weaken, and is
not consulted by, the finish transaction's own quarantine preflight, for three
independent reasons. It compares **proxies**: the rename Grove will eventually
perform moves `.grove/` into the control directory's `grove/` child, and at lease
time neither operand need exist, so a `.grove/` that is itself a mount point
passes here and is correctly refused there. Layout is **mutable** while the lease
is held — a remounted `.jj/`, a relocated workspace, or a changed bind mount all
alter the answer, and the lease pins the root's identity rather than the
destination's device. And `finish-commit` is **separately
invocable**, including by an operator retrying a blocked transaction, so it can
attest nothing about which driver validated what. Carrying a startup fact to the
teardown gate would be precisely the stale disposition the transaction's
revalidate-at-every-gate discipline exists to reject.

The accepted cost is that a workspace which would have done months of useful work
before hitting a permanent wall is now refused at the start instead. That is the
trade being made deliberately: the refusal costs an operator nothing recoverable,
because it mutates no tree and creates no revision, and it arrives while
relocating a worktree is still cheap.

## Considered options

- **Discover the constraint only at teardown.** Rejected because the finish gate
  is the last moment an operator can act on it and the first moment they see it:
  the workstream is already complete, there is no fallback, and the remedy —
  relocating the worktree — is at its most expensive. Reopen if a portable atomic
  cross-filesystem move becomes available, which would remove the constraint
  rather than relocate its diagnosis.
- **Validate at root initialization only.** Rejected because it misses every
  workspace whose tree already exists and every layout that changes afterwards,
  and because root initialization is itself the first mutation the check exists to
  prevent. Reopen only if a task tree can no longer outlive the invocation that
  created it.
- **Record a durable capability marker after one successful check.** Rejected
  because it puts lifecycle state outside `.grove/`, and because its only effect
  is to license a fact that has since become false — the layout can change while
  the marker cannot. Reopen only if workspace layout becomes immutable for a
  tree's lifetime.
- **Warn and continue.** Rejected because Grove has no advisory channel, and a
  warning re-emitted on every iteration of a self-relaunching loop is noise
  ignored by construction. The condition it reports is also unconditional rather
  than probabilistic: the workspace cannot finish.
- **Gate only the invocations that could reach teardown.** Rejected because every
  invocation can: a finish leaf becomes eligible the moment no non-finish leaf is
  live, and no driver can know which iteration is the last one. A gate that must
  run on every iteration is not narrower than one at acquisition, only later.
- **Fall back to a copy or a working-tree-sibling quarantine.** Rejected by
  *task-tree-transactions-fail-closed*, which owns that trade; this decision
  records when the resulting constraint is surfaced, not whether it holds.
