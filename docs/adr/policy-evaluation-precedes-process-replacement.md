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
evidence says otherwise. The handoff does export the run's identity, so the
launched session can name its own run later. That is how a review finds its
creator without the tool reconstructing one from attempts
([a review carries its creator reference](a-review-carries-its-creator-reference.md)).

Bun supplies TypeScript and ordinary filesystem/network computation without
maintaining a new JavaScript standard library. Compiling the worker keeps a
separately installed runtime out of the launch contract. Its costs are a larger
installation, a second build tool, runtime notices, and per-target execution
checks. Native probes on one macOS arm64 host saw several ambient classes fire and
the chosen controls hold. cwd dotenv and bunfig preloads fired in a default build
and stayed inert under both a build without dotenv and bunfig autoloading and the
private worker cwd. A
`BUN_OPTIONS` preload fired even in the guarded build, which is why Rust scrubs
the environment before start. A later control found a second input of that
kind. Bun's runtime transpiler cache, under HOME, ran an altered cached output
of an imported file in place of the file, and only a variable set before start
turns the cache off, so Rust sets it. A `node_modules` shadow beside the entry lost to a
registered embedded module. tsconfig `paths` fired only in a build that enables
tsconfig autoloading. package.json autoloading stays off as well, at a cost: a
package whose entry `main` or `exports` declares does not load. With it on, the
worker read each `package.json` at or above its own directory, which the front
creates under the caller's TMPDIR. One there stalled a plain policy to its
deadline, and another answered an import from a module with no file location.
That probe also found what the private directory does not control. Its ancestors
are not private, and with every switch off a `node_modules` above it answered a
bare import from such a module. So the worker starts in the private directory
and then moves to `/` before it loads anything. The runtime reads its startup
files from where a process starts and resolves such a module from where the
process is, and `/` has no ancestors. With the move, a `node_modules` above
the directory the worker started in answered nothing, and nor did a
`package.json` there with the switch on for an experiment. Other classes were
not seen to fire at
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
