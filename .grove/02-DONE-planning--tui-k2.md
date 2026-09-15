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

## Decisions (running log)

The human chose to keep the independently useful stages as ordered leaves in
this grove, overriding the planning skill's separate-grove default. The stages
are a runnable tree/file browser, formatted Markdown, then automatic refresh
with identity and reading-position preservation. Each stage carries its own
tests, usage documentation and runnable command.

The viewer will be a `grove-tui` library behind `grove view [WORKTREE]` in the
existing human binary. Bare `grove` keeps its loop behavior. This keeps terminal
dependencies out of `grove-loop` and `grove-llm`, and keeps the binary thin.
Putting widgets in the binary would lose the agreed application test seam;
putting them in `grove-loop` would make terminal dependencies part of the loop's
surface. A third installed binary would add delivery work without a second
application consumer. The viewer resolves its directory directly and needs
neither a driver lease, session configuration nor a jj subprocess.

The existing reader blocks and can print a contention diagnostic. A quiet
try-read operation must share its parser, snapshot construction and containing-
directory lock, returning busy without a snapshot when acquisition would block.
The first runnable browser owns this small library extension and its caller;
there is no standalone framework or reader-only implementation leaf. An
unlocked probe followed by the blocking reader has a race and is insufficient.

The design outline belongs in the root brief: the requirements and test seams
are already agreed, so no new agreement-point spec is needed. Implementation
will reconcile the current architecture and usage documents as behavior lands.

Observation will use a 500 ms polling deadline and selected-byte comparison,
with scripted refresh entering the same application operation. A root lifetime
is observed filesystem identity, retained through an open root descriptor, not
the private driver epoch. This supports replacement between polls and prevents
keys from a new tree inheriting old reading state. The descriptor holds no tree
access lock. Duplicate keys are a visible invalid-view condition rather than a
first-match selection rule.

Published source archives were inspected for Ratatui 0.29.0, Crossterm 0.28.1
and pulldown-cmark 0.13.0: the backend versions match, the renderer has a test
backend, input supports timed polling, and the parser supplies table events and
source ranges. Their declared Rust floors are 1.74.0, 1.63.0 and 1.71.1. These
are candidates compatible with the workspace's 1.85 floor at the direct-package
level; each dependency-changing increment must verify the resolved lockfile on
1.85. No filesystem watcher, async runtime or syntax-highlighting dependency is
needed for the agreed behavior.

The lock extension and lifetime/anchor reconciliation cross existing interfaces
and are load-bearing for all three increments. A fresh `review-planning` leaf
will contest the completed outline and slices before implementation. No
in-session reviewer is used because that scheduled leaf owns the same review.

Source grounding used Tier 2, project
`Users-antony-Development-grove.create-tui-for-monitoring-and-viewing-grove-dir`,
generation 2026-09-15T07:00:01Z. The relevant symbol searches and outbound traces
were fully paginated; reader behavior was checked with exact snippets and
direct source. Generic call-name matches in the CLI trace included unrelated
functions, so CLI routing claims use the exact source. Coverage reported no
recorded gaps and matching metadata for the cited Rust files. Documentation is
excluded from this fast index; documents and changed/new task files were read
directly. This is task-directed evidence, not a claim of index completeness.
