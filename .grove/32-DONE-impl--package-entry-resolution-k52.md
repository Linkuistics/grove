# package-entry-resolution-k52

## Goal

Decide, and then deliver, whether owner policy can import an ordinary npm
package: one whose entry point its `package.json` declares with `main` or
`exports`. Today the shipped worker cannot load one.

## Context

`ambient-authority-k30` observed this against the shipped launcher, on macOS
arm64 with Bun 1.4.2. Its runtime evidence records the observation under
*Ambient authority*. The worker is compiled with `--no-compile-autoload-package-json`,
and so reads no `package.json` at run time. A bare import therefore resolves
only by file layout. A package with an `index.js` loads. A package whose
`package.json` has `"main": "./lib/entry.js"`, or only an `exports` map,
refuses as `policy_import_failed`, "Cannot find package". Plain `bun` loads all
three. Most published packages declare their entry this way.

The spec contradicts itself here. `#policy-authority` says bare specifiers
resolve through `node_modules` from the importing module. It also lists
package-json autoloading among the switches the worker is compiled without.
k30 put the limitation in the README's *Which policy runs* and in a sentence of
`#policy-authority`, as current state, and left the contract to this leaf.

The precise question: can `--compile-autoload-package-json` be enabled in the
shipped build, with tsconfig autoloading still off, without letting any
hostile class fire? The classes are cwd dotenv and bunfig, `BUN_OPTIONS`, the
`node_modules/harness-dispatch` shadow, tsconfig `paths` beside an entry or in
the cwd, and a `package.json` in the caller's cwd. The runtime evidence's
integration probe saw tsconfig `paths` fire only with tsconfig *and*
package.json autoloading on. It never tried package.json alone. Also check
whether a package's own `imports` map (`#alias`) or `exports` conditions give an
ambient input any new reach. They belong to code the owner already admitted.

## Done when

- The question has an answer, grounded in the Bun 1.4.2 source for what that
  switch reads (cite it at the decision site) and in a probe build that is
  seen to fire.
- If enabling it opens nothing, the shipped build enables it, with a
  command-seam test that `main` and `exports` packages beside an entry load,
  and the hostile-fixture tests in `tests/hostile.rs` stay green. The
  `tsconfig` probe's firing configuration may then need rethinking.
- If it opens something, the limitation is kept and stated as the contract.
  The spec's resolution sentence says so, not only the README.
- `runtime-evidence.md` records what was seen. The ADR's list of controls in
  `docs/adr/policy-evaluation-precedes-process-replacement.md` matches.
  `task check` passes, and `task release:smoke` reruns if the worker build
  changed.

## Notes

A trade-off that admits a hostile class is the human's to accept. Stop and
ask, with a recommendation, rather than choosing it alone.

## Decisions (running log)

**What the switch reads, from the bun-v1.4.2 source.** The compile switch
clears the graph flag `DISABLE_AUTOLOAD_PACKAGE_JSON`
(`src/runtime/cli/build_command.rs`), which sets `resolver.opts.load_package_json`
at start (`src/bun.js.rs`, `apply_standalone_runtime_flags`). That option has
one reader: `dir_info_uncached` in `src/resolver/resolver.rs`, which decides
whether a directory's own `package.json` is parsed into its record. The
resolver builds a record only for a directory at or above a path it is
resolving or loading, and for a candidate package directory. Everything else
hangs off the record: `main`, `exports`, the nearest enclosing `imports` map,
self-reference by `name`, and module `type` (`load_node_modules`, and the
module-type sniffs in `src/runtime/jsc_hooks.rs`). tsconfig has its own gate,
`load_tsconfig_json`, in the same function. Automatic installation is off in
any standalone executable whatever this switch says (`use_package_manager`).
The exports conditions are the target's (`bun`, `node`), `import` or
`require`, `node-addons` and `default`, plus `--conditions`; nothing reads
`NODE_ENV` for them, and `browser` maps apply only to the browser target.

**What a probe saw, macOS arm64, Bun 1.4.2, scratch builds of the shipped
source.** Six builds, one per switch combination, each driven directly.
With package.json autoloading alone on: packages declared by `main`, by
`exports`, by an `exports` subpath, by `exports` conditions, and a package
using its own `#imports` all loaded, and each refused under today's build.
The condition chosen was `bun`, with `NODE_ENV` set to `production` or
`development` in the worker's environment. A `package.json` in the cwd, with
an `imports` map, a self-referencing `name` and `exports`, and a `node_modules`
beside it, did nothing for an entry elsewhere under any build, with the worker
run in that directory. The same file fired for an entry inside that directory,
so the fixture is live. A `harness-dispatch` shadow declared by `exports`,
in `node_modules` or as the entry's own enclosing package, lost to each
registered specifier and won under the build without the registration.
tsconfig `paths` fired with tsconfig autoloading alone, with or without a
`package.json` beside it, and never with package.json autoloading alone: the
integration probe's "tsconfig and package.json" was an untested cell, not a
dependency. cwd dotenv and a bunfig preload stayed inert. A `package.json`
naming a dependency, with no `node_modules`, installed nothing, where plain
`bun` fetched it.

**First build of those probes was wrong, and said nothing.** It ran under zsh,
which does not split an unquoted variable, so the dotenv and bunfig switches
travelled as one word that Bun ignored. The only sign was the dotenv case
firing under a build meant to disable it. The builds were remade from a bash
script and every case rerun; the results above are the rerun's.

**What changes inside the admitted set.** An entry's own enclosing
`package.json` now applies: its `imports` map resolves `#` names, and a bare
import of its own `name` resolves through its `exports` ahead of any
`node_modules`, a nearer one included. That is Node's package resolution, and
the file sits at or above the importing module, where `node_modules` is
already trusted. It is still a change an owner could meet, so the spec states
it and the leaf's one in-session review attacks it.

**The answer looked like yes, so the shipped build enabled it. Reversed below.**
No class the task
names fires, and the files newly read sit only where `node_modules` already
resolves. That admits no hostile class, so it is not a trade-off for the human
to accept. The switch's reasoning and its two source citations sit at the
decision site, above `SHIPPED_SWITCHES` in `scripts/dispatch.sh`.

**The switches are stated once.** The first probe build went wrong by losing
switches unseen, and the old `tsconfig` probe enabled two switches where its
name promised one. So `dispatch.sh` holds the shipped set in one array and
`switches_enabling` derives each probe's from it, refusing a class the shipped
build does not turn off. The `tsconfig` probe is now the shipped worker with
tsconfig autoloading on and nothing else changed.

**A cwd `package.json` is a counted class, not a tripwire. Parked with the
switch.** It has a firing
configuration through the public launcher: an entry in that directory named
with `--config`. The test also starts the shipped worker in the hostile
directory, to show the private directory is not what keeps the class out.

**The shadow fixture has three layouts. Parked with the switch.** The worker
can then be shadowed by a
`node_modules` package that declares `exports`, and by the entry's own package
named `harness-dispatch`, so the shadow case proves each registered specifier
against all three, and sees each fire under the `unregistered` probe.

**Each new control was seen to fail.** With the shipped switch put back to off
in `dispatch.sh`, and the worker and probes rebuilt, the two new authority
cases, the cwd `package.json` case and the shadow case failed. The
auto-install case passed, as it must either way. The installed smoke test's
new `declared_package` case passed against the checkout's pair and refused the
same mutant with `policy_import_failed`. `dispatch.sh` was restored to digest
`59b723faca559d5d` each time.

**The installed smoke test gained a case. Parked with the switch.** The Linux
targets are otherwise
unmeasured for anything this leaf changes, and the case costs one inspection
per front. It imports one package declared by `main` and one by `exports`.

**All three targets passed with the switch on.** `task release:smoke` ran the
four cases through both fronts on macOS arm64, Linux arm64 natively and at the
Cortex-A53, and Linux x64 at Nehalem, worker build `ac71fded3ec5…`. The 54
files it reads had the same digests before and after. That run is of a build
this leaf did not keep.

**The review broke the claim, and each break was reproduced before it was
believed.** The claim was that the worker reads a `package.json` only at or
above a module being imported, or in a package an import resolved to. Bun
resolves a module compiled into the worker, and any module with no file
location, from the directory the process started in
(`resolve_and_auto_install`), and builds that directory's record and each
ancestor's even for an import of an absolute path (`resolve_without_symlinks`).
The front creates the worker's directory under the caller's TMPDIR. Through
the checkout's front, with TMPDIR naming a planted directory: a `package.json`
there with an `imports` map and a name its `exports` answer ran the planted
module for a `data:` importer, and did nothing for the policy file's own
imports; a sparse one of 4097 MiB turned a plain routed policy into
`selection_timeout`, exit 124. Driving the builds showed the same for `blob:`
and policy-registered virtual modules.

**Each finding, classified.**

- A `package.json` above the worker's directory stalls every selection: valid
  and actionable. It is new with the switch and fails closed.
- The same file answers `#` and own-name imports from a module with no file
  location: valid and actionable, new with the switch.
- A `node_modules` above the worker's directory answers a bare import from
  such a module with every switch off: valid, and not this leaf's. It predates
  the switch and belongs to the control `ambient-authority-k30` delivered.
- The nearest `package.json` beats a nearer `node_modules` for its own name:
  a visible trade-off inside the admitted set, already found here.
- An `imports` alias to a registered specifier loads a `node_modules` shadow:
  valid, inside the admitted set, and reproduced. It is `k62`'s to state or
  close.
- `main` and `exports` win over an `index.js` beside them: noise. That is the
  resolution asked for.
- My spec sentence was unclear as a contract and false as a fact. It is gone.

**The answer is no, as the worker stands, so the limitation is the contract.**
The task's own rule decides it: enabling opens something. I did not weigh
shipping it anyway. That would admit a hostile class, which is the human's to
accept, and the route that needs no such acceptance still ends with packages
loading. The reason and its citations sit above `SHIPPED_SWITCHES` in
`scripts/dispatch.sh`, and the spec's `#policy-authority` says it.

**Two leaves carry the rest, ahead of the documentation.**
`worker-directory-chain-k61` closes the chain, which is a repair to an earlier
step's control and a choice with costs, so it is not absorbed here.
`package-json-autoloading-k62` then turns the switch on. The enable work is
parked as `.grove/package-json-autoloading-k62.enable.patch`, which applies
cleanly to this leaf's final tree. It holds the four test and smoke files, not
the switch or the documents, since the documents' central sentence was the
false one.

**The gap that predates the switch is stated, not hidden.** The spec and the
README each say a module with no file location resolves from the worker's
directory upward, and name k61. The ADR says the private directory does not
control its ancestors.

**What this leaf keeps from the enable work.** `SHIPPED_SWITCHES` and
`switches_enabling`, so the `tsconfig` probe turns on tsconfig autoloading
alone, which the probe showed is all that class needs. One command-seam case
of the contract, `a_package_resolves_by_its_file_layout_and_never_through_its_package_json`.
The spec's firing-configuration row for tsconfig, corrected to match.

**The in-session review is spent.** No second pass is needed: what ships is
the switch set the parent revision shipped, and the contract case covers it.

**`task check` passes, all twelve checks, and `task release:smoke` passes on
all three targets.** Both ran once, after the last edit, and the working
copy's snapshot was `59a947cc2aa7…` before and after. The smoke rebuilt the
archives with worker build `0846937789b3…`, whose switches are the parent
revision's but whose source digest moved with `scripts/dispatch.sh`. Its three
cases passed through both fronts on macOS arm64, on Linux arm64 natively and
at the Cortex-A53, and on Linux x64 at Nehalem; the glibc and CPU controls
fired. Before that run, the contract case was seen to fail with the switch
put on in `dispatch.sh`, and to pass with the script restored to digest
`bcea7090461023c0`. The `tsconfig` probe, now tsconfig autoloading alone, still
fires its fixture in the hostile suite.
