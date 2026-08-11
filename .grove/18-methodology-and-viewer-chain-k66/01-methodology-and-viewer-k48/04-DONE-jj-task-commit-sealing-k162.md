# jj-task-commit-sealing-k162

**Kind:** impl

## Goal

Make the methodology's Commit step keep "one task = one focused commit" true in
a jj-enabled tree, where the working copy is itself a commit.

## Context

- Demonstrated during `finish-transaction-handoff-hardening-k138`: the previous
  session recorded its task by describing `@` without opening a fresh change, so
  this session's first edit was snapshotted into the *previous* task's commit
  and had to be split back out before it could be committed under its own
  handle.
- The Commit step currently says only "one task = one focused commit" and
  "name the work item by its `<slug>-k<key>` handle". Neither half survives a
  jj tree unless the change is also sealed, because the next session inherits a
  live working-copy commit rather than a clean slate.
- `linkuistics:using-jujutsu` already prescribes the `new → work → describe →
  new` lane; the gap is that Grove's own Commit step never sends a session
  there, while the rest of the methodology is explicitly jj-first (tree verbs
  already branch on a jj-enabled tree).
- Splitting after the fact is expensive and lossy: a file touched by both tasks
  cannot be separated by `jj split <fileset>`, so recovery needs the operation
  log.

## Done when

- `content/SKILL.md`'s Commit step states how a session leaves the working copy
  in a jj-enabled tree so the next session starts on its own change, without
  turning the step into a VCS tutorial or duplicating `using-jujutsu`.
- The guidance is symmetric: it says what git and jj sessions each do, matching
  how the tree-mutation verbs already describe themselves.
- Canonical-guidance and reference-navigation tests pass; `cargo fmt --check`
  and `cargo test --locked` pass.

## Notes

Sibling of the lifecycle/session-kind/review methodology leaves under
`methodology-and-viewer-k48`, so it lands inside the same review chain. Keep it
to the Commit step — session-start bootstrapping is not in scope.
