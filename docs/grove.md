# grove

grove is a tool for driving long, multi-session workstreams as a VCS-tracked tree of task files — one task per session, where planning tasks grow the tree as understanding deepens and completed leaves are marked done in place. It builds on two established ideas, not de-novo invention: Matt Pocock's [`skills`](https://github.com/mattpocock/skills) repo — whose `grilling` procedure and `CONTEXT.md` / `ADR-FORMAT.md` conventions (from the `grilling` and `domain-modeling` skills) grove bundles wholesale — and Domain-Driven Design's **Ubiquitous Language** and **bounded contexts**. This document covers why it exists, how it works, how to install it, and how the CLI's verbs map onto the methodology.

The methodology itself — the loop, the seven constraints, the BRIEF/CONTEXT/ADR/TASK formats, the grilling procedure — lives in [`content/SKILL.md`](../content/SKILL.md) and its sibling files. This doc is the *project-level* introduction; `SKILL.md` is what an agent reads at runtime.

## The problem grove solves

Software work that spans many sessions and many months does not arrive with its full shape known. Some early steps are themselves planning steps — their output is not code but more steps, and you cannot enumerate them until you have done the planning. A monolithic implementation plan suits work whose scope is settled; it does not suit a project that grows as you walk it. Once you commit to an exhaustive upfront decomposition, every unexpected discovery either breaks the plan or gets swept under it.

Each agent session starts fresh — no memory of prior sessions. In a long workstream this is the acute failure mode: session 1 coins a term; session 7, with no memory of session 1, reinvents it under a different name, or reuses the words with a subtly shifted meaning. The glossary becomes incoherent without anyone noticing. Decisions made weeks ago are silently relitigated. The design fractures across sessions because no single session sees the whole.

Earlier attempts to solve this failed in characteristic ways. Phase-machinery approaches — a state machine tracking the workstream through named phases — became brittle: any corrupted phase file could block work, the machinery accrued special cases, and eventually the overhead of managing the process exceeded the cost of the work. GitHub-issue tree approaches were the natural next thing to consider, but they have their own problems: the task tree is decoupled from the repo's history, and a session can't orient itself without running external tools (API calls, issue trackers) — violating the principle that bootstrap should be markdown-only.

grove is the alternative that avoids both traps.

## How grove solves it

A grove is one workstream as a **VCS-tracked tree of task files** at `.grove/` (inside the grove's own working tree). Nodes are directories; leaves are `.md` task files with numeric prefixes. The tree's shape — what `ls` shows — is the only state. The VCS holds the history. No phase file, no session log, no status tracker.

One task = one session = one focused commit. Planning tasks, which may grow the tree rather than produce code, are first-class — not an awkward edge case but a named kind with a defined procedure.

Task files are drawn from a closed set of five kinds: **planning** (**opens with a grilling session** — using `grilling.md`, the procedure bundled from Matt's `grilling` and `domain-modeling` skills — to interrogate the design one question at a time, then through the grilling sharpens vocabulary, may raise an ADR, and grows the tree by replacing a leaf with a node of child briefs and ordered leaves), **research** (a citation-disciplined survey, producing `docs/research/<slug>.md`), **prototype** (a cheap throwaway artifact built to react to, not to ship), **work** (produces code, docs, or tests), and **review** (a fresh-context adversarial read of already-done work). A task too big for one focused session *is* a planning task — its job is to decompose, not to do. Only planning carries methodological force — it is the loop's sole branch and the only kind that grows the tree; the other four differ in discipline and in the model the self-driving loop launches them on ([`docs/adr/model-per-task-kind.md`](adr/model-per-task-kind.md)), not in what the loop does with them.

The **[Ubiquitous Language](concepts.md#ubiquitous-language)** — DDD's term for the project's shared domain vocabulary — lives in `CONTEXT.md` at the repo root: a terse glossary of domain terms, aliases-to-avoid, and nothing else. It is read at the start of every session and appended *inline* whenever a term is resolved during a session. This is the forcing function against terminology drift: the glossary is always live, always current, and always the first thing a session reads.

When a project splits into multiple **[bounded contexts](concepts.md#bounded-context)** — DDD's term for distinct domain partitions, each with its own vocabulary — each gets its own `CONTEXT.md`, linked by a root [`CONTEXT-MAP.md`](concepts.md#context-map). A bounded context (a *domain* partition) is orthogonal to a task-tree node (a *process* partition): the glossary is per-bounded-context; a node carries a `BRIEF.md`, not a glossary. The two axes don't compete.

Artifacts are **lazy and optional**. An [ADR](concepts.md#adr) is raised only when a decision is hard to reverse, surprising, or a real trade-off — not because a step demands one. A [spec](concepts.md#spec) is written only at a genuine human-facing agreement point. A brief is created only when a node is needed. Nothing is produced speculatively.

Bootstrap is **read-only**: a session reads the glossary, the ADRs cited by the briefs, the `BRIEF.md` chain from root to the current leaf, and the task file itself. No script must succeed before work begins. Delete the grove skill and `.grove/` is still a legible folder of notes.

grove operates under **seven constraints** — the non-negotiable rules that keep it from becoming brittle machinery. They are not restated here; see [`content/SKILL.md`](../content/SKILL.md) for the list and their rationale.

The loop in one sentence: pick the first live leaf depth-first → bootstrap by reading → execute → commit → retire the node if its last leaf is done, then repeat. `SKILL.md` has the full mermaid diagram.

## Installing grove

grove ships as a single Rust binary. Install it via Homebrew:

```
brew tap Linkuistics/taps
brew install grove
```

That is the whole installation. The `grove` binary **embeds its full methodology** — `SKILL.md`, the format references, the grilling and driving guides, the launcher prompts — and provisions it to your **personal** global skill directory, `~/.claude/skills/grove/`, on the first `grove do`. The extraction is idempotent against a content-hash stamp and re-runs only when the binary changes, so the skill an agent reads can never drift from the installed binary (self-extension-core-and-methodology / task-tree-scheme). There is no per-repo install step, no files to commit into a consuming repo, and nothing to keep in sync; `grove --version` reports the binary's version, and the methodology version *is* the binary version.

Provisioning to Claude Code's **personal** skill dir is deliberate: Claude Code resolves skills enterprise > personal > project, so the binary-provisioned copy takes precedence over any same-named project skill and always wins. That precedence is the reason grove does not also drop a per-repo copy — a project-local mirror would be dead, shadowed code that could only mislead a contributor into editing the wrong file. To change the methodology, edit `content/` in the grove repo and rebuild; the new binary reprovisions the global skill on its next `grove do`.

### Updating

Upgrading the binary upgrades the methodology. `brew upgrade grove` installs a new binary, and the next `grove do` re-provisions the global skill from it — the content-hash stamp makes a matching skill a no-op and a changed one a refresh. There is no separate update verb and no per-project version to pin: the binary on your `$PATH` is the single source of truth.

## Driving a grove

A grove lives in two places — the CLI binary (Homebrew, used from anywhere, carrying the embedded methodology it provisions to `~/.claude/skills/grove/`); and the grove itself, which is **any working tree you provide** — git or jj — created however you like (`git worktree add`, `git init`, `jj git init --colocate`, `jj git clone`, `jj workspace add`, a plain checkout, or a dedicated tool such as [worktrunk](https://github.com/max-sixty/worktrunk)), on any branch, anywhere on disk (user-owned-worktrees). The task tree — the `.grove/` directory of briefs and leaves that the methodology talks about — lives **inside** that working tree, committed to whatever branch it's on. All sessions of a single grove share that one working tree continuously; there is no per-session worktree, and grove reads no branch anywhere.

Different groves — including several against the same repo — run in separate working trees in parallel, each on whatever branch its owner gave it. They all read the one binary-provisioned global skill, so parallel groves never drift in methodology version. Finishing a grove is an **in-session** step (there is no `grove finish` verb): when the grove has no live leaves left, the running loop first promotes anything from the grove's briefs that should outlive it (ADRs, docs, glossary entries), then **deletes `.grove/` in a focused commit** and signals the loop to stop. That is the whole cycle — grove creates no VCS topology, so it merges none and deletes none either (user-owned-worktrees): integrating the branch and tearing down the working tree are the user's own git/gh or jj, or their worktree tooling, done after `.grove/` is already gone.

The CLI writes a one-line stamp at `<repo>/.grove-stamps/<name>` whenever `--harness` is passed explicitly, and also when a multi-harness repo (two or more of `.claude/`, `.codex/`, `.pi/`) launches a grove, so later verbs know which harness this grove is bound to. A single-harness repo relying on auto-detection stays stamp-free — there's nothing to disambiguate.

```
grove do                   # the sole lifecycle entry verb, run from inside your working tree
grove retire <node-path>   # promote a finished node's brief upward (its leaves stay marked done in place)
```

`grove do` is **argument-less** and run from inside the working tree you're standing in — it inspects the state on disk and dispatches: no `.grove/` yet → open a bootstrap session; a live tree → continue; no live leaves left → propose the complete finish cycle. (The former `grove start`, `grove continue`, and `grove finish` are removed; `do` covers all three — do-is-sole-lifecycle-verb.) If `.grove/` is still in an older on-disk format, `grove do` migrates it to the current directory scheme first — one reviewable, committed change — before driving (task-tree-scheme).

Once driving, `grove do` runs the **self-driving loop** (self-driving-loop): it launches a fresh, clean-context session per task and relaunches automatically each time a session fires its completion signal (`grove-llm complete`), walking the tree until no live leaf is left — at which point the loop proposes the in-session finish cycle. Any non-signalling exit — your `/exit`, a Ctrl-C, or a crash — stops the loop; re-running `grove do` from the same working tree resumes it, because the loop holds no state of its own and re-derives its position from the tree each iteration.

Each verb takes optional `--harness <name>` (auto-detected by default) and `--no-launch` (report readiness but skip exec'ing the harness — useful for inspection or scripting).

The exec'd session is pre-named `<repo-basename>: <name> grove`, where `<name>` is the working tree's own basename; a one-line stamp at `<repo>/.grove-stamps/<name>` records the harness binding whenever `--harness` is passed explicitly, and also when needed to disambiguate in multi-harness repos.

For end-to-end walkthroughs of each verb in context, see [`workflows/`](workflows/).

### What each verb tells the harness

The CLI doesn't gate or enforce — it composes a prompt and execs the harness in the worktree. The prompts are in `content/prompts/*.md` in the grove repo, ship inside the binary, and are provisioned to `~/.claude/skills/grove/prompts/`:

- `start` — grill on the goal, sharpen new terminology into `CONTEXT.md` inline, propose the root `BRIEF.md` and one or two initial leaves. Don't over-plan.
- `continue` — pick the next live leaf depth-first, bootstrap by reading, execute, commit, signal completion, judge retirement. This is the per-task prompt the self-driving loop relaunches.
- `retire` — promote anything still relevant from the node's `BRIEF.md` upward; its leaves stay marked done in place, so nothing moves.

There is no `finish` prompt: finishing is an in-session step of the loop, not a launched verb — the running session proposes the finish cycle when `pick` comes up empty. To change any prompt or the methodology, edit `content/` and rebuild; grove guides, it does not gate. Anything you can do via a verb you can also do by launching the harness by hand inside the worktree and giving it a free-form prompt.

### Steering a planning session

A planning session opens with a grilling — the LLM asks one question at a time, walks down a design tree, and sharpens vocabulary as it goes. The user's job is not to anticipate the agenda but to **redirect it as concerns surface**. Most planning sessions of any depth end up touching subjects neither party started with: a name that lies about its scope, a sync semantics that was silently assumed, a class of failure modes that prior tools have already mapped. These concerns rarely arrive in the order the LLM is grilling them, and they should not wait — the cost of capturing a foundational concern mid-session is one renumber; the cost of capturing it later is a migration.

The pattern the methodology is built around: **new concerns are captured as leaves at the moment they surface**. The planning task that was originally `05-x-k12` may end up renumbered to `07-x-k12` (or further) as foundational concerns are inserted ahead of it, while still being the leaf that gets picked when its turn comes. The per-level position numbers carry the *resolved* dependency order, not the order in which concerns came up — and each leaf's permanent `-k<key>` rides through every renumber unchanged, so references stay valid. The parent `BRIEF.md`'s notes section records why each insertion happened; that is the durable audit trail, and it is the place future readers go to understand the shape.

Three directions worth giving explicitly during planning:

- **Interrupt when the grilling is asking the wrong question.** The LLM cannot see what you can — if the line of questioning is missing a concern you have spotted, surface it directly. "Before we keep going on X, I want us to look at Y" is enough. The grilling absorbs interrupts gracefully; the planning leaf shifts to accommodate.
- **Make foundational asks when they occur to you, not at the end.** A concern that reshapes the surrounding subtree (a rename, a shape change, a sync model that was assumed) is cheap to act on while the briefs are still wet ink. Waiting until the planning session has otherwise concluded means the concern either becomes a migration or gets lost. The session can absorb several such asks in succession; the cost is mechanical bookkeeping the LLM handles.
- **Say "pause and consolidate" when the renumbers start stacking.** The renumber cost is the visible signal that the session is absorbing rather than executing. After a few rounds it is more productive to commit the current shape and pick up the next actual task than to keep extending the planning brief. "Pause and consolidate" is a recognised direction: the LLM stops adding leaves, ensures the tree is in coherent state, and produces a summary you can act on next.

The mechanical bookkeeping — renumbering files, updating headers, hunting cross-references, growing the parent brief — belongs to the LLM, not the user. Judgement calls (what the new concern is, what to name it, where in the order it should sit) belong to the user. The healthy planning session is one where the user makes a small number of substantive judgements and the LLM converts each into the right tree shape; the unhealthy planning session is one where the user is mentally tracking the numbering. If you notice yourself doing the latter, that is a signal to pause.

For a longer field guide on driving grove well — when to commission prior-art research, how to write a research-leaf brief, grilling moves (WDYT, pushback, running decision log, citation discipline), and when research findings retire into ADRs — see [`driving-a-grove.md`](driving-a-grove.md).

### One-off and exploratory use

There is no per-repo install gesture to skip: the binary already carries the methodology and provisions it globally, so a working tree plus argument-less `grove do` is all it takes to start even a single short workstream. The cost is one command; the benefit is that the experiment runs under the same loop and leaves the same legible `.grove/` notes as any other grove. If the work truly does not warrant its own working tree, run the harness freeform without grove at all — that is a more honest choice than bending grove around a task too small for it.

## License

grove is licensed under [Apache-2.0](../LICENSE). The bundled grilling procedure and ADR/CONTEXT format guides originate in Matt Pocock's `grilling` and `domain-modeling` skills (MIT); that upstream licence is preserved at `content/LICENSES/mattpocock-skills.LICENSE`.
