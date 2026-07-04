# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Global skill provisioning** / **skill precedence**:
The `grove` binary embeds `content/` and extracts it to the **personal** global skill dir `~/.claude/skills/grove/` on every `grove do` — idempotent against a content-hash stamp (task-tree-scheme). `content/` stays canonical; the binary is the only writer of the global skill, and `launch::load_prompt` reads the launcher prompts from this same dir (no repo-local mirror). Claude Code skill precedence is **enterprise > personal > project** (Claude Code docs): a **personal** skill *overrides* a same-named **project** skill (`<repo>/.claude/skills/grove/`), so the binary-provisioned global copy always wins. A leftover project mirror is therefore **dead, shadowed code** — not an active override — and is removed (070/050) so it cannot mislead a contributor into editing it; no new project mirror should be re-added (edit `content/` and rebuild instead). This is now the only distribution model: the old fetch+materialise machinery and its `cli`/`repo`/`worktree` `VERSION.md` drift were deleted in 090, along with `grove install`/`uninstall`/`status`.

**Complete finish cycle**:
The terminal, whole-grove sequence that retires a grove once it has no live leaves left: (1) promote durable artifacts from the briefs (ADRs, docs, glossary); (2) delete `.grove/` in a focused commit; (3) merge the branch into the default branch (`git -C <repo> merge <name>` — fast-forwards or makes a merge commit); (4) remove the worktree; (5) delete the branch. Steps 3, 4 and 5 run against the main repo with `git -C <repo>`; worktree-remove precedes branch-delete (git refuses `branch -d` on a branch checked out in a live worktree). Driven by the in-session LLM, not Rust automation (in-session-finish-cycle); proposed and executed only on explicit human confirmation, so a headless run with no human reports the plan and stops; resumed from git/filesystem state with no progress marker (constraint 1). Triggered whenever `grove-llm pick` reports no live leaves — or errors because `.grove/` is already gone, the partial-finish resume case. Distinct from the per-leaf Retire step (which marks one leaf done in place with a `DONE` infix); the finish cycle is what retiring the *last* leaf leads into. The former `grove finish` verb that launched this was removed (do-is-sole-lifecycle-verb): finishing is a step of the loop, not a launched verb.

**root-init** / **fresh-grove start**:
The bootstrap of a brand-new grove (worktree + branch exist, but no `.grove/` tree yet), enacted by the `grove-llm root-init [<slug>]` verb. It creates `.grove/`, the root `BRIEF.md` stub, and a first **planning** leaf `01-<slug>-k1.md` (default slug `plan`) — a working-tree change with no commit, refusing to clobber an existing `.grove/`. It is the one tree verb that sits *below* the floor the others stand on (`leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` all require `.grove/` to already exist), so it is what makes a fresh grove enter the steady-state loop. Creating the first leaf — not just the root brief — is load-bearing: `grove-llm pick` skips every `BRIEF.md`, so a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the [[Complete finish cycle]] (fresh-grove-start-contract). The first session's commit folds the scaffold in. Distinct from [[Bootstrap]]-the-loop-step (reading context at the start of every session); this is the one-time creation of the tree that the loop then reads.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

### Task-tree scheme (v2 directories, task-tree-scheme)

**Node directory** / **node**:
A grove tree node is a **directory** named `NN-<slug>-k<key>/` holding a `BRIEF.md` charter plus its numbered children (leaf files and child node directories); `.grove/` is itself the root node (its charter is `.grove/BRIEF.md`). The filesystem carries the hierarchy, so a name encodes only its *per-level* position — not a global path (task-tree-scheme).
_Avoid_: calling a node a "file" — a node is always a directory.

**Leaf**:
A single unit of work — a file `NN-[DONE-]<slug>-k<key>.md` inside a node directory, executed in one session. The only thing `pick` returns is a *live* leaf (one with no `DONE` infix).

**Position** (`NN`):
The **mutable** 2-digit zero-padded per-level locator of an entry among its directory's siblings — the sort input within one directory (lexical == numeric == DFS), rewritten on insert/reorder. It is a locator, **not** an identity.
_Avoid_: using a position (or a directory path) as a durable cross-reference — it moves under renumber. Reference by the [[Permanent key]] or [[Work-item handle]] instead.

**Permanent key** / **stable id** (`-k<key>`):
The never-rewritten identity token of a leaf or node, always the **terminal** token before the extension/slash, assigned once as `max key in tree + 1` (the keys in the names *are* the counter — no counter file; `DONE` leaves stay in the tree so the max is always visible). `grove-llm resolve [n]` / `n` finds an entity's current path by key across any renumber, move, or slug edit.
_Avoid_: "position" as identity; reusing a retired key.

**Work-item handle** / **title** (`<slug>-k<key>`):
The position-free in-file `# …` header of a task or brief (`# <slug>-k<key>`, or `# <slug>-k<key> — brief` for a node; the root brief is `# <grove name> — brief`) **and** the canonical way to name a work item in commit messages and prose (task-tree-scheme §5). Stable across renumber, because it omits the mutable position. `resolve` also accepts the full handle, not just the bare key.
_Avoid_: naming a work item by its position or directory path in a commit message.

**DONE infix**:
The in-place retirement marker: the literal `DONE` placed right after the position in a retired leaf's filename (`NN-DONE-<slug>-k<key>.md`), at a fixed column. Leaves only — a node is never marked done (its done-ness is the absence of a live leaf in its subtree). The leaf keeps its position and key, and its file contents are untouched.
_Avoid_: moving a retired leaf into a separate folder or list — retirement is in place, so the tree always shows complete state.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `do`, `migrate`, `retire`, etc.
2. The **methodology** embedded in `content/SKILL.md` and provisioned to the global skill dir.
3. A single **workstream** — one named task tree under `.grove-worktrees/<name>/.grove/`. `grove do` operates on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
