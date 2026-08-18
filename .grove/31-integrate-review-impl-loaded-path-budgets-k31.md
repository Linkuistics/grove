# loaded-path-budgets-k31

**Integrates:** loaded-path-budgets-k29

## Goal

Triage and integrate the actionable findings from the adversarial review of
`loaded-path-budgets-k10`: close the fail-open budget and inventory mutations,
make the load-column reader enforce the notation it claims to parse, restore a
scoped guard for the loop's size, and reconcile the standing prose with the
implemented measure.

## Context

### Findings

1. **P1 — the “+10% / +25% band” does not enforce the +10% set point and
   admits the zero-width defect it was introduced to prevent.** The two tests
   establish only `measurement <= budget <= measurement + 25%`
   (`tests/loaded_path_budgets.rs:758-792`); `ceiling_over`, which computes the
   claimed +10% fit, is used only in diagnostics (`:629-652`). Setting any budget
   to its exact current measurement therefore passes, recreating the prior state
   where the next word fails, and directly raising one to just under +25% also
   passes without the promised +10% fit. Give the set point an enforceable
   representation or narrow the claim to the interval actually checked. The
   spec currently makes the mismatch worse by saying the *maximum* retained
   headroom is +10% (`docs/specs/corpus-rule-ownership.md:1266-1273`).

2. **P1 — `read_load` accepts a non-notation and discards the trigger it is
   supposed to preserve.** After recognising `static(...)`, it accepts every
   backticked value containing ` @ ` and retains only the suffix
   (`tests/loaded_path_budgets.rs:514-535`). Replacing
   ``on(the condition) @ F`` with ``anything @ F`` or ``on() @ F`` leaves the
   graph, partition and all sixteen tests green even though the closed
   `on(<trigger>) @ <file>` notation — and, for an in-file row, the whole load
   predicate — has been deleted (`docs/specs/corpus-rule-ownership.md:133-155`).
   Parse and retain a non-empty `on(...)` trigger, and add malformed-value
   controls rather than checking only column count.

3. **P1 — the mirror/citation reader accepts an invalid class and silently
   drops plural citations after the first.** Both schema tests classify with
   `mirror.contains("trigger")` (`tests/loaded_path_budgets.rs:1012-1070`), while
   `cited_sentences` rewrites plural to singular and intentionally collects only
   the first number (`:1095-1114`). A mirror cell such as
   ``not-trigger (sentences 1 and 999)`` is therefore accepted as `trigger`, and
   the invalid citation 999 is never checked. That concrete mutation passes the
   reader control at `:1321-1329`. Parse the exact `own` / `trigger` / `none`
   class and the complete permitted citation grammar, rejecting plural unless
   the design is changed to define it.

4. **P1 — the inventory control accumulates permanent slack, so a row can now
   disappear without any loaded-path assertion seeing it.** The reader checks
   only `rows.len() >= 139` and lower bounds on the two partitions
   (`tests/loaded_path_budgets.rs:1127-1180`). The later corpus-split integration
   legitimately added `hard-to-reverse-pairs-with-doubt`, so the current
   inventory has already moved above that floor: deleting any static row returns
   it to the old count, changes no graph edge, and leaves every test green.
   Duplicating a well-formed row is likewise invisible because edge sets
   deduplicate. Assert rule-id uniqueness and a control that cannot gain slack
   when the inventory grows; apply the same identity check to `BUDGETS`, whose
   `find` lookup accepts duplicate dead rows (`:655-662`).

5. **P1 — a multi-owner static group passes when only one named owner is really
   static.** The runtime check uses `owners.iter().any(...)`
   (`tests/loaded_path_budgets.rs:809-833`), and the multi-owner control checks
   only that the row is static (`:1217-1237`). Adding ``no-such.md`` to the
   `finish.md` / `SIGNAL-FINISH.md` heading therefore attributes every row to a
   nonexistent third owner while the real owner discharges `any` and all tests
   pass. Require every parsed owner to exist and be on every claimed kind's
   static path; that is also what makes the spec's “owner is on k's path” claim
   true (`docs/specs/corpus-rule-ownership.md:1292-1300`).

6. **P1 — four non-`SKILL.md` edges have no sufficient realisation check.** The
   edge test accepts the owner's path appearing anywhere in the source file
   (`tests/loaded_path_budgets.rs:870-894`). Its comment says
   `tests/methodology.rs` supplies the sufficient half, but that audit explicitly
   covers only the 26 edges out of `SKILL.md`
   (`tests/methodology.rs:393-406`). For the requirements→grilling and the three
   decompose/retire→format edges, delete the actual conditional sentence and add
   the same path to an unrelated history/example sentence: the inventory graph
   and all tests stay green while no session is told when to follow the edge.
   Pin each non-`SKILL.md` source to its situation as well as to the path, or
   narrow the prose so it no longer claims a sufficient audit exists.

7. **P2 — deleting the loop-section alarm lost its scope, not merely its line
   unit.** A loaded-path word budget sees the whole `SKILL.md` body. Moving words
   from Artifacts into `## The loop` keeps every static and reachable measurement
   identical while allowing the loop to consume the whole 900-word body; the
   deleted section alarm would have exposed that redistribution. The architecture
   still says constraint 7 is specifically that the loop fit one page, then
   claims a whole-path measure answers whether the loop is small
   (`docs/ARCHITECTURE.md:870-891`). Retiring line count is sound — rewrapping is
   not growth — but the section needs a scoped word/structure guard if the
   one-page constraint remains normative.

8. **P2 — the recorded prose contains three factual overclaims after the
   structural repair.** `docs/ARCHITECTURE.md:853-855` says only `SKILL.md` and
   the kind reference can be static, contradicting both the signal-file
   derivation (`tests/loaded_path_budgets.rs:602-624`) and the spec's explicit
   three-file statement. The test says a +10%-fitted ceiling survives about a
   15% corpus shrink (`tests/loaded_path_budgets.rs:647-650,752-789`), but
   `1 - 1.10/1.25` is 12% before rounding. And the acceptance claim says every
   ratio is at most two-fifths (`docs/ARCHITECTURE.md:934-952`) even though
   `finish` is `1585/3944 = 40.19%`; the displayed 0.40 is rounded. Correct the
   static-file count, the band arithmetic, and either the boundary wording or
   the ratio claim.

### Confirmed verdicts

- The signal-file recovery is fail-closed: editing a signal file to equal any
  other embedded file makes `carriers.len() != 1` at
  `tests/loaded_path_budgets.rs:602-616`. `src/prompt.rs` is unchanged by the
  producer commit.
- The six `static(K)` spellings are stated in the spec and the reader control
  pins the family memberships, the finish exclusion and representative singleton
  forms. An unknown spelling resolves empty and fails rather than asserting
  against no kind.
- The graph's exact fourteen-edge set, cycle check and orphan-source check close
  the structural failures they name. The remaining edge defect is finding 6's
  sentence-realisation seam, not graph traversal.
- The per-kind before/after table's component arithmetic is internally
  consistent. Finding 8 is the prose boundary placed over the rounded `finish`
  ratio, not a recomputation of the nineteen measurements.

## Done when

- All eight findings are triaged against the producer revision and fixed or
  accepted visibly with a reason.
- A zero-width budget and a direct +25% refit cannot masquerade as the stated
  +10% fit; the spec and diagnostics describe the invariant actually enforced.
- The inventory reader rejects malformed load/class/citation syntax, preserves
  non-empty triggers, and cannot lose or duplicate a rule row silently.
- Every owner named by a heading is real and valid for its claimed static kinds,
  and every non-`SKILL.md` edge is tied to the sentence situation that realises
  it rather than to a path substring anywhere in the file.
- The loop's one-page constraint has a scoped measure or is deliberately
  re-decided, and all corrected architecture/spec claims agree with the code.
- Relevant post-fix verification, including mutation controls for each accepted
  fail-open case, is run by this integration session.

## Notes

This review was inspection-only. It ran no test, build, lint or format command
and edited no production, test, spec or architecture file.
