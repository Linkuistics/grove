# direct-dispatch-k5

**Reviews:** direct-dispatch-k3

## Goal

An adversarial read of the design `direct-dispatch-k3` delivered, before
`direct-dispatch-k4` cuts it into increments: the reworked
`docs/specs/harness-selection-and-execution.md` and
`docs/specs/standalone-invocations.md`, the reworked
`docs/adr/harness-selection-is-owned-by-policy.md` and the smaller edits to
three other records, against the root brief's settled requirements and the code
the design changes.

## Context

- **Report only what would change what gets built or how it is tested.** The
  human's rule for this grove is simplicity, and it covers this review: no
  wording polish, and no finding whose remedy is more mechanism than a
  requirement asks for. A finding that the design builds *too much* is in scope.
- **The requirements are the human's and are not under review.** What is under
  review is whether the design meets each one, and whether two of its answers
  can both hold.
- `direct-dispatch-k3`'s decision log says what each call was made against.
- **The doubts the producing session could not close for itself:**
  1. *Confined `grove run` through `inspect`.* Requirements 1 and 10 say every
     standalone invocation is launched by Grove running harness-dispatch,
     through a caller-neutral capability. The design selects with
     `inspect --json` and has Grove's runner launch the reported command, so
     dispatch selects and does not launch, and no run is recorded. Is that an
     honest reading of both requirements, and of requirement 14? Does the
     reported `command` give the runner everything its confinement needs, given
     that the runner resolves the program again for its read grant
     (`crates/keyed-launch/src/confinement.rs`)?
  2. *Owner settings are one global file.* Requirement 8 says what an owner sets
     per invocation today through flags in a command definition stays settable.
     A command definition could differ by kind; the settings file cannot. Does
     that lose anything an owner needs, a longer bound for one kind included?
  3. *The records left for the implementation to delete.* The two configuration
     ADRs and the modular-configuration spec stay until the machinery goes, so
     the ADR set briefly holds a record the reworked one contradicts. Is the
     deferral sound, and is `direct-dispatch-k4`'s list of what goes with them
     complete?
  4. *The launch document stays version 1 with nulls.* A new run writes `null`
     for the candidate ID, the selection form and the explicit choice. Does any
     reader of a run (`record show`, `record observe`, run lookup, the review
     example) break on that, or on a 21.13.0 record?
  5. *What the tool no longer checks.* The command validates the returned
     command's shape and nothing else, and inspection without a prompt evaluates
     with a marker. Is anything a requirement relies on now unchecked?
  6. *Parts of the specification this session did not rewrite.* Its runtime
     discovery, bounded context and delivery sections were edited only where a
     sentence named the catalog, the choice or the withheld prompt. Does any
     sentence left standing contradict the new contract?
  7. *The sample's selections.* Four arrangements, two modifiers, a default, and
     a choice file that replaces the default whole. Is parity with the pinned
     configuration well defined for every selection the capture enumerates,
     `codex-sol`'s route patches under an arrangement whose reviews are not
     Codex's included?

## Done when

- Each doubt above carries a ruling.
- Findings are recorded as this kind's skill directs, each naming the
  requirement or the design statement it is against and what it rests on.
- If a finding is worth acting on, an `integrate-review-design` leaf is cut
  where the walk reaches it before the planning leaf.

## Notes

`direct-dispatch-k3` was launched directly, not through harness-dispatch, so it
had no run to name and this leaf carries no `**Creator:**` line.
