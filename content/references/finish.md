Your leaf is the driver-owned `finish` sentinel at the grove root, mandated under
the `finish` session kind.
It is a real, **resumable** task: it carries its own stable handle
(`finish-k<key>`, which step 2 needs), and it is created once and reused, never
duplicated. Ordinary work inserted ahead of it (`leaf-insert`) makes the driver
pass it over until that work is terminal, so the sentinel can neither starve nor
preempt real work. The **complete finish cycle** itself is driven in-session by
the LLM (no Rust automation): the session **proposes** it and **waits for
explicit human confirmation before any teardown** — never run steps 2–3
unprompted; with no human to ask, report the plan instead.

**How this session ends is decided by what it did, and the three outcomes are
stated together in `SIGNAL-FINISH.md`** — the same bytes the driver inlines as
the last part of a `finish` prompt, so the outcomes have one source and two
deliveries of it, and neither can drift from the other. On confirmation, run:

1. **Promote** anything from the briefs that should outlive the grove — ADRs,
   docs, glossary entries. Reviewable working-tree edits; often a near no-op
   when decisions landed inline as they were made.
2. **Tear the tree down with `grove-llm finish-commit <finish-handle>`** — the
   live `finish` leaf's stable handle, e.g. `finish-k42`. Never delete `.grove/`
   by hand and commit it yourself: the helper revalidates the live finish leaf
   and the absence of new work under the tree lock, refuses an untracked tree,
   and then deletes `.grove/` and takes one path-scoped `jj commit` — so
   unrelated working-copy changes stay uncommitted, which a by-hand `jj commit`
   would not guarantee.

   **Grove implements no transaction around that, and neither should you.** The
   version control system owns it: jj snapshots the working copy before every
   command and its operation log is the transaction record. So a failure stops
   with a message naming the command that puts the tree back — `jj restore
   .grove` if the deletion is what failed, `jj undo` if the commit is — and no
   Grove-authored recovery runs. Once the tree is back, fix what failed and
   rerun the same command with the same handle. If you cannot tell what failed,
   stop and hand the diagnostic to the human; Grove never rewrites history to
   clear anything.
3. **End on the row that matches what this session did** — the endings and which
   one each outcome takes are `SIGNAL-FINISH.md`'s, and the signal is the **very
   last** action. It must come last: the loop driver is watching for the signal
   file and ends this session after a short grace, so signalling any earlier
   would cut teardown short. Run it from inside this session's working tree —
   the verb resolves the current directory to verify the live session epoch,
   which stays valid after `.grove/` is deleted, and it writes only the launch's
   randomly named signal file in the workspace's VCS-administration control
   directory, nothing in the working tree.

Nothing after: integrating the grove's branch and tearing down the working tree
are **not** grove workflow — both belong to jj, to `gh`, or to the user's own
workspace tooling (user-owned-worktrees). Whoever integrates does so after step
2, so the integrated history never carries `.grove/`.

**Resume is state-checked, never a marker file** (constraint 1) — and the state
that gets checked is the *repository's*, never task-root absence. If you lose
step 2's result, rerun `grove-llm finish-commit <finish-handle>` with the same
handle: with `.grove/` already gone it verifies the immediate VCS result rather
than trusting the absence, and reports idempotent success only for an exact
handle-and-attempt-named commit whose sole change is deleting `.grove/`.
Success there means step 2 is done — go to step 3. A refusal means teardown did
*not* complete, however rootless the tree looks; report it and stop. That proof
is bound to this launch, so it is available only to the still-confirmed session
that ran the command — a later bare `grove` into a rootless tree is an ordinary
fresh grove, not a resumed finish.

**Ending after step 2 but before step 3 is an ordinary no-signal stop.** The
driver reports the child's real status and elapsed time and stops the loop; it
never reads a deleted `.grove/` as the `--done` you did not send. Nothing is
lost — the teardown commit is already in history — and nothing is pending: there
is no half-finished grove to resume, only a working tree without one.

**finish** (HITL, driver-reserved) — the whole-grove teardown session. It
proposes the complete finish cycle and waits for explicit human confirmation
before any teardown; declining leaves the leaf live for a later resume, and a
`finish` leaf is never retired.
