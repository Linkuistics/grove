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

The human chose ordered leaves in this grove rather than separate groves for
the working increments. Build a runnable browser first, add formatted Markdown,
then make it refresh automatically. Each increment includes its own interface
tests, usable command and accurate usage documentation. Framework setup and
library changes belong with the first behavior that consumes them. A planning
review precedes implementation; the root's full Done when applies after all
five implementation increments below. The basic browser's temporary limits are
explicit in its leaf; the following two slices complete the manual browser
before Markdown and automatic observation land.

1. `tui-k6` reviews this outline and the leaf boundaries before they are built.
2. `tui-k7` integrates that review into this outline and the leaf boundaries.
3. `tree-viewer-k3` delivers a basic plain-text browser using the existing
   blocking reader, manual refresh and minimal navigation.
4. `responsive-viewer-k8` adds quiet try-read and responsive contention/retry.
5. `viewer-interaction-k9` completes navigation, resize/help and terminal
   hardening, with explicit restoration evidence.
6. `markdown-viewer-k4` makes the file pane a formatted Markdown reader.
7. `live-viewer-k5` adds automatic observation, state preservation and recovery.

## Design outline

### Ownership and launch

- Add `grove view [WORKTREE]` to the human binary. `WORKTREE` defaults to the
  current directory and means the directory containing `.grove/`; there is no
  ancestor search. Resolve the argument to an absolute path once and keep
  observing that location, including while absent. Help gives examples for the
  current and another working tree. Help explicitly says: from a subdirectory,
  view observes that subdirectory's `.grove/`; it never searches upward. Bare
  `grove` retains its workspace resolution and argument-less lifecycle;
  `view` selects an observation path, never a workstream or launch policy.
- A `grove-tui` library owns the application, terminal lifetime, display and
  observation. The CLI parses and dispatches before resolving a jj workspace,
  acquiring a driver lease or loading launch configuration. Viewing requires
  none of those; temporary directories and concurrent viewers work too.
- Keep one application seam: construct a viewer with a worktree path, deliver
  navigation/refresh actions, and render it into a terminal-sized frame. The
  production terminal and Ratatui `TestBackend` exercise this same application.
  Tests create real temporary trees; they do not reimplement the name grammar.
  Terminal input and refresh deadlines feed this seam rather than reaching
  widgets directly. Private modules may separate observation, Markdown layout
  and terminal lifecycle without adding public widget interfaces.
- The tree adapter uses Grove's existing typed snapshot, `TaskName`, `Parts`,
  `Outcome` and node-file ownership. Copy an owned display model and the
  selected file's bytes under a short shared guard, then drop it before layout,
  rendering, waiting or another acquisition. Names remain canonical; titles do
  not come from headings. Reuse Grove's path-composition helper through a narrow
  public reader affordance if needed; do not introduce another filename parser.
- Add a quiet try-read affordance through `grove-loop` and `ordinal-fs-tree`.
  It attempts the real shared lock without waiting, then uses the existing
  presence check, parser and snapshot construction under that same descriptor.
  Busy is distinct from vacant and from malformed/unreadable. The existing
  blocking APIs and their CLI diagnostics retain their behavior. A preliminary
  unlocked probe followed by a blocking read cannot satisfy this contract.
  No guard or tree reference escapes into long-lived application state.
- This try-read deliberately revises the no-try/no-timeout rule in
  `docs/ordinal-fs-tree/ARCHITECTURE.md`, the `fs/mod.rs` and `fs/lock.rs`
  comments, and `docs/ARCHITECTURE.md`'s Tree access lock section.
  `responsive-viewer-k8` records the replacement rule there when implementing
  it: one nonblocking observer read, Busy with no snapshot, and unchanged
  blocking reads/writes. Production acquisition stays in `task_tree.rs`;
  the viewer neither calls `flock` nor reaches `ordinal_fs_tree::fs` itself.
- Selection, expansion, focus and saved reading positions exist only in process
  memory. No state file, configuration write or cache is created anywhere;
  quitting discards all viewer state.

### Interaction

- Show a selectable root row for `_BRIEF.md`, with ordered task/node rows below
  it and a file pane beside it. Initially select the root and expand branches.
  Tasks display their handle, open-token kind and explicit `LIVE`, `DONE` or
  `ABANDONED` label. Nodes are labeled branches. Root and branch rows aggregate
  descendant leaves, including hidden descendants: `LIVE` if any live leaf;
  otherwise `DONE` if any done leaf; otherwise `ABANDONED` if any abandoned
  leaf; otherwise `EMPTY`. Show descendant outcome counts beside the aggregate,
  so mixed terminal subtrees still expose abandoned work. Empty nested branches
  contribute no leaves. This is display-only; nodes gain no stored outcome and
  Grove's picker/retirement semantics do not change.
- Tab switches pane focus. In the tree, Up/Down or j/k moves among visible
  rows; Right/l expands a node or enters its first child, Left/h collapses it
  or moves to its parent, and Enter/Space toggles a branch. Selection immediately
  loads the leaf file or node brief. Home/End selects the first/last visible row.
- In the file pane, Up/Down or j/k scrolls a line, PageUp/PageDown or Ctrl-u/
  Ctrl-d scrolls a page, Home/End moves to the beginning/end, and Left/Right
  scrolls wide code/table content. `r` requests refresh; `?` shows key help;
  Escape dismisses help. `q` and Ctrl-c exit from any pane.
- Keep the selected row visible and distinguish focus without relying only on
  color. Resize reflows prose and clamps scroll without changing selection.
  Below 60 columns or 10 rows, show a resize message while still accepting quit
  and refresh; retain state until the two panes fit again.
- Require interactive stdin/stdout before changing terminal modes; otherwise
  print an actionable error and exit unsuccessfully. Restore raw mode, alternate
  screen and cursor on quit, input/draw errors, handled termination signals and
  unwinding panic. Fatal errors print after restoration.
- `viewer-interaction-k9` proves ordinary exits and Ctrl-c/SIGTERM using the
  actual binary under a PTY, inspecting saved terminal attributes and captured
  leave-alternate-screen/show-cursor output. A test-only child uses the same
  production lifetime code and injects partial setup failures, input/draw
  errors and an actual unwinding panic. Verify cleanup rather than just hook
  registration; no fault switch ships in the product. A closed PTY is a separate
  bounded-exit case, not evidence of output delivered after its master closes.

### Markdown

- Parse with `pulldown-cmark`, including tables. Render headings with hierarchy,
  emphasis, nested ordered/unordered lists, block quotes, rules, inline code,
  fenced/indented code blocks and aligned tables. Prose wraps to pane width;
  code preserves indentation and tables preserve columns, with horizontal
  scrolling for overflow. Syntax highlighting is not required.
- Links show readable labels and destinations; images show alt text. HTML is
  inert text. Control characters cannot issue terminal commands. Nothing opens
  links, runs code, fetches images or invokes an editor.
- Retain parser source ranges in the layout so reading position can be anchored
  to source text instead of a screen-row number. A width change maps the anchor
  into the new layout. On an edit, keep the anchored unchanged source line/block
  where identifiable; if deleted, use the nearest surviving position and clamp.
  Duplicate anchors use source order/proximity deterministically. Save reading
  positions per item for revisits, bounded to items still present in this tree.

### Observation, identity and recovery

- The final increment rescans names and reads selected content every 500 ms,
  and on explicit refresh or selection. Compare selected bytes, not just mtime
  or size. Coalesce requests; do not build an event backlog. On ordinary local
  fixtures an external change is visible within one second. An unchanged
  observation must not reset selection, scroll, expansion or pane focus.
- Identity is the permanent key within one observed root lifetime, plus a
  separate root identity. Hold an open descriptor of `.grove/` and compare its
  filesystem identity with the current directory at observation time; retaining
  the descriptor prevents inode reuse from impersonating the previous root.
  That descriptor holds no tree access lock. An observed absence or replacement
  starts a new lifetime and clears item state, even if keys are reused. Replacing
  `_BRIEF.md` alone is a content edit, not a new root. Detect replacement even
  when deletion and recreation happen between two polls; recheck identity around
  capture and retry an inconsistent observation. Support the repo's existing
  macOS/Linux targets; do not infer a lifetime from driver epoch files.
- Within a lifetime, match keys across renumbering, slug changes and moves.
  A selected leaf becoming a node keeps its identity and opens the node brief;
  preserve its reading anchor wherever that content supports it. Retirement or
  abandonment leaves the selected task readable. Reject duplicate keys visibly
  instead of letting the snapshot's first-match lookup redirect selection.
- Preserve expansion by node key. If a selected item moves under a collapsed
  ancestor, expand its new ancestors to reveal it, preserving other choices.
  When an accepted snapshot lacks the selected key, select the nearest surviving
  old ancestor (otherwise root), show which item disappeared, and discard the
  removed item's saved state. A later reappearance is a fresh item. Explicitly
  choosing another item starts at its saved position or the top.
- Busy retains the last good view with a waiting indicator and retries on the
  next deadline. A malformed/unreadable tree retains a clearly labeled stale
  view and diagnostic, or an empty error view before any successful read.
  Missing/replaced roots clear the previous tree and show the observed path and
  waiting state until a valid new tree appears. Never repair or scaffold it.
- If only a selected file is unreadable or disappears during capture, keep the
  tree, show a file-specific error and retry; do not silently show another file
  or discard the last saved reading anchor. Non-cooperating external edits are
  not an atomic transaction: reject inconsistent captures and retry. Recover
  automatically once files/structure become readable and valid again.
- From `responsive-viewer-k8` until automatic refresh lands, the browser provides
  `r` and uses the same visible failure states; it retries a busy initial/manual
  read so contention
  cannot strand an otherwise idle UI. Its documentation calls it a manual
  refresh browser until the final increment lands.
- `tree-viewer-k3` alone uses the existing blocking reader and diagnostic;
  startup/selection/manual reload may wait for a writer. Its usage documents
  this temporary limit. `responsive-viewer-k8` removes it before the full
  interaction contract and Markdown are added.

### Dependencies and delivery

Use Ratatui 0.29 with only its Crossterm backend, matching Crossterm 0.28.1;
add pulldown-cmark 0.13 with default features disabled in the Markdown increment.
Keep these private to the viewer crate. The published crate sources establish
the backend match, `TestBackend`, timed event polling, table events and parser
source offsets; their manifests declare Rust floors below the workspace's 1.85.
The resolved lockfile still needs a Rust 1.85 build, since transitive dependency
versions can raise that floor. Preserve the floor or document and resolve a real
conflict rather than upgrading it implicitly.

The completed manual viewer uses Crossterm's `use-dev-tty` input backend to
avoid its default backend's EOF loop. Its terminal owner makes stdin nonblocking
only during input polling/reading, restores descriptor flags before drawing,
and caps input waits at 100 ms for handled termination. Preserve this lifetime
boundary when adding Markdown and observation. Live PTY cleanup tests cover the
shipped binary and a separate cfg(test) fault child.

Published source references: [Ratatui 0.29.0](https://docs.rs/crate/ratatui/0.29.0/source/),
[Crossterm 0.28.1](https://docs.rs/crate/crossterm/0.28.1/source/), and
[pulldown-cmark 0.13.0](https://docs.rs/crate/pulldown-cmark/0.13.0/source/).
Sources were inspected from the corresponding static.crates.io release archives.

The viewer ships inside the existing `grove` executable. Its library inherits
workspace version, Rust floor and lint settings and has no independent release.
Set `[package.metadata.release] release = false` in the new crate, retaining
workspace version inheritance. Reconcile the shipped-package inventory in
`docs/RELEASING.md` and `release.toml`; book-validation remains a separately
versioned authoring tool outside that shipped package count.
Update command-surface assertions, architecture/module descriptions, README,
usage coverage and release documentation with the increment that changes them.
Check the existing two-binary archive/install path; no third binary is needed.

Every implementation slice updates `CHANGELOG.md` under `## Unreleased` for its
delivered behavior. Book-reconstructed source changes carry affected manifests,
ownership ranges, fragments and explanations in the same commit, with final
validation of each affected book (`docs/specs/walkthrough-books.md`). The
overview book owns the human crate; the grove-loop book owns the reader facade;
the ordinal-fs-tree book owns its fs implementation. Update affected corpus
count assertions from the new validated corpus, preserving byte-exact checks.
New `grove-tui` sources do not by themselves require a new walkthrough book.

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

## Delivery evidence

The complete live monitor is covered through the public Viewer application seam
and the shipped binary's PTY tests. Real-clock filesystem fixtures check recovery
within one second and recursive names/bytes preservation; the binary also permits
Grove retirement while open. Source-edit anchors preserve current and saved
reading positions. The release candidate's extracted two-binary installation
passes the same terminal suite. `live-delivery-k15` records the final workspace,
Rust 1.85, formatting/lint, six-book and package evidence, completing
live-reading-k13 and live-viewer-k5 against this brief's full contract.
