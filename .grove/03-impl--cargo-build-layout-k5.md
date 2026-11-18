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
