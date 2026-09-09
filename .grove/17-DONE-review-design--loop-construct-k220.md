# loop-construct-k220

**Reviews:** loop-construct-k7

## Goal

An adversarial read of the pass-series design: `docs/specs/pass-series.md`,
`docs/adr/iteration-is-a-node-of-nodes.md`, and the **Pass series / pass** entry
added to `CONTEXT.md`. Findings, not fixes.

## Context

The producer designed **against a human's proposed shape and departed from it**,
which is the first thing to contest. The human proposed *a directory carrying a
marker token*; the design keeps the directory, drops the marker, and argues the
mark has no reader. If that argument is wrong the whole record is wrong, so read
the six objections in the ADR's trade-off section one at a time and ask which
would survive a reader arriving — and whether the design should have been written
to accommodate one rather than to reject it.

Four places this design is most likely to be wrong, named because the producer
could not check them from inside:

- **The pass/correction-run distinction is the load-bearing seam and it is
  new.** *A pass is a repetition the series declared in advance; a correction run
  is one the finding session decided.* Everything else rests on it — which
  repetitions get a directory, which reuse
  `a-feedback-edge-is-forward-tree-growth`. Is it decidable by a session at the
  moment it must decide, or does it require knowing why an earlier session acted?
- **A pass node's brief has two writers with different jobs.** `leaf-decompose`
  moves the decomposed leaf's body in as `BRIEF.md`, so the pass's charter is
  written by the previous pass's last step. But `references/editorial.md` also
  puts a running `## Handed forward` list in a document node's brief. Under a
  series those are the same file at different levels. Check the design does not
  quietly require one brief to be both.
- **The one-step pass has no directory, which makes the shape non-uniform.** A
  series of one-step passes is a run of sibling leaves; a series of two-step
  passes is a run of directories. A reader of `find .grove` sees two different
  pictures for one construct. Is the exemption right, and does anything in the
  spec break at the boundary?
- **The declarations are unenforced and the spec says so.** Test whether *says
  so* is enough: enumerate what a session could get wrong (a cap chosen after
  pass 1, a sequence never written, an escalation skipped) and ask whether each
  is detectable by any reader at all, including a human with `find`.

**Three claims are measurements and go stale.** The producer reported them as a
frozen reading, deliberately: zero repeated `(parent, kind, slug)` triples in the
tree; one `copy-edit`/`art`/`proof` per book across four books; 36 of 150 leaves
under `crate-books-k14` allocated after `proof--grove-loop-k182`. **Re-derive
them rather than reading them** — the producer's own leaf added entries after
taking them, and the extractor is a shell pipeline whose controls are described
in `loop-construct-k7`'s running log rather than committed as a script.

**One claim about another repository is unverified.** The spec's Problem section
rests on this grove's tree only. The producer looked at `Writegood`'s tree and
found no iterative shape there either, but did not survey `grove.gh-issue-12` or
`TheGreatExplainer`, which the root brief names as live in-house prior art. A
counter-example in either — a loop somebody already ran in a grove — is the
strongest single finding available here.

## Done when

Every finding is written down with the evidence for it, and the read has reached
a verdict on the marker-token rejection specifically: sound, or unsound and why.
Cut an `integrate-review-design` leaf only if there are findings worth acting on.

## Findings

### F1 — high — the marker-token rejection is unsound on its load-bearing premise

The ADR calls `find .grove` “the whole reader” and says pass identity and count
are visible **from names** (`docs/adr/iteration-is-a-node-of-nodes.md:16-23`),
then rejects the only name-level discriminator because “nothing would read it”
(`:55-59`). The human reading `find` is already that reader. Replacing the mark
with prose in `BRIEF.md` (`:61-64`) changes the interaction the requirement
asked to preserve: an ordinary node, a series with one pass, and an unmarked
series whose children have not repeated yet have identical names. The reader
must open and interpret a file to distinguish them. The spec repeats the
contradiction when it says nothing marks a series and then says the example is
legible without a task file (`docs/specs/pass-series.md:49-82`).

Four of the six objections do not establish that every marker shape has the
claimed cost. The slot after a position is an outcome slot only for leaves;
nodes have no outcome (`crates/grove-loop/src/task_name.rs:105-109`). The shown
uppercase `LOOP` syntax cannot collide with an existing slug, because slugs are
lowercase-only (`task_name.rs:190-215`), so it does not imply that every slug
spelling the marker is refused. A marker before the slug can preserve the
terminal `<slug>-k<key>` substring, contrary to “cannot go anywhere else.” And
the type need not be `Option<Marker>`: the real cost is a deliberate second node
variant/species, which is the proposal the design needed to weigh rather than
turn into an invalid optional-field argument (`task_name.rs:530-589`). The slug
suffix objection does survive, but it defeats only that placement.

The ADR's own reversal argument points the other way. It says adding a reader
later would silently under-report every series created unmarked and require a
cross-tree cutover (`iteration-is-a-node-of-nodes.md:101-113`). No pass series
exists yet—the re-derived current-tree reading below remains clean—so that is
evidence for settling reader-facing identity before the first one, not for
creating the legacy population now. Marker verdict: **unsound**.

### F2 — high — the ADR and spec describe different pass species

The ADR's decision is categorical: every pass **is** a node because it needs
more than one session (`iteration-is-a-node-of-nodes.md:3-14`). Its explicit
reopen condition is a pass small enough that a pass node is heavier than the
pass, because then the central claim is false (`:115-122`). The spec already
admits exactly that case: “A one-step pass is a leaf and gets no directory”
(`docs/specs/pass-series.md:95-98`). `CONTEXT.md:658-668` places both statements
in one glossary entry.

This is not merely non-uniform presentation. For a one-step series, the ADR
title “node of nodes,” its explanation of pass allocation, and its argument that
the existing one-node-species rule covers the construct are false. It also
demonstrates the failure mode of restating an ADR inside a spec that
`SPEC-FORMAT.md:33-36` warns about: the minimum coherent design set currently
has two answers to whether a pass is a node.

### F3 — high — cap compliance and successful exit have the same filesystem state

The four facts that make a node a pass series are freeform, mutable prose in its
brief (`docs/specs/pass-series.md:55-65`), but the design claims they were fixed
before pass 1. A present-tree reader cannot distinguish a cap written before
pass 1 from one added or changed after it. A missing sequence is visible only
after opening the brief; a sequence deviation is visible only by manually
comparing the brief with every pass.

The more serious collision is at termination. A satisfied exit condition cuts
nothing (`pass-series.md:124-129`); reaching the cap also cuts nothing and “stop
and say so” (`:144-150`). If the last step silently treats the cap as a normal
exit and skips escalation, the tree is byte-for-byte the same as a correctly
exited series. The spec acknowledges that nothing detects skipped escalation
(`:175-193`) but still calls the declarations the filesystem representation.
“Says so” is insufficient for the stated reader-facing contract where two
different outcomes deliberately collapse to one observable tree and the only
distinction is ephemeral session output.

### F4 — high — the design leaves its load-bearing methodology seam to implementation

The spec says all four declarations are discipline “carried by a skill” while
claiming there is no seam (`docs/specs/pass-series.md:175-182`). No session reads
the spec or ADR. The follow-on implementation leaf says so explicitly and then
owns the unresolved architectural choice among a new kind, a family reference
file, or a condition in the spine (`pass-series-discipline-k221:5-8,23-46`).

That is the interface through which a session recognizes a series, distinguishes
a pass from a correction run, knows which declarations bind it, and performs
the cap escalation. Leaving its placement until `impl` means the agreement-point
artifact has not settled the module or seam the implementation depends on—the
work `SPEC-FORMAT.md:5-13,52-57,107-116` assigns to design. The conceptual test
is decidable by the session that acts: a predeclared failed exit creates a pass;
a defect the current session finds creates a correction run. But that answer is
not delivered to any loaded path until the implementation leaf makes the design
decision the spec omitted.

### F5 — medium — the pass/correction binary omits resumed live work seen in prior art

The design defines work done again as either a predeclared pass or a correction
run decided by the finding session (`docs/specs/pass-series.md:117-122`) and says
a series repeats kinds, never a leaf (`:161-168`). Two in-house groves exhibit a
third form: the loop stops with a leaf live and a later launch picks the same
handle again. Writegood committed six successive `machine-supply-k22` readings
(`713fe3c855d6` through `c8f584abfba7`); that task's own notes call them the
first through sixth sessions and explain that the leaf stayed live behind a
human gate. `grove.gh-issue-12` likewise has repeated `control-recount-k14`
changes (`ad4d6809d4ab`, `58ae3ccc5165`, `ad06216b918f`, `5efa3f33cf8d`).

These are not pass-series counterexamples: neither repetition was declared as
a pass sequence, and no new correction-run leaves represented the retries. They
are nevertheless real Grove repetition, and they make the binary taxonomy and
the unqualified “a pass's step is one leaf and one session” claim incomplete.
The design needs to bound its subject to completed work being deliberately run
again, or account for resumed live leaves when defending the reopened
one-task/one-session rule.

## Evidence re-derived for this review

- Current grove: 222 keyed entries, all 222 parsed; 222 unique keys, maximum
  key 222, no gaps, no repeated `(parent, kind, slug)` triple. The same parser's
  in-memory dirty control reported one duplicate triple and one malformed keyed
  name.
- `crate-books-k14`: four book nodes each contain exactly one `copy-edit`, one
  `art`, and one `proof`; an injected second `proof` produced one violation.
  The node contains 150 leaves: 36 have keys after k182, 114 do not, and 29 of
  the 36 are flat siblings at the node level. Threshold controls returned all
  150 after k0 and zero after the current global maximum k222.
- Prior art: Writegood's live tree has 67 parsed keyed entries and
  `grove.gh-issue-12` has 22; neither has a repeated `(parent, kind, slug)`
  triple. TheGreatExplainer has no current or historical Grove tree; its
  `docs/requirements.md` §1.6 specifies feedback to earlier pipeline stages but
  is requirements prior art, not a loop already run. No predeclared pass-series
  counterexample was found in those repositories; the resumed-leaf evidence is
  the narrower finding F5.

## Checks that held

- The two-writer `BRIEF.md` concern is not independently defective. A pass's
  predecessor writes the initial charter; stages then maintain and clear
  `## Handed forward` as current-state context
  (`plugins/grove/skills/grove/references/editorial.md:39-45`). That is consistent
  with `BRIEF-FORMAT.md`'s mutable-context role. The pass node is the editorial
  document node for that pass, so the two jobs compose in one file rather than
  compete for authority.
- The three frozen measurements in the producer's log remain accurate after
  re-derivation. Their counts are not findings; the design conclusions drawn
  from them are contested above.

## Evidence limitation

`codebase-memory-mcp` could not start: three attempts reported an active
pre-coordination or unverified generation, and no deferred MCP graph surface was
available in this harness. The grammar claims above therefore use exact source
search and line reads in `crates/grove-loop/src/task_name.rs`; no negative or
exhaustive code-graph claim relies on the unavailable index.
