# usage-guide-k52

**Integrates:** usage-guide-k51

## Goal

Triage the findings in `usage-guide-k51`, apply the valid ones, and restore the
user guide's claim to complete, source-exact coverage of its inventory.

## Context

- Read the review from its committed handle. Its findings are inputs to triage,
  not obligations accepted in advance.
- `docs/specs/user-guide-coverage.md` remains the standard; if triage changes the
  standard, make that change explicit rather than silently fitting it to the
  guide.
- The review was inspection-only and ran no tests. Post-fix verification belongs
  to this leaf.

## Done when

- Every review finding is classified and each valid one is fixed at its real
  source.
- The guide and coverage inventory agree without crossing their documented
  ownership boundary.
- The relevant focused tests and `bash scripts/check.sh` pass.

## Notes

Keep the guide's eight explicit `usage-*` anchors stable unless triage proves a
contract change is necessary; future walkthrough books reserve from that set.

## Decisions (running log)

1. **F1 — real issue, fixed at its source.** `$?` after `kill` is the signal
   delivery's status, so the transcript could not have printed `143`. Re-derived
   against `crates/keyed-launch/src/run.rs:186-211`: `reraise` really does
   restore the default disposition and re-raise, so the contract is right and
   only the *demonstration* was wrong. Rewritten as two shells — the loop's own
   shell reads `143` where Grove exits, a second shell signals it and reads `0` —
   with a sentence saying where the status must be read and how a wrapper reads
   it (`wait "$grove_pid"`). Two shells rather than a backgrounded job so the
   transcript carries no shell job-control notice it would then be lying about.

2. **F2 — real issue, fixed.** L11 obliges a *worked* `finish-commit`, and both
   invocations in the guide were refusals. Added the success case to the section
   the coverage map assigns to L11, with the real output line from
   `crates/grove-llm/src/cli.rs:455` (`finish-commit <handle>: committed as
   <change-id>`, on stderr), the path-scoping of that commit, and a pointer
   marking the two refusals as the boundary cases they are rather than the row.

3. **F3 — real issue, fixed, and the enumeration is exact.** Enumerated every
   `println!`/`eprintln!` in `crates/grove-llm/src/cli.rs` rather than trusting
   the finding's list: the exceptions to "none commits" and "prints absolute
   paths on stdout" are exactly `kind`, `finish-commit` and `complete`. Replaced
   the two universals with a three-row exception table. The review said `complete`
   "prints nothing"; it prints its outcome on **stderr**
   (`cli.rs:474`) — corrected in the table, and the in-loop success lines were
   added to the `complete` transcript, which had shown only the no-loop case.

4. **F4 — partly valid; the narrow part fixed, the broad part rejected.** The
   full restatement of the reviewer budget was a second copy of
   `plugins/grove/skills/grove/references/execute.md` and is gone. But the
   finding also condemned the integration placement algorithm and the body
   convention, and **the inventory's own J8 row requires them**: J8's end
   state is "`integrate-review-*` placed by the insert rule". Those stay. So does
   a one-sentence naming of the in-session allowance: it is a stated seam, and
   `crates/grove-llm/tests/composition_guidance.rs:598` binds `docs/USAGE.md` to
   contain `"one in-session"` — a tripwire that caught the over-broad first cut of
   this fix. The guide now names the allowance and links its owner for the
   per-kind detail, which is the drift surface F4 actually identified.
   **No change to `docs/specs/user-guide-coverage.md` was needed**; its *Out of
   scope* section already draws this line where the fix now stands.

5. **F5 — real issue, fixed, with the counterexample tested.**
   `crates/grove/tests/user_guide_coverage.rs` kept occurrences on the guide side
   (`BTreeMap<_, Vec<_>>`) and collapsed them on the standard side (`BTreeSet`),
   so a second obligation filed under an existing inventory id would vanish
   behind the first and every equality check would still pass. Split into
   `inventory_row_occurrences` (a `Vec`, occurrences preserved) and
   `unique_inventory_rows` (returns the duplicates on the error side), and added
   `a_duplicated_inventory_row_id_is_refused_rather_than_absorbed`, which
   exercises the rule on a synthetic standard — the repository's own inventory is
   correct and therefore cannot exercise it at all.

6. **The two producer-authored inventory corrections stand.** The review
   re-derived L7 and J14 and found both sound; nothing here reopens them.

7. **Node `user-guide-k11` closes with this leaf.** Its `Done when` holds: the
   inventory exists in its own change (`usage-inventory-k16`), the guide answers
   every row of it with a worked invocation (true only after F2), the guide owns
   exactly its *Human workflow and commands* row, and the adversarial read has
   closed it (`usage-guide-k51` → this leaf). Nothing in the brief needs promoting
   upward: the anchor set and the guide-link contract already live in
   `docs/specs/user-guide-coverage.md` and `docs/specs/walkthrough-books.md`, and
   the root brief already records the guide-before-books ordering and why it binds.

8. **No ADR.** Every finding resolved to a defect in an artifact or a test, not
   to a decision — the one boundary question (F4) was already recorded in the
   inventory's *Out of scope*, and re-recording it would duplicate rather than
   decide.
