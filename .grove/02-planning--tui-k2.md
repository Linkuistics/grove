# tui-k2

## Goal

Turn the agreed read-only TUI requirements into a source-grounded design outline
and an ordered set of independently demonstrable implementation increments.
The root brief owns the requirements and the human-approved test seams.

## Context

- `plan-k1`: the completed requirements interview and running decision log.
- `crates/grove-loop/src/lib.rs`: `read`, `Reading` and `Tree` expose the
  existing shared-lock reader and its snapshot.
- `crates/grove-loop/src/task_name.rs`: `TaskName`, `Parts`, `Outcome`, `Kind`
  and `Handle` expose the canonical names and their domain meaning.
- `docs/specs/module-decomposition.md`: the existing library/binary split.
- `docs/USAGE.md` and `Cargo.toml`: current invocation, supported workflow and
  workspace conventions. Verify dependency choices against their actual source.

## Done when

- A compact design outline assigns ownership of the viewer, its command entry
  point, its use of the existing tree reader, Markdown rendering and refresh.
  Record it in the root brief unless a durable spec earns its place. If a
  substantial design question needs a separate session, cut that work explicitly
  rather than hiding it inside an implementation leaf.
- The outline specifies launching against one tree, keyboard navigation and
  scrolling, branch expansion, visible outcomes, refresh responsiveness and
  preservation of the selected item's identity and reading position.
- Missing or temporarily malformed trees, unreadable or disappearing files,
  tree removal/recreation and terminal resize have explicit visible behavior
  and recovery rules. Tree reads remain read-only and release their guards
  before waiting for input or another change.
- Implementation leaves cover complete user-visible slices, each with a
  checkable outcome through the agreed test seam. The first slice can be run
  and demonstrated without waiting for another implementation leaf.
- The plan includes real filesystem-change verification and a terminal smoke
  check, along with usage documentation and any delivery changes needed to make
  the viewer usable. Keep framework and dependency work inside the increments
  that need it.

## Notes

The human has agreed to the requirements and test approach. Do not repeat that
interview or widen the permanent read-only scope. Routine command, rendering,
navigation and recovery choices are yours to resolve under that contract.

The name's permanent key supplies identity across renumbering and moves within
one tree. A leaf can become a node, and a removed/recreated root can reuse keys;
specify selection behavior for those cases rather than treating a path or key as
an identity across every possible lifetime. Preserve selection, scroll and
expansion wherever the selected item and content still support them.

`Outcome::Live` means unfinished work, not a running session. The inspected
private driver epoch record carries process identity and a signal path, so it
does not supply the selected task for a viewer. The agreed product observes
the tree; avoid inventing runtime status from the next eligible leaf.

Requirements used Tier 2 graph verification for this workspace, indexed in fast
mode at generation 2026-09-15T07:00:01Z. Coverage recorded no gaps for
`lib.rs`, `task_name.rs`, `task_tree.rs` or `driver_lease.rs`. Documentation
was excluded by that mode and read directly. Reconfirm freshness before using
the graph. Relevant symbol searches were fully paginated. Call edges for the
generic name `read` included unrelated matches; the reader and lock behavior
were checked with exact snippets and direct source instead.
