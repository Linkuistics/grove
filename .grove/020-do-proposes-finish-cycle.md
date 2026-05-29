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

## Notes
- Depends on 010 having simplified the verb surface (no `finish` verb to reconcile).
