# task-format-conditions-k14

## Goal

Rewrite `content/TASK-FORMAT.md`'s **14 universal triggering units** into
condition lines plus deferred remainders, and settle where the task-file grammar
lives in `content/references/`.

## What this covers

The universal half of `TASK-FORMAT.md`, after `per-kind-references-k12` has taken
its **14 narrowed** units into the per-kind files — which is exactly half of that
file's 28 triggering units, so this child works on a materially smaller file than
the one on disk today.

What remains universal: the leaf filename grammar, the kind-in-the-filename rule,
the nineteen kinds table, the HITL/AFK mark, the in-session doubt budget, the
too-big-is-planning rule, the two composing shapes, declaring the relationship in
the body, what the shapes are not, no node for a shape, the five-field grammar,
nothing-in-a-body-is-metadata, the three design kinds, and the
deliverable-split-is-not-a-gate rule.

Its 10 procedural units move to `content/references/` alongside.

## Two things not to lose in the compression

Both are units whose *point* is a distinction that dies if the condition line is
written carelessly:

- **`task-grammar-is-five-fields`** turns on the difference between what is
  *parsed* (position, outcome infix, kind, slug, key) and what is *convention*
  (the shared stem, relative ordering, the two declaration lines). A condition line
  that says "a leaf name is five fields" without the convention half loses the
  argument that decided the bare-stem rule.
- **`task-in-session-doubt-budget`** is a table keyed by the picked session's kind.
  A table is not a condition sentence, and squeezing it into one would misstate it.
  The condition is *the session has run Bootstrap and adopted the mandate, and
  doubt has arisen*; the table is the procedure and belongs in the reference file.

## Done when

- `TASK-FORMAT.md`'s 14 universal units are condition lines in `SKILL.md` with
  deferred remainders in `content/references/`.
- The composed-mandates golden shows exactly the expected shrink.
- The build gate passes.
- `cargo test` is green.

## Notes

`tests/session_kind_guidance.rs` (84 kB) is the suite file most exposed to this
child. It generates its claims from `Kind::ALL` and the real `grove-llm` command
model rather than enumerating spellings — which is why it is worth reconciling
carefully rather than trimming: it is the check that fails the day a *twentieth*
kind ships undocumented, and that property must survive the rewrite. Where it
asserts a kind's guidance exists, the guidance now lives in that kind's reference
file; point it there rather than weakening the claim.

The nineteen-kind table and the two-shapes section are the material
`skill-opening-k16`'s routing table draws on. Leave them in a shape that child can
route to without restating.

## Running log — decisions this session made

### `TASK-FORMAT.md` stays where it is, as the fifth format document

The goal line asked where the task-file grammar should live *in
`content/references/`*, and the answer is that it does not go there. Four format
documents already sit at `content/` root, wholly procedural, cited by bare name
from the skill page — `ADR-FORMAT.md`, `BRIEF-FORMAT.md`, `CONTEXT-FORMAT.md`,
`SPEC-FORMAT.md`. Once its fourteen conditions leave, `TASK-FORMAT.md` is exactly
that species and nothing else, so moving it would have split one species across
two directories to satisfy a phrasing. It keeps `order=3`, keeps its name — which
`src/`, three specs, `docs/ARCHITECTURE.md` and four `content/` files cite — and
`loop-conditions-k13`'s *"a sibling that needs an eighth file should say why"* is
answered by not needing one.

### Fourteen units became thirteen conditions

`task-what-shapes-are-not` is merged away rather than rewritten. Its body was a
**section framing** — *"Three things that shape looks like it could be and is
not:"* — over three sub-items that are separate units. Once those sub-items are
conditions in their own right, the framing has no condition of its own left, and
a line in `SKILL.md` whose whole content is *a list follows* is precisely the
procedure-shaped prose the page is being emptied of. Its one real claim (grove
reconstructs no relationship from a name, a position or a body) was **already
stated verbatim** by `task-grammar-is-five-fields`, which keeps it; its prose
survives as procedure at the head of `task-bare-stem-reasoning`, where the two
remaining sub-items still sit. Both its `defers=` targets stay reachable —
`skill-why-the-stem-is-bare` and `skill-which-hop-a-gap-costs` in
`references/decompose.md` already defer to them.

### The two units the brief said not to lose, and how each was kept

- **`task-grammar-is-five-fields`** keeps *both* halves in the condition itself.
  The parsed half is named as five fields; the convention half is spelled out as
  the three things a name may imply about *another* leaf — shared stem, relative
  ordering, the two declaration lines — with the three refusals grove does not
  make. What moved to the remainder is only *what each field does*, which is a
  table, and the *adds-versus-duplicates* test that decided the bare stem.
- **`task-in-session-doubt-budget`** states the predicate and the allowance as a
  sentence — Bootstrap-and-mandate, one reviewer for a plain producer, one narrow
  reviewer for an integration, none for everything else, leaf-wide not
  per-decision — and hands the by-kind table to `TASK-FORMAT.md`. The table stayed
  in that file rather than moving to `references/execute.md`: `execute.md`'s
  `skill-review-ownership` already states the same rule in prose, so a second copy
  there would be two statements one paragraph apart, and the composition suite
  pins the integration row on `TASK-FORMAT.md` verbatim.

### Duplication across `content/` was left alone, deliberately

`references/decompose.md` restates the chain, the pair, the bare stem and the
gap asymmetry that `TASK-FORMAT.md` also carries, and the tempting cleanup was to
collapse them. The guidance suite says not to: it pins the integration-placement
rule on seven surfaces at once and says why —
*"so a reader reconciling the glossary or the architecture against the
methodology finds one rule rather than two"*. Multi-surface statement is this
corpus's design, not its drift, so `task-review-chain-mechanics`,
`task-vendor-pair-mechanics`, `task-bare-stem-reasoning` and
`task-chain-contiguity` are unchanged and no assertion moved off them.

### The golden diff was checked, not read

Regenerated, then verified programmatically against the recorded copy across all
nineteen kinds: **one id lost** (`task-what-shapes-are-not`), **none gained**, the
thirteen surviving conditions **contiguous** and immediately before
`skill-artifacts` in every mandate, and the order of every other unit **identical**
once those thirteen are removed from both sides. That is the whole claim: a
merge, and a relocation from file position 3 to inside file position 2.

The ids did **not** change with the files. `methodology.rs`'s doc comment claimed
ids are *file-scoped by prefix*, which `per-kind-references-k12` and
`loop-conditions-k13` had already outgrown (`skill-` ids live under
`references/`); it now says the prefix records where a unit was first carved and
is not a claim about where it lives, which is what keeps this diff a relocation
rather than thirteen renames.

### The shrink

The fourteen universal triggering units were **10,621 bytes** in
`TASK-FORMAT.md`; the thirteen conditions are **6,151** in `SKILL.md` — 42% off
this child's slice. Every kind's composed mandate falls by 4,470 bytes
(45,676 → 41,206 across the whole corpus). `SKILL.md` is 304 lines, its loop
section still 94 against the alarm of 100, and the residue is `driving.md`'s
6,889 bytes — `driving-conditions-k15`'s work.

### One claim re-homed, none changed

`canonical_guidance_drops_receipt_and_diversity_era_review_routing` proved
*"never a note the producer left behind about how its own session ran"* against
`TASK-FORMAT.md`. It is a condition, so it now states itself on the loop page and
the assertion reads `content/SKILL.md`. No wording changed and nothing was
dropped; every other guidance assertion held unmoved.

### No review leaf cut here

Same reasoning `loop-conditions-k13` recorded: the *no procedure in `SKILL.md`*
limb is a review obligation the spec refuses to let a budget test stand in for,
and the node brief assigns it to `skill-opening-k16`, after the whole corpus has
moved. This child spent no in-session reviewer either — the golden check above is
mechanical verification, not doubt.
