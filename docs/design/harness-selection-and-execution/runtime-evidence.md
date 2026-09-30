# Runtime and source evidence

This evidence supports the [worker decision](../../adr/policy-evaluation-precedes-process-replacement.md).
It is deliberately narrower than the full acceptance contract.

<a id="native-probe"></a>
## Native probe

On 2026-09-29, Bun **1.4.2**, macOS arm64, compiled a one-module worker which
dynamically imported an absolute TypeScript policy path. The policy used a type
annotation/interface and a relative import of another TypeScript file. Its
result was the expected candidate `personal-choice` and effort `high`.

The native standalone executable ran under `env -i PATH=/usr/bin:/bin`, without
Bun or Node on that PATH. The worker reported no Grove completion variable.
This establishes embedded TypeScript execution for this native artifact, not
the final Rust boundary or Linux installation.

The same worker was compiled with:

```text
bun build --compile --no-compile-autoload-dotenv --no-compile-autoload-bunfig
  --no-compile-autoload-tsconfig --no-compile-autoload-package-json
  worker.ts --outfile worker
```

The command is shown wrapped for reading; the build used one command line.
The run's cwd contained a `.env` sentinel, a bunfig preload that wrote a sentinel
file, a tsconfig wildcard path mapping to a throwing module, and package metadata.
The guarded build returned the personal policy, no dotenv value and no preload
sentinel. The otherwise identical default build returned the dotenv sentinel and
created the preload sentinel, so the fixture demonstrably exercised those two
risks. The tsconfig and package fixtures did not fire in the default build
either. Bun 1.4.2 already disables tsconfig and package.json runtime loading in
standalone builds by default, so this probe showed no risk for those classes. It
ran the worker in the hostile cwd, so it did not exercise the private worker cwd.
It does not exhaust module-resolution or shadowing cases; the
[integration probe](#integration-probe) below covers the ones the design relies
on.

A second positive control supplied `BUN_OPTIONS=--preload <absolute throwing
module>` to the guarded executable. It ran that module and exited with its error.
Compile flags alone therefore do not establish the boundary. The front process
must sanitize runtime options **before** the worker starts. No live Grove control
value was supplied to these probe processes.

The throwaway experiment is under `/tmp/harness-dispatch-probe.hDMPYZ`; its
durable results are the observations above. Production acceptance must reproduce
these cases against the shipped Rust entry point, including worker-location
spoofing, explicit config authority, module shadowing, cancellation and structured
output. The prototype is not an implementation to adopt.

<a id="integration-probe"></a>
## Integration probe

On 2026-09-30, the design-review integration ran a second probe with the same
Bun 1.4.2 on the same macOS arm64 host. Every worker ran under
`env -i PATH=/usr/bin:/bin` with a private HOME. Unless noted, each ran in a
private empty cwd and was built with all four no-autoload switches. Each run
dynamically imported an absolute policy entry that used a type annotation and a
relative TypeScript import. Sentinel files recorded whether a hostile module ran.

| Class | Case | Observed |
|---|---|---|
| Embedded SDK specifier | Worker registers `harness-dispatch/sdk` with `Bun.plugin` `build.module` before importing a policy that imports it; a `node_modules/harness-dispatch` shadow sits beside the policy | Embedded SDK returned; shadow did not load |
| Same, positive control | Identical worker without the registration | Shadow loaded and wrote its sentinel |
| Same, beside an admitted entry | The shadow sits beside an explicitly chosen repository-style entry | Registered build: embedded SDK. Unregistered build: shadow fired |
| Prefix reservation | Registered build also installs an `onResolve` filter for `^harness-dispatch(/.*)?$` that throws for unregistered names; policy imports `harness-dispatch/other` | The shadow's `other` module loaded: the filter did not intercept |
| `node_modules` shadow in the caller cwd only | Worker run in the hostile directory, policy elsewhere | Did not fire in either build; resolution starts at the importing module |
| tsconfig `paths` beside the entry | Mapping for a bare alias the policy imports | Fired only in a build with tsconfig and package.json autoloading enabled; missing-import failure otherwise |
| tsconfig `paths` in the caller cwd only | Same mapping, worker run in that directory | Did not fire, even with tsconfig autoloading enabled |
| cwd `.env` and bunfig preload | Default-autoload build | Fired when run in the hostile cwd; inert in the private empty cwd |
| cwd `.env` and bunfig preload | Guarded build in the hostile cwd | Inert |
| `~/.bunfig.toml` preload | HOME set to a directory holding one | Did not fire in either build |

Consequences for the design:

- Registered virtual modules displace an adjacent package shadow, including
  beside an admitted entry. The prefix reserves nothing, so every documented
  specifier must be registered.
- The private worker cwd and the no-autoload switches are independent controls
  for cwd dotenv and bunfig; either alone kept them inert here.
- The firing configurations named in the acceptance table come from these rows.
  A HOME bunfig and a cwd tsconfig have no observed firing configuration, so no
  control is claimed for them.

This is one host and one Bun version. Linux, the shipped Rust launcher and every
module-resolution path the SDK does not use remain unmeasured. The throwaway
experiment lived in session scratch space; its durable results are the table
above.

<a id="implementation-observations"></a>
## Implementation observations

On 2026-09-30, the first implementation increment made two more observations
with the same Bun 1.4.2 on the same macOS arm64 host.

- **Automatic package installation.** A policy imported `is-odd`, a real npm
  package, with no `node_modules` anywhere above it. Plain `bun` fetched it
  from the registry into a fresh HOME cache, which is the positive control.
  The worker, compiled with all four no-autoload switches, refused with
  `ERR_MODULE_NOT_FOUND` and created no cache. A command-seam test,
  `a_missing_package_is_never_installed_automatically`, keeps checking the
  shipped worker.
- **Compile leftovers.** Every `bun build --compile` left a read-only copy of
  its roughly 60 MB compile template, named `.<hash>-00000000.bun-build`, in
  its working directory, whatever the output path. The build script therefore
  compiles from a throwaway directory.

Building the release layout on 2026-09-30, with Bun 1.4.2 on the same host,
observed how a cross-compile obtains its target runtime:

- **Bun fetches it unverified.** The `bun-v1.4.2` source
  (`src/options_types/compile_target.rs`, `to_npm_registry_url` and
  `exe_path`; `src/standalone_graph/StandaloneModuleGraph.rs`,
  `target_executable`) downloads `@oven/bun-<os>-<arch>` 1.4.2 from
  `registry.npmjs.org` into `$BUN_INSTALL_CACHE_DIR` (default
  `~/.bun/install/cache`) with no integrity check, and reuses any cached file
  of the right name as is. `--compile-executable-path` skips that fetch.
- **The pinned path fetches nothing.** Compiling for all three targets with
  `--compile-executable-path` and an empty isolated cache left the cache empty.
  The positive control, the same compile without the flag, was seen to
  download `bun-linux-x64-v1.4.2` into that cache, byte-identical to the `bun`
  in the pinned tarball. Each pinned tarball's SHA-512 equalled npm's published
  `dist.integrity`. The darwin output is ad-hoc signed, and none of these
  compiles left a template in the working directory.
- **Homebrew leaves the worker's bytes alone.** A keg installed from a local
  archive held a front and worker byte-identical to the archive's, and its
  `brew test` passed under Homebrew's sandbox. Homebrew 7.0.7's source runs
  `patchelf` only when bottling or pouring, and skips files with a `.bun`
  section even then.

Integrating the static-dispatch review on the same date, with the same Bun and
host, observed how `import()` treats an absolute path string:

- **A `?` in the entry path is a query.** `import("/d/policy.ts?x")` loaded
  `/d/policy.ts`, and `import("/d/d?q/policy.ts")` loaded `/d/d.ts`. The same
  happened through `pathToFileURL`'s `%3F`. `Bun.resolveSync` returned the
  path with its `?` suffix intact, so it could not detect the substitution.
  `#`, `%`, spaces and newlines imported exactly. A `\` failed to load, and
  nothing was substituted. The front therefore refuses an entry whose resolved
  path contains `?` or is not UTF-8. The command-seam test
  `an_entry_path_the_worker_cannot_import_exactly_refuses_before_any_code_runs`
  shows each substitute firing when it is named directly. Recheck this on a
  Bun upgrade.

Adding computed selection on the same date, with the same Bun and host,
observed what an abandoned `await` does to the worker:

- **A pending top-level await spins.** A main module whose top-level `await`
  waited on a promise that nothing was left to settle ran at full CPU, never
  exited and never emitted `beforeExit`, both under `bun` and compiled. The
  same await inside an async function, which the module called without a
  top-level await, let the event loop empty: `beforeExit` fired and the
  process could report and exit. That held for a pending dynamic `import()` of
  a module whose own top-level await never settles, and for a pending promise
  a policy callback returned. Node documents the event for an emptied loop and
  never for `process.exit`
  ([`'beforeExit'`](https://nodejs.org/api/process.html#event-beforeexit)).
  The worker therefore drives its conversation from an async function and
  reports an abandoned import or `select` from `beforeExit`. The command-seam
  tests `each_way_select_can_fail_refuses_with_its_own_code_and_launches_nothing`
  and `an_import_left_unsettled_refuses_as_a_load_failure_at_once` fail with
  that handler disabled. Recheck this on a Bun upgrade.

Adding bounded context on 2026-10-01, with the same Bun and host, observed
three behaviours the SDK's reads and signal rely on:

- **A FIFO opened without blocking returns at once.** `openSync` with
  `O_RDONLY | O_NONBLOCK` on a FIFO with no writer returned in under a
  millisecond, and `fstatSync` reported it as a FIFO, not a file. The host
  therefore opens every read that way and refuses anything but a regular file.
  The front opens a `--context` document the same way. The command-seam tests
  `a_context_document_that_cannot_be_read_refuses_without_waiting_on_it` and
  `a_missing_required_source_refuses_and_names_it` each include a FIFO.
- **Strict decoding keeps a BOM.** `new TextDecoder("utf-8", { fatal: true,
  ignoreBOM: true })` threw a `TypeError` on invalid UTF-8 and kept a leading
  byte-order mark in the text, so the text a read returns matches the bytes it
  measured.
- **TERM listeners are countable.** `process.listenerCount("SIGTERM")` counted
  the listeners registered with `process.on`. The host's signal registers one,
  aborts on TERM, and exits only when no other listener remains, so a policy
  with a TERM handler of its own still owns its exit. The command-seam test
  `the_host_signal_aborts_when_the_deadline_stops_a_waiting_loader` covers
  both, and fails if the host's listener exits regardless.

<a id="installed-smoke"></a>
## Installed smoke

`task release:smoke` builds the release archives from the working copy and
runs `scripts/release-smoke.sh` over them. Each archive is extracted with its
target environment's own `tar` into a fresh prefix. `bin/grove`,
`bin/grove-llm` and `bin/harness-dispatch` must report the archive's version.
Then `crates/harness-dispatch/scripts/installed-smoke.sh` runs its cases
through the prefix's front and through a relative symlink to it. PATH is
`/usr/bin:/bin`, with no Bun or Node on it. The static TypeScript case uses an
interface, annotated bindings and a type-only import across a relative import,
and imports the embedded `harness-dispatch/sdk`. Its candidate fills every
slot. The case asserts `inspect`'s choice, expanded argv and worker path. A
`run` against a temporary state directory must exit with the fake harness's
42, and the harness must have received that argv, run ID, state directory and
cwd. `record show` must read that run back, so the bundled SQLite executes.
The computed TypeScript case exports an asynchronous `select`, annotated with
the SDK's types, that waits on a timer, reads the request and imports the
embedded `harness-dispatch/examples/dynamic`. `inspect` must report the
computed selection, with a reason carrying the request's kind and task
identity. A `run` whose explicit choice the policy refuses must exit 3 with
the policy's code and start nothing. The plain `run` must reach the fake
harness with its run ID, and `record show` must report the `select` form.

The C library instrument runs in `docker.io/library/centos:7@sha256:be65f488b7764ad3638f236b7b515b3678369a5124c47b8d32916d6487418ea4`,
CentOS Linux 7.9.2009, with `getconf GNU_LIBC_VERSION` required to be
`glibc 2.17`. Where Docker runs the target's architecture natively, that
userland runs as a container. Every Linux target also runs the same image's
filesystem under a pinned user-mode QEMU at its [CPU floor](#cpu-floor); on an
arm64 Docker that is x64's only route, as the next paragraphs explain. Either
way it has no network and runs as uid 1000. Its
positive control is two probes built by Zig 0.16.0 from one C source. The
first, built against glibc 2.17, must run. The second, which calls `getrandom`
and is built against glibc 2.25, must be refused for its symbol version. The
first shows the userland runs that architecture's binaries at all; only then
does the refusal of the second show the floor is enforced.

Observed on 2026-09-30 with archives of version 21.12.0, whose workers Bun
1.4.2 compiled from its digest-pinned runtimes, and Docker Desktop 28.1.1 on an
arm64 macOS host:

| Target | Where | Result |
|---|---|---|
| aarch64-apple-darwin | Natively, macOS 26.6.2 (Darwin 25.6.0), bsdtar 3.5.3 | Passed through both fronts |
| aarch64-unknown-linux-gnu | CentOS 7.9 aarch64, native to Docker's linux/arm64, GNU tar 1.26 | Passed through both fronts; control refused: ``/lib64/libc.so.6: version `GLIBC_2.25' not found`` |
| aarch64-unknown-linux-gnu | The same userland under QEMU 10.2.2 with `-cpu cortex-a53` ([CPU floor](#cpu-floor)), in a chroot inside an arm64 ubuntu:24.04 container | Passed through both fronts; glibc control refused as above; CPU control fired |
| x86_64-unknown-linux-gnu | CentOS 7.9 x86_64 under QEMU 10.2.3 user-mode emulation with `-cpu Nehalem` and a 2^47 guest base, in the same kind of chroot, GNU tar 1.26 | Passed through both fronts; glibc control refused as on arm64; CPU control fired |

On 2026-10-01, once computed selection landed, the same instruments ran the
static and computed cases from archives of version 21.12.0 whose workers
report build `77b6f49de68c…`, on the same host and Docker. Every row above
held for both cases: each passed through both fronts on every target, and the
glibc and CPU controls fired as before.

**Docker Desktop cannot run the x64 userland.** With Rosetta off, its VM runs
amd64 containers through a binfmt handler, `/usr/bin/qemu-x86_64` 8.1.5, which
gives x86-64 guests no vDSO. Under it, CentOS 7's own `cat` segfaults reading a
bash heredoc (`qemu: uncaught target signal 11`), and bash's own `read` hangs
on one. So the in-container scripts write their fixtures with `printf`. After
that change the x64 front ran, but its worker panicked with a segmentation
fault at `0xFFFFFFFFFF601000`, the address bash crashes at. The same worker
under the same QEMU in ubuntu:24.04 (glibc 2.39) inspected correctly, so the
defect is this emulator running a glibc-2.17 x86-64 userland. That class is
reported against Docker Desktop on Apple silicon
([docker/for-mac#5883](https://github.com/docker/for-mac/issues/5883),
[#6261](https://github.com/docker/for-mac/issues/6261)). Direct calls to
`time()` and `gettimeofday()`, and `time()` in a forked child, did work, so the
exact trigger is not established.

**Newer QEMU maps an x86-64 guest where no x86-64 kernel would.** QEMU 10.2.3
from `tonistiigi/binfmt` does supply a vDSO, and runs CentOS 7's bash heredoc.
There, though, the worker aborted with JavaScriptCore `MemoryExhaustion` in
`LocalAllocator::allocateSlowCase` at 33 MB RSS, with no resource limit set,
and QEMU's `-strace` showed no failing system call before it. The guest's own
`/proc/self/maps` showed the cause: libc, the stack and the vDSO at `0xffff…`,
with bit 47 set. An x86-64 kernel ends user space at `0x7fffffffffff`. Without
`-R`, QEMU 10.2.3 sets no address limit for a 64-bit guest
([`linux-user/main.c`](https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/linux-user/main.c)
sets `guest_addr_max` to `~0ul`), so on this 48-bit-address arm64 kernel it
maps the guest wherever the host does. QEMU 9.2.2, 10.0.4 and 10.1.3, and
Docker Desktop's own 10.2.3 build, do the same. Only 8.1.5 keeps the guest
below 2^47, and it has no vDSO. Reserving the guest's space with `-R` fails at
every size, with "Cannot allocate vsyscall page", because QEMU maps that page
at `0xffffffffff600000`. A guest base of 2^47 (`-B 0x800000000000`) maps the
host's upper half onto guest addresses `[0, 2^47)`, and with it the worker runs.

**So x64 runs under that QEMU, with that guest base, in a private binfmt_misc
instance.** `scripts/release-smoke.sh` copies `/usr/bin/qemu-x86_64` out of
`docker.io/tonistiigi/binfmt@sha256:400a4873b838d1b89194d982c45e5fb3cda4593fbfd7e08a02e76b03b21166f0`
(qemu-v10.2.3-68), and exports the floor image's amd64 filesystem.
The front gives the worker a scrubbed environment, so no `QEMU_*` variable
reaches the worker's emulator. The options instead come from a static arm64
interpreter, built by Zig, that runs QEMU with `-B 0x800000000000` before the
arguments binfmt_misc passes it. `scripts/release-smoke-qemu.sh` runs in
`docker.io/library/ubuntu:24.04@sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3`
with Docker's default security profile and no network. It unpacks the
filesystem and enters a new user and mount namespace whose uids and gids
0-65535 map to themselves. There it mounts a private binfmt_misc instance,
which a user namespace gets on kernel 6.7 and later (Docker's VM runs
6.10.14), and registers the interpreter for x86-64 ELF files. Docker's own
handler is left untouched. It then chroots into the userland as uid 1000. Before
the smoke test, the guest's own `/proc/self/maps` must show a vDSO and no
mapping at or above 2^47 but the vsyscall page. This QEMU build takes the
argument after the executable's path as its `argv[0]`, which is the layout
binfmt_misc's `P` flag passes.

Each assertion was seen to fail against a subject that violates it. For the
cases, on macOS: Bun on PATH, a wrong version, a front copied rather than
linked out of the prefix, the worker moved away, a harness exiting 0, and a
wrong argv or record expectation. For the floor instrument, in arm64
containers: ubuntu:24.04's glibc 2.39 was refused; with that check removed, the
2.25 probe ran and the control failed; an x64 target in an arm64 container was
refused for its machine. For the emulated x64 route: with no guest base, the
address check refused libc at `0xffff7f200000`, and with that check removed the
worker aborted with `MemoryExhaustion`. Docker Desktop's QEMU 8.1.5
(`tonistiigi/binfmt@sha256:a870fb6484bee975214c2987b135771bdb727c3e2b99552902b80a65bee72fe1`)
was refused for its missing vDSO. ubuntu:24.04's amd64 filesystem was refused
for glibc 2.39, and with that check removed the control failed because the
2.25 probe ran.

<a id="cpu-floor"></a>
### CPU floor

Bun 1.4.2's single x64 build targets Nehalem and "selects AVX2/AVX-512 code
paths at runtime when the CPU supports them"
([installation](https://github.com/oven-sh/bun/blob/bun-v1.4.2/docs/installation.mdx)).
A newer CPU therefore runs code that a floor CPU never reaches. So each Linux
target's installed smoke test also runs under user-mode QEMU with the floor's
model, in the glibc-2.17 userland above: `-cpu Nehalem` for x64 and `-cpu
cortex-a53` for arm64, the models Bun's own baseline verification emulates.
The x64 target already ran under QEMU, so its interpreter gains `-cpu Nehalem`
beside the guest base, and its one emulated run covers both floors. Linux arm64
runs natively in its container, then again emulated. That second route needed
two things the x64 route did not.

**An arm64-host `qemu-aarch64`.** The pinned `tonistiigi/binfmt` arm64 image
carries only foreign architectures' emulators, such as `qemu-x86_64` and
`qemu-arm`. Debian's `qemu-user` `1:10.2.2+ds-1` for arm64 has a static-pie
`qemu-aarch64`. snapshot.debian.org serves it permanently at
[its SHA-1](https://snapshot.debian.org/file/4b7f47627ad6e57745d33d1108b893970e19ebf2),
and it is checked against SHA-256
`f8bf89dacd04e66a1e34526bf6eb9b4eb1b4da6205d0d820381f0c87cb0db8bb`. Unlike
tonistiigi's build, upstream QEMU takes a guest's `argv[0]` from after the
executable's path only when its own auxiliary vector carries
`AT_FLAGS_PRESERVE_ARGV0`
([`linux-user/main.c`](https://gitlab.com/qemu-project/qemu/-/blob/v10.2.2/linux-user/main.c)).
The kernel sets that for the interpreter it starts, not for a QEMU that the
interpreter executes. So the arm64 interpreter passes `-0 argv0` instead.

**A registration that does not match its own tools.** An aarch64 registration
on an aarch64 host also matches the interpreter and QEMU themselves. Without an
exemption, the helper's `chroot` failed with "Too many levels of symbolic
links": each interpretation invoked the interpreter again until `exec` returned
`ELOOP`. QEMU's aarch64 mask requires ELF ident bytes 8 to 15 to be zero
([`qemu-binfmt-conf.sh`](https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/scripts/qemu-binfmt-conf.sh)),
and Linux's ELF loader never reads them. So the copies in the userland get
`EI_ABIVERSION` 1. The helper's own executables still match, so after
registration its `chroot` runs emulated too, and a `/qemu` link in the helper's
root lets the interpreter find QEMU from there.

The CPU control is a probe built by Zig from one C source. It prints a line,
executes one instruction beyond the model, and prints again: `vpaddd` on `ymm`
registers (AVX2) on x64, and `ldadd` (the Armv8.1 LSE atomics) on arm64. Under
the registered interpreter it must be killed by SIGILL between the two lines.
Under a twin interpreter, identical but for `-cpu max`, it must run to the end.
That shows the instruction is valid and that this QEMU executes it, so the
refusal is the model's. A model that silently accepted newer instructions would
otherwise pass everything.

The control shows only that a matching executable runs under the model. An
archive executable whose ident bytes 8 to 15 were not zero would instead run on
the helper's own CPU and pass untested; on arm64 that CPU is the host's, which
has LSE. So the host lists every ELF file in the archive, and the helper holds
each one's first 20 bytes against the registration's magic and mask. The front
and the worker must be among them. All four ELF files in each Linux archive
match.

Observed on 2026-09-30, on the finished source of `cpu-floor-k19`. `task
release:smoke` rebuilt all three archives of version 21.12.0 (SHA-256
`b451bf73…` macOS arm64, `381bde71…` Linux arm64, `5ca21891…` Linux x64;
worker build `bdd58ee3…`, Bun 1.4.2) and passed every target through both
fronts. macOS arm64 ran natively, and Linux arm64 in its native container, then
under QEMU at the Cortex-A53. Linux x64 ran under QEMU at Nehalem. In each
emulated run, all four ELF files in the archive matched the registration, the
guest's address space passed its check, and the glibc control refused the 2.25
probe. Under each model the CPU probe printed `cpu probe started`, then died
with `qemu: uncaught target signal 4 (Illegal instruction)` and exit 132; under
`-cpu max` it ran to the end. The task exited 0, and its nine subjects,
including the pinned Debian package, had identical digests before and after.
After a comment-only edit to `scripts/release-smoke-qemu.sh`,
`scripts/release-smoke.sh` passed the same archives again with the committed
scripts.

Each CPU-floor assertion was seen to fail against a subject that violates it,
on a scratch copy of the scripts; the repository's scripts, archives and cached
package had identical digests afterwards. With `-cpu max` as the model, x64 and
arm64 each refused, because the probe was not killed. With the probe's
instruction replaced by one no CPU executes (`ud2`, `udf #0`), each refused,
because `-cpu max` could not run it either. An arm64 archive whose worker
carries `EI_ABIVERSION` 1 was refused by name. With the match check removed,
that same archive passed every other check, so the escape it guards against is
real. An enumeration that matched no ELF file was refused. A mutated package
digest was refused with the digest found. An arm64 address limit of 2^40
refused a guest mapping at `0xffffac600000`. An archive with a stray file
failed the manifest before extraction.

The kernel is not observed. A container and user-mode emulation both run on
Docker's 6.10.14 kernel. The kernel floor is Bun's documented range, as the
[specification](../../specs/harness-selection-and-execution.md#delivery)
records.

<a id="primary-runtime-references"></a>
## Primary runtime references

- [Bun installation](https://bun.sh/docs/installation) documents glibc 2.17 and
  the macOS arm64/Linux arm64/Linux x64 distributions. This is a runtime claim,
  not a substitute for executing the shipped artifact at the compatibility floor.
  For 1.4.2 it also documents a single x64 build targeting Nehalem (SSE4.2), a
  macOS 13.0 minimum, and Linux kernels "as old as 3.10 (RHEL 7) with graceful
  degradation". Bun's README for the same tag instead gives "the minimum is 5.1".
  Neither kernel claim is observable with a container or user-mode emulation; the
  specification records the kernel floor as documented rather than executed.
  Bun's own baseline verification emulates `-cpu Nehalem` (x64) and
  `-cpu cortex-a53` (arm64).
- [Bun standalone executables](https://bun.sh/docs/bundler/executables) documents
  compilation with an included runtime, target selection, autoload switches,
  `BUN_OPTIONS` and `BUN_BE_BUN`. These startup inputs motivate the native front
  process and fresh worker environment. The installed 1.4.2 help was also checked
  for all four explicit no-autoload switches.
- [Bun TypeScript](https://bun.com/docs/typescript) describes its TypeScript
  execution support. Policy type checking belongs to package checks; runtime
  result validation remains required.

No claim is made here about current model quality, alternate runtimes' measured
latency, or final archive size. No Linux runtime smoke test was performed in this
design leaf.

<a id="source-grounding"></a>
## Source grounding

Tier 2 graph verification used project
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`,
generation `2026-09-29T11:18:53Z`. These are bounded positive findings:

- [Session expansion](../../../crates/grove-loop/src/session_config.rs) currently
  supplies prompt, session name, worktree and repository, with exact native
  argument boundaries. Its existing integration test exercises spaces and shell
  punctuation. The three new lifecycle slots are proposed, not existing API.
- [The loop driver](../../../crates/grove-loop/src/loop_driver.rs) expands the
  command from its authoritative selection, then activates the epoch and calls
  the observed runner. Inbound/outbound graph tracing and the exact launch
  snippet were inspected. The existing three scrubbed values are signal path and
  two legacy harness PID values.
- [Tree lifetime](../../../crates/grove-loop/src/observation.rs) is a pinned open
  directory, compared by device/inode while pinned. Its numbers alone are
  unsuitable as permanent provenance identity. The design therefore carries
  provenance as a dispatch run ID and derives none from tree identity.
- [Release construction](../../../scripts/release-build.sh),
  [the target list](../../../scripts/release-common.sh) and
  [Homebrew installation](../../../scripts/templates/grove.rb.tmpl) currently
  build three archives and install the two existing commands. Linux is linked
  against a 2.17 glibc floor. The worker/layout and installed smoke checks are
  additions the implementation must make.

Coverage reported no recorded source gaps for the inspected Rust files and
matching source metadata. Docs and scripts are excluded from the graph and were
read directly. Task notes and glossary had changed/untracked metadata and were
read directly. No claim of repository-wide completeness is based on graph
absence. The command-seam and PTY integration tests are retained as the agreed
acceptance instruments; this design did not replace them with mocked internals.
