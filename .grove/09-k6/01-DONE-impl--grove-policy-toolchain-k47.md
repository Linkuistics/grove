# grove-policy-toolchain-k47


## Goal
Give Grove-owned policy assets their own pinned development compiler, independent
of the retained dispatch package, so that package can later be removed safely.

## Context

Consume k42/k46's readable assets and public installed-Ravel SDK contract. The
existing helper extends the old worker config and uses its node_modules; separate
this development-only setup from both shipped assets and the independent Ravel
installation. Keep Bun 1.4.2, @types/bun 1.4.2 and TypeScript 7.0.2 plus exact
locked transitive packages. No new runtime/framework API or dependency upgrade.

## Done when

- Own package/lock/strict tsconfig under tools/ravel-policy and expose a reusable ravel:policy:deps task. Installed-SDK typechecking no longer needs old source, generated declarations or dispatch:deps; ordinary relative imports/type fixtures stay covered.
- An isolated checkout fragment with no crates directory runs the actual Taskfile/typecheck helper against installed Ravel. An invalid policy type and missing installed SDK refuse; observe the old dependency failure before changing the helper.
- Keep the retained worker compiler/tasks and compatibility fixtures functional for k49. Installed policy/provenance/compatibility checks and task check pass; applicable source-pinned books stay current.
- Record complete logs, frozen subjects and the narrow remaining-reference classification; no install, release, personal configuration or live-driver mutation.

## Decisions (running log)

- Place development tooling under tools/ravel-policy, outside shipped assets. Copy the existing lock pins and strict compiler flags; remove only old SDK path aliases and old source includes. Typechecking continues to derive public declarations from the explicitly supplied installed Ravel's canonical prefix.
- Execute inline within this AFK mandate. Plan: add a real compiler isolation/refusal test; observe the old dispatch dependency fail; switch compiler/task paths; run focused policy/review checks and one frozen task check; retire/seal this child only. The mandated k6 remains a live node with later children.
- The isolated fragment first failed in dispatch:deps because no old package.json existed. After relocation it passes with the real compiler; an invalid asset produces TS2322 and a prefix without declarations refuses. Host PATH now resolves Bun 1.4.3; the unchanged 1.4.2 guard correctly refused it. Use the already preserved Ravel/target/tools/bun 1.4.2 only in this session's check PATH, without changing the host installation or project pins.
- Decline an in-session reviewer: this is a mechanical compiler/configuration relocation, and the real isolated Taskfile/compiler plus invalid-type/absent-SDK controls are a stronger instrument for its dependency claim. The later delivery acceptance child is already chartered for the complete boundary. No review allowance was spent.

## Notes

## Handoff

The shipped artifact for this child is the Grove-owned development compiler
under tools/ravel-policy, the ravel:policy:deps/compiler-test tasks and the
relocated public-SDK helper. It does not ship in archives. Its lockfile equals
the retained old compiler lock byte-for-byte apart from the root package name;
strict options equal the old config apart from removed legacy SDK aliases.
All caller assets and ordinary type fixtures remain inputs. The installed
Ravel front's real prefix supplies public declarations; no source checkout,
old worker declarations or dispatch:deps enters the new path.

The real isolation test first failed against the old Taskfile/helper because
dispatch:deps required the absent old package.json. It now passes in a fragment
with no crates directory and a path containing spaces. TS2322 catches a deliberately
invalid caller type; a prefix with no declarations refuses. The host Bun 1.4.3
refusal also fired. Checks used the existing preserved
/Users/antony/Development/Ravel/target/tools/bun 1.4.2 via session-local PATH;
no host tool installation or pin change occurred.

One final task check passed all 14 principal checks, 1,723 workspace nextest
cases (31 ignored cases remain outside that workspace run), nine doctests and
six final/all books. The gate separately passed all 88 policy/provenance/guidance/
retained compatibility cases, including the nine installed-worker policy cases,
plus the compiler isolation/refusal suite and release/manifest tests. Shellcheck
and shfmt pass on the changed helpers. No runtime/caller implementation changed.

Complete gate output is target/k47/check.log. Additional evidence is
old-dependency-red.log, host-bun-refusal.log, surface-audit.json,
before.json, after.json, digest-comparison.json and each adjacent checkout's
state-before/state-after text. measure.py reuses k46's exact subject instrument
and adds this compiler's installed graph; audit/surface.py records the bounded
surface inventory, locked-graph/strict-flag checks, observed controls and every
book-corpus intersection. All 18,123 individually named source/config/manifest/
compiler/dependency/installed-layout/tool subjects match before and after the
full gate. Coordination/handoff prose below this boundary is added afterwards.

The nine affected production/configuration surfaces were token-enumerated and
classified; the new helper/config/lock has no old worker path, while the original
helper positive control and retained .gitignore cross-tree control are dirty.
Removing the old path from the positive control makes it fail. Old build,
compatibility, archive and authority references in Taskfile/check/.gitignore
are intentional until k49. This is a bounded k47 audit, not a repository-wide
removal claim. Every modified production path lies outside all six book corpora.
Graph generation was 2026-10-13T10:21:19Z; exact helper discovery/snippet and
coverage were checked. Untracked lock/config evidence was read directly; compiler
node_modules are intentionally excluded from the graph and were verified by real
compiler execution and per-file digests instead. Clean coverage is best-effort.

Adjacent Ravel main remains 3916a4a1, tap main bd5aa37b and default Grove main
89f947d5, each with an unchanged clean empty successor. Installed Ravel remains
0.1.0; the complete original Grove 22.1.0 keg, original commands and owner config
retain their measured bytes. No publication, permanent installation/configuration
changes, history access, new driver or teardown occurred. The remaining four
k6 children stay live; k6 and its ancestors do not close here. This session's
last action remains the original harness-dispatch exit.
