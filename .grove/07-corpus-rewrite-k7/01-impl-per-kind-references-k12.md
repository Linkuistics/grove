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
