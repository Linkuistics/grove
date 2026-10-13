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
