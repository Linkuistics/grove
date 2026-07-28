# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Global skill provisioning** / **skill precedence**:
The `grove` binary embeds `content/` and, on every `grove do`, sweeps it out to **every installed harness's** personal global skill dir — `provision_all` writes the launching harness's dir unconditionally plus every other harness whose home root (`~/.claude`, `~/.codex`, `~/.pi`) already exists, each via `skill_dir_for(harness)` (`$HOME/<harness.skills_dir>/grove`; pi nests under `agent/`) — idempotent per target against a content-hash stamp (task-tree-scheme). `content/` stays canonical; the binary is the only writer of any of these dirs, and `launch::load_prompt(harness, verb)` reads the launcher prompts from the *launching* harness's own copy (no repo-local mirror). `provision_target` guards each write: a symlink (the old cross-harness link-farm layout) is replaced as a link, never deleted through; a dir with no grove stamp and real content is refused outright, so a foreign dir can never be silently clobbered. Claude Code skill precedence is **enterprise > personal > project** (Claude Code docs): a **personal** skill *overrides* a same-named **project** skill (`<repo>/.claude/skills/grove/`), so the binary-provisioned global copy always wins there. A leftover project mirror is therefore **dead, shadowed code** — not an active override — and is removed (070/050) so it cannot mislead a contributor into editing it; no new project mirror should be re-added (edit `content/` and rebuild instead). This is now the only distribution model: the old fetch+materialise machinery and its `cli`/`repo`/`worktree` `VERSION.md` drift were deleted in 090, along with `grove install`/`uninstall`/`status`.

**Complete finish cycle**:
The terminal, whole-grove sequence that retires a grove once it has no live leaves left: (1) promote durable artifacts from the briefs (ADRs, docs, glossary); (2) delete `.grove/` in a focused commit; (3) signal the loop with `grove-llm complete --done`. Nothing after: integrating the branch and tearing down the working tree are **not** grove workflow — both belong to plain git/gh or jj, and the user's own worktree tooling (user-owned-worktrees). Driven by the in-session LLM, not Rust automation (in-session-finish-cycle); proposed and executed only on explicit human confirmation, so a headless run with no human reports the plan and stops; resumed from VCS/filesystem state with no progress marker (constraint 1). Triggered whenever `grove-llm pick` reports no live leaves — or errors because `.grove/` is already gone, the partial-finish resume case. Distinct from the per-leaf Retire step (which marks one leaf done in place with a `DONE` infix); the finish cycle is what retiring the *last* leaf leads into. The former `grove finish` verb that launched this was removed (do-is-sole-lifecycle-verb): finishing is a step of the loop, not a launched verb.
_Avoid_: describing the finish as merging or deleting anything git-topological — that was the pre-v11 cycle.

**Grove name**:
The working-tree directory's basename — never a branch, a bookmark, or a canonical layout. Resolved **jj-first**: a `.jj/` directory heading the tree makes it jj-enabled (native, secondary workspace, or colocated — `.jj/` wins even with a `.git` beside it) and the workspace root is the directory holding `.jj/`; only a not-jj-enabled tree falls back to `git rev-parse --show-toplevel`. The closest marker walking up decides, so a plain-git checkout nested under a jj tree stays git. Names the root brief (`# <name> — brief`), the harness session (`<repo-basename>: <name> grove`), and the harness stamp (`<repo>/.grove-stamps/<name>`).
_Avoid_: "the grove name equals the branch name" — grove reads no branch anywhere.
_Avoid_: describing resolution as git-with-jj-fallback — it is the other way round, in `repo.rs` and in every tree-mutation verb (a jj tree renames with `fs::rename`; `git mv` is the fallback).

**root-init** / **fresh-grove start**:
The bootstrap of a brand-new grove (a working tree exists — user-provided, git or jj-enabled, any branch, anywhere — but no `.grove/` tree yet), enacted by the `grove-llm root-init [<slug>]` verb. It creates `.grove/`, the root `BRIEF.md` stub, and a first **requirements** leaf `01-<slug>-k1.md` (default slug `plan`) — a working-tree change with no commit, refusing to clobber an existing `.grove/`. It is the one tree verb that sits *below* the floor the others stand on (`leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` all require `.grove/` to already exist), so it is what makes a fresh grove enter the steady-state loop. Creating the first leaf — not just the root brief — is load-bearing: `grove-llm pick` skips every `BRIEF.md`, so a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the [[Complete finish cycle]] (fresh-grove-start-contract). The kind is **fixed** — there is no `--kind` — on two grounds: the bootstrap session's only input is the human's own words, which is the [[HITL]] rule, and the loop driver launches `start` *before* this verb has run, so that launch can only be routed by construction. Hence `GROVE_REQUIREMENTS_MODEL` (or a harness-scoped spelling) is the single [[Kind routing]] var a brand-new grove cannot start without. The first session's commit folds the scaffold in. Distinct from [[Bootstrap]]-the-loop-step (reading context at the start of every session); this is the one-time creation of the tree that the loop then reads.
_Avoid_: "the bootstrap leaf is planning" — that was the pre-taxonomy answer, and it survives only by marking `planning` HITL again. The bootstrap session may still *cut* leaves (the requirements/design/planning fusion a small workstream is allowed); the label names the discipline that always applies.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

**Task kind** (`**Kind:**`):
The one-word session-shape declaration on a [[Leaf]]'s task file, drawn from a
**closed, parameterised set of seventeen**: five producers — `requirements`
(grills; establishes *what*), `design` (establishes *how*; produces a spec / ADR
set), `planning` (cuts vertical slices; grows the tree), `prototype` (a cheap
throwaway artifact to react to), `impl` (produces code, docs, tests) — each with
its own `review-` and `integrate-review-` step, plus `research` (produces
`docs/research/<slug>.md`) and `combine-research`. `planning` is the only kind
with methodological force — the sole branch in the loop's Execute step, and the
only kind that grows the tree; every other kind produces an artifact and differs
in discipline and in [[Kind routing]]. grove **gates on write** (a grow verb
rejects an unknown `--kind`, where a human is present to fix it) and **degrades
on read** (an unrecognised `**Kind:**` line warns and is treated as `impl`, so a
hand-edited leaf can never jam the loop — constraint 5). `leaf-decompose` gives
the first child its parent leaf's kind, so a research leaf that proves bigger
becomes a research node. `work` was renamed `impl`; it still *reads* as `impl`,
silently, but is refused on write. See ADR *task-kind-taxonomy* for why the set
is closed and parameterised, and `docs/specs/task-kind-taxonomy.md` for the
membership and each kind's discipline.
_Avoid_: adding a kind that carries no behaviour beyond a name — a kind must earn
its place with a distinct discipline, and an eighteenth is a change to this closed
set, not a free-text label.
_Avoid_: "the five kinds" — that count is pre-taxonomy-rework, as is `work` as a
live label.

**Review chain** / **vendor pair**:
The two composition patterns over the [[Task kind]] set, and the reason [[Kind
routing]] needs two mechanisms. The **review chain** is `X` → `review-X` →
`integrate-review-X`: sequential, **adversarial** (the reviewer's job is to find
fault), and each step is a *different kind*, so per-kind routing alone expresses
it. The **vendor pair** is `research` → `research` → `combine-research`:
**breadth-and-confirmation** (two independent surveys, unioned), where the two
producers are the *same kind differing only by vendor* — which no kind→harness
function can express, and is therefore the entire reason a per-leaf harness
declaration exists. The fan is a **pair**, not an N-way fan-out, so the combine
step is binary.
Both are **named off a shared stem with a terminal step suffix** —
`<stem>` / `<stem>-review` / `<stem>-integrate`, and `<stem>-a` / `<stem>-b` /
`<stem>-combine` — so a cut chain is legible from `find .grove` without opening a
file, which the [[Task kind]] cannot be (it lives in the file, not the name). The
convention is a **habit nothing parses**; the guidance a session reads while
cutting leaves carries it (`content/SKILL.md`'s Decompose step,
`content/TASK-FORMAT.md`, and the bootstrap prompt), and
`docs/specs/task-kind-taxonomy.md` holds the reasoning.
_Avoid_: a *leading* step token (`review-<stem>`) — it sorts every review beside
every other review and scatters the chains the naming exists to reveal.
_Avoid_: giving a chain its own [[Node directory]] — a node already means "this
work proved bigger than one session", and one per chain overloads that signal,
buys a `BRIEF.md` no step earned, and would apply `leaf-decompose` (a *reactive*
verb) speculatively.
_Avoid_: treating either as enforced — grove validates no ordering between
leaves, because a grammar is a relation *between* leaves and grove expresses
none (ADR *task-kind-taxonomy*).
_Avoid_: running the *researchers* adversarially — that discards the breadth the
pair was run for. The adversarial move belongs to `combine-research`, whose
discipline is that **agreement without independent primary sourcing is a red
flag, not a confirmation** (two vendors on overlapping corpora can agree on
something false).

**HITL** / **AFK**:
Whether a [[Task kind]] resolves through live exchange with a human who speaks for
themselves (`requirements`, `prototype`) or is driven by the agent alone (every
other kind). The generating rule: a kind is HITL when **a human's own words are
the session's input or its deliverable** — not merely when a human would want to
read the output, which is true of everything and which the VCS already serves. A
HITL leaf reached by an unattended relaunch of the self-driving loop stalls until
a human arrives; that is correct, not a fault.
The mark **predicts, it does not permit**: *any* kind may stop and ask a human,
and doing so is always legitimate. `design` often will;
`integrate-review-requirements` is the borderline cell (it edits what was asked
for, which it cannot change unilaterally) and is marked AFK anyway.
_Avoid_: an agent answering its own HITL questions — a `requirements` session
that grills itself has broken the distinction (`grilling.md`).
_Avoid_: "`planning` is HITL" — it flipped to AFK when grilling moved to
`requirements`. Planning keeps its methodological force but no longer
interrogates.

**Kind routing** (`GROVE_<KIND>_HARNESS`, `GROVE_<KIND>_MODEL`):
How the self-driving loop decides **which harness** runs the picked [[Leaf]] and
**which model** that harness loads, both keyed on the leaf's [[Task kind]]. The
driver peeks the leaf (`grove-llm kind --with-harness`) every iteration and
resolves two axes via each harness's native launch flags — no router, no proxy.
*Harness*: **leaf beats kind beats family beats stamp** — a leaf's own
`**Harness:** <name>` line first, then `GROVE_<KIND>_HARNESS`, then
`GROVE_<FAMILY>_HARNESS`; unset everywhere means the **stamped** harness, which
is an explicit on-disk binding, not a default. The per-leaf line exists for the
[[Review chain]]'s counterpart, the vendor pair, and is read **strictly** — an
unrecognised name refuses to launch rather than degrading as a [[Task kind]]
does, because a wrong harness runs the leaf on a vendor the tree said not to.
*Model*: four keys, **harness-major** — `GROVE_<HARNESS>_<KIND>_MODEL` >
`GROVE_<HARNESS>_<FAMILY>_MODEL` > `GROVE_<KIND>_MODEL` > `GROVE_<FAMILY>_MODEL`
— because the harness axis is a *correctness* axis (a codex profile name is
garbage to pi) while the kind axis is only a *preference* axis. A **rerouted**
launch consults no unscoped var, truncating the lattice to the first two.
**A kind that resolves no model var fails loudly**; there is no fall-through to
the harness's own default, because that is still grove deciding, only less
visibly. Exempt: no live leaf, a harness whose model-flag template is empty, and
harness absence. Two [[Task kind]] **families** exist (`review-*`,
`integrate-review-*`) so one policy line covers five kinds. Ceiling 95 vars.
See ADR *model-per-task-kind*.
_Avoid_: "the stamp absorbs every kind that is not rerouted" as a claim about the
**model** surface — it absorbs the *harness* axis only. Falling through to the
stamp still needs a model var per kind, so the "about nine vars" figure is one
stamped harness resolving through the unscoped keys; drive groves stamped to
several harnesses and it is nine *per harness* in the harness-scoped spelling
(measured at ~27 on the first real migration).
_Avoid_: "per-kind model selection" as the whole name — the harness axis is half
of it, and the family axis spans both.
_Avoid_: calling this "model routing" — that implies a multi-provider proxy;
this is native launch-flag selection against harnesses grove already knows.
_Avoid_: "an unset var means the session inherits your own default" — that was
the pre-rework rule and is now an error. The `/model`-persistence asymmetry it
caused is gone with it, except for a harness that takes no model flag at all.
_Avoid_: "there is no fallback chain" unqualified — falling back *across* kinds
is still rejected, but resolving within a **family** the user configured as a set
is carved out.

**Spec** (`docs/specs/<slug>.md`):
The human-facing, team-shareable design of an *area* of the system — problem,
solution, settled decisions, agreed test seams, out-of-scope — written lazily by a
`design` task at a genuine agreement point. The flow around it spans four kinds:
grill (`requirements`) → spec (`design`) → decompose (`planning`) → execute
(`impl`).
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
_Avoid_: "a planning task writes the spec" — pre-taxonomy, from when `planning`
covered grilling and design both. `design` is the producer whose deliverable is a
spec, an ADR set, or both; `planning` consumes it and cuts the leaves.

**herdr integration**:
grove's relationship with [herdr](https://herdr.dev), the agent multiplexer whose
sidebar rolls per-pane agent state up to tabs and workspaces: **optimised-for,
never required** (ADR *herdr-optional-ui*). Split in two. **Semantic state**
(`idle` / `working` / `blocked`) is reported *by grove* over herdr's unix socket,
addressed by the `HERDR_*` variables herdr places in the pane environment —
skipped entirely when they are absent, and a no-op if the socket refuses, or if
the herdr on the other end is stock and drops the report ([[Authority patch]]).
**Everything richer** — the tree, the live leaf, progress — is rendered by the
[[Tree viewer plugin]], which reads `.grove/` directly; grove pushes it nothing.
The split works because the tree on disk already *is* the status (constraint 1),
so the only thing worth reporting is what no artifact records: whether the agent
is mid-turn or has stopped and is waiting for a human. grove reports as agent
**`grove`**, not as the harness it launched — a `grove do` pane is a loop
relaunching a *sequence* of sessions, and the harness may vary per leaf.
_Avoid_: "grove requires herdr" or "herdr drives the loop" — with no herdr
present every grove behaviour is unchanged, minus the status surface. Panes-per-leaf
sequencing over the socket would be an amendment to the spine (constraint 6), not
a feature.
_Avoid_: treating the plugin as a state authority — herdr's *full lifecycle
authority* is a compiled-in allowlist of `(source, agent)` pairs that nothing
outside the binary can join; the plugin owns UI only.

**Tree viewer plugin** (`herdr-plugin/`, plugin id `linkuistics.grove`):
The rendering half of the [[herdr integration]] — a herdr plugin that draws a
grove's task tree in a pane: the live leaf marked, its [[Task kind]], the done
and pruned behind it, live [[Node directory]]s expanded and finished ones
collapsed to their counts. **UI only, never state**: herdr's full lifecycle
authority is a compiled-in allowlist nothing outside its binary can join, and a
plugin has exactly the socket access `grove-llm` already has, so routing state
through one would add a hop and buy nothing.
Its **only** contract is the [[Node directory]] naming scheme (task-tree-scheme).
It never invokes `grove` or `grove-llm`, opens no socket and writes no state, so
the plugin and the binary version independently — the property that makes
"optimised-for" real rather than aspirational. Consequently *changing the
directory scheme is now also a plugin-compatibility question*. Filename-only
parsing is what the scheme was designed for, so the whole shape costs one
`scandir` per directory and the only file read is a live leaf's `**Kind:**` line;
refresh is a 1 s poll redrawing on change, not a filesystem watcher.
Lives at the repo root, **not** under `plugins/` — that directory is the *skills*
bounded context (Claude Code plugins behind `.claude-plugin/marketplace.json`),
and this is grove's own context by subject. Installed separately again
(`herdr plugin install Linkuistics/grove/herdr-plugin`); needs `python3` and
nothing else, so there is no build step. Its pane is summoned by a plugin
**action**, because a herdr keybinding can target an action but never a pane
entrypoint.
_Avoid_: opening its pane with `plugin pane open --cwd` — that **replaces** the
plugin root as the process cwd, so herdr resolves the manifest's relative command
against the new directory and the pane dies on spawn. The invoking pane's cwd
already arrives in `HERDR_PLUGIN_CONTEXT_JSON` (on the split *and* the overlay
path), which is why the override buys nothing.
_Avoid_: resolving the grove from the plugin's *own* cwd — that is the plugin
directory, which says nothing about the pane the human is looking at, and which
(when linked from a checkout that is itself a grove) sits **under** one and would
render the wrong tree.
_Avoid_: treating a `herdr plugin link` as durable — it records the path, so
linking a grove's throwaway working tree dies with that worktree.

**Authority patch** / **session identity vs lifecycle state**:
The two-hunk change grove carries in its herdr fork, encoding one principle: **a
hook report that makes no session-identity claim neither conflicts with, nor
clears, the identity owner**. herdr conflates the two concerns — who owns a
pane's session identity (for resume) and who reports its lifecycle state — so
its `set_hook_authority_at` lets the identity owner veto a differently-sourced
state report, and clears the identity record on every accepted one. Both
behaviours are gated on `session_ref.is_some()` by the patch. grove never sends a
`session_ref`, so it reports state without disturbing the harness's session
resume. Shipped from `linkuistics/taps` as upstream's version plus
**`-linkuistics.<seq>`** — a sequence incrementing per ship and resetting when
upstream's version bumps, because a commit sha does not order and Homebrew needs
ordering to see an upgrade. Carried on **two** fork branches with different jobs:
`authority-fix` off `upstream/master` holding *only* the fix (kept pure, so it
stays reviewable and rebasable on its own), and `ui-layout` as the ship branch,
which takes the fix by **merge** so it never needs a force-push. Tracked closely
against upstream. The carry is **permanent**: offering the patch upstream was
considered and rejected, so only upstream reaching the same separation
independently would end it — which makes every additional hunk a rebase
obligation forever, and is why the patch stays at two. See ADR
*herdr-optional-ui* for the decision, and `docs/specs/herdr-fork-maintenance.md`
for how the carry is actually maintained (rebase cycle, version suffix scheme,
the required build environment, and how to verify a rebase).
_Avoid_: reading `herdr --version` to tell which build is installed — it prints
bare upstream `0.7.5` either way, since the suffix is Homebrew's, not Cargo's.
Check the Cellar path or `brew list --versions`.
_Avoid_: "grove joins herdr's authority allowlist" — verified false. Allowlist
membership is a *stricter* path, not a fast lane: an allowlisted report must
pass `route_full_lifecycle_hook_report`, which needs the label to parse to the
detected agent and then requires a `session_ref` grove does not have, so every
report would be dropped.
_Avoid_: "grove needs a forked herdr" as a statement about the *loop* — only the
status surface depends on it. Under stock herdr the reports are dropped and the
pane falls back to screen detection, exactly as with no herdr at all.

**Authority release**:
grove's obligation to un-report **when it stops having an opinion about the
pane**. herdr never expires a hook authority — there is no TTL, and its
clear-on-process-exit path only fires for a label that parses to a known agent,
which `grove` does not. So grove's last report is what the pane shows until
something releases it. The loop driver releases on `complete --done` (after
reporting `idle`) and on **SIGTERM/SIGHUP** — and deliberately **does not**
release on a no-signal stop (`/exit`, Ctrl-C, a crash), on a version-skew stop,
or on an error: those report **`blocked`** and hold it, because the grove has
live leaves and genuinely needs a human. Releasing there would return the pane to
screen detection, which reads a parked grove as `idle` — herdr's derived `done` —
i.e. the very complaint the reporter exists to fix. SIGKILL, panic, OOM and power
loss stay uncovered by design: the pane pins at grove's last state and the user
recovers with `herdr pane release-agent`.
_Avoid_: "release on every catchable exit" — that was the route decision's
formulation and it is wrong; see the table in ADR *herdr-optional-ui*.
_Avoid_: "grove needs a SIGINT handler" — the driver already sets SIGINT to
`SIG_IGN` so it survives Ctrl-C and reaches the relaunch-vs-stop decision, so a
Ctrl-C arrives as an ordinary no-signal stop and reports `blocked`. SIGTERM
(with SIGHUP, the pane-close signal) is the case that needed new code.
_Avoid_: calling the uncovered case "latching" — that named a *different*,
now-dissolved failure where herdr dropped grove's later reports and froze the
pane mid-loop. [[Authority patch]] fixed that; grove can always correct itself
while it is alive.

**Session-boundary visibility** / **Turn hooks** (the two reporting mechanisms,
and why there are two):
The loop driver is the harness's **parent process**, so its whole observable
vocabulary is *session started* / *session ended, with or without a completion
signal*. It sees nothing **inside** a session — and a session that stalls
mid-session on a [[HITL]] question ends no session, so driver-level reporting
alone leaves it reading `working` forever. Hence a **second mechanism**: on every
**claude** launch the driver appends `--settings` with an inline JSON hook block
wiring four claude events to `grove-llm report-turn`, so the harness reports what
the driver cannot see. Two are **turn boundaries** — `UserPromptSubmit` ⇒
`working`; `Stop` ⇒ **`blocked` unless `$GROVE_SIGNAL_FILE` shows the task
completed on purpose** (the discriminator, which needs no new model contract
because `grove-llm complete` is already mandatory as every task's last action).
Two are the **mid-turn pair** — `Notification` (matched to the dialog types
claude raises only after six seconds of human silence) ⇒ `blocked`, and
`PostToolUse` ⇒ `working`. That one is a *pair* because granting a permission
fires no event of its own, so a mid-turn `blocked` needs a paired restore or it
pins the pane until the turn ends; `PostToolUse` is the only event claude fires
in between, which is why the restore is per-tool-call. A `done` disposition
silences every row a machine can fire, leaving the driver's idle-then-release the
last word. Injected **per launch, persisting nothing** (hooks are *unioned*
across settings sources, so grove contends with neither herdr's own installed
hook nor the user's), and **not injected at all** outside a herdr pane, so the
argv is then byte-identical to a grove without them. **claude only**: codex has
no turn-end hook event *and* persists hook trust per source-and-content-hash;
pi's herdr extension already reports full lifecycle but reports `idle` at turn
end — the same conflation. See ADR *herdr-turn-boundary-hooks*.
_Avoid_: claiming driver-level reporting closes the "stalled overnight on a
question" case — it closes the half where the session **ended**; the hooks close
the halves where the *turn* ended and where the turn is still running but a
dialog is waiting, both on claude.
_Avoid_: "the status surface stops at session boundaries" unqualified — true of
the driver's own reporting, and true end-to-end on codex and pi, but false for a
claude-hosted grove.
_Avoid_: "the turn hooks fire only at turn boundaries" — that was the pre-
mid-turn set of two, and the per-tool-call row is now by far the most frequent
site in the whole surface.
_Avoid_: reaching for `PermissionRequest` as the mid-turn block signal because it
sounds more direct — it is a **decisional** hook (exit 2 *denies the
permission*), and it fires the instant the prompt appears rather than waiting out
the idle check that makes "unattended" mean something.
_Avoid_: treating `grove-llm report-turn` as a verb the model calls. It is
machinery the injected hook runs; it must exit zero and print nothing, because a
`UserPromptSubmit` hook's stdout is injected into the conversation as context.

**Pane mis-detection**:
herdr labelling a `grove do` pane with the wrong agent — in practice `codex`,
whatever harness grove actually launched — so its screen manifests evaluate
against the wrong agent's UI. herdr identifies the agent from the pane's
foreground **process group**: it prefers the group *leader*, and only falls back
to scoring every process in the group when the leader is unrecognised. In a
grove pane the leader is `grove` itself, which herdr cannot identify, so the
fallback runs and a `codex`-named MCP helper can outrank the real harness. Bites
only where grove is not holding hook authority (before its first report; after
release), since a landed report takes precedence over detection. Undecided —
`herdr-pane-misdetection-k11`.
_Avoid_: "MCP servers inherit the harness's process group, so a codex MCP server
makes the pane read as codex" — true but the wrong half of the story, and it
was the version this grove's root brief carried. herdr **already** defends
against exactly that (upstream #161, fixed in v0.5.11, same claude+codex-MCP
shape); grove sitting at the head of the process group is what disables that
defence.

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
_Avoid_: **deleting** a leaf to abandon it (`git rm`, or just removing the file in a jj tree) — that lowers the max and the next `leaf-add` re-issues a live key. Use [[Pruning]] (`leaf-prune`); the mark is what keeps the counter monotonic.

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
4. The **repo** `Linkuistics/grove`, which since the skills graft holds two products — grove *and* the `linkuistics` / `testanyware` plugins under `plugins/` (`docs/adr/skills-monorepo.md`). So "grove's README / CHANGELOG / ADR set" now names artifacts shared with a second bounded context; this glossary covers only senses 1–3, and `CONTEXT-MAP.md` routes the rest.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove", "the grove repo".
