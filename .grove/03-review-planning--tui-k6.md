# tui-k6

**Reviews:** tui-k2

## Goal

Adversarially review the read-only TUI design outline and its ordered working
increments against the human-approved root contract. Produce actionable
findings, or establish that no substantive findings remain, before implementation.

## Context

- Read the committed root brief and `tui-k2`, then `tree-viewer-k3`,
  `markdown-viewer-k4` and `live-viewer-k5` as a single planned subtree.
- Follow the source pointers in those leaves. Their parent supplied Tier 2
  evidence at graph generation 2026-09-15T07:00:01Z; reconfirm current coverage
  and check actual source for every load-bearing claim.
- The human explicitly chose these as ordered leaves in this grove, overriding
  the planning skill's separate-grove default. Requirements and the application
  test seam are settled; review how this plan satisfies them.

## Done when

- Contest whether each implementation leaf fits a focused session and delivers
  a runnable behavior on its own, including dependencies, documentation and
  verification. Look for a missing working increment or a hidden design project
  in the first browser leaf, which also adds the quiet try-read consumer.
- Trace the proposed read path against actual shared-lock acquisition, snapshot
  ownership, path composition and diagnostics. Find any way it could freeze the
  UI, retain a guard while idle, read through the wrong lock, duplicate grammar,
  or invoke driver/session mutation from the viewer command.
- Challenge the root-lifetime and selection rules with moves under collapsed
  ancestors, leaf-to-node changes, duplicate keys, selected-file races, brief
  replacement, and removal/recreation entirely between polls. Check that stale
  views, missing-item fallback and saved anchors have unambiguous recovery.
- Check formatted Markdown, source-anchor preservation, terminal restoration
  including partial initialization/signals, actual filesystem-change detection,
  Rust-floor dependency evidence and executable delivery against the proposed
  verification. Flag required behavior that only an injected refresh test or
  a widget-state assertion would appear to prove.
- Findings cite the producer's committed artifact and the relevant contract or
  exact source. Apply the review skill's disposition and integration procedure;
  do not implement fixes in this review.

## Notes

This review is deliberately before `tree-viewer-k3`. If actionable findings
earn an integration step, insert it ahead of that first implementation sibling
so the plan is reconciled before consumers build on it. The integration body
must point to this review's handle rather than copy its findings as obligations.
