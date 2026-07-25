# Lifecycle walkthroughs

End-to-end, per-verb walks through grove's main flows: each shows the command(s) for one flow, explains what happens, and shows what changes in the repo or worktree at each step. These walkthroughs are **not** the CLI reference (that's [`../../README.md`](../../README.md) — the flag surface) and **not** the methodology (that's [`../../content/SKILL.md`](../../content/SKILL.md) at runtime, or [`../grove.md`](../grove.md) for the project-level introduction). They sit between the two: how the CLI drives the methodology, in order, with worked examples.

## The three flows

1. [**`start.md`**](start.md) — open a new grove, `grove do`'s new-grove path: create your own working tree, run argument-less `grove do` inside it, the harness exec, and the bootstrap session that writes the root `BRIEF.md`.
2. [**`multi-step.md`**](multi-step.md) — the inner loop across several sessions: the single-`grove do` self-driving loop, leaf-vs-node retirement, and the manual `grove retire <node-path>` verb.
3. [**`finish.md`**](finish.md) — close out a completed grove: the **in-session** complete finish cycle (there is no `grove finish` verb) — triggered by an empty `grove-llm pick`, gated by one confirmation, it promotes durable content out of `.grove/` and deletes the scaffolding in a focused commit. Integrating the branch and tearing down the working tree are the user's own git/gh or jj from there, not part of the cycle.

There is no install walkthrough: `brew install grove` is the whole installation, and the binary provisions its embedded methodology to the global skill dir on the first `grove do` (see [`../../README.md`](../../README.md)).

## Running example

Every walkthrough drives the same fictional repo, `acme/orders-api`, and `start.md` through `finish.md` follow one fictional grove, `add-rate-limiting`, end to end. Pinning the demo target here means readers can move between walkthroughs without re-orienting.

Terms used across these pages are defined in [`../../CONTEXT.md`](../../CONTEXT.md) — grove's glossary. (The repo carries a second one for the skill plugins; [`../../CONTEXT-MAP.md`](../../CONTEXT-MAP.md) routes between them. Nothing on these pages needs it.)

## What's not here

`grove retire` *is* walked through, but as a subsection of [`multi-step.md`](multi-step.md), because it lives inside the inner-loop flow rather than constituting a flow of its own. For the flag surface of every verb, see [`../../README.md`](../../README.md) and `grove --help`.

## House style (for editors and new walkthrough authors)

The four pages share a deliberate shape so they read consistently:

- **Hybrid prose + fenced blocks.** Short paragraph of intent → fenced command → *optionally* a small "what changed" block. The panel earns its place only where the change is the point of the step.
- **Evidence is mixed.** `tree -L N <path>` for tree shape, `git status` for clean/dirty state, `git log --oneline -N` for commit history. No mermaid, no before/after ASCII trees.
- **Claude Code primary; Codex as a short callout at the foot of each page.** Never a parallel rewrite of the page.
- **Verification is prose-only** for now. If output drift across CLI releases becomes a real problem, add a sibling snapshot-test scaffold — but do not pre-build one.
