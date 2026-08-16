---
name: grove
description: Use when driving a long, multi-session workstream that cannot be planned exhaustively upfront — work spanning many sessions and months where some steps are themselves planning steps — or when picking up or continuing a task tree under .grove/.
---
<!-- file: order=2 -->
<!-- unit: skill-what-a-grove-is kinds=* class=triggering -->

# grove — hierarchical, self-extending workstreams

A **grove** is one workstream driven as a VCS-tracked **tree of task files**,
one task per session. Planning tasks grow the tree as understanding deepens;
completed leaves are marked done in place. The tree's shape — the directory tree
under `.grove/` — is the only state; the VCS holds the history.

<!-- unit: skill-spine-constraints kinds=* class=triggering defers=skill-the-spine-in-full -->
## The spine — seven constraints

Non-negotiable; everything below is subordinate to them, and
`references/grove.md` argues each one.

1. **Artifacts, not state.** No phase file, no session log, no status file.
2. **Read, don't run.** A session bootstraps by *reading markdown*.
3. **Suggested shape, not enforced schema.** Task files and briefs are freeform.
4. **Lazy and optional.** An artifact is created only when it earns its place.
5. **grove guides, it does not gate.** grove never refuses to proceed.
6. **Walk-away-able.** Delete this skill and `.grove/` is still legible notes.
7. **One page of rules.** If the loop does not fit a page, cut until it does.

<!-- unit: skill-working-tree kinds=* class=triggering -->
## What the driver settled before your session

One task is one session. A grove runs in **a working tree the user
provides** — git or jj-enabled (a `.jj/` present, colocated or native); a main
checkout, linked git worktree, or jj workspace; on any branch, anywhere on
disk; grove reads no branch and no bookmark anywhere (user-owned-worktrees).
The grove's name is that working tree's directory basename, and its task tree
lives at `.grove/` inside it.

<!-- unit: skill-bare-grove-dispatch kinds=* class=triggering defers=skill-dispatch-and-migration -->
Sessions are launched by the `grove` CLI: bare **`grove`**, run from inside the
working tree, is the whole human surface *and* the sole lifecycle entry. It
inspects the state on disk and dispatches — scaffolding a new tree, launching
the next leaf of a live one, appending a `finish` leaf when no ordinary work is
left, and migrating an older-format tree before driving it
(`references/driver.md`).

<!-- unit: skill-self-driving-loop kinds=* class=triggering defers=skill-the-loop-is-stateless -->
Bare `grove` drives the **whole loop**, not one task: one fresh foreground
harness session per task, each with fresh context. The loop holds zero engine
state and re-derives its position from the tree every iteration, so **restart ≡
continuation** — a task that dies before its retire-and-commit boundary leaves
its leaf live and is simply redone (`references/driver.md`).

<!-- unit: skill-one-configuration kinds=* class=triggering defers=skill-what-the-configuration-carries -->
**One configuration, no other launch policy.** Every session is launched by
`~/.config/grove/config.kdl`, which gives each session kind exactly one complete
command template. **Nothing else routes a session** — no environment variable,
no flag, no repository stamp, no field in a task file, and no fallback — and
grove never creates or edits that file (`references/driver.md`).

<!-- unit: skill-session-name kinds=* class=triggering defers=skill-deriving-the-session-name -->
The driver offers this grove's session name — `<repo-basename>: <name> grove` —
as `${session_name}` and never renames a session itself. If your template does
not pass it and the session name doesn't already match, suggest `/rename
<repo-basename>: <name> grove` once per session and move on
(`references/driver.md` derives both names).

<!-- unit: skill-starting-a-new-grove kinds=* class=triggering defers=skill-what-the-scaffold-creates -->
**You never scaffold the tree yourself.** A brand-new grove is a working tree
with no `.grove/` yet; bare `grove` creates the tree, the root `BRIEF.md` stub
and a first `requirements` leaf before any agent exists, then launches it. So
your session starts at **Bootstrap** like every other one, and your commit folds
the scaffold in (`references/driver.md`).

<!-- unit: skill-pick kinds=* class=triggering defers=skill-how-the-pick-walks -->
## The loop

**Pick.** The driver makes **one authoritative pick** before the session exists:
the first live leaf in a pre-order walk of `.grove/`, with nothing modulating
it. That leaf's **stable handle** is your mandate (`references/driver.md`).

<!-- unit: skill-do-not-pick-again kinds=* class=triggering defers=skill-why-a-second-walk-disagrees -->
**Do not pick again.** `grove-llm pick` stays a diagnostic and tree-interface
verb, not this session's dispatcher: a second walk can disagree with your
mandate, and **the mandate wins**.

<!-- unit: skill-stated-vcs-is-definitive kinds=* class=triggering -->
**The version control the driver states is definitive.** It resolved which lane
this working tree is on before the session existed, so **do not re-derive it**
from the working tree, and a harness banner that says otherwise does not win.

<!-- unit: skill-bootstrap kinds=* class=triggering defers=skill-what-bootstrap-reads -->
**Bootstrap.** `grove-llm resolve <handle>` turns your handle into its current
path; one resolving to nothing or to a terminal (`DONE` / `ABANDONED`) leaf is a
stale launch — stop rather than redo it. Then read, in order, the glossary, the
ADRs the briefs cite, the `BRIEF.md` chain root→leaf, and the task file — that
is the whole mandate, so **read nothing else by reflex**
(`references/bootstrap.md`).

<!-- unit: skill-execute kinds=* class=triggering defers=skill-what-each-kind-produces -->
**Execute.** The **filename** states the leaf's session kind — nothing in its
body does — from a closed set of **nineteen**, and **your kind's discipline is
in its own reference file**. `planning` is the only kind with methodological
force: it finds the smallest independently useful working increments, then
grows the tree (`references/execute.md`).

<!-- unit: skill-decompose kinds=* class=triggering defers=skill-two-triggers-two-verbs -->
**Decompose.** Work that surfaces mid-session and does **not** serve this leaf's
stated goal goes to the tree as a new leaf, **never** inline; a leaf that proves
**bigger than its brief** becomes a node, and you do only its first child.
Continue inline only while the work serves this goal *and* fits one focused
session — the bar is *"fits this session,"* not *"I can finish it."*
(`references/decompose.md`).

<!-- unit: skill-bare-stem-rule kinds=* class=triggering defers=skill-why-the-stem-is-bare -->
**Every step of a shape carries the same bare stem** — no `-review`, no
`-integrate`, no `-a` / `-b` / `-combine`. The kind field states the role; the
slug names the artifact (`references/decompose.md`).

<!-- unit: skill-chain-gap-asymmetry kinds=* class=triggering defers=skill-which-hop-a-gap-costs -->
**A chain is not contiguous by construction, and only one of its two hops needs
protecting.** A `review-*` step re-derives from its producer's handle and may
land anywhere; an `integrate-review-*` step consumes findings anchored to
`path:line`, which any intervening edit moves **silently** — so cut it where
`pick` reaches it next (`references/decompose.md`).

<!-- unit: skill-no-exception-to-check kinds=* class=triggering defers=skill-why-there-is-no-exception -->
**There is no exception to check.** Adjacency is unconditional guidance: the
check an exception would need cannot be performed, and a session that departs
anyway owns the drift.

<!-- unit: skill-retire kinds=* class=triggering defers=skill-leaf-retire-mechanics -->
**Retire.** A leaf ends **done** (*harvested*) or **abandoned** (*pruned*): both
mark it **in place**, neither deletes it, and both are skipped by `pick`. Retire
*before* you commit, so the rename lands in it (`references/retire.md`).

<!-- unit: skill-retirement-touches-one-filename kinds=* class=triggering -->
Retirement touches **one filename and nothing else** — not the leaf's own body,
not a sibling, not an ancestor. A leaf that a review is waiting on is no
exception: the review reads the committed artifact.

<!-- unit: skill-pruning-is-hitl kinds=* class=triggering defers=skill-leaf-prune-mechanics -->
Pruning is **HITL — an agent never prunes on its own**: an AFK session that
finds its leaf's path decided against says so and stops. The loop stalling on an
abandonment decision is the system working, not a fault.

<!-- unit: skill-node-close-cascade kinds=* class=triggering defers=skill-node-close-steps -->
Then walk the parent chain: a node with **no live leaf left in its subtree** is
**implicitly done** — never marked, since a brief is context rather than a task.
**The close asks the human nothing.** Instead the session **verifies and
reports** (`references/retire.md`).

<!-- unit: skill-commit kinds=* class=triggering defers="skill-commit-boundary-in-git-and-jj skill-why-the-handle-outlives-the-path" -->
**Commit.** One task = one focused commit — the artifact, whatever the grow
verbs wrote, and the `DONE` rename that retires the leaf, together with anything
the cascade above promoted or added. **This is why Retire comes first**: the
message cannot name a node you have not yet closed. **Name the work item, and
each closed node, by its stable handle `<slug>-k<key>`** rather than by position
or path (`references/commit.md`).

<!-- unit: skill-finish kinds=* class=triggering -->
**Finish.** You do not discover that a grove is finished — the driver does, and
says so by launching a `finish` session; retiring the last live leaf is an
ordinary retirement, not a cue to tear anything down. That session's human
confirmation is the loop's **only routine human gate** (confirmation-boundary);
every other question a session asks is a discretionary escalation — always
legitimate, never a step grove requires of you.

<!-- unit: skill-artifacts kinds=* class=triggering -->
## Artifacts

Only the task tree is grove-specific and ephemeral. Everything else is a
standard artifact that outlives grove (constraint 6).

| Artifact | Path | Role |
|---|---|---|
| Glossary | `CONTEXT.md` (+ `CONTEXT-MAP.md`) | the Ubiquitous Language — read every session, appended inline |
| ADRs | `docs/adr/<slug>.md` | one decision each, as a **minimum coherent set** — slug-named, edited in place; philosophy per `linkuistics:decision-records` |
| Specs | `docs/specs/<slug>.md` | how an area works — the human-facing agreement point, also a **minimum coherent set** (`SPEC-FORMAT.md`) |
| Task tree | `.grove/` (inside the grove's working tree) | the process: the self-extending decomposition of work; deleted at the in-session Finish step |

<!-- unit: skill-adrs-and-specs kinds=* class=triggering defers=skill-the-record-sets -->
Whichever kind is running: raise ADRs *sparingly* (`ADR-FORMAT.md`), write a
spec only at a genuine agreement point (`SPEC-FORMAT.md`), and treat both as a
**minimum coherent set describing the current design** — a session that
*changes* a recorded decision **reworks that set in place** and **never appends
a superseding record** (`references/execute.md`).

<!-- unit: skill-glossary-is-load-bearing kinds=* class=triggering defers="skill-why-the-glossary-holds context-structure" -->
**The glossary is load-bearing.** `CONTEXT.md` is read every session and
appended *inline* whenever a term is resolved — that is the forcing function
against terminology drift, the acute failure mode of multi-session work. Keep it
a glossary and nothing else — terse definitions, aliases-to-avoid, no
implementation detail (`CONTEXT-FORMAT.md`).

<!-- unit: skill-briefs-vs-glossary kinds=* class=triggering -->
**Briefs vs. the glossary.** A bounded context is a *domain* partition; a
task-tree node is a *process* partition. They are orthogonal axes. The glossary
is per-bounded-context; a node that carries anything carries a `BRIEF.md`, not a
glossary.

<!-- unit: skill-linkuistics-prerequisite kinds=* class=triggering defers=skill-what-the-plugin-carries -->
**Prerequisite — the `linkuistics` plugin**, which grove **requires** and does
not provision: ADR philosophy in `linkuistics:decision-records`, test seams in
`linkuistics:codebase-design`, and the working-copy-as-commit lane in
`linkuistics:using-jujutsu`, which the Commit step cites rather than restates.
It installs separately, through the Claude Code marketplace or the repo's
`plugins/install.sh` (`references/grove.md`).
