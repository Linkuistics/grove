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
