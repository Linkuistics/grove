# finish-lifecycle-k43

**Kind:** impl

## Goal

Complete the bare lifecycle with a resumable driver-owned finish leaf and a
last-moment guarded, path-scoped Git/jj teardown commit.

## Context

- Depends on `lifecycle-cutover-integrate-k41`.
- Binding design: `docs/specs/config-driven-sessions.md` sections "Finish leaf"
  and "Scoped Git and Jujutsu commits", plus the Complete finish cycle glossary
  contract.
- Primary code surfaces: `src/loop_driver.rs`, `src/tree_read.rs`,
  `src/tree_lifecycle.rs`, `src/repo.rs`, `src/complete.rs`, `src/llm_cli.rs`,
  and isolated finish tests in Git/native-jj/colocated-jj worktrees.
- The helper enforces tree/VCS facts; the provisioned methodology, reconciled by
  `methodology-and-viewer-k48`, owns explicit human confirmation.

## Done when

- After valid config and under the exclusive Tree access lock, an empty tree
  gets exactly one `finish` leaf with the next position/key; a declined live
  finish is reused, duplicate finish is malformed, and later ordinary work
  preempts finish without starving it.
- The generated task body is an ordinary `finish-k<key>` leaf with a Goal that
  proposes the complete finish cycle and a Done-when that names its strict
  promote durable material → `grove-llm finish-commit <finish-handle>` →
  `grove-llm complete --done` order; it carries no body kind marker.
- Finish is driver-reserved across generic grow/terminal/promotion verbs, while
  non-finish insertions may target it.
- `grove-llm finish-commit <finish-handle>` revalidates the same live finish and
  absence of non-finish work under the lock, refuses byte-identically when work
  appeared after launch, and deletes/commits `.grove/` only on success.
- Git only/path commits record tracked deletion (including relevant unborn
  behavior) without consuming unrelated staged work; jj fileset commits
  preserve unrelated working-copy changes. Commit messages name the finish
  handle and no separate commit is made for finish allocation.
- The configured `finish` target receives the ordinary mandate. Confirmed
  teardown ends with `complete --done`; decline/crash/no-signal leaves the same
  finish resumable.
- Tests cover allocation/reuse/preemption, every reservation, post-launch work,
  malformed unborn finish, intermediate snapshots, scoped Git/jj preservation,
  finish deletion followed by root initialization with handle reuse and stale-
  epoch refusal, and clean loop stop.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Do not infer or automate human confirmation inside `finish-commit`.
