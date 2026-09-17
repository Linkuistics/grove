# modular-examples-k9

## Goal

Stop shipping the flat override example while retaining the modular examples and
the installer's preservation contract.

## Context

The packaged inventory is `crates/grove/src/examples.rs`; its existing acceptance
tests are `example_installation.rs` and `config_examples.rs`. The overview book
reconstructs the inventory verbatim.

## Done when

- The packaged legacy sample and its successful-load case are removed.
- Installer inventory, usage list, example instructions and overview source
  fragment agree with the modular sample set.
- Existing installation tests verify that old installed samples survive and
  that fresh installation does not create the retired sample.
- `bash scripts/check.sh` passes.

## Notes

Parser removal and the general configuration documentation audit belong to the
next child. This increment is independently verifiable while both forms load.

## Decisions (running log)

Remove the flat sample rather than replacing it with another sample: existing
local selection and override examples already exercise modular local policy.
Extend the existing installer preservation assertion to include the previously
installed filename; do not add a new CLI acceptance suite.

Keep the walkthrough's established audience and chapter contract; update only
the exact inventory fragment and its line ownership. No new framework API or
resolver behavior is introduced. The bounded deletion and executable installer
assertions do not warrant an independent review leaf.

## Configuration-form inventory

Read every packaged KDL file: the personal sample declares commands, parameters,
profiles, bindings, routes and selection inside `config`; the codex-led,
claude-led and high-effort samples contain modular selections; the local-override
sample contains a modular selection and route parameter/unset patches. The flat
legacy override file is removed. The installer test retains its filename solely
as preservation evidence, with opaque contents that installation must not read
as policy. The broader repository forms audit remains modular-loader-k10's work.

## Verification

The existing installer test failed before the package deletion because a fresh
install created the retired sample. After the change, all seven example and
installation integration tests pass, as do the three installer unit tests.
The late-write test now addresses the last packaged item rather than a fixed
index; its failure and preservation assertions remain intact.

`bash scripts/check.sh` passes all eight principal checks, including the whole
workspace test suite and all six books' final source reconstruction. The
overview reconstructs 968 lines. SHA-256 digests of all 1,782 tracked non-task
files (source, manifests, fixtures, documentation and scripts) match before and
after that final run. No independent technical/editorial review is claimed.
