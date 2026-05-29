# grove-do-subsumes-start-continue-and-prompts-finish-cycle — brief

## Goal
Two lifecycle-verb changes to the `grove` CLI surface:

1. **Remove `grove start`, `grove continue`, and `grove finish`** — `grove do` is
   already a strict superset of start/continue (`launch.rs:42`: no worktree →
   start; live worktree → continue; orphaned branch → re-attach + continue), and
   once `do`/end-of-step proposes the finish cycle (change #2) the standalone
   `finish` verb is redundant too. Make `do` the **sole lifecycle entry verb**.
2. **`do` (and end-of-step) prompts the complete finish cycle** — when a grove
   has no live leaves, the in-session LLM proposes running the full cycle:
   delete `.grove/` commit → merge to default → delete branch → remove worktree.

Same neighbourhood as the prior `grove status`/install-surface cleanup, applying
the same "redundant verb removal" pattern (which removed `list`, `version`, and
merged `update` into `install`).

## Done when
- `grove start`, `grove continue`, and `grove finish` no longer exist as CLI
  verbs; `grove do` is the sole lifecycle entry verb and covers every path; all
  docs/skill/prompt/CONTEXT/ADR references updated. (Note: `do` still dispatches
  to the internal `start()`/`continue_grove()` helpers, which still load the
  `start`/`continue` launcher prompts — only the public verbs are removed.)
- When `grove-llm pick` returns empty (or `.grove/` is gone), a session proposes
  the complete finish cycle and, on confirmation, carries it out end-to-end
  (including merge + branch + worktree deletion, run after `cd`-ing to the main repo).
- The combined breaking-change release story is settled and documented.

## Decomposition
- `010-remove-redundant-verbs` (work) — delete the `Start`/`Continue`/`Finish`
  public verbs; `do` is the sole lifecycle entry verb; sweep all
  docs/skill/prompt/CONTEXT/ADR references. Runs first: simplifies the surface
  that 020 then layers finish behaviour onto.
- `020-do-proposes-finish-cycle` (planning) — grill the still-open finish-cycle
  design (step order, partial-failure/resume, interactive-vs-headless UX, prompt
  rewrites) and grow whatever work leaves it needs.

Ordering encodes dependency: 010 removes references and the `finish` verb; 020
defines the in-session finish flow on the cleaned-up surface.

## Release
Both changes are breaking CLI changes after the v4.0.0 in flight; they bundle
into the **next major** version bump. Per repo convention, releases are cut
manually via `scripts/release-{doctor,build,publish}.sh` — this is a post-merge
step, not a grove leaf. The grove's Finish should leave the branch merged and
note that the release bump + README CLI-reference update is the operator's next
manual action. (Captured here rather than as a thin leaf, per the user's call.)

## Pointers
- Code: `src/cli.rs` (Command enum, lines 20-35), `src/launch.rs`
  (`start`/`continue_grove`/`do_grove` lines 10-62, `finish` line 68).
- Skill loop: `.claude/skills/grove/SKILL.md` ("The loop", Finish step) and the
  launcher prompts in `.claude/skills/grove/prompts/` (start/continue/finish/retire).
- Glossary: `CONTEXT.md` — the three senses of "grove", lifecycle verbs.
- Prior-pattern precedent: v4.0.0 removed `list`/`version`, merged `update` into
  `install` (ADR-0007, ADR-0008).

## Decisions (running log)
- **2026-05-29 — Finish-cycle automation home: in-session LLM, not Rust.**
  The whole complete-finish cycle is driven by the in-session LLM, not new Rust
  automation. Worktree-self-deletion is *not* a blocker: the session `cd`s out
  into the main repo before `git worktree remove` + branch delete. Keeps the
  cycle in the skill (markdown, walk-away-able, "read don't run"); partial-failure
  recovery is the LLM reasoning over git output, acceptable for an interactive agent.
- **2026-05-29 — Trigger model: unify on empty `pick`.**
  Both seed triggers (end-of-step retire cascade empties root; `grove do` on an
  already-finished grove) collapse into one in-session rule: whenever
  `grove-llm pick` returns empty — or `.grove/` is already gone (partial-finish
  resume) — the session proposes the complete finish cycle. `grove do` needs no
  CLI-side finished-detection; it launches `continue` as today and the session
  detects via `pick`. This makes change #2 largely a **skill + launcher-prompts**
  change, not a Rust change.
- **2026-05-29 — `grove finish` the verb is also removed.**
  The same redundancy logic that retires start/continue applies to finish once
  `do`/end-of-step proposes the cycle: `do` becomes the sole lifecycle verb.
  Trade-off accepted: no more force-finish of a grove that still has live leaves
  (retire/clear the leaves first). The `finish.md` launcher prompt becomes
  obsolete (finish moves into the in-session loop, not a launched prompt); the
  Finish *step* of the methodology loop stays.
- **2026-05-29 — Tree shape: `010` work → `020` planning; release as a note.**
  010 removes the redundant verbs first (mechanical + doc sweep); 020 grills the
  finish-cycle design on the simplified surface. No dedicated release leaf —
  release-alignment captured in the "Release" section above. ADRs deferred to the
  leaves where the changes land: 010 raises "`do` is the sole lifecycle verb",
  020 raises "finish cycle is in-session LLM-driven". `CONTEXT.md` left untouched
  this session — it must describe current reality (the verbs still exist until
  010 lands); the finish-cycle glossary entry waits on 020 settling the steps.
