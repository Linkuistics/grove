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
output. The prototype is not an implementation to adopt. The
[ambient authority](#ambient-authority) section records that reproduction.

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

<a id="ambient-authority"></a>
## Ambient authority

On 2026-10-01, `ambient-authority-k30` proved each hostile class against the
shipped launcher. The host was macOS 26.6.2 (Darwin 25.6.0) on arm64, with Bun
1.4.2 and worker build `ad1b0271bbc3…`. Each class stayed inert through the
checkout's front and its shipped worker. In the same test, the class's firing
configuration was seen to fire. `task dispatch:probes` builds the probe
builds from the same source, and each reports `probe-<name>-<source digest>`,
which no front accepts. So a probe is driven directly, by a test that plays
the front's side of the protocol, and a probe placed at an installation's
worker path refuses with exit 5.

| Class | Through the shipped front | Firing configuration, seen to fire | Command-seam test |
|---|---|---|---|
| cwd policy entry | No cwd, parent, XDG or variable-named entry ran | The same file named by `--config` ran | `authority::no_cwd_search_or_environment_variable_selects_an_entry` |
| cwd `.env`, `.env.local` and bunfig preload | Neither variable set, no preload, under `inspect` and `run` | The `autoload` probe in the hostile directory loaded both files and ran the preload | `hostile::a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe` |
| `BUN_OPTIONS` preload | No preload, and the policy selected | The shipped worker started directly with it ran the preload | `hostile::bun_runtime_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly` |
| `BUN_BE_BUN` | The worker identified itself and the policy selected | The shipped worker started directly with it ran `-e` code as Bun | the same |
| `node_modules/harness-dispatch` shadow beside an admitted entry | Each of the four documented specifiers resolved to its embedded module | The `unregistered` probe loaded every shadow | `hostile::every_documented_specifier_resolves_to_its_embedded_module_beside_a_package_shadow` |
| tsconfig `paths` beside an admitted entry | The alias did not apply, and the import refused | The `tsconfig` probe applied it and loaded the aliased module | `hostile::tsconfig_paths_beside_an_admitted_entry_stay_inert_and_fire_under_the_tsconfig_probe` |
| Worker location | No decoy on PATH, in the cwd, named by a variable or at `argv[0]`'s prefix ran | A decoy at the real layout path, or beside a front really in that prefix, ran | `worker::a_front_without_its_worker_refuses_and_no_ambient_decoy_substitutes`, `worker::an_argv0_naming_another_prefix_never_relocates_the_worker` |
| Runtime transpiler cache, under HOME or a granted `XDG_CACHE_HOME` | The front's worker wrote no cache entry, and beside an altered entry in each place it ran the admitted file | The shipped worker started directly cached the policy under HOME, and once that entry's output was altered, ran the altered output from HOME and from a copy under `XDG_CACHE_HOME` | `hostile::the_runtime_transpiler_cache_stays_inert_through_the_front_and_fires_in_the_worker_started_directly` |
| `NODE_PRESERVE_SYMLINKS` and `NODE_CHANNEL_FD` | The caller's values stayed out of the worker, and each grant refused | The shipped worker started directly with each: a helper behind a directory symlink resolved its `dep` beside the link, and a module's `process.send` put Bun's JSON (`{"vi…`) where the policy frame belonged | `hostile::node_resolver_and_channel_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly` |

The dotenv case also ran the other two corners of its square. The shipped
worker, started directly in the hostile directory, and the `autoload` probe in
an empty directory, like the front's private one, were each inert. So each of
the two controls holds on its own, as the [integration probe](#integration-probe)
saw. A mutated front showed the same from the other side. With the worker
started in the caller's cwd, the case stayed green. With the shipped build's
dotenv and bunfig autoloading on, the front stayed inert, and only the
direct-start corner failed. With both, the front arm failed.

`evaluation-boundary-k54` added the last two rows on the same host and worker
build, triaging `evaluation-boundary-k53`. Before its fix, the front's worker
wrote `Library/Caches/bun/@t@/<input hash>.pile` under the caller's HOME for an
imported file of 4 KiB or more, and one altered entry changed the selection
while inspection reported the admitted file's unchanged `sha256`. The front now
sets `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`. That is the only switch that turns
the cache off, and Bun reads it before `XDG_CACHE_HOME` and HOME (bun-v1.4.2
`src/jsc/RuntimeTranspilerCache.rs`, `really_get_cache_dir`). The control
patches Bun 1.4.2's entry layout, version 28, and says so when it meets
another, so a Bun upgrade re-derives it. Before the fix, the three
`NODE_*` grants were admitted. `NODE_PRESERVE_SYMLINKS` changed the helper's
resolution, and only through a symlinked directory; a symlinked file resolved
alike either way. With `NODE_CHANNEL_FD=3`, the policy ended in
`protocol_error` or `worker_failed`, so it failed closed rather than
selecting. Each fixture is inert without its variable, which is the control's
other corner.

Some classes have no known firing configuration, and none is counted. A
`~/.bunfig.toml` preload did not fire under the `autoload` probe with that HOME.
tsconfig `paths` in the caller's cwd did not apply under the `tsconfig` probe
run there. `hostile::classes_with_no_known_firing_configuration_are_reported_not_counted`
keeps checking each, and fails if one begins to fire.
[The worker's directory](#worker-directory) added another and changed how the
cwd tsconfig one is checked.

The environment has its own instrument, `tests/environment.rs`. A policy
records the environment it was given and that of a child it spawns. Grove's
`GROVE_SIGNAL_FILE`, `GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` reached
neither. The harness received all three, unchanged. Granted by
`--policy-env GROVE_SIGNAL_FILE`, the channel reached both the worker and its
child, which is the control. A policy that flooded both streams at import, in
`loadContext` and in `select`, with a well-formed protocol frame and a JSON
report naming another candidate, left `inspect --json` one document, the
selection its own, and `run --json` one notice line. That is
`hostile::a_policy_flooding_both_streams_leaves_json_output_and_the_protocol_intact`.

**The worker reads no `package.json` at run time.** The worker is compiled with
`--no-compile-autoload-package-json`. Through the shipped front, a
`node_modules` package with an `index.js` loaded. One whose `package.json` has
`"main": "./lib/entry.js"`, or only an `exports` map, refused with "Cannot find
package". Plain `bun` loaded all three. That is why the shadow fixture is laid
out as files. `package-entry-resolution-k52` asked whether the limitation could
go, and [package entry resolution](#package-entry-resolution) records why it
stays.

The classes were observed on this one host. The installed smoke test runs no
hostile fixture, so the Linux targets are unmeasured for these controls. Each
control there is the same compile switch, Rust scrubbing or JavaScript
registration.

<a id="package-entry-resolution"></a>
## Package entry resolution

On 2026-10-01, `package-entry-resolution-k52` asked whether the worker could be
compiled with `--compile-autoload-package-json`, the other three switches still
off, without any hostile class firing. It could not, so the switch stays off and
the limitation is the contract. The host was macOS 26.6.2 (Darwin 25.6.0) on
arm64, with Bun 1.4.2.

**What the switch reads, from the `bun-v1.4.2` source.** The compile switch
decides one graph flag, `DISABLE_AUTOLOAD_PACKAGE_JSON`
([`build_command.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/cli/build_command.rs)).
At start the worker turns that flag into the resolver option
`load_package_json` ([`bun.js.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bun.js.rs),
`apply_standalone_runtime_flags`). The option has one reader,
`dir_info_uncached` in
[`resolver.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs),
which decides whether a directory's own `package.json` is read, whole, and
parsed into the record the resolver keeps for that directory. Everything a
`package.json` does at run time hangs off that record: a package's `main` and
`exports`, the nearest enclosing `imports` map, a module's import of its own
package's `name`, and the module `type` (`load_node_modules` in the same file,
and the module-type reads in
[`jsc_hooks.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/jsc_hooks.rs)).

The resolver builds a record for more directories than an import names. For an
importer inside the compiled executable, and for any module with no file
location, it takes the directory the process started in as the importing
directory (`resolve_and_auto_install`: "module resolution is never relative to
our special /$bunfs/ directory"). Even for an import of an absolute path it
then builds that directory's record, and with it each ancestor's
(`resolve_without_symlinks`). So the worker's own import of the policy entry
records every directory from the worker's to the root.

Four neighbouring facts come from the same files. tsconfig has its own gate,
`load_tsconfig_json`, in `dir_info_uncached`. A standalone executable never
uses the package manager, whatever a `package.json` lists
(`use_package_manager`). The conditions an `exports` or `imports` map is read
under are the target's `bun` and `node`, `import` or `require`, `node-addons`
and `default`, plus any `--conditions` argument
([`options.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/options.rs),
`default_conditions_map` and `ESMConditions::init`); nothing derives one from
`NODE_ENV`. A `browser` map applies only to the browser target.

**What a probe saw.** Scratch builds of the shipped worker source, one per
switch combination, were each driven directly, as a
[probe build](#ambient-authority) is. Each ran under the front's base
environment, in a private empty directory unless the case says otherwise.

| Case | All four off, as shipped | package.json on, the other three off | Other builds |
|---|---|---|---|
| A package with only an `index.js` | Loaded | Loaded | |
| A package declared by `main`, by `exports`, by an `exports` subpath or by `exports` conditions, and one reaching its own file through its `imports` map | "Cannot find package" | Loaded. The condition chosen was `bun`, also with `NODE_ENV` set to `production` or to `development` | |
| A `package.json` in the worker's cwd with an `imports` map, a name its `exports` answer and a `node_modules` beside it, for a policy file elsewhere | Did not fire | Did not fire | Did not fire with tsconfig on as well, nor with dotenv and bunfig on as well |
| The same `package.json`, for an entry in that directory | Did not fire | Fired: the `#` name and the package's own name both resolved | |
| A `harness-dispatch` shadow declared by `exports`, in `node_modules` and as the entry's own package | "Cannot find package" | Each registered specifier resolved to its embedded module, and the unregistered name loaded the shadow | Without the registration, every specifier loaded the shadow |
| tsconfig `paths` beside the entry, with and without a `package.json` there | Did not fire | Did not fire | Fired with tsconfig autoloading on, alone and with package.json |
| tsconfig `paths` in the cwd only | Did not fire | Did not fire | Did not fire with tsconfig on |
| cwd `.env` and bunfig preload, the worker run there beside a `package.json` | | Inert | Fired with dotenv and bunfig on as well |
| A `package.json` beside the entry naming `is-odd` as a dependency, and no `node_modules` | Refused, no cache | Refused, no cache | Plain `bun` fetched it into the private HOME |
| The entry's own `package.json`: an `imports` map, and a name its `exports` answer where a `node_modules` package of that name sits beside the entry or nearer | The `#` name refused, and the `node_modules` package loaded | The `#` name resolved, and the own name resolved through `exports`, ahead of `node_modules` | |
| The entry's own `package.json` aliasing `#sdk` to `harness-dispatch/sdk`, beside a `node_modules/harness-dispatch` shadow | The alias refused, and the registered name resolved to the embedded module | The alias loaded the shadow, and the registered name still resolved to the embedded module | |

**What the worker's own directory lets in.** The leaf's one in-session review,
a fresh context given the builds and the source, attacked the claim that no
file outside an entry's own directories takes part. It found the directory
substitution above, and each finding was reproduced before it was recorded
here. The front creates the worker's private directory under the caller's
TMPDIR, so the fixtures below sit in TMPDIR, one level above the worker.

| Case in TMPDIR | All four off, as shipped | package.json on |
|---|---|---|
| A `package.json` with an `imports` map and a name its `exports` answer, for a policy file's own imports | Did not fire | Did not fire |
| The same, for a module with no file location that the policy imports: a `data:` URL, a `blob:` URL, or a virtual module the policy registers | Did not fire | Fired: the `#` name and the own name both ran the planted module |
| A `node_modules` package there, for the same three kinds of module | Fired | Fired |
| A `node_modules/harness-dispatch` there, for a `data:` module | | The registered `sdk` resolved to the embedded module, and an unregistered name loaded the shadow |
| A sparse `package.json` of 4097 MiB, for a plain routed policy | Selected at once | No answer. Through the front, `selection_timeout`, exit 124, at a five-second deadline |

The second, third and fifth rows were seen through the checkout's front as
well as by driving a build, with TMPDIR naming the planted directory. A
`node:vm` script and `createRequire` with a relative name did not resolve
anything in either build.

Consequences for the design:

- The switch stays off. With it on, a `package.json` that no entry admits
  takes part in every evaluation: any at or above the worker's directory. It
  can stall a selection, which then refuses and launches nothing, and it can
  answer imports from a module with no file location.
- One reach predates the switch and was open with it off. A `node_modules` at
  or above the worker's directory answered a bare import from a module with no
  file location. The worker's directory is private and empty, and its
  ancestors are neither. A module in a file is not affected, because its
  imports resolve from its own directory. [The worker's directory](#worker-directory)
  records the repair, and `package-json-autoloading-k62` turns the switch on
  after it.
- The two tsconfig switches are independent. The
  [integration probe](#integration-probe) saw `paths` fire with tsconfig and
  package.json autoloading both on, and never tried either alone. The
  `tsconfig` probe build now turns on tsconfig autoloading only, so it differs
  from the shipped worker by one switch.
- Three things would change inside what an entry admits once the switch is on,
  and `package-json-autoloading-k62` must state each. The nearest
  `package.json` at or above an importing module supplies that module's
  `imports` map. A bare import of that package's own `name` resolves through
  its `exports`, ahead of any `node_modules` package of the name, a nearer one
  included. An `imports` alias whose target is a registered specifier does not
  reach the embedded module: it loaded a `node_modules/harness-dispatch`
  shadow.

The command-seam test of the contract is
`authority::a_package_resolves_by_its_file_layout_and_never_through_its_package_json`.
`scripts/dispatch.sh` now states the shipped switches once and derives each
probe's from them.

This is one host and one Bun version. A Bun upgrade rereads the three resolver
functions named above and reruns the cases.

<a id="worker-directory"></a>
## The worker's directory

On 2026-10-01, `worker-directory-chain-k61` closed the reach that
[package entry resolution](#package-entry-resolution) found open. The worker
still starts in the front's private empty directory, and now moves to `/`
before it registers or loads anything. The host was macOS 26.6.2 (Darwin
25.6.0) on arm64, with Bun 1.4.2. The command-seam tests below ran against
worker build `3261c425f866…`.

**What decides the directory, from the `bun-v1.4.2` source.** The leaf that
found the reach left open whether the runtime fixes the directory at start. It
does not. `process.chdir` reaches `set_process_cwd` in
[`VirtualMachine.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/VirtualMachine.rs)
(through `set_cwd` in
[`node_process.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/node/node_process.rs)),
which changes the process's directory and rewrites the filesystem singleton's
`top_level_dir`. That is the value `resolve_and_auto_install` in
[`resolver.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs)
takes, at each resolution, for an importer inside the executable, one with no
source directory, and one whose source directory is not absolute. So such a
module resolves from the directory the process is in, not the one it started
in.

The other half is seen and not read. `run_env_loader` in
[`transpiler.rs`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/transpiler.rs)
loads `.env` files from `top_level_dir` when it is called, and skips that
directory entirely when dotenv loading is disabled, as it is in the shipped
build. Its caller on a compiled executable's start path was not traced. What
the `autoload` probe showed is below: the files of the directory it starts in
load, and those of a directory it is later moved into do not.

**What was seen through the checkout's front.** Each fixture sat in the
caller's TMPDIR, one level above the worker's private directory. The first
column is the shipped worker. The second is the same source with the move
replaced by a no-op, built for the experiment. The third and fourth had
package.json autoloading turned on as well, for the experiment only; the
switch went back to off afterwards.

| Case in TMPDIR | Moves, as shipped | Does not move | Moves, package.json on | Does not move, package.json on |
|---|---|---|---|---|
| A `node_modules` package, for a `data:` module, a `blob:` module and a virtual module the policy registers | "Cannot find package" for each, exit 3 | Each loaded | "Cannot find package" for each | Each loaded |
| The same, after the policy itself moved back with `process.chdir` | Each loaded | Each loaded | Each loaded | Each loaded |
| A `package.json` with an `imports` map and a name its `exports` answer, for a `data:` module | | | Neither the `#` name nor the own name resolved | Both ran the planted module |
| A sparse `package.json` of 4,298,113,024 bytes, for a plain routed policy | | | Selected at once | `selection_timeout`, exit 124, at a five-second bound |
| What a policy reads as `process.cwd()` | `/` | The private directory under TMPDIR | `/` | The private directory |

The second row is the policy's own act. It shows the fixture stayed loadable
throughout, and that the worker's directory is the one thing that differs
between the columns.

**The command-seam tests.** `task dispatch:probes` now also builds an
`unmoved` probe: the shipped source and switches without the move.

| What | Through the shipped front | Firing configuration, seen to fire | Command-seam test |
|---|---|---|---|
| `node_modules` package above the worker's start directory, for a `data:` module, a `blob:` module and a policy-registered virtual module | `policy_import_failed` under `inspect`, exit 3 under `run`, and the package never loaded | The `unmoved` probe, started in a directory under the same TMPDIR, loaded it for the same entry. The shipped worker started in that same directory did not | `hostile::a_package_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe` |
| Where the front starts its worker | An owner-only empty directory under TMPDIR, not the caller's cwd, removed afterwards | | `worker::the_front_starts_its_worker_in_a_private_empty_directory_and_removes_it` |
| Where policy code runs | `/`, and nothing left under TMPDIR | | `authority::the_worker_evaluates_policy_in_the_root_directory_with_null_stdin_and_a_fresh_environment` |

With the move replaced by a no-op in the shipped source, the first test failed
at its front arm: `inspect` exited 0 where a missing package is exit 3.

The second test uses a stand-in worker at the layout path, a shell script that
records its directory, since the real worker has left by the time a policy
could look. It found the directory was not private. `tempfile` 3.27.0 creates a
directory with mode `0o777 & !umask` unless a mode is given
([`Builder::permissions`](https://docs.rs/tempfile/3.27.0/tempfile/struct.Builder.html#method.permissions)),
so the stand-in reported `drwxr-xr-x`, and a permissive umask would have left
the directory writable by others. The front now asks for `0o700` at creation,
and the test asserts it.

**The dotenv and bunfig controls.** The square in
[ambient authority](#ambient-authority) is unchanged and still passes: the
`autoload` probe loads both files and runs the preload when started in the
hostile directory, although it then moves to `/`; the shipped worker started
there, and the `autoload` probe started in an empty directory, are each inert.
So each control still holds alone, and the private directory is still where
the front starts the worker.

`/` is not empty and no fixture can be planted in it, so what a move could add
was checked another way. A policy moved the worker into a directory holding a
`.env`, a `.env.local` and a bunfig preload, under the `autoload` probe
started in an empty directory. Nothing loaded and no preload ran. That is a
class with no known firing configuration, and
`hostile::classes_with_no_known_firing_configuration_are_reported_not_counted`
now reports it with the others. The same test's cwd tsconfig case changed for
the same reason. Every worker but `unmoved` leaves the directory it starts in,
so the entry now moves the worker back into the caller's cwd before it imports
the alias. It still did not fire under the `tsconfig` probe.

Limits of these observations:

- One host and one Bun version. A Bun upgrade rereads `set_process_cwd` and
  `resolve_and_auto_install`, and reruns the cases.
- The Linux targets are unmeasured for this control, as for the others: the
  installed smoke test runs no hostile fixture. The control there is the same
  JavaScript statement. The smoke test did pass on all three targets with this
  worker build, at the glibc and CPU floors, so the move itself succeeds there.
- `/node_modules`, and with the switch on `/package.json`, still take part for
  such a module. Both are already above every module in a file, so the move
  gives them no new say.
- Not examined: whether tsconfig `paths` above the start directory would answer
  such a module in a build with tsconfig autoloading on and no move. The
  shipped build has neither.

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
