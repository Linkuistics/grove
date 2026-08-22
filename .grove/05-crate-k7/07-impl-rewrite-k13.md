# rewrite-k13

## Goal

`rewrite`: replace an entry's parts, keeping its ordinal, key and species. This
is how an attribute changes, and it is the general form of every "mark this
entry" operation a consumer might want.

A small leaf. That is fine — the bar is that it fits one session, not that it
fills one.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — the `rewrite` row, the `rewrite` refusals, and the
  occupancy paragraph's carve-out: occupancy **excludes** the object being moved,
  or a rewrite whose new parts equal the old — a rename onto itself — would
  refuse its own no-op.
- `docs/adr/entries-are-never-removed.md`, which is why this operation matters
  more than its size suggests: a domain retires an entry by rewriting an
  attribute, because there is no removal.
- `06-impl-promote-k12.md`'s species check, which is this one with the opposite
  verdict. If the two are not the same code by the time this leaf is done, say
  why in the findings entry.

## Done when

- `rewrite` works: ordinal, key and species survive; only the opaque remainder
  of the name moves.
- New parts implying a different species are refused — a file cannot be renamed
  into a directory.
- The self-rename no-op is not refused by the occupancy check.
- Each test names the model claim it discharges, or says it has none.
- An entry in `docs/formalism-findings.md`. If this leaf found nothing, the
  entry says so and records the cost — an uneventful episode is H2 evidence, and
  a log that records only disagreements is a survivorship sample.

## Notes

**The library neither knows nor cares what changed.** Attributes are opaque; the
operation verifies that the ordinal, key and species survived, and renames. Any
temptation to inspect parts beyond their species is the seam leaking, and the
models are written on the premise that it cannot — their state contains no
strings at all.

**This is the last mutation.** After it, every operation in `ARCHITECTURE.md`'s
tables exists, which is the precondition `08-impl-h3-probe-k14.md` is waiting on.
