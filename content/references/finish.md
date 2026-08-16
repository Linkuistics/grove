<!-- file: order=18 -->
<!-- unit: skill-finish-cycle kinds=finish class=triggering -->
Once no ordinary live leaf is left, bare `grove` appends one driver-owned
`finish` leaf at the grove root and mandates it under the `finish` session kind;
that mandate *is* the signal, so a finish session never asks `pick` anything.
The leaf is a real, **resumable** task: it carries its own stable handle
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

<!-- unit: skill-finish-steps class=procedural -->
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
3. **End the loop cleanly**: run **`grove-llm complete --done`** as the **very
   last** action, then do nothing else. This signals the self-driving loop to
   *stop* (vs the per-task `complete`, which relaunches), so a clean finish is
   distinct from a crash or Ctrl-C. It must come last: like the per-task signal
   it ends this session after a short grace (applied by the loop driver, which
   is watching for the signal file — not this verb), so running it any earlier
   would cut teardown short. It writes only the launch's randomly named loop
   signal file in the workspace's VCS-administration control directory —
   nothing in the working tree. It still resolves the current directory to
   verify the live session epoch, so run it from inside that session's working
   tree (which remains valid after `.grove/` is deleted).
   Outside a `grove` loop (no loop to stop) it is a safe no-op: just exit.

<!-- unit: skill-finish-nothing-after class=procedural -->
Nothing after: integrating the grove's branch and tearing down the working tree
are **not** grove workflow — both belong to plain git/gh or jj, or the user's
own worktree tooling (user-owned-worktrees). Whoever integrates does so after step
2, so the integrated history never carries `.grove/`.

<!-- unit: skill-finish-resume class=procedural -->
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

<!-- unit: skill-finish-no-signal-stop class=procedural -->
**Ending after step 2 but before step 3 is an ordinary no-signal stop.** The
driver reports the child's real status and elapsed time and stops the loop; it
never reads a deleted `.grove/` as the `--done` you did not send. Nothing is
lost — the teardown commit is already in history — and nothing is pending: there
is no half-finished grove to resume, only a working tree without one.

<!-- unit: task-finish-session kinds=finish class=triggering -->
**finish** (HITL, driver-reserved) — the whole-grove teardown session the driver
appends once no ordinary work is live. It proposes the complete finish cycle and
waits for explicit human confirmation before any teardown; declining leaves the
leaf live for a later resume. No session creates one, and none is ever retired.

