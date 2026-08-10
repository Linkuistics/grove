# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Global skill provisioning** / **skill precedence**:
The `grove` binary embeds `content/` and, on every bare lifecycle invocation,
sweeps it out to **every installed harness's** personal global skill dir before
trying to own a working tree — `provision_all` writes each target whose home
root (`~/.claude`, `~/.codex`, `~/.pi`) already exists, each via
`skill_dir_for(harness)` (`$HOME/<harness.skills_dir>/grove`; pi nests under
`agent/`) — idempotent per target against a content-hash stamp. A refused second
driver therefore still refreshes the independently delivered methodology.
Standard `grove --help` and `grove --version` are metadata-only and perform no
provisioning, repository discovery, or locking. `content/`
stays canonical; the binary is the only writer of these dirs. The single
embedded launcher is `content/prompts/continue.md`, augmented with the selected
leaf's stable-handle mandate; it tells the configured target to use this
provisioned skill. `start.md` and `retire.md` disappear with their lifecycle
verbs. Because Grove treats the configured command as opaque, exposing the
provisioned skill is a target prerequisite it cannot infer or validate.
`provision_target` guards each write: a symlink is replaced as a link, never
deleted through; a dir with no grove stamp and real content is refused outright,
so a foreign dir can never be silently clobbered.

**Complete finish cycle**:
The terminal, whole-grove sequence performed by a generated `finish` [[Leaf]]:
(1) promote durable artifacts from the briefs; (2) delete `.grove/` in a focused
commit whose message names the finish leaf's [[Work-item handle]]; (3) signal
the loop with `grove-llm complete --done`. After configuration has passed full
validation, an empty driver pick mechanically appends one working-tree-only
`finish` leaf at the grove root using the next position and permanent key, then
runs the ordinary pick/config/launch path again. Only the driver may create this
kind. A live finish leaf is reused rather than duplicated. A declined finish, or
one interrupted before `finish-commit` begins, exits without a completion signal
and leaves that leaf live for an explicit later resume. Once teardown begins,
the [[Finish transaction]] keeps the task root present until the deletion commit
is proven and the atomic quarantine handoff succeeds; task-root absence never
classifies an attempted finish as success. If later non-finish work appears,
[[Pick]] skips finish until that work is terminal, so the sentinel cannot starve
it. After confirmation,
`grove-llm finish-commit <finish-handle>` revalidates under the exclusive
[[Tree access lock]] that the
same finish is live and no non-finish work appeared after launch; refusal leaves
the tree untouched for the next driver iteration. The helper cannot attest the
human decision through an opaque command — confirmation remains the HITL
session's obligation. On success no commit is made *for* the finish
leaf: its addition and deletion cancel from the final tree, although Jujutsu or
a broad Git task commit may retain an intermediate snapshot in history. The
leaf is never retired. `leaf-retire`, `leaf-prune`, `leaf-decompose`, and
`leaf-promote-chain` refuse a finish operand; `leaf-insert` may target it to put
ordinary work before teardown. The deletion
commit is scoped to `.grove/` in Git and jj, leaving every unrelated staged,
working-tree, or working-copy change outside it. Teardown still requires explicit
human confirmation; it is the loop's only routine human gate. Branch integration
and working-tree removal remain outside Grove.
If the helper result is lost, a retry distrusts `.grove/` task-root absence and
instead verifies the exact immediate handle-and-attempt-named,
`.grove/`-scoped Git or jj commit it could have produced. The opaque attempt
identity is the active [[Session epoch]] launch nonce. With the
witness already gone, the proof is self-contained in that immediate result: the
exact handle-and-attempt message, a diff that only deletes `.grove/` from its
own parent, and no tracked task root in the result. It never requires the
working-tree-only finish leaf in the parent. A match is idempotent helper
success only for the same still-active launch; no match never licenses `done`. This narrow
command-outcome proof is not lifecycle state for a later bare driver. It is
reachable only while the task root is absent, and a new grove or replacement
launch has a different attempt even if it reuses the handle. A
no-signal exit remains a no-signal stop after successful deletion: the driver
reports the child's status and elapsed time rather than inferring confirmation
from absence. If the driver itself dies before observing
`done`, a successor first invalidates the old epoch; an orphaned shared guard may
make that attempt stop `blocked` at the handoff bound. Once handoff succeeds, a
later bare invocation treats the absent task root as a fresh grove and
initializes new requirements; no VCS-history heuristic or unlocked control file
acts as a rootless-driver finish receipt. Recovery intent and new-work intent are
observationally identical without another input or durable marker, both excluded
by the single-command and artifact-only contracts. Reused handles belong to the
new task tree and old cooperating sessions are rejected by [[Session epoch]]
rotation.
_Avoid_: describing the finish as merging or deleting anything git-topological — that was the pre-v11 cycle.

**Finish transaction** (`FINISHING-<finish-handle>/`, built as
`PREPARING-FINISH-<finish-handle>-<attempt-identity>/`):
The fail-closed transaction owned by `grove-llm finish-commit` after explicit
finish confirmation. Under the exclusive [[Tree access lock]], it validates the
live finish, opens and identity-revalidates `.grove/` itself as a real no-follow
directory, writes a manifest-backed reserved witness inside it, and
evacuates every ordinary root entry beneath that witness. The witness is built
under the `PREPARING-FINISH-` name — created *before* repository preparation, so
every auxiliary an adapter writes is already owned on disk by a named handle and
attempt — and published by one atomic rename to the `FINISHING-` name once its
manifest and ready marker are written. Both prefixes are reserved and refused by
every ordinary reader and mutator; a preparing witness never holds an evacuated
entry, because publication precedes evacuation, so recovery discards it by
aborting that repository preparation and fails closed on anything it cannot
classify as its own. Git or jj then commits
only the deletions at the original paths, excluding the witness. The task root
therefore remains visibly present and unwalkable throughout the uncertain
pre-commit window rather than temporarily resembling a fresh grove.

Preflight requires a non-empty tracked deletion fingerprint, an untracked
witness prefix, and a same-device workspace-control quarantine; a wholly
untracked task tree, including one ignored before it was recorded, is refused
unchanged because it has no focused deletion to record. The manifest's
root-entry digest is recursively defined and no-follow: SHA-256 covers
length-delimited path/type/mode records, directories hash raw-name-byte-ordered
children, files hash bytes, and symlinks hash their link-target bytes; other
entry types are refused before mutation. For jj, the start anchor includes the
exact preflight commit ID, current working-copy change, and its parents; a
partial commit succeeds only when that change becomes the exact teardown parent
of a new successor, while rollback
requires that the current working-copy commit itself still be the recorded
change at those parents. Colocated jj preserves the user's Git index before any
preflight snapshot can export into it.

Evacuation and rollback move one entry at a time, so an interruption leaves an
arbitrary prefix moved. Recovery locates every manifest entry — each must sit in
exactly one of the task root and the recovery tree — instead of matching the two
extreme shapes, so a moved prefix is an ordinary recoverable state whose retry
restores whatever remains. An entry in both places or in neither, an unrecorded
recovery-tree entry, and a foreign root entry all stay fail-closed and named.

Every ordinary tree reader and mutator refuses the witness. Recovery runs before
format, liveness, or missing-root classification and asks the repository seam
whether the exact immediate handle-and-attempt-named, `.grove/`-scoped commit exists. With
no proof, it first proves the recorded Git/jj starting topology still holds,
then restores any Git/colocated-jj index backup before restoring the exact live
finish tree; a failure of any proof or restoration leaves the witness in place
as **Recovery pending**. That diagnostic names the recorded and observed
topology, the artifact that holds the blocked transaction, and asks the operator
to preserve divergent work, restore either the
exact start or the exact teardown result, and retry; Grove never rewrites the
history automatically. With commit proof, it never resurrects the tree:
it finishes repository cleanup and atomically renames the entire task root,
witness intact, into a preflighted same-device workspace-control quarantine
before descriptor-rooted recursive disposal that unlinks rather than follows
symlinks. A workspace whose control directory cannot provide that atomic target
is refused before mutation.
Repository classification is revalidated immediately before and after rollback
or quarantine handoff. The witness is removed only after the restored repository
reproduces its exact start; a changed forward result atomically returns the
quarantine to `.grove/` and remains blocked, and a return that cannot complete
names both that change and the quarantine still holding the tree.
VCS-administration index images and post-commit quarantine are auxiliary cleanup,
never rootless workflow state. The helper attempts cleanup immediately and a
later lease-owning driver reaps orphaned entries carrying a valid Grove cleanup
manifest only when no matching in-tree witness owns them, without using cleanup
bytes for lifecycle classification. That sweep is over manifests, never over the
reserved staging namespaces: a staged entry stranded before its state document is
*attributable* but not proven Grove's, and the post-copy substitution refusal
deliberately leaves a foreign entry of the same shape, so both stay. Grove
narrows those windows instead — the colocated index filter's private staging
directory is released before the first publication boundary — and accepts the
bounded leak. Plain Git disables hooks for this internal
commit
because an index backup cannot reverse arbitrary hook side effects. See ADR
*task-tree-transactions-fail-closed*.
_Avoid_: deleting `.grove/` before the commit boundary and treating repository
history as a rollback source — a process death exposes an indistinguishable
fresh-root shape.

**Grove name**:
The working-tree directory's basename — never a branch, a bookmark, or a canonical layout. Resolved **jj-first**: a `.jj/` directory heading the tree makes it jj-enabled (native, secondary workspace, or colocated — `.jj/` wins even with a `.git` beside it) and the workspace root is the directory holding `.jj/`; only a not-jj-enabled tree falls back to `git rev-parse --show-toplevel`. The closest marker walking up decides, so a plain-git checkout nested under a jj tree stays git. The name supplies the root brief (`# <name> — brief`) and the harness session name (`<repo-basename>: <name> grove`); Grove stores no repository-local harness binding or stamp.
_Avoid_: "the grove name equals the branch name" — grove reads no branch anywhere.
_Avoid_: describing resolution as git-with-jj-fallback — it is the other way round, in `repo.rs` and in every tree-mutation verb (a jj tree renames with `fs::rename`; `git mv` is the fallback).

**Meta-grove**:
A grove whose subject *is* the grove machinery — this repo. The distinguishing property is not the topic but the **process ancestry**: its build and test commands run as descendants of the very session driving them, so they inherit that session's control environment and can act on it. Ordinary groves have the same variables in scope and no reason to write them, which is why hazards of this class are invisible everywhere else and routine here (guard-loop-signal-k37). Read it as a *test-environment* category, not a privilege level: a meta-grove gets no extra authority, it merely has descendants that can spend the authority the session already holds.
_Avoid_: "nested grove" — that is `grove` launched from inside another grove's session (two drivers, two trees). A meta-grove is one grove whose *code under test* happens to be grove.

**Loop control channel** (`GROVE_SIGNAL_FILE`):
The collision-resistant per-launch path the loop driver watches while its
harness child runs;
its **appearance alone** ends the session (grace → SIGTERM → kill-grace →
SIGKILL), with content read only to tell `Relaunch` from `Done`
(self-driving-loop). The path lives in the working tree's VCS-administration
control directory and carries a fresh 128-bit suffix from the OS cryptographic
randomness source — never a PID, time, address, iteration, or task key. A driver
removes the path after post-reap epoch invalidation; a replacement removes
abandoned signal files only after taking the lease and exclusively invalidating
the old epoch. An occupied draw is retried; after cleanup, literal cross-restart
non-reuse has the accepted one-in-`2^128` collision bound rather than a durable
tombstone. It is therefore **ambient authority**: the foreground launch is
the only site that sets it, and every other harness spawn must scrub it. The
exact path also names the active [[Session epoch]]: an ambient `grove-llm` tree
operation must match the live driver's epoch before it can touch the tree, so a
descendant of a previous launch cannot act on the next one. The path remains
inherited rather than secret — this is freshness and process-ownership binding,
not authentication. In a [[Meta-grove]] the suite is such a descendant, so its
guards are structural: `.cargo/config.toml`'s `[env]` override with `force =
true` plus the shared `tests/support` scrub list, asserted by
`tests/env_hygiene.rs`.
_Avoid_: calling the path a credential or security token — any descendant can read or deliberately discard its environment. The session epoch prevents ordinary stale-loop behavior; it does not defend against a hostile local process.
_Avoid_: treating a redirected `cargo test` as evidence the guard works — a redirected run is safe by construction and passes with the guard removed. The acceptance test is a full run from a live pane with the real path in ambient env, verified absent afterwards.

**root-init** / **fresh-grove start**:
The mechanical first step of `grove` when the provided working tree has no
`.grove/`. Before launching any agent, the driver creates `.grove/`, the root
`BRIEF.md` stub, `01-requirements-<slug>-k1.md` (default slug `plan`), and the
[[Tree format witness]], then runs the ordinary authoritative [[Pick]] and
launches that requirements leaf through [[Grove configuration]]. The universal
[[Tree access lock]] serializes the whole scaffold; `FORMAT` lands last and an
exact partial scaffold is restart-completed rather than launched. There is no
special rootless session: every session owns a real selected leaf, and a grove
begins with requirements gathering by construction. The first requirements
session's commit folds the scaffold in.
Full configuration validation precedes this mutation, so a missing or malformed
personal config leaves a rootless working tree byte-identical.
Task-root absence is authoritative even when VCS history contains a completed
Grove teardown. History proves that an earlier task tree ended but cannot say
whether the current bare invocation intends recovery or a new grove;
interpreting it would need a second lifecycle input or a persistent finish
tombstone. A new scaffold may therefore reuse `plan-k1`; the new [[Session
epoch]] keeps that identity disjoint from stale cooperating processes belonging
to the deleted task tree.
_Avoid_: "the bootstrap leaf is planning" — that was the pre-taxonomy answer, and it survives only by marking `planning` HITL again. The bootstrap session may still *cut* leaves (the requirements/design/planning fusion a small workstream is allowed); the label names the discipline that always applies.
_Avoid_: asking the first agent to run `grove-llm root-init`; the driver creates
the leaf it must select before the agent exists.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

**Driver lease**:
The exclusive, process-scoped ownership of one working tree by one bare `grove`
driver. After independent methodology provisioning, the invocation reports
resolves the working tree, opens its root, and nonblockingly locks a control
file derived from the closest on-disk VCS marker: the current
workspace's `.jj/grove/` without following its shared-repository link, or the
canonical per-worktree Git directory from its `.git` directory/gitfile, never
Git's common directory. The resolver invokes no VCS discovery and ignores
`GIT_DIR`, `GIT_WORK_TREE`, and other ambient selectors; no `TMPDIR` or per-user
runtime namespace participates.
An alias reaches the same lease, while a different Git worktree or jj workspace
does not. Acquisition and every later lifecycle/launch transition revalidate
the locked descriptor's device/inode against the path; a replacement race gets
bounded retries, and loss by external VCS-administration mutation stops the
driver visibly. The lease record carries a per-process 128-bit nonce from OS
cryptographic randomness. The driver holds the root and lease through the whole
loop. Kernel release on return, panic, or process
death makes restart ordinary continuation while the task tree still exists;
leftover bytes and PIDs carry no ownership. A successfully deleted `.grove/`
task tree is no longer a grove to continue: after epoch handoff, a later bare
invocation follows [[root-init]] as a new workstream. The descriptors are
close-on-exec, so an opaque configured command cannot leak ownership into a
helper. See ADR
*one-live-driver-per-working-tree*.
_Avoid_: the [[Tree access lock]] — that shorter guard serializes one tree observation or mutation and must be released before foreground launch. A driver lease serializes the loop lifetime and lives on a separate control file in the VCS administration area.
_Avoid_: waiting for a contended driver lease — a second driver would issue duplicate mandates, so it is refused immediately rather than queued as an ordinary tree operation.

**Session epoch**:
The ephemeral launch-generation binding between one live [[Driver lease]], one
working-tree identity, and one collision-resistant [[Loop control channel]]
path. A driver
writes the stable per-workspace epoch file at three exclusive-guarded points:
inactive after lease acquisition, active before every spawn, and inactive after
every reap before reading the signal. Each guard is a separate scope and is
released before another guard, tree operation, or spawn. An ambient `grove-llm`
tree command holds a shared guard for its whole operation, validates the exact
record, and probes liveness with a nonblocking exclusive attempt on the lease;
successful acquisition is closed immediately and means stale, while contention
plus the matching nonce means a driver is live. The shared epoch guard closes
the probe race by forcing a replacement to wait for an already-admitted
operation. Every shared or exclusive epoch acquisition identity-revalidates its
opened-and-locked path and retries an open/lock replacement race. Acquiring the
shared guard is admission: an old call admitted before exclusive invalidation
may finish and block handoff, while calls begun after invalidation fail. Every
epoch acquisition emits one contention diagnostic and has a
fixed internal 30-second bound. If an orphan keeps a shared guard after reap,
the driver times out and stops `blocked` without consuming the signal or
launching again; restart continues after that operation releases. Descriptors
stay close-on-exec; the signal path is the only exported value, and the record
carries no leaf, kind, model, or hidden target. Manual commands with no loop-
control context remain available. The epoch prevents stale access through
`grove-llm`, not direct file edits, commits, or forged signal writes; it is
workflow consistency, not authentication. The finish transaction also uses the
active launch nonce as an opaque attempt identity and records it beside
the stable handle in its internal deletion commit; only a rootless retry under
that same active epoch accepts it. The historical nonce is audit data, not a
credential or later-driver finish receipt. See ADR
*one-live-driver-per-working-tree*.
_Avoid_: a durable **grove generation** in `.grove/` or in a [[Work-item handle]]. Epoch rotation is stronger and catches stale sessions between every launch as well as after finish plus root recreation; stable handles remain identities within one task tree.
_Avoid_: inferring authority from the existence or bytes of a control file. The live kernel locks bind the record; unlocked leftovers mean nothing.

**Session kind**:
The launch-and-discipline label encoded only in a [[Leaf]] `.md` filename as
`NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md`. The kind is routing
metadata, not identity: the stable [[Work-item handle]] remains `<slug>-k<key>`.
The leaf parser separates exactly one member of the closed kind set. No kind
label plus `-` prefixes another kind label, so rendered names remain
unambiguous and round-trip without changing the stable slug. A [[Node directory]]
is kind-free by construction and is never passed through this parser, including
when its slug begins with a kind token. The closed set
has nineteen members: five producers — `requirements`, `design`, `planning`,
`prototype`, and `impl` — each with its corresponding `review-` and
`integrate-review-` kind, plus `research-a`, `research-b`,
`combine-research`, and `finish`. `research-a` and `research-b` share the research discipline
but are distinct configuration keys, so a vendor pair needs no harness metadata
in either task file. Every session kind resolves one complete target through
[[Grove configuration]]. Every task-shaped `.md` leaf filename — live, `DONE`,
or `ABANDONED` — must carry a known kind. A missing or unknown kind is malformed
and stops tree operations with the path and valid kind set; it never degrades to
`impl`. Terminal validation is load-bearing for permanent-key allocation and
filename-only viewers. Foreign non-task files remain ignored. `finish` is
driver-reserved: generic grow verbs refuse creating it; retire, prune,
decompose, and promotion refuse it as an operand; insertion may target it with a
non-finish kind. The driver and reader still share the same closed set.
_Avoid_: **task kind** — the label shapes and launches a session, and no longer
lives in a `**Kind:**` task-file field.
_Avoid_: plain `research` — a vendor pair uses the independently configurable
`research-a` and `research-b` kinds.

**Session-kind migration**:
The one-time, fail-closed conversion `grove` applies before ordinary selection
to every legacy task-tree shape. If the tree also uses the original `NNN-slug/`
or v1 flat dotted-decimal layout, directory migration is the first phase and
session-kind filenames are the second; both phases land as one recoverable
transaction and one focused commit, never as a runnable mixed grammar.

Migration renames every deterministic leaf, live or terminal, and removes the
obsolete `**Kind:**`, `**Harness:**`, and `**Producer launch:**` lines while
preserving `Reviews` and `Integrates`. A missing legacy kind takes the old
read-side default `impl`; `work`, `review-work`, and `integrate-review-work` map
to their `impl` spellings; the two structural research children of a vendor pair
map to `research-a` and `research-b`, while a standalone `research` maps to
`research-a`. An unknown kind or structurally ambiguous pair stops before
mutation with actionable guidance rather than guessing a configured target.

The positive `.grove/FORMAT` value `session-kinds-v1` is the only
legacy/current discriminator. Without it the tree is legacy even when a slug
begins with `design` or another current kind token; migration parses the whole
legacy name and body marker, then writes the witness last. A known witness means
current, and an unknown value stops without mutation.

Full config validation precedes migration. Process interruption never exposes a
runnable mixed grammar: the next invocation either recovers the conversion or
refuses tree work with an actionable recovery diagnostic. The design owns the
transaction mechanism and whether it reuses or generalises the existing
promotion seam. The resulting commit is path-scoped to `.grove/` in plain Git
and fileset-scoped in jj, leaving unrelated dirty Git changes or jj working-copy
changes outside the migration commit.

**Review chain** / **vendor pair**:
The two composition patterns over the [[Session kind]] set. The **review chain**
is `X` → `review-X` →
`integrate-review-X`: sequential, **adversarial** (the reviewer's job is to find
fault), and each step is a *different kind*, so per-kind routing alone expresses
it. The **vendor pair** is `research-a` → `research-b` → `combine-research`:
**breadth-and-confirmation** (two separately configured survey sessions,
unioned). `leaf-add-pair <parent> <stem>` writes the three kinds with no harness
flags. The configuration owner may choose materially independent targets, but
Grove cannot enforce that property by comparing opaque command strings; the
tree guarantees two sessions and the combine discipline, not two vendors.
The fan is a **pair**, not an N-way fan-out, so the combine step is binary.
Both are **a [[Node directory]] of their own** — `NN-<stem>-chain-k<key>/` and
`NN-<stem>-pair-k<key>/` — holding their steps as children, **still named off the
shared stem with a terminal step suffix**: `<stem>` / `<stem>-review` /
`<stem>-integrate`, and `<stem>-a` / `<stem>-b` / `<stem>-combine`. The directory
is what makes the group **structural** rather than merely contiguous, so *any*
tree viewer — `yazi`, Finder, `ls -R`, `find .grove` — shows it as one collapsible
object without being taught anything. That is the whole reason it beat the flat
shape it replaced, and it survives grove shipping **no** viewer of its own: the
grouping is a property of the filesystem, so no renderer has to be provisioned,
configured, or kept in step for it to read.
_Avoid_: justifying any part of the tree scheme by what a grove-controlled
renderer would show. Grove ships no viewer; a claim that depends on one is a claim
about a component that does not exist.
The stem-and-suffix names **survive inside the node**, on a *different*
justification than the one that created them. They no longer exist to make a
chain legible from a flat listing — the directory does that now. They exist to
keep every child's [[Work-item handle]] unique and self-describing: `resolve`
matches a bare slug exactly and reports >1 as ambiguous, and `.grove/` dies at the
[[Complete finish cycle]] leaving commit messages as the only record, so
step-only children (`review-k4`) would collide across chains *and* name no
artifact in the history that outlives the tree. The node's `-chain` / `-pair`
token exists for the same reason: without it the node's slug collides with its
own first child's.
The convention remains a **habit nothing parses**; the guidance a session reads
while cutting leaves carries it (`content/SKILL.md`'s Decompose step,
`content/TASK-FORMAT.md`, and the bootstrap prompt), and the architecture's
[task-kind taxonomy](docs/ARCHITECTURE.md#task-kind-taxonomy) holds the reasoning.
**Sequencing and construction are separate questions with opposite answers.**
Sequencing gets no mechanism — see the _Avoid_ on units below. Construction gets
one call per shape (`leaf-add-chain` / `leaf-add-pair`), because deriving
`review-<producer>` and `integrate-review-<producer>` from a producer kind is a
derivation grove owns rather than a judgement, and the escalation call stays with
the caller (*Constructing a chain is one call*; the bar a verb must clear is ADR
*cli-binary-split*).
The review chain is also the grove-scale counterpart to an **in-session doubt
cycle**: doubt is the cheap move for one narrow, unexpected decision that still
fits the current leaf, while a load-bearing artifact, repeated review cycle, or
multi-axis review belongs in the tree so a fresh session and [[Kind routing]]
choose its model and harness. The distinction is orchestration scale, not whether
the review is adversarial — both are. A picked Grove leaf may spend **at most
one** in-session reviewer across the whole leaf; one reviewer means one
independently materialised fresh context, so a diverse-lens pass with N subagents
spends N reviewers. A second review need is the mechanical signal that review has
become tree-sized work.
This rule applies only when the Grove driver launched the current session with
an explicit selected-leaf mandate in `${prompt}` and the session adopted that
mandate by running [[Bootstrap]] — never merely because a `.grove/` directory
exists or a process inherited Grove control variables. The prompt-visible
Bootstrap-and-mandate fact replaces the old session-side-pick discriminator.
An allowed reviewer that finds a substantive actionable issue normally creates
that second need after the producer changes; promote the producer atomically,
finish it only to a reviewable boundary, then retire and signal so Grove launches
the fresh review leaf. Trivial findings, noise, a visible accepted trade-off, or
a fix conclusively covered by an executable test seam do not force promotion.
Once escalated, Grove launches the review kind's explicitly configured command.
Whether that target differs from the producer is visible configuration policy,
not something Grove infers or warns about.
_Avoid_: using many in-session doubt reviewers as a substitute for cutting a
review chain; once review itself is substantial work, externalise it to the tree.
_Avoid_: letting the doubt skill launch a competing cross-model review after the
work has been escalated to Grove; that bypasses the review kind's routing policy.
_Avoid_: a *leading* step token (`review-<stem>`) — it sorts every review beside
every other review and scatters the chains the naming exists to reveal.
_Avoid_: "a chain gets no node directory of its own" — reversed. The three
arguments that decided it have all lapsed: `leaf-add-chain` / `leaf-add-pair` are
proactive shape-emitting verbs, so node creation is no longer `leaf-decompose`'s
*reactive* monopoly; a chain node is **brief-less by rule**, so it buys no
`BRIEF.md` any step had to earn; and the cascade confirmation it was said to
introduce was never asked of a brief-less node, and is now asked of no node at all
([[Confirmation boundary]]). What survives of the old reasoning is
that node-ness now means two things — which the `BRIEF.md` discriminator keeps
legible rather than overloaded ([[Node directory]]).
_Avoid_: treating either as enforced — grove validates no ordering between
leaves, because a grammar is a relation *between* leaves and grove expresses
none (ADR *task-kind-taxonomy*).
_Avoid_: treating a chain as a **unit** in [[Pick]]'s sense — something the walk
will not leave. Containment is not immunity: `pick` is unchanged, descends a chain
node in pre-order exactly as it descends any node, and walks straight out of it
into the next sibling once its steps are done. What the directory *does* give is
the narrower property the flat shape lacked — **a sibling-level `leaf-insert` can
no longer split a chain**, because the steps are children rather than siblings.
That was the one real gap in the flat shape, and it closed as a by-product of the
grouping rather than by teaching `pick` anything.
_Avoid_: hand-cutting review and integration leaves around a picked plain
producer. `grove-llm leaf-promote-chain <producer>` moves that producer into a
brief-less review-chain node without changing its handle and derives both steps.
If a legacy or hand-cut producer already sits under a brief-less node, it is
already composition-managed and promotion refuses rather than nesting chains.
_Avoid_: naming a construction verb `chain-add` — "chain" is overloaded (see
*Flagged ambiguities*), and the `leaf-add-` prefix is what anchors the sense and
puts the verb beside the `leaf-add` a session already calls.
_Avoid_: giving either research leaf a `**Harness:**` declaration. Their
`research-a` and `research-b` session kinds are the configuration keys.
_Avoid_: running the *researchers* adversarially — that discards the breadth the
pair was run for. The adversarial move belongs to `combine-research`, whose
discipline is that **agreement without independent primary sourcing is a red
flag, not a confirmation** (two vendors on overlapping corpora can agree on
something false). None of a pair's three leaves runs an in-session doubt
reviewer: the two producers already supply independent corpora and the combiner
already supplies the adversarial move. A load-bearing decision derived from the
combined research belongs in its own reviewed producer chain.

**HITL** / **AFK**:
Whether a [[Session kind]] resolves through live exchange with a human who speaks for
themselves (`requirements`, `prototype`, `finish`) or is driven by the agent alone (every
other kind). The generating rule: a kind is HITL when its normal successful path
requires human-originated input the session cannot supply for itself — the
human's own words for `requirements` and `prototype`, or the explicit teardown
decision for `finish`. Merely wanting to read an output does not qualify, which
is true of everything and which the VCS already serves. A HITL leaf reached by
an unattended relaunch of the self-driving loop stalls until a human arrives;
that is correct, not a fault.
The mark **predicts, it does not permit**: *any* kind may stop and ask a human,
and doing so is always legitimate. `design` often will;
`integrate-review-requirements` is the borderline cell (it edits what was asked
for, which it cannot change unilaterally) and is marked AFK anyway.
_Avoid_: an agent answering its own HITL questions — a `requirements` session
that grills itself has broken the distinction (`grilling.md`).
_Avoid_: "`planning` is HITL" — it flipped to AFK when grilling moved to
`requirements`. Planning keeps its methodological force but no longer
interrogates.
_Avoid_: attaching a *mandatory* question to a **loop step** rather than a kind —
every kind runs every step, so a question in the Retire or Commit step overrides
the mark by construction and stalls AFK kinds at a moment nothing in the tree
predicts. That is what the Retire cascade's confirmation did before
[[Confirmation boundary]] removed it.

**Confirmation boundary**:
The rule deciding which moments in the loop stop and ask a human, applied in two
ordered tests: **(1) does the answer change what is written?** — if every answer
leaves the same bytes on disk, do not ask; **(2) if it does, is the fact the
session's to establish or the human's to decide?** — a session can establish *what
it did*, never *what is worth doing*. Four moments sit near the line and it
separates them: [[DONE infix]] (writes; the session holds the fact) and a **node
close** ask nothing. The node receives no lifecycle mark or review-target side
effect. [[Pruning]] (the mark asserts a human decided against a path) and the
[[Complete finish cycle]] (deletes `.grove/`) ask.
So the finish cycle's single gate is the loop's **only routine human gate**, and
everything else a session asks is an **escalation** — discretionary, triggered by
evidence the session actually met, and always legitimate ([[HITL]]/[[AFK]]: the
mark predicts, it does not permit). What replaced the node-close confirmation is a
**verify-and-report** obligation the session discharges itself: check the node's
brief `Done when` against what the subtree delivered, `leaf-add` the missing work
if the check fails and the gap can be named, escalate if it cannot, promote what
survives upward, and name the closed node by its [[Work-item handle]] in the
commit message. The human reviews the close in the diff instead of being
interrupted before it. See ADR *confirmation-boundary*.
_Avoid_: "all retirements need confirmation" — it over-reads the design in both
directions. Marking a *leaf* done was never confirmed (mechanical bookkeeping);
only the node close changed.
_Avoid_: reading the removal as a friction argument. A node is **never marked**
([[Node directory]]), so the question gated an *inference*. Review targets are
configuration policy and no close-time receipt survives; a confirmation that
gates a judgement-bearing write ([[Pruning]]) is untouched.
_Avoid_: re-adding a per-ancestor question as the cascade recurses — that is the
wizard anti-pattern *in-session-finish-cycle* already rejects, and it terminated
into that cycle's own confirmation, giving up to four questions about one fact.

**Grove configuration** (`~/.config/grove/config.kdl`):
The sole source of Grove's user configuration. It assigns every [[Session kind]]
one command-template string. The template chooses the executable or wrapper and
every user-controlled argument, including harness, model, reasoning effort,
approval, permission, and sandbox policy. Grove neither knows nor infers which
harness the command ultimately runs. The template uses POSIX shell-word quoting,
but Grove invokes no shell: it parses the template into argv, expands only the
declared Grove-owned substitutions, and executes the result directly. The first
word may be any executable or script on `PATH`; interactive shell aliases are
not supported, and a wrapper must `exec` its underlying harness to preserve the
driver's direct foreground-child ownership.

The first template word is a literal executable or script name and contains no
substitution. `${prompt}` is required exactly once in any later argv position.
It is the embedded `continue.md` launcher plus the selected leaf's stable handle
as its explicit mandate; the launcher relies on [[Global skill provisioning]]
rather than inlining a second copy of the methodology. Optional scalar
substitutions are `${session_name}`, `${worktree}`, and `${repo}`. “Word zero”
is the literal first parsed word — `env` in `env MODE=review agent …` — not the
eventual program an opaque wrapper reaches. Unknown, embedded, word-zero, or
multiply used substitutions are errors. Grove adds no hidden harness-specific
argv fragments.
Before the configured foreground spawn it scrubs stale Grove control variables
and grants its fresh `GROVE_SIGNAL_FILE`, but otherwise preserves the caller's
environment — including `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, and local
`core.worktree` behavior. Driver-internal lifecycle VCS children are separate:
they scrub repository selectors, and Git is explicitly anchored to the leased
working tree so personal launch context cannot redirect a migration commit.

A launch has no configuration precedence lattice: task files, command-line
flags, repository-local stamps, and environment variables do not override or
supplement this file. All nineteen session kinds must appear exactly once with
complete targets; a missing, duplicate, or unknown kind is an error. There are
no defaults, families, profiles, or inheritance. The driver re-reads and fully
validates the file before **any** task-tree mutation — including root-init,
legacy migration, and finish-leaf materialisation — and again immediately before
every launch, so an edit affects the next session. Validation failure launches
nothing and leaves an existing selected leaf live and resumable. An invalid
**pre-mutation** read leaves a rootless or pre-migration tree byte-identical; if
the file becomes invalid after a completed mutation but before the launch read,
that mutation remains resumable and no session launches. Configuration is never
cached across loop iterations. No user-settable `GROVE_*` environment
configuration remains. The driver still exports the unique
`GROVE_SIGNAL_FILE` capability to its foreground child; its path and the other
VCS-administration control files are internal orchestration, never settings
selected by the ambient environment.

Each iteration resolves `grove-llm` beside the running `grove` executable, with
`PATH` only as the no-sibling fallback, and rejects a missing, malformed, or
different version before configuration validation or tree mutation. There is no
`GROVE_LLM_BIN` override.
Foreground waits preserve child status and elapsed time: every no-signal exit
reports both, and a nonzero exit also names the session kind, configured word
zero, and config path. Test-only tool, clock, and grace injection is an internal
module seam, never supported environment configuration.

Grove never creates or edits the file because it cannot choose personal model,
effort, approval, or wrapper policy. A missing or invalid file is a
**task-tree-non-mutating** error that names the exact path and every missing,
duplicate, or unknown kind; global skill provisioning retains its independent
per-invocation contract. No init command or implicit default exists. Adding a
session kind is therefore an intentional breaking configuration-schema change:
a release must announce the required new entry, and older complete configs fail
validation until their owner adds it.
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
driver-side [[Pick]]. The driver reads the session kind from that leaf's filename,
obtains its complete session target from [[Grove configuration]], and embeds the
selected stable handle in `${prompt}` as the launched session's mandate. The
session first validates its [[Session epoch]], then resolves that handle to its
current path, rejects an unavailable or
non-live result, Bootstraps the resolved leaf, and does not pick again. A leaf
inserted ahead of the selection while the configured command starts may move the
mandated leaf's path but not its handle; it becomes the next iteration's work
and does not preempt the mandate already launched. `leaf-promote-chain` likewise
trusts the named live producer after its structural gates and never recomputes
pick: the CLI cannot observe prompt authority, and a second walk would reject
this legitimate insertion case. A session started outside
bare `grove` has no mandate and is not a Grove loop session; the supported
continuation is to exit and run `grove`, while `grove-llm pick` remains a
diagnostic/tree-interface verb rather than session dispatch. Grove executes the
configured command directly; it is not a model router or proxy.
_Avoid_: describing environment variables, a harness stamp, `--harness`, or a
leaf-level `**Harness:**` declaration as configuration fallbacks. Grove has one
configuration source.

**Review target diversity**:
Whether a scheduled review uses a different harness or model from its producer
is explicit policy in [[Grove configuration]]. Grove does not interpret command
templates to recover target identity, persist producer launch receipts, export
session-target metadata, compare targets, or warn. The review chain still supplies
a fresh session; choosing a materially different command is the configuration
owner's responsibility.

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

### Task-tree scheme (v2 directories, task-tree-scheme)

**Tree format witness** (`.grove/FORMAT`):
The positive discriminator for the current session-kind filename grammar, with
exact LF-terminated content `session-kinds-v1\n`. Root initialization and
migration write it last by same-directory temporary write plus atomic rename,
after the rest of the current tree is complete. Before absence means legacy,
bare `grove` first recovers a migration witness or an exact partial root-init
scaffold. A legacy slug may begin with a valid kind token; an unknown value means
a newer or foreign format and stops without mutation. It is format metadata
inside the artifact tree, not a phase, status, or counter file. Atomic
replacement covers process interruption, not ordered power-loss durability.
_Avoid_: inferring current format by parsing a filename prefix — a legacy slug
such as `design-notes` makes that test silently change both kind and handle.

**Node directory** / **node**:
A grove tree node is a **directory** named `NN-<slug>-k<key>/` holding its numbered children (leaf files and child node directories), optionally headed by a `BRIEF.md` charter; `.grove/` is itself the root node (its charter is `.grove/BRIEF.md`). The filesystem carries the hierarchy, so a name encodes only its *per-level* position — not a global path (task-tree-scheme).
**Two species, discriminated by whether the directory carries a `BRIEF.md`.** A
**decomposition node** (written by `leaf-decompose`; `root-init` for the root)
always carries one: it means *this work proved bigger than one session*, and the
charter is the context those extra sessions need. A **chain node** (written by
`leaf-add-chain` / `leaf-add-pair`, or atomically retrofitted around a named live
producer by `leaf-promote-chain`) never does: it means *these steps compose one
artifact*, with no charter anyone is in a position to write. The discriminator
is a **file's presence, never a name
pattern** — which is what lets the Retire cascade tell them apart without reading
the step-suffix convention back (task-kind-taxonomy).
Three consequences. It decides whether a closing node has **work to do** — a
brief-carrying node has a `Done when` rollup to check against its subtree and a
brief to promote upward, a chain node has neither and closes as a silent no-op. (It
decides nothing about *asking*, which the cascade no longer does of any node —
[[Confirmation boundary]].) `brief-chain`'s tolerance of a missing level is now
**load-bearing** rather than incidental (it was "some nodes do not carry one
*yet*"; one species never will). And nothing is enforced: a human who writes a
`BRIEF.md` into a chain node has simply made it brief-carrying, and gets the
close-time work that follows from that.
_Avoid_: calling a node a "file" — a node is always a directory.
_Avoid_: "every node has a charter", or reading a bare directory as malformed —
both were true only before chain nodes existed.
_Avoid_: discriminating the two species by the `-chain` / `-pair` token in the
node's slug. That token exists to keep the node's slug distinct from its first
child's for `resolve`, and it is ordinary slug text a human may not use; the
`BRIEF.md` test is the one that holds.

**Leaf**:
A single unit of work — a file `NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md` inside a node directory, executed in one session. The only thing `pick` returns is a *live* leaf — one carrying **no outcome infix** at all. A leaf has exactly two terminal states: `DONE` (the work was done) and `ABANDONED` (the path was closed); see [[DONE infix]] and [[Pruning]].

**Pick** (`grove-llm pick`):
The loop's dispatcher: absent a pending [[Promotion transaction]], the **first
eligible live [[Leaf]] in depth-first pre-order** over `.grove/`. Eligibility
has one lifecycle exception: a driver-owned `finish` sentinel is skipped while
any non-finish leaf is live, then becomes eligible when it is the sole live
leaf. Multiple live finish leaves are malformed and stop selection. Among
non-finish leaves nothing modulates the walk — no priority, no
grouping, no set of leaves that must finish before another is considered. A
transaction is not a scheduling input: it is a
fail-closed malformed-tree condition that every reader and mutator refuses until
the interrupted operation is recovered. Ordering in a grove is **contiguity**,
at every level, and it is the only ordering grove offers: a [[Review chain]]'s
steps run in sequence because they are one node's children at adjacent
[[Position]]s, exactly as a decomposition's are. It reads the selected leaf's
session kind from its filename and no task-file contents. The driver's one pick
is authoritative for the launched session; the next loop iteration re-derives
its place from the tree, which is what makes restart ≡ continuation
(self-driving-loop). An empty result tells the driver to materialise the
[[Complete finish cycle]] as a `finish` leaf and then pick again. Its value is
that a human computes it **by eye** — `find .grove` shows the tree and the next
session is the first non-finish name with no outcome infix, or the live finish
name when none exists — which is
what makes "the tree is the only state" worth something rather than merely true.
_Avoid_: expecting `pick` to **schedule** — to finish a group before considering an earlier live leaf, or to skip a leaf that is merely blocked. Answering "is a group in flight?" needs either state outside the tree (constraint 1) or a rule that skips live work and ranks groups, turning the walk into a scheduler no reader of `find .grove` can predict (task-tree-scheme, *`pick` is a walk, not a scheduler*).

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

**Promotion transaction** (`PROMOTING-<final-node-name>/`):
The reserved, fail-closed sibling directory used while
`grove-llm leaf-promote-chain` turns the named live plain producer into a
brief-less [[Review chain]] without changing the producer's handle. Under the
exclusive [[Tree access lock]], the command prebuilds the generated steps there
and moves the producer into child `01` through the VCS-aware rename seam. In
plain Git it then rewrites the staged producer index entry to its final path
while the reserved witness still exists; the complete node lands last with one
plain same-parent filesystem rename. Jujutsu never touches Git's index and uses
that same final rename. Every reader and mutator recursively refuses while the
prefix exists, so an interruption can expose neither review-before-producer
ordering, a key run another grow verb can reuse, nor a filesystem-complete tree
whose index still names `PROMOTING-`. Recovery runs before ordinary
pick/liveness gates and accepts a stable producer handle, the stale picked path,
or the exact reserved path from a generic diagnostic. Retrying reuses and
finishes or reverses that exact transaction; a reported failure rolls it back
and prints no success output. All other foreign names remain leniently ignored.
This is process-interruption consistency, not a power-loss durability claim;
Grove performs no ordered `fsync` protocol.
See ADR *task-tree-transactions-fail-closed*.
The explicit producer reference is workflow discipline, not an unforgeable
capability: the command structurally validates and trusts it, while concurrent-
driver and stale-session exclusion are a separate process-ownership concern.
_Avoid_: deleting the directory as if it were an unknown foreign file — after
the producer move it may contain the only copy of that task. Recover through
the exact `leaf-promote-chain <PROMOTING-path>` command a refusing reader prints.

**Tree access lock**:
The process-scoped advisory lock every task-tree reader and mutator takes on an
open descriptor for the working-tree root before inspecting names. Readers hold
it shared and mutators exclusive; exported operations acquire once and pass the
guard into lock-neutral helpers. The working-tree directory exists before
`.grove/` is created and through its deletion, so root initialization,
migration, finish allocation/deletion, and agent tree verbs share one invariant;
a missing optional `BRIEF.md` never jams the tree and no lock file or state bytes
are created. Descriptors are close-on-exec; the driver releases its read guard
as soon as it copies the selected path/handle/kind and before foreground launch,
so the session never inherits or deadlocks on the lock. The lock serializes live
processes but adds no crash atomicity; operations that promise process-
interruption recovery use their own in-tree witnesses. Acquisition first tries
without blocking, prints one waiting diagnostic on contention, then waits until
the owning process exits or releases it. Direct human edits remain outside the
cooperative serialization. See ADR *task-tree-transactions-fail-closed*.
_Avoid_: locking `.grove/BRIEF.md` — root briefs are lazy, optional artifacts,
and existing tree readers deliberately tolerate their absence.
_Avoid_: locking `.grove/` itself — it cannot serialize either its own creation
or its finish deletion, and adding a second lifecycle lock creates an ordering
contract instead of one seam.

**DONE infix**:
The in-place retirement marker: the literal `DONE` placed right after the
position in a retired leaf's filename
(`NN-DONE-<session-kind>-<slug>-k<key>.md`), at a fixed column. Written by `leaf-retire`.
Leaves only — a node is never marked done (its done-ness is the absence of a
live leaf in its subtree, however those leaves finished). The retired leaf keeps
its position and key, and **its own contents remain untouched**.
Its sibling mark is `ABANDONED` ([[Pruning]]); the two are the only terminal leaf
states.
_Avoid_: moving a retired leaf into a separate folder or list — retirement is in place, so the tree always shows complete state.

**Pruning** / **ABANDONED infix** (`leaf-prune`):
Marking a work path **decided against** — as opposed to done. `grove-llm leaf-prune`
writes an `ABANDONED` infix in place (`NN-ABANDONED-<session-kind>-<slug>-k<key>.md`), exactly as
`leaf-retire` writes `DONE`; `pick` skips both, and neither ever leaves the tree.
grove's metaphor names the pair: a `DONE` leaf is **harvested**, an abandoned one is
**pruned**. Deciding against a path is a normal outcome of exploration, not an
exception — but it is **[[HITL]]**: an agent never prunes on its own, and an AFK
session that finds a leaf dead says so and stops. It is the loop's one *per-leaf*
human gate, and it clears both of the [[Confirmation boundary]]'s tests — the mark
is a real write, and what it asserts (*this path is not worth taking*) is the
human's to decide. Contrast a **node close**, which asks nothing because it writes
nothing.
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
use ("brief chain", "review chain") and never let a bare "chain" head a verb
name — the reason a construction verb is `leaf-add-chain` rather than
`chain-add`, which would sit beside `brief-chain` meaning something else entirely.
