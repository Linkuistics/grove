# live-observation-k12

## Goal

Deliver a runnable live monitor with root lifetime and permanent-key navigation.

## Done when

- Rescan every 500 ms through the application's existing tick seam, coalescing
  deadlines; selected bytes are reread and errors recover automatically.
- Retain selection, expansion, focus and unchanged-content reading positions by
  key; reveal moved selection, support decomposition and retirement, and fall
  back to the nearest surviving ancestor when selection disappears.
- Hold a root descriptor without a lock, clear state on absence/replacement,
  reject duplicate keys and inconsistent captures, and retain stale/file errors.
- Application tests cover these transitions and no writes; focused tests,
  formatting, Clippy and a real executable build pass. Update usage, architecture
  and changelog with the temporary content-edit anchor limitation.

## Notes

Implementation plan: first add failing application tests in browser.rs; then
replace path identity and the two capture paths in observation.rs/lib.rs with
one keyed capture and root identity check. Keep terminal lifetime unchanged:
its existing loop already feeds tick and bounds input waits. Verify fixtures,
update documentation, retire this child and seal its change.

## Decisions (running log)

The existing refresh discards all selection/expansion and saved positions use
paths; Markdown anchors are byte offsets without edit reconciliation. This is
the concrete seam for the parent leaf's two-child sizing note. live-reading-k13
owns edit reconciliation and the full real-clock recovery/delivery matrix.

Use the approved root design directly. The graph generation 2026-09-15T07:00:01Z
does not track grove-tui sources; direct source reads provide the evidence here.

Observation uses one coalesced 500 ms deadline for explicit, selection and timed
reads. Two bounded captures compare canonical rows and selected bytes; each
releases its guard before the next acquisition. The root descriptor is checked
around that operation. This detects observed inconsistency without promising an
atomic snapshot against non-cooperating editors.

The narrow doubt review found two actionable root-opening defects: FIFO opens
could block, and opening an unreadable replacement failed before clearing the
old lifetime. Both were reproduced in application tests before repair. Root
opening now uses the existing rustix dependency's DIRECTORY/NONBLOCK/CLOEXEC
flags, and metadata compares identity before an open can fail. The release
archive for rustix 0.38.44 verified the API; the source citation is beside the
call. Executable regression tests conclusively cover these fixes; no second
reviewer is commissioned.

## Verification

File errors render as separately wrapped diagnostic text: a regression test
exposed that a long path hid the permission-denied reason. The diagnostic now
wraps without replacing the source document or its saved reading anchor.

- `cargo test -p grove-tui --test browser`: 24 passed, including actual fixture
  edits, identity/decomposition/fallback, same-size/restored-mtime bytes, busy
  recovery, FIFO/unreadable replacement, file-error anchor recovery and no-write
  comparisons. The six initial live tests failed against the missing behavior
  or unsafe root open before their implementations were corrected.
- `cargo test -p grove-tui --lib`: 2 passed, including the fault child covering
  partial terminal setup, input/draw errors and unwinding panic cleanup.
- `cargo test -p grove-tui -p grove --test browser --test view_terminal --test
  view_command`: passed before the final additional file-error test. All four
  actual-binary PTY tests passed: idle same-size/restored-mtime selected edits
  appeared in under one second, navigation and small/full resize worked, and
  quit/interrupt/termination restored attributes, alternate screen and cursor.
- `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets`: passed.
- `cargo build --locked -p grove --bin grove`: passed; the resulting
  `./target/debug/grove view --help` describes automatic refresh and the fixed
  observation path.
- `rustup run 1.85 cargo check --workspace --locked`: passed. This machine names
  its installed toolchain `1.85`, and its default cargo is not the rustup shim.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/overview --final --check all`: valid, 3 files, 224 resolved
  lines, zero deferred. SHA-256 checks before/after covered the three human
  crate sources/manifests and all overview Markdown plus its manifest; unchanged.
  Source line counts and fragment ownership ranges did not change.

The broader `cargo test -p grove-tui -p grove` run completed the human crate's
suites, then exposed a transient nonblocking flock failure in the browser test
fixture. FIFO creation now calls mkfifo directly instead of launching a process
alongside descriptor-sensitive tests; focused runs pass. The full workspace
suite and complete recovery/install matrix remain chartered to live-reading-k13.
