# dispatch-delivery-k16 — brief

## Goal

Ship the `harness-dispatch` pair through Grove's existing release route. Every
release archive and the Homebrew formula must install the front executable and
its matching compiled worker in a layout the front can find. Each supported
target proves the installed layout by executing it at the declared floor. No
separately installed runtime may be present.

## Done when

- `scripts/release-build.sh` compiles the worker for macOS arm64, Linux arm64
  and Linux x64 with pinned Bun 1.4.2. It stages the front executable, the
  worker with its embedded SDK and examples, their type declarations and
  readable sources, and the runtime and license notices, SQLite's included,
  into each of the three existing archives. The Linux front executables compile
  bundled SQLite at the glibc 2.17 floor. The release still publishes exactly
  three archives.
- The archive and the Homebrew install keep the same relative layout, so the
  front finds its worker from its real path, including through Homebrew's
  symlink. The formula installs that layout, and `brew test` checks that the
  front, the worker and Grove report one version. The release task's
  post-install verification checks `harness-dispatch --version`.
- The new member opts out of a second cargo-release cut, and the worker embeds
  the workspace version. The release scripts' tests assert the archive contents.
- Each target runs an installed-layout smoke test from the extracted archive:
  static TypeScript policy, fake harness, a `run` whose record is read back
  from a temporary state directory, and no Bun or Node on PATH. macOS
  arm64 runs natively. Each Linux target runs in a glibc-2.17 userland
  container, and again under user-mode emulation with the Nehalem and
  Cortex-A53 CPU models. Each floor instrument has a positive control that has
  been seen to fail against a subject that violates it.
- The kernel floor is stated as Bun's documented range, labelled documented
  rather than executed. Bun's README gives 5.1 and its installation page gives
  3.10. The macOS 13.0 minimum is stated too. `docs/RELEASING.md` gives the
  delivery prerequisites and the recovery path.

## Decomposition

1. `release-layout-k17`: cross-compiled workers, archive layout and content
   assertions, formula and version checks, and the cargo-release opt-out.
2. `installed-smoke-k18`: the per-target installed smoke task, macOS native plus
   Linux glibc-2.17 containers, with the glibc control.
3. `cpu-floor-k19`: the same smoke under CPU-model emulation with its control,
   then release-pipeline wiring and the documented floors.

## Pointers

- Spec sections: `#delivery` and `#package-boundary`.
- ADR: `docs/adr/policy-evaluation-precedes-process-replacement.md`. Its
  "embed a small JavaScript engine" reopen condition is this node's failure
  mode. If the compiled worker cannot meet the floor, stop and escalate with
  the evidence. Do not weaken the floor or fall back to a host runtime.
- Existing route: the release target list, the release build, the Homebrew
  formula template, the release task in `Taskfile.yml`, `docs/RELEASING.md` and
  the release script tests.
- Tools observed on the development host on 2026-09-30: Bun 1.4.2, Docker 28,
  QEMU 11.1 (system emulators only; user-mode emulation must run inside a Linux
  container), zig and cargo-zigbuild. Treat these as observations, not
  guarantees.

## Notes

Later increments add shipped files: the Grove adapter, the review example and
more declarations. Each adding leaf extends the archive assertions in the same
change. Any leaf that changes the worker, the layout or the native dependencies
reruns the smoke task. Bundled SQLite arrived with `handoff-records-k24` in
static dispatch, so this node's first cross-build already compiles C, and its
floor instruments probe that native dependency from the start.

Docker Desktop's own amd64 emulation cannot run the glibc-2.17 x86-64
userland on this arm64 host (runtime evidence, `#installed-smoke`), so
`x64-floor-k49` settled an environment of its own, and `installed-smoke-k18`
closed with every Linux target executed at the C library floor. On an arm64
Docker, `smoke_under_qemu` in `scripts/release-smoke.sh` runs the floor image's
amd64 filesystem in a chroot, under a pinned QEMU 10.2.3, inside an ordinary
container. `scripts/release-smoke-qemu.sh` registers QEMU in a binfmt_misc
instance private to a user namespace, and checks that the guest has a vDSO and
nothing mapped above 2^47. The front scrubs `QEMU_*` from the worker's
environment, so QEMU's options come from a Zig-built interpreter.
`build_qemu_interpreter` holds each architecture's options: `cpu-floor-k19`
adds `-cpu Nehalem` beside the `-B 0x800000000000` that x64 already needs. The
spec's C library row now names this route beside the container. Linux arm64
still runs natively in Docker. Emulating it at Cortex-A53 on this arm64 host is
not a drop-in: an aarch64 registration would also match the helper's own
executables and the static interpreter itself.
