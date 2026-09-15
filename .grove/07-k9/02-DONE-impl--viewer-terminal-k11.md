# viewer-terminal-k11


## Goal
Harden terminal lifetime and prove cleanup with live PTY observations.



## Context
The parent brief owns the full exceptional-exit contract. The previous child
completes application interaction; extend the existing production terminal owner.

## Done when
Satisfy the parent's cleanup, actual-binary PTY, and test-only fault-child bullets
in full: partial setup, input/draw errors, actual unwinding panic, q, Ctrl-c,
SIGTERM, navigation/resize, termios equality and captured cursor/screen cleanup
before diagnostics, bounded waits and child reaping, separate closed-master case.
No shipped fault flag. Record commands and observations here. Update accurate
usage/changelog and affected books if their owned source changes. Run the
parent's required validation and close the parent against its full contract.

## Notes
The current Session restores on returned errors and Drop, but this is not yet
evidence for panic diagnostic ordering or handled termination. Timed event
polling must permit signal handling without doing terminal I/O in raw handlers.

## Decisions (running log)

- Keep terminal lifecycle private to grove-tui. Use the already locked
  signal-hook 0.3.18 flag registration for SIGINT/SIGTERM/SIGHUP; handlers only
  set an atomic flag, and the event loop checks at most every 100 ms.
- Restore before delegating to the previous panic hook, then catch and resume
  unwinding only to restore the process hook outside the panicking state.
  Returned errors leave the same lifetime scope before the caller prints them.
- Share a live openpty fixture between actual-binary integration tests and a
  test-only child in the viewer's unit-test executable. Keep fault selection
  entirely under cfg(test), with bounded waits and kill/reap on assertion failure.

- The closed-master regression exposed an infinite EOF read loop in Crossterm
  0.28.1's default Mio source. Enable that version's `use-dev-tty` input backend:
  its read loop breaks on zero bytes and propagates non-retryable read errors.
  Published release source was inspected locally in Cargo's registry, including
  both unix event sources and tty_fd's stdin fallback. This adds filedescriptor
  0.8.3; the full locked Rust 1.85 check will verify the dependency floor.
- PTY children use setsid without acquiring a controlling terminal: stdin/stdout
  still use the real slave, while macOS cannot revoke the observable slave when
  the session leader exits. Set CLOEXEC on both openpty descriptors. Seed the
  canonical baseline with PENDIN (macOS also sets it when restoring canonical
  mode), then compare saved flags, control characters and speeds exactly.
- The one in-session reviewer found two actionable gaps: incomplete escape
  sequences could still block the alternate input backend, and an unbounded
  PTY drain could prevent deadline checks. The new actual-binary ESC-[ then
  SIGTERM regression failed at its 10-second deadline before the fix. Input
  now uses nonblocking flags only while polling/reading, restoring them before
  drawing (stdin/stdout may share their open file description), and cleanup
  restores them too. Each PTY drain has a finite read budget. These fixes have
  direct executable coverage or a mechanical bound; no second reviewer needed.
- Strengthen input/draw injection through the actual driver: a failing input
  callback and a Crossterm backend writer failure now reach its error propagation;
  panic unwinds from that same driver. All fault controls stay in cfg(test).
  The review's observation that terminal attributes are sampled after exit is
  a visible measurement limit; ordered cleanup bytes establish diagnostic order,
  and live slave attributes separately establish the restoration result.

## Validation

- `cargo test --workspace`: passed, 1,196 tests in 89 result groups. SHA-256
  inventories of repository files (including hidden files, excluding .git, .jj,
  target and node_modules) matched before and after this final workspace run.
- `cargo test -p grove-tui -p grove --lib --test view_terminal --test view_command`:
  passed after a mechanical openpty pointer portability adjustment. Actual-binary
  PTYs verified selection, small/large resize, retained selection, help and
  q/Ctrl-c/SIGINT/SIGTERM/SIGHUP exits. Saved termios flags, control characters,
  speeds and descriptor flags matched; captured output contained screen/cursor
  restoration. Closed-master exit and partial-escape/SIGTERM exit were bounded
  and reaped. These observations were made on the macOS host.
- The test-only child exercised errors after raw/alternate/cursor setup, input
  callback errors, actual backend write errors and actual unwinding panic.
  Every live PTY restored its attributes, and both cleanup sequences appeared
  before the injected diagnostic. The pre-fix panic-order, SIGTERM and
  incomplete-escape tests failed for the intended defects.
- The existing 13 application tests passed in the workspace run: navigation,
  focus, help, Unicode scrolling, zero/small frames, resize and read-only tree
  preservation close the interaction half delivered by viewer-navigation-k10.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`, and
  `rustup run 1.85 cargo check --locked --workspace --all-targets`: passed.
- `./target/debug/book-check --repo . --book docs/walkthroughs/overview --final
  --check all`: valid, 3 files, 224 resolved lines, zero deferred lines. No
  human-crate manifest/source or reader/filesystem source changed, so their
  reconstruction fragments and corpus assertions require no edits.

## Parent closure

viewer-interaction-k9's full Done when is satisfied by its two children:
application interaction and read-only evidence from viewer-navigation-k10,
terminal lifetime and PTY evidence here, and current README/usage/changelog.
Promoted the input backend and descriptor-lifetime constraint into the root
brief for the following Markdown and automatic-observation increments. The
existing ADR set remains applicable; no filename, kind or tree-access decision
changed.

`cargo build -p grove --bin grove` also passed. The executable PTY suite above
is the terminal smoke, including actual resize and exceptional exits.
