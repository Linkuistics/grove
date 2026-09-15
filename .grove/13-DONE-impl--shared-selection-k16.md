# shared-selection-k16


## Goal

Make driver selection, grove-llm pick and viewer validation reject the same
ambiguous tree through one typed loop rule. This working increment follows
lifecycle-rows-k10 and supplies the selector consumed by idle-next-k11.



## Context

Implement One selection rule in docs/specs/item-status.md. Start from selected,
select_in and select_in_write in crates/grove-loop/src/task_tree.rs and the
viewer duplicate-key validation in crates/grove-tui/src/observation.rs;
verify the current source and graph coverage before editing. Move whole-tree
validation and selection behind the highest useful typed loop seam over an
already-read snapshot, with an optional excluded permanent key. Ordinary
selection supplies no exclusion. The viewer uses the common validation now;
idle-next will consume exclusion-aware selection for its forecast.

## Done when

- The common rule validates duplicate keys across every item, including terminal
  leaves and branches, and multiple live finishes before applying exclusion.
  Exclusion removes only the named candidate; running-branch children remain
  eligible. Select the first ordinary leaf in depth-first position order, or
  the sole finish when no ordinary candidate remains, or none.
- Driver, grove-llm pick and public Viewer tests demonstrate consistent
  duplicate-key refusal. The viewer's private duplicate-key rule is removed.
  Valid-tree ordering is unchanged and viewing never creates a finish sentinel.
- Typed selection tests cover no exclusion, excluded ordinary/finish/terminal/
  branch keys, finish-only remainder, early finish with later ordinary work,
  no candidates, duplicate finishes and duplicates that exclusion might hide.
- Update selection/viewing sections of docs/USAGE.md with the new refusal and
  repair guidance that does not reuse permanent identity. Reconcile G6 in
  docs/specs/user-guide-coverage.md for this shipped part, leaving activity
  obligations pending. Update architecture/module/context-map ownership where
  the shared seam changes it.
- Update affected docs/walkthroughs/grove-loop explained fragments, source and
  concept indexes and walkthrough.toml ownership ranges in this same change,
  following the book authoring rules. New source files matched by the corpus
  are part of that obligation. Run focused grove-loop and grove-tui tests and
  the root brief's principal checks after all edits.

## Notes

This increment introduces no runtime observation, namespace discovery or activity
chrome. Its handoff is consistent malformed-tree refusal plus a tested selector.
If a substantive selection doubt remains, commission review-impl with bare stem
shared-selection after the artifact exists; integration is created for findings.

## Decisions (running log)

- Expose `grove_loop::select_snapshot(root, snapshot, excluded_key)` over the
  already-read typed snapshot, returning the existing `Selection` and opaque
  crate error. Both guarded driver paths use the same internal rule with no
  exclusion; the viewer calls the public rule to validate its snapshot.
- Validate every positioned key and all live finishes before filtering live
  candidates. Exclusion compares only the candidate's permanent key, so it
  cannot hide malformed input or exclude a branch's descendants.
- Keep selector contract tests at the public library seam with temporary
  non-jj trees, and exercise refusal through the actual driver, pick binary
  and public Viewer. Update the existing walkthrough fragments after source
  changes, then run the focused suites and `bash scripts/check.sh`.
- One bounded fresh-context review examined the implementation and public
  consumer tests for missed validity, exclusion and ordering cases; it reported
  no actionable findings. No review leaf is needed for a remaining selection
  doubt. This changes no driver-lease decision and needs no new ADR.
