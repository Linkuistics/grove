# grove

grove is a skill for driving long, multi-session workstreams as a git-tracked tree of task files — one task per session, where planning tasks grow the tree as understanding deepens and completed branches retire to an archive. It builds on two established ideas, not de-novo invention: Matt Pocock's [`grill-with-docs`](https://github.com/mattpocock/skills) — whose grilling procedure and `CONTEXT.md` / `ADR-FORMAT.md` conventions grove bundles wholesale — and Domain-Driven Design's **Ubiquitous Language** and **bounded contexts**. This document covers why it exists, how it works, how to install and update it, and a set of starting-point prompts.

## The problem grove solves

Software work that spans many sessions and many months does not arrive with its full shape known. Some early steps are themselves planning steps — their output is not code but more steps, and you cannot enumerate them until you have done the planning. A monolithic implementation plan suits work whose scope is settled; it does not suit a project that grows as you walk it. Once you commit to an exhaustive upfront decomposition, every unexpected discovery either breaks the plan or gets swept under it.

Each Claude session starts fresh — no memory of prior sessions. In a long workstream this is the acute failure mode: session 1 coins a term; session 7, with no memory of session 1, reinvents it under a different name, or reuses the words with a subtly shifted meaning. The glossary becomes incoherent without anyone noticing. Decisions made weeks ago are silently relitigated. The design fractures across sessions because no single session sees the whole.

Earlier attempts to solve this failed in characteristic ways. Phase-machinery approaches — a state machine tracking the workstream through named phases — became brittle: any corrupted phase file could block work, the machinery accrued special cases, and eventually the overhead of managing the process exceeded the cost of the work. GitHub-issue tree approaches were the natural next thing to consider, but they have their own problems: the task tree is decoupled from git history, and a session can't orient itself without running external tools (API calls, issue trackers) — violating the principle that bootstrap should be markdown-only.

grove is the alternative that avoids both traps.

## How grove solves it

A grove is one workstream as a **git-tracked tree of task files** at `groves/<name>/`. Nodes are directories; leaves are `.md` task files with numeric prefixes. The tree's shape — what `ls` shows — is the only state. Git is the history. No phase file, no session log, no status tracker.

One task = one session = one focused commit. Planning tasks, which may grow the tree rather than produce code, are first-class — not an awkward edge case but a named kind with a defined procedure.

Task files are one of two kinds: **work** (produces code, docs, or tests) or **planning** (grills the design using `grilling.md` — bundled from Matt's `grill-with-docs` — sharpens vocabulary, may raise an ADR, and grows the tree by replacing a leaf with a node of child briefs and ordered leaves). A task too big for one focused session *is* a planning task — its job is to decompose, not to do.

The **Ubiquitous Language** — DDD's term for the project's shared domain vocabulary — lives in `CONTEXT.md` at the repo root: a terse glossary of domain terms, aliases-to-avoid, and nothing else. It is read at the start of every session and appended *inline* whenever a term is resolved during a session. This is the forcing function against terminology drift: the glossary is always live, always current, and always the first thing a session reads.

When a project splits into multiple **bounded contexts** — DDD's term for distinct domain partitions, each with its own vocabulary — each gets its own `CONTEXT.md`, linked by a root `CONTEXT-MAP.md`. A bounded context (a *domain* partition) is orthogonal to a task-tree node (a *process* partition): the glossary is per-bounded-context; a node carries a `BRIEF.md`, not a glossary. The two axes don't compete.

Artifacts are **lazy and optional**. An ADR is raised only when a decision is hard to reverse, surprising, or a real trade-off — not because a step demands one. A PRD is written only at a genuine human-facing agreement point. A brief is created only when a node is needed. Nothing is produced speculatively.

Bootstrap is **read-only**: a session reads the glossary, the ADRs cited by the briefs, the `BRIEF.md` chain from root to the current leaf, and the task file itself. No script must succeed before work begins. Delete the skill and `groves/` is still a legible folder of notes.

grove operates under **seven constraints** — the non-negotiable rules that keep it from becoming brittle machinery. They are not restated here; see [`plugins/linkuistics/skills/grove/SKILL.md`](../plugins/linkuistics/skills/grove/SKILL.md) for the list and their rationale (or `.claude/skills/grove/SKILL.md` in a materialised project).

The loop in one sentence: pick the first live leaf depth-first → bootstrap by reading → execute → commit → retire the node if its last leaf is done, then repeat. [`SKILL.md`](../plugins/linkuistics/skills/grove/SKILL.md) has the full mermaid diagram.

## Installing grove in a project, and when and how to update it

### grove is consumed by materialisation, not plugin installation

Installing the `linkuistics` plugin gives you all the coding-style skills globally, but grove is different. A project with many concurrent, long-lived workstreams needs each to be reproducible across its many sessions — and different projects need to pin different grove versions independently. A globally-installed plugin is one version per machine; it cannot satisfy that. So grove is consumed by **materialisation**: a plain copy of the grove files committed into the target repo's own git history.

Per-project version pinning, offline reproducibility, and no runtime dependency on an external plugin — these are the reasons. The pin is the project's own git history; `VERSION.md` documents the upstream correspondence; `git log .claude/skills/grove/` is the update history.

### The command

From a clone of `Linkuistics/skills` (at `main` or a ref you want to pin):

```
scripts/materialise-grove.sh <path-to-target-repo> [<ref>]
```

### What it does

The script copies `grove/` into the target repo's `.claude/skills/grove/` and writes a `VERSION.md` provenance stamp recording the source SHA (`Linkuistics/skills@<sha>`), the bundled `mattpocock/skills` SHA, the date, and the one command to update it.

The materialised footprint is entirely small markdown files:

- `SKILL.md`
- `BRIEF-FORMAT.md`
- `TASK-FORMAT.md`
- `CONTEXT-FORMAT.md`
- `ADR-FORMAT.md`
- `grilling.md`
- `LICENSES/mattpocock-skills.LICENSE`
- `VERSION.md`

### Commit the footprint

Commit the materialised `.claude/skills/grove/` as part of the target repo — this commit IS the pin. Claude Code auto-discovers project-local skills at higher precedence than plugins, so a materialised grove works automatically as a project skill.

### Updating

Run the same command again at a new ref. The diff is plain files — review and commit. By discipline, record the version bump in an ADR (`docs/adr/`) so the update decision is traceable.

### Git worktrees — one per grove

All sessions of a single grove run in the **same git worktree**. The grove's state lives in the working tree (the task-tree shape — what `ls` shows) plus git history; a single, continuous worktree is what makes that visible session-to-session. Don't create a new worktree per task or per session.

Git worktrees come into play for a different reason: **running different groves in parallel** in the same repo. Worktrees of a repo all share the same committed `.claude/skills/grove/` — correct: it's the same project at the same methodology version. So a worktree per concurrent grove gives parallel isolation without methodology divergence; just don't fragment a *single* grove across worktrees.

### One-off and exploratory use

If you do not need a pin — a single short workstream, an experiment — install the `linkuistics` plugin in Claude Code and use `linkuistics:grove` directly. Latest, global, unpinned. Fine for a one-off; not the right choice for serious long-lived projects.

## How to use it

The prompts below are starting points, not rigid scripts. grove is designed around constraint 5: it guides, it does not gate. You can adapt any prompt, skip a step, or do a task by hand. These cover the common scenarios in rough order from first-time setup to ongoing use.

### Start a new grove

The grove doesn't exist yet — you want to bootstrap it with a root brief and an initial decomposition.

```
I want to start a new grove for <workstream-name> in this repo. Goal: <one-sentence goal>. Grill me on it first (use grove's bundled grilling procedure), sharpening any new terminology into CONTEXT.md inline as terms resolve. Then propose the root groves/<workstream-name>/BRIEF.md (per BRIEF-FORMAT.md) and a small initial decomposition — first one or two leaves only. Don't over-plan; planning tasks will grow the rest.
```

### Continue a grove

The grove exists with at least one live leaf — you want the next session's worth of work done.

```
Do the next task in groves/<name>/. Follow grove's loop: pick → bootstrap → execute → commit → judge retirement.
```

### Run a planning task explicitly

You already know the next leaf is a planning task and you want to make that explicit — planning tasks may grow the tree rather than produce code, so setting expectations about scope matters.

```
The next leaf in groves/<name>/ is a planning task. Grill the design, update CONTEXT.md inline as terms resolve, raise an ADR only if the decision meets all three of ADR-FORMAT.md's criteria (hard to reverse, surprising, a real trade-off), and grow the tree if the work is bigger than one focused session.
```

### Run a work task explicitly

You know the next leaf is a clean work task — just produce the artifact.

```
The next leaf in groves/<name>/ is a work task. Execute it as one focused session: produce the artifact, write tests if applicable, make one focused commit. Don't grow the tree.
```

### Materialise grove in a project (or update it)

Two steps — a script invocation you run yourself, then a Claude session in the target repo to handle the diff. By **discipline**, grove records every version bump as an ADR even though a routine bump doesn't otherwise meet `ADR-FORMAT.md`'s criteria — the ADR log is the only durable record of *when* the project's methodology version moved (`VERSION.md` only carries the current version, not the history).

First, from a `Linkuistics/skills` clone (current `main`, or checked out at a ref you want to pin):

```
scripts/materialise-grove.sh <path-to-target-repo> [<ref>]
```

Then, in a Claude session started in the target repo:

```
Review the diff in .claude/skills/grove/. If this is the first materialisation, branch and commit as feat: materialise grove. If it's an update, commit as chore: bump grove to <ref>. Either way, record the bump as an ADR in docs/adr/ (grove's standing discipline for version changes).
```

### Take over an existing grove

You're picking up someone else's grove — or your own from a long break — and need to orient before acting.

```
Take over the <name> grove. Don't pick a task yet — first orient: read CONTEXT.md, the root BRIEF.md, and skim the most-recently-touched files (git log --since='1 week ago' -- groves/<name>/). Tell me where the grove stands: what's done, what's open, what the next task would be, and any open questions in the briefs.
```

### Retire a node by hand

The last live leaf in a node just completed — you want to archive the subtree explicitly.

```
The groves/<name>/<NNN-node>/ node's last live leaf is done. Promote anything still relevant from its BRIEF.md upward (to the parent brief, an ADR, or the glossary), then mv the node into groves/<name>/done/, preserving its relative path. One focused commit.
```

### Bootstrap a brand-new project (first time using grove)

The project has no `groves/`, no `CONTEXT.md`, no materialised grove yet — wire it all up.

```
Set up grove in this repo from scratch:
1. Materialise grove into .claude/skills/grove/ from a Linkuistics/skills clone at <path>.
2. Create an empty CONTEXT.md at the repo root with a one-line description of the project's domain.
3. Create docs/adr/ as an empty directory (placeholder only — we'll add ADRs lazily).
4. Then start a grove for <first-workstream> using the "start a new grove" flow.
```
