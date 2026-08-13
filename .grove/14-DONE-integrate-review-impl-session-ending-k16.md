# session-ending-k16

**Integrates:** session-ending-k15

## Goal

Repair the three actionable prose/test-contract findings from the adversarial
review of `session-ending-k9`. Keep the integration bounded to the specialised
session-ending instruction; `unit-scope-audit-k4` still owns the wider scope
audit.

## Context

The reviewed producer is commit `794219b6c61a` (`session-ending-k9`). Its
recorded verification was green: `cargo test` (40 binaries, 1042 tests),
`cargo fmt --check`, and `cargo clippy --all-targets`. The review was inspection
only and did not repeat those commands.

### Finding 1 — the finish ending contradicts its no-signal outcome (high)

At `content/SKILL.md:599`, `skill-finish-endings` says all three outcomes are
open and then says, "Whichever you reach, the signal is your last action."
The third outcome at `content/SKILL.md:607` is explicitly **no signal**. A
declining or unattended finish session therefore receives mutually exclusive
directions about whether it must signal. Reword the common preface so it applies
to both signalled outcomes without claiming that the no-signal outcome has a
signal action.

### Finding 2 — undeclared units already restate session endings (high)

The prose limb at `docs/specs/mandate-delivered-methodology.md:1124` says no
unit may restate an ending without naming `grove-llm complete` or `--done`, but
the composed mandates already contain two such restatements:

- `skill-self-driving-loop` at `content/SKILL.md:59` says sessions relaunch only
  after a completion signal and that every other exit stops the loop. For the
  eighteen non-finish kinds this repeats the no-signal ending stated again by
  `skill-signal` at `content/SKILL.md:570`. Calling one the driver's contract
  does not remove the duplicated decision from the same mandate; the spec's own
  rationale rejects two statements of one rule because the reader must decide
  whether they agree.
- `skill-finish-cycle` at `content/SKILL.md:584` says an unattended finish
  session "reports the plan and stops." That repeats the no-human/no-signal
  outcome in `skill-finish-endings` at `content/SKILL.md:607` while naming
  neither token the complement sweep recognizes.

Reconcile or rephrase those units so only the declared ending unit states each
session ending while the self-driving-loop and finish-cycle units retain the
non-ending context they genuinely own. Re-check the whole composed corpus for
the same semantic shape; do not widen this into the separately scheduled scope
audit.

### Finding 3 — the drift pin cannot guard the second prose limb (medium)

`tests/session_kind_guidance.rs:1204` pins only `skill-signal` and
`skill-finish-endings`. The claimed second limb concerns **every other unit**,
so a rewrite of `skill-self-driving-loop`, `skill-finish-cycle`, or the universal
negative trigger can add an ending phrased without the exact command or flag
while the complement sweep, composition golden, and byte pin all remain green.
Finding 2 is a present demonstration of that blind spot. This contradicts the
claim at `docs/specs/mandate-delivered-methodology.md:1130` and
`tests/session_kind_guidance.rs:1143` that prose drift re-enters classification
review.

Either provide a drift boundary that can actually notice changes to the units
whose prose is being classified, or narrow the spec/test claim and record the
remaining manual-review limitation honestly. A pin of only the declared ending
units cannot support the "no other unit restates" limb.

## Done when

- The shared finish preface no longer contradicts the no-signal outcome.
- No composed mandate contains an ending restatement outside its declared
  ending unit, including the two concrete cases above.
- The drift-pin claim and its actual boundary agree; the relevant negative
  control demonstrates the failure shape the chosen boundary is meant to catch.
- The spec remains a current-state description of the resulting cut.
- The focused and full verification appropriate to the touched methodology,
  tests, and spec are recorded.

## Notes

The other review doubts cleared: the table is keyed by what the finish session
did; the cross-reference to the universal externalisation rule is not itself a
kind exception; the merged universal fragments read coherently for all nineteen;
the mechanism withheld on each side is sufficient; the new units otherwise read
standalone; the four structural claims use the production composer and can fail;
and the driver's `None`/`Relaunch`/`Done` branches confirm the documented runtime
outcomes (`src/loop_driver.rs:170`, `src/complete.rs:52`). The spec amendments
match the narrowing/three-way split except for the drift-pin overclaim above.
