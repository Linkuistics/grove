# grove-directory-confinement-k99

## Goal

Prove the confined standalone route with directory delivery. Measure the
confined native case separately, with no directory-grant bypass.

## Context

- The "Confined standalone" row and the "Installation and bootstrap" section of
  the delivery specification.
- `crates/grove/src/standalone.rs` and `crates/grove/tests/standalone.rs`.
- Ravel's read grants for skill snapshots, from ravel-skill-confinement-k59.

## Done when

- A confined `grove run` on the 2.0 build with directory delivery reads the
  required resources outside cwd. Its writes and protected configuration reads
  still fail.
- With injection disabled and native trees outside every grant, the fake agent
  cannot read the guidance. It reports the failure and runs the exact
  force-stop command of its prompt. Grove publishes no output.
- The same case on the 1.1 build gives the same outcome. Grove adds no recursive
  native-directory grant and never retries unconfined.
- The cases run on macOS. Each Linux backend that the floor tooling supports
  runs them too, or the evidence names it as unmeasured.
- `docs/specs/standalone-invocations.md` and the overview book's standalone
  chapter agree with behaviour. Any regenerated book passes its check.

## Notes

Native installation and confined native readability are separate claims. This
leaf establishes no working native-only standalone configuration.

Controls: a grant of the native tree must make the unreadable case fail. An
unconfined retry must fail a test. Watch each fail once.

## Decisions (running log)

**Builds.** On 2026-11-03 this session installed the 2.0 build from
`Ravel.skill-directory-delivery` at output commit `4f831feb4d0f`, with an empty
working copy, by `task install PREFIX=<scratch>`. Its `bin/ravel` SHA-256 was
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`. The 1.1 keg
`bin/ravel` SHA-256 was
`c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. Both match
the node brief. The bookmark has not moved.

**Native case.** `native_guidance_outside_every_grant_is_reported_and_force_stops_without_publication`
in `crates/grove/tests/standalone.rs` runs under the selected runtime, so it is
not ignored. The prompt names native copies under `.claude/skills`,
`.codex/skills` and `.agents/skills` in the fixture HOME. The agent reports
each failed read on stderr, which reaches the transcript. A second start of
the agent fails the check, so the check detects a retry. `task ravel:directory`
also runs this case on the 2.0 build.

**Directory case.** `directory_route.rs` gains a confined standalone case. The
agent can write only in its working directory, so its evidence returns as
published outputs. The configuration passes `request.originalPrompt` and
`request.skills` as an argument. A probe script, granted as a runtime read,
checks writes and protected reads. The test runs the same script unconfined as
the control.

**Snapshot writes.** The unconfined probe also cannot write in a snapshot,
because Ravel publishes snapshots read-only. The snapshot probes therefore
show that writes fail, not which layer refuses them. The source and
outside-cwd probes succeed unconfined, so their denials are the sandbox's.

**Controls, each seen to fail once.** A mutation of `src/standalone.rs` that
retries unconfined after a force-stop made the native case fail: Grove
published the output. A mutation that expected the granted run to pass the
unreadable check failed, because the agent read each granted copy. The source
digests were the same before and after both mutations. The unconfined probe
run returned `ok` for each read and each non-snapshot write that the confined
run denied.

**macOS results.** The native case passed on the 1.1 keg and on the 2.0 build.
`task ravel:directory` passed all seven directory cases and the native case.

**Linux floor case.** `scripts/grove-floor-native-cases.sh` gains the native
case in its `supported` mode, on the 1.1.0 floor inputs. Its agent reports
each failed read and force-stops. A second run grants each copy as a regular
file and must publish. The case runs the agent with the selected `ravel`
path, as the existing floor agent does. The macOS cases prove the use of the
exact prompt command.

**Linux results.** `task release:floor:qemu` with fresh Grove archives and the
pinned Ravel 1.1.0 archives passed on aarch64 and x86_64 under bubblewrap
0.11.0. Each target reported three unreadable copies, a force-stop with no
publication and a published granted control.

**Unmeasured on Linux.** The native arm64 container floor did not run. The
runner refused the freshly prepared aarch64 bubblewrap, SHA-256
`d35694e8…`, against the recorded `21ad63fa…`. That backend drift is
outside this leaf, and the node's k101 leaf notes it. The directory case and
the 2.0 native case did not run on Linux, because the floor tooling has no 2.0
lane. k101 owns that lane, and its notes now name these cases.

**Documentation.** The delivery and caller specifications, the route view and
`docs/USAGE.md` state that the confined `grove run` supports the directory
route. The standalone specification and its obligations state the measured
reads, denials and force-stop. The delivery test seams name both test files and
the Linux status. The configure-grove policy reference recommends the directory
route for a skill that `grove run` needs. The changelog splits the standalone
outcome into its own entry. The overview book's standalone chapter states that
Grove's argv grants no skill directory. Its book check passed.

**Review.** The work changes tests, a floor case script and documentation. No
caller code changed, and executable controls cover each new check. The work
earns no `review-impl` leaf, and this session spent no in-session reviewer.

**Full check.** With `GROVE_TEST_RAVEL` at the 1.1 keg,
`mise exec bun@1.4.3 -- bash scripts/check.sh` passed all 20 principal checks
and all five books.
