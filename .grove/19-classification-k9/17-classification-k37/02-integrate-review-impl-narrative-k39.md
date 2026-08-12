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
