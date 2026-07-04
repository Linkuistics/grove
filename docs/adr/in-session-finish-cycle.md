# The finish cycle is an in-session, state-checked LLM step, not Rust automation

When `grove-llm pick` reports no live leaves, the grove is finished by a terminal,
whole-grove sequence the in-session LLM carries out by reasoning over `git` output —
no Rust automation, no progress-marker file. The five teardown steps are:

1. **Promote** durable artifacts from the briefs to ADRs / docs / glossary.
2. **Delete `.grove/`** in one focused commit.
3. **Merge** the grove branch into the default branch.
4. **Remove** the worktree.
5. **Delete** the grove branch.

The cycle then ends by signalling the loop to stop (the `grove-llm complete --done`
handoff of the *self-driving-loop* decision), which distinguishes a clean finish
from a crash or interrupt.

## Why in-session, not Rust

Two forces point the same way. grove's spine wants the methodology to live in
markdown a reader can follow by hand ("read, don't run"; "walk-away-able"), not in a
binary that must succeed for work to proceed — a finish cycle expressed as skill
prose stays legible and editable without a release. And the cycle's hard part is
*judgement under partial failure* — which step already ran, whether a merge
conflicted, whether the branch is safe to delete — which an interactive agent
reasoning over `git` does well and a fixed Rust state machine does badly. Encoding it
in Rust would buy determinism the operation does not need while forfeiting the
recovery flexibility it does. The trade-off (no machine-enforced atomicity) is
acceptable because every step is individually recoverable in git.

## Two git-mechanics constraints baked into the order

1. **Worktree-remove precedes branch-delete.** `git branch -d <name>` refuses to
   delete a branch checked out in a live worktree, so the worktree must go first.
   Reversing this is the single most likely way to author a finish cycle that wedges.
2. **No `cd` — use `git -C <repo>`.** The session's working directory is *inside* the
   worktree it is about to remove, so steps 3–5 run `git -C <repo_path>` against the
   main repo, which needs no particular cwd.

## Merge strategy: plain `git merge`

`git -C <repo> merge <name>` — a fast-forward when the default branch has not advanced
since the grove branched, a merge commit when it has. Chosen over always-`--no-ff`
(which manufactures merge bubbles even on a clean fast-forward) and over
rebase-then-`--ff-only` (strict linearity is not worth moving conflict resolution
earlier for this workflow). On conflict the cycle stops and the operator resolves
before continuing.

## Resume is state-checked, never a marker file

A finish-progress file is forbidden ("artifacts, not state"), so resume is derived
entirely from inspectable state. `grove do` into a half-finished grove resumes from
the first incomplete step:

- skip 1–2 if `.grove/` is gone (`grove-llm pick` errors "grove root not found");
- skip 3 if `git -C <repo> merge-base --is-ancestor <name> <default>` passes;
- skip 4 if the worktree is gone;
- skip 5 if the branch is gone;
- if all are done, report "already finished" and stop.

## Interactive UX and headless behaviour

The teardown (steps 2–5) is gated by a **single** confirmation, taken after promotion
(step 1) has produced its reviewable working-tree edits. Per-step confirmation is
rejected as a wizard anti-pattern and is redundant anyway, because the state-checks
decouple confirmation granularity from failure recovery. The single gate is safe
because nothing in the cycle is irreversible in git — the `.grove/` deletion is a
commit, the merge is revertible, deleting a *merged* branch loses nothing, and a
removed worktree is re-attachable. The real risk is finishing the *wrong* grove,
which one clear plan-and-confirm addresses.

Headless behaviour needs no mode detection: the LLM **proposes the teardown and
waits** for explicit human confirmation, never running steps 2–5 unprompted. A
headless run with no human present ends the turn with the plan as output and runs
nothing destructive.

## Where it lives

The operational instructions are the **Finish** step of the grove skill's loop prose,
not a launcher prompt — per the "launcher prompts stay small" convention, and the
empty-pick→finish trigger is noted in the loop's Pick step.
