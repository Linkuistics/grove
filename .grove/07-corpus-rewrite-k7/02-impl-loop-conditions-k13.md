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
