# grove.harness-and-model-chosen-via-config — brief

## Goal

Simplify Grove so one bare `grove` command drives every session from the task
tree, while one personal KDL file is the entirety of user launch policy.

## Done when

- `~/.config/grove/config.kdl` is the only user configuration and contains one
  complete command template for every session kind.
- A single driver-side pick selects each real leaf; its filename supplies the
  session kind, `${prompt}` carries its stable handle as the mandate, and the
  launched session resolves that handle without picking again.
- Bare `grove` initializes requirements, migrates legacy trees, drives the loop,
  and materializes a resumable finish leaf without subcommands or flags;
  complete config validation precedes every one of those mutations.
- Legacy harness/model routing, stamps, user-settable environment overrides,
  task-body kind/harness fields, one-off retire/migrate commands, review target
  receipts, diversity warnings, structured harness-routing peeks,
  `GROVE_SESSION_TARGET`, and grow-verb harness flags are removed.
- Git and jj behavior, migration, error paths, direct process execution, and the
  self-driving loop are verified through the agreed seams; `content/`, the
  doubt skill, user documentation, and the minimum coherent ADR/spec set all
  describe the resulting system.

## Decomposition

- `plan-chain-k2`: establish, adversarially review, and integrate the shared
  requirements captured in this brief and `CONTEXT.md`.
- `config-driven-sessions-chain-k5`: design and review the durable command,
  configuration, task-tree, lifecycle, migration, and deletion contracts.
- `implementation-slices-chain-k9`: after design integration, cut and review the
  vertical implementation slices; implementation work is intentionally deferred
  to the tree this planning stage grows.

## Pointers

- Glossary: `CONTEXT.md` — Grove configuration, Driver lease, Session epoch,
  Session kind, Kind routing, Session-kind migration, Review target diversity,
  Pick, Complete finish cycle.
- Current behavior to reconcile: `docs/CONFIGURATION.md`, `docs/USAGE.md`, and
  `docs/ARCHITECTURE.md`.
- Records in scope: `docs/adr/complete-session-configuration.md`,
  `docs/adr/grove-owns-escalated-review.md`,
  `docs/adr/one-live-driver-per-working-tree.md`,
  `docs/adr/task-tree-transactions-fail-closed.md`, and
  `docs/specs/doubt-grove-review-mechanics.md`.
- Test seams: the bare `grove` process in isolated Git/jj worktrees with fake
  commands, and the internal `grove-llm` tree interface.

## Notes

Confirmed and closed to design: one bare command; one strict nineteen-kind KDL
configuration; one POSIX-shell-word command-template string per kind, executed
directly without a shell; one authoritative driver pick; real requirements and
finish leaves; automatic restart-safe legacy migration; no receipts or diversity
warnings; and no inferred harness, model, or reasoning effort. `${prompt}` occurs
once after a literal executable word, not necessarily last. The KDL document is
nineteen flat kind nodes with one string each. `.grove/FORMAT` is the positive
session-kind-format witness; the working-tree-root descriptor is the universal
tree lock; a finish leaf is eligible only when no non-finish work remains; and a
mandated producer may promote without a second pick. The embedded launcher plus
mandate relies on the provisioned Grove skill, standalone legacy `research` maps
to `research-a`, and pair target diversity is configuration-owner policy.
This is a meta-grove, and that constraint outlives the node it was written
under (`legacy-launch-removal-k46`, now closed): the **installed** v16.5.0
driver and its running loop own this workstream through finish, so its `grove
do` control path must not be replaced by a branch build of the bare command
before the grove finishes. Exercise branch binaries only in isolated homes or
worktrees with ambient loop authority scrubbed; `acceptance-verification-k50`
owns the pre-install matrix. The **working directory is itself a selector**, and
the sharpest edge here: bare `grove` takes no arguments, so it adopts, migrates
and commits whatever tree it finds at `$PWD` — running a branch build from the
repository root migrates *this* grove out from under the installed driver, with
no prompt. Drive branch binaries only through `cargo test` fixtures, which set
`current_dir` on every child, or from a subshell already inside the fixture; a
scrubbed environment does not help, because none of the scrubbed variables
select the tree. (`jj op restore` recovers it if this happens.)

Promoted from `acceptance-verification-k50` on its close, because it outlives
that node: **do not publish a release from this grove.** Cutting a tag, running
`scripts/release-publish.sh`, integrating the branch, and tearing down the
working tree are all outside this workstream — the last two by methodology
(`CONTEXT.md`, *Complete finish cycle*), publishing by this grove's own scope.
Verification may run `scripts/release-doctor.sh`, which installs and changes
nothing.

No-signal exits retain status and elapsed time, scoped Git deletion uses
only/path mode, and internal tool/grace injection is not user configuration.
`docs/specs/config-driven-sessions.md` is the stable input to implementation
planning. `driver-exclusivity-k14` establishes a process-scoped driver lease and
per-launch session epoch: a second driver is refused, admitted old operations
finish before crash handoff, orphan calls begun after exclusive invalidation
fail, lock descriptors never cross exec, and epoch rotation makes a durable
grove-generation suffix
unnecessary. Its integrated contract places untracked controls in the exact
Git-worktree or jj-workspace administration area rather than an ambient temp
directory, uses OS-random 128-bit process and launch identities, revalidates
locked file identity, and bounds every epoch handoff so an orphan produces a
visible `blocked` stop instead of parking the loop.
