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
   by hand and commit it yourself. The helper revalidates the live finish and
   the absence of new work, then runs teardown as **one fail-closed
   transaction**: the tree is evacuated beneath a `.grove/FINISHING-<handle>/`
   witness and stays visibly present until the repository proves the exact
   `.grove/`-scoped commit named by that handle and this launch's finish-attempt
   identity; only then does the whole root move, in one atomic rename, to a
   cleanup quarantine. **An absent `.grove/` never proves teardown
   succeeded** — a death before the commit exposes exactly that shape, which is
   why the by-hand version is unsafe (ADR *task-tree-transactions-fail-closed*).
   Every reported failure is retryable: an uncommitted one restores the live
   finish tree, so just rerun the same command. If the diagnostic says
   **`Recovery pending`**, stop and hand it to the human — it names the artifact
   holding the blocked transaction, the recorded and observed topology, and the
   two operator exits (restore the recorded start to roll back, or make the
   exact teardown result immediate to finish forward). Grove never rewrites
   history to clear it, and neither should you.
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
are **not** grove workflow — both belong to plain git/gh or jj, or the user's
own worktree tooling (user-owned-worktrees). Whoever integrates does so after step
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
