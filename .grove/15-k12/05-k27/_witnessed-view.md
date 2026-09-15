# witnessed-view-k27 — brief


## Goal

Show real witnessed RUNNING and exclusion-aware NEXT in tree rows and both
views' persistent summaries, completing the product increment.



## Context

Consume witnessed-observation-k26 through the existing two-capture
`grove-tui::observation::capture` and public `Viewer::new`/`act`/`tick`/`render`.
Starting source is `crates/grove-tui/src/observation.rs`, `src/lib.rs` and
`tests/browser.rs`. Rows already contain typed identity/lifecycle/species/counts;
refresh_at already clears NEXT before capture and retains independent activity
on tree error/absence. Reuse these landed seams and the shared select_snapshot.
The fixed activity column, lifecycle styles and four-line chrome already exist.

## Done when

- Compare runtime identity, witness state and verified relation across captures
  independently of row/file/tree consistency. Activity change yields Busy and
  clears current attachments without rejecting a stable tree. Tree failure,
  root-sync failure or stale retained rows suppress all row activity and NEXT;
  a fresh witnessed summary survives with the correct absence/freshness label.
- Only a verified same-tree result attaches by permanent key or excludes it
  before shared selection's finish rule. Rename, move, renumber, DONE/ABANDONED
  retirement and decomposition to a branch preserve direct RUNNING using the
  current handle/species. Ancestors inherit no marker; a branch's children may
  be NEXT. Only-running-ordinary-plus-finish makes finish NEXT. Validate duplicate
  keys and multiple finishes before exclusion, including terminal/branch keys.
- Item absence uses the launch handle plus `item absent` without row attachment.
  Root removal retains the mandate with `tree absent`; unreadable tree uses
  `tree unavailable`. Replacement, including the first viewer arriving after
  replacement and a reused key/handle, says `previous tree`, excludes no key
  from the new tree and clears old reading/selection state. Same-root brief
  edits preserve that state. No viewer allocates a finish sentinel.
- RUNNING word and item text are bold yellow; lifecycle marker/word keep their
  own colors, including green DONE and red ABANDONED beside RUNNING. NEXT is bold
  normal-foreground activity text only. Selection adds the cursor gutter alone.
  Words/glyphs remain sufficient with color disabled. Hidden/folded/offscreen
  activity remains in Tree and File summaries; reserve labels and qualifiers
  before eliding long slugs, retaining the permanent key at 60 × 10.
- Actual controlled launches reach the public Viewer seam for LIVE/DONE/
  ABANDONED plus RUNNING, failed/immediate launch, signal-before-reap, handoff,
  killed driver, replacement and absent item/root. Keep the real positive
  Running control alongside stale-record rejection. Snapshot read-only tree and
  administration behavior with absent trees, non-jj locations, aliases, missing
  namespace and multiple viewers. No injected viewer-only activity provider.
- Retain saved Tree/File viewports, folds, Unicode/clipping, resize, aggregates,
  help/undersized ticking, one pending retry, quit responsiveness and legacy
  active-epoch behavior. Use semantic text and style assertions through
  TestBackend; terminal mapping/cleanup fixtures remain regressions.
- Update usage, architecture, module and context-map summaries plus affected
  walkthroughs. Reconcile the shipped witnessed behavior in G6, removing only
  its implementation deferral and keeping unrelated signal/panic-restoration
  evidence obligations. Focused application/selection/controlled-launch tests
  and `bash scripts/check.sh` pass after all edits.
- After the artifact and k26's macOS/Linux evidence exist, commission a
  `review-impl` leaf under witnessed-activity-k12 with bare stem
  `witnessed-activity`. Its Reviews pointer names this producer and its context
  names launch-events-k24, launch-witnesses-k25 and witnessed-observation-k26.
  Ask it to attack the complete protocol, resource lifetime, probe precedence,
  forced-reuse mutations, platform evidence, viewer binding and current docs.
  Do not also spend an in-session reviewer. The node remains live until review
  and any actionable integration are resolved.

## Notes

The reviewer re-derives from producer commits; it decides whether findings earn
an adjacent integration. Do not prewrite integration tasks or transcribe future
findings. The final close must still check every node criterion, not infer
completion from these four leaves having retired.

## Decomposition

Source inspection separates ordinary same-tree viewing from exceptional tree
lifetimes. The former changes selection and rendering together; the latter
must preserve independent runtime evidence through failures and replacement.
Both children ship their own application controls and current documentation.

- running-rows-k43 delivers witnessed same-tree RUNNING, exclusion-aware NEXT,
  lifecycle styling and persistent summaries through real controlled launches.
  Other running relations remain explicitly unavailable until the next child.
- running-lifetimes-k44 delivers absent-item/absent-tree/unavailable-tree and
  previous-tree summaries, replacement state reset and the remaining runtime
  transition/failure controls. It reconciles the whole node and G6, then
  commissions the complete protocol review under witnessed-activity-k12.

The original Done when remains the node's full contract. No in-session reviewer
is used alongside the required eventual protocol review.
