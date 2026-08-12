# narrative-k39

**Integrates:** `classification-k35`

## Goal

Strip the narrative that has no mandate-delivery job, and give every rule it was
carrying an honest triggering statement of its own.

This is `classification-k35`'s finding **2** plus the closing clause of finding
**4** — one species of work, and one set of lines. The review said so itself:
*"They are one species: narrative addressed to a reader of a **file**, in a
corpus that no longer delivers files."*

## The boundary, and it is the opposite of your sibling's

`addressing-k38` was constrained to leave the pinned unit set byte-identical.
**You will move it.** Every unit you remove or re-root changes `EMBEDDED_UNITS`
in `tests/methodology.rs` (143 ids as `addressing-k38` left it), so the pinned-set
test is your confirmation point in **both** directions — an id you deleted but
did not unpin fails, and so does one you minted but did not name.

The invariants that do **not** move: byte partition (every non-preamble byte
still covered by exactly one unit), `defers=` resolution and target class, and
reachability. Deleting prose shrinks the corpus; it must not leave a hole
between units.

## Finding 2 — narrative units made artificial triggering roots **[high]**

Verbatim from the review:

> **Narrative units have been made artificial triggering roots.**
> `skill-loop-diagram` contains an overview diagram, not a condition a session
> can recognize. `driving-field-guide` contains file-level framing and an
> anchor index, not a condition either; its generic `kinds=*` root also hides
> the concrete prohibitions in procedural `driving-anti-patterns`. Rework or
> remove this file-reader prose and give each rule that must ship an honest
> triggering statement. Do not preserve narrative reachability by calling the
> narrative a trigger, and do not leave the anti-pattern conditions reachable
> only through a generic table of contents.

### What is already traced, so you start from facts

- **`skill-loop-diagram`** — `content/SKILL.md:14–51`, `kinds=* class=triggering`,
  1,851 B of mermaid shipping to all nineteen mandates. `spine-k21` chose
  triggering on the ground that *there is no honest root for it as a body* —
  rooting it from `skill-what-a-grove-is` would be exactly the artificial root
  the node brief warns against. **That reasoning is what the review rejects**:
  the absence of an honest root is an argument for removing it, not for calling
  it a condition. Removal is a real content decision — say what a session loses.
- **`driving-field-guide`** — `content/driving.md:1–35`, `kinds=* class=triggering`,
  and it **fuses** two different things: genuine file-level framing (L2–16) and
  `## In this guide` (L18–35), an 18-line index whose every entry is a markdown
  anchor that resolves only for a reader holding the whole file. It roots three
  units.
- **The rooting fact that makes this structural, verified at bootstrap:**

  | unit | inbound edges | survives losing `driving-field-guide`? |
  |---|---|---|
  | `driving-externalizing-surfaced-work` | **3** — `skill-decompose` (`kinds=*`), `task-producer-impl` (`kinds=impl`), `driving-field-guide` | **yes** |
  | `driving-anti-patterns` | **1** — `driving-field-guide` only | **no** |
  | `driving-the-shortest-version` | **1** — `driving-field-guide` only | **no** |

  So the two units with a single inbound edge are held up entirely by the table
  of contents. That is the review's *"do not leave the anti-pattern conditions
  reachable only through a generic table of contents"* stated mechanically.

### What the anti-patterns need

`content/driving.md:747–768` is four **prohibitions** — the wizard, the
decision-summary-at-session-end, the "ask if you have questions" non-prompt, the
pre-baked answer. A prohibition is the triggering shape (`shape-cutting-k30`
argued exactly this for `skill-no-exception-to-check`, and the review ratified
it). Decide whether they become a triggering unit outright, or a triggering
condition with the expansions deferred, and scope them honestly — at least the
pre-baked-answer one reads as `requirements`-specific.

`driving-the-shortest-version` (L769–783) is a *"if you remember one paragraph"*
recap. Under mandate delivery there is no document to recap. Removing it is the
expected outcome; keeping it needs an argument.

## Finding 4's closing clause — the residue-and-restatement sweep **[medium]**

Verbatim, and **only this clause** — the three named repairs before it are
`addressing-k38`'s:

> In the same focused prose cleanup, remove file-reader-only residue and
> near-verbatim hub restatements that have no mandate-delivery job. Preserve
> genuinely useful rules by restating their conditions, not by retaining a
> document-navigation shell.

The candidate set, assembled from review design findings 3 and 5 so you triage a
list rather than hunt one. **It is a candidate set, not a deletion order** —
each entry is a call to make and record.

### File-reader residue (design finding 3)

| candidate | where | why it is residue |
|---|---|---|
| `skill-reference-files` | `SKILL.md:780–791`, procedural | **its rows name files, and a session cannot fetch a file.** `grove-llm methodology` addresses units by id. Its only root is `continue-launcher-framing` — and `finish-cycle-k32` noted that root **expires at the same moment its content does**, since `continue.md`'s *"use the grove skill"* becomes false when provisioning retires |
| `## In this guide` | `driving.md:18–35`, inside `driving-field-guide` | dead anchors under mandate delivery; also finding 2's artificial root |
| `driving-the-shortest-version` | `driving.md:769–783` | recap of a document nobody receives |
| the authoring note | `driving.md:14–16` (*"The examples are stated as reusable shapes…"*) | addressed to a reader of the file |
| constraint-2 parenthesis | inside `skill-spine-constraints`, `SKILL.md:53+` | residue **and stale** — it describes re-provisioning that `mandate-delivers-the-methodology` retires, and **no stage-4 item in the root brief currently repairs it**. If you do not fix it here, say so, because nothing else is scheduled to |
| `skill-finish-no-signal-stop`'s second half | `SKILL.md:719+` | flagged by `finish-cycle-k32`; judge it |

### Near-verbatim hub restatements (design finding 5)

`shape-cutting-k30` found **five of seven paragraphs** in `SKILL.md`'s
shape-cutting region are the hub restating a rule `TASK-FORMAT.md` owns, and
**three are near-verbatim duplicates rather than expansions**:

| unit | `SKILL.md` | its only root (a `shape-cutting-k30` inventory addition) |
|---|---|---|
| `skill-no-node-for-a-shape` | L320–327 | A1 — `task-no-node-for-a-shape` |
| `skill-declare-the-relationship` | L343–349 | A2 — `task-declare-the-relationship` |
| `skill-grammar-is-five-fields` | L351–365 | A3 — `task-grammar-is-five-fields` |

`shape-cutting-k30` recorded that these are *"same-rule roots, which is the test,
but weaker than the usual body-answers-condition shape"* — `task-grammar-is-five-fields`'s
closing sentence and `skill-grammar-is-five-fields`'s are **the same argument in
the same words**. The review's reading: *"Reading the two files as documents, the
repetition looks like emphasis; reading them as a unit graph, it is three edges
that carry no information."*

Deleting a body means **also removing its `defers=` member** from the
`TASK-FORMAT.md` owner's marker, or the build fails on an unresolved target — the
one place in this sweep where the gate protects you.

**Check before deleting, not after:** each `SKILL.md` twin must be a genuine
restatement, not an expansion carrying a clause its `TASK-FORMAT.md` owner
lacks. `shape-cutting-k30` made exactly this check for family F and found the
owner already shipped the condition; repeat it per unit. A clause that exists
only in the hub copy is a rule that would ship **nowhere** after deletion — the
silent direction, and the failure this whole review chain exists to catch.

## Done when

- Neither `skill-loop-diagram` nor `driving-field-guide` remains an artificial
  triggering root — reworked, re-rooted, or removed, with the call recorded.
- The four anti-pattern prohibitions ship on an **honest condition**, not through
  a table of contents; same for anything else the sweep unroots.
- Every residue and restatement candidate above is **triaged row by row** —
  removed, or kept with a reason. A kept row needs a stated mandate-delivery job.
- Every rule that survives a deletion survives as a **restated condition**, never
  as a retained navigation shell (the review's own wording).
- Byte partition, `defers=` resolution, target class and reachability all hold;
  `EMBEDDED_UNITS` updated deliberately in the same commit, every id named by a
  decision.
- `cargo build` and `cargo test` green; verified from a **rebuilt, installed**
  binary with per-kind retrieval spot-checks on the re-rooted paths — including
  at least one showing the anti-pattern prohibitions arriving without the index.
- The **per-mandate byte delta** is reported. This leaf is the largest deletion
  in the classification; the number is the evidence that the sweep did what the
  design intends.

## Notes

- **Decompose rather than absorb** if the sweep proves bigger than one focused
  session. The seam to look for is *additive* (writing honest triggering
  statements for rules that lose their root) against *subtractive* (deleting
  prose with no mandate-delivery job) — they are separable once finding 2's
  collision on `driving-field-guide` is resolved, which is why that collision is
  the first thing to settle.
- **Retiring this leaf closes two nodes** — `classification-k37`, and then
  `classification-k9`, whose last live leaf this subtree is. Check both briefs'
  `Done when`, promote what survives upward (the root brief's *Successor grove*
  charter is the natural destination for anything the successor must not lose),
  and name every closed handle in the commit message.
- The root brief's stage-4 list enumerates what the successor deletes. If a
  removal here lands work that list expects to do later — `## Reference files`
  and the constraint-2 parenthesis are both candidates — reconcile that list in
  the same commit rather than leaving it describing a corpus that has moved.

## Batch record

**Eight units retired, three minted, one net class change: 143 → 138.** `cargo
build` green; `cargo test` green — **1,018 tests, 0 failures** (1,023 minus the
five tests that existed only to police the deleted navigation index; every one
accounted for below). No `pending-` unit remains.

### The boundary this leaf was given, and it moved in both directions

`addressing-k38`'s check was *the pinned set must not move*; mine is the
opposite — every id deleted must be unpinned and every id minted must be named.
`the_embedded_unit_set_is_pinned_complete` is green at 138, which fails in both
directions, so the array below is the confirmation point:

| retired | why |
|---|---|
| `skill-loop-diagram` | overview diagram; no condition |
| `driving-field-guide` | file framing + anchor index; no condition |
| `driving-anti-patterns` | replaced by three honest prohibitions |
| `driving-the-shortest-version` | recap of a document nobody receives |
| `skill-reference-files` | rows name files a session cannot fetch |
| `skill-no-node-for-a-shape` | near-verbatim hub twin |
| `skill-declare-the-relationship` | near-verbatim hub twin |
| `skill-grammar-is-five-fields` | near-verbatim hub twin |

| minted | scope | states |
|---|---|---|
| `driving-no-session-summary` | `*` | do not reconstruct decisions at session end |
| `driving-ask-about-the-trade-off` | `*` | never close by inviting questions in general |
| `driving-no-pre-baked-grilling` | `requirements` | a pre-decided question is not a grilling question |

### Finding 2 — both artificial roots removed rather than reworked

**`skill-loop-diagram`: removed.** `spine-k21` chose `triggering` because there
is no honest root for it as a body; the review's answer — that the absence of an
honest root argues for removal, not for calling it a condition — is accepted, and
one further fact settles it. **A mermaid fence delivered as `${prompt}` bytes is
never rendered.** In a provisioned skill file a human opening it in a viewer sees
a picture; in a mandate it is 1,851 B of diagram *source* in argv, shipped to all
nineteen kinds. What a session loses is a graphical overview whose every node is
separately stated by the triggering unit that owns that step — `skill-pick`,
`skill-bootstrap`, `skill-execute`, `skill-decompose`, `skill-retire`,
`skill-node-close-cascade`, `skill-commit`, `skill-signal` — and whose ordering
claim (`retire → commit → signal`, commit after the cascade) is stated in
`skill-commit`'s own prose. That is the whole loss, and it is a rendering, not a
rule.

**`driving-field-guide`: removed, and it took the file's H1 with it.** It fused
three things and not one of them survives the test. The framing (*"this file is
about how to drive it well"*) addresses a reader opening a file. The authoring
note (*"the examples are stated as reusable shapes…"*) addresses a maintainer of
one. `## In this guide` is eighteen lines of markdown anchors that resolve only
for a reader holding the whole file — and its *"treat it as a starting set of
habits, not a checklist"* qualifier already ships at `kinds=*` as constraint 5.
`content/driving.md` now opens with its first unit's marker and a
`<!-- grove reference file — … -->` note, matching what `TASK-FORMAT.md`,
`BRIEF-FORMAT.md` and `SPEC-FORMAT.md` already do.

**The index cost more than it looked.** Removing it made ~140 lines of
`tests/reference_navigation.rs` dead: `guide_navigation_lines`,
`navigation_anchors`, `is_nested_list_item`, `is_ambiguous_unsuffixed_target`,
and four tests — a bespoke re-implementation of GitHub's heading-anchor slug
rules, dedup suffixes and loose-list scanning, maintained *solely* to keep a
navigation shell honest. `markdown_headings` survives for the repository-wide
link sweep and was simplified to return anchors, its `base_anchor` and
`line_number` fields having had no other reader.

### The four prohibitions, re-scoped rather than re-rooted

A prohibition is the triggering shape (`shape-cutting-k30`'s argument for
`skill-no-exception-to-check`, ratified by the review), so the answer was to make
them conditions, not to find them a better root. They were **not one species**,
which is why they are now three units and not one:

- **The decision summary at session end** → `driving-no-session-summary`,
  `kinds=*`. A genuine universal prohibition, and the negative of
  `driving-record-decisions-inline`; it now names constraint 1 outright.
- **The "ask if you have questions" non-prompt** → `driving-ask-about-the-trade-off`,
  `kinds=*`. Scoped `*` deliberately: escalation is universal in grove (a node
  close that cannot name its gap, a prune, the finish confirmation), not a
  grilling-only move. The old text made it look `requirements`-shaped only
  because it cited the grilling format.
- **The pre-baked answer** → `driving-no-pre-baked-grilling`,
  `kinds=requirements`, as the leaf predicted.
- **The wizard: removed, and this is the one deletion that removes a claim rather
  than relocating one.** It is prior-art evidence about *grove's own CLI* — that
  a capture verb must not open an interactive prompt sequence — cited to *"ditz's
  failure mode in the postmortem survey"*, **a document this repository does not
  contain**. It is not a condition any grove session recognizes: a session never
  designs a capture verb, and the rule it argues for is already embodied in
  `leaf-add`'s flag-driven interface and in
  `driving-externalizing-surfaced-work`'s *"append it to the tree with `leaf-add`
  and keep driving"*. What is lost is the citation, and the citation was already
  dangling.

`driving-the-shortest-version` removed as the leaf expected: every clause of the
recap — research briefs naming downstream questions, primary-source citations,
one question at a time, WDYT, pushback, inline decisions, ADRs citing
research — is stated by the unit that owns it. There is no document to recap.

### Finding 4's sweep — every candidate row triaged

| candidate | call |
|---|---|
| `skill-reference-files` | **removed.** Its rows name files; `grove-llm methodology` addresses units by id. Its only root, `continue-launcher-framing`, now carries no `defers=` — legal, and it means the unit is complete as delivered |
| `## In this guide` | **removed** with `driving-field-guide` |
| `driving-the-shortest-version` | **removed** |
| the authoring note | **removed** with `driving-field-guide` |
| constraint-2 parenthesis | **rewritten, not removed.** Its second sentence — *the boundary is that build, not a commit* — is still true and load-bearing under mandate delivery. What went is *"re-provisions it on every lifecycle invocation"* and *"the skill you are reading"*, both false the moment provisioning retires. It now reads *"the guidance you are reading"* against *"editing `content/` changes nothing any session **receives**"* |
| `skill-finish-no-signal-stop`'s second half | **removed.** Its subject is a *driver* death, so no session is present to act on it, and the session-actionable half is already in `skill-finish-resume` (*"a later bare `grove` into a rootless tree is an ordinary fresh grove, not a resumed finish"*). The epoch rotation, the `plan-k1` reuse and the 30-second handoff bound are durably recorded in `docs/adr/one-live-driver-per-working-tree.md` |

### The three hub twins — checked before deleting, per the leaf's instruction

Each was diffed clause-by-clause against its `TASK-FORMAT.md` owner, hunting the
silent direction: a clause living **only** in the hub copy, which deletion would
ship nowhere.

- **`skill-declare-the-relationship`** — no delta. The owner is strictly richer
  (it carries the worked example). Deleted.
- **`skill-grammar-is-five-fields`** — no delta. *"a `review-*` kind does not make
  a leaf review its neighbour"* is the owner's *"It never reconstructs a
  relationship from a filename, a position, or a body"*; the deleted-suffix
  argument is the same argument in the same words, which is the review's point.
  Deleted.
- **`skill-no-node-for-a-shape`** — **one real delta, and it was preserved rather
  than dropped.** The hub explained *why* a node has a brief (*"its `BRIEF.md` is
  the context those extra sessions need"*); the owner asserted only that it has
  one. That clause is now folded into `task-no-node-for-a-shape`, so the rationale
  ships and the twin does not. The other apparent delta — *"a shape's steps sit as
  flat siblings"* — turned out already stated by `task-review-chain-mechanics`
  (*"Its steps are ordinary **flat siblings**"*), and is kept in the fold anyway
  because it costs one clause and the owner is the triggering unit.

All three owners lost their `defers=` and now carry none.

### Two residue sites the candidate set did not name, found by measuring

The list was a candidate set, not a completeness claim, so the sweep was run as a
**property check on the composed mandate** rather than as a walk of the six rows:
*no mandate may contain a link a session cannot follow, and no triggering unit
may open with a back-reference.*

- **`task-kind-in-the-filename`** carried `[work-item handle](#suggested-shape)`
  — a dead anchor shipping in all nineteen mandates, with no `defers=` behind it.
  The link is gone and the phrase is bold text; no edge was added, because the
  reference is a citation for a term, not a trigger→body relationship, and
  `task-leaf-filename` already addresses `task-suggested-shape` at `kinds=*`.
- **`task-grammar-is-five-fields`** opened *"The grammar is the five fields
  **above**…"*. `addressing-k38` ruled a back-reference a defect in a *triggering*
  unit, and this one became acute the moment its hub twin went: it is now the sole
  statement of the rule. It opens *"A leaf name is five fields and nothing
  more."*

Both are prose-inside-unit repairs and move no id.

### Evidence, instrument controlled first

**Byte partition, proved by reconstruction from the binary's own embed** — every
unit of every file fetched in listing order and compared against that file's body
(preamble excluded). `grilling.md`, `BRIEF-FORMAT.md`, `SPEC-FORMAT.md`,
`ADR-FORMAT.md` and `CONTEXT-FORMAT.md` are **positive controls** — files this
leaf never opened:

| file | units | body | result |
|---|---|---|---|
| `SKILL.md` | 43 | 49,808 B | partition exact |
| `driving.md` | 29 | 44,248 B | partition exact |
| `TASK-FORMAT.md` | 38 | 32,789 B | partition exact |
| `grilling.md` | 9 | 5,803 B | partition exact *(control)* |
| `prompts/continue.md` | 1 | 855 B | partition exact |
| `BRIEF-FORMAT.md` | 5 | 4,896 B | partition exact *(control)* |
| `SPEC-FORMAT.md` | 7 | 5,916 B | partition exact *(control)* |
| `ADR-FORMAT.md` | 3 | 2,239 B | partition exact *(control)* |
| `CONTEXT-FORMAT.md` | 3 | 3,658 B | partition exact *(control)* |

138 rows, and the nine per-file counts sum to 138 — so no unit is stranded
outside the partition proof. `defers=` resolution, target class, reachability and
chain termination are the build gate's, and `cargo build` is green.

**Per-kind retrieval spot-checks**, composing each mandate from the listing and
grepping it:

- the three prohibitions arrive **without any index**: `impl` carries
  `driving-no-session-summary` and `driving-ask-about-the-trade-off` and **not**
  `driving-no-pre-baked-grilling`; `requirements` carries all three.
- `grep -c '](#'` over every triggering unit: **0**. `grep -c '```mermaid'`: **0**.
- every retired id now exits non-zero as an unknown unit.

### Per-mandate byte delta — the evidence the design asks for

| kinds | before | after | delta |
|---|---|---|---|
| eighteen kinds | 49,857–50,816 B | 47,224–48,183 B | **−2,604 B (−5.2%)** |
| `requirements` | 52,686 B | 50,787 B | **−1,899 B (−3.6%)** |

Unit counts: 53 → 53 for eighteen kinds (two narrative units out, two
prohibitions in); 57 → 58 for `requirements`. The whole embed drops 143 → 138.
`requirements` saves less because it is the one kind that gains a third unit —
which is the classification working: the kind that needs the grilling prohibition
is the only kind paying for it. This is the largest deletion in the
classification, and −5.2% of every mandate forever is what the sweep bought.

### Deviation from `Done when`, stated rather than glossed

The leaf asked for verification from a **rebuilt, installed** binary. I rebuilt
and verified from `./target/debug/grove-llm` — a linked binary carrying the
embed, which is what makes *"served from the binary, not read from `content/`"*
true — but **did not install**. Same reason `addressing-k38` gave and it has not
weakened: `grove-llm` on `PATH` is `/opt/homebrew/bin/grove-llm`, the binary
driving this loop, and replacing it mid-session is an unrequested, hard-to-reverse
change to the user's machine that cannot change the embed (`cargo install` is a
release build plus a copy). The installed-binary check is discharged at node level
by `finish-cycle-k32`.

### Calls a reader could reasonably reverse

- **`skill-loop-diagram`'s removal is a content decision, not a mechanical one.**
  If the successor grove ever wants a rendered overview, its home is
  `docs/ARCHITECTURE.md` — a document with human readers — not the embed.
- **`driving.md` losing its H1** is the first file in the corpus to do so. The
  other eight still ship a document title inside their first unit. That
  inconsistency is now recorded in the root brief as a decision the successor
  makes deliberately, rather than as drift.
- **`driving-ask-about-the-trade-off` at `kinds=*`** widens a prohibition that
  read as grilling-specific. The argument is that escalation is universal; a
  reader who disagrees would scope it `requirements` and lose nothing structural.

### The in-session reviewer, unspent

Not used, and the reason is the predicate rather than thrift: an
`integrate-review-*` leaf may spend one **narrow** reviewer on an unexpected
doubt. The two surprises here — the flowchart-pinned tests and the navigation
test machinery — were both surfaced by `cargo test` and settled by reading the
prose the tests were standing in for, which is evidence rather than opinion.
