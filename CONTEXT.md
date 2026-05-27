# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Install scope**:
The bounded set of paths a single `grove install` or `grove update` invocation modifies — the union of `harness.install_path(repo)` over the harnesses targeted in that invocation. grove's auto-commit stages and commits exactly this set, leaving unrelated dirty state alone.

**Path-scoped commit**:
A commit constructed via explicit `git add <paths>` and `git commit -- <paths>` rather than via whatever happens to be in the index. The technique grove's `install`/`update` use so the auto-commit cannot sweep up unrelated work-in-progress.

**Lifecycle walkthrough**:
A per-verb prose document under `docs/workflows/` that shows the command(s) for one grove flow (install, update, start, multi-step, finish), explains what happens, and shows what changed in the repo or worktree at each step. Distinct from the CLI reference in `README.md` (which lists flags) and from the methodology doc `docs/grove.md` (which is "what grove is and why"). Avoid the aliases *tutorial*, *guide*, and *runbook* in this codebase.

**Inbox**:
A markdown file accumulating observations addressed to a named grove, living at `inboxes/<name>.md` on the [[grove-inboxes branch]]. Observations are appended as they surface, by the LLM working in any grove that notices something belonging elsewhere. The inbox is addressed to the workstream, not to a person. A running grove [[drain]]s its own inbox at every `grove start` and `grove continue`. An inbox whose addressed grove does not currently exist as a worktree is called a [[seed]].

**Seed**:
An [[inbox]] whose addressed grove does not currently exist as a worktree — whether yet to start or already finished. Same artifact and same path; only the lifecycle state differs. The seed's name is the working title of the addressed grove (e.g. `racket-bugs-discovered-while-implementing-chez`); it may later be renamed, narrowed, or folded into a catchall seed. When `grove start <name>` runs, the seed simply becomes the new running grove's inbox — there is no separate consumption step. A seed is distinct from a brief note (in-scope context for the current node, retired with it), from an ADR (a decision, not an observation), and from a TODO comment in code (local to one file, not a workstream candidate). Avoid the alias *issue* — it collides with GitHub Issues and with bug-tracker connotations.

**grove-inboxes branch**:
A dedicated branch in each repo, materialised as a sibling worktree at `<repo>/.grove-inboxes/`, holding shared grove coordination data. The primary occupants are [[inbox]] files at `inboxes/<name>.md`; the branch is reserved as a home for other cross-grove coordination artifacts that may surface later. All reads and writes go through `grove` CLI subcommands rather than direct git/filesystem operations, so the LLM does not interact with the branch's git plumbing. Cross-repo inbox writes follow the same rule against another repo's `.grove-inboxes/` worktree, and require that worktree to be present locally.

**Drain**:
The session-bootstrap triage of a running grove's [[inbox]], performed at every `grove start` and `grove continue`. For each pending observation the LLM proposes incorporating it into the current task, deferring it to a later leaf, or rejecting it as out-of-scope (and possibly seeding a different grove). After triage the inbox file is cleared; drained observations live in git history.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `install`, `update`, `start`, `continue`, etc.
2. The **methodology** bundled in `content/SKILL.md` and materialised into harnesses.
3. A single **workstream** — one named task tree under `.grove-worktrees/<name>/.grove/`. `grove start` / `grove continue` operate on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
