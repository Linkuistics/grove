# ravel-caller-acceptance-k45


## Goal
Validate the complete caller migration and close k29/k5 against each acceptance
condition after surface reconciliation.



## Context
Consumes k43's nextest/Ravel check gate and k44's complete current surface
classification/source corpus reconciliation. All inherited k29 and k5 Done when
items remain binding. Ravel is independently installed 0.1.0; this Grove remains
driven by original installed 22.1.0/dispatch. No integration/release/teardown.

## Done when
- Check every k5 and k29 Done when item against actual source/fixtures/delivery;
  add precisely named missing work rather than closing an incomplete contract.
- Run task check with GROVE_TEST_RAVEL=/opt/homebrew/bin/ravel and pinned Bun
  1.4.2 on PATH. Include task ravel:fixtures ravel:commands to exercise ignored
  installed reader/resolver/command fixtures, and task ravel:review (also in
  the repository gate). Runtime suites use nextest; retain relevant doctests.
- Preserve complete saved logs and compare individual hashes before/after for
  every subject read, including all book sources/indexes/manifests, ignored
  compiler dependencies, installed Ravel, complete original v22 keg, owner
  configuration and tap/adjacent checkout state. No mutable inputs mid-run.
- Promote concise actual evidence/classification exceptions upward and reconcile
  durable decisions before retiring and closing k29/k5. Original exit signal is
  the final action after retirement/commit; no new driver is launched.

## Notes

## Decisions (running log)
- Validate each inherited acceptance clause against its actual caller source,
  installed-pair fixtures and named k6/k12 delivery/prose boundaries. The existing
  caller ADRs already describe the settled boundary; acceptance introduces no
  new routing, provenance or result decision.
- Freeze complete tracked inputs, book corpora/indexes/manifests, ignored worker
  dependencies, Cargo dependency source trees, measured tools, both installed
  kegs, owner configuration and adjacent tracked checkouts before running the
  required gate. Compare individual identities/hashes afterward. Only evidence
  and coordination handoffs are written after measurement; no input changes
  while checks run.
- The full gate and explicit installed tasks pass, but the policy ADR's later
  confinement/considered-options clauses and usage refusal recovery still
  describe retired capabilities. Their active caller classification is not a
  semantic reconciliation. Add ravel-caller-prose-repair-k46 under k29; leave
  k29/k5 open until that precise repair and fresh acceptance hold.

## Verification and handoff

`PATH=/Users/antony/Development/Ravel/target/tools:$PATH
GROVE_TEST_RAVEL=/opt/homebrew/bin/ravel task check` exits 0: all 14 principal
checks, 1,723 workspace nextest cases, nine doctests, all six final/all books
with zero deferred lines, strict types, release/archive checks and 21 skill
mutation controls pass. Explicit `task ravel:fixtures ravel:commands ravel:review
test:authority RAVEL=/opt/homebrew/bin/ravel` also exits 0 with 116 executions,
including all six ignored installed resolver/reader/command cases. Public and
retained policy/provenance checks pass. Across both logs, 1,738 unique nextest
identities pass; all 101 shared identities/outcomes match individually.

Complete logs/instruments under `target/k45/`: `check.log`,
`installed-acceptance.log`, `measure.py`, `cargo-metadata.json`, `before.json`,
`after.json`, `digest-comparison.json`, `probes-before-runtime.json`,
`per-case-comparison.json`, `inventory-comparison.json`, `observed-controls.json`,
`authority-list-comparison.json`, `coverage-summary.json`, `acceptance-gaps.json`
and the three adjacent checkout before/after state pairs. All 17,062 individually
named subjects and all four generated probes match without changed/additional/
removed inputs; adjacent states match. The frozen subjects include complete
book sources/indexes/manifests, ignored worker/compiler dependencies, Cargo
dependency sources, both installed kegs and owner configuration. Ravel/tap stay
clean successors of 3916a4a1/bd5aa37b; default Grove stays at 89f947d5.
Ravel 0.1.0 and original installed Grove/dispatch 22.1.0 retain their bytes.

`docs/ravel-caller-surface-audit.md` now carries the clause-by-clause k5/k29
matrix, exact remaining prose gaps, controls and measurement limitations.
The policy ADR at lines 108–116/141–162 retains private records, installation,
executable provenance and shared release claims; usage lines 435–440 prescribe
an absent inspection line. k46 carries the repair and full reacceptance charter.
These current caller clauses cannot be excused by k12's broader prose scope.
No other missing runtime/fixture acceptance was found within the examined
contract. k6/k12 retain their existing delivery/prose responsibilities.

k45 completes validation and precisely names missing work as its Done when
requires. k29/k5 remain live through k46: no ancestor close is claimed. Only
evidence/coordination notes and the new leaf change after the frozen run. No
runtime edit, installation, integration/release, new driver or teardown occurred.
This session ends with its original `harness-dispatch exit`, after retirement
and a sealed task commit, as its final action.

## k44 acceptance inputs

Read `docs/ravel-caller-surface-audit.md`, its per-file TSV and k44's terminal
handoff before measuring. `target/k44/` has exact lexicon/line classifications,
observed controls, source pins, per-case comparisons and two matching 3,099-file
measurements. Corrected dispatch:check passes 345 nextest cases and has no
library doctest target; the root gate still runs actual workspace doctests.
Caller focused cases (121), sentinel cases (2), all six final/all books,
conformance/mutation controls, clippy and real release-preparation pass.
The TSV is a measurement-boundary snapshot: coordination handoff/retirement
notes follow it. k44 leaves all ownership-prose-k12 and retained-delivery-k6
files explicitly classified; verify those scope boundaries against the parent
contract rather than calling old spellings absent from the whole repository.
