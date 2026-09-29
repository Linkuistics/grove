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
  punctuation. The four new lifecycle slots are proposed, not existing API.
- [The loop driver](../../../crates/grove-loop/src/loop_driver.rs) expands the
  command from its authoritative selection, then activates the epoch and calls
  the observed runner. Inbound/outbound graph tracing and the exact launch
  snippet were inspected. The existing three scrubbed values are signal path and
  two legacy harness PID values.
- [Tree lifetime](../../../crates/grove-loop/src/observation.rs) is a pinned open
  directory, compared by device/inode while pinned. Its numbers alone are
  unsuitable as permanent provenance identity. The proposed dispatch UUID is
  evidence identity, not reuse of the advisory observer or epoch as authority.
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
