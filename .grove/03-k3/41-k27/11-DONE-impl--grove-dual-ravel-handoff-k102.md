# grove-dual-ravel-handoff-k102

## Goal

Close grove-dual-ravel-k27 with full checks against each explicit build. Hand
the stage record to common-loading-acceptance-k29 and discovery-rollout-k6.

## Context

- The Done when of the node brief and of `common-skill-loading-k3`.
- The running logs of the leaves before this one.
- Tap states A to D and the publication order of
  `docs/specs/coordinated-release.md`.

## Done when

- `bash scripts/check.sh` passes with the 1.1 build. It also passes with the
  2.0 build and its runtime selected explicitly. Each run uses its own HOME.
- `ravel:resolution`, `ravel:commands`, `ravel:policy`, `ravel:activation`,
  `ravel:fixtures` and core conformance pass for each build. The record names
  each run's explicit build.
- A stage record in Grove's evidence names the Grove commits of this stage. It
  also names each build commit, build identity and `bin/ravel` SHA-256.
- The record gives reproducible recipes for each build prefix, each HOME, each
  configuration and settings file and each local source.
- The record states the publication order for acceptance and rollout. k29
  measures the unpublished build. Rollout publishes `ravel@2` before compatible
  Grove. It repeats admission and installed checks against the published
  archives before Grove's release. Promotion comes last, in finish.
- Every Done when of the node brief is met by a recorded result, or it becomes
  precise repair work before this node closes.
- The briefs of k29 and k6 name the stage record.

## Notes

Review each earlier leaf's record before deciding which checks need another
run. A green total is not a per-item confirmation.

## Decisions (running log)

**Earlier records.** Every 2.0 run of k94 to k101 used the build of
ravel-local-sources-k90, whose `bin/ravel` was `151635d2…`. The 2.0 build is
now the glibc 2.17 repair of k103. Every earlier 2.0 result on macOS therefore
needs a new run. No earlier leaf ran `bash scripts/check.sh` with a 2.0 build.

**Builds.** On 2026-11-03 the 1.1 keg `bin/ravel` had SHA-256 `c52f990b…`.
Ravel `main` was `24a545236185`, and the bookmark `skill-directory-delivery`
was at the output commit `1ca4aecbeae1`, with an empty working copy above it.
`task install PREFIX=<scratch>/ravel-2.0` from that workspace gave a
`bin/ravel` with SHA-256 `8e10e311…` and no `libexec` worker. The executable
holds the identity `ef5857e9…` twice and the identity `9feb86b7…` of k90
nowhere. It reports `ravel 1.1.0`.

**Run environment.** Each run sets its own scratch HOME. It keeps
`CARGO_HOME`, `RUSTUP_HOME` and the directories of mise at their real paths,
because a toolchain is not a subject of the run. The 2.0 run puts a guard
`ravel` first on PATH, which exits 86 with `host-ravel-reached`. Runs are
sequential, because they share the Cargo target directory.

**Publication state.** On 2026-11-03 `git ls-remote` showed Ravel `main` at
`24a545236185` and no tag `v2.0.0`. `brew info linkuistics/taps/ravel@2`
found no formula. The tap is in state A of the coordinated release.

**Ported provenance.** `scripts/ravel-installed-smoke-rhai.sh` and
`assets/ravel-policy/examples/static.rhai` name `4f831feb` as their source.
Between `4f831feb` and `1ca4aecb`, Ravel changed only `crates/ravel/src/skills`
and three documents, so those provenance lines stay correct.

**Stale specification line.** The Test seams section of the delivery
specification said that the directory part on Linux stays unmeasured until the
floor tooling has a 2.0 lane. k103 delivered that lane and ran the confined
directory case on both Linux floors. A sweep of the tracked files for that
claim found this line only, and the same sweep found the line itself as its
positive control.

**Installed checks to repeat.** k100 and k101 ran the macOS formula test,
installed smoke, target smoke and release smoke with the archive of k90.
k103 ran the Linux target smoke, registration and floors with the current
archives. This leaf repeats the macOS installed checks and runs the whole
release smoke on all three targets, after the two gate runs.

**First runs.** Both runs used Grove `852b7ec1` with a subject digest of
`85bd9863…` before and after each run. The 1.1 run passed `check.sh` (20 of
20) and the six named tasks. The 2.0 run passed the six named tasks and
`task ravel:directory` (seven directory cases and the native case). Its
`check.sh` failed one check of 20: release preparation.

**Defect found by 2.0: release preparation.** `scripts/release-prepare.test.sh`
runs a real `grove run release-notes` through `GROVE_TEST_RAVEL`. It always
wrote a 1.1 `policy.ts`, so the 2.0 build refused with
`config_format_retired`. The test now reads `GROVE_TEST_RAVEL_RUNTIME` as
`testing/ravel.rs` does. It writes `config.rhai` with `SCHEMA_VERSION` 4 for
2.0 and `policy.ts` for 1.1 or an unset runtime. It refuses a build whose
layout does not match the runtime, and a runtime other than `1.1` or `2.0`.
The failed gate run is the control of the old script. With the fix the test
passed on the 2.0 build, on the 1.1 build and with the runtime unset. A 2.0
build named `1.1`, a 1.1 build named `2.0` and the runtime `3.0` each stopped
with a remedy. This was a test defect. No caller code changed.

**Per-item comparison.** For each log, the sets of passed test names on 1.1
and 2.0 are identical: 1,371 lines in `check.sh`, 796 in `ravel:activation`,
and every name of the other tasks. No log reports a case as not run. On 2.0,
`ravel:policy` skipped the TypeScript typecheck and compiler test, as the
Taskfile states. The first comparison used a pattern that missed the padded
counter of nextest and reported false differences. That failure showed the
comparison can fail. The corrected pattern found 18 differences between two
different task logs, as a second control.

**Stage record.** `docs/grove-dual-ravel-stage-record.md` holds the record. It
links to the usage guide, the delivery specification and the target inputs
for the rules they own, and it states only this stage's builds, recipes,
results and order. The delivery specification's Test seams section now states
the Linux 2.0 lane and links to the record. A pre-existing paragraph of seven
sentences in its Caller assets section is split in two. The briefs of k29, k6
and common-skill-loading-k3 name the record.

**Final runs.** All edits except the record's results were made before the
final runs. The final runs are three lanes in sequence: the 1.1 gate and
tasks, the 2.0 gate and tasks with `ravel:directory`, and an installed lane.
The installed lane runs the target controls, the target smoke of all
targets, the whole release smoke, the formula test, the installed smoke, the
upgrade check and the legacy-driver rehearsal with the 2.0 prefix as `ALSO`.
The 22.1.0 keg of the rehearsal comes from the published archive, SHA-256
`b45a030c…`, which matches the recorded value.

**Final results.** Each lane ran on Grove `852b7ec1` plus this leaf's working
copy, with the subject digest `0f9c65ce…` before and after every lane. The
1.1 keg stayed `c52f990b…` and the 2.0 front stayed `8e10e311…`.

- 1.1: `check.sh` passed 20 of 20, and the six named tasks passed.
- 2.0: `check.sh` passed 20 of 20, and the six named tasks passed.
  `ravel:directory` passed seven directory cases and the native case.
- Per item, the passed test names of each log are identical between the
  runs: 1,371 in `check.sh` and 796 in `ravel:activation`. Release
  preparation passed on both.
- Installed: the target controls passed (34 unit cases and the real-layout
  controls of both lanes). The target smoke passed six combinations of lane
  and target. The release smoke passed all three targets with both lanes,
  including the native and QEMU floors and the confined directory case on
  each Linux target. Grove's macOS archive was `35da914e…`. The installed
  smoke passed both lanes. The upgrade check left 286 entries unchanged. The
  rehearsal left the 1.1 front, its ending refusal and both inventories
  unchanged.

**Homebrew trust.** The first formula test in the installed lane failed with
`UntrustedTapError`, because Homebrew reads its trust store from
`$HOME/.homebrew/trust.json`. The `brew` wrapper resets
`HOMEBREW_USER_CONFIG_HOME` to that path, so the variable alone failed too.
A copy of the owner's `trust.json`, mode 600, in the scratch HOME let the test
pass with both builds on the same archive. The owner's file kept SHA-256
`791277e4…`. Like the toolchain, the trust store is not a subject of the run.

**Node close.** Retiring this leaf leaves grove-dual-ravel-k27 with no live
leaf. The "Node outcomes" table of the stage record maps each Done when of its
brief to a recorded result, and no outcome needs repair work. The brief of
common-skill-loading-k3 now names the record. No ADR changes: the runtime
selection of one test seam is not a design decision. No review names k102 or
k27, so no Creator line is written. common-skill-loading-k3 stays open,
because common-loading-acceptance-k29 is live.

**Review.** The work changes one test script and documents. No caller code
changed, and three observed mismatch controls and both builds cover the
script. The work earns no `review-impl` leaf, and this session spent no
in-session reviewer.

**Final gate.** After the record's results were written, `check.sh` with the
1.1 keg passed 20 of 20 on the finished tree, in a scratch HOME. The subject
digest was `e90ff8eb…` before and after the run.
