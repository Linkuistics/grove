# grove-delivery-contraction-k49


## Goal
Remove bundled dispatch delivery and ship only Grove executables/caller assets,
with independently installed Ravel as Homebrew's runtime dependency.

## Context

Use k47's detached compiler and k48's validated separate target layouts. This
wide refactor must land atomically across workspace/lock, gate/tasks, archive
manifest/build/assertions, formula and installed smoke consumers. Read k42/k46's
test/asset inventory before removing the old package.

## Done when

- Remove crates/harness-dispatch and every obsolete build/typecheck/probe/compatibility recipe only after every applicable caller fixture/asset has a preserved new owner. Grove's own keyed-launch remains local; no shared Ravel library or bundled front/worker/notices ships.
- Exact archives contain Grove executables and assets/ravel-policy relative imports in an installed readable layout; formula depends on linkuistics/taps/ravel, tests caller assets and public CLI integration without host runtime or equal-version assumptions.
- Normal task check uses independent installed Ravel and passes; release pipeline/manifest tests and focused installed lifecycle/standalone paths pass with separate prefixes/different versions. k51 owns final rebuilt all-target acceptance.
- Update affected source-pinned walkthroughs and gate-facing install/release instructions. Enumerate old-name/dependency tokens across the actual affected surfaces with observed positive/cross-tree controls, classifying necessary historical/authority/compatibility remnants. Keep all live-session guards.

## Notes

## Decisions (running log)

- Execute the settled atomic contraction inline. First change the executable archive/formula fixtures and observe their refusal of the old layout; then switch workspace/lock, build, tasks/gate and installed consumers together. Ship the complete readable `assets/ravel-policy` tree as `libexec/ravel-policy`, preserving relative imports and developer fixtures. Ravel supplies its own SDK/runtime/notices from a separate prefix and version.
- Preserve caller behavior through `ravel_policy_assets`, creator lifecycle, shared command/result and actual caller tests. Retire old inspect/init/store/probe/package tests with the removed implementation; generic SDK/choice/process tests belong to independent Ravel. Keep Cargo and helper legacy authority exclusions and the installed original 22.1.0 pair unchanged.

- The new native consumer and real Homebrew install/test pass through independent Ravel after correcting fixture imports/staged inputs/output names. The single narrow reviewer found no issue in host admission, separate dependency staging or combined QEMU ELF coverage. Linux floor execution exposes the pre-existing CentOS image's missing `/usr/bin/bwrap`; keep its fail-closed refusal and externalize environment preparation as linux-standalone-floor-k52 before k51. This increment's focused installed acceptance is macOS; no complete Linux/all-target acceptance is claimed.

- Final frozen acceptance meets k49's atomic delivery/focused-installed contract: the default PATH-resolved task check, native archive consumers, actual private Homebrew install/test, combined Linux ELF controls and explicit installed public command/result fixtures pass. No remaining dependency pin changed; 345 retired old-package case identities are the only prior gate cases removed. The known Linux standalone environment gap is k52's, and final rebuilt target acceptance remains k51's.

## Implementation plan

1. Update `scripts/release.test.sh` to require the two Grove fronts, the exact caller asset tree and the Ravel formula dependency; retain omission/extra/mode/link/AppleDouble and publication-order controls. Observe RED.
2. Contract `Cargo.toml`/lock, `Taskfile.yml`, `scripts/check.sh`, release build/common/doctor and formula together; remove the old package only after recording the caller fixture ownership matrix.
3. Adapt release smoke to admit k48's pinned target archive before launch, stage it separately in native/container/QEMU routes, check every ELF from both prefixes, and exercise installed lifecycle/standalone paths through public Ravel with fake agents and no host runtime.
4. Reconcile gate-facing docs and affected overview book prose; enumerate/classify retained tokens with observed controls, then freeze subjects and run focused archive/installed checks and one full `task check`. k51 owns final rebuilt all-target acceptance.
5. Record measured evidence and handoff, retire k49, seal its single jj change, and invoke the original `harness-dispatch exit` as the last action.

## Verification and handoff

`env -u GROVE_TEST_RAVEL PATH=/Users/antony/Development/Ravel/target/tools:$PATH
task check` exits 0 and resolves independent installed Ravel from PATH. All 14
principal checks pass: 1,378 workspace nextest cases, nine doctests, strict
public-SDK/compiler controls, archive/publication/refusal tests, 21 conformance
mutations and six final/all books with zero deferred lines. Explicit
`task ravel:fixtures ravel:commands RAVEL=/opt/homebrew/bin/ravel` passes 19
executions, including all six named ignored installed cases. The combined
1,393 unique identities pass; every 1,387 retained prior gate identity agrees
individually. The only 345 removed identities belong to the retired package.

All three unreleased archives build with the new exact manifest. Final measured
native direct/symlink lifecycle/standalone/import cases, real Homebrew
install/test and both Linux combined ELF missing/escape controls pass. Independent
Ravel-only smoke still passes native/arm64-floor/arm64-QEMU/x86_64-QEMU routes,
including CPU/glibc controls. The new Grove consumer refuses missing/tampered
Ravel before invocation. Linux Grove standalone execution currently refuses
confinement because the original CentOS floor userland lacks /usr/bin/bwrap;
linux-standalone-floor-k52 is inserted before k51 and owns a real compatible
backend/environment. This is not credited as complete Linux caller acceptance.

All 15,780 individually frozen subjects match before/after with no additions,
removals or changed bytes/links. They include tracked source/Taskfile/book
manifests and indexes, compiler/locked Cargo dependencies, all generated Grove
archives, pinned Ravel target inputs, tools, owner config and complete original
22.1.0 and independent 0.1.0 kegs. All 2,076 protected installation/config/adjacent
checkout subjects also match k48's baseline. Ravel/tap/default Grove remain clean
empty changes at their original handoffs. Evidence/coordination promotion follows
this measurement and is not misrepresented as measured source.

Complete logs/instruments are in target/k49: check-final.log,
installed-commands-final.log, all-build.log, native-smoke-measured.log,
formula-measured.log, registration-measured.log, independent-ravel-smoke.log,
target-controls.log, consumer-refusals.log, linux-smoke.log, measure.py,
before-final.json, after-final.json, digest-comparison.json,
preservation-comparison.json, per-case-comparison.json, lock-comparison.json,
removed-package-paths.txt, inventory.py/lexicon-final.json/inventory-final.json,
removal-controls.json, coverage.json and direct-coverage-fallback.json. Earlier
red/debug logs and the incomplete Linux run remain evidence, not passes.

The refreshed tracked-text inventory covers the whole tracked tree; executable
negative/history/mutation controls were observed before crediting the clean
delivery scope. Best-effort graph coverage disclosed excluded Cargo/generated
state and partial diagrams/installer/QEMU ranges, which were read directly.
Changed Rust files are test/support evidence, outside the literal source-book
corpora; the overview package table and release/install/README links are repaired.
Historical changelog README links now use immutable v22.1.0 source with verified
anchors. Broader ownership prose remains k12's named scope. No new ADR is needed
for this already settled contraction; current caller ADRs remain consistent.

The parent remains live for k50, k52 and k51. No installation, release, personal
policy mutation or history lookup, new driver or teardown ran. Preserve the original
harness-dispatch exit/admission contract; retire k49 and seal its one jj change
before that original ending command as the last action.
