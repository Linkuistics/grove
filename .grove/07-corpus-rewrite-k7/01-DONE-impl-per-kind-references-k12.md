# per-kind-references-k12

## Goal

Establish the target file layout — `content/references/` with the ten per-kind
files — by **moving units, not rewriting them**, so the composed mandate comes out
byte-identical and the layout the later children write into already exists.

## Scope

**The ten per-kind files.** The corpus carries **eleven** distinct narrowed
marker scopes, not ten. The eleventh is `skill-signal`, narrowed to the eighteen
non-`finish` kinds (`content/SIGNAL.md:2`); it is not a family, it is the
session-ending text, and the too-late test moves it to `${prompt}`. Remove it from
the reckoning first and the remaining ten distinct scopes are exactly the per-kind
reference files:

| file | kinds it serves |
|---|---|
| `references/requirements.md` | `requirements` |
| `references/design.md` | `design` |
| `references/planning.md` | `planning` |
| `references/prototype.md` | `prototype` |
| `references/impl.md` | `impl` |
| `references/review.md` | the five `review-*` kinds |
| `references/integrate-review.md` | the five `integrate-review-*` kinds |
| `references/research.md` | `research-a`, `research-b` |
| `references/combine-research.md` | `combine-research` |
| `references/finish.md` | `finish` |

Ten files for nineteen kinds, because `content/` already treats each family as one
unit — the five `review-*` kinds share one marker today, as do the five
`integrate-review-*` and the two research producers. **Keep the thin `design`
file** rather than folding it: the driver selects a kind's file directly, so a
one-kind file costs a path and buys the mapping its exhaustiveness.

Each file takes that scope's narrowed `class=triggering` unit(s) **unchanged**,
plus the `class=procedural` bodies they `defers=`. Twenty narrowed triggering
units in total, sourced mostly from `TASK-FORMAT.md` (14), with the rest in
`driving.md` (3), `SKILL.md` (2) and `SIGNAL.md` (1).

**`SIGNAL.md`'s final resting path is settled here too.** `guaranteed-core-k9`
inlines that file's bytes into `${prompt}` by path, and
`tests/methodology.rs`-shaped checks will assert the composed ending *is* those
bytes — so the path must stop moving before the cutover reads it. Its unit keeps
composing last (`file: order=` highest) until the cutover retires it from the
mandate.

## The invariant that makes this cheap

`<!-- file: order=N -->` controls composition order **independently of file
layout**. So a pure relocation — new files, same units, `order=` values chosen to
preserve the existing sequence — leaves every kind's composed mandate byte-identical.

- `tests/goldens/composed-mandates.tsv` must come out **unchanged**. That is the
  whole verification, and it is a strong one: an unchanged golden proves no
  session's mandate moved by a byte while ~40 kB of prose changed files.
- The build gate must pass: `order=` uniqueness across the whole embed, id
  uniqueness, every `class=procedural` unit still reachable by `defers=`, no
  deferral cycles.

## Done when

- `content/references/` exists with the ten files above, each carrying its scope's
  units verbatim.
- The golden is byte-identical and `cargo test` is green.
- `content/SIGNAL.md` sits at its final path.
- The path-coupled suite is reconciled: `tests/composition_guidance.rs`,
  `tests/commit_guidance.rs`, `tests/retire_guidance.rs`,
  `tests/session_kind_guidance.rs` and `tests/reference_navigation.rs` all name
  `content/` files by path. Fixing a path is the whole of what they need here —
  **no claim in them changes**, because no prose changed. If a claim needs
  rewording, that is a signal the move was not pure.

## Notes

`provision` needs no change: extraction already handles nested directories — the
embedded corpus carries a `LICENSES/` subdirectory today.

Reference files stay **one level deep and flat**: `references/design.md`, not
`references/kinds/design.md`. That is the house progressive-disclosure rule and it
keeps the path the core names short enough to read at a glance.

*One file per kind label, by naming convention, needing no map* was considered and
rejected in the spec: it makes fifteen of nineteen files near-duplicates of four,
and duplicated prose is the drift risk one level down from the one being removed.
Reopen only if the families ever stop sharing discipline.

## Running log — decisions this session made

### The brief's invariant is false, and the verification is replaced

*"A pure relocation … leaves every kind's composed mandate byte-identical"* cannot
hold for **this** layout, and the reason is structural rather than a matter of
choosing `order=` values well. Composition sorts by `(file_order, offset)`, so a
file occupies **one** point in the global order, while a kind's narrowed units
occupy **several** — separated by universal units that must stay put:

- `requirements` reads `task-hitl-afk`(\*) → `task-producer-requirements` →
  nine universal units → `task-deliverable-requirements` →
  `task-deliverable-split-not-a-gate`(\*) → `task-bootstrap-leaf-is-requirements`.
- `design` reads `task-producer-design`, nine universal units, then
  `task-deliverable-design`.

One file cannot sit in three places, and splitting the universal remainder does
not help — the interleaving is per kind, and every kind's mandate carries the
same universal units in the same order. **Ten per-kind files and a byte-identical
golden are mutually exclusive.** No flag, no sequencing, and no `order=`
assignment recovers it.

**What replaces it is stronger than "the golden did not move", not weaker.** Three
claims, all mechanically checked this session:

1. **Every one of the 140 units is byte-identical** — checked by extracting each
   unit's source span from `@-` and from the working tree and comparing. Nothing
   was reworded, re-wrapped, or lost a trailing blank line. This is the claim the
   golden was a proxy for.
2. **Each kind's mandate carries the same *set* of units**, and the 51 universal
   units appear in the **same order** in all nineteen.
3. **The only change is relocation**: each kind's narrowed units are now one
   contiguous block at the tail of its mandate, immediately before `skill-signal`.

The golden is regenerated and its diff *is* exactly (3) — which is the property
the brief wanted from an unchanged golden, stated where it is true.

### Placement

`content/references/*.md` take `order=` 10–19 in the spec's table order, and
`content/SIGNAL.md` moves 10 → 20. So the universal corpus keeps orders 1–9
untouched, the kind block lands after it, and the ending still composes last.
Ordering *within* each reference file is the units' original `(file_order,
offset)`, so no two units changed places.

### Which procedural bodies moved

**Only those a per-kind scope owns exclusively** — computed by walking `defers=`
from every triggering unit and keeping the procedural units no universal unit and
no second scope reaches: `driving-dont-merge-questions` (requirements),
`driving-find-working-increments` (planning), `driving-the-combine-step`
(combine-research), and the four `skill-finish-*` bodies (finish).

`task-research-write-paths` is the one exception and it is **deliberate**: both
`task-research-pair` and `task-combine-research` defer to it, so it belongs to
two per-kind files and can live in neither. It stays in `TASK-FORMAT.md` for now.
**This is a seam a later child owns** — once `defers=` is deleted with the
machinery, `references/combine-research.md` read on its own loses the write paths
unless that prose is duplicated into both files or promoted to a shared
reference. Flagged, not solved.

### `SIGNAL.md` stays at `content/SIGNAL.md`

That is the settlement this leaf owed. It is **not** under `references/`, and the
reason is the too-late test rather than tidiness: a `references/` path says *this
is a procedure a session is pointed at*, and the session ending is the one piece
of methodology that must never be behind a pointer. Top-level, short, and stable
is what `guaranteed-core-k9` should inline by path.

### Two things deliberately not done

- **The reference files carry no title heading.** A body's first line after the
  file directive must be a unit marker, so a title would have to sit *inside* a
  unit — which would change that unit's bytes and break claim (1). Titling them
  belongs with the rewrite children.
- **`docs/ARCHITECTURE.md`'s "seven files ago"** is now off by the ten new files.
  Left for the documentation reconciliation the later children carry; it states no
  number this move invalidates.

### One pre-existing failure absorbed

`tests/legacy_claim_sweep.rs` was already red at `@-` (from
`wording-micro-test-k6`): a `REFUTATIONS` quotation for `CONTEXT.md` no longer
matched the reworked glossary, and `docs/research/wording-micro-test.md`'s naming
of `content/prompts/continue.md` was unclassified. Both are requoted here rather
than externalized, because this leaf's `Done when` says *`cargo test` is green*
and a suite that stays red makes every later child unable to tell its own
breakage from an inherited one. No claim in the sweep changed — only the
quotations that locate them.

### Path reconciliation in the suite

Two surfaces needed a path fix and **no claim moved**, exactly as the brief
predicted: `tests/composition_guidance.rs` (the working-increments claim now sits
in `content/references/planning.md`, which is one surface carrying what
`driving.md` and `TASK-FORMAT.md` carried between them) and
`tests/retire_guidance.rs` (the finish cycle's confirmation sentence now sits in
`content/references/finish.md`).
