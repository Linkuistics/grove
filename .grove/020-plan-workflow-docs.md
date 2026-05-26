# 020-plan-workflow-docs

**Kind:** planning

## Goal
Decide the shape of grove's lifecycle walkthrough docs and grow the tree with the actual writing leaves. Five flows must be covered:

1. **Installing** — `grove install` against a target repo from scratch.
2. **Updating** — `grove update` to refresh an existing install (and the ADR nudge).
3. **Starting a grove** — `grove start <name>`: worktree creation, branching, bootstrap session.
4. **Working through a multi-step grove** — the inner loop: `grove continue` runs, planning leaves growing the tree, retiring branches into `done/`, the brief chain as session context.
5. **Finishing a grove** — `grove finish`: promoting durable content out, deleting `.grove/`, merging.

Each walkthrough must show every command, explain what happens, and show what changed in the repo/worktree at that step.

## Context
- The auto-commit feature from `010-implement-path-scoped-commit.md` should already have shipped; the install/update walkthrough demonstrates that new behavior.
- Existing docs to align with / extend / supersede: `README.md` (CLI surface), `docs/grove.md` (what grove is and why), `content/SKILL.md` (the methodology agents read).
- The grove skill itself is bundled inside this repo at `content/SKILL.md` plus the reference files in `.claude/skills/grove/`. Walkthroughs should not duplicate the methodology — they show *how the CLI drives it*.
- This is grove's own repo (`Linkuistics/grove`). The walkthroughs are likely best demonstrated against a *different* example repo to avoid the meta-confusion of grove installing itself.

## Done when
- Decisions are made for each of the doc-structure questions below (see "Grilling agenda"), recorded inline in `CONTEXT.md` where new terms arise, and in an ADR if any decision is hard to reverse + surprising + a real trade-off.
- A child node has been grown under `.grove/020-plan-workflow-docs/` containing a `BRIEF.md` and ordered leaves for the actual writing — one leaf per flow, or one leaf for the lot, per the decomposition decision below.
- *Optionally* a PRD if the team needs to align on the walkthrough shape before writing begins.

## Grilling agenda
The session running this leaf must grill the user on at least:

1. **Home of the docs.** New top-level `docs/workflows.md`? Expansion of `docs/grove.md`? Per-flow files under `docs/workflows/`? Section in `README.md`?
2. **One doc or many.** Single sweeping lifecycle walkthrough that flows from install → finish, vs. per-verb walkthroughs that can be read independently. Affects whether `020` decomposes into 1 leaf or 5.
3. **Style.** Narrative prose with fenced command blocks; or step-table (column-per-step: command / what happens / what changed); or shell transcript style; or a hybrid.
4. **Demo target.** Should walkthroughs use a *throwaway example repo* (clearer separation) or grove's own repo (no setup overhead but meta-confusing)?
5. **What-changed evidence.** Is `git status` / `tree` output good enough, or do walkthroughs need before/after diagrams? Affects authoring effort substantially.
6. **Harness coverage.** Show only Claude Code, or both Claude Code and Codex, or harness-neutral with notes on differences?
7. **Verification.** Should the walkthroughs be *executable* (e.g., a script that runs the steps end-to-end against a temp repo and snapshots `git status`)? Trades initial effort for long-term drift protection.

## Notes
- Don't over-pre-decide. Run the grill, capture decisions inline, and grow the tree only as deeply as needed to make the writing tractable. If item 2 lands on "one big walkthrough", a single child leaf is enough.
- If item 7 lands on "executable walkthroughs", a sibling leaf for the harness/test scaffolding is probably needed before the writing leaves.
