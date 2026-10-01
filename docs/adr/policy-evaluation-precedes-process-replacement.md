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
tsconfig autoloading. package.json autoloading is the one switch left on, so
that a package whose entry `main` or `exports` declares loads. It could not be
on while the worker stayed where it started. The worker then read each
`package.json` at or above its own directory, which the front creates under
the caller's TMPDIR. One there stalled a plain policy to its deadline, and
another answered an import from a module with no file location.
That probe also found what the private directory does not control. Its ancestors
are not private, and with every switch off a `node_modules` above it answered a
bare import from such a module. So the worker starts in the private directory
and then moves to `/` before it loads any policy code. The runtime reads its
startup files from where a process starts and resolves such a module from where
the process is, and `/` has no ancestors. With the move, a `node_modules` above
the directory the worker started in answered nothing, and nor did a
`package.json` there, whether it mapped an import or would stall a worker that
opened it.
`/` itself still
answers, as it does for a module in a file. The move costs one control. The
runtime loads dotenv files again for each VM it starts, from where the process
then is, so a native `Worker` a policy starts would read `/.env`, where one
started by a worker that stayed put read the empty directory. Only the dotenv
switch keeps that out, and it was seen to. That was accepted: the reach the
move closes was open in the shipped build to anyone who could write TMPDIR,
and what the move leaves to one control is a file only the owner of `/` can
write, behind a switch a command-seam test holds. Turning package.json
autoloading on has a limit of its own. An entry's own `package.json` takes
part as it does in Node when it is well formed, and one that does not parse is
passed over for the next one above. An `imports` alias there that names a
registered specifier is looked up in `node_modules`, so beside a shadow it
loads the shadow. A registered specifier an import names still resolves to its
embedded module. Other classes were not seen to fire at all. The
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
