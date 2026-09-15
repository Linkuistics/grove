# running-rows-k43


## Goal

Display a real witnessed launch on its current same-tree row, with independent
lifecycle colors and an exclusion-aware NEXT forecast in both views.



## Context

Use `grove-tui`'s existing two-capture observation, typed row renderer and public
Viewer application seam. Consume `TreeRelation::SameTree` from the loop rather
than comparing launch inode numbers. The parent contains the reviewed contract.

## Done when

- Stable same-tree Running binds only the mandate key, using current handle and
  species across rename/move/renumber, retirement and decomposition. Children
  remain eligible and finish becomes NEXT after excluding the last ordinary leaf.
- Whole-tree validation precedes exclusion. Runtime inconsistency, rejected tree
  and root-sync failure clear all row activity and NEXT. Non-same-tree or absent
  item cases remain explicitly unavailable until running-lifetimes-k44.
- RUNNING activity and item text are bold yellow; DONE/ABANDONED marker and word
  remain green/red. NEXT is bold normal activity text. Cursor changes no style.
  Both summaries preserve labels and permanent keys at 60 × 10, folded/offscreen
  and File views. No viewer allocates a finish sentinel.
- Real controlled launch tests exercise public Viewer and production observation
  for lifecycle plus RUNNING, current handles/branches, finish exclusion,
  hidden activity, selection styles and read-only snapshots. Existing viewer
  navigation, refresh and terminal tests remain green.
- Update usage/architecture/module/context-map and G6 for this shipped increment;
  keep exceptional-lifetime deferral explicit. Focused tests and the principal
  `bash scripts/check.sh` gate pass after all edits.

## Decisions (running log)

The planned first slice uses the already-reviewed same-tree binding rule. Compare
runtime evidence before computing exclusion for either display capture, keeping
activity change independent from tree acceptance. Bind rendering only after the
Viewer accepts the tree. Test through a real loop launch, not an injected
viewer activity provider. The existing locked fixture remains useful for forcing
changes between the two captures.

Implementation sequence: write real-launch application failures; implement
same-tree exclusion and row/summary presentation; run focused regressions;
reconcile shipped docs; freeze inputs and run the principal gate; retire and seal.

Public application tests use a separate self-exec driver invoking `grove_loop::run`
and a real configured session synchronized over a Unix socket. Child readiness
is followed by production Running evidence because Started can publish later;
normal driver return confirms reap. No viewer activity provider is substituted.
Byte snapshots cover `.grove` and `.jj` around two viewers; the fixture's socket
lives outside those surfaces. Timeout sleeps are polling backoff, never liveness
evidence. Helpers require an explicit child environment marker so running ignored
tests manually cannot drive the caller's working tree.

RUNNING binding is derived from current rows and activity at use time, rather
than storing a second key that could survive a failed refresh. The row renderer
resolves that key once per frame. The ordinary same-tree contract is unchanged;
all no-current-item cases remain unavailable for running-lifetimes-k44 to extend.

The first principal run caught the helper marker outside the existing
`GROVE_TEST_` namespace. The fixture now follows that classified internal-test
prefix; no runtime configuration or classifier exception is added. The run's
1,798 tracked input digests were unchanged, and its seven other gates passed.

## Notes

Validation: `cargo test --locked -p grove-tui` passed 7 unit tests, 49 browser
tests and 3 real-launch application cases. The two ignored subprocess entry
points were explicitly launched by those application cases. The initial launch
assertions failed against conservative unavailable presentation before binding
was implemented. The corrected fixture prefix also passed the focused
`every_grove_name_in_the_tree_is_classified` control.

Final `bash scripts/check.sh` passed all eight principal checks, including the
full workspace tests and six final walkthrough validations. SHA-256 comparison
covered every regular-file input listed by `jj file list`: all 1,798 were
unchanged across the final run, including source, tests, manifests, scripts,
plugin skills, docs/books and task files. The sorted digest manifest's SHA-256
was `9f834d3508af96a556d95d42745c49002218961a4ce54d6be9b4a81661411324`.
Only this validation note and retirement bookkeeping follow that measurement.

The child contract is delivered. running-lifetimes-k44 remains live and owns
exceptional-tree summaries, remaining transition controls and the complete
protocol review handoff; neither witnessed-view-k27 nor witnessed-activity-k12
closes here. No source-derived book corpus changed in this increment.
