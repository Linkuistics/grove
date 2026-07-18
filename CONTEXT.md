# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Global skill provisioning** / **skill precedence**:
The `grove` binary embeds `content/` and, on every `grove do`, sweeps it out to **every installed harness's** personal global skill dir — `provision_all` writes the launching harness's dir unconditionally plus every other harness whose home root (`~/.claude`, `~/.codex`, `~/.pi`) already exists, each via `skill_dir_for(harness)` (`$HOME/<harness.skills_dir>/grove`; pi nests under `agent/`) — idempotent per target against a content-hash stamp (task-tree-scheme). `content/` stays canonical; the binary is the only writer of any of these dirs, and `launch::load_prompt(harness, verb)` reads the launcher prompts from the *launching* harness's own copy (no repo-local mirror). `provision_target` guards each write: a symlink (the old cross-harness link-farm layout) is replaced as a link, never deleted through; a dir with no grove stamp and real content is refused outright, so a foreign dir can never be silently clobbered. Claude Code skill precedence is **enterprise > personal > project** (Claude Code docs): a **personal** skill *overrides* a same-named **project** skill (`<repo>/.claude/skills/grove/`), so the binary-provisioned global copy always wins there. A leftover project mirror is therefore **dead, shadowed code** — not an active override — and is removed (070/050) so it cannot mislead a contributor into editing it; no new project mirror should be re-added (edit `content/` and rebuild instead). This is now the only distribution model: the old fetch+materialise machinery and its `cli`/`repo`/`worktree` `VERSION.md` drift were deleted in 090, along with `grove install`/`uninstall`/`status`.

**Complete finish cycle**:
The terminal, whole-grove sequence that retires a grove once it has no live leaves left: (1) promote durable artifacts from the briefs (ADRs, docs, glossary); (2) delete `.grove/` in a focused commit; (3) signal the loop with `grove-llm complete --done`. Nothing after: integrating the branch and tearing down the working tree are **not** grove workflow — both belong to plain git/gh and the user's own worktree tooling (user-owned-worktrees). Driven by the in-session LLM, not Rust automation (in-session-finish-cycle); proposed and executed only on explicit human confirmation, so a headless run with no human reports the plan and stops; resumed from git/filesystem state with no progress marker (constraint 1). Triggered whenever `grove-llm pick` reports no live leaves — or errors because `.grove/` is already gone, the partial-finish resume case. Distinct from the per-leaf Retire step (which marks one leaf done in place with a `DONE` infix); the finish cycle is what retiring the *last* leaf leads into. The former `grove finish` verb that launched this was removed (do-is-sole-lifecycle-verb): finishing is a step of the loop, not a launched verb.
_Avoid_: describing the finish as merging or deleting anything git-topological — that was the pre-v11 cycle.

**Grove name**:
The working-tree directory's basename — derived from `git rev-parse --show-toplevel`, never from a branch or a canonical layout. Names the root brief (`# <name> — brief`), the harness session (`<repo-basename>: <name> grove`), and the harness stamp (`<repo>/.grove-stamps/<name>`).
_Avoid_: "the grove name equals the branch name" — grove reads no branch anywhere.

**root-init** / **fresh-grove start**:
The bootstrap of a brand-new grove (a git working tree exists — user-provided, any branch, anywhere — but no `.grove/` tree yet), enacted by the `grove-llm root-init [<slug>]` verb. It creates `.grove/`, the root `BRIEF.md` stub, and a first **planning** leaf `01-<slug>-k1.md` (default slug `plan`) — a working-tree change with no commit, refusing to clobber an existing `.grove/`. It is the one tree verb that sits *below* the floor the others stand on (`leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` all require `.grove/` to already exist), so it is what makes a fresh grove enter the steady-state loop. Creating the first leaf — not just the root brief — is load-bearing: `grove-llm pick` skips every `BRIEF.md`, so a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the [[Complete finish cycle]] (fresh-grove-start-contract). The first session's commit folds the scaffold in. Distinct from [[Bootstrap]]-the-loop-step (reading context at the start of every session); this is the one-time creation of the tree that the loop then reads.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

**Task kind** (`**Kind:**`):
The one-word session-shape declaration on a [[Leaf]]'s task file, drawn from a
closed set of five: **planning** (grills, grows the tree), **research** (produces
`docs/research/<slug>.md`), **prototype** (a cheap throwaway artifact to react to),
**work** (produces code, docs, tests), **review** (fresh-context adversarial read;
produces findings). `planning` is the only kind with methodological force — it is
the sole branch in the loop's Execute step, and the only kind that grows the tree;
the other four are work-shaped sessions that differ in discipline and in
[[Per-kind model selection]]. grove **gates on write** (a grow verb rejects an
unknown `--kind`, where a human is present to fix it) and **degrades on read** (an
unrecognised `**Kind:**` line warns and is treated as `work`, so a hand-edited leaf
can never jam the loop — constraint 5). `leaf-decompose` gives the first child its
parent leaf's kind, so a research leaf that proves bigger becomes a research node.
See ADR *task-kind-taxonomy*.
_Avoid_: adding a kind that carries no behaviour beyond a name — a kind must earn
its place with a distinct discipline, and a sixth is a change to this closed set,
not a free-text label.

**HITL** / **AFK**:
Whether a [[Task kind]] resolves through live exchange with a human who speaks for
themselves (`planning`, `prototype`) or is driven by the agent alone (`research`,
`work`, `review`). A HITL leaf reached by an unattended relaunch of the self-driving
loop stalls until a human arrives; that is correct, not a fault.
_Avoid_: an agent answering its own HITL questions — a `planning` session that
grills itself has broken the distinction (`grilling.md`).

**Per-kind model selection** (`GROVE_<KIND>_MODEL`):
The self-driving loop launches each task's `claude` session on a model chosen by
the picked [[Leaf]]'s [[Task kind]] — one env var per kind
(`GROVE_PLANNING_MODEL`, `GROVE_RESEARCH_MODEL`, `GROVE_PROTOTYPE_MODEL`,
`GROVE_WORK_MODEL`, `GROVE_REVIEW_MODEL`) — via Claude Code's native `--model`
flag (no router, no proxy). The driver peeks the kind (`grove-llm kind`), reads
that kind's var, and passes `--model` **only if it is set**. There is **no
fallback chain**: an unset var means no flag at all, so the session inherits the
user's own default rather than a model grove picked for a kind the user never
configured. The launched value is a *default*, not a lock — in-session `/model`
outranks `--model` for that session. See ADR *model-per-task-kind*.
_Avoid_: calling this "model routing" — that implies a multi-provider proxy;
this is single-provider launch-flag selection on the same (e.g. Max) subscription.
_Avoid_: "an in-session `/model` never survives relaunch" — unqualified, that is
false. Interactive `/model` saves as the user's default, so it survives into the
next session of any kind whose var is **unset** (grove passes no `--model`, and
the saved default governs); a configured kind's `--model` overrides it.

**Spec** (`docs/specs/<slug>.md`):
The human-facing, team-shareable design of an *area* of the system — problem,
solution, settled decisions, agreed test seams, out-of-scope — written lazily by a
planning task at a genuine agreement point (grill → spec → decompose → execute).
Slug-named, never dated or numbered. Like `docs/adr/`, `docs/specs/` is a **minimum
coherent set describing the design's current state**: edited, merged and split in
place, and deleted once a spec describes nothing (constraint 1 — git holds the
past). Two rules bound the set. The **membership test**: *would a session on an
unrelated future grove need to read this?* If not it is a [[Node directory]]'s
`BRIEF.md`, and it dies with `.grove/` — work-orders and disposition tables are
briefs. The **grain rule**: an ADR records *one decision and its trade-off*, a spec
describes *how an area works*, and a spec **cites** the ADRs in its area rather than
restating them (restate one and the two sets will disagree, after which neither
binds). Shape and the seam-recording rule: `content/SPEC-FORMAT.md`; what a seam
*is*: `linkuistics:codebase-design`.
_Avoid_: "PRD" — grove names no product-requirements artifact. _Avoid_ a
`## Decomposition` section inside a spec: that is brief material, dead when the
grove finishes.

### Task-tree scheme (v2 directories, task-tree-scheme)

**Node directory** / **node**:
A grove tree node is a **directory** named `NN-<slug>-k<key>/` holding a `BRIEF.md` charter plus its numbered children (leaf files and child node directories); `.grove/` is itself the root node (its charter is `.grove/BRIEF.md`). The filesystem carries the hierarchy, so a name encodes only its *per-level* position — not a global path (task-tree-scheme).
_Avoid_: calling a node a "file" — a node is always a directory.

**Leaf**:
A single unit of work — a file `NN-[DONE-|ABANDONED-]<slug>-k<key>.md` inside a node directory, executed in one session. The only thing `pick` returns is a *live* leaf — one carrying **no outcome infix** at all. A leaf has exactly two terminal states: `DONE` (the work was done) and `ABANDONED` (the path was closed); see [[DONE infix]] and [[Pruning]].

**Position** (`NN`):
The **mutable** 2-digit zero-padded per-level locator of an entry among its directory's siblings — the sort input within one directory (lexical == numeric == DFS), rewritten on insert/reorder. It is a locator, **not** an identity.
_Avoid_: using a position (or a directory path) as a durable cross-reference — it moves under renumber. Reference by the [[Permanent key]] or [[Work-item handle]] instead.

**Permanent key** / **stable id** (`-k<key>`):
The never-rewritten identity token of a leaf or node, always the **terminal** token before the extension/slash, assigned once as `max key in tree + 1` (the keys in the names *are* the counter — no counter file; **every finished leaf stays in the tree, `DONE` or `ABANDONED` alike**, so the max is always visible). `grove-llm resolve [n]` / `n` finds an entity's current path by key across any renumber, move, or slug edit.
_Avoid_: "position" as identity; reusing a retired key.
_Avoid_: `git rm`-ing a leaf to abandon it — that lowers the max and the next `leaf-add` re-issues a live key. Use [[Pruning]] (`leaf-prune`); the mark is what keeps the counter monotonic.

**Work-item handle** / **title** (`<slug>-k<key>`):
The position-free in-file `# …` header of a task or brief (`# <slug>-k<key>`, or `# <slug>-k<key> — brief` for a node; the root brief is `# <grove name> — brief`) **and** the canonical way to name a work item in commit messages and prose (task-tree-scheme §5). Stable across renumber, because it omits the mutable position. `resolve` also accepts the full handle, not just the bare key.
_Avoid_: naming a work item by its position or directory path in a commit message.

**DONE infix**:
The in-place retirement marker: the literal `DONE` placed right after the position in a retired leaf's filename (`NN-DONE-<slug>-k<key>.md`), at a fixed column. Written by `leaf-retire`. Leaves only — a node is never marked done (its done-ness is the absence of a live leaf in its subtree, however those leaves finished). The leaf keeps its position and key, and its file contents are untouched. Its sibling mark is `ABANDONED` ([[Pruning]]); the two are the only terminal leaf states.
_Avoid_: moving a retired leaf into a separate folder or list — retirement is in place, so the tree always shows complete state.

**Pruning** / **ABANDONED infix** (`leaf-prune`):
Marking a work path **decided against** — as opposed to done. `grove-llm leaf-prune`
writes an `ABANDONED` infix in place (`NN-ABANDONED-<slug>-k<key>.md`), exactly as
`leaf-retire` writes `DONE`; `pick` skips both, and neither ever leaves the tree.
grove's metaphor names the pair: a `DONE` leaf is **harvested**, an abandoned one is
**pruned**. Deciding against a path is a normal outcome of exploration, not an
exception — but it is **[[HITL]]**: an agent never prunes on its own, and an AFK
session that finds a leaf dead says so and stops.
Given a **node**, `leaf-prune` marks every *live* leaf in that subtree (leaving
`DONE` ones alone) and refuses the grove root. The arity asymmetry with `leaf-retire`
is deliberate: retirement is *incremental* (one leaf per session, as work completes),
abandonment is *bulk by nature* (one decision kills N leaves at once).
Because `.grove/` dies at the [[Complete finish cycle]], the mark records only *that*
a path was closed; the durable *why* goes to the **ADR set** — the positive fact the
abandonment establishes ("we rejected cross-family review" *is* "grove is
single-provider"), with the rejection as a *Considered options* entry stating what was
rejected, why, and **what would reopen it**. Too small to clear the ADR when-to-write
bar? Then nothing durable is written; the mark and the commit message suffice. See ADR
*pruning*.
_Avoid_: reading "pruned" in git's sense (`git remote prune` = *delete*) — a pruned
leaf **stays in the tree**; that is the entire point, since the keys in the names are
the counter ([[Permanent key]]).
_Avoid_: a taxonomy of outcomes (`blocked` / `deferred` / `superseded`). `blocked` is
expressed by **ordering** and would break the finish trigger if `pick` skipped it (a
blocked leaf is *live* work); `deferred` is a reorder or a GitHub issue; `superseded`
differs only in *reason*, which is prose and belongs in the ADR, not the filename.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `do`, `migrate`, `retire`, etc.
2. The **methodology** embedded in `content/SKILL.md` and provisioned to the global skill dir.
3. A single **workstream** — one named task tree at `.grove/` inside the working tree the user provides, named for that working tree's basename ([[Grove name]]), no canonical path. `grove do` operates on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
