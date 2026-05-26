# Lifecycle walkthroughs

End-to-end, per-verb walks through grove's main flows: each shows the command(s) for one flow, explains what happens, and shows what changes in the repo or worktree at each step. These walkthroughs are **not** the CLI reference (that's [`../../README.md`](../../README.md) — the flag surface) and **not** the methodology (that's [`../../content/SKILL.md`](../../content/SKILL.md) at runtime, or [`../grove.md`](../grove.md) for the project-level introduction). They sit between the two: how the CLI drives the methodology, in order, with worked examples.

## The five flows

1. [**`install.md`**](install.md) — materialise grove into a repo for the first time. Covers the default auto-commit, `--no-commit`, `--message`, and the refuse-on-pre-existing-staged-changes case.
2. [**`update.md`**](update.md) — refresh an existing materialisation, including the ADR-bump nudge and the no-op-update behaviour.
3. [**`start.md`**](start.md) — open a new grove: worktree creation, branching, the harness exec, and the bootstrap session that writes the root `BRIEF.md`.
4. [**`multi-step.md`**](multi-step.md) — the inner loop across several sessions: `grove continue` cadence, leaf-vs-node retirement, the manual `grove retire <name>/<node-path>` verb, and `grove takeover` for orientation.
5. [**`finish.md`**](finish.md) — close out a completed grove: promote durable content out of `.grove/`, delete the scaffolding, merge per project convention, clean up worktree and branch.

## Running example

Every walkthrough drives the same fictional repo, `acme/orders-api`, and `start.md` through `finish.md` follow one fictional grove, `add-rate-limiting`, end to end. Pinning the demo target here means readers can move between walkthroughs without re-orienting.

Terms used across these pages — *install scope*, *path-scoped commit*, *lifecycle walkthrough* — are defined in [`../../CONTEXT.md`](../../CONTEXT.md).

## What's not here

The walkthroughs cover the five flows above. The remaining CLI verbs — `uninstall`, `status`, `list`, `version` — are documented in [`../../README.md`](../../README.md) and `grove --help`; they don't have lifecycle stories of their own. `grove takeover` and `grove retire` *are* walked through, but as subsections of [`multi-step.md`](multi-step.md), because they live inside the inner-loop flow rather than constituting a flow of their own.

## House style (for editors and new walkthrough authors)

The five pages share a deliberate shape so they read consistently:

- **Hybrid prose + fenced blocks.** Short paragraph of intent → fenced command → *optionally* a small "what changed" block. The panel earns its place only where the change is the point of the step.
- **Evidence is mixed.** `tree -L N <path>` for tree shape, `git status` for clean/dirty state, `git log --oneline -N` for commit history. No mermaid, no before/after ASCII trees.
- **Claude Code primary; Codex as a short callout at the foot of each page.** Never a parallel rewrite of the page.
- **Verification is prose-only** for now. If output drift across CLI releases becomes a real problem, add a sibling snapshot-test scaffold — but do not pre-build one.
