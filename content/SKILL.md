---
name: grove
description: Use when driving a long, multi-session workstream that cannot be planned exhaustively upfront — work spanning many sessions and months where some steps are themselves planning steps — or when picking up or continuing a task tree under .grove/.
---

# grove — hierarchical, self-extending workstreams

A **grove** is one workstream driven as a VCS-tracked **tree of task files**,
one task per session. Planning tasks grow the tree as understanding deepens;
completed leaves are marked done in place. The tree's shape — the directory tree
under `.grove/` — is the only state; the VCS holds the history.

```mermaid
flowchart TD
  subgraph tree["A grove — a directory tree under .grove/; a node is a directory"]
    direction TB
    root["BRIEF.md — root brief (heads .grove/)"]
    n1["01-design-k1/ — decomposition node"]
    nb1["BRIEF.md — node brief"]
    l1["01-DONE-spec-k2.md — retired leaf, in place"]
    l2["02-impl-k3.md — live leaf"]
    n2["02-build-chain-k4/ — chain node, no brief"]
    c1["01-build-k5.md — live leaf"]
    root --- n1
    root --- n2
    n2 --- c1
    n1 --- nb1
    n1 --- l1
    n1 --- l2
  end
  subgraph loop["The loop — one task per session"]
    direction TB
    pick["Pick — first live leaf: depth-first walk, skip briefs + DONE"]
    boot["Bootstrap — read glossary, ancestor BRIEFs, cited ADRs, the task"]
    exec{"kind — planning, or one of the other sixteen?"}
    plan["Planning — cut vertical slices; grow the tree"]
    work["Produce — requirements grills; design specs; impl codes; review-* finds; integrate-review-* applies"]
    commit["Commit — one task = one focused commit (name it by <slug>-k<key>)"]
    retire{"parent chain — node now has no live leaf?"}
    ret["Brief-carrying node: verify Done when; promote brief up; report close. Brief-less node: no-op"]
    signal["Signal — grove-llm complete; loop relaunches with fresh context"]
    pick --> boot --> exec
    exec -->|planning| plan --> commit
    exec -->|any other kind| work --> commit
    commit --> retire
    retire -->|yes| ret --> retire
    retire -->|no| signal --> pick
  end
```

## The spine — seven constraints

grove drives long work *without* becoming brittle, constraining machinery.
These seven rules are non-negotiable; everything below is subordinate to them.

1. **Artifacts, not state.** No phase file, no session log, no status file.
   The directory tree under `.grove/` is the only state; the VCS holds the
   history.
2. **Read, don't run.** A session bootstraps by *reading markdown* — no script
   must succeed before work begins. (Materialising or updating grove itself is
   a separate maintenance action and may use a script — see `VERSION.md`.)
3. **Suggested shape, not enforced schema.** Task files and briefs are freeform
   markdown. The format files are guides; nothing validates them.
4. **Lazy and optional.** Every artifact — brief, ADR, spec, glossary entry — is
   created only when it earns its place, never because a step demands it. Lazy
   means *just-in-time, not few*: a tree that keeps sprouting small, concrete
   leaves is healthy, not a smell.
5. **grove guides, it does not gate.** grove never refuses to proceed. A task
   may be done by hand, reordered, or skipped.
6. **Walk-away-able.** Delete this skill and `.grove/` is still a legible
   folder of notes; every durable output is standard, team-readable markdown.
7. **One page of rules.** If the loop below does not fit on a page, it is too
   complex — cut until it does.

## The loop

One task is one session. A grove runs in **a working tree the user
provides** — git or jj-enabled (a `.jj/` present, colocated or native); a main
checkout, linked git worktree, or jj workspace; on any branch, anywhere on
disk; grove reads no branch and no bookmark anywhere (user-owned-worktrees).
The grove's name is that working tree's directory basename, and its task tree
lives at `.grove/` inside it.

Sessions are launched by the `grove` CLI (installed via `brew install Linkuistics/taps/grove`): run **argument-less `grove do`** from inside the working tree. `do` is the **sole lifecycle entry verb** — it inspects the state on disk and dispatches: no `.grove/` yet → a bootstrap session; a live tree → the loop continues; no live leaves left → the session proposes the complete finish cycle (do-is-sole-lifecycle-verb). It pre-names the harness session, so the rename ritual is unnecessary in the common case. If the grove's `.grove/` is in an older format — the original `NNN-slug/` directories, or the v1 flat dotted-decimal scheme — the first `grove do` **migrates it to the current directory scheme** — one reviewable, committed change — before driving; the migration is idempotent once a tree is current-format, and there is **no** transitional dual-format reader (task-tree-scheme).

`grove do` drives the **whole loop**, not one task (self-driving-loop). It is a thin, stateless **self-driving loop**: launch one fresh foreground harness session (owning the real TTY, so grilling / resize / Ctrl-C are all native), and when that session ends, **relaunch with fresh context** — but only if the agent fired the completion signal. That makes each task a clean-context session without a manual `/clear`+relaunch crank. **Relaunch is opt-in:** any other exit — your `/exit`, the human's Ctrl-C, or a crash — **stops** the loop, resumable later by re-running `grove do` from the same working tree. Because the loop body holds zero engine state and re-derives its position from `grove-llm pick` every iteration, **restart ≡ continuation** by construction; a crashed mid-task leaf (commit-before-retire, then signal) is simply re-picked and redone. There is no PTY wrapper and no daemon — a plain shell `while` loop could stand in (constraint 6).

If a session was started without the helpers and the session name doesn't already match `<repo-basename>: <name> grove`, suggest `/rename <repo-basename>: <name> grove` once per session and move on. The skill already knows both names: `<name>` from the working tree's own basename (`git rev-parse --show-toplevel`; `jj workspace root` in a jj-enabled tree), `<repo-basename>` from the **main repo**'s basename (`git rev-parse --git-common-dir`'s parent, or `jj workspace root --name default`'s basename in a jj-enabled tree — the repo a linked worktree or secondary workspace belongs to, not the working tree's own path).

**Starting a new grove.** Provide a working tree by whatever means you
like — `git init`, `git clone`, `jj git init --colocate`, `jj git clone`, a
plain checkout, a linked worktree or jj workspace (`jj workspace add`), or
your own tooling (e.g. [worktrunk](https://github.com/max-sixty/worktrunk)) —
then run argument-less `grove do` from inside it. A brand-new grove has a
working tree but no `.grove/` tree yet — and every step below assumes
`.grove/` already exists. Resolve that chicken-and-egg first: a rootless grove
has nothing for `grove-llm pick` to walk (it errors `grove root not found`), and
the tree-growing verbs (`leaf-add` and friends) all need a root too. Run
**`grove-llm root-init [<slug>]`** (default slug `plan`) once: it creates
`.grove/`, the root `BRIEF.md` stub, and a first **requirements** leaf
`01-<slug>-k1.md` — working-tree only, no commit (the first session's commit
folds it in), refusing to clobber an existing `.grove/`. Creating the first
leaf, not just the brief, is load-bearing: `pick` skips every brief
(`BRIEF.md`), so a brief-only `.grove/` reports `no live leaves; this grove is
done` and would mis-trigger the Complete finish cycle — a newborn grove
indistinguishable from a finished one (fresh-grove-start-contract). The kind is
fixed at `requirements` (no `--kind`): the bootstrap session's only input is the
human's own words, and the loop routes that launch *by construction*, having no
leaf to peek — so `GROVE_REQUIREMENTS_MODEL` is the one var a brand-new grove
cannot start without. After `root-init`, `pick` returns that leaf and you enter
the normal loop below at **Bootstrap**: grill it, record the outcome in the root
brief, then grow the tree — cutting the leaves yourself if the workstream is
small, otherwise adding a `planning` leaf for a fresh session. The launcher's
`start.md` prompt names this as step one.

**Pick.** Run `grove-llm pick` — it walks the `.grove/` directory tree
depth-first in **pre-order**, visiting each directory's children in per-level
position order (the `BRIEF.md` charter first — and skipped — then by numeric
position), descending node directories in place, and prints the absolute path of
the next live leaf (a `.md` file with no `DONE` infix). Retired (`DONE`) leaves,
briefs, and foreign files are skipped. Empty stdout (and a diagnostic on stderr)
means the grove has no live leaves and is ready to **Finish**. The walk's
*semantics* (depth-first pre-order; a node directory is fully explored before
its later siblings; skip every `BRIEF.md` and `DONE` leaf; a brief is not a
leaf) are what the verb implements; reach for them only when reasoning about the
walk, not when running it.

Under `grove do` this is the **second** pick: the driver already peeked the tree
to route this session's harness and model (*model-per-task-kind*), necessarily
*before* the session existed. That peek is a **forecast**; your `pick` is the
**fact**, and the fact wins — grove hands a session no leaf identity, so the tree
stays the only state. Work the leaf `pick` returns even if the launch was routed
for a different one; the next iteration re-derives and is already correct.

**Bootstrap.** Read, in order: the glossary (`CONTEXT.md`, or the relevant
bounded context via `CONTEXT-MAP.md`); the ADRs cited by the briefs; the
`BRIEF.md` chain root→leaf, enumerated by `grove-llm brief-chain` — the verb
walks the picked leaf's **ancestor directories**, from the grove root down to
the leaf's own directory, and prints each level's `BRIEF.md`, one absolute brief
path per line, root→leaf (a missing brief at any level is skipped silently —
some nodes do not yet carry one); the task file. That assembled context is the
session's entire mandate; read nothing else by reflex.

**Execute.** The task file states its kind, drawn from a closed set of
**seventeen** — five producers (`requirements`, `design`, `planning`,
`prototype`, `impl`), each with its own `review-` and `integrate-review-` step,
plus `research` and `combine-research` (`TASK-FORMAT.md` for every kind's
discipline and its HITL/AFK mark). `work` is the previous spelling of `impl` and
still reads as it. **`planning` is the only kind with methodological force** —
the sole branch here, and the only kind that grows the tree generatively:
- Every other kind **produces an artifact** and differs in discipline, not in
  what the loop does with it. `requirements` opens with a **grilling session**
  (`grilling.md`): interview the user one question at a time, propose a
  recommended answer for each, walk down the design tree until shared
  understanding is reached, updating `CONTEXT.md` *inline* as terms resolve.
  `design` turns that into a spec or an ADR set; `impl` produces code, docs, or
  tests; the `review-*` kinds produce findings and the `integrate-review-*` kinds
  apply them.
- A `review-*` session is **inspection-only**: inspect the producer's committed
  changes, source, requirements or specifications, and recorded verification
  evidence. Do not run test, build, lint, or format commands, edit production or
  test code, or redo the implementation. Review output is findings only; the
  paired `integrate-review-*` task owns every fix and all post-fix verification.
- A **planning task** cuts the design into vertical slices and **grows the
  tree** (Decompose, below). It no longer interrogates — grilling moved to
  `requirements` — but it MAY still sharpen the glossary or raise an ADR inline,
  as any kind may.

**Review ownership inside a picked leaf.** This applies only after the session
ran Bootstrap, invoked `grove-llm pick` itself, and adopted its result — checkout
state and inherited `GROVE_*` values do not count. A picked plain producer may
materialise at most one reviewer across the **whole picked leaf**; each
independent diverse-lens context counts. A second need — normally re-review
after a substantive non-mechanical fix — runs `grove-llm leaf-promote-chain
<picked-producer>`; trivia, noise, visible trade-offs, and test-conclusive fixes
do not force it. A chained producer, `review-*`, and every research-pair leaf
spawn none; `integrate-review-*` may spend one narrow reviewer, then externalises
substantial redesign inside the owning chain node. Outside this predicate doubt
keeps its standalone cycles. Grove routes external review and warns unless both
harness and exact model differ, but never blocks on diversity.

Whichever kind is running: raise ADRs *sparingly* (`ADR-FORMAT.md` for placement;
the `linkuistics:decision-records` skill for the philosophy, format, and
when-to-write test), and write a spec only at a genuine agreement point
(`SPEC-FORMAT.md`). Treat the ADR set as a **minimum coherent set describing the
current design**: when a session *changes* a decision an ADR already records,
**rework the set in place** — merge / split / delete — and reconcile the briefs
that cite it; never append a superseding ADR (the VCS holds the history). The
same rule governs `docs/specs/`, one grain coarser. See `driving.md` for the
field-guide habits that make grilling, research-leaf commissioning, and the
review chain productive (WDYT, pushback, running decision log, citation
discipline).

**Decompose.** When work surfaces mid-session, default to **externalizing it as
a new leaf** rather than absorbing it into the current session — grove's value
is many small, low-context sessions, and that value is lost the moment a session
quietly grows to cover work that should have been its own leaf. Two triggers,
two verbs:
- **A new concern** — the human raises it, or a tangent appears that does not
  serve *this leaf's stated goal* — goes to the tree with `leaf-add` (or
  `leaf-insert` when it must sequence ahead of live leaves), **never** inline.
- **The current item proves bigger** than its brief assumed — turn the leaf into
  a node (a brief, `BRIEF-FORMAT.md`, and ordered child leaves) with
  `leaf-decompose`, doing **only the first child** this session, each child
  shaped as a vertical slice that stands demoable on its own (`driving.md`).

Continue inline **only** while the work still serves this leaf's stated goal
*and* fits one focused, low-context session — the bar is *"fits this session,"
not "I can finish it."* Decomposition stays lazy (constraint 4): grow the tree
just-in-time, at the genuine seam, never speculatively.

**Compose, don't just append.** When more than one leaf serves *one* artifact,
two shapes are the habitual answer — reach for them by default, and argue
yourself *out* of one rather than into it. **Each is one call**, so cutting the
whole shape is no more work than cutting the first leaf of it:
- **The review chain** — `X` → `review-X` → `integrate-review-X`: a fresh
  context asked to *disprove*, then a leaf licensed to act on what it found.
  Cut it proactively when the artifact is load-bearing — a spec, a
  decomposition you will build on for months, a subsystem. One narrow,
  unexpected doubt in a picked plain producer may use its single in-session
  reviewer instead (`driving.md`).

      grove-llm leaf-add-chain <parent> <stem> --kind <producer>

  You name the **producer** kind — `requirements`, `design`, `planning`,
  `prototype` or `impl` — and the verb derives `review-<producer>` and
  `integrate-review-<producer>`. That derivation is the point: a hand-written
  `--kind review-impl` beside a `design` producer is a perfectly valid
  invocation, and it silently gives the reviewer the wrong discipline. Reviews
  route by *policy* (`GROVE_REVIEW_HARNESS`), so the verb takes no `--harness`.
- **The vendor pair** — `research` → `research` → `combine-research`, the two
  surveys differing *only* by vendor. Cut it when the question is load-bearing
  enough to pay for two corpora.

      grove-llm leaf-add-pair <parent> <stem> --harness-a <name> --harness-b <other>

  **Both** producers are declared and the two must differ — that is what makes
  "two corpora" a fact in the tree rather than a guess about what routing policy
  will say weeks later. An equal pair is refused.

**Each shape is a node directory** — `<stem>-chain/` or `<stem>-pair/` — holding
its three steps at `01`–`03`. Four keys per shape: the node holds the first, and
stdout prints its path before the three leaves. **The whole shape lands or none
does**; failure rolls back and prints nothing. It is byte-identical to the same
manual shape only with its stable relationships: the review declares
`**Reviews:** <producer-handle>` and integration declares `**Integrates:**
<review-handle>`.

**The node carries no `BRIEF.md`, by rule.** A charter means work proved bigger;
a composed shape has no new context to charter. Absence is also Retire's
**discriminator** — a file test, never a parse of ordinary `-chain` / `-pair`
slug text. Add a brief yourself and the node simply becomes brief-carrying.

**The stem gets a step suffix, not a prefix** — `<stem>` / `<stem>-review` /
`<stem>-integrate`, and `<stem>-a` / `<stem>-b` / `<stem>-combine`. Children keep
the stem and the node takes `-chain` / `-pair` so bare slugs stay unique and
surviving commit handles still name their artifact. A terminal suffix also keeps
stem-mates together; `review-<stem>` would group unrelated reviews.

Filename convention is not grammar: Grove validates no ordering and never
infers relationships from suffixes or positions. Explicit `Reviews` / `Integrates`
lines serve promotion and handoff. Nor is a chain a scheduling **unit**: `pick`
walks out after its children, though sibling insertion cannot split them.

When a **currently picked plain producer** needs fresh review, run `grove-llm
leaf-promote-chain <picked-producer>`. It atomically preserves the producer's
bytes and handle inside a derived brief-less chain, leaving a recoverable
`PROMOTING-*` witness on interruption. Finish to a reviewable boundary, commit
artifact plus promotion under that handle, retire the returned producer path,
then complete. Retirement writes the review's `Producer launch` receipt
best-effort after `DONE`, naming the producer, factual source session, and
producer generation; diversity warnings never block launch.

The tree is a real **directory tree** under `.grove/`: a node is a **directory**
`NN-<slug>-k<key>/` holding its numbered children (`01-…`, `02-…`), optionally
headed by a `BRIEF.md` charter — always one for a node a leaf *decomposed* into,
never one for a chain node; the filesystem carries the hierarchy, and `.grove/` is
itself the root node. Convert the leaf by running `grove-llm leaf-decompose <leaf-path>
<first-child-slug>`: the verb moves the leaf file `NN-<slug>-k<key>.md`
(`git mv`; a plain rename in a jj-enabled tree, where jj snapshots the working
copy) into a new directory `NN-<slug>-k<key>/` as its `BRIEF.md` (**keeping its permanent
key `-k<key>`** — the leaf that was `k<key>` becomes the *node* `k<key>`, same
position and slug), retitles the brief's position-free `# <slug>-k<key>` header
with ` — brief`, and atomically grows the node's first child
`01-<first-child-slug>-k<new>.md` (a node is never childless). Reshape the brief
body afterwards if needed (that part is judgement; the verb only does the
mechanical move). Grow the node further by running `grove-llm leaf-add <parent>
<slug>` (parent `.` for the grove root, or a node by its key or path) to append
a leaf at the node's next free child position with a fresh key (the common
case), or `grove-llm leaf-insert <target> <slug>` when a new concern must
sequence *ahead* of existing leaves — the insert verb shifts the target and
every later sibling up one position. Because the hierarchy lives in directories,
that shift is a single move of each sibling **directory** (`git mv`, or a plain
rename under jj) and the whole
subtree — child names *and* keys — rides along untouched; in-file `# …` headers
are position-free, so the renumber rewrites **zero file contents**. The verb
surfaces any stray **position-prefixed** cross-reference (`05-mid-k14`) on
stderr for the operator to review (it does not auto-rewrite — references may be
intentional historical pointers). Prefer the **permanent key** for any durable
cross-reference: a key never moves under renumber or a slug edit, and `grove-llm
resolve <ref>` turns a key (`[n]` / `n`), a bare slug, or the full
`<slug>-k<key>` handle back into the current file path. All three grow verbs are
working-tree changes only; the enclosing task's commit folds them in.

Every grow verb takes `--kind <kind>` — one of the seventeen, defaulting to
`impl` — and gates on it: an unrecognised value errors and lists the set, because
a human is present at authoring time (reading a kind degrades instead, so a
hand-edited file can never jam the loop). `leaf-add-chain` is the one exception
to the default: it **requires** `--kind`, since defaulting would silently pick
the producer that parameterises all three of its leaves. `leaf-decompose` gives
the node's first child its parent leaf's kind unless `--kind` overrides, so a
`research` leaf that proves bigger becomes a `research` node. A leaf may
additionally name the harness its session runs on with `--harness <name>`
(written as a `**Harness:**` line); almost none does — it exists for the **vendor
pair**, two `research` leaves differing only by vendor, which is the one shape a
per-kind policy cannot express, and `leaf-add-pair` writes both declarations for
you.

**Commit.** One task = one focused commit. **Name the work item in the commit
message by its stable handle `<slug>-k<key>`, never by its position or directory
path** — positions and paths move under renumber and reorder, but the
`<slug>-k<key>` handle is permanent, so the historical record stays meaningful
after restructures (task-tree-scheme §5).

**Retire.** A leaf ends one of two ways — **done** (the work was completed) or
**abandoned** (the path was decided against); grove's own metaphor: a done leaf
is *harvested*, an abandoned one is *pruned*. Both mark the leaf **in place**,
neither ever deletes it, and both are skipped by `pick`.

The common case: after committing the task, retire the just-finished leaf by
running `grove-llm leaf-retire <leaf-path>` — the verb marks it done **in
place** by adding a `DONE` infix (`NN-<slug>-k<key>.md` →
`NN-DONE-<slug>-k<key>.md`); there is no `done/` directory, and the leaf keeps
its position and key in its directory. The infix is filename-only — the file's
contents (including its `# <slug>-k<key>` header) are untouched. Mechanical
bookkeeping, no need to ask.

That same retirement may also close reviewed, brief-carrying decomposition
ancestors. While your leaf is still the factual pick, Grove prepares one
best-effort `Producer launch` receipt for the direct leaf and each such ancestor:
`producer` names the reviewed entity, `session` names this factual source
session, and `generation` is the greatest permanent key in the producer's
subtree. `DONE` lands first. Only a live linked review is rewritten; a terminal
review stays byte-identical and reports `review-terminal`. Receipt failure is
advisory and never reverses retirement. This side effect is why a reviewed node
close can change its live review task even though the node itself is never
marked.

The other case: a session finds the leaf's path decided against, not done. This
is **pruning**, and it is **HITL — an agent never prunes on its own**: an AFK
session (every kind but `requirements` and `prototype`) that discovers this says
so and stops; the
loop stalling on an abandonment decision is the system working, not a fault.
Only on explicit human confirmation, run `grove-llm leaf-prune <path>` (a leaf
or a node — given a node it marks every live leaf in the subtree, leaving `DONE`
ones alone, and refuses the grove root) to mark it `ABANDONED` in place.
Pruning a reviewed producer writes no handoff receipt and leaves a sibling
review live, next and deliberately uncheckable. If the human is abandoning the
whole reviewed path, prune the enclosing review-chain node instead; that marks
producer, review, and integration scope together.
`.grove/` dies at the finish cycle, so the mark records only *that* the path
closed — the durable *why* (what was rejected, why, and what would reopen it)
goes to the **ADR set**, the positive fact the abandonment establishes, if it
clears the when-to-write bar; otherwise the mark and the commit message suffice
(ADR *pruning*).

Then walk the parent chain: if a node now has no live leaf left in its subtree —
however its leaves finished — it is **implicitly done** — a brief is context,
not a task, so it is never marked done; its done-ness *is* the absence of a live
child. **The close asks the human nothing, for either node species.** A node is
never marked. A reviewed close may best-effort update its live review's factual
handoff receipt, but that is a fact this session establishes rather than a human
judgement; `leaf-add` reopens a close in error and changes its generation. That
is the first of the two tests deciding where grove *does* ask; ADR
*confirmation-boundary* carries both, and the second is why pruning and the
finish cycle still do.

Instead the session **verifies and reports**. On a **brief-carrying** node:

1. **Check** the node's brief `Done when` against what its subtree delivered.
2. **`leaf-add`** the missing work if the check fails and you can name the gap —
   a failed check names *work*, and grove has one answer for missing work.
3. **Escalate** — stop and say so — if the check fails and you *cannot* name the
   gap, because the residue is a scope judgement rather than work. That is an
   ordinary escalation, discretionary and always legitimate, not a routine gate.
4. **Promote** anything still relevant from the brief upward — to the parent
   brief, an ADR, or the glossary — so it stays in the brief chain of future
   siblings; and **report** the close by naming the node's `<slug>-k<key>` handle
   in the commit message alongside the leaf's own. The human reviews the close
   after the fact, in the diff. The brief and its now-terminal leaves stay
   exactly where they are (nothing moves).

A **brief-less** node — a chain node, written whole by `leaf-add-chain` /
`leaf-add-pair` — closes with **nothing to do**: no `Done when` to check and no
brief to promote, so every step above is vacuous. The discriminator is the
presence of `BRIEF.md`, a file test — never the node's slug.

Retirement is also the moment to **reconcile the ADR set**
with what the finished work established: edit it in place to keep it a minimum
coherent set (merge / split / delete), and fix any citation the rework leaves
dangling — in the briefs, the other ADRs, or `docs/`; never append a superseding
ADR (`linkuistics:decision-records`). That may leave the next ancestor with no
live leaf either; re-check and recurse, until a node still has a live
leaf or you reach the grove root — silently, so an unattended run crosses a whole
chain of closes without stopping. Terminal branches stay in the tree, marked in
place, never deleted while the grove is live — so a recursive view of `.grove/`
(`find .grove`, or a tree-style file manager) shows the whole state, done and
abandoned alike. The cascade walk and the brief-promotion-upward stay prose
deliberately: both are judgement steps (does the `Done when` hold? what survives
upward?) with no stable input/output shape that would justify a verb.

**Signal.** Once the task is committed and retired (and any parent-chain cascade
is settled), run **`grove-llm complete`** as your **last action — then do
nothing else**. This is how the self-driving loop ends this session and starts
the next task with fresh context: the verb only writes the relaunch flag to a
signal file (`GROVE_SIGNAL_FILE`) and returns. Ending the session is the **loop
driver's** job, not this verb's — the driver launched this session and is
watching for the signal file while it runs, so it applies grace → SIGTERM →
kill-grace → SIGKILL to its own child once the file appears (driver-side
watcher: the driver can always signal its child, unlike an in-agent self-kill,
which some harness sandboxes — e.g. codex's Seatbelt — silently deny). Run
outside `grove do` (no `GROVE_SIGNAL_FILE`) it is a safe no-op that just tells
you to exit manually. Plain `complete` signals a
**relaunch**; the **Finish** cycle below ends instead with **`grove-llm complete
--done`**, which signals a clean *stop*. The loop tells the three cases apart by
the signal: a relaunch flag, a `--done` flag, or no flag at all (a crash /
Ctrl-C, which stops).

**Finish.** A grove is ready to finish when it has no live leaves —
`grove-llm pick` exits 0 with empty stdout and "no live leaves; this grove is
done" on stderr. The **complete finish cycle** is driven in-session by the LLM
(no Rust automation): the session **proposes** it and **waits for explicit human
confirmation before any teardown** — never run steps 2–3 unprompted, so a
headless run with no human present simply reports the plan and stops. This is the
loop's **only routine human gate** (ADR *confirmation-boundary*) — everything else
a session asks is a discretionary escalation. On confirmation, run:

1. **Promote** anything from the briefs that should outlive the grove — ADRs,
   docs, glossary entries. Reviewable working-tree edits; often a near no-op
   when decisions landed inline as they were made.
2. **Delete `.grove/` in one focused commit.**
3. **End the loop cleanly**: run **`grove-llm complete --done`** as the **very
   last** action, then do nothing else. This signals the self-driving loop to
   *stop* (vs the per-task `complete`, which relaunches), so a clean finish is
   distinct from a crash or Ctrl-C. It must come last: like the per-task signal
   it ends this session after a short grace (applied by the loop driver, which
   is watching for the signal file — not this verb), so running it any earlier
   would cut teardown short. It writes only the loop's signal file (in the temp
   dir) — nothing about the working tree — so run it from any valid directory.
   Outside `grove do` (no loop to stop) it is a safe no-op: just exit.

Nothing after: integrating the grove's branch and tearing down the working tree
are **not** grove workflow — both belong to plain git/gh or jj, or the user's
own worktree tooling (user-owned-worktrees). Whoever integrates does so after step
2, so the integrated history never carries `.grove/`.

**Resume is state-checked, never a marker file** (constraint 1). `grove do` into
a half-finished grove resumes from the first incomplete step: if `.grove/` is
already gone (`grove-llm pick` errors with "grove root not found"), promotion
and deletion are already done — report "already finished" and stop.

## Artifacts

Only the task tree is grove-specific and ephemeral. Everything else is a
standard artifact that outlives grove (constraint 6).

| Artifact | Path | Role |
|---|---|---|
| Glossary | `CONTEXT.md` (+ `CONTEXT-MAP.md`) | the Ubiquitous Language — read every session, appended inline |
| ADRs | `docs/adr/<slug>.md` | one decision each, as a **minimum coherent set** — slug-named, edited in place; philosophy per `linkuistics:decision-records` |
| Specs | `docs/specs/<slug>.md` | how an area works — the human-facing agreement point, also a **minimum coherent set** (`SPEC-FORMAT.md`) |
| Task tree | `.grove/` (inside the grove's working tree) | the process: the self-extending decomposition of work; deleted at the in-session Finish step |

**The glossary is load-bearing.** The acute failure mode of multi-session work
is terminology drift: a later session, with no memory of an earlier one,
reinvents its term under a new name or reuses the words with a shifted meaning.
`CONTEXT.md`, read every session and appended *inline* whenever a term is
resolved, is the forcing function against that. Keep it a glossary and nothing
else — terse definitions, aliases-to-avoid, no implementation detail
(`CONTEXT-FORMAT.md`).

**Briefs vs. the glossary.** A bounded context is a *domain* partition; a
task-tree node is a *process* partition. They are orthogonal axes. The glossary
is per-bounded-context; a node that carries anything carries a `BRIEF.md`, not a
glossary.

## Specs

A **spec** is the human-facing, team-shareable design of an area of the system,
produced lazily by a `design` task *when the increment is a genuine agreement
point*. The flow there: grill → spec (review & agree) → decompose → execute.
Specs live in `docs/specs/<slug>.md` and, like ADRs, are a **minimum coherent
set describing the current design**: edited, merged, split in place; deleted
when one no longer describes anything (constraint 1 — the VCS holds the past).

Two rules keep the set honest. **Membership:** would a session on an unrelated
future grove need to read this? If not, it is a `BRIEF.md` and it dies with
`.grove/`. **Grain:** an ADR records one decision and its trade-off; a spec
describes how an area works, and *cites* the ADRs in its area rather than
restating them. Shape and the seam-sketching rule: `SPEC-FORMAT.md`.

## Reference files

- `BRIEF-FORMAT.md` — the `BRIEF.md` shape.
- `TASK-FORMAT.md` — the task-file shape and the closed task-kind taxonomy.
- `CONTEXT-FORMAT.md` — the glossary format (bundled from `mattpocock/skills`).
- `ADR-FORMAT.md` — grove's ADR **placement** note: where ADRs live, slug-named `docs/adr/<slug>.md`. Philosophy, format, and the when-to-write test live in the `linkuistics:decision-records` skill (see the prerequisite note below).
- `SPEC-FORMAT.md` — the spec shape, the membership and grain rules, and where agreed test seams are recorded. Seam philosophy lives in the `linkuistics:codebase-design` skill.
- `grilling.md` — the grilling procedure for `requirements` tasks (bundled).
- `driving.md` — field guide for driving grove sessions well: when to commission prior-art research, how to write a research-leaf brief, grilling moves (WDYT, pushback, running log), and when research findings retire into ADRs.
- `prompts/` — the launcher prompts read by the `grove` CLI at exec time (`start.md`, `continue.md`, `retire.md`). There is no `finish.md`: finishing is an in-session step of the loop, not a launched verb.

**Prerequisite — the `linkuistics` plugin.** Two bodies of guidance grove used
to carry now live in a **separately installed** plugin, and grove **requires**
it: ADR philosophy in `linkuistics:decision-records`, and what a test seam is
and how to judge one in `linkuistics:codebase-design`. The plugin is developed
in grove's own repo (`plugins/linkuistics/`), but the `grove` binary provisions
only grove's methodology — never the plugin — so it is installed on its own,
through the Claude Code marketplace or the repo's `plugins/install.sh`.
Self-containment is not a constraint for either body of guidance —
`ADR-FORMAT.md` and `SPEC-FORMAT.md` keep only grove's placement and recording
conventions. A session raising or reworking an ADR, or sketching a spec's test
seams, should consult the matching skill. The dependency is
documentation-level, not install-enforced; everything else grove needs stays
self-contained (constraint 6).
