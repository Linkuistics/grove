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
checks.

Bun also takes ambient input as it starts and as it resolves imports, and each
control answers a class that fires without it. cwd dotenv and bunfig preloads
fire in a default build. They stay inert under a build without dotenv and
bunfig autoloading, and under a private empty start directory, each alone. A
`BUN_OPTIONS` preload fires even in the guarded build, which is why Rust scrubs
the environment before start. Bun's runtime transpiler cache, under HOME, runs
an altered cached output of an imported file in place of the file. Only a
variable set before start turns the cache off, so Rust sets it. A
`node_modules` shadow beside the entry loses to a registered embedded module.
tsconfig `paths` fire only in a build that enables tsconfig autoloading.

package.json autoloading is the one switch left on, so that a package whose
entry `main` or `exports` declares loads. It can be on only because of where
the worker runs. The front creates the private start directory under the
caller's TMPDIR, and that directory's ancestors are not private. A worker that stayed there would
read each `package.json` at or above it on every evaluation. One there can
stall a plain policy to its deadline, and another can answer an import from a
module with no file location. With every switch off, a `node_modules` above
the start directory answers a bare import from such a module too. So the
worker starts in the private directory and moves to `/` before it loads any
policy code. The runtime reads its startup files from where a process starts
and resolves such a module from where the process is, and `/` has no
ancestors. After the move, a `node_modules` above the start directory answers
nothing, and nor does a `package.json` there, whether it maps an import or
would stall a worker that opened it. `/` itself still answers, as it does for
a module in a file.

The move costs one control. The runtime loads dotenv files again for each VM
it starts, from where the process then is. So a native `Worker` a policy starts
would read `/.env`, where one started by a worker that stayed put would read
the empty directory. Only the dotenv switch keeps that out. The cost is
accepted: the reach the move closes would be open to anyone who can write
TMPDIR, and what the move leaves to one control is a file only the owner of `/` can
write, behind a switch a command-seam test holds.

package.json autoloading has a limit of its own. An entry's own `package.json`
takes part as it does in Node when it is well formed, and one that does not
parse is passed over for the next one above. An `imports` alias there that
names a registered specifier is looked up in `node_modules`, so beside a shadow
it loads the shadow. A registered specifier an import names still resolves to
its embedded module.

Each claim above was observed on one macOS arm64 host with one Bun version, and
some classes have no known firing configuration at all. The
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
  but would replace the direct-child contract to obtain measurements that are
  explicitly permitted to remain unknown. Reopen only with an
  independently agreed supervisor/terminal design. The owner wants that design
  eventually: dispatch would handle the launched harness's completion signal
  and so become the process wrapper for one run of an interactive harness.
  That moves the runner's completion channel, supervision, kill escalation,
  terminal handover and confinement under dispatch. Until then the runner
  stays a domain-free unit separable from Grove's loop, and a confined launch
  selects through a capability that does not assume the `exec` handoff.

The [policy ownership decision](harness-selection-is-owned-by-policy.md) remains
separate: changing the TypeScript host need not move selection rules into Grove.
