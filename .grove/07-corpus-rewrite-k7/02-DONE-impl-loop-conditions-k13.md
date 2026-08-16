# loop-conditions-k13

## Goal

Rewrite `content/SKILL.md`'s **27 universal triggering units** — the loop — into
condition lines plus deferred remainders, and move its 15 procedural units into
`content/references/`. This is the first child that actually shrinks anything, and
it is the largest single piece of the rewrite.

## What this covers

`SKILL.md`'s own units, after `per-kind-references-k12` has taken its two narrowed
ones out: the spine's seven constraints, the working tree, bare-`grove` dispatch,
the self-driving loop, one configuration, session name, starting a new grove, Pick,
Do not pick again, Bootstrap, Execute, ADRs and specs, Decompose, the bare-stem
rule, the chain-gap asymmetry, Retire, pruning, the node-close cascade, Commit,
Finish, Artifacts, the glossary, briefs-vs-glossary, and the `linkuistics`
prerequisite.

Each yields **one condition sentence** (staying `kinds=* class=triggering`, now
carrying `defers=`) and **one remainder** (`class=procedural`, in the reference
file the condition routes to). Several already carry `defers=` — those keep their
existing targets and gain the new remainder alongside.

## The loop narrative is written here

`SKILL.md` gets the loop stated as a narrative, and the spec makes constraint 7
literally checkable for the first time:

- the loop section is **at or under 100 lines**, measured heading-to-next-heading-of-the-same-level, blank lines included;
- the estimate it is an alarm on is ~80 lines.

100 is the alarm, not the budget — do not fit the prose to it. `skill-opening-k16`
owns the whole-file 500-line check and the routing table; this child owns the loop
section.

## Grouping the procedures

Bounded rather than enumerated. The **loop steps are the natural seam** — Bootstrap,
Execute, Decompose, Retire, Commit, Finish — and the whole universal-procedure set
across this child and its two siblings stays **under about eight files** beside the
ten per-kind ones. Any file over ~300 lines gets a table of contents. Fixing
filenames a session has not yet written would be over-specification, so choose them
here and let `driving-conditions-k15` fill in around them.

## Two rules the core sheds, which must land in this prose

The closed fact test in the spec pushes two rules out of `${prompt}` and into the
skill, and this is the child that owns both. Neither is a new rule; what is new is
that after the cutover the skill is their *only* home.

- The driver's **pick is authoritative and must not be re-walked** — already in
  `skill-pick` / `skill-do-not-pick-again`, so this is a survival check, not new
  writing.
- The driver's **stated version control is definitive and is not re-derived** from
  the working tree or from a harness banner. Today's `${prompt}` carries the
  *do not probe for it* clause; under the closed test it leaves the core, so
  `content/` must state it. **It does not state it today.** This is the closure's
  one real cost and this child is where it is paid.

## Done when

- `SKILL.md`'s 27 universal units are condition lines with deferred remainders,
  and its loop section is at or under 100 lines.
- The composed-mandates golden shows exactly the expected shrink — the diff is the
  claim, so read it rather than regenerating it blind.
- The build gate passes: every new remainder id is unique, reachable by `defers=`,
  and cycle-free.
- The two rules above are findable in `content/` prose.
- `cargo test` is green; the guidance suite's claims are re-homed or their loss is
  argued in the commit.

## Notes

Watch the guidance suite closely here — unlike `per-kind-references-k12`, this
child *does* change prose, and `tests/commit_guidance.rs`, `tests/retire_guidance.rs`
and `tests/composition_guidance.rs` assert on specific sentences. A claim that no
longer has a home in `SKILL.md` usually belongs in the reference file the condition
defers to; move the assertion, do not delete it, unless the claim itself is gone.

`grove-llm methodology <id>` is the fastest way to read a body while writing the
condition that defers to it. It is still live and dies in `mandate-machinery-k10`.

This child is a strong candidate for `leaf-decompose` if 27 units prove more than
one focused session holds — the seam would be the loop steps themselves. Do not
pre-empt that here; the bar is *fits this session*, not *I can finish it*.

## Running log — decisions this session made

### The seven universal reference files, and the seam they were cut on

The loop steps are the seam, as the brief predicted, with the frame either side
of them. Seven new files, all `class=procedural` throughout, joining the ten
per-kind ones in the same flat directory:

| file | what it carries |
|---|---|
| `references/grove.md` | the seven constraints argued, why the glossary holds, the Specs section, what the `linkuistics` plugin carries |
| `references/driver.md` | dispatch and migration, the stateless loop, the configuration, deriving the session name, what the scaffold creates, how the pick walks, why a second walk disagrees |
| `references/bootstrap.md` | what Bootstrap reads, in order |
| `references/execute.md` | what each kind produces, the record sets, review ownership |
| `references/decompose.md` | the two triggers and two verbs, the grow verbs, both shapes, the bare stem, which hop a gap costs, integration placement |
| `references/retire.md` | harvesting, pruning, the node close's four steps |
| `references/commit.md` | the boundary in git and jj, why the handle outlives the path |

**Seven, not eight, and the siblings should land in them rather than beside
them.** `task-format-conditions-k14`'s and `driving-conditions-k15`'s universal
remainders map onto these by topic — leaf naming, the kinds and the shapes go to
`execute.md` and `decompose.md`, the field-guide habits to `execute.md`,
`decompose.md` and `grove.md` — so the brief's *under about eight* is met by
reuse, not by restraint. A sibling that needs an eighth file should say why the
topic is not one of these seven.

`CONTEXT.md` gains **Loop-step reference file** beside **Kind reference file**:
two species in one directory, and the distinction that matters is *who selects* —
the driver picks the kind's file before the session exists, the session reaches
for a loop-step file when a condition it has just read sends it there.

### The loop section is 94 lines, and the measure needed a boundary to be honest

`## The loop` now spans **Pick → Finish** and nothing else. What used to sit
under that heading and does not now is the launch machinery — the working tree,
bare-`grove` dispatch, the self-driving loop, the configuration, the session
name, starting a new grove — which moved to `## What the driver settled before
your session`. That is not a way of getting under the alarm: those units describe
what happened *before* an agent existed, and the loop the session performs starts
at the mandate it was handed. Measured heading-to-next-heading-of-the-same-level,
blank lines and marker lines included, the loop is **94** and the file is **208**
(12.7 kB against 44 kB).

**The measure is now a test with a control** (`tests/methodology.rs`,
`the_loop_section_of_the_skill_fits_a_page`). `skill-opening-k16` still owns the
whole-file 500-line check; this one is added here because two sibling children
edit `content/` next and 94/100 is close enough that unguarded drift would be
silent. It establishes nothing semantic, and the doc comment says so.

`skill-adrs-and-specs` moved out of the loop and into `## Artifacts`, where the
ADR and spec rows of the artifacts table already are. It is explicitly a
*"whichever kind is running"* rule rather than a step, so the relocation is a
correction as well as six lines.

### Not every condition earned a remainder

The brief's shape — one condition, one remainder — is the common case, not a
quota. Six units ship whole because they already *were* conditions and splitting
them would have produced a pointer longer than the prose it withheld:
`skill-what-a-grove-is`, `skill-working-tree`,
`skill-retirement-touches-one-filename`, `skill-briefs-vs-glossary`,
`skill-artifacts` (a routing table, not a procedure) and `skill-finish`.

### The golden diff is exactly two claims, and it was checked rather than read

Regenerated and then verified programmatically against the recorded copy, per
kind: **no unit lost from any mandate**, one unit gained
(`skill-stated-vcs-is-definitive`), and the relative order of everything else
identical in all nineteen. The two visible hunks per kind are the addition and
`skill-adrs-and-specs` moving later within `SKILL.md`. Composed mandates fall
from ~48 kB to ~34–37 kB; the residue is `TASK-FORMAT.md` and `driving.md`, which
are the two siblings' work.

The seven new files take positions 20–26 and `content/SIGNAL.md` moves 20 → 27,
keeping the ending at the highest position — the invariant `per-kind-references-k12`
settled. Since every new file is wholly procedural, none of them appears in any
mandate, so the renumber is legible rather than load-bearing.

### The closure's one real cost is paid

`skill-stated-vcs-is-definitive` is new prose and the only new triggering unit:
*the driver's stated version control is definitive, do not re-derive it, and a
harness banner that disagrees does not win*. It ships whole with no remainder —
a rule the guaranteed core will stop carrying should not itself be behind a
pointer. The pick's other half needed nothing: `skill-pick` and
`skill-do-not-pick-again` already said it and still do.

### `${prompt}` is described in regime-neutral words

`skill-what-the-configuration-carries` says `${prompt}` "carries what grove has
to say to the session" rather than "your whole mandate — this methodology, sliced
for that kind". The old wording is true today and false after `guaranteed-core-k9`;
the new one is true under both, and `MANDATE.md`'s framing unit still tells
today's session exactly what it is holding. No cutover behaviour is pre-empted.

### The guidance suite: nine claims re-homed, none changed

Same reconciliation `per-kind-references-k12` performed, one level up: a claim
about a *procedure* is now proved against the file the condition routes to
(`composition_guidance.rs` ×9, `commit_guidance.rs` ×2, `retire_guidance.rs`,
`session_kind_guidance.rs`). **No claim was reworded and none was dropped.** Two
that read differently and are worth naming:

- `retire_guidance.rs` now asserts *pruning is HITL* on the page and *the verb is
  gated on human confirmation* in `references/retire.md` — one claim split across
  the seam it now straddles, which is stronger than moving it wholesale.
- `legacy_claim_sweep.rs` flagged `references/retire.md` as the deleted launcher
  prompt `retire.md`. The token ban is kept and given a **discriminator**
  (`LIVE_PATH_PREFIXES`): an occurrence directly preceded by `references/` is the
  live file. A stale claim about the deleted prompt cannot spell it that way, so
  the bare token stays banned everywhere — with a control asserting exactly that.

### No review leaf cut here, deliberately

The `no procedure in SKILL.md` limb is a review obligation the spec refuses to
let any budget test stand in for, and the node brief already assigns that review
to `skill-opening-k16`, after the whole corpus has moved. Cutting a
`review-impl` beside this leaf would put a reviewer in front of two-thirds of a
rewrite. The doubt is real and it is scheduled; it is not scheduled here.

### Left for the documentation reconciliation

`docs/ARCHITECTURE.md`'s "seven files ago" is reworded (it was already stale and
this child made it more so), but neither `ARCHITECTURE.md` nor `USAGE.md` yet
describes `content/references/` as the skill's second half. That belongs with the
cutover that makes those files the delivery path.
