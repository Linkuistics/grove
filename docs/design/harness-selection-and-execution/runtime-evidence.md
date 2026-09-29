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
created the preload sentinel: the fixture demonstrably exercised the risk.
This does not exhaust module-resolution/shadowing cases.

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

<a id="primary-runtime-references"></a>
## Primary runtime references

- [Bun installation](https://bun.sh/docs/installation) documents glibc 2.17 and
  the macOS arm64/Linux arm64/Linux x64 distributions. This is a runtime claim,
  not a substitute for executing the shipped artifact at the compatibility floor.
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
