# loaded-path-budgets-k29

**Reviews:** `loaded-path-budgets-k10`

## Goal

Adversarially read `tests/loaded_path_budgets.rs` and the prose it settled — the
budget model, the four load-column assertions, and the recorded acceptance
comparison — as the corpus's standing shape measure. Every future edit to
`content/` is gated by it, so a fail-open assertion or a mis-set band licenses
growth silently and for good.

## Context

`loaded-path-budgets-k10` replaced the 500-line ceiling with per-kind loaded-path
word budgets and spent its one in-session reviewer on the result. That pass
returned **eleven findings**, all confirmed and all integrated in the same
commit. That is the evidence this leaf exists on: the artifact rewards adversarial
reading, and this grove's pattern is that each structural repair carries new
defects — `k12`→`k13`→`k14` found four fresh P1s after the first repair, and
`k16` five after the second.

**Two of the integrations were structural**, so what stands is a design no
reviewer has seen:

- The headroom became a **band** rather than a number. Budgets are set at
  measurement + 10% and asserted to stay within measurement + 25%. The single
  number was self-defeating: every budget sat exactly on its own limit, a
  fifteen-word deletion turned fourteen rows red, and the over-budget failure's
  own advice ("move a procedure out") shrank the measurement and tripped the
  other check. The band's width is a judgement — ~12% shrink tolerance, ~10%
  growth — made from two mutation runs, not from a principle.
- The recorded **before** figures changed. The workstream's brief carried
  "roughly 3,200–3,700 words"; recomputed at `b6ecdbd0` it is **3,108–3,944**,
  and `docs/ARCHITECTURE.md` now states a per-kind table with ratios rather than
  a range. The ratios are the grove's acceptance claim.

The in-session pass also confirmed, by independent recomputation, all nineteen
static and nineteen reachable measurements, `content/` = 14,741, `SKILL.md`'s
body = 796, and the core at 314/353. Those are not worth re-deriving; what is
worth attacking is everything downstream of them.

## Done when

Each of these is either confirmed sound or reported as a finding with the
concrete mutation that exposes it.

- **The band.** Is +10% / +25% defensible, or is it two numbers picked to make
  the observed mutations pass? What legitimate edit is still painful? Note the
  two checks pull in opposite directions by construction — is a two-sided budget
  the right instrument at all, or does the shrink side buy less than it costs?
- **Fail-open sweep, per assertion.** For each of the sixteen tests, construct a
  mutation that *should* fail it and check that it does. The in-session pass found
  three that did not (a `trigger` row citing nothing, a fail-open frontmatter
  stripper, an edge source owning no rows) — assume more remain.
- **The reader.** `inventory`, `heading_paths`, `table_cells`, `read_row`,
  `read_load`, `read_kind_set`, `cited_sentences`. A row lost or mis-attributed
  makes every assertion over it vacuous, and the count control is a floor rather
  than an identity.
- **The `static(...)` check against the runtime**, including the third static
  file — the signal file, recovered by byte-matching `prompt::ending_of` against
  the embed. Does that derivation hold if a signal file is ever edited to equal
  another embedded file?
- **The prose.** `docs/ARCHITECTURE.md`'s *The corpus's shape, and what is
  measured over it*, and the spec's *Load predicate notation* and *Test seams*.
  The in-session pass killed one false argument there ("strictly dominated") and
  one wrong number range; check the rest, including the six-spelling table now
  stated in the spec.
- **What the deletions lost.** Nothing bounds `SKILL.md`'s line count or measures
  the loop section now. Is the stated ground — a line is not a unit anyone reads,
  a section is not a unit anyone loads — sound, or is there a failure the deleted
  pair caught that nothing catches?

## Notes

- Inspection only: read the committed diff, the source and the recorded evidence;
  run no build, test, lint or format command, and edit nothing. Findings are the
  deliverable, and the paired integration owns every fix.
- The eleven integrated findings are in the commit message and in this file's
  *Context*. Re-finding one of them is not a finding; finding one the integration
  got *wrong* is.
- `src/prompt.rs` is out of scope for the whole workstream and is unchanged. The
  budget reads it and must not have widened it — check that it did not.
