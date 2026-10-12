# ravel-local-runner-k17

## Goal

Establish Ravel's independent local job runner and a green reusable Rust check
entry point before transferring the front and compiled policy worker.

## Context

Copy `crates/keyed-launch` from this workspace into the reused Ravel checkout.
The parent's complete runtime contract stays live on ravel-generic-pair-k18.
Read `docs/retired-project.md` and recheck its exact local/remote retired revision
before replacing active historical sources. Preserve that document and LICENSE.

## Done when

- The active Ravel tree is a standalone Rust workspace at development version
  0.0.0 with its own local keyed-launch crate and matching locked dependencies.
  The retired actor-daemon tree remains accessible through its named bookmark.
- Root Taskfile build/check tasks compile and verify the local runner without
  any cross-repository dependency. Preserve source MSRV and zero-clippy gates.
- Transfer the complete existing runner suite: channels, signal escalation,
  foreground/terminal restoration, cancellation, confinement, noninteractive
  descendants and injected wait-error recovery. Preserve its test-only fixtures.
- Cargo clears old/new session authority controls; directly invoked fixtures
  also scrub inherited controls before spawning. Verify with isolated dummy
  controls and verify the actual live exit channel remains absent.
- Current README and runner documentation describe exactly this executable
  boundary and its source provenance. Old daemon docs/state are retired history,
  rather than active requirements. Do not advertise a front that is not built.
- Run the root Taskfile check and build tasks, commit through jj in Ravel, and
  record the exact commit/validation here. Retire this child and seal the
  coordination change, leaving k18 and k16 live.

## Notes

Do not copy personal configuration/history, install a command, launch a model,
prepare/publish a release or modify the old installed front. No runner behavior
is redesigned; preserve the implementation and generalize caller-specific prose.

## Decisions (running log)

**Implementation plan.** Recheck the preserved revision; replace only the clean,
tracked old active tree while retaining retrieval documentation and LICENSE;
copy the local runner and lockfile into a 0.0.0 workspace; add cargo/test guards
and reusable Taskfile build/check tasks; update current README/runner provenance;
run the full runner suite and lint/format gates; seal Ravel and coordination jj
changes. Existing design decisions and process/terminal logic are retained.

## Validation and handoff

Ravel commit: `ee5e64b1f0e2993fdc81ad38cf3753173f87bd55`
(`ravel-local-runner-k17: establish Ravel's independent job runner`). Local
`main` names it; the next working-copy change is empty. This is an ordinary
descendant of the preserved revision. The local and origin retired bookmark
were rechecked at `035edfa30f3bf40eb870a525f49ac98a3b5abbb1` before source
replacement, and `docs/retired-project.md` remains byte-identical.

- `task check`: formatting and all-target Clippy passed; 60 tests passed, zero
  failed. Eleven ignored subprocess roles are driven by their parent tests.
- `task build`: passed with the committed lockfile.
- `rustup run 1.85 cargo check --locked --all-targets`: passed.
- The authority regression failed before the scrub was implemented, naming
  inherited `GROVE_LAUNCH_DIR`, and passed afterward. The compiled authority
  test binary also passed when invoked directly with isolated dummy controls.
- All 20 measured inputs (root manifests, lockfile, Taskfile, cargo guard,
  runner manifest and every Rust source) retained their before/after digests.
  Every retained registry package matches the source lockfile's version and
  checksum. Production source differences are documentation comments only.
- README/runner-documentation local links resolve. The tracked active tree
  contains only the intended standalone files. Former nested Elixir build
  products stay on disk and ignored; they are not part of the new commit.
- The actual inherited exit/signal paths remain absent after all checks.

The source graph provided Tier 2 discovery and current coverage for the copied
runner. Ravel's own graph still described the retired project, so the new-tree
checks used direct source/diff inspection and executable validation, not that
stale graph. Validation ran on the macOS host; other-target delivery belongs to
the later delivery leaf.

`ravel-generic-pair-k18` owns the remaining parent runtime acceptance contract,
including the front/worker, store boundary, current executable documentation and
fresh-prefix installation. `ravel-runtime-k15` and `ravel-cli-k3` remain open;
`ravel-history-migration-k16` still owns actual personal configuration/history.
