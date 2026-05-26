# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Install scope**:
The bounded set of paths a single `grove install` or `grove update` invocation modifies — the union of `harness.install_path(repo)` over the harnesses targeted in that invocation. grove's auto-commit stages and commits exactly this set, leaving unrelated dirty state alone.

**Path-scoped commit**:
A commit constructed via explicit `git add <paths>` and `git commit -- <paths>` rather than via whatever happens to be in the index. The technique grove's `install`/`update` use so the auto-commit cannot sweep up unrelated work-in-progress.

**Lifecycle walkthrough**:
A per-verb prose document under `docs/workflows/` that shows the command(s) for one grove flow (install, update, start, multi-step, finish), explains what happens, and shows what changed in the repo or worktree at each step. Distinct from the CLI reference in `README.md` (which lists flags) and from the methodology doc `docs/grove.md` (which is "what grove is and why"). Avoid the aliases *tutorial*, *guide*, and *runbook* in this codebase.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `install`, `update`, `start`, `continue`, etc.
2. The **methodology** bundled in `content/SKILL.md` and materialised into harnesses.
3. A single **workstream** — one named task tree under `.grove-worktrees/<name>/.grove/`. `grove start` / `grove continue` operate on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
