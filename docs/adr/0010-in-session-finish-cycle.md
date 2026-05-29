# The complete finish cycle is an in-session, state-checked LLM step, not Rust automation

ADR-0009 removed the `grove finish` verb and moved finishing into the running session, but deliberately deferred the step-level design — "exact order, partial-failure resume, interactive vs. headless UX" — to a separate decision. This ADR records that design. The **complete finish cycle** is the terminal, whole-grove sequence that retires a grove once `grove-llm pick` reports no live leaves: (1) promote durable artifacts from the briefs to ADRs/docs/glossary; (2) delete `.grove/` in one focused commit; (3) merge the branch into the default branch; (4) remove the worktree; (5) delete the branch. It is carried out by the in-session LLM reasoning over git output — no new Rust automation, no progress-marker file.

## Status
accepted

## Why in-session, not Rust
Two forces point the same way. First, grove's spine (`content/SKILL.md`, constraint 2 "read, don't run"; constraint 6 "walk-away-able") wants the methodology to live in markdown the reader can follow by hand, not in a binary that must succeed for work to proceed. A finish cycle expressed as skill prose stays legible and editable without a release. Second, the cycle's hard part is *judgement under partial failure* — which step already ran, whether a merge conflicted, whether the branch is safe to delete — and that is exactly what an interactive agent reasoning over `git` output does well and what a fixed Rust state machine does badly. Encoding it in Rust would buy determinism the operation does not need while forfeiting the recovery flexibility it does need. The trade-off (no machine-enforced atomicity) is acceptable because every step is individually recoverable in git (see below).

## Step order and the two git-mechanics corrections
The seed sketch ordered the teardown as merge → delete branch → remove worktree, run after a `cd` into the main repo. Tracing the actual commands surfaced two corrections, both now baked into the canonical order:

1. **Worktree-remove precedes branch-delete.** `git branch -d <name>` refuses to delete a branch that is checked out in a live worktree, so the worktree must go first. (Reversing this is the single most likely way to author a finish cycle that wedges.)
2. **No `cd` — use `git -C <repo>`.** The session's working directory is *inside* the worktree it is about to remove. Rather than the fragile "delete the floor I am standing on" `cd` dance the seed worried about, steps 3–5 run `git -C <repo_path>` against the main repo, which needs no particular cwd.

## Merge strategy: plain `git merge`
`git -C <repo> merge <name>` — a fast-forward when the default branch has not advanced since the grove branched, a merge commit when it has. Chosen over `--no-ff`-always (which would manufacture merge bubbles even on a clean fast-forward) and over rebase-then-`--ff-only` (strict linearity is not worth moving conflict resolution earlier for this workflow). The plain-merge fallback reconciles with the one prior grove merge in this repo's history (`3f2b728 "Merge grove: …"`, a real merge bubble — its default had moved). On conflict the cycle stops and the operator (or the in-session LLM) resolves before continuing.

## Resume is state-checked, never a marker file
grove constraint 1 ("artifacts, not state") forbids a finish-progress file. Resume is therefore derived entirely from inspectable state. The two entry conditions are distinguishable by `grove-llm pick`'s own output: **exit 0 with empty stdout and "no live leaves; this grove is done"** means `.grove/` is still present (fresh finish, run from step 1); **a non-zero exit with "grove root not found"** means `.grove/` is already gone (partial-finish resume, steps 1–2 done). From there each step is guarded:

- skip 1–2 if `.grove/` is gone;
- skip 3 if `git -C <repo> merge-base --is-ancestor <name> <default>` passes (already merged);
- skip 4 if the worktree is gone;
- skip 5 if the branch is gone;
- if all are done, report "already finished" and stop.

`grove do` into any half-finished state thus resumes from the first incomplete step. One benign quirk: in the window where the worktree is gone but the branch remains, `grove do` re-attaches the worktree before the session starts, so step 4 simply removes it again — wasteful but convergent.

## Interactive UX and headless behaviour
The cycle is gated by a **single** confirmation before the mechanical teardown (steps 2–5), after promotion (step 1) has produced its reviewable working-tree edits. Per-step confirmation was rejected as the "wizard" anti-pattern (`content/driving.md`) — four flow-breaking prompts — and is redundant anyway because the state-checks decouple confirmation granularity from failure recovery. The gate is safe to collapse because nothing in the cycle is irreversible in git: the `.grove/` deletion is a commit, the merge is revertible, deleting a *merged* branch loses nothing, and a removed worktree is re-attachable. The real risk is finishing the *wrong* grove, which one clear plan-and-confirm addresses.

Headless behaviour needs no mode detection. The skill instructs the LLM to **propose the teardown and wait** for explicit human confirmation, never running steps 2–5 unprompted. Interactive runs get a confirmation and proceed; a headless run with no human present ends the turn with the plan as output and runs nothing destructive. A `--yes-finish` / auto-proceed flag was considered and deferred: it would be a Rust/CLI addition (which this change set deliberately avoids) and is only warranted if unattended nightly finishing is ever actually wanted.

## Considered options
1. **Rust-automated finish cycle (a `finish` subcommand or library state machine).** Rejected: re-introduces the verb ADR-0009 removed, forfeits walk-away-ability, and trades recovery flexibility for atomicity the operation does not need.
2. **Marker file recording finish progress.** Rejected: violates constraint 1; redundant because git and `grove-llm pick`'s exit behaviour already encode the resume point.
3. **Per-step confirmation gauntlet.** Rejected: the wizard anti-pattern; decoupled-from-recovery so it buys nothing.
4. **In-session, single-gate, state-checked cycle (chosen).** Markdown-driven, one confirmation, resume from git state.

## Where it lives
The operational instructions are the **Finish** step of `content/SKILL.md`'s loop (and its materialised copy), not the launcher prompts — per the "launcher prompts stay small" convention, `continue.md` stays a one-liner and the empty-pick→finish trigger is noted in the loop's Pick step. The dead `prompts/finish.md` launcher prompt is removed (no code path has loaded it since ADR-0009 removed the `finish` verb). The human-facing `docs/workflows/finish.md` walkthrough rewrite to this model is tracked as a follow-up.

## Release
No release impact of its own: this is a skill/methodology change, not a CLI-surface change (the breaking surface change — removing the verbs — was ADR-0009, bundled into the next major after v4.0.0). `CONTEXT.md` gains a "complete finish cycle" entry alongside this ADR.
