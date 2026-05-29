# 020-do-proposes-finish-cycle

**Kind:** planning

## Goal
Settle the design of the in-session **complete finish cycle** and grow whatever
work leaves it needs. Foundations are already settled (root BRIEF running log):
the cycle is in-session LLM-driven (no new Rust), and it triggers whenever
`grove-llm pick` returns empty (or `.grove/` is gone). This leaf grills the
still-open design questions and writes the skill/prompt changes (or child work
leaves for them).

## Context
- Root BRIEF running log — the two settled foundations and the `finish`-verb
  removal.
- `.claude/skills/grove/SKILL.md` — current "Finish" step: "promote anything that
  should outlive the grove (ADRs/docs/glossary), then delete `.grove/` in one
  focused commit before merging the branch to the default branch." It is **silent
  on branch-delete and worktree-delete** — that gap is what this leaf closes.
- `.claude/skills/grove/prompts/` — `continue.md`, `retire.md`, `finish.md`. The
  bootstrap/continue flow must learn to propose finish on empty pick; `finish.md`
  is likely deleted (verb gone) with its content folded into the loop.
- `src/launch.rs:148` `exec_harness` — sessions run with `current_dir(worktree)`;
  the cycle must `cd` to the main repo (`repo_path`) before `git worktree remove`
  + branch delete.

## Open questions to grill (≥3 interdependent → grilling justified)
1. **Exact cycle step order & atomicity/resume.** Proposed: (a) promote
   briefs→ADRs/glossary; (b) delete `.grove/` in a focused commit; (c) `cd` to
   main repo; (d) merge branch to default (fast-forward? no-ff? what on
   conflict?); (e) delete the branch; (f) `git worktree remove`. What happens on
   partial failure at each step, and how does a re-run (`grove do` into the
   half-finished state) resume?
2. **Interactive prompt UX & granularity.** All-or-nothing confirm, or per-step
   (the seed wrote "[merge to default branch / delete branch / delete worktree]")?
   How does the LLM present it?
3. **Headless / non-interactive behaviour.** With the cycle in-session, a headless
   harness run has no human to confirm. Does the LLM stop-and-report, or is there
   an opt-in auto-proceed signal (the seed floated `--yes-finish`/`--no-prompt`)?
4. **Where the instructions live.** Rewrite of the SKILL.md Finish step + which
   launcher prompts change/are deleted; whether the empty-pick→finish proposal
   belongs in the loop's Pick/Retire description or the continue prompt.
5. **Glossary + ADR.** Define "complete finish cycle" in `CONTEXT.md` once the
   steps settle; raise an ADR recording "finish cycle is in-session LLM-driven,
   not Rust automation" (decision settled in root BRIEF; this leaf supplies the
   step-level detail and trade-off rationale).

## Done when
- The open questions above are settled (recorded inline as they land).
- SKILL.md Finish step + launcher prompts updated (directly or via child work
  leaves); `CONTEXT.md` gains the finish-cycle entry; ADR raised.
- A grove can be carried from empty-pick through merge + branch delete + worktree
  removal entirely in-session.

## Decisions (running log)
- **2026-05-29 — Canonical happy-path sequence (Q1).** Five steps:
  (1) promote durable artifacts from briefs → ADRs/docs/glossary; (2) delete
  `.grove/` in one focused commit on the grove branch; (3) merge into default;
  (4) `git -C <repo> worktree remove <worktree>`; (5) `git -C <repo> branch -d
  <name>` (safe delete — succeeds only because merged). Two corrections to the
  seed's proposed order: **worktree-remove precedes branch-delete** (git refuses
  `branch -d` on a branch checked out in a live worktree), and **no `cd`** — all
  post-commit ops run `git -C <repo_path>` against the main repo, sidestepping
  the "remove the worktree I'm standing in" problem the seed worried about.
- **2026-05-29 — Merge strategy: plain `git merge` (Q1b).** `git -C <repo>
  merge <name>` — fast-forwards when the default branch hasn't advanced, creates
  a merge commit when it has. Never fails, never blocks. User chose ff-preferred
  over `--no-ff`-always; the plain-merge fallback reconciles with the prior grove
  merge `3f2b728` (a real merge bubble — its default had moved). Rejected:
  `--no-ff` always (unwanted bubbles on clean ff), and rebase+`--ff-only`
  (strict linearity not worth moving conflict-resolution earlier for a
  solo-ish workflow).

- **2026-05-29 — Resume is state-checked, no marker (Q2).** Constraint 1
  (artifacts, not state) forbids a finish-progress marker file. Resume is driven
  by inspecting git/filesystem state: each step is guarded by a check, and
  `grove do` into a half-finished grove resumes from the first incomplete step.
  The two triggers are distinguishable by `grove-llm pick`'s own output:
  **exit 0 + empty stdout + "no live leaves; this grove is done"** = fresh
  finish, `.grove/` still present, run from step 1; **non-zero exit + "grove
  root not found"** = `.grove/` already gone, steps 1–2 done, resume from the
  merge. Guards: skip 1–2 if `.grove/` gone; skip 3 if `git -C <repo>
  merge-base --is-ancestor <name> <default>` passes; skip 4 if worktree gone;
  skip 5 if branch gone; all-done → report "already finished" and stop. (Quirk
  noted: in the worktree-gone-but-branch-present window, `grove do` re-attaches
  the worktree before the session starts, so step 4 re-removes it — wasteful but
  convergent.)

- **2026-05-29 — Single confirmation gate (Q3).** Step 1 (promote) is done as
  normal reviewable session work (ADR/glossary/doc edits land in the diff). The
  mechanical teardown (steps 2–5) is gated by **one** up-front confirmation that
  presents the concrete plan (merge target, branch name, worktree path), then
  runs all of 2–5 reporting each. Rejected per-step confirm (the "wizard"
  anti-pattern; 4 flow-breaking prompts) — safe to collapse because the Q2
  state-checks decouple confirmation granularity from failure recovery. Grounded
  by reversibility: nothing in the cycle is irreversible in git (every step is a
  commit/merge/merged-branch-delete/worktree-detach), so the real risk is
  finishing the *wrong* grove, which one clear plan-and-confirm addresses.
- **2026-05-29 — Headless: propose-and-wait, no flag (Q4).** The skill instructs
  the LLM to propose the teardown and **wait** for explicit human confirmation,
  never running steps 2–5 without it. This is safe in both modes without the LLM
  detecting which it's in: interactive → human confirms → proceeds; headless →
  no answer → the turn ends with the plan as output (stop-and-report). No
  `--yes-finish` / auto-proceed flag in this grove — that would be a Rust/CLI
  change the BRIEF steers away from; deferred as a future opt-in leaf only if
  unattended finishing is ever wanted.

- **2026-05-29 — Where instructions live + scope split (Q5/Q6).** The
  operational finish instructions live in `SKILL.md` (the loop's Finish step),
  **not** in `continue.md` — the "launcher prompts stay small" rule keeps
  `continue.md` a one-liner; the empty-pick→finish trigger is already noted in
  the Pick step. This session (020) does the tightly-coupled core: rewrite the
  SKILL.md Finish step (both `content/SKILL.md` and the materialised
  `.claude/skills/grove/SKILL.md`), delete the dead `finish.md` launcher prompt
  (`content/prompts/` + `.claude/` copy — no code path loads it since 010
  removed the `finish` verb), add a "complete finish cycle" entry to
  `CONTEXT.md`, and raise **ADR-0010** recording the in-session step-level
  design (ADR-0009 deliberately deferred this detail). The larger, separable
  human-doc work — rewriting the `docs/workflows/finish.md` walkthrough (already
  flagged "superseded — pending rewrite", pointing here) to the in-session model
  plus sweeping remaining doc refs — becomes child work leaf
  `030-rewrite-finish-walkthrough`, kept out of this commit to preserve
  one-task-one-focused-commit.

## Notes
- Depends on 010 having simplified the verb surface (no `finish` verb to reconcile).
