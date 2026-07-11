# plan-k1

**Kind:** planning

## Goal

Grill the abandonment design space, settle it, and grow the tree that ships it.

## Context

The whole charter is in the root brief. Beyond it, this session in particular
needs:

- `gh issue view 2 --repo Linkuistics/grove --comments` — the four open questions
  below are lifted from the comment.
- `src/leaf.rs`, `src/leaf_id.rs`, `src/tree_read.rs` — where keys are assigned
  (`max + 1`), the `DONE` infix is written, and the walk skips.
- `git show 5177ea4` — the by-hand abandonment ritual this grove exists to
  replace.

## The questions (open, in dependency order)

1. **Visibility** — does an abandoned branch stay *visible* in the tree (an
   `ABANDONED` infix, symmetric with `DONE`), or vanish? Visibility cuts for the
   key counter and against tree noise.
2. **The durable record** — where does "we rejected this, here is why, here is
   what would reopen it" live, given briefs die at the finish cycle? Is an ADR's
   *Considered options* the general answer, or only the answer when the abandoned
   path had already reached an ADR?
3. **Taxonomy** — is abandonment one outcome or several (abandoned / blocked /
   deferred / superseded)? Does a taxonomy earn its place, or is that exactly the
   over-modelling the spine forbids? (Precedent: `task-kind-taxonomy` — a kind
   must earn its place with a distinct discipline.)
4. **The verb** — `leaf-abandon`, `leaf-retire --abandoned`, or nothing plus
   honest documentation?

Q1 is foundational (Q3 and Q4 only mean something once visibility is settled);
Q2 is independent of Q1 and can be grilled either side of it.

## Done when

- Each question above is settled with the human, and each settled answer is
  recorded inline in the running log below **as it lands**.
- `CONTEXT.md` is updated inline as terms resolve (*pruned*, whatever else).
- ADRs raised/reworked *sparingly* — and in place (the set is current-state).
- The tree is grown: leaves that ship the decision (methodology in `content/`,
  CLI, glossary, and issue #2 closed).

## Decisions (running log)

**D0 — Tree-visibility and the durable record are orthogonal, not a spectrum.**
`.grove/` is deleted wholesale at the finish cycle, so *nothing* in the tree
outlives the grove. Q1 is therefore only ever a question about this grove's own
lifetime; it cannot be the answer to "where does the rejection record live"
(Q2), which is the load-bearing one.

**D1 — Visibility: an abandoned leaf stays in the tree, marked in place.**
Symmetric with `DONE`: an infix, skipped by `pick` exactly as `DONE` is.
Rationale: (a) it closes the key-reuse defect *by construction* — `next_key` is
`max(key over all names) + 1` and the names *are* the counter, so the fix is to
never remove the name; every vanishing scheme needs a new source of truth for
the max (a counter file — constraint 1 forbids it — or git-log archaeology —
constraint 2 forbids it). (b) The `DONE` glossary entry already commits to this
principle: *"retirement is in place, so the tree always shows complete state"* —
a vanished abandonment makes the tree lie, showing a path never considered
rather than one considered and rejected. (c) The noise cost is bounded: the tree
dies at finish.
Accepted cost: a second skip-token in the walk and a second state in the tree's
vocabulary — which must earn its place (see D2).

**D2 — Taxonomy: one mark, not four.** No `blocked` / `deferred` / `superseded`.
The `task-kind-taxonomy` test governs — *a state earns its place only by carrying
behaviour beyond a name* — and none of the three does:
- `blocked` is already expressed by **ordering** (a prerequisite is an earlier
  sibling); the same ADR dropped upstream wayfinder's `task` type for exactly this
  reason ("the tree already sequences prerequisites, so grove has no
  blocked-decision concept"). It is also *actively wrong* as a skip-state: a
  blocked leaf is live work, so skipping it makes `pick` report "no live leaves;
  this grove is done" while work remains — the tree would lie and the finish cycle
  would mis-fire.
- `deferred` is a **reorder** (position is mutable) or a **GitHub issue** — both
  mechanisms exist; this grove was itself born from the latter.
- `superseded` differs only in *reason*, not behaviour. The filename's sole
  consumer is a boolean (*does `pick` skip this?*); the reason is prose and lives
  in the durable record (D3).

**D3 — The durable record: the ADR set, graded by the existing when-to-write
test. No new artifact class.** An abandonment always establishes a *positive fact
about the current design* ("we rejected cross-family review" ⇒ "grove is
single-provider"). That fact's ADR is the home; the rejection rides as a
**Considered-options** entry carrying three parts — what was rejected, why
(including *what is not the reason*), and **what would reopen it**. If no ADR
states the fact yet, write one: a deliberate non-action *is* a current-state
decision, and it passes the when-to-write test almost by construction (people
keep re-proposing it). If the abandonment is too small to clear that bar, nothing
durable is written — the tree mark and the commit message suffice, and that is
enough.
`linkuistics:decision-records` already designates the ADR set for this ("record
it, or someone re-proposes GraphQL in six months") and already supplies the
grading test, so grove adds **no machinery** here — only the discipline.
The one genuinely new component is the **reopening trigger**, which the skill does
*not* yet ask for. It is the difference between a tombstone and a gate with a key
(worked example: `model-per-task-kind`, whose rejection entry ends "What would
reopen this: a coherent provider/credential design for grove, or evidence that
actually measures the cross-family increment"). Push it upstream into
`linkuistics:decision-records`, not down into a grove-local habit.

**D4 — The verb: `leaf-prune`, a sibling of `leaf-retire`, not a flag on it.**
Mechanically the two are the same operation (a `git mv` adding an infix) and share
a code path, so the split costs nothing in code and buys the vocabulary. A flag
(`leaf-retire --abandoned`) would frame abandonment as a *mode of completion*,
which the skill prose would then have to keep apologising for — and forgetting the
flag would silently assert the work was *finished*. A missing flag must degrade to
something harmless, never to the opposite outcome. Omitting `leaf-prune` fails
safe: the leaf just stays live.

**D5 — The token: `ABANDONED`.** The verb carries grove's metaphor; the *state
token stays plain*, exactly as `leaf-retire` writes `DONE` and not `HARVESTED`.
Decisive argument is constraint 6 (walk-away-able): with the skill deleted,
`03-ABANDONED-spike-k3.md` needs no explanation, while `PRUNED` needs the metaphor
*and* collides with git's own `prune` (= **delete**: `git remote prune`, `gc
--prune`) — a cold reader could take it to mean "slated for removal" while the file
sits right there. Having to write a `CONTEXT.md` **_Avoid_** note defending a token
against a misreading drawn from the very tool grove is built on is the tell that
the name is fighting its context. `DONE` never needed defending.
Accepted cost: 9 chars vs 4, so the glossary's "at a fixed column" property becomes
*per-mark* rather than global. Cosmetic only — `sort_key` parses the position
first, so ordering is untouched.

**D6 — Arity: `leaf-prune` takes a leaf *or a node*; on a node it marks every
*live* leaf in the subtree** (leaves `DONE` ones alone — that work really was
done), reports what it marked, and refuses the root `.` (abandoning a whole grove
is a branch-delete, not a tree mark).
The asymmetry with `leaf-retire` is real and is the justification: **retirement is
incremental** (one leaf per session, as each piece of work completes) but
**abandonment is bulk by nature** — one decision kills N leaves at once. The worked
example proves it: `5177ea4` killed `spike-k12` *and* the `impl-k13` it gated, on a
single decision. The two verbs differ in *arity by nature*, not just in token. A
leaves-only verb would make abandoning a subtree an N-call chore, rebuilding the
tedium that drives people to hand-edit — the exact failure this grove exists to fix.
A node itself never carries the mark (`parse_stem` forbids an infix on a directory);
its state remains "no live leaf remains", and the existing retire-cascade applies
unchanged.

**D7 — Pruning is HITL: it requires explicit human confirmation.** An agent must
never unilaterally abandon planned work. Same guard as the retire cascade's "ask
the user before treating a node as done" — and for the same reason: the
confirmation is the moment a human can say "no, that path is still alive." An AFK
session (`research` / `work` / `review`) that finds a leaf dead says so and stops;
the loop stalling on an abandonment decision is the system working, not a fault.
Rejected: letting an agent prune when a premise is "objectively disproven" — that
is exactly the judgement an overconfident agent makes badly, and the failure mode
(quietly closing a path the human still wanted, under a mark that reads as though
the *human* closed it) is the one this design most needs to prevent.

## Outcome

Written here: ADR *pruning* (new), ADR *task-tree-scheme* (edited in place — the
grammar and the key-counter rule), `CONTEXT.md` (new *Pruning* entry; *Leaf*,
*Permanent key* and *DONE infix* reconciled).

Tree grown: `prune-verb-k2` → `methodology-k3` → `decision-records-skill-k4` →
`review-k6` → `release-k5`.

Two incidental findings, **externalised not absorbed**:
- **Issue #3** — `leaf-insert` renumbers with `git mv`, which fails on an *untracked*
  source, so you cannot insert ahead of a leaf added in the same session. Hit while
  growing this very tree. A different verb from the one this grove is about; filed.
- **v9.1.0 gates on read.** `grove-llm kind` errors on any kind outside
  `work`/`planning`, and the loop driver calls it every iteration — so a canonical
  `**Kind:** review` leaf would jam *this* grove's loop. `main` already fixes it
  (degrade-on-read); it is merely unreleased, so no issue filed. Every leaf here
  therefore says `work`/`planning` on disk, `review-k6` carrying its discipline in
  prose. Verified by probe, not assumed.
