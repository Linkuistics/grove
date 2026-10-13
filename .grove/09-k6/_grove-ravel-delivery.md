# grove-ravel-delivery-k6 — brief

## Goal

Stop building and shipping dispatch inside Grove, and make Homebrew supply
Ravel as an independently versioned runtime dependency.

## Context

- Requires grove-ravel-caller-k5's passing runtime/caller migration.
- Update Grove's workspace/lockfile, Taskfile/check gate, release scripts and
  tests, exact archive manifest and `scripts/templates/grove.rb.tmpl` together.
- Caller assets moved by the preceding leaf remain Grove-owned and ship through
  Grove's delivery. The local runner remains in each project's own source.

## Done when

- Grove archives ship its own executables and caller assets; they contain no
  Ravel/dispatch front or private worker. Grove's formula depends on
  `linkuistics/taps/ravel`; no front/worker version comparison crosses projects.
- Remove the old dispatch package and its build/typecheck/probe recipes only
  after preserving/migrating every caller test and asset. Grove's normal check
  path verifies integration through independently installed Ravel.
- Archive assertions and release-pipeline tests pass with the new manifest.
  Installed-layout tests on all current targets put Ravel in a separate prefix
  with a different version and prove both lifecycle and standalone paths work.
  The formula test checks the dependency and caller assets without a host runtime.
  Obtain each target's Ravel layout from k11's immutable archive URL and digest;
  check target, manifest and digest before invoking it, including Linux/QEMU
  where Homebrew is absent. Missing/mismatched archives fail, never skip. Reuse
  the existing archive and CPU/glibc seams without installing a host runtime.
- Source-pinned walkthroughs affected by workspace/manifest/package removal and
  gate-facing release/install documentation change with this leaf. `task check`
  stays green; broader ownership/prose reconciliation belongs to k12.
- Enumerate remaining old-name/dependency references over the actual affected
  surfaces, classify necessary history/compatibility fixtures, and verify removal
  claims with positive/cross-tree controls. Preserve all live-session guards.
- Prepare and test the live-install transition before k13 can publish: upgrading
  Grove unlinks its old dispatch front, and the running driver keeps its original
  canonical dispatch path for future launches. Retain the complete original keg
  (front, worker, SDK and notices) with Homebrew automatic cleanup disabled for
  the release. Preserve bare `harness-dispatch exit` through a temporary link to
  that original front in an already inherited stable PATH directory, such as
  `~/.local/bin`; refuse to overwrite an existing entry. Do not copy only the
  front or repoint the old canonical path at Ravel. Preserve access to the
  original grove-llm for its admission/teardown contract as well.
  Test an isolated upgrade/unlink with the actual old 22.1.0 front/worker: it can
  launch the next fake session, reap/write the old store, and the bare command
  signals only its temporary legacy channel. Ravel's end-session uses its own
  process context; the new CLI is not a replacement ending command for a live
  session launched by the old driver.
  Record paths/digests, cleanup suppression, link ownership and a post-driver
  cleanup procedure. There is no historical catch-up/import work. Keep the legacy installation until the driver has
  ended; neither k13 nor final finish removes it before its final signal.

## Decomposition

This leaf spans compiler ownership, target inputs, an atomic package/delivery
removal, a live-install compatibility instrument, and complete target acceptance.
Each needs a fresh focused session; the first child runs in the decomposing session.

- grove-policy-toolchain-k47 detaches the caller compiler and validates installed-SDK typechecking without the old source tree.
- ravel-target-inputs-k48 supplies fail-closed, pinned independent layouts for all supported targets before delivery removal.
- grove-delivery-contraction-k49 removes the old package/build recipes and switches archives, formula and smoke consumers together, preserving caller assets/tests and source-pinned books.
- legacy-driver-transition-k50 prepares/tests isolated old-keg preservation and upgrade/unlink, without modifying the live installation.
- linux-standalone-floor-k52 prepares the newly exposed missing Linux confinement backend before complete acceptance.
- grove-delivery-acceptance-k51 runs frozen complete target/formula/check acceptance and reconciles every parent criterion before closing k6.

## Decisions (running log)

- Decompose at the independently testable delivery seams. Keep the original installed driver/exit channel and all Cargo authority guards throughout. First move only Grove's policy-development compiler dependencies/configuration; retained dispatch build/typecheck/probe tasks remain until the atomic contraction child.

## Notes

Verify source and unreleased archives here. k13 performs the human-requested
mid-grove extraction release after k12; final finish releases session-control
behavior later. No installation changes here. k6's preservation is a temporary
transition instrument outside Grove's new archives/formula, not bundled Ravel.

## k28 delivery input

Ship `assets/ravel-policy/` with its relative imports intact. Before removing the
old dispatch crate, relocate the pinned policy-development dependencies and
`tsconfig.json` used by `scripts/ravel-policy-typecheck.sh`/`dispatch:deps`, and
preserve installed-Ravel checks from `ravel:policy`/`ravel:provenance`. Drop
`ravel:policy:compatibility` only with the old package's delivery removal; k42
has preserved and verified those fixtures at the current boundary. k5's promoted
handoff and k42's terminal leaf contain exact tested asset/evidence contracts.

## Compiler handoff — grove-policy-toolchain-k47

Grove caller typechecking now owns tools/ravel-policy and ravel:policy:deps;
the actual Taskfile/helper passes against installed Ravel 0.1.0 in a checkout
fragment without crates/. Exact compiler lock pins and strict flags are retained.
The invalid-type/absent-SDK controls and full task check pass; 18,123 individually
frozen subjects match. k47 carries the complete evidence/log boundaries.

The old dispatch package/compiler/tasks remain intentionally for k49's atomic
contraction, independent of the new caller compiler. k48 supplies pinned target
layouts next; k50 prepares the actual-old-keg transition; k51 owns final delivery
acceptance. No k6 archive/formula/removal/transition criterion is declared done
by this compiler increment. Original 22.1.0 exit/admission bytes remain preserved.

## Target-input handoff — ravel-target-inputs-k48

`docs/ravel-target-inputs.md` documents k49's reusable prepare/controls/smoke
interface and exact independent Ravel 0.1.0 pins. Public three-target downloads,
full admission/refusal controls, native and Linux/QEMU routes all pass. Separate
prefixes and frozen full logs/layouts are retained in target/k48; all 18,172
named subjects match, and task check passes all 15 principal checks. Original
driver/config/installation and adjacent checkouts remain intact. k49 still owns
the atomic consumer/archive/formula contraction; k50 and k51 remain live.

## Contraction handoff — grove-delivery-contraction-k49

Grove's workspace/build/gate/tasks/archive/formula now contract atomically to its
two fronts plus the complete readable libexec/ravel-policy tree. The formula
depends on independent Ravel; old package/worker/probe/compatibility recipes and
the unused old-front test helper are removed. docs/grove-delivery-contraction.md
records fixture ownership, current tasks, preserved guards and the exact boundary.

Default task check resolves installed Ravel from PATH and passes all 14 checks,
1,378 workspace cases, nine doctests and six final/all books. Native installed
callers, actual private Homebrew install/test, explicit ignored public fixtures
and combined Linux ELF refusal controls pass. Independent Ravel-only all-target
smoke remains green. All 15,780 frozen subjects and 2,076 protected k48 baseline
subjects match; k49 holds complete per-item logs and coverage/removal controls.

The new real standalone caller exposed the CentOS floor image's missing bwrap.
linux-standalone-floor-k52 is inserted before k51 to supply a deterministic real
compatible backend/environment, never an unconfined fallback or skip. k51 consumes
k47-k50 plus k52 and owns final rebuilt all-target caller/formula acceptance.
k50's original-keg transition preparation and k51 remain unfinished. Original
22.1.0 installation/config/driver ending are unchanged; no publication/install
or parent close is implied by this contraction.

## Legacy transition handoff — legacy-driver-transition-k50

`docs/legacy-driver-transition.md` supplies k13's tested preparation/application/
verification/recovery/post-driver cleanup; `docs/legacy-driver-inventory.json`
pins the complete original 22.1.0 keg and original fronts/worker/helper digests.
Taskfile legacy:* entry points pass 31 ownership/refusal/cleanup controls and
an actual-original copied-keg upgrade/unlink rehearsal, including canonical old
launch, bare exit, old store/export, reaping, original helper refusal and nested
independent Ravel context isolation. The tool never removes a keg; link cleanup
requires original-driver ending and retains replacements by filesystem identity.

Final task check passes all 15 checks, 1,378 workspace tests, nine doctests and
six final/all books under pinned Bun 1.4.2 on a child-only PATH. All 15,788 frozen
subjects match individually; complete evidence/logs/instruments are in target/k50.
The live installation, config/history/authority and stable PATH entries remain
unchanged. k13 must prepare/apply a fresh plan and keep its complete backup,
canonical original keg, original dispatch/helper links and cleanup protection
through final finish's last signal. Record that plan's absolute path in the
root brief; every later release/recovery shell must source its release-env.sh.

k50's preparation contract is met. k52 supplies the Linux standalone floor;
k51 owns final all-target delivery acceptance and reconciliation. No k6 close,
release/publication, live transition application or teardown is implied.
