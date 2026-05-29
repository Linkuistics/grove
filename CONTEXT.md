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
A directory accumulating observations addressed to a named grove, living at `inboxes/<name>/` on the [[grove-meta branch]]. Each observation is a single markdown file at `inboxes/<name>/<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`; the directory's existence (preserved by a `.gitkeep`) is the "this grove is known" signal even when no observations are pending. Observations are written by the LLM working in any grove that notices something belonging elsewhere, via `grove-llm inbox-add` (never direct file edits; see ADR-0006 for the LLM-binary split). The inbox is addressed to the workstream, not to a person. A running grove [[drain]]s its own inbox at every `grove start` and `grove continue`. An inbox whose addressed grove does not currently exist as a worktree is called a [[seed]]. The shape moved from "one file per grove" to "one directory per grove, one file per observation" in ADR-0004 to escape the merge-conflict pathology of single-file multi-writer (see `docs/research/in-repo-issue-tracker-postmortems.md`).

**Seed**:
An [[inbox]] whose addressed grove does not currently exist as a worktree — whether yet to start or already finished. Same artifact and same path; only the lifecycle state differs. The seed's name is the working title of the addressed grove (e.g. `racket-bugs-discovered-while-implementing-chez`); it may later be renamed, narrowed, or folded into a catchall seed. When `grove start <name>` runs, the seed simply becomes the new running grove's inbox — there is no separate consumption step. A seed is distinct from a brief note (in-scope context for the current node, retired with it), from an ADR (a decision, not an observation), and from a TODO comment in code (local to one file, not a workstream candidate). Avoid the alias *issue* — it collides with GitHub Issues and with bug-tracker connotations.

**grove-meta branch**:
A dedicated branch in each repo, materialised as a sibling worktree at `<repo>/.grove-meta/`, holding shared cross-grove coordination data and repo-level metadata that does not belong on the default branch. The primary occupants today are [[inbox]] directories at `inboxes/<name>/`; the branch is reserved more broadly as the home for any cross-grove coordination artifact or repo-level metadata (grove-related or otherwise) that future work surfaces. All reads and writes go through `grove` CLI subcommands rather than direct git/filesystem operations, so the LLM does not interact with the branch's git plumbing. Cross-repo inbox writes follow the same rule against another repo's `.grove-meta/` worktree, and require that worktree to be present locally. The branch is created (and the worktree attached) by `grove meta init` — invoked explicitly for repos that pre-date the feature or to repair a missing worktree, and implicitly during `grove install` / `grove update`. Remote-sync is **opt-in**: by default the branch has no upstream and lives local-only. Multi-machine users add a remote with `grove meta remote add <url>` (which sets upstream tracking); thereafter `grove start` / `grove continue` fetch-before-drain and `grove-llm inbox-add` / `grove-llm inbox-drain` push-after-commit, both best-effort with one auto-retry on non-ff (ADR-0005). The previously-considered name `grove-inboxes` was rejected because it narrowed the branch's scope to its first occupant.

**Drain**:
The session-bootstrap triage of a running grove's [[inbox]], performed at every `grove start` and `grove continue`. If the `grove-meta` branch has a remote configured, drain fetches it first (soft-on-failure — warn-and-continue if offline; refuse-and-instruct on non-ff). For each pending observation the LLM proposes incorporating it into the current task, deferring it to a later leaf, or rejecting it as out-of-scope (and possibly seeding a different grove). After triage the triaged observation files are deleted in one session-commit; drained observations live in git history (`git log inboxes/<name>/`). The `.gitkeep` stays so the directory's existence remains the "known grove" signal even when no observations are pending.

**cli version** / **repo version** / **worktree version**:
The three locations at which a grove-methodology version stamp can live, and which any pair can drift between. **cli version** is the installed `grove` binary's own version (`env!("CARGO_PKG_VERSION")`, the methodology bundled inside the binary). **repo version** is the version stamped in `<repo>/.<harness>/skills/grove/VERSION.md` — one per installed harness, written by `grove install` / `grove update`. **worktree version** is the version stamped in `<repo>/.grove-worktrees/<name>/.<harness>/skills/grove/VERSION.md` — one per worktree per harness, written when `grove start` materialises the skill into the new worktree. The three are independent because `grove install`/`update` only touches the repo's install path and `grove start` only touches the new worktree's install path; the binary the user runs `grove status` with is a third axis entirely. Surfacing all three (and drift between them) is the job of `grove status`, `grove list`, `grove version`, and the TUI. **Drift rule:** raw string equality, both stamps shown on mismatch, no semver interpretation. The optional leading `v`/`V` is a git-tag artifact — materialised `VERSION.md` stamps carry it (`v3.0.1`) while the **cli version** from `CARGO_PKG_VERSION` does not (`3.0.1`) — so it is stripped both for comparison (the two name the same release and must not read as drift) and for display (output never shows the `v`). A missing/unreadable worktree `VERSION.md` renders `(unknown)` and is never drift; an orphan worktree (harness no longer installed in the repo) shows `repo=(none)` informationally without a warning. The shared `status::strip_v` / `status::same_version` helpers exist so every surface applies the identical rule.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `install`, `update`, `start`, `continue`, etc.
2. The **methodology** bundled in `content/SKILL.md` and materialised into harnesses.
3. A single **workstream** — one named task tree under `.grove-worktrees/<name>/.grove/`. `grove start` / `grove continue` operate on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
