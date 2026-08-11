# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

Definitions only. How a seam works belongs to `docs/ARCHITECTURE.md` and the
specs under `docs/specs/`; this glossary names those seams rather than restating
them.

## Language

**Global skill provisioning** / **skill precedence**:
The `grove` binary's per-invocation sweep of its embedded `content/` into
**every installed harness's** personal global skill directory, before it tries
to own a working tree — every one, because an opaque configured command cannot
be traced to a single harness. `content/` stays canonical and the binary is the
only writer of these directories; the single embedded launcher
`content/prompts/continue.md` relies on that provisioning rather than inlining a
second copy of the methodology, which makes the provisioned skill a target
prerequisite Grove cannot itself verify.
_Avoid_: naming `start.md` or `retire.md` — those launcher prompts disappeared
with their lifecycle verbs, and `continue.md` is the only survivor.

**Complete finish cycle**:
The terminal, whole-grove sequence performed by a generated `finish` [[Leaf]]:
(1) promote durable artifacts from the briefs; (2) delete `.grove/` in a focused
commit whose message names the finish leaf's [[Work-item handle]]; (3) signal
the loop with `grove-llm complete --done`. Only the driver creates that leaf, a
live one is reused rather than duplicated, and it is never retired — its
addition and deletion cancel from the final tree. Teardown requires explicit
human confirmation and is the loop's only routine human gate; its mechanism is
the [[Finish transaction]]. Branch integration and working-tree removal remain
outside Grove.
_Avoid_: describing the finish as merging or deleting anything git-topological — that was the pre-v11 cycle.
_Avoid_: reading task-root absence as proof that a finish succeeded — at a
driver lifecycle transition it is always a fresh-tree fact, so a deletion that
committed before the driver observed `done` starts a *new* grove.
_Avoid_: "an interrupted finish always leaves that leaf live" — only a decline,
or an interruption the [[Finish transaction]] proved rolled back, leaves it
selectable for an explicit later resume; what recovery cannot classify stays
blocked and operator-recoverable instead.

**Finish transaction** (`FINISHING-<finish-handle>/`, built as
`PREPARING-FINISH-<finish-handle>-<attempt-identity>/`):
The fail-closed transaction owned by `grove-llm finish-commit` after explicit
finish confirmation: under the exclusive [[Tree access lock]] it evacuates the
task tree beneath a manifest-backed reserved witness, commits only the `.grove/`
deletions, and hands the root off to a same-device quarantine. Its point is that
`.grove/` stays visibly present and unwalkable throughout the uncertain
pre-commit window rather than temporarily resembling a fresh grove. Both
reserved prefixes are refused by every ordinary reader and mutator, and anything
recovery cannot classify as its own stays blocked and operator-recoverable. See
ADR *task-tree-transactions-fail-closed*.
_Avoid_: deleting `.grove/` before the commit boundary and treating repository
history as a rollback source — a process death exposes an indistinguishable
fresh-root shape.

**Grove name**:
The working-tree directory's basename — never a branch, a bookmark, or a
canonical layout — resolved **jj-first** from the closest repository marker
walking up, so a plain-git checkout nested under a jj tree stays git. The name
supplies the root brief (`# <name> — brief`) and the harness session name
(`<repo-basename>: <name> grove`).
_Avoid_: "the grove name equals the branch name" — grove reads no branch anywhere.
_Avoid_: describing resolution as git-with-jj-fallback — it is the other way
round, in repository detection and in every tree-mutation verb.

**Meta-grove**:
A grove whose subject *is* the grove machinery — this repo. The distinguishing
property is not the topic but the **process ancestry**: its build and test
commands run as descendants of the very session driving them, so they inherit
that session's control environment and can act on it. Read it as a
*test-environment* category, not a privilege level: a meta-grove gets no extra
authority, it merely has descendants that can spend the authority the session
already holds (`tests/env_hygiene.rs` exists for exactly this).
_Avoid_: "nested grove" — that is `grove` launched from inside another grove's session (two drivers, two trees). A meta-grove is one grove whose *code under test* happens to be grove.

**Loop control channel** (`GROVE_SIGNAL_FILE`):
The collision-resistant per-launch path the loop driver watches while its
harness child runs; its **appearance alone** ends the session, with content read
only to tell `Relaunch` from `Done` (self-driving-loop). The exact path also
names the active [[Session epoch]], so a descendant of a previous launch cannot
act on the next one. It is therefore **ambient authority**: the foreground
launch is the only site that sets it, and every other harness spawn must scrub
it.
_Avoid_: calling the path a credential or security token — any descendant can read or deliberately discard its environment. The session epoch prevents ordinary stale-loop behavior; it does not defend against a hostile local process.
_Avoid_: treating a redirected `cargo test` as evidence the guard works — a redirected run is safe by construction and passes with the guard removed. The acceptance test is a full run from a live pane with the real path in ambient env, verified absent afterwards.

**root-init** / **fresh-grove start**:
The mechanical first step of bare `grove` when the provided working tree has no
`.grove/`: before launching any agent the driver creates `.grove/`, the root
`BRIEF.md` stub, `01-requirements-plan-k1.md`, and the [[Tree format witness]],
then runs the ordinary authoritative [[Pick]] and launches that requirements
leaf through [[Grove configuration]]. There is no special rootless session —
every session owns a real selected leaf, and a grove begins with requirements
gathering by construction.
_Avoid_: "the bootstrap leaf is planning" — that was the pre-taxonomy answer, and it survives only by marking `planning` HITL again. The bootstrap session may still *cut* leaves (the requirements/design/planning fusion a small workstream is allowed); the label names the discipline that always applies.
_Avoid_: asking the first agent to run `grove-llm root-init`; the driver creates
the leaf it must select before the agent exists.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

**Task commit boundary** / **sealing**:
Where one session's focused commit closes. That commit covers the session's
whole task — the artifact, whatever the grow verbs wrote, the [[DONE infix]]
rename, and whatever the parent-chain close promoted or added — and names it by
the [[Work-item handle]], each closed node's alongside the leaf's. Everything in
that list is written by **Retire**, so **Retire precedes Commit** as a loop step
order, not merely as advice. In plain Git the commit *is* the boundary; in a
jj-enabled tree the working copy is itself a commit, so **sealing** with `jj
new`, once the rename has landed, is what leaves the next session its own empty
change.
_Avoid_: reading `jj describe` as the boundary — it records the task but leaves
`@` open, so the next session's first edit is snapshotted into this task's change,
and a file both tasks touched cannot be separated afterwards by
`jj split <fileset>`.
_Avoid_: the **Commit → Retire** order any prose or diagram may still show. In jj
it is the exact cross-task contamination sealing exists to stop; in Git it leaves
the rename and every close-time edit uncommitted or forces a second commit.

**Driver lease**:
The exclusive, process-scoped ownership of one working tree by one bare `grove`
driver: a nonblocking advisory lock on a control file in that exact workspace's
VCS-administration area, held with the open working-tree root through the whole
loop. Kernel release on return, panic, or process death makes restart ordinary
continuation while the task tree still exists; leftover bytes and PIDs carry no
ownership. See ADR *one-live-driver-per-working-tree*.
Acquisition is also where the [[Workspace layout preflight]] runs, making it the
one chokepoint every tree-creating and tree-driving path passes through.
_Avoid_: the [[Tree access lock]] — that shorter guard serializes one tree observation or mutation and must be released before foreground launch. A driver lease serializes the loop lifetime and lives on a separate control file in the VCS administration area.
_Avoid_: waiting for a contended driver lease — a second driver would issue duplicate mandates, so it is refused immediately rather than queued as an ordinary tree operation.

**Workspace layout preflight**:
The device comparison bare `grove` makes during [[Driver lease]] acquisition,
between the created workspace-control directory and the pinned working-tree root,
proving the layout can supply the atomic same-filesystem rename target the
[[Finish transaction]]'s quarantine handoff needs. Failure is a resumable
no-mutation stop, so an unfinishable workspace is named before it holds a task
tree rather than at the finish gate. `<workspace>/.jj/grove/` and a `.git/`
directory keep resolution inside the working tree; a `.git` **file** — a linked
worktree or submodule — is the only family whose devices can differ. See ADR
*supported-workspace-layouts*.
_Avoid_: reading it as a licence the [[Finish transaction]] may consult. It
compares proxies for operands that need not yet exist, the layout can change
while the lease is held, and `finish-commit` is separately invocable, so finish
revalidates independently.
_Avoid_: a durable capability marker recording that a workspace once passed —
layout is mutable and the marker is not.
_Avoid_: inferring support from the marker's kind alone. A symlinked `.git` or
`.jj`, or a control directory that is its own mount point, leaves the working
tree without changing that kind, so Grove measures every layout.

**Session epoch**:
The ephemeral launch-generation binding between one live [[Driver lease]], one
working-tree identity, and one collision-resistant [[Loop control channel]]
path, rewritten by the driver around every spawn. An ambient `grove-llm` tree
command must match the live record before it may touch the tree, so a stale
session cannot act on the current launch; it is workflow consistency among
cooperating processes, not authentication, and it does not prevent direct file
edits, commits, or forged signal writes. See ADR
*one-live-driver-per-working-tree*.
_Avoid_: a durable **grove generation** in `.grove/` or in a [[Work-item handle]]. Epoch rotation is stronger and catches stale sessions between every launch as well as after finish plus root recreation; stable handles remain identities within one task tree.
_Avoid_: inferring authority from the existence or bytes of a control file. The live kernel locks bind the record; unlocked leftovers mean nothing.

**Session kind**:
The launch-and-discipline label encoded only in a [[Leaf]] `.md` filename as
`NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md`. The kind is routing
metadata, not identity: the stable [[Work-item handle]] remains `<slug>-k<key>`.
The closed set has nineteen members — five producers (`requirements`, `design`,
`planning`, `prototype`, `impl`), each with its corresponding `review-` and
`integrate-review-` kind, plus `research-a`, `research-b`, `combine-research`,
and the driver-reserved `finish`. Every kind resolves one complete target
through [[Grove configuration]]; a missing or unknown kind is a malformed tree
that stops tree operations, never a degradation to `impl`. A [[Node directory]]
is kind-free by construction and is never passed through this parser.
_Avoid_: **task kind** — the label shapes and launches a session, and no longer
lives in a `**Kind:**` task-file field.
_Avoid_: plain `research` — a vendor pair uses the independently configurable
`research-a` and `research-b` kinds.

**Session-kind migration**:
The one-time, fail-closed conversion bare `grove` applies before ordinary
selection to every legacy task-tree shape — the older directory layouts and
current-layout leaves that still carry body markers — landing as one recoverable
transaction and one focused commit, never as a runnable mixed grammar. It
renames every deterministic leaf, live or terminal, and removes the obsolete
`**Kind:**`, `**Harness:**`, and `**Producer launch:**` lines while
preserving `Reviews` and `Integrates`. The positive `.grove/FORMAT` value is the
only legacy/current discriminator, and an unknown kind or structurally ambiguous
pair stops before mutation rather than guessing a configured target.

**Review chain** / **vendor pair**:
The two composition patterns over the [[Session kind]] set. The **review chain**
is `X` → `review-X` → `integrate-review-X`: sequential, **adversarial** (the
reviewer's job is to find fault), and each step a *different kind*, so per-kind
routing alone expresses it. The **vendor pair** is `research-a` → `research-b` →
`combine-research`: **breadth-and-confirmation**, two separately configured
survey sessions unioned by a binary combine step. Both are **flat siblings** in
their parent, named off a shared stem with a terminal step suffix (`<stem>` /
`<stem>-review` / `<stem>-integrate`, and `<stem>-a` / `<stem>-b` /
`<stem>-combine`), which keeps every step's [[Work-item handle]] unique and
still naming its artifact after `.grove/` dies.
**They are constructed in opposite ways, and the asymmetry is the design.** A
chain is **lazy**: each step is cut with `leaf-add` as the *last act* of the
session before it — a producer adds `review-<producer>` only if review is
required, a review adds `integrate-review-<producer>` only if it has findings
worth acting on, and a review that finds nothing creates nothing. The creating
session writes the new leaf's body, so it carries the specific case, finding or
datum a constructor could not have known. A pair is **eager**, one call, all or
nothing: lazy creation would let `research-b` inherit `research-a`'s framing and
corpus, destroying the independence the pair is run for.
The review chain is also the grove-scale counterpart to an **in-session doubt
cycle**: doubt is the cheap move for one narrow, unexpected decision that still
fits the current leaf, while a load-bearing artifact, repeated review cycle, or
multi-axis review belongs in the tree so a fresh session and [[Kind routing]]
choose its target. A picked Grove leaf may spend **at most one** in-session
reviewer across the whole leaf — a rule owned by a session the driver mandated
and that adopted the mandate by running [[Bootstrap]], never one that merely
found a `.grove/` directory — and a second review need is the mechanical signal
that review has become tree-sized work. The architecture's [task-kind
taxonomy](docs/ARCHITECTURE.md#task-kind-taxonomy) holds the reasoning and
`docs/specs/doubt-grove-review-mechanics.md` the ownership predicate.
_Avoid_: justifying any part of the tree scheme by what a grove-controlled
renderer would show. Grove ships no viewer; a claim that depends on one is a claim
about a component that does not exist.
_Avoid_: using many in-session doubt reviewers as a substitute for cutting a
review leaf; once review itself is substantial work, externalise it to the tree.
_Avoid_: letting the doubt skill launch a competing cross-model review after the
work has been escalated to Grove; that bypasses the review kind's routing policy.
_Avoid_: a *leading* step token (`review-<stem>`) — it sorts every review beside
every other review and scatters the chains the naming exists to reveal.
_Avoid_: "a chain gets a [[Node directory]] of its own" — reversed, and the three
arguments that once decided it have all lapsed again. What killed it is that the
hierarchy was not worth its navigation cost, and that a node species meaning
*these steps compose one artifact* forced every reader to carry a second sense of
node-ness. Deleting it collapses the species back to one, so **every node carries
a `BRIEF.md`** and no discriminator survives.
_Avoid_: an eager chain constructor (`leaf-add-chain`) or a retrofit verb
(`leaf-promote-chain`). Both are gone: the first would emit only a producer,
which is `leaf-add`, and the second existed solely to wrap a chain node around a
picked producer without changing its handle — which is why it needed a
fail-closed transaction. Escalating review is now one `leaf-add`.
_Avoid_: treating either shape as enforced — grove validates no ordering between
leaves, because a grammar is a relation *between* leaves and grove expresses
none (task-kind-taxonomy). The suffix convention is a habit nothing parses, and
so are the `**Reviews:**` / `**Integrates:**` lines: they are written by hand by
the session authoring the body, and **no code reads them**.
_Avoid_: expecting a chain's steps to stay contiguous. They are appended at the
parent's next free position, so one cut after unrelated work lands after it, and
a sibling `leaf-insert` can split a chain that was contiguous. Grove enforces
none of it.
_Avoid_: reading that as "adjacency never matters" — it is per hop, and the two
differ. A `review-*` step re-derives: its body names the producer's stable
handle, task commits name their work item by that handle, so it locates the
producer's commit and reads that diff against the current source, and nothing it
consumes had been written down for intervening work to stale. An
`integrate-review-*` step consumes citations the review already froze into prose,
against a working tree that has since moved, and the drift is **silent**. So an
integration is cut where [[Pick]] reaches it next — `leaf-insert` at **the first
sibling entry after the review whose subtree still holds live work**, an *entry*
because the walk descends a node in place, so a later sibling node with a live
descendant blocks and the **node** is the target rather than the leaf inside it.
Terminal leaves, wholly terminal nodes and the `finish` sentinel do not block,
and when nothing blocks `leaf-add` is right, because pre-order finishes the
review's own directory before any later sibling of an ancestor. It is a rule for
the session cutting the leaf, never something the tree guarantees.
_Avoid_: an exception for intervening work that "provably touches no file the
findings cite". Nothing can supply that proof at the moment the leaf is cut: the
intervening leaf has not run, and grove makes no leaf's eventual file set part of
its contract, so a goal or a pointer list is a guess wearing a check's clothes.
_Avoid_: giving either research leaf a `**Harness:**` declaration. Their
`research-a` and `research-b` session kinds are the configuration keys.
_Avoid_: running the *researchers* adversarially — that discards the breadth the
pair was run for. The adversarial move belongs to `combine-research`, whose
discipline is that **agreement without independent primary sourcing is a red
flag, not a confirmation**. None of a pair's three leaves runs an in-session
doubt reviewer: the two producers already supply independent corpora and the
combiner already supplies the adversarial move.

**HITL** / **AFK**:
Whether a [[Session kind]] resolves through live exchange with a human who
speaks for themselves (`requirements`, `prototype`, `finish`) or is driven by
the agent alone (every other kind). The generating rule: a kind is HITL when its
normal successful path requires human-originated input the session cannot supply
for itself. The mark **predicts, it does not permit**: *any* kind may stop and
ask a human, and doing so is always legitimate.
_Avoid_: an agent answering its own HITL questions — a `requirements` session
that grills itself has broken the distinction (`grilling.md`).
_Avoid_: "`planning` is HITL" — it flipped to AFK when grilling moved to
`requirements`. Planning keeps its methodological force but no longer
interrogates.
_Avoid_: attaching a *mandatory* question to a **loop step** rather than a kind —
every kind runs every step, so a question in the Retire or Commit step overrides
the mark by construction and stalls AFK kinds at a moment nothing in the tree
predicts.

**Confirmation boundary**:
The rule deciding which moments in the loop stop and ask a human, applied in two
ordered tests: **(1) does the answer change what is written?** — if every answer
leaves the same bytes on disk, do not ask; **(2) if it does, is the fact the
session's to establish or the human's to decide?** — a session can establish
*what it did*, never *what is worth doing*. [[DONE infix]] and a **node close**
ask nothing; [[Pruning]] and the [[Complete finish cycle]] ask. What replaced
the node-close confirmation is a **verify-and-report** obligation the session
discharges itself: check the node's brief `Done when` against what the subtree
delivered, `leaf-add` the missing work if the check fails and the gap can be
named, escalate if it cannot, promote what survives upward, and name the closed
node in the commit message. See the architecture's [human authority and
completion](docs/ARCHITECTURE.md#confirmation-boundary).
_Avoid_: "all retirements need confirmation" — it over-reads the design in both
directions. Marking a *leaf* done was never confirmed (mechanical bookkeeping);
only the node close changed.
_Avoid_: reading the removal as a friction argument. A node is **never marked**
([[Node directory]]), so the question gated an *inference*; a confirmation that
gates a judgement-bearing write ([[Pruning]]) is untouched.
_Avoid_: re-adding a per-ancestor question as the cascade recurses — that is the
wizard anti-pattern *in-session-finish-cycle* already rejects, and it terminated
into that cycle's own confirmation, giving up to four questions about one fact.

**Grove configuration** (`~/.config/grove/config.kdl`):
The sole source of Grove's user configuration: a flat document assigning each of
the nineteen [[Session kind]]s one complete command-template string, which
chooses the executable or wrapper and every user-controlled argument — harness,
model, reasoning effort, approval, permission, and sandbox policy included.
Templates use POSIX shell-word quoting but Grove invokes no shell: it parses the
template into argv, expands only its own substitutions (`${prompt}` exactly once
in any position after a literal word-zero executable, plus optional
`${session_name}`, `${worktree}`, and `${repo}`), and executes the result
directly, adding no hidden harness-specific argv fragments. A launch has no
precedence lattice, no defaults, families, profiles, or inheritance, and no
user-settable `GROVE_*` environment configuration — task files, flags,
repository-local stamps, and variables neither override nor supplement this
file, and there is no `GROVE_LLM_BIN` override. Grove never creates or edits it
because it cannot choose personal policy, and fully validates it before **any**
task-tree mutation and again before every launch, so a validation failure
launches nothing and leaves an existing selected leaf live and resumable.
_Avoid_: "primary harness" — harness selection is a property of each session kind,
not of the grove as a whole.
_Avoid_: "thinking effort" — use **reasoning effort**, the launch-policy term.
_Avoid_: a fallback chain — target resolution is one exact lookup.
_Avoid_: executing the template with `sh -c` or an interactive login shell — the
configured process remains Grove's direct foreground child.
_Avoid_: describing a diagnostic environment override as configuration; user
policy has one home.

**Kind routing**:
How the self-driving loop launches the [[Leaf]] selected by one authoritative
driver-side [[Pick]]. The driver reads the session kind from that leaf's
filename, obtains its complete session target from [[Grove configuration]], and
embeds the selected stable handle in `${prompt}` as the launched session's
mandate. The session first validates its [[Session epoch]], then resolves that
handle to its current path, rejects an unavailable or non-live result,
Bootstraps the resolved leaf, and does not pick again. A session started outside bare
`grove` has no mandate and is not a Grove loop session; Grove executes the
configured command directly and is not a model router or proxy.
_Avoid_: describing environment variables, a harness stamp, `--harness`, or a
leaf-level `**Harness:**` declaration as configuration fallbacks. Grove has one
configuration source.

**Review target diversity**:
Whether a scheduled review uses a different harness or model from its producer
is explicit policy in [[Grove configuration]]. Grove does not interpret command
templates to recover target identity, persist producer launch receipts, export
session-target metadata, compare targets, or warn; a `review-*` leaf supplies a
fresh session, and choosing a materially different command is the configuration
owner's responsibility.

**Spec** (`docs/specs/<slug>.md`):
The human-facing, team-shareable design of an *area* of the system — problem,
solution, settled decisions, agreed test seams, out-of-scope — written lazily by a
`design` task at a genuine agreement point, in a flow spanning four kinds: grill
(`requirements`) → spec (`design`) → decompose (`planning`) → execute (`impl`).
Like `docs/adr/`, `docs/specs/` is a **minimum coherent set describing the
design's current state**: slug-named, edited, merged and split in place, and
deleted once a spec describes nothing. Two rules bound the set — the
**membership test** (*would a session on an unrelated future grove need to read
this?* if not it is a [[Node directory]]'s `BRIEF.md` and dies with `.grove/`)
and the **grain rule** (an ADR records *one decision and its trade-off*, a spec
describes *how an area works* and **cites** the ADRs in its area rather than
restating them). Shape and the seam-recording rule: `content/SPEC-FORMAT.md`;
what a seam *is*: `linkuistics:codebase-design`.
_Avoid_: "PRD" — grove names no product-requirements artifact. _Avoid_ a
`## Decomposition` section inside a spec: that is brief material, dead when the
grove finishes.
_Avoid_: "a planning task writes the spec" — pre-taxonomy, from when `planning`
covered grilling and design both. `design` is the producer whose deliverable is a
spec, an ADR set, or both; `planning` consumes it and cuts the leaves.

### Task-tree scheme (v2 directories, task-tree-scheme)

**Tree format witness** (`.grove/FORMAT`):
The positive discriminator for the current session-kind filename grammar, with
exact LF-terminated content `session-kinds-v1\n`, written last by root
initialization and migration once the rest of the tree is complete. It is format
metadata inside the artifact tree, not a phase, status, or counter file; absence
means legacy and an unknown value means a newer or foreign format that stops
work without mutation.
_Avoid_: inferring current format by parsing a filename prefix — a legacy slug
such as `design-notes` makes that test silently change both kind and handle.

**Node directory** / **node**:
A grove tree node is a **directory** named `NN-<slug>-k<key>/` holding its numbered children (leaf files and child node directories), headed by a `BRIEF.md` charter; `.grove/` is itself the root node (its charter is `.grove/BRIEF.md`). The filesystem carries the hierarchy, so a name encodes only its *per-level* position — not a global path (task-tree-scheme).
**One species, and it always carries a charter.** A node is a leaf that proved
*bigger than one session*, so the charter is exactly the context those extra
sessions need, and the only writers are `leaf-decompose` (which moves the
decomposed leaf's own body in as the brief) and `root-init` for the root. Nothing
composes leaves into a node: a [[Review chain]]'s steps and a vendor pair's are
flat siblings. Every node close therefore has the same work — a `Done when`
rollup to check and a brief to promote.
_Avoid_: calling a node a "file" — a node is always a directory.
_Avoid_: a second, **brief-less** species meaning *these steps compose one
artifact*, and any `BRIEF.md`-presence **discriminator** that reads one. Both
went with the eager chain constructors; a reader that still tests for a charter
is deciding a question with one answer, and a `Done when` check that skips a node
for want of a brief silently drops a real rollup. A node with no charter is a
lapse to fix, not a species — though a *reader* still tolerates it, because a
brief is a lazy artifact (constraint 4) and nothing validates one (constraint 3).
_Avoid_: discriminating anything by a `-chain` / `-pair` token in a slug. No verb
writes one any more, and it was ordinary slug text a human may use for anything
even when they did.
_Avoid_: reading a **task-shaped** directory name Grove cannot parse — most
often one hand-marked `DONE` — as an ignorable foreign entry. Every positioned,
keyed name is Grove's at the species its `.md` suffix declares, and a directory
skipped takes its whole live subtree with it while [[Pick]] reports the grove
done; such a name is a malformed tree that stops reads and mutations. Names
outside that grammar stay foreign at either species, and the reserved
transaction witnesses are unpositioned, so none is reached by the rule.

**Leaf**:
A single unit of work — a file `NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md` inside a node directory, executed in one session. The only thing `pick` returns is a *live* leaf — one carrying **no outcome infix** at all. A leaf has exactly two terminal states: `DONE` (the work was done) and `ABANDONED` (the path was closed); see [[DONE infix]] and [[Pruning]].

**Pick** (`grove-llm pick`):
The loop's dispatcher: the **first
eligible live [[Leaf]] in depth-first pre-order** over `.grove/`, read from
filenames and never from task-file contents. A pending transaction witness
([[Finish transaction]], session-kind migration) is not a scheduling input: it is
a fail-closed malformed-tree condition every reader and mutator refuses until the
interrupted operation is recovered. Eligibility has one
lifecycle exception — a driver-owned `finish` sentinel is skipped while any
non-finish leaf is live — and among non-finish leaves nothing modulates the
walk: no priority, no grouping, no set of leaves that must finish before another
is considered. Ordering in a grove is
**position order**, at every level, and it is the only ordering grove offers:
what runs next is the next live leaf the walk reaches, exactly as for a
decomposition's children. Nothing holds a [[Review chain]]'s steps together —
a step cut after unrelated work lands after that work, and a sibling
`leaf-insert` can split a chain that was adjacent, so its steps are **not**
contiguous by construction and need not run consecutively. The flat shape accepts
that cost rather than defending against it, but not uniformly: an
`integrate-review-*` step consumes `path:line` citations its review already
froze, so it is cut where this walk reaches it next — `leaf-insert` at the first
sibling entry after the review whose subtree still holds live work — while a
`review-*` step re-derives everything it needs and is placed wherever `leaf-add`
lands it. Its value is that a
human computes it **by eye** — `find .grove` shows the tree and the next session
is the first non-finish name with no outcome infix — which is what makes "the
tree is the only state" worth something rather than merely true.
_Avoid_: expecting `pick` to **schedule** — to finish a group before considering an earlier live leaf, or to skip a leaf that is merely blocked. Answering "is a group in flight?" needs either state outside the tree (constraint 1) or a rule that skips live work and ranks groups, turning the walk into a scheduler no reader of `find .grove` can predict (task-tree-scheme, *`pick` is a walk, not a scheduler*).

**Position** (`NN`):
The **mutable** 2-digit zero-padded per-level locator of an entry among its directory's siblings — the sort input within one directory (lexical == numeric == DFS), rewritten on insert/reorder. It is a locator, **not** an identity.
_Avoid_: using a position (or a directory path) as a durable cross-reference — it moves under renumber. Reference by the [[Permanent key]] or [[Work-item handle]] instead.
_Avoid_: reading adjacency as a structural claim, or as a fact about how chains
are built. A [[Review chain]]'s steps are adjacent only when the session cutting
the next one made them so; an append lands at the parent's *end*, behind whatever
live work already sat there, and nothing refuses an insert between them either.
_Avoid_: the converse — concluding that because nothing holds them together,
where a step lands is free. It is free for a `review-*` step and not for an
`integrate-review-*` one, whose findings are anchored to positions in files an
intervening leaf can move without erroring. See [[Review chain]].

**Permanent key** / **stable id** (`-k<key>`):
The never-rewritten identity token of a leaf or node, always the **terminal** token before the extension/slash, assigned once as `max key in tree + 1` (the keys in the names *are* the counter — no counter file; **every finished leaf stays in the tree, `DONE` or `ABANDONED` alike**, so the max is always visible). `grove-llm resolve [n]` / `n` finds an entity's current path by key across any renumber, move, or slug edit.
_Avoid_: "position" as identity; reusing a retired key.
_Avoid_: **deleting** a leaf to abandon it (`git rm`, or just removing the file in a jj tree) — that lowers the max and the next `leaf-add` re-issues a live key. Use [[Pruning]] (`leaf-prune`); the mark is what keeps the counter monotonic.

**Work-item handle** / **title** (`<slug>-k<key>`):
The position-free in-file `# …` header of a task or brief (`# <slug>-k<key>`, or `# <slug>-k<key> — brief` for a node; the root brief is `# <grove name> — brief`) **and** the canonical way to name a work item in commit messages and prose (task-tree-scheme §5). Stable across renumber, because it omits the mutable position. `resolve` also accepts the full handle, not just the bare key.
_Avoid_: naming a work item by its position or directory path in a commit message.
_Avoid_: citing a **real** work item in provisioned `content/` at all — not even by a
correct handle. The handle resolves only inside the tree that issued it, a reader is
rarely in that tree, and it dies at the [[Complete finish cycle]]; anchor a worked
example on a durable artifact instead (task-tree-scheme §5). A *synthetic* name
illustrating the grammar is fine — there the position is the subject.
_Avoid_: citing an **index into** a work item — a review finding (`B5`), a numbered
decision, a bare key (`k29`) — by that index alone. The handle rule governs naming an
*item*, not indexing into one, so a bare index satisfies it in full and still resolves
nowhere; write the pair, `branch-review-k14 B5`. An unscoped index is **worse** than a
bare position: a position merely fails to resolve, while a number can resolve
*incorrectly* against an unrelated live series of the same shape (task-tree-scheme §5).

**Tree access lock**:
The process-scoped advisory lock every task-tree reader and mutator takes on an
open descriptor for the **working-tree root** before inspecting names — shared
for readers, exclusive for mutators. The root rather than `.grove/`, because it
exists before the task tree is created and through its deletion, so root
initialization, migration, finish allocation and deletion, and the agent tree
verbs all share one invariant. It serializes live processes but adds no crash
atomicity; the only operations that promise process-interruption recovery — the
[[Finish transaction]] and the one-time session-kind migration — each use their
own in-tree witness. A grow verb has neither and promises neither: it unwinds on
a *reported* error, and a process killed mid-run can leave a partial shape. See
ADR *task-tree-transactions-fail-closed*.
_Avoid_: locking `.grove/BRIEF.md` — root briefs are lazy, optional artifacts,
and existing tree readers deliberately tolerate their absence.
_Avoid_: locking `.grove/` itself — it cannot serialize either its own creation
or its finish deletion, and adding a second lifecycle lock creates an ordering
contract instead of one seam.

**DONE infix**:
The in-place retirement marker: the literal `DONE` placed right after the
position in a retired leaf's filename
(`NN-DONE-<session-kind>-<slug>-k<key>.md`), written by `leaf-retire`. Leaves
only — a node is never marked done (its done-ness is the absence of a live leaf
in its subtree, however those leaves finished). The retired leaf keeps its
position and key, and **its own contents remain untouched**. Its sibling mark is
`ABANDONED` ([[Pruning]]); the two are the only terminal leaf states.
_Avoid_: moving a retired leaf into a separate folder or list — retirement is in place, so the tree always shows complete state.

**Pruning** / **ABANDONED infix** (`leaf-prune`):
Marking a work path **decided against** — as opposed to done — with an
`ABANDONED` infix written in place by `grove-llm leaf-prune`, exactly as
`leaf-retire` writes `DONE`; `pick` skips both, and neither ever leaves the
tree. grove's metaphor names the pair: a `DONE` leaf is **harvested**, an
abandoned one is **pruned**. Deciding against a path is a normal outcome of
exploration, but it is **[[HITL]]**: an agent never prunes on its own, and it
clears both of the [[Confirmation boundary]]'s tests — the mark is a real write,
and what it asserts is the human's to decide. Given a **node** it marks every
*live* leaf in that subtree and refuses the grove root; because `.grove/` dies at
the [[Complete finish cycle]], the mark records only *that* a path was closed
and the durable *why* goes to the **ADR set**. See the architecture's [human
authority and completion](docs/ARCHITECTURE.md#pruning).
_Avoid_: reading "pruned" in git's sense (`git remote prune` = *delete*) — a pruned
leaf **stays in the tree**; that is the entire point, since the keys in the names are
the counter ([[Permanent key]]).
_Avoid_: a taxonomy of outcomes (`blocked` / `deferred` / `superseded`). `blocked` is
expressed by **ordering** and would break the finish trigger if `pick` skipped it (a
blocked leaf is *live* work); `deferred` is a reorder or a GitHub issue; `superseded`
differs only in *reason*, which is prose and belongs in the ADR, not the filename.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — invoked as bare `grove`, with no subcommands or flags.
2. The **methodology** embedded in `content/SKILL.md` and provisioned to the global skill dir.
3. A single **workstream** — one named task tree at `.grove/` inside the working tree the user provides, named for that working tree's basename ([[Grove name]]), no canonical path. The `grove` command operates on this sense.
4. The **repo** `Linkuistics/grove`, which holds two products — Grove and the
   `linkuistics` / `testanyware` plugins under `plugins/` (see
   [repository products](docs/ARCHITECTURE.md#skills-monorepo)). So "Grove's
   README / CHANGELOG / docs" names artifacts shared with a second bounded
   context; this glossary covers only senses 1–3, and `CONTEXT-MAP.md` routes
   the rest.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove", "the grove repo".

**"chain"** carries two unrelated senses, and one of them is already a verb name:

1. The **brief chain** — a leaf's ancestor `BRIEF.md` files, root→leaf, which
   [[Bootstrap]] reads and `grove-llm brief-chain` prints. A relation between a
   leaf and its *ancestors*.
2. The **[[Review chain]]** — `X` → `review-X` → `integrate-review-X`, a step
   sequence over one artifact. A relation between *sibling* leaves.

Neither is derivable from the other and both are load-bearing, so qualify every
use ("brief chain", "review chain"). Only sense 1 heads a verb name
(`brief-chain`), and sense 2 no longer names any verb at all: a review chain is
built with plain `leaf-add` calls, so there is nothing left for a bare "chain" to
head ambiguously.
