# composition-k7

## Goal

Add composition over `(units, kind)` to the `methodology` seam, and the
file-ordering directive it consumes, proven by test. Nothing consumes the
composer at the end of this slice — the driver still emits the launcher whole,
so what sessions receive is unchanged and the product keeps working.

## Context

The parser, the whole-embed gate and `grove-llm methodology` already exist. What
is missing is the third function the spec agrees on the same seam
(`docs/specs/mandate-delivered-methodology.md`, *Test seams*): a separate
composer seam was considered and rejected, because the composer's fixtures come
from the parser's output.

- Seam: `src/methodology.rs`, over `src/methodology/parse.rs` and
  `src/methodology/whole_embed.rs`. Both readers are shared with `build.rs`;
  keep it one implementation.
- **Composition order** is the framing unit's file first, then every triggering
  unit whose scope admits this kind, ordered by file position then by position
  within the file. Slices are joined by a blank line and nothing else. The
  framing unit needs no rule of its own — it leads because its file is ordered
  first.
- **The composer returns the composed slices only.** The driver's runtime facts
  (the selected handle, the stated VCS) are appended by the driver, not by this
  function; `mandate-delivery-k8` owns that.
- **The file-ordering directive** is an HTML-comment file directive — the same
  device and the same recogniser as a unit marker: an unindented whole line at
  neutral fence state. `content/` gains no metadata language for it. The order
  must be total, so every embedded markdown file carries one and a duplicate
  position is a build error.
- The directive sits **before the first unit marker**, which today is the
  `parse` fault *body text before the first marker*. Admitting it must not
  reopen that hole for prose.
- **Choose the position values with the rename in mind.** Position 1 goes to
  `content/prompts/continue.md`, so `mandate-delivery-k8` moves the file to
  `content/MANDATE.md` without moving its position and nothing churns.

## Done when

- The `methodology` seam exposes composition over `(units, kind)`, returning the
  selected units' source bytes verbatim — marker lines included — joined only by
  blank lines. No new seam, one new row in the architecture's module-seam table.
- Every embedded markdown file carries a file-ordering directive; the build
  fails on a duplicate position and on a file that carries none, named the way
  every other malformation is.
- Pinned by test through the seam:
  - **byte-exactness** — no paraphrase, truncation, re-wrapping or introduction
    (*Requirement: Slices are byte-exact*);
  - **scope selection** — `kinds=*` in all nineteen, `kinds=<one>` in that one
    and no other, `class=procedural` in none (*Requirement: Triggering units
    reach every kind they are scoped to*);
  - **golden per-kind snapshots**, for drift only;
  - **the per-kind size alarm** — each of the nineteen composed mandates at or
    under 64 KiB, counting the composer's returned bytes and not the driver's
    runtime facts;
  - **the directive's own shapes** — missing, duplicated, indented, and inside a
    balanced fence.
- `src/loop_driver.rs` is unchanged, `content/` prose is unchanged apart from
  the directive lines, and provisioning is untouched.
- `cargo test` is green.

## Notes

**Nothing consumes the composer yet, and that is the fault line.** The planning
leaf named it: composition can land and be proven by test before the driver
consumes it, so a slice that changes what sessions receive is separable from one
that only adds a function. Resist wiring the driver here.

`tests/methodology.rs::the_embedded_unit_set_is_pinned_complete` pins the full
unit-id set. The directive declares no unit, so that constant should **not**
move in this slice; it moves in both later ones.
