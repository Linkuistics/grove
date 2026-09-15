# grove.create-tui-for-monitoring-and-viewing-grove-dir — brief

## Goal

Provide a permanently read-only terminal UI for monitoring and viewing a
`.grove/` task tree. Reflect changes to the tree automatically.

## Done when

- The TUI shows the task tree and updates when its files or structure change.
- Every product interaction preserves the observed tree's contents and names.
- Selecting a task displays its Markdown file; selecting a branch displays its
  node brief. Ancestor briefs are reachable through tree navigation.
- Automatic refresh preserves the selected work item and reading position;
  renumbering or moving the item does not redirect the reader to another task.
- The file pane renders formatted Markdown, including headings, lists, tables
  and code blocks.
- The tree includes all tasks, clearly labels live, completed and abandoned
  work, and supports collapsing and expanding branches.

## Decomposition

- `plan-k1` established the behavioral scope and agreed the test seams with the
  human.
- `tui-k2` turns this contract into a compact design outline and small,
  independently demonstrable implementation increments.

## Test seams

Exercise the viewer through its application interface: give it a temporary
`.grove/`, scripted navigation input and a terminal-sized viewport, then inspect
the displayed tree and file pane. Reuse Grove's existing tree reader rather
than duplicating its grammar for tests or production. Prefer this one main seam
to tests that depend on individual widgets or private state.

Cover the observable contract:

- All task outcomes and nested branches are visible and navigable; selecting a
  task or branch shows the corresponding formatted Markdown, including headings,
  lists, tables and code blocks.
- External additions, edits, retirement, abandonment, moves and renumbering
  appear automatically. Changes to the selected file refresh its content.
- Refresh preserves selection by permanent key, reading position and the user's
  expansion choices wherever those still apply. A retired or abandoned selected
  task remains readable.
- Comparing filesystem contents and names against the expected external edits
  demonstrates that the viewer itself performs no writes.

Exercise actual filesystem change detection through the same viewer interface,
as well as scripted change delivery, so the tests establish that external writes
reach the screen. A terminal smoke check covers keyboard navigation, resize and
terminal restoration on exit. Keep read guards short enough for Grove's own
tree mutations to proceed while the viewer is open.

## Pointers

- `CONTEXT.md`: task-tree scheme and tree access lock.
- `docs/adr/task-names-are-canonical.md`: the authoritative filename grammar.
- `docs/adr/a-kind-is-an-open-token.md`: session kinds are open tokens.

## Notes

The human confirmed the behavioral requirements and test seams above in
`plan-k1`. That leaf's running decision log records the individual answers.

Planning must specify ordinary UI details and failure behavior coherently:
launching the viewer against one tree, navigation and scrolling, refresh timing,
handling a selected item that disappears, unreadable or temporarily malformed
trees, and recovery when the tree returns. These are implementation/design work
under the agreed contract, not grounds to repeat the requirements interview.
