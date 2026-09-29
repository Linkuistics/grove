# Policy evaluation precedes process replacement

`harness-dispatch` uses a Rust front process and an installed, matching Bun-compiled
policy worker. Rust retains the original launch environment and terminal, gives
the worker a bounded request without completion authority, reaps it, durably
records the selected handoff, then replaces itself with the harness. The
[area specification](../specs/harness-selection-and-execution.md) owns the
protocol, trust controls, cancellation and evidence contracts.

The [foreground-job contract](the-launched-child-is-a-job.md) makes keeping the
wrapper as a post-launch supervisor the wrong boundary. Replacing it preserves
Grove's ownership of the real child, but sacrifices automatic exit/usage
observation. A handoff receipt therefore remains an attempt until external
evidence says otherwise. Original-creator registration uses such evidence or an
explicit owner declaration; it cannot be reconstructed from an attempted exec.

Bun supplies TypeScript and ordinary filesystem/network computation without
maintaining a new JavaScript standard library. Compiling the worker keeps a
separately installed runtime out of the launch contract. Its costs are a larger
installation, a second build tool, runtime notices, and per-target execution
checks. Native probes on one macOS arm64 host saw several ambient classes fire and
the chosen controls hold. cwd dotenv and bunfig preloads fired in a default build
and stayed inert under both the no-autoload build and the private worker cwd. A
`BUN_OPTIONS` preload fired even in the guarded build, which is why Rust scrubs
the environment before start. A `node_modules` shadow beside the entry lost to a
registered embedded module. tsconfig `paths` fired only in a build that enables
tsconfig and package.json autoloading. Other classes were not seen to fire at
all. The
[runtime evidence](../design/harness-selection-and-execution/runtime-evidence.md)
records exactly which, the limits of each observation and the primary runtime
documentation.

Considered alternatives:

- **One Bun executable owns everything.** Rejected because executable
  configuration would start in the same process holding completion authority,
  and runtime startup hooks would run before application-level environment
  scrubbing. A native boundary is needed before evaluation. Reopen if a runtime
  offers a verified equivalent startup and process-replacement contract.
- **Embed a small JavaScript engine into Rust.** This makes distribution smaller
  but makes the package own TypeScript transformation, module loading and the
  host APIs needed for filesystem and network policy. Reopen if the compiled
  worker cannot meet the supported-target floor or its delivery cost outweighs
  those obligations. Do not silently fall back to a system runtime.
- **Keep a supervisor after launching the harness.** It could collect exits,
  but would replace the existing direct-child contract to obtain measurements
  the first release explicitly permits to remain unknown. Reopen only with an
  independently agreed supervisor/terminal design.

The [policy ownership decision](harness-selection-is-owned-by-policy.md) remains
separate: changing the TypeScript host need not move selection rules into Grove.
