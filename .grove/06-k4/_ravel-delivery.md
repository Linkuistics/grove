# ravel-delivery-k4 — brief

## Goal

Prepare Ravel's independent release tasks, three supported unreleased archives
and tested formula template, ready for ravel-publication-k11 to publish/install.

## Context

- Consume ravel-cli-k3's committed Ravel source and validated build/install tasks.
- Adapt the actual `scripts/release-{common,build,doctor,prepare,publish,smoke*}`,
  release tests, release configuration and formula template. Retain the pinned
  runtime fetch/digest checks, glibc/CPU instruments and archive assertions.
- Supported targets: aarch64-apple-darwin, aarch64-unknown-linux-gnu and
  x86_64-unknown-linux-gnu. Keep Bun 1.4.2, glibc 2.17 and current CPU floors.

## Done when

- Ravel owns version cuts, release notes, check/build/install and archive/smoke
  tasks in its root Taskfile, documented prerequisites and partial-release
  recovery. Use k3's 0.0.0 development version and prepare a first minor cut to
  0.1.0 after rechecking remote version tags; its lifecycle is independent of
  Grove's. No tag or public cut here.
- Archives contain `ravel`, its matching private worker, generic SDK/examples
  and notices. They contain no Grove binary, adapter or policy. Exact manifest
  checks and their omission/extra/probe controls pass.
- The moved release tests prove prerequisite failures stop before the cut,
  dirty taps refuse before publication, and smoke runs before pushes/releases.
  Every supported target executes the installed-layout seam without Grove or a
  host TypeScript runtime; report missing prerequisites as unfinished work.
- The `ravel.rb` template installs the independent layout and passes its
  fake-agent checks from a symlinked temporary prefix. The tap's `ravel-lite.rb`
  is a different command and remains intact. Formula publication is k11.
- Record source commit, unreleased artifact paths/digests, per-target smoke
  evidence and validated publication/recovery commands in the handoff. The
  pinned Bun runtime evidence moved by k3 remains cited by build/release scripts.
  Missing prerequisites or unrun target checks are unfinished work, never skips.

## Notes

This is preparation in `/Users/antony/Development/Ravel`. Commit the working
recipes and instruments through jj; publication/installation is k11's separate
verified boundary. Do not publish/install Grove or replace the running tool.

## Decisions (running log)

- 2026-10-12: Split delivery into unreleased archives/formula validation and
  release-lifecycle preparation. The actual inherited pipeline spans separate
  build/floor instruments and version-cut/publishing/recovery instruments;
  each child must deliver an independently tested boundary. This session owns
  the first child only; k11 still requires both children and the complete k4
  contract. No public cut or installation in either preparation child.
- 2026-10-12: Remote Ravel tags currently contain only
  `structured-plan-migration`, no version tags. Retain development 0.0.0 and
  plan the first minor cut at 0.1.0; lifecycle/publication must recheck tags.

## Archives handoff — ravel-archives-k24

Ravel local main is now `454ecc817342860b645af06a3c42212fe72baeb9`, sealed with
an empty successor. All three 0.0.0 unreleased archives, exact manifests,
omission/extra/type/mode/probe controls, individual native/QEMU/glibc/CPU-floor
installed seams and actual Homebrew formula install/test methods passed.
The 134 runtime tests passed; the 14 ignored subprocess roles are real-test
fixtures. All 14,171 measured subjects retained matching individual digests.

Exact archive paths/digests, per-target evidence and complete logs are in
ravel-archives-k24 and Ravel's durable `docs/archive-evidence.md`. The runtime
pins are unchanged; config/tap/original installed Grove and dispatch are intact.
The tap is clean, ravel-lite.rb remains intact, and ravel.rb is absent. The
rendered 0.0.0 formula is unpublished preparation output, never a tap input.

ravel-release-lifecycle-k25 remains live and owns the actual independent
cut/notes/preflight/publish recipes and their prerequisite/tap/order/dry-cut/
recovery checks. This parent's full Done when is not met yet; k11 publication
requires k25 and all parent criteria. Rechecked remote tags still contain no
version releases; 0.0.0 to 0.1.0 remains the planned first minor cut.


## Complete delivery handoff — k25 closes ravel-delivery-k4

Ravel main is sealed at `64b1e6e7b92ebd3eac28c9765e6c705d40c3e95d`, followed by a clean
empty change. It contains independent sequential cut/notes/doctor/publish tasks,
1.1.2 cargo-release config and partial-boundary recovery, built on k24's archives.
The final source remains 0.0.0; no production cut/tag/push/publication or permanent
installation occurred. Remote version tags remain absent; first minor is 0.1.0.

| Acceptance criterion | Delivered evidence |
|---|---|
| Independent root release/check/build/install/notes/archive tasks and recovery | Taskfile.yml, release.toml, docs/RELEASING.md and passing isolated lifecycle task |
| Exact independent three-target archives, pins and floors | k24 manifest/build instruments plus k25 all-target rebuild and every native/QEMU/floor installed seam |
| Prerequisite/tap refusal and smoke before every publication step | Actual Task dry/order/failure runs, real local jj/remotes and doctor controls in release-lifecycle.aBzyix |
| No caller/host runtime in target layouts | Exact-manifest/omission/extra/type/mode/probe controls, direct/symlink installed seam and pinned worker build |
| Actual ravel.rb layout/fake-agent test; ravel-lite retained | Real Homebrew methods in temporary prefixes, all four worker-identity controls, isolated tap/recovery fixtures |
| Exact committed source, digests, individual routes, logs and commands | docs/release-lifecycle-evidence.md and k25 terminal handoff |

All criteria are met; no preparation check/prerequisite remains unrun or missing.
All 134 runtime tests passed; 14 ignored entries are invoked subprocess fixture
roles. All 14,183 subject digests matched individually. Config and the original
installed 22.1.0 pair are intact. Tap remains clean at e7416e57, ravel-lite intact,
ravel.rb absent. k25's latest three local archive/formula digests supersede k24's.

Promote this exact source/evidence to k11/root; this node closes with k25's
retirement and needs no additional child. The actual 0.1.0 cut, tagged rebuild/
smoke/formula, GitHub/tap publication and Homebrew installation remain k11's
separate contract. Original live-driver authority and ending remain unchanged.
