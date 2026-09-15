# markdown-viewer-k4

## Goal

Make the runnable browser's selected-file pane render formatted Markdown,
including headings, lists, tables and code blocks, while preserving navigation
and the permanent read-only contract.

## Context

- `tree-viewer-k3` delivers the application interface, tree adapter, terminal
  lifetime, command and main test seam.
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

## Notes

Use parser events as the Markdown grammar and keep terminal layout private.
Do not replace it with regex stripping or make the production interface expose
individual widget state. Color themes, syntax highlighting and mouse navigation
are not needed to satisfy this increment.

Run the focused viewer/application tests and affected CLI/documentation tests,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`, and the
locked Rust 1.85 workspace check after the dependency change. Build and run the
real `grove view` command for the smoke check.
