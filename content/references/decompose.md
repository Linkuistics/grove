<!-- file: order=24 -->
<!-- unit: skill-two-triggers-two-verbs class=procedural defers="driving-externalizing-surfaced-work brief-the-node-briefing driving-what-a-good-child-leaf-looks-like skill-directory-tree-and-grow-verbs" -->
## Two triggers, two verbs

When work surfaces mid-session, default to **externalizing it as a new leaf**
rather than absorbing it into the current session — grove's value is many small,
low-context sessions, and that value is lost the moment a session quietly grows
to cover work that should have been its own leaf.

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

<!-- unit: skill-directory-tree-and-grow-verbs class=procedural defers=skill-kind-on-the-tree-verbs -->
The tree is a real **directory tree** under `.grove/`: a node is a **directory**
`NN-<slug>-k<key>/` holding its numbered children (`01-…`, `02-…`), headed by a
`BRIEF.md` charter — a node is always a leaf that *decomposed*, so it always has
one; the filesystem carries the hierarchy, and `.grove/` is
itself the root node. Convert the leaf by running `grove-llm leaf-decompose <leaf-path>
<first-child-slug>`: the verb moves the leaf file
`NN-<session-kind>-<slug>-k<key>.md`
(`git mv`; a plain rename in a jj-enabled tree, where jj snapshots the working
copy) into a new directory `NN-<slug>-k<key>/` as its `BRIEF.md` (**keeping its permanent
key `-k<key>`** — the leaf that was `k<key>` becomes the *node* `k<key>`, same
position and slug, and **dropping the kind**, which a node has no use for),
retitles the brief's position-free `# <slug>-k<key>` header
with ` — brief`, and atomically grows the node's first child
`01-<session-kind>-<first-child-slug>-k<new>.md` (a node is never childless). Reshape the brief
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
`<slug>-k<key>` handle back into the current file path. Every grow verb is a
working-tree change only; the enclosing task's commit folds them in.

<!-- unit: skill-kind-on-the-tree-verbs class=procedural -->
**`--kind <kind>` appears on the grow verbs whose kind is a free choice**, and
every one that accepts it gates on it: an unrecognised value errors and lists the
nineteen, and driver-reserved `finish` is refused, because a human is present at
authoring time. `grove-llm leaf-add <parent> <slug> --kind <kind>` and
`grove-llm leaf-insert <target> <slug> --kind <kind>` take it with the `impl`
default — and since a review chain is now cut one `leaf-add` at a time, `--kind`
is where you name `review-<producer>` and `integrate-review-<producer>`
yourself, off the producer that actually ran. `leaf-decompose` takes it as an
*override* of the kind it otherwise inherits from the leaf being decomposed — the
node directory it creates carries none — so a `research-a` leaf that proves
bigger keeps producing `research-a` work in its first child. **`leaf-add-pair`
takes no `--kind` at all**: its three kinds (`research-a`, `research-b`,
`combine-research`) are fixed by the shape, so there is nothing to choose.
**Reading is strict too**: every task-shaped leaf
filename, live or terminal, must carry a known kind, and a missing or unknown one
stops tree operations naming the path and the valid set rather than degrading to
`impl` — the kind is a configuration key, and a kind grove cannot spell is a
session it cannot launch. No grow verb
selects a harness, a model, or anything else about the launch: the kind is the
whole routing input, and configuration maps it to one command.

<!-- unit: skill-cut-the-next-step class=procedural defers="task-review-chain-mechanics task-vendor-pair-mechanics" -->
**Cut the next step, when it is needed.** When more than one leaf serves *one*
artifact, two shapes are the habitual answer — reach for them by default, and
argue yourself *out* of one rather than into it. They are built in **opposite**
ways, and the asymmetry is the design:

- **The review chain** — `X` → `review-X` → `integrate-review-X`: a fresh
  context asked to *disprove*, then a leaf licensed to act on what it found. Its
  steps are **ordinary flat siblings**, and **each session creates the next, only
  when it is required**:

      grove-llm leaf-add <parent> <stem> --kind review-<producer>
      grove-llm leaf-add <parent> <stem> --kind integrate-review-<producer>

  The **last act of a producer session** is to decide whether review is required
  and, if so, cut the `review-<producer>` leaf itself. The **last act of a review
  session** is to cut the `integrate-review-<producer>` leaf — **only if it has
  findings worth acting on**. A review that finds nothing creates nothing and
  simply retires; that empty session is the one this shape exists to remove.
  Decide for review when the artifact is load-bearing — a spec, a decomposition
  you will build on for months, a subsystem. One narrow, unexpected doubt in a
  picked plain producer may use its single in-session reviewer instead
  (`driving.md`).

  **You write the new leaf's body, and that is the payoff.** Because the leaf is
  cut by the session that knows why it is needed, it can carry **specific
  instructions, findings and data** — a review leaf naming the exact case its
  producer could not cover, an integrate leaf carrying the findings verbatim.
  That is strictly more than the generic template a constructor could write up
  front, and it is why the creating session is the right author. The template
  you get is bare on purpose: a handle and empty sections, nothing to edit
  around.

  You name the step kind yourself, so **name it off the producer**:
  `review-<producer>` and `integrate-review-<producer>`, for the producer that
  actually ran. `--kind review-impl` beside a `design` producer is a perfectly
  valid invocation that silently gives the reviewer the wrong discipline, and
  nothing will catch it. Each step resolves its own `review-` /
  `integrate-review-` configuration entry, so the kind is the whole routing
  decision.

- **The vendor pair** — `research-a` → `research-b` → `combine-research`, the two
  surveys differing *only* by which configured target runs them. Cut it when the
  question is load-bearing enough to pay for two corpora. **This one is still
  one eager call**, and the whole shape lands or none does:

      grove-llm leaf-add-pair <parent> <stem>

  **Laziness would be wrong here, which is why the pair kept its verb.** If
  `research-a` cut `research-b` at the end of its own session, `b` would inherit
  `a`'s framing and corpus — destroying the independence the pair is run for.
  Eagerness is the point for a pair; it is not for a chain. The two producers are
  **separate session kinds**, so each resolves its own configuration entry and
  neither task file carries routing metadata. The tree guarantees two independent
  sessions and the combine discipline; whether they reach two genuinely different
  vendors is the configuration owner's policy, not something grove can recover
  from an opaque command template.

<!-- unit: skill-cutting-a-review-leaf class=procedural -->
**When a picked producer needs fresh review**, the answer is the same
`leaf-add`. Finish to a reviewable boundary, cut the `review-<producer>` leaf
with the specific doubt written into its body, retire the producer, and commit
the artifact plus that leaf plus the retirement under the producer's handle —
the Retire-then-Commit order. Retiring it is the filename `DONE` transition
and nothing else, and it leaves the review byte-identical, because the review
needs no record of how its producer ran. The next loop iteration picks that
review and resolves its command from the `review-*` entry in configuration.

<!-- unit: skill-why-the-stem-is-bare class=procedural defers=task-bare-stem-reasoning -->
## Why the stem is bare

**The kind field is the canonical statement of a leaf's role; the slug names the
artifact and does not restate the kind.** A step marker in the slug was a 1:1
restatement of the kind sitting right beside it — a second, *unvalidated* source
of truth for a fact grove already parses and routes on. Nothing stops `leaf-add
<parent> foo-review --kind impl`, and when the two disagree the slug lies while
the filename tells the truth. So a chain reads `<stem>` / `<stem>` / `<stem>`,
three leaves differing only by kind and key, and `grove-llm resolve <stem>`
prints the whole chain with each step's role spelled out in its path. Both
spellings remain legal filenames, the grammar is unchanged, and **no existing
tree is invalidated** — an older suffixed slug you meet is fine and stays as it
is (`TASK-FORMAT.md` carries the full reasoning).

<!-- unit: skill-which-hop-a-gap-costs class=procedural defers="skill-integration-placement task-chain-contiguity" -->
## Which hop a gap costs

Steps are appended at the parent's next free position, so a step cut after some
unrelated leaf already exists lands *after* that leaf, and a later `leaf-insert`
can split a chain that was contiguous. Grove refuses none of it — it validates
no cross-leaf grammar, `pick` is a walk and not a scheduler, and contiguity was
never an enforced unit. What decides whether a gap costs anything is **what the
next step consumes**:

- **A `review-*` step re-derives, so `leaf-add` is right wherever it lands.** Its
  handoff is the producer's **stable handle**: the review's own body names it
  under `**Reviews:**`, every task commit message names the work item by that
  handle, so the reviewer finds the producer's commit from the handle and reads
  that diff against the current source. Nothing had been written down for
  intervening work to stale. That is narrower than *free* — work that rewrote the
  reviewed artifact, its requirements, or its recorded evidence leaves the
  reviewer reconciling a historical diff with a tree that has moved — but the
  reconciliation is visible, and the reviewer performs it deliberately.
- **An `integrate-review-*` step consumes, so a gap corrupts.** A review's
  findings are anchored to a commit and to `path:line` coordinates, and the
  integrating session opens the working tree as it *then* stands. Any intervening
  edit to a cited file moves those lines, and the drift is **silent**: nothing
  errors, the finding simply points somewhere slightly wrong, and the integrating
  session has to re-derive what the reviewer meant from a codebase the reviewer
  never saw.

<!-- unit: skill-integration-placement class=procedural -->
So an integration is cut **where `pick` reaches it next**, and the condition for
that is mechanical and **directory-local**. Read the review's *own parent*
directory at the entries after the review's position, and find **the first
sibling entry after the review whose subtree still holds live work** — leaf file
or node directory, whichever comes first. That entry is what blocks, and it is
the insert target:

```text
grove-llm leaf-insert <first blocking sibling entry> <stem> --kind integrate-review-<producer>
```

Three things that condition gets right where "the first live leaf after it" does
not:

- **Terminal entries never block.** A later `DONE` or `ABANDONED` leaf, and **a
  node whose subtree is wholly terminal**, are stepped straight over; so is the
  driver's `finish` sentinel, which is skipped while any ordinary work is live.
- **A later sibling *node* blocks as one entry, and the node is the target.**
  `pick` descends a node in place, so a live leaf anywhere beneath a later
  sibling node runs before anything appended after that node. Insert at the
  **sibling directory**, never the live leaf inside it — targeting the descendant
  inserts at the wrong level, dropping the integration inside a node whose brief
  does not charter it.
- **Nothing outside the directory can intervene.** Pre-order finishes the
  review's own directory — including an integration just appended to its end —
  before it visits any later sibling of an *ancestor*. So live work in an outer
  sibling node is irrelevant, and when no sibling entry blocks, plain `leaf-add`
  is exactly right.

<!-- unit: skill-why-there-is-no-exception class=procedural -->
## Why there is no exception to check

Adjacency is unconditional guidance, not because departing is forbidden — grove
enforces none of this — but because the check an exception would need cannot be
performed: at the moment a review cuts its integration the intervening leaf has
not run, and grove makes no leaf's eventual file set part of its contract, so a
goal or pointer list is not proof of what it will touch. A session that departs
anyway owns the drift.
