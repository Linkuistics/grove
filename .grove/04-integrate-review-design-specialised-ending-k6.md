# specialised-ending-k6

**Integrates:** `specialised-ending-k5`

## Goal

Repair the spec amendment from `specialised-ending-k2` against the review
findings below, and reconcile
`docs/adr/mandate-delivers-the-methodology.md` where the same overclaim appears.
Keep D1-D4 themselves settled: this leaf corrects their rationale and test
contract; it does not choose different endings or broaden the unit-scope audit.


## Context

The review inspected producer commit `1c81a50a`, the current spec, its cited ADR
and glossary entries, `content/SKILL.md`, `content/prompts/continue.md`,
`tests/session_kind_guidance.rs`, and the finish/driver code. Findings are ordered
by consequence.

### F1 — the ending guard cannot prove its stated contract

**Finding.** The agreed `methodology` seam exposes unit metadata and composed
bytes, but the proposed guard counts only a **named ending set**. It therefore
cannot establish the requirement's semantic claims that there is exactly one
ending *statement*, that the `finish` prose states its choices "as outcomes",
or that no other unit restates a finish ending. In particular,
`docs/specs/mandate-delivered-methodology.md:1117-1119` is backwards: adding an
ending unit without adding its id to the named set does **not** leave a kind at
zero. The old named unit still counts as one while the new, unnamed duplicate is
invisible to the count. This is the same duplicate-prose blind spot the spec
already identifies at lines 482-489.

The five requirement scenarios split cleanly:

- coverage, later-kind omission, `--done` absence, and universal inclusion of a
  named negative-trigger unit are observable through composition;
- "no statement of the finish cycle's ending" beyond the `--done` token, and
  "stated as outcomes rather than as a rule", are prose semantics. The composer
  returns opaque bytes and has no role metadata with which to classify them.

**Repair.** Make the test contract say only what the seam can prove. Either add a
structural designation that makes *every* ending unit enumerable (and justify the
new marker surface), or narrow the mechanical guard to named unit membership and
tokens while assigning rhetorical framing/duplicate prose to the golden plus the
classification review. Do not keep semantic SHALL language while implementing a
substring heuristic. Correct the "both leave some kind at zero" explanation and
add positive controls for whatever classifier remains.

Evidence:

- `docs/specs/mandate-delivered-methodology.md:977-1008`
- `docs/specs/mandate-delivered-methodology.md:1106-1122`
- `tests/session_kind_guidance.rs:1-38` — the precedent explicitly states the
  limits of each sweep and requires controls; it is still the right thematic
  home for a genuinely structural ending guard.

### F2 — the negative-trigger paragraph contradicts its own universal scope

**Finding.** The spec makes the negative trigger `kinds=*` at
`docs/specs/mandate-delivered-methodology.md:533-535`, and the requirement
explicitly includes `finish` at lines 983-984 and 1005-1008. Yet lines 538-540
say its readers are *exactly* the kinds from which the finish endings are
withheld and that `finish` does not need it. That describes the eighteen-member
scope, not `*`.

**Repair.** Keep the settled universal scope and rewrite the explanation: the
non-`finish` readers are the load-bearing case, while `finish` also receives the
true driver-owned lifecycle fact. Reconcile every occurrence so one scope is
stated.

### F3 — absorption is mechanically catchable; the rationale is too absolute

**Finding.** `docs/specs/mandate-delivered-methodology.md:277-282` and
`docs/adr/mandate-delivers-the-methodology.md:145-150` claim that a test can
close omission but no mechanical claim can distinguish absorption from correct
generalisation. An exhaustive expected-ending mapping over `Kind` can: enumerate
each variant with no wildcard and compare the composed ending unit; adding a
variant then fails compilation or the test until its ending is chosen. A checked
literal scope expectation can do the same. This does not make negation the better
design — it moves the explicit classification away from the adjacent marker and
creates a second source of truth — but it refutes the claimed impossibility.

**Repair.** Retain the explicit eighteen-label decision, but justify it by
adjacency and single ownership of scope rather than by saying absorption cannot
be tested. Qualify the claim to checks derived only from `Kind::ALL` plus the
current universal ending invariant, which do silently accept complement
absorption. Reconcile the ADR in place. Its reopen gate can remain: the kind set
has not become unauditable and no superior replacement has been adopted.

Evidence: `tests/session_kind_guidance.rs:127-175` demonstrates the relevant
pattern: `Kind::ALL` supplies the moving set while an independently stated prose
expectation supplies what each member must satisfy.

### F4 — no-signal/manual-resume is a real trade-off, though still not an ADR

**Finding.** `docs/specs/mandate-delivered-methodology.md:612-618` dismisses the
supported alternative as "not a genuine trade-off". An operator can reasonably
prefer stopping after a `finish` session expands scope so they can inspect or
reframe the new leaf before more self-driving sessions run. The driver expressly
makes relaunch opt-in and treats no signal as a resumable stop
(`src/loop_driver.rs:169-180`). Automatic continuation versus deliberate human
resumption is therefore a real control trade-off, even though D4 correctly chose
automatic continuation.

**Repair.** Acknowledge that trade-off and keep the no-ADR verdict on the first
when-to-write limb: changing this documented ending is easy to reverse and needs
no migration. Do not claim it also fails the real-trade-off limb.

### Confirmed claims and citations

Do not churn these while integrating:

1. **Finish endings must remain triggering.** `content/SKILL.md:574-591` does not
   require a `finish` session to fetch `skill-finish-steps` before it decides to
   externalise work; the relevant trigger comes from `skill-decompose`. A
   reopening session can therefore finish without ever fetching the teardown
   procedure. The glossary's *Triggering unit / procedural unit* entry
   (`CONTEXT.md:169`) supports this reading.
2. **No confirmation crosses a reopening.** The live sentinel is reused and
   still asks for explicit confirmation (`src/tree_lifecycle.rs:127-159`),
   ordinary leaves preempt it (`src/tree_read.rs:70-111`), and every launch gets
   a fresh signal channel (`src/loop_driver.rs:135-147`). Finish transaction and
   cleanup evidence is keyed by the current signal-file attempt identity
   (`src/finish_transaction.rs:246-260`, `src/finish_transaction.rs:1172-1188`,
   `src/finish_cleanup.rs:32-42`) and is explicitly not lifecycle state read by a
   later driver.
3. **The `skill-finish` split has two universal fragments.** The negative
   driver-owned lifecycle fact and the global statement that non-routine asks are
   discretionary escalations are universal. The remaining sentinel mechanics,
   confirmation gate, and outcome-specific endings are `finish`-only. The
   "sentinel cannot starve nor preempt real work" clause is mechanics of the
   `finish` session's reopening path, not another universal ending instruction.
4. **The launcher duplicate inventory is complete.** Its instructions map to
   the six `kinds=*` units named by the spec: bootstrap, decompose, retire,
   commit, signal, and finish (`content/prompts/continue.md:2-11`; marker lines
   in `content/SKILL.md:125,204,425,525,556,574`). Only "use the grove skill" has
   no composed-unit replacement, and it is intentionally replaced by framing.
5. **The cited sources otherwise hold.** The ADR's current reopen condition is
   accurately quoted (subject to F3's rationale correction); `CONTEXT.md`'s
   *Complete finish cycle* entry at lines 204-228 already states all three
   endings; and `tests/session_kind_guidance.rs` is an appropriate home for the
   structural portion of the guard.

## Done when

- F1-F4 are repaired in the current spec and the ADR is reconciled in place for
  F3.
- The ending requirement and test-seam bullet distinguish structural assertions
  from prose reviewed by golden/classification, or introduce and justify a real
  structural surface that makes the full claim testable.
- D1-D4 and the separate `unit-scope-audit-k4` boundary remain unchanged.
- The resulting diff is documentation-only; composer/content implementation
  remains owned by later leaves.

## Notes

This is an `integrate-review-design` leaf. Apply these findings; do not implement
the composer or edit `content/` here.
