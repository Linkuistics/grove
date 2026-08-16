---
name: grove
description: Grove's methodology for driving a long, multi-session workstream as a VCS-tracked tree of task files under .grove/ — the loop, the nineteen session kinds, the conditions every session holds to, and the reference file each kind reads. Use when a Grove mandate names this skill; when running any session inside a grove working tree; or when starting, picking up, or continuing a task tree under .grove/.
---
<!-- file: order=2 -->
<!-- unit: skill-what-a-grove-is kinds=* class=triggering -->

# grove — hierarchical, self-extending workstreams

A **grove** is one workstream driven as a VCS-tracked **tree of task files**,
one task per session. Planning tasks grow the tree as understanding deepens;
completed leaves are marked done in place. The tree's shape — the directory tree
under `.grove/` — is the only state; the VCS holds the history.

<!-- unit: skill-kind-routing kinds=* class=triggering -->
## Read your kind's reference file first

**The driver resolved your leaf and its kind before this session existed, so you
select nothing here — open the one row your kind names, before you act.** Ten
files serve the nineteen kinds; each path is relative to this skill's directory.

| your session kind | read |
|---|---|
| `requirements` | `references/requirements.md` |
| `design` | `references/design.md` |
| `planning` | `references/planning.md` |
| `prototype` | `references/prototype.md` |
| `impl` | `references/impl.md` |
| the five `review-*` kinds | `references/review.md` |
| the five `integrate-review-*` kinds | `references/integrate-review.md` |
| `research-a`, `research-b` | `references/research.md` |
| `combine-research` | `references/combine-research.md` |
| `finish` | `references/finish.md` |

The rest of this file states **conditions** — what holds in every session, and
which file carries the procedure for each. It is not an abridged methodology: a
condition that applies to you is a pointer, and the steps live in the file it
names.

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

<!-- unit: task-leaf-filename kinds=* class=triggering defers="task-what-each-field-does task-suggested-shape" -->
## Leaves, kinds and shapes

**A leaf is one `.md` file whose name is its whole grammar** —
`NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md`: a per-level position, an
outcome infix once it is terminal, the session kind, a human slug, and a
permanent key assigned once and never reused. One task is one session, and the
body is freeform markdown (`TASK-FORMAT.md`).

<!-- unit: task-kind-in-the-filename kinds=* class=triggering defers=task-name-reading-is-strict -->
**The kind lives in the filename and nowhere else.** It is routing metadata, not
identity — the stable **work-item handle** stays `<slug>-k<key>`, so `resolve`, a
commit message and an in-file header are all unaffected by it. A **node
directory** carries no kind at all (`NN-<slug>-k<key>/`), even when its slug
happens to begin with a kind word.

<!-- unit: task-nineteen-kinds kinds=* class=triggering defers="task-the-kind-table task-work-is-not-a-kind" -->
**The kinds are a closed set of nineteen**: five producers — `requirements`,
`design`, `planning`, `prototype`, `impl` — each with its own `review-` and
`integrate-review-` step, plus `research-a`, `research-b`, `combine-research`,
and the driver-reserved `finish`. Each kind is the key one command template is
configured under, so a twentieth is a deliberate change to grove's code, its
configuration schema and its docs — never a free-text label a leaf may coin
(`TASK-FORMAT.md`).

<!-- unit: task-hitl-afk kinds=* class=triggering -->
**Three kinds are HITL** — `requirements`, `prototype` and `finish` — because
each needs input the session cannot supply for itself: the human's own words,
their reaction, or their teardown decision. Every other kind is **AFK**. The mark
**predicts, it does not permit**: *any* kind may stop and ask a human, and a HITL
leaf reached by an unattended relaunch of the loop simply waits for one, which is
correct behaviour rather than a fault.

<!-- unit: task-in-session-doubt-budget kinds=* class=triggering defers="task-the-doubt-budget-table driving-doubting-inside-a-picked-leaf skill-review-ownership skill-cutting-a-review-leaf" -->
**In-session doubt is budgeted across the whole picked leaf**, once you have run
Bootstrap and adopted the driver's mandate: a plain producer may materialise **at
most one** reviewer and an `integrate-review-*` session one narrow reviewer;
every other kind spends **none**. The allowance is leaf-wide, not per artifact or
decision, and beyond it the answer is a `review-*` leaf rather than another
reviewer (`TASK-FORMAT.md` tabulates it by kind).

<!-- unit: task-too-big-is-planning kinds=* class=triggering defers=task-decompose-inherits-kind -->
A task too big for one focused session *is* a planning task — its job is to
decompose, not to do.

<!-- unit: task-two-shapes kinds=* class=triggering defers="task-review-chain-mechanics task-vendor-pair-mechanics task-leaf-never-names-a-harness driving-the-review-chain driving-review-chain-habits skill-cut-the-next-step" -->
**Two habitual shapes compose the kinds** — the **review chain** `X` →
`review-X` → `integrate-review-X`, and the **vendor pair** `research-a` →
`research-b` → `combine-research`. Reach for them by default, and argue yourself
*out* of one rather than into it. **They are built in opposite ways**, and the
asymmetry is the design: a chain's steps are cut **lazily, one at a time, by the
session that needs the next one**; a pair is cut **eagerly, whole, in one call**
(`references/decompose.md`).

<!-- unit: task-no-node-for-a-shape kinds=* class=triggering -->
**Neither shape gets a node directory.** A charter means *this work proved bigger
than one session*, and a composed shape is neither — no extra sessions, and no
context to charter. Its steps sit as flat siblings among their neighbours, and
there is **one node species**: a node is a leaf that decomposed, and it carries a
`BRIEF.md`.

<!-- unit: task-declare-the-relationship kinds=* class=triggering -->
**Declare the relationship in the body, by hand.** A review's body carries
`**Reviews:** <producer-handle>` and an integration's carries `**Integrates:**
<review-handle>`, on their own line, naming the stable handle. Nothing writes
those lines and **nothing parses them** — write them because the next session
benefits, not because a verb requires it.

<!-- unit: task-grammar-is-five-fields kinds=* class=triggering defers=task-what-each-field-does -->
**A leaf name is five fields and nothing more**, and every one of them is parsed:
position, outcome infix, kind, slug and key. What is **convention, not grammar**
is everything a name might *imply about another leaf* — the shared stem, the
relative ordering, and the two declaration lines. Grove does not require a
`review-X` after every `X`, reject a partial chain, or parse a `**Reviews:**`
line, and it never reconstructs a relationship from a filename, a position, or a
body (`TASK-FORMAT.md`).

<!-- unit: task-nothing-in-a-body-is-metadata kinds=* class=triggering -->
**Nothing in a body is metadata**, and no verb writes anything there. Retirement
and pruning change a filename and stop; a node close writes nothing at all. So a
`review-*` session has exactly one thing to read — its producer's committed
artifact — and never a note the producer left behind about how its own session
ran.

<!-- unit: task-three-design-kinds kinds=* class=triggering -->
**The design work is three kinds, not one.** What a single `planning` label used
to cover is split across `requirements`, `design` and `planning`, and each kind's
own reference file carries the part of the old checklist it kept.

<!-- unit: task-deliverable-split-not-a-gate kinds=* class=triggering -->
The split is a division of *deliverable*, not a gate: a small workstream may
resolve all three in one leaf, and any of the three may sharpen the glossary
inline. What does not blur is tree growth — **only `planning` may grow the tree
generatively**; *reactive* decomposition (a leaf proving bigger than its brief)
is kind-agnostic and available to every kind.

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

<!-- unit: spec-when-a-spec-is-written kinds=* class=triggering defers="spec-set-is-current-state spec-suggested-shape" -->
**A spec is written by a `design` task at a genuine agreement point and nowhere
else** — the point where a human signs off on the design's shape before
decomposition turns it into `impl` leaves. Most increments write none
(constraint 4). One file per area at `docs/specs/<slug>.md`, the slug is the
identity, and the directory is created lazily (`SPEC-FORMAT.md`).

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

<!-- unit: driving-when-to-commission-prior-art-research kinds=* class=triggering defers="driving-signs-of-a-research-leaf driving-how-to-write-a-research-leaf-brief driving-running-the-vendor-pair" -->
## Working habits

**Commission prior-art research when a leaf's design depends on lessons prior
tools learned the hard way** and those lessons are not obvious from the current
codebase. Insert it *after* the design questions are visible but *before* the
leaves that need the answers, so later grilling spends its time on the remaining
decisions instead of rediscovering failure modes. One survey is a `research-a`
leaf; the **vendor pair** is what buys two corpora (`driving.md`).

<!-- unit: driving-when-to-retire-research-into-adrs kinds=* class=triggering defers="driving-the-findings-adopted-bridge driving-reworking-adrs-and-briefs" -->
**Research outlives the grove that commissioned it (constraint 6), and ADRs are
where its findings become binding.** A finding that *changed* a decision is cited in that
ADR's rationale; one that *confirmed* a decision gets a "validated here against
…" note, or stays in `docs/research/` with a forward pointer. Either way you edit
the ADR **in place**, because the set is current-state (`driving.md`).

<!-- unit: driving-when-code-depends-on-a-framework-version kinds=* class=triggering defers=driving-cite-framework-decisions-to-the-source -->
<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     addyosmani/agent-skills@13e43f23 (skills/source-driven-development)
     — MIT licensed; see LICENSES/addyosmani-agent-skills.LICENSE. -->
**Framework- or library-specific code whose correctness depends on the version is
verified against the source, not against memory** — training data goes stale, and
an API you "remember" may have been deprecated two releases ago. Read the
manifest, fetch the official docs, cite at the decision site, and flag what you
could not verify. Version-invariant logic, renames and plumbing are exempt
(`driving.md`).

<!-- unit: driving-when-asserting-a-repo-wide-claim kinds=* class=triggering defers=driving-turning-a-sweep-into-evidence -->
**A repo-wide claim about your own codebase — *"every X is now Y"*, *"that
pattern is gone"* — needs a control, not just a clean grep.** Every way a sweep
fails produces a *clean-looking* result, and a broken instrument reads clean
**everywhere**, so clean-here alone proves nothing. Pair the sweep with a
positive and a cross-tree control, and **enumerate then classify** rather than
sweeping a pattern list — a list is complete only as far as the list
(`driving.md`).

<!-- unit: driving-recording-fog kinds=* class=triggering defers="driving-the-fog-or-ticket-test brief-suggested-shape" -->
<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Fog of war" / "Not yet
     specified") — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->
**Work you can see coming but cannot yet leaf-shape goes in a brief's *On the
horizon* note**, not into a speculative leaf — laziness means a leaf either
exists or it doesn't, and the note is the only place that dim view keeps. The
line between the two is the **fog-or-ticket test**: can you state the question
precisely *right now* — not whether you can answer it (`BRIEF-FORMAT.md`).

<!-- unit: driving-when-a-leafs-place-is-in-doubt kinds=* class=triggering defers=driving-prune-reorder-or-file-an-issue -->
**A leaf whose place is in doubt never gets a status word** — no `blocked`, no
`deferred`, no `superseded`. A fourth state is how a tree starts lying about what
is still live, and the doubt always resolves to one of three existing mechanisms:
*not now, but still ours* → a **reorder**; *not ours at all* → a **GitHub
issue**, not a leaf; *decided against* → a **prune** (HITL). Name which of the
three sentences is true before reaching for the CLI — the sentence picks the verb
(`driving.md`).

<!-- unit: driving-no-session-summary kinds=* class=triggering defers=driving-record-decisions-inline -->
**Do not reconstruct the session's decisions at the end of it** — no
session-summary file, and no commit message that re-tells every decision.
Decisions are recorded **inline, as they land**: in the task file's running log
while the session still holds the reasoning, and in the ADR set where one is
durable enough to earn a record. A third telling written afterwards duplicates
them, rots against them, and is exactly the status artifact constraint 1 rejects.

<!-- unit: driving-ask-about-the-trade-off kinds=* class=triggering -->
**Never close by inviting questions in general.** "Let me know if you have any
questions" is not a prompt — the human has to invent your agenda before they can
answer it. Name the **specific** trade-off you want input on, propose a
recommended answer, and give the evidence behind that recommendation; that turns
an escalation into a decision instead of an open-ended handback.
