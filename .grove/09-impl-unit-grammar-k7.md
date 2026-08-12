# unit-grammar-k7

## Goal

Land the mechanism that makes `content/` addressable — the `methodology` module
seam, the marker grammar, the parser, and the build gate — **green against a
trivially correct marking**, so no classification judgement is spent while the
mechanism is still moving.

Design: `docs/specs/mandate-delivered-methodology.md` (*Units partition a file*,
*The marker grammar*, *A unit names the procedure it defers to*, *Fence state*,
*A malformed embed fails the build*, *Test seams*). Do not re-decide it; the one
question it left open is settled by `ordering-key-placement-k6`, which runs
first.

## Context

### What lands

- The `methodology` module seam: the parse function over `(path, text)` and the
  embed's unit set. **Not** the composition function — composition belongs to the
  successor grove, and so does everything that measures a composed mandate.
- The marker grammar, exactly as specified: fixed attribute order `id`, `kinds`,
  `class`, `defers`; `kinds` required on triggering and forbidden on procedural;
  `defers` optional on either; ids kebab-case and unique across the whole embed.
- Total partition: every body byte in exactly one unit, marker line included in
  the unit's source, no nesting, no gaps, no close markers.
- Fence state tracked across the whole file body; a marker recognised only as an
  unindented whole line at neutral state; **unbalanced fence state at end of file
  is a build error**.
- All three malformation classes failing `cargo build` — syntax, semantics, and
  reference (including *every procedural unit is reachable by following `defers=`
  from a triggering unit*).
- **One parser implementation shared with the crate**, not a second traversal in
  `build.rs`. `build.rs` today is standalone and links nothing; the equality test
  between its hash traversal and `provision::content_hash` exists because that
  duplication was accepted once already. Do not accept it twice.
- `content/` marked throughout with a **trivially correct** marking, and whatever
  file-level ordering carrier `ordering-key-placement-k6` settles on.

### The trivial marking is `class=triggering kinds=*`, not `class=procedural`

`increments-k4` corrects the design leaf here. The plan written before the review
was integrated said *one unit per file, `class=procedural`* — a legal total
partition. It is no longer legal: `mandate-delivery-integrate-k5` added `defers=`
and the reachability rule, and a corpus of nothing but procedural units satisfies
no reachability check, so an all-procedural marking **fails the build** by the
very rule this increment is landing.

One unit per file at `class=triggering kinds=*` is the marking that passes: total
partition holds, there are no procedural units so reachability is vacuous, and no
`defers=` target can dangle. It is also *only* legal while composition does not
exist — the per-kind 64 KiB size alarm would reject an all-triggering 139 kB
corpus outright — which is a second reason composition and its alarm stay in the
successor grove rather than arriving early.

### What is deliberately not here

- The composer, `content/MANDATE.md`, driver wiring, golden per-kind snapshots,
  the completeness invariant's *mandate* claims, and the size alarm. Successor
  grove.
- `content/prompts/continue.md` **stays and stays true**. Provisioning is live for
  the whole of this grove, so the launcher's "use the grove skill" is still the
  fact. It is an embedded markdown file like any other, so it carries a marker and
  the ordering carrier — it is not renamed and its text does not change.
- Any real classification judgement. That is `classification-k9`.

## Done when

- `cargo build` fails, by name and with the file and offset, on every malformation
  the spec enumerates: unparseable marker, unknown attribute, attributes out of
  order, missing `class`, `kinds` on a procedural unit, unbalanced fence at end of
  file, duplicate id anywhere in the embed, body text before the first marker,
  duplicate ordering key, `kinds=` member outside the closed nineteen, `defers=`
  naming no declared unit, `defers=` naming a non-procedural unit, and an
  unreachable procedural unit.
- Parse shapes are pinned on the forms that decide the reading rule — accepted
  **and** ignored alike, including a balanced fenced example marker and an
  indented marker-shaped line. The repository's instructed-verb scanner is the
  precedent, and its two live holes are why.
- A **positive control pinned complete**: the full set of unit ids is a test
  constant, so losing a unit fails and gaining one fails until someone names it.
  Under the trivial marking that set is one id per embedded markdown file.
- The classifier is shown able to fail, on a synthetic malformed marker and a
  synthetic well-formed one.
- `cargo test` and `cargo build` are green with `content/` trivially marked, and
  the running loop is unaffected — no session-visible behaviour changes.

## Notes

- The whole increment is inert by construction: nothing reads a unit yet. That is
  the point of the split, and it is what makes the next increment's judgement
  cheap to review in isolation.
- Two content files are **vendored** — `content/grilling.md` and
  `content/CONTEXT-FORMAT.md`, bundled from `mattpocock/skills` under the notices
  in their leading HTML comments, and `content/CONTEXT-FORMAT.md` is deliberately
  not re-synced upstream. Marking them is required (they are embedded markdown),
  and marker placement must not disturb the provenance comments or invite a
  re-sync that would drop grove's divergence.
- `build.rs` keeps its `rerun-if-changed` emission and, for now, `GROVE_CONTENT_HASH`
  and `content_hash`. The hash retires in the successor grove, with the
  compile-time methodology identity — not here.
- Sharing the parser with `build.rs` decides a small layout question (a `#[path]`
  include, a separate crate, or a workspace member). Whichever way it goes, the
  reason is one implementation, and the header comment should say so where the
  next reader will look.
- If the classification review earmarked by the design has to start early because
  the trivial marking turns out not to be reviewable in isolation, say so and cut
  the leaf — do not absorb it.
