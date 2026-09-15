# markdown-viewer-k4

## Goal

Make the runnable browser's selected-file pane render formatted Markdown,
including headings, lists, tables and code blocks, while preserving navigation
and the permanent read-only contract.

## Context

- `tree-viewer-k3` delivers the basic application/command; `responsive-viewer-k8`
  adds quiet reads and `viewer-interaction-k9` completes interaction and terminal
  hardening. All share the root's application test seam.
- Extend `grove-tui` behind that interface; add its private Markdown/layout
  module and the pulldown-cmark dependency described by the root brief.
- The actual parser source exposes `Parser::new_ext`, `Options::ENABLE_TABLES`
  and `Parser::into_offset_iter`. Inspect the selected version before using
  these; source offsets are the input to reading-position preservation.

## Done when

- Selecting a task or node shows headings with hierarchy, styled emphasis and
  inline code, nested ordered/unordered lists, quotes, rules, code blocks and
  aligned tables. Prose wraps, indentation survives, and wide code/table content
  remains reachable with horizontal scrolling. Links/images/HTML and control
  characters have the inert behavior specified in the root brief.
- The layout retains the source range corresponding to rendered lines. Resizing
  a document preserves its reading anchor through reflow, with clamping when
  needed. The application retains scroll positions on ordinary selection
  revisits; content-change reconciliation is `live-viewer-k5`'s increment.
- The existing application seam reads real Markdown fixtures and checks visible
  text, layout and meaningful styles for all required constructs. Include Unicode,
  narrow viewports, long code/table rows, nested lists, an empty file, a partial
  Markdown edit and repeated blocks. Check that a unique visible reading marker
  stays visible across a resize, rather than asserting private scroll fields.
- The same filesystem manifest comparison proves that rendering, scrolling and
  resize preserve the observed contents and names. No link, image, code block
  or HTML element starts a subprocess or performs network access.
- Help/usage describe formatted reading and horizontal scrolling, with a
  short terminal smoke demonstration of a representative task and branch brief.
  The crate dependency graph still builds with the workspace's Rust floor.
- Update CHANGELOG.md under Unreleased. If CLI/manifest changes touch the human
  crate, update docs/walkthroughs/overview/; reader-facade changes also update
  docs/walkthroughs/grove-loop/ (and ordinal-fs-tree's book if its source changes).
  Carry affected manifests, ownership ranges, fragments and explanations with
  source changes, update changed corpus assertions and run final validation of
  every affected book. Preserve grove-tui's workspace version and release=false
  metadata; a private dependency adds no independently released package.

## Notes

Use parser events as the Markdown grammar and keep terminal layout private.
Do not replace it with regex stripping or make the production interface expose
individual widget state. Color themes, syntax highlighting and mouse navigation
are not needed to satisfy this increment.

Run the focused viewer/application tests and affected CLI/documentation tests,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`, and the
locked Rust 1.85 workspace check after the dependency change. Build and run the
real `grove view` command for the smoke check.

## Decisions (running log)

- Follow the approved root design: a private Markdown layout module consumes
  pulldown-cmark 0.13 events and source offsets, producing styled terminal lines
  with source ranges. The application continues to own navigation and scrolling.
- Execution order: first exercise formatting through real-file application tests;
  implement parser/layout and integrate it; then verify resize/revisit anchors,
  update usage and run the required checks and actual-binary terminal smoke.
- Keep saved positions by observed file path for ordinary revisits in this slice;
  permanent-key and root-lifetime reconciliation remain live-viewer-k5's scope.
  Store source-byte anchors plus horizontal columns, not width-dependent rows.
- The layout sanitizes parser output (including decoded entities), retains
  grapheme-level source ranges for unchanged text and event ranges for transformed
  text, and clips only code/table rows horizontally. Existing line-navigation
  fixtures now use fenced code where their line-preserving contract requires it.
- Doubt claim: source-range mapping keeps the reading location through width
  changes and ordinary revisits. The compiler cannot establish this; spend the
  leaf's single reviewer on transformed events, repeated text and anchor lookup.
- Reviewer reconciliation: both findings are valid/actionable. Wrapped inline
  code and generated destinations need an intra-event rendered offset in addition
  to the source byte; blank code rows need the specific newline range rather than
  the whole text event. Visible-marker application regressions exercise both
  fixes through revisit and resize, so no second review is needed for these fixes.
- The only dependency change is private to grove-tui plus Cargo.lock; no human
  crate, reader facade or book-reconstructed source changes. README, usage and
  architecture explain the delivered behavior. No walkthrough corpus changes.
- Validation passed: grove-tui's 17 application tests and 2 terminal tests;
  view_command, view_terminal, user_guide_coverage and reference_navigation;
  cargo fmt --all -- --check; cargo clippy --workspace --all-targets;
  rustup run 1.85 cargo check --locked --workspace --all-targets; cargo build -p grove.
  Final format/lint/viewer-test/build checks kept all 1,756 tracked-file digests
  unchanged while running (including sources, manifests, fixtures and docs).
- Actual-binary PTY smoke displayed ROOT_DEMO, a nested-list/table BRANCH_DEMO,
  and a fenced-code TASK_DEMO; horizontal scrolling reached TAIL_DEMO. Quit
  restored canonical input/echo and emitted leave-screen/show-cursor sequences;
  the fixture's file hashes were unchanged. Capture: /tmp/grove-markdown-smoke.ansi.
