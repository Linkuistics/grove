# tree-viewer-k3

## Goal

Deliver a basic runnable, permanently read-only plain-text browser through
`grove view [WORKTREE]`: navigate the tree, read tasks/briefs, and manually
refresh. This is the first working slice of the root's final contract.

## Context

- Add the `grove-tui` library with the root's application seam and dependencies.
  `crates/grove/src/cli.rs` dispatches view before jj, driver lease or launch
  configuration resolution.
- Use the existing blocking grove-loop Reading/Tree API and typed snapshot.
  Reuse canonical path composition; expose only the existing helper if needed.
  `responsive-viewer-k8` owns quiet try-read. Copy display data/selected bytes
  and release the guard before layout, input waits and idle time.
- `docs/walkthroughs/overview/walkthrough.toml` reconstructs the human crate's
  manifest/main/CLI. Carry affected pages and ranges with source changes. If
  exposing the path helper changes grove-loop, update its walkthrough too.

## Done when

- `grove view` observes `./.grove`; its optional argument is the containing
  directory. Help states there is no upward search and gives both examples.
  Temporary non-jj directories and concurrent viewers work. Bare grove keeps
  its lifecycle and no selector arguments; assert exactly `{view}` as the
  subcommand set and preserve help/version/description coverage.
- A selectable root and ordered nested branch/task rows include every outcome
  and open-token kind. Use the root's derived branch labels/counts, including
  all-done, all-abandoned, mixed terminal and empty subtrees. Initially select
  root and expand branches; Up/Down or j/k selects rows and Enter toggles a
  branch. Selection
  opens the corresponding file/brief as inert plain text. PageUp/PageDown
  scroll the file so long documents remain readable. q/Ctrl-c exit.
- `r` reloads tree/content; selection may reset to root. Missing/malformed trees
  and selected-file errors have visible states and recover on manual refresh.
  A retained last-good tree is visibly stale after a failed reload. No repair or
  scaffolding occurs. Usage states the temporary blocking-read/diagnostic limit
  on startup, selection and reload; this slice does not claim quit remains
  responsive while waiting for a writer.
- Non-TTY input/output fails before terminal setup. Basic terminal ownership
  restores modes, alternate screen and cursor on ordinary quit and returned
  errors, with fatal messages afterward. An actual-terminal smoke demonstrates
  navigation, file scrolling, refresh and q/Ctrl-c restoration. Full fault and
  signal evidence belongs to `viewer-interaction-k9`.
- Application-seam tests use real temporary trees for nested navigation,
  collapse/expand, root/node brief selection, labels, safe control characters,
  empty documents and long-file reading. Rendering is safe at zero/small sizes.
  Compare names/bytes before and after interactions; viewer state is memory-only.
- README, docs/USAGE.md and docs/specs/user-guide-coverage.md describe this
  subset; G4 retains the bare lifecycle's lack of launch-policy selectors.
  Reconcile docs/specs/module-decomposition.md, docs/ARCHITECTURE.md and
  CONTEXT-MAP.md for the viewer crate, including touched manifests' obsolete
  claim that only grove-loop can be domain-bound.
- The library inherits workspace version, Rust floor and lints, and sets
  `[package.metadata.release] release = false`. Reconcile docs/RELEASING.md and
  release.toml's shipped-package inventory. Check scripts/release-build.sh's
  two-binary delivery: building/installing grove includes the viewer transitively.
  grove-loop and grove-llm acquire no terminal dependencies.

## Notes

Full pane focus, arrow/hjkl expansion, Home/End, line/horizontal scrolling,
key help and the too-small/resize state arrive in `viewer-interaction-k9`.
Markdown and live observation follow. Keep this slice usable without absorbing
those later leaves to satisfy the root prematurely.

Update CHANGELOG.md under Unreleased. Run focused viewer/CLI and changed loop
tests, affected books' final validation (including ownership/fragments and any
changed corpus assertions), `cargo test --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`,
`rustup run 1.85 cargo check --locked --workspace --all-targets`, and a real
grove executable build/smoke. Record commands and results here.
