# 040-multi-step-grove

**Kind:** work

## Goal
Write `docs/workflows/multi-step.md` — the lifecycle walkthrough for the **inner loop** of a grove (`grove continue` driving session-after-session). The running example is `add-rate-limiting` in `acme/orders-api`, picking up from where the `start` walkthrough left off.

## Context
- This is the densest walkthrough — it covers the *shape* of the loop (pick → bootstrap → execute → commit → retire) across several sessions in a row, not just one command.
- `content/SKILL.md` is authoritative for the methodology; this walkthrough must not restate the seven constraints or re-explain DDD. It demonstrates how a sequence of `grove continue` invocations and one `grove retire` call cause the task tree to grow and contract.
- Use a worked sequence of ~3–4 sessions in the example grove. Each session needs: the command (`grove continue add-rate-limiting`), a one-sentence narrative of what that session did (e.g., "planning session split `010-design-limit-store.md` into a node of three child leaves"), and a "what changed" panel showing the `.grove/` subtree before and after that session via two `tree .grove/` snippets and the corresponding `git log --oneline` line.

## Done when
- `docs/workflows/multi-step.md` exists.
- Walks a realistic ~3–4 session sequence:
  1. Resume on a work leaf — produces code + one focused commit.
  2. Resume on a planning leaf — grows the tree (subtree appears under one of the leaves).
  3. Resume on the new subtree's first leaf — produces work + commit.
  4. Last live leaf of a subtree completes — `grove retire add-rate-limiting/<node-path>` moves it into `.grove/done/`, leaving the parent brief promoted.
- Shows `.grove/` evolving across the sequence: include at least two `tree .grove/` snapshots (initial vs. mid-loop) and the final `tree .grove/done/` after retirement.
- Briefly names `grove takeover <name>` as the orientation move for picking up an unfamiliar grove without picking a task.
- Short Codex-equivalent callout where harness-specific commands differ.
- No prescription about *what* the sessions write — that's the methodology's job. The walkthrough's job is the CLI cadence and the on-disk evolution.

## Notes
- It is fine — and probably clearer — to *narrate* the example sessions ("session 2 was a planning task that decomposed …") rather than transcribing imagined prompts and outputs. The point of this walkthrough is the *tree-shape evolution over time*, not session-by-session UX.
- The retire example must show that `grove retire` is *manual* (the loop judges retirement but the user issues the verb). Don't imply automatic retirement.
- If the worked example would mislead readers into thinking "every grove needs a planning leaf at step 2," explicitly call out that the sequence is one example shape, not a template.
