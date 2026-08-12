# mandate-delivery-integrate-k5

**Integrates:** `mandate-delivery-review-k3`
**Reviewed producer:** `mandate-delivery-k2` (`a9d138175ac9`)

## Goal

Repair the mandate-delivery design before `increments-k4` decomposes on top of
it. Rework the spec, ADR set, glossary, and architecture citations in place;
this leaf owns design artifacts only, not implementation.

## Context

The review inspected the producer's committed diff, the current record set, and
the source claims about the release scan and methodology identity. Five findings
survived.

### B1 — A triggering unit cannot address its deferred procedure

`docs/specs/mandate-delivered-methodology.md:100` says a triggering slice's own
id is what the session passes to `grove-llm methodology` for "the deferred
half". But ids are globally unique across both classes
(`docs/specs/mandate-delivered-methodology.md:112`), and the verb contract
returns the source bytes of the unit named by that id
(`docs/specs/mandate-delivered-methodology.md:324`). Passing a triggering id
therefore returns the triggering unit again, not a distinct procedural unit.
The design makes this worse at
`docs/specs/mandate-delivered-methodology.md:177`, where it says content
references no ids at all. The session has no specified path from an inlined
condition to the procedure it is now expected to know it should fetch.

Choose and specify an actual relationship: for example, an optional procedural
target on a triggering marker, an explicit reference in triggering source with
a mechanically parsed contract, or different verb semantics. Then extend the
build invariant to prove every declared target exists and has the procedural
class. Reconcile the same unsupported "unknown id is loud" claim in
`docs/adr/one-build-owns-a-session.md:20` and `CONTEXT.md:75`.

### B2 — An unterminated fence defeats the build gate

Markers inside fenced blocks are ignored
(`docs/specs/mandate-delivered-methodology.md:127`), but an unterminated fence
is absent from the exhaustive build-error classes at
`docs/specs/mandate-delivered-methodology.md:171`. A fence opened after the
first real marker can therefore absorb every later marker-shaped line into one
giant final unit without violating any stated syntax, semantic, or reference
check. Golden ids may catch that in this repository's test suite, but the embed
itself has not failed the build as the design promises.

Require neutral fence state at end of file and make an unterminated fence a
named build error. Add requirement scenarios for a balanced fenced example
marker being ignored and an unterminated fence being rejected; the current
requirements at `docs/specs/mandate-delivered-methodology.md:288` cover neither.
This also resolves the standalone-slice doubt for fences: a recognised boundary
can occur only while fence state is neutral. Treat broader "reads correctly
standing alone" quality as authoring review unless a further mechanical rule is
actually specified.

### B3 — The architecture cites an ADR that no longer supports its claim

The new pointer correctly labels the embedded-methodology section as describing
built behavior scheduled to retire (`docs/ARCHITECTURE.md:626`). Later, however,
the same section still says `one-build-owns-a-session` settles three
provisioning-era mechanisms (`docs/ARCHITECTURE.md:731`). The reworked ADR now
says the shared directory is gone and retains only the pre-launch pairing report
(`docs/adr/one-build-owns-a-session.md:9`); it no longer records the stamp repair
or in-session directory warning in the adjacent table. The top pointer does not
make that citation true when followed.

Keep the built description if retirement really owns its rewrite, but remove or
locally qualify the false citation now, or retain the still-live rationale in a
record that actually supports it until the mechanism retires. Re-check every
other citation to the reworked ADR under the minimum-coherent-set rule.

### B4 — The size alarm has no bound

The spec says a per-kind bound is asserted
(`docs/specs/mandate-delivered-methodology.md:283`) and later refers to the alarm
"as specified above" (`docs/specs/mandate-delivered-methodology.md:391`), but it
never states a number, comparison rule, or counted bytes. The review leaf names
64 KiB, but that choice did not reach the durable design.

Specify the alarm completely. An arbitrary fixed threshold is honest here: it
is a classification-drift alarm, not an argv safety limit, so deriving it from a
machine's current environment would confuse its purpose. If 64 KiB remains the
choice, say why it separates the expected low-tens-of-KiB triggering set from a
substantially misclassified mandate, and define whether the comparison includes
join bytes and runtime facts.

### B5 — The methodology listing has no stable data format

The no-argument verb returns a multi-field inventory
(`docs/specs/mandate-delivered-methodology.md:324`), but the exact line grammar
is unspecified and JSON is explicitly rejected because the consumer is an
agent (`docs/specs/mandate-delivered-methodology.md:407`). Agent consumption is
the reason the inventory needs a stable parseable form: ids must round-trip into
fetch calls without scraping prose, and a procedural unit has no `scope` value
despite the listing promising that field for every row.

Keep raw fetched units byte-exact, but give listing mode a stable schema — for
example `--json` only when no ids are requested — or fully specify an escaped
line format and the representation of an absent scope. Reconcile the out-of-scope
decision with the chosen CLI contract.

## Done when

- The trigger-to-procedure relationship is explicit and mechanically checked.
- Unterminated fences fail the build and fence behavior has requirement
  scenarios.
- Every surviving ADR citation supports the claim attached to it.
- The size alarm and methodology-listing output are fully specified.
- The glossary and both ADRs agree with the corrected spec.

## Notes

Confirmed non-findings from the review, to avoid re-opening settled ground:

- Forbidding `kinds` on procedural units is sound. A session-kind filter for the
  inventory is a query concern, not marker semantics.
- The source claims are correct. `assert_methodology_pairing` currently requires
  the marker in `grove` and rejects it in `grove-llm`; the production/test
  `CONTENT_MARKER` definitions are confined to `scripts/release-common.sh` and
  `tests/provision.rs`. The compile-time hash exists to keep an embed-free
  `grove-llm`; once both binaries link the embed, `--content-hash` and the
  pre-launch report can hash that embed directly while the `build.rs` hash,
  `GROVE_CONTENT_HASH`, the duplicate traversal, and their equality test retire.
- Per-file order can place ordinary `MANDATE.md` first without a composer special
  case. No current format-guide reference requires interleaving with part of
  `SKILL.md`; that conclusion assumes the standalone-unit contract repaired in
  B1/B2.
- The byte-exact requirement already includes the marker line explicitly at
  `docs/specs/mandate-delivered-methodology.md:342`; no additional acceptance
  scenario is required for that point alone.
