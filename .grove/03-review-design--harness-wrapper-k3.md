# harness-wrapper-k3

**Reviews:** harness-wrapper-k2
**Creator:** run 7b883bd2-52c6-4b91-b5c4-6dda307b40c7

## Goal

Adversarially read the supervised-dispatch design `harness-wrapper-k2`
produced — the reworked area specification, the ADR set and the specs that must
agree with it — against the root brief's settled requirements and guarantees,
and report findings. Fix nothing.

## Context

- The artifact: `harness-wrapper-k2`'s commit. Read its diff against its parent.
  The load-bearing files are `docs/specs/harness-selection-and-execution.md`
  (*Execution and authority*, *Supervision*, *Confinement*, *Grove
  integration*, *Records*, *Diagnostics*, *Agreed test seams*, *Out of scope*),
  the new `docs/adr/dispatch-supervises-the-harness.md`, the renamed
  `docs/adr/policy-evaluation-precedes-the-launch.md`, the edits to
  `the-launched-child-is-a-job`, `harness-selection-is-owned-by-policy`,
  `one-live-driver-per-working-tree` and `a-review-carries-its-creator-reference`,
  decisions 7 and 9 of `docs/specs/module-decomposition.md`,
  `docs/specs/standalone-invocations.md`, `docs/specs/item-status.md`, and the
  glossary's new and changed entries.
- `harness-wrapper-k2`'s running log (`W1`–`W13`) states each design decision
  with the alternatives it rejected; `plan-k1`'s (`D1`–`D9`) holds the owner's
  requirements in their own words.
- The visual document is under `docs/design/harness-selection-and-execution/`
  (`task design:harness-selection`).

## Done when

- Every finding is recorded with its location and why it matters, or the
  review records that it found none.
- If findings warrant action, an `integrate-review-design` leaf is
  `leaf-insert`ed ahead of the planning leaf, so planning reads an agreed design.

## Notes

The producer's own doubts, each a place it was least sure:

- **The launch carries two flags beyond D1's list.** D1 says a lifecycle launch
  carries the kind, the task file, the task identity and the prompt; the design
  adds `--exit-dir` and `--ending-file` as run mechanics that reach no policy
  (W3, W5). Is that within design's remit, or a change to D1 that should go back
  to the owner? Is there a design that needs neither?
- **Entry signal state through two runners.** The spec promises the harness the
  caller's entry mask and dispositions, while `keyed-launch` resets the
  terminal-generated signals to default in its child and installs TERM/HUP
  handlers. Decision 7 says a transparent caller passes its own entry state and
  the runner installs no handler over an ignored disposition. Is that coherent,
  implementable, and enough to keep a `nohup` caller's HUP ignored through
  dispatch?
- **Cancellation and grace arithmetic.** Grove waits 10 s for dispatch;
  dispatch's interactive cancellation forwards and kills after 5 s; a confined
  run is killed at once; the end observation can wait up to 2 s on the store
  lock. Check the nested `grove run`-inside-a-session case and the driver's own
  TERM against these bounds.
- **Dispatch's death.** The design accepts an orphaned harness and stops the
  loop. Is the terminal reliably given back to the driver when the orphan's
  group still holds it, and is "never relaunch" enough given that a teardown
  record still finishes?
- **The run ending and exit status.** Cancellation takes precedence over the
  exit signal; the exit-signal ending exits 0 unless the harness failed on its
  own. Does `grove run`'s publication rule still equal today's (token `done` and
  escalated-or-success)? Is reproducing a core-dumping signal without a core
  well-defined?
- **The teardown record.** `record-teardown` refuses while `.grove/` exists.
  Does any legitimate finish sequence, this repository's CLAUDE.md one included,
  hit that refusal? Is a launch-scoped record consistent with the
  one-live-driver record's rejected finish tombstone?
- **Confinement.** The confined harness must execute dispatch's own executable
  to send the exit signal, on macOS and under bubblewrap, and gets a minimal
  environment from dispatch rather than Grove. Is anything the old staging
  provided (the copied `grove-llm`, `root/tmp`, the control directory) lost
  without a replacement?
- **The ADR set.** Is splitting the old process-replacement record into the
  renamed worker record and the new supervision record the minimum coherent set,
  and is every citation of the old slug reconciled?
- **The seams.** Does each guarantee in the root brief map to a case in one of
  the four agreed seams, and does the first seam really absorb the runner's
  interface cases?
