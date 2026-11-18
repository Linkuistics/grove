# cargo-build-layout-k5

## Goal

Make `bash scripts/check.sh` pass again with the installed Rust toolchain.
Every leaf of this grove has that condition in its *Done when*, and today it
fails for a reason outside the grove's design.

## Context

- `glyph-rows-k2` found the failure. The same failures occur on the base
  revision `89eba142`, before any change of this grove. So the cause is the
  environment, not the viewer work.
- The installed toolchain is Cargo 1.100.0 (2026-11-05). No toolchain file pins
  a version. Its test executables are in
  `target/debug/build/<package>/<hash>/out/`, not in `target/debug/deps/`.
- Two failure messages result. 38 tests fail in all, and each one has one of the
  two messages:
  - `testing/support.rs`, function `workspace_binary`, takes the profile
    directory as two levels above the test executable. With the new layout it
    looks in `target/debug/build/<package>/<hash>/`, and it reports
    `grove-llm was not built`.
  - Tests in `crates/grove-llm/tests/` call `assert_cmd` for the `grove`
    binary, which belongs to another package. They report
    `CARGO_BIN_EXE_grove is unset`.
- Both the `cargo nextest` check and the *Ravel caller assets and provenance*
  check fail for these reasons. The other 16 checks pass.
- About 28 test files look up a binary. Run
  `grep -rn -E 'cargo_bin\(|CARGO_BIN_EXE|workspace_binary\(' crates testing`.

## Done when

- `bash scripts/check.sh` passes on the tree that includes `glyph-rows-k2`.
- The chosen remedy works for whoever runs the gate. Either the repository pins
  a toolchain, or the tests find the binaries in both layouts.

## Notes

- Choose between pinning the toolchain and adapting the binary lookup. The
  machine owner's environment rules govern a pin. Read the Cargo release notes
  for 1.100 before you depend on the layout. If the choice needs the owner,
  ask.

## Decisions (running log)

- **Adapt the lookup, do not pin the toolchain.** The Cargo 1.100 changelog
  lists the new layout under *Changed* (rust-lang/cargo#17354). It also says
  that the layout "is not a stable interface and is subject to change", and
  that tools must use Cargo's JSON messages to find artifacts. A pin would
  only postpone the break, and it would make each contributor install an old
  toolchain. The lookup therefore must not depend on where Cargo puts the test
  executable.
- **The lookup has two stages.** First, the helper searches each ancestor
  directory of the test executable for the uplifted binary. Both layouts put
  it in the profile directory, which is an ancestor of the test executable.
  This stage needs no subprocess, and it keeps the profile of the test run.
  Second, if no ancestor holds the binary, the helper builds the package with
  `--message-format=json` and reads the `executable` path. This stage covers a
  binary that is not built yet, and a `build.build-dir` outside the target
  directory. A lookup with no work to do measured 0.17 seconds.
- **The `grove-llm` tests use the shared helper for the `grove` binary.**
  `assert_cmd` 2.2.2 knows a binary of another package only through the old
  `deps` layout, so those calls change to `support::grove_bin()`.
