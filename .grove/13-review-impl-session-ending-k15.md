# session-ending-k15

**Reviews:** session-ending-k9

## Goal

Inspect the commit that specialised the session-ending instruction. The
mechanical claims are covered and green; what is uncovered is **prose**, and
uncovered by design — the requirement itself names two limbs the composer's
opaque bytes cannot support a SHALL about
(`docs/specs/mandate-delivered-methodology.md`, *Every kind's mandate states
exactly one session ending*). Those two limbs are this review's centre of
gravity.

Inspection only — read the committed diff, the source, the spec and the recorded
evidence. Do not run the suite, edit code, or redo the work; findings go to an
`integrate-review-impl` leaf if there are any worth acting on, and to nothing at
all if there are not.

## Context

The producer's commit names `session-ending-k9`. It changed four things:

1. **`content/SKILL.md`** — two markers in, four out. `skill-signal` was
   *narrowed*, not split: `kinds=*` → the explicit eighteen, with the branch
   sentences ("Plain `complete` signals a **relaunch**; the **Finish** cycle
   below ends instead with `--done`… the loop tells the three cases apart")
   deleted and replaced by one sentence saying that ending without signalling
   stops the loop. `skill-finish` was split three ways — `skill-finish`
   (`kinds=*`, the negative trigger plus the discretionary-escalation clause),
   `skill-finish-cycle` (`kinds=finish`, sentinel mechanics and the human gate),
   and `skill-finish-endings` (`kinds=finish`, the three-outcome table, carrying
   the four `defers=` targets that were on `skill-finish`). No other unit's
   marker moved.
2. **`tests/session_kind_guidance.rs`** — a new closing section: four claims
   generated from `Kind::ALL`, two controls, and a byte-level drift pin holding
   the two ending units' whole source as hand-edited constants.
3. **`tests/methodology.rs` + the golden** — `EMBEDDED_UNITS` 138 → 140, the
   composition golden regenerated.
4. **`docs/specs/mandate-delivered-methodology.md`** — three paragraphs
   reconciled with what the cut turned out to be, because the spec had said both
   units would be *split* and one was narrowed instead.

Verification the producer ran, so you can judge it rather than repeat it:
`cargo test` green (40 binaries, 1042 tests), `cargo fmt --check` clean,
`cargo clippy --all-targets` silent. The golden moved by exactly three rows, all
on `finish`: `-skill-signal`, `+skill-finish-cycle`, `+skill-finish-endings`.
Cross-checked out of the golden: `skill-signal` reaches 18 kinds and none of them
is `finish`; the two new units reach `finish` only; `skill-finish` reaches all 19.

## Done when

Each doubt below is either cleared or written up as a finding, ordered by how
little the suite says about it.

- **Limb 2 — does any unit restate an ending in words naming neither
  `grove-llm complete` nor `--done`?** This is the one the complement sweep
  provably cannot see, and the producer judged one candidate clear rather than
  finding none: `skill-self-driving-loop` (`kinds=*`, so it is in all nineteen)
  says relaunch happens "only if the agent fired the completion signal" and that
  "**Relaunch is opt-in:** any other exit — your `/exit`, the human's Ctrl-C, or
  a crash — **stops** the loop". The producer's reasoning was that this states
  the *driver's* contract rather than instructing the session, so it is not a
  second ending. Test that reasoning adversarially, and note that the narrowed
  `skill-signal` now ends with a sentence making the same point in the session's
  voice — so if the judgement is wrong, the eighteen hold two statements of one
  rule with nothing keeping them in step, which is the exact cost D1 spent the
  launcher to remove. Sweep the other `kinds=*` units for the same shape rather
  than judging this one alone.
- **Limb 1 — do the `finish` endings read as outcomes?** Read
  `skill-finish-endings` as a `finish` session holding only its own mandate. The
  table is keyed by *what the session did*, which is the shape the requirement
  asks for; the paragraph under it is where a rule qualified by another kind's
  could have crept back in. "You are told, like every session, to externalize
  surfaced work" is the phrase to weigh — it appeals to a rule delivered by a
  different unit (`skill-decompose`), which is legitimate cross-reference or a
  smuggled exception depending on how you read it.
- **The merge of two universal fragments into one unit was forced, and reads
  correctly for all nineteen.** The negative trigger and the
  discretionary-escalation clause sat at opposite ends of the old paragraph with
  `finish`-only prose between them, and a unit is one contiguous span with one
  scope — so they were merged rather than left as a span each side. Two things to
  check: that the merged text reads correctly standing alone for the *eighteen*
  (it now describes a gate none of them will meet), and that it reads correctly
  for `finish`, which meets itself in the third person — "That `finish` session
  asks a human for explicit confirmation first" is a session reading about
  itself. If that is wrong the fix is prose, not scope.
- **What the narrowing withheld from `finish` is genuinely not needed by it.**
  `skill-signal` carried the mechanism — `GROVE_SIGNAL_FILE`, the driver-side
  watcher, grace → SIGTERM → kill-grace → SIGKILL, and the safe no-op outside a
  loop — and none of it reaches the `finish` mandate any more. What replaces it
  is one clause in `skill-finish-endings` ("the loop driver is watching for it
  and ends the session itself") plus step 3 of the *procedural*
  `skill-finish-steps`. A `finish` session that reaches the **reopening** ending
  never fetches that procedural body, so judge the withholding against that path
  specifically, not against the teardown path.
- **What the narrowing withheld from the eighteen is genuinely not needed by
  them.** The mirror question. The replacement sentence claims ending without
  signalling stops the loop; confirm that is true of the driver as it stands
  (`src/loop_driver.rs`, `src/complete.rs`) and not merely true of the two cases
  the deleted sentences enumerated.
- **The two new `finish` units read correctly standing alone.**
  `skill-finish-cycle` opens "Once no ordinary live leaf is left…" with no
  subject — fine in the mandate, where `skill-finish` precedes it, and fine in
  the document. Judge the `grove-llm methodology skill-finish-cycle` fetch, which
  is the third way its bytes are read.
- **The drift pin's boundary is the right one.** It holds `skill-signal` and
  `skill-finish-endings` whole, and nothing else. So a rewrite of the *negative
  trigger* — a `kinds=*` unit shipping into all nineteen, whose withholding the
  design calls an unasked question against a destructive action — is caught by no
  test at all, and the composition golden will not move for it either. Decide
  whether that is the honest boundary the spec names ("the ending units
  themselves") or a hole the spec did not notice it was leaving.
- **The guard's four claims are the four the spec asked for, and each can fail.**
  Both controls exercise the *helpers* over synthetic or mutated input. Check
  that the helper each control exercises is the same one the corresponding claim
  calls, and that no claim's assertion is trivially satisfiable in a way its
  control does not reach — in particular
  `every_kind_is_told_exactly_one_session_ending`, whose control mutates a scope
  rather than adding a kind, which is the hazard it stands in for.
- **The spec amendments are current-state and honest.** Three paragraphs were
  edited because the design had predicted two splits and got one narrowing and
  one three-way cut. Read them against the diff: a spec that describes a cut
  nobody made is worse than one that is merely terse. The added justification for
  a hand-edited pin over a regenerable golden is a claim about human behaviour —
  weigh whether it belongs in a spec at all.

## Notes

**The wider scope question is out of bounds.** `unit-scope-audit-k4` owns whether
any `kinds=*` unit that carries no ending should be narrowed, and it is a
separate increment for a stated reason. A finding that some unit looks wider than
it needs to be is **already known and already scheduled** — do not raise it. The
in-scope question is only ever *does this unit state an ending*.

**No behavioural check is available.** This is a meta-grove across the build
boundary, and provisioning is still live, so every session in this loop also
receives the whole unsliced `SKILL.md` as a harness skill. Nothing here can be
verified by watching a session behave differently; judge the composed mandates
and the units' bytes.

**`content/SKILL.md` must still read as a document.** It is provisioned whole
while both delivery paths are live, so a cut that composes correctly and leaves
the Finish section reading as a broken sequence of fragments is a finding.
