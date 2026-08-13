# specialised-ending-k2

## Goal

Amend `docs/specs/mandate-delivered-methodology.md` with the four decisions the
`requirements` grilling took, and agree the test seams for them. The spec already
designs the composer; this leaf does **not** re-derive that. What is new is
what the composer's first consumer needs: the launcher's reduction, where the
ending specialisation lands, how the complement scope is spelled, and the
reopened-`finish` ending.

## Context

Read the root `BRIEF.md` first — D1–D4 are stated there in full, with the code
citations that back them. This leaf's job is to turn them into spec text under
the spec's existing `## Decisions` / `## Requirements` / `## Test seams`
structure, not to re-open them.

Two of the four are pure applications of decisions the spec and its ADR already
record, and should read as such rather than as new argument:

- **D1** applies the spec's own `content/prompts/continue.md` becomes
  `content/MANDATE.md` section. That section already says the file's surviving
  job is *framing*; what it does not yet say is that every instruction the
  current launcher carries is a duplicate of a `kinds=*` unit composition
  delivers, so framing-only is a subtraction with nothing to replace.
- **D3** applies the ADR's rejection of family shorthands and negation. The spec
  section `kinds=` admits `*` and explicit lists, and nothing else argues from
  a kind added later being silently *absorbed*; the eighteen-member list poses
  the mirror hazard — silently *omitted* — and that argument is not yet written
  down anywhere.

Two are genuinely new:

- **D2** decides that the ending specialisation lands in `skill-signal` and
  `skill-finish`, not only in the launcher, because `skill-signal` states both
  endings at `kinds=*`. Note the one sentence that stays at `kinds=*`: a session
  never discovers a grove is finished; the driver tells it by launching a
  `finish` session. That is a negative trigger against a destructive action, and
  withholding it is the unasked-question failure the ADR is built around.
- **D4** documents an ending the code already supports and no prose names: a
  `finish` session that externalised work signals a plain relaunch. Verified
  during grilling at `src/tree_read.rs:90` (pick prefers any live non-`finish`
  leaf regardless of position) and `src/complete.rs` (the verb is not gated by
  kind).

## Done when

- The spec's `## Decisions` carries D1–D4, each argued in the spec's own voice
  and citing the ADR where it is applying an existing decision rather than
  making a new one.
- The spec's `## Requirements` gains the ending claim in falsifiable form:
  every one of the nineteen composed mandates carries exactly one session-ending
  instruction, and the eighteen non-`finish` kinds carry no `--done`.
- `## Test seams` records the guard for D3 — the all-nineteen assertion that
  makes a twentieth kind fail loudly rather than silently omitted — and states
  whether it is a new check or a strengthening of the golden per-kind snapshots
  the spec already agrees.
- The prose edits to `content/` are **specified, not made**. This is a `design`
  leaf; cutting the unit boundaries and writing the replacement sentences is
  implementation, and belongs to the leaves `03-planning-composer-k3` cuts.
- Decide, and record, whether D4 clears the `linkuistics:decision-records`
  when-to-write test. The root brief's expectation is that it does not — it
  writes down a path the code already supports — but if the design finds a real
  trade-off there (a reopening `finish` that relaunches never gets a second
  human confirmation before the *next* teardown proposal, for instance), that is
  the one ADR candidate in this grove.

## Notes

**Do not widen to a scope audit.** Several `kinds=*` units are plausibly
narrower than their scope — `skill-finish`'s cycle body is the obvious one, and
this leaf touches it only because it carries the ending. A systematic review of
every unit's scope is a separate concern: `leaf-add` it rather than absorbing it.

**Provisioning stays live.** Nothing in this grove retires it; the spec already
sequences that as the next increment, and the transient both-paths state is
admitted there. A design that assumes the mandate is the session's only input is
describing the increment after this one.
