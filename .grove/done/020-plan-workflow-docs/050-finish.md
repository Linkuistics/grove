# 050-finish

**Kind:** work

## Goal
Write `docs/workflows/finish.md` — the lifecycle walkthrough for `grove finish add-rate-limiting`: promote durable content out of the grove, delete `.grove/`, merge the branch, clean up the worktree. Closes the example narrative started in `030`/`040`.

## Context
- Assumes the multi-step walkthrough (`040`) is the prior state: every leaf retired into `.grove/done/`, briefs ready to be promoted.
- `content/SKILL.md` "Finish" paragraph is authoritative. This walkthrough demonstrates the verb, not the philosophy.
- `docs/grove.md` "Driving a grove" mentions that the default branch never carries any grove's local state; the walkthrough makes that concrete.

## Done when
- `docs/workflows/finish.md` exists.
- Walks: starting state (all leaves under `.grove/done/`), the **promotion step** the harness does inside the session that `grove finish` exec's (move anything still relevant from briefs upward — to ADRs, glossary, or design docs), the focused commit that **deletes `.grove/`**, the merge back to the default branch, and the worktree/branch cleanup.
- Includes `tree .grove/` showing the final shape just before deletion, then a `git status` post-deletion, then `git log --oneline` on the default branch after merge to show the grove's history as a contiguous run of commits.
- Names the per-project merge convention explicitly as a variable (some teams squash-merge the whole grove branch, some fast-forward, some PR-and-rebase). Show one example and tell the reader to substitute.
- Brief note on the multi-harness stamp cleanup: if `.grove-stamps/add-rate-limiting` exists it is removed alongside the worktree.
- Short Codex-equivalent callout.

## Notes
- The promotion step is the highest-stakes thing in the walkthrough — get it right. It is NOT a CLI operation: it happens in the exec'd session driven by `prompts/finish.md`. The walkthrough should make clear that `grove finish` *launches the harness with the finish prompt*; the harness does the promotion; the CLI then deletes and merges only after that session commits.
- Do not advocate for one merge style. grove guides, it does not gate (constraint 5). The walkthrough should reflect that ethos.
- This is the last writing leaf before the index; if any of the running-example assumptions broke during `030`/`040`, fix them here before the index ties everything together.
