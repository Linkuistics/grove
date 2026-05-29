# 030-rewrite-finish-walkthrough

**Kind:** work

## Goal
Rewrite the human-facing `docs/workflows/finish.md` lifecycle walkthrough to the
in-session finish-cycle model settled in 020 (ADR-0010), and sweep the remaining
doc references to the removed `grove finish` verb / `prompts/finish.md` launcher
prompt so nothing points at artifacts that no longer exist.

## Context
The design is fully settled — transcribe it, don't re-decide it:
- **ADR-0010** (`docs/adr/0010-in-session-finish-cycle.md`) — the authoritative
  step-level design: 5-step cycle, plain `git merge`, `git -C <repo>` + worktree-
  before-branch, single confirmation gate, propose-and-wait headless behaviour,
  state-checked resume.
- **SKILL.md Finish step** (`content/SKILL.md` + materialised copy) — the
  operational prose; the walkthrough should agree with it, not diverge.
- **CONTEXT.md** — the "Complete finish cycle" glossary entry.

`docs/workflows/finish.md` currently opens with a "⚠️ Superseded — pending
rewrite" banner pointing at grove leaf `020`. Its body still describes a
`grove finish` *verb* invocation (gone) and references `prompts/finish.md`
(deleted). The walkthrough's *what* (promote → delete `.grove/` → merge → drop
branch + worktree) is broadly right; the *trigger* and several mechanics are not.

Specific fixes the rewrite must make (vs. the current draft):
- Trigger is **empty `grove-llm pick`**, not `grove finish <name>`.
- **Worktree-remove precedes branch-delete** (the current "Step 4" has the
  `git branch -d` after `worktree remove` already — verify and keep).
- Merge strategy is **plain `git merge`** (the current page offers ff-only /
  squash / PR shapes as a menu; reconcile with the now-canonical plain merge —
  the page may keep "convention is the variable" framing but should lead with
  the canonical plain merge and drop the claim that the session "picks whichever
  shape fits").
- The session runs the **whole** cycle in-session on confirmation (the current
  page says "session does 1–3, then exits, you run step 4 from the shell" —
  that split is obsolete).
- Drop the references to `prompts/finish.md` as "the canonical instructions"
  (deleted); point at the SKILL.md Finish step instead.
- Remove the "⚠️ Superseded" banner once rewritten.

## Done when
- `docs/workflows/finish.md` describes the in-session cycle accurately, with the
  superseded banner gone and no reference to the removed verb or deleted prompt.
- Remaining `finish` references swept and each verified correct or fixed:
  `docs/workflows/README.md`, `docs/workflows/multi-step.md`,
  `docs/workflows/update.md`, `README.md` (CLI reference), `src/tui.rs` (check
  any "finish" label/string). Most were touched by 010's sweep; this is a
  verification pass plus the finish.md rewrite, not a re-sweep of the verb removal.
- No dangling links to `prompts/finish.md` anywhere under `docs/` or `content/`.

## Notes
- Pure documentation work — no behaviour change, no Rust change. Verify with a
  repo-wide `grep -rn "grove finish\|prompts/finish" docs/ content/ README.md`
  returning only intentional historical mentions (e.g. ADR bodies, which are
  immutable per the ADR-0007/0009 stance).
