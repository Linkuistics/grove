# grove

grove is a tool for driving long, multi-session workstreams as a git-tracked tree of task files — one task per session, where planning tasks grow the tree as understanding deepens and completed branches retire to an archive. It builds on two established ideas, not de-novo invention: Matt Pocock's [`grill-with-docs`](https://github.com/mattpocock/skills) — whose grilling procedure and `CONTEXT.md` / `ADR-FORMAT.md` conventions grove bundles wholesale — and Domain-Driven Design's **Ubiquitous Language** and **bounded contexts**. This document covers why it exists, how it works, how to install it, and how the CLI's verbs map onto the methodology.

The methodology itself — the loop, the seven constraints, the BRIEF/CONTEXT/ADR/TASK formats, the grilling procedure — lives in [`content/SKILL.md`](../content/SKILL.md) and its sibling files. This doc is the *project-level* introduction; `SKILL.md` is what an agent reads at runtime.

## The problem grove solves

Software work that spans many sessions and many months does not arrive with its full shape known. Some early steps are themselves planning steps — their output is not code but more steps, and you cannot enumerate them until you have done the planning. A monolithic implementation plan suits work whose scope is settled; it does not suit a project that grows as you walk it. Once you commit to an exhaustive upfront decomposition, every unexpected discovery either breaks the plan or gets swept under it.

Each agent session starts fresh — no memory of prior sessions. In a long workstream this is the acute failure mode: session 1 coins a term; session 7, with no memory of session 1, reinvents it under a different name, or reuses the words with a subtly shifted meaning. The glossary becomes incoherent without anyone noticing. Decisions made weeks ago are silently relitigated. The design fractures across sessions because no single session sees the whole.

Earlier attempts to solve this failed in characteristic ways. Phase-machinery approaches — a state machine tracking the workstream through named phases — became brittle: any corrupted phase file could block work, the machinery accrued special cases, and eventually the overhead of managing the process exceeded the cost of the work. GitHub-issue tree approaches were the natural next thing to consider, but they have their own problems: the task tree is decoupled from git history, and a session can't orient itself without running external tools (API calls, issue trackers) — violating the principle that bootstrap should be markdown-only.

grove is the alternative that avoids both traps.

## How grove solves it

A grove is one workstream as a **git-tracked tree of task files** at `.grove/` (inside the grove's own worktree). Nodes are directories; leaves are `.md` task files with numeric prefixes. The tree's shape — what `ls` shows — is the only state. Git is the history. No phase file, no session log, no status tracker.

One task = one session = one focused commit. Planning tasks, which may grow the tree rather than produce code, are first-class — not an awkward edge case but a named kind with a defined procedure.

Task files are one of two kinds: **work** (produces code, docs, or tests) or **planning** (**opens with a grilling session** — using `grilling.md`, the procedure bundled from Matt's `grill-with-docs` — to interrogate the design one question at a time, then through the grilling sharpens vocabulary, may raise an ADR, and grows the tree by replacing a leaf with a node of child briefs and ordered leaves). A task too big for one focused session *is* a planning task — its job is to decompose, not to do.

The **[Ubiquitous Language](concepts.md#ubiquitous-language)** — DDD's term for the project's shared domain vocabulary — lives in `CONTEXT.md` at the repo root: a terse glossary of domain terms, aliases-to-avoid, and nothing else. It is read at the start of every session and appended *inline* whenever a term is resolved during a session. This is the forcing function against terminology drift: the glossary is always live, always current, and always the first thing a session reads.

When a project splits into multiple **[bounded contexts](concepts.md#bounded-context)** — DDD's term for distinct domain partitions, each with its own vocabulary — each gets its own `CONTEXT.md`, linked by a root [`CONTEXT-MAP.md`](concepts.md#context-map). A bounded context (a *domain* partition) is orthogonal to a task-tree node (a *process* partition): the glossary is per-bounded-context; a node carries a `BRIEF.md`, not a glossary. The two axes don't compete.

Artifacts are **lazy and optional**. An [ADR](concepts.md#adr) is raised only when a decision is hard to reverse, surprising, or a real trade-off — not because a step demands one. A [PRD](concepts.md#prd) is written only at a genuine human-facing agreement point. A brief is created only when a node is needed. Nothing is produced speculatively.

Bootstrap is **read-only**: a session reads the glossary, the ADRs cited by the briefs, the `BRIEF.md` chain from root to the current leaf, and the task file itself. No script must succeed before work begins. Delete the materialised skill and `.grove/` is still a legible folder of notes.

grove operates under **seven constraints** — the non-negotiable rules that keep it from becoming brittle machinery. They are not restated here; see [`content/SKILL.md`](../content/SKILL.md) for the list and their rationale.

The loop in one sentence: pick the first live leaf depth-first → bootstrap by reading → execute → commit → retire the node if its last leaf is done, then repeat. `SKILL.md` has the full mermaid diagram.

## Installing grove

grove ships as a single Rust binary. Install it via Homebrew:

```
brew tap Linkuistics/taps
brew install grove
```

This puts the `grove` command on `$PATH`. It is not yet wired into any project — that's what `grove install` does, below.

### grove is materialised into each repo, not installed globally as a skill

`grove` the CLI is global, but the methodology files it operates on — `SKILL.md`, the format references, the prompt templates — are copied into each consuming repo's harness directory (`.claude/skills/grove/` for Claude Code, `.codex/skills/grove/` for Codex). This is **materialisation**, not installation, and the distinction is deliberate.

A project with many concurrent, long-lived workstreams needs each to be reproducible across its many sessions — and different projects need to pin different grove methodology versions independently. A globally-installed skill is one version per machine; it cannot satisfy that. Per-project materialisation gives offline reproducibility, version pinning to the project's own git history, and no runtime dependency on an external installer. The pin *is* the project's own git history; `git log .claude/skills/grove/` is the update history.

From a project repo:

```
grove install [<repo>]            # create the .<harness>/skills/grove/ tree
grove update  [<repo>]            # refresh an existing materialisation
grove uninstall [<repo>]          # remove it (refuses if live groves exist; --force overrides)
grove version                     # CLI version + the materialised content version per harness
grove status [<repo>]             # installed versions + per-grove summary
grove list [<repo>]               # grove names in the repo, one per line (scriptable)
```

All file-system verbs auto-detect the harness from the repo's `.claude/` and `.codex/` directories. In a multi-harness repo every detected harness is operated on; pass `--harness <name>` (repeatable) to target specific ones. Commit the materialised `.{claude,codex}/skills/grove/` as part of the repo — that commit is the methodology version pin.

### Updating

`grove update` refreshes the materialised files in place. By discipline, record version bumps in an ADR (`docs/adr/`) so the update decision is traceable — `VERSION.md` only carries the current version, not the history.

## Driving a grove

A grove lives in three places — the CLI binary (Homebrew, used from anywhere); the materialised methodology at `<repo>/.<harness>/skills/grove/`, committed as part of the repo and serving as the version pin; and the grove itself, which is a **git worktree** at `<repo>/.grove-worktrees/<name>/` on branch `<name>`. The task tree — the `.grove/` directory of briefs and leaves that the methodology talks about — lives **inside** that worktree, at `<repo>/.grove-worktrees/<name>/.grove/`, committed to the `<name>` branch. All sessions of a single grove share that one worktree continuously; there is no per-session worktree.

Different groves in the same repo run in separate worktrees on separate branches in parallel. Worktrees all share the same committed `.<harness>/skills/grove/`, so parallel groves never drift in methodology version. `grove finish` first promotes anything from the grove's briefs that should outlive it (ADRs, docs, glossary entries), then **deletes `.grove/` in a focused commit** and merges the branch into the default branch. The default branch never carries any grove's local state; the history of completed groves lives in git's commit graph, not in retained directories.

If a multi-harness repo (both `.claude/` and `.codex/`) launches a grove, the CLI writes a one-line stamp at `<repo>/.grove-stamps/<name>` so later verbs know which harness this grove is bound to. Single-harness repos skip the stamp entirely.

```
grove start <name>                # new grove: create worktree + launch harness on the start prompt
grove continue <name>             # resume: open the worktree and run the loop
grove takeover <name>             # orient on an unfamiliar grove without picking a task
grove retire <name>/<node-path>   # promote brief upward, mv node into done/
grove finish <name>               # grove is done: merge + cleanup per project convention
```

Each verb takes optional `--harness <name>` (auto-detected by default) and `--no-launch` (set up the worktree but skip exec'ing the harness — useful for inspection or scripting). `grove start` also takes `--start-point <ref>` to branch from somewhere other than origin's HEAD.

The exec'd session is pre-named `<repo>: <name> grove` and the worktree carries a `.harness` stamp only when needed to disambiguate in multi-harness repos.

For end-to-end walkthroughs of each verb in context, see [`workflows/`](workflows/).

### What each verb tells the harness

The CLI doesn't gate or enforce — it composes a prompt and execs the harness in the worktree. The prompts are in `content/prompts/*.md` in this repo and live in `.<harness>/skills/grove/prompts/` after materialisation:

- `start` — grill on the goal, sharpen new terminology into `CONTEXT.md` inline, propose the root `BRIEF.md` and one or two initial leaves. Don't over-plan.
- `continue` — pick the next live leaf depth-first, bootstrap by reading, execute, commit, judge retirement.
- `takeover` — read `CONTEXT.md`, the root `BRIEF.md`, skim recent activity. Report state; don't pick a task.
- `retire` — promote anything still relevant from the node's `BRIEF.md` upward, `mv` the subtree into `done/`.
- `finish` — promote anything from the grove's briefs that should outlive the grove, merge per project convention, remove the worktree, delete the branch.

You can edit those prompts in a materialised repo to taste — grove guides, it does not gate. Anything you can do via a verb you can also do by launching the harness by hand inside the worktree and giving it a free-form prompt.

### Steering a planning session

A planning session opens with a grilling — the LLM asks one question at a time, walks down a design tree, and sharpens vocabulary as it goes. The user's job is not to anticipate the agenda but to **redirect it as concerns surface**. Most planning sessions of any depth end up touching subjects neither party started with: a name that lies about its scope, a sync semantics that was silently assumed, a class of failure modes that prior tools have already mapped. These concerns rarely arrive in the order the LLM is grilling them, and they should not wait — the cost of capturing a foundational concern mid-session is one renumber; the cost of capturing it later is a migration.

The pattern the methodology is built around: **new concerns are captured as leaves at the moment they surface**. The planning task that was originally `050-x` may end up renumbered to `070-x` (or further) as foundational concerns are inserted ahead of it, while still being the leaf that gets picked when its turn comes. The numeric prefixes carry the *resolved* dependency order, not the order in which concerns came up. The parent `BRIEF.md`'s notes section records why each insertion happened; that is the durable audit trail, and it is the place future readers go to understand the shape.

Three directions worth giving explicitly during planning:

- **Interrupt when the grilling is asking the wrong question.** The LLM cannot see what you can — if the line of questioning is missing a concern you have spotted, surface it directly. "Before we keep going on X, I want us to look at Y" is enough. The grilling absorbs interrupts gracefully; the planning leaf shifts to accommodate.
- **Make foundational asks when they occur to you, not at the end.** A concern that reshapes the surrounding subtree (a rename, a shape change, a sync model that was assumed) is cheap to act on while the briefs are still wet ink. Waiting until the planning session has otherwise concluded means the concern either becomes a migration or gets lost. The session can absorb several such asks in succession; the cost is mechanical bookkeeping the LLM handles.
- **Say "pause and consolidate" when the renumbers start stacking.** The renumber cost is the visible signal that the session is absorbing rather than executing. After a few rounds it is more productive to commit the current shape and pick up the next actual task than to keep extending the planning brief. "Pause and consolidate" is a recognised direction: the LLM stops adding leaves, ensures the tree is in coherent state, and produces a summary you can act on next.

The mechanical bookkeeping — renumbering files, updating headers, hunting cross-references, growing the parent brief — belongs to the LLM, not the user. Judgement calls (what the new concern is, what to name it, where in the order it should sit) belong to the user. The healthy planning session is one where the user makes a small number of substantive judgements and the LLM converts each into the right tree shape; the unhealthy planning session is one where the user is mentally tracking the numbering. If you notice yourself doing the latter, that is a signal to pause.

For a longer field guide on driving grove well — when to commission prior-art research, how to write a research-leaf brief, grilling moves (WDYT, pushback, running decision log, citation discipline), and when research findings retire into ADRs — see [`driving-a-grove.md`](driving-a-grove.md).

### One-off and exploratory use

There is no global, ambient grove — a globally-installed skill would conflict with the per-project materialised copy and re-introduce the drift problem grove exists to prevent. Even for a single short workstream, run `grove install` in the target repo. The cost is one command and one commit; the benefit is that the experiment is still reproducible weeks later. If the work truly does not warrant a commit, run it freeform without grove at all — that is a more honest choice than a globally-unpinned grove.

## License

grove is licensed under [Apache-2.0](../LICENSE). The bundled grilling procedure and ADR/CONTEXT format guides originate in Matt Pocock's `grill-with-docs` skill (MIT); that upstream licence is preserved at `content/LICENSES/mattpocock-skills.LICENSE`.
