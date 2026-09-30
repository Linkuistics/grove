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

The C library instrument runs in `docker.io/library/centos:7@sha256:be65f488b7764ad3638f236b7b515b3678369a5124c47b8d32916d6487418ea4`,
CentOS Linux 7.9.2009, with `getconf GNU_LIBC_VERSION` required to be
`glibc 2.17`. The container has no network and runs as uid 1000. Its positive
control is two probes built by Zig 0.16.0 from one C source. The first, built
against glibc 2.17, must run. The second, which calls `getrandom` and is built
against glibc 2.25, must be refused for its symbol version. The first shows the
container runs that architecture's binaries at all; only then does the refusal
of the second show the floor is enforced.

Observed on 2026-09-30 with archives of version 21.12.0, whose workers Bun
1.4.2 compiled from its digest-pinned runtimes, and Docker Desktop 28.1.1 on an
arm64 macOS host:

| Target | Where | Result |
|---|---|---|
| aarch64-apple-darwin | Natively, macOS 26.6.2 (Darwin 25.6.0), bsdtar 3.5.3 | Passed through both fronts |
| aarch64-unknown-linux-gnu | CentOS 7.9 aarch64, native to Docker's linux/arm64, GNU tar 1.26 | Passed through both fronts; control refused: ``/lib64/libc.so.6: version `GLIBC_2.25' not found`` |
| x86_64-unknown-linux-gnu | CentOS 7.9 x86_64 under Docker's emulation | Not executed at the floor; control refused as on arm64 |

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

A newer QEMU, 10.2.3 from `tonistiigi/binfmt`, does supply a vDSO. It was
registered in a private binfmt_misc instance inside a user namespace of a
privileged arm64 container, leaving Docker's own handler untouched, and run on
a chroot of the same image's amd64 root filesystem. It ran CentOS 7's bash
heredoc and reported glibc 2.17. There, though, the worker aborted with
JavaScriptCore `MemoryExhaustion` at 33 MB RSS, with no resource limit set. So
no x86-64 environment on this host has yet executed the x64 archive at the
floor.

Each assertion was seen to fail against a subject that violates it. For the
cases, on macOS: Bun on PATH, a wrong version, a front copied rather than
linked out of the prefix, the worker moved away, a harness exiting 0, and a
wrong argv or record expectation. For the floor instrument, in arm64
containers: ubuntu:24.04's glibc 2.39 was refused; with that check removed, the
2.25 probe ran and the control failed; an x64 target in an arm64 container was
refused for its machine.

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
