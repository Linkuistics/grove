# release-record-k17

## Goal

Write this increment's `## Unreleased` entries in `CHANGELOG.md`. Five
substantive commits have landed with no release record at all, and the standing
`## Unreleased` heading is still empty.

## Context

The repo's own rule, stated at the top of `CHANGELOG.md`: *"A session logs its
change when it makes it, so the heading has to exist before the release that
closes it does"* — and the reason given there is exactly the situation this leaf
exists to repair, that an unlogged change can only be written up retroactively
"by whoever cuts the release and no longer has the context."

The previous grove followed that rule and also reconciled at finish, so the two
are complementary rather than alternatives: its `impl:` and `integrate:` commits
each touched `CHANGELOG.md` (`8fe82ee537f0`, `d17c3d752050`, `6258697dcf49`,
`43620bc1e77a`, `e3b5eb4a3cc5`, `a6645a50d17d`), and `finish-k40`
(`4fff40fb45bf`) then "reconcile[d] the release record" on top. This grove has
skipped the per-session half entirely, so leaving it to the finish cycle would
be the retroactive write-up the rule names.

The unlogged commits, oldest first:

- `1c81a50ac41f` `design: specialise the session-ending instruction by kind`
  (`specialised-ending-k2`) and `c4c19f3239ac` its integrate step — spec and ADR
  only; may need no entry of its own beyond what the impl commits carry.
- `54fddf564264` `impl: compose a kind's mandate from ordered content/ slices`
  (`composition-k7`) + `6df7ec5d0c61` (`composition-k11`) — the composition
  function on the `methodology` seam and the `<!-- file: order=n -->` directive.
- `46062d69d4dc` `impl: deliver the composed mandate and reduce the launcher to
  framing` (`mandate-delivery-k8`) + `191f8b617a33` (`mandate-delivery-k14`) —
  the driver composes `${prompt}` per kind; `content/prompts/continue.md` became
  `content/MANDATE.md` and `content/prompts/` is gone.
- `794219b6c61a` `impl: specialise the session ending per kind`
  (`session-ending-k9`) + `session-ending-k16` (this leaf's predecessor) — the
  ending split, the all-nineteen guard, and the reconciliation of the drift
  pin's claim with its actual boundary.

Two facts the entries must not overstate, both from the root brief's Notes:

- **The specialisation lands structurally in this release and behaviourally in
  the next.** Global skill provisioning is untouched and both delivery paths are
  live, so no session behaves differently yet.
- v18.2.0 already shipped `grove-llm methodology` as an *inspection tool* whose
  entry says "nothing consumes a unit yet". That is now false, and the entry
  belongs to a tagged release, so the correction goes in the new `## Unreleased`
  entries rather than by editing v18.2.0's.

## Done when

- `## Unreleased` carries this increment's changes, grouped in the file's own
  style, at the grain the file's existing entries use.
- The v18.2.0 "nothing consumes a unit yet" statement is superseded by an
  Unreleased entry rather than by an edit to the tagged section.
- The transient both-delivery-paths state is stated rather than implied, so a
  reader does not conclude provisioning is gone.
- `## Unreleased` remains exactly that string on a line of its own — the release
  cut anchors on it and aborts if it cannot find it.

## Notes

Sequenced ahead of `unit-scope-audit-k4` on the root brief's directive that any
further leaf for this increment inserts ahead of the audit rather than appending
after it. The audit is a separate increment and will log its own change when it
makes it.
