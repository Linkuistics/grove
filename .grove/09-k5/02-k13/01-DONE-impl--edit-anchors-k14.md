# edit-anchors-k14

## Goal

Keep current and saved reading positions attached to surviving source text
across edits, including recovery from selected-file failures.

## Context

The existing Viewer seam and Markdown source ranges supply selection and reflow.
No new public API or dependency is needed.

## Done when

- Insertions above a visible marker preserve it, both immediately and on revisit.
- Duplicate lines resolve deterministically using surrounding source and proximity;
  deleted anchors use the nearest surviving source position and clamp.
- File errors preserve the source associated with the saved anchor until recovery.
- Application tests cover edits, duplicate passages, deletions, empty replacement,
  transformed Markdown, resize, per-item isolation and no writes.
- Focused tests, formatting and crate lint checks pass; README, usage,
  architecture and Unreleased describe the delivered behavior.

## Notes

Full production-clock recovery and workspace/package/PTY delivery evidence belongs
to live-delivery-k15; the parent remains open until that evidence is established.

## Decisions (running log)

The original leaf includes two independently verifiable deliverables. Decompose
at the source-edit mapping versus complete observation/delivery evidence seam;
this session implements only the first child.

Keep each saved anchor with shared immutable source text and horizontal offset.
On successful capture, map from that item's source, never the previously selected
item's source. Exact source-line matches use nearby context, then source proximity
and order for ties; absent lines fall back to the nearest surviving old line.
The current document and its saved position share source storage. No filesystem
read is added for unselected items; map those lazily when revisited.

Implementation plan: first add application regressions that fail with raw byte
offsets; implement the private mapping and saved-source ownership; run focused
tests and edge cases; update delivered documentation and validate before retiring.

The single in-session adversarial review supplied two actionable counterexamples:
a deleted duplicate reused a prefix occurrence; long repeated blocks lost their
shift when both document ends changed. Both reproduce through application tests.
Reserve prefix/suffix occurrences and replace fixed neighbour context with full
contiguous match lengths computed by linear Z windows. The tests conclusively
exercise these fixes, so no second review is required. The reviewer found no
additional concrete issue in saved-source ownership or restoration.

## Validation

- Initial insertion/revisit regression failed with the original byte-only state.
  Both review counterexamples subsequently failed through the application seam
  before their fixes; all now pass.
- `cargo test -p grove-tui`: 31 browser tests and 2 terminal fault tests passed;
  doc tests passed (none defined). New edit cases compare recursive names/bytes
  with the expected external mutation. Permission-denial assertions are
  conditional on the fixture actually denying reads, as in the existing suite.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy -p grove-tui --all-targets -- -D warnings`: passed.
- `rustup run 1.85 cargo check -p grove-tui --all-targets --locked`: passed.
  This machine's cargo is not a rustup shim, and its toolchain is named `1.85`,
  so `cargo +1.85.0` and `rustup run 1.85.0` were unavailable spellings.
- SHA-256 before/after comparison was identical for Cargo.toml, Cargo.lock,
  grove-tui's manifest, all its src/*.rs, browser.rs, tests/support/pty.rs,
  README, usage, architecture and changelog across the final checks.

Only grove-tui source changed; no book-owned source or dependency changed.
The selected child's acceptance clauses are covered by the application tests
and updated docs. live-delivery-k15 still owns the full parent/root checks;
neither live-reading-k13 nor live-viewer-k5 closes in this commit.
