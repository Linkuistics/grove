# grove-directory-lifecycle-k98

## Goal

Prove the directory route on 2.0 through Grove's lifecycle caller with local
sources and native copies absent. Then advertise the route in the owner
guidance.

## Context

- The "Directory bootstrap", "Source and skill failures", "Original mandate",
  "Settings isolation" and "Resource lifetime" rows of the delivery
  specification.
- Ravel's specification `docs/specs/skill-discovery.md` at the directory build.
  Its acceptance case "Local sources only" uses a spy `git`.
- Ravel's `task smoke:pair OLD=<1.1 executable> NEW=<prefix>` runs a 1.1 and a
  directory launch at the same time, each under its own HOME.

## Done when

- A lifecycle launch on the 2.0 build uses `skills` settings in a HOME of its
  own. Its sources sit outside cwd. One is a plain directory, and one is a
  plain Git checkout at a commit.
- The fake agent reads the spine, the named kind and a supporting resource from
  the injected directories. No native copy exists in that HOME.
- `request.originalPrompt` and the caller block of the prepared prompt each
  equal the composed mandate byte for byte. The ending commands are exact. The
  preamble names the mandatory reads before the caller block.
- A missing checkout refuses as `skill_source_missing` before any worker. Grove
  reports Ravel's remedy, the leaf stays live and the loop does not relaunch.
- One directory launch runs under a guard that denies the network. A spy `git`
  first on the PATH records no call.
- A native lifecycle launch on 1.1 and a directory launch on 2.0 run at the same
  time in separate HOMEs. A 1.1 HOME with `skills` settings refuses as
  `settings_invalid` and leaves the leaf live.
- A running session keeps its resolved directories while its source changes.
  The next launch receives the new view.
- The delivery specification, `docs/USAGE.md`, the configure-grove skill and
  the plugin installation guidance state the directory route as supported.
  They state the checkout boundary and the separate HOME.

## Notes

These fixtures prove delivery and Grove's interpretation, not the behaviour of
an LLM. common-loading-acceptance-k29 owns the real harness trials. The
explicit plugin-qualified name request also belongs to k29.

Never use the real HOME or the owner's `~/.config/ravel`. Controls: a fixture
whose fake agent reads a native copy must fail. A caller block that differs by
one byte must fail. Watch each fail once.

## Decisions (running log)

**Builds.** On 2026-11-03 this session installed the 2.0 build from
`Ravel.skill-directory-delivery` at output commit `4f831feb4d0f`, with an empty
working copy, by `task install PREFIX=<scratch>`. Its `bin/ravel` SHA-256 was
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`. The 1.1 keg
`bin/ravel` SHA-256 was
`c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. Both match
the node brief.

**Hand probe.** One direct `start-session` with two local sources showed the
preamble, the label `Caller task`, the delimited block, `request.skills` and
`RAVEL_SKILL_DIRECTORIES`. The cache is under `$HOME/.cache/ravel`, so a
separate HOME also separates the snapshots.

**Test file.** `crates/grove/tests/directory_route.rs` holds six cases. Each is
ignored by default, because only the 2.0 directory build can run it. The new
task `ravel:directory RAVEL=<2.0 front> OLD=<1.1 front>` runs them with the
runtime fixed at `2.0`. A case under another runtime stops with that remedy.

**Second build.** The pair case needs a 1.1 build beside the 2.0 build under
test. `GROVE_TEST_RAVEL_OLD` names its front. `Runtime::named` and
`InstalledRavel::copy_as` take the runtime explicitly, and `for_build` and
`copy_from` now call them. The layout check and its remedies are unchanged.

**Fake agent.** The agent takes the mandatory paths and the search directories
from the preamble, and the skill name from the mandate in the caller block. It
reads the resource relative to the spine's directory, and it runs the exact
continue command from the caller block. A case therefore passes only when the
delivered prompt is usable.

**Before any worker.** The missing-checkout case uses a `config.rhai` that is
not valid Rhai. A worker that ran would refuse with a configuration error. With
the checkout restored, the same script refuses for its own reason.

**Network guard.** `sandbox-exec` with `(deny network*)` wraps the driver, so
Ravel and the agent inherit it. An unguarded `nc` reaches a local listener, and
the same probe under the guard fails. The guarded agent's probe also fails.

**Controls, each seen to fail once.** The native-copy case with `unwrap` in
place of `unwrap_err` failed. An expected caller block one byte off failed with
the byte difference. The network case without the guard failed, because the
agent reached the listener. The file digest `cac69c4f…` was the same before and
after the mutations.

**Documentation.** `docs/USAGE.md` has a third route row and a section "The
directory route". That section gives the settings, the checkout boundary, the
separate HOME, the missing-source refusal and the snapshot lifetime. The
configure-grove skill, its policy reference, `plugins/README.md` and
`README.md` state the route as supported. The delivery and caller
specifications, the route view, the glossary, the architecture row and the
changelog agree. Each says that the confined standalone proof remains ahead
for grove-directory-confinement-k99. No book quotes a changed file, and the
anchors that the books cite are unchanged.

**Results.** `task ravel:directory` passed all six cases against the 2.0 prefix
and the 1.1 keg, behind a host-guard stub that exits 86. `task ravel:resolution`
with `RAVEL_RUNTIME=2.0` passed. With `GROVE_TEST_RAVEL` at the 1.1 keg,
`mise exec bun@1.4.3 -- bash scripts/check.sh` passed all 20 principal checks.
The first run failed only `cargo fmt` on the new test file. After rustfmt the
second run passed, and the diff digest was the same before and after it.

**Review.** The work changes test wiring, test support and documentation. No
caller code changed, and executable controls cover each new check. The work
earns no `review-impl` leaf, and this session spent no in-session reviewer.
