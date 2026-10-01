# package-json-autoloading-k65

**Reviews:** package-json-autoloading-k62

## Goal

An adversarial, inspection-only read of `package-json-autoloading-k62`: the
shipped worker now compiled with package.json autoloading on, the cases that
claim what that admits and what it does not, and the documents that state
it. Produce findings, not fixes.

## Context

k62 changed one word in `SHIPPED_SWITCHES` in `scripts/dispatch.sh`. The
shipped worker now reads `package.json` at run time, which changes what every
installation reads on every evaluation. `package-entry-resolution-k52` enabled
the same switch once, and its one review found a break that every test had
missed. Assume this attempt has one too.

Read *Package.json autoloading*, *Package entry resolution* and *The worker's
directory* in the runtime evidence first, then k62's running log. The contract
is the spec's `#policy-authority`, from "Other external imports" to the
paragraph before "Because tsconfig", and the firing-configuration table under
`#test-seams`. The README states it under *Which policy runs* and after "Nothing
ambient takes part otherwise". The ADR is
`docs/adr/policy-evaluation-precedes-process-replacement.md`. The code is the
comment above `SHIPPED_SWITCHES`, `tests/support/direct.rs`, the two package
cases in `tests/authority.rs`, five cases in `tests/hostile.rs` (shadow, alias,
cwd `package.json`, and the two "above the worker's start directory" cases),
the auto-install case in `tests/inspect.rs`, and `case_declared_package` in
`scripts/installed-smoke.sh`.

Specific doubts k62 could not settle from inside:

- **A limit was accepted without the human.** An `imports` alias to a
  registered specifier, such as `"#sdk": "harness-dispatch/sdk"`, is a
  `node_modules` lookup. With no package of the name it refuses, and beside a
  `node_modules/harness-dispatch` it loads that package. k62 judged this
  inside what an entry admits, stated it, and did not close it or ask. Its
  running log gives the argument. Attack it. Find a party who gains reach they
  lacked, or a configuration that worked before the switch and now loads a
  shadow. Then judge whether the worker could close it at a cost k62
  overstated.
- **The spec claims no `package.json` between the worker's start directory and
  `/` is read on that account.** k62 has a small file there that answers
  nothing and one past 4 GiB that stalls nothing, through the front. Check what
  Bun does before the worker's move runs, with the switch on. Check the other
  ways a policy can resolve: `import.meta.resolve`, `Bun.resolveSync`,
  `require.resolve`, `createRequire` from a module with no file location, and
  a native `Worker` the policy starts. k52's universal sentence about where
  the worker reads a `package.json` was false. Decide whether k62's
  replacement is true as written or only narrower.
- **Which `package.json` is a module's own.** The spec says the nearest one at
  or above the module. k62 read `load_node_modules` in the bun-v1.4.2 source
  for that and tested one layout. Check a nameless file, one that does not
  parse, one with no `imports` field below one that has it, and a package
  reached through a symlinked directory, whose real location's ancestors the
  resolver also records. k62 listed the last, and the `type` field, as not
  examined.
- **No variable chooses a condition.** The case grants `NODE_ENV` as
  `production` and as `development` and sees `bun` chosen for an `exports`
  map. Check the same for an `imports` map, and read the source for any other
  input that adds a condition and that the front's environment does not
  already exclude.
- **Nothing is installed.** The auto-install case now has a `package.json`
  naming the dependency. Check that a populated Bun install cache under HOME
  cannot answer that import in the compiled worker.
- **The oversized firing arm is weak by design.** It asserts that the
  `unmoved` probe sends its hello and then reports nothing in five seconds,
  and it accepts a probe the host killed. Decide whether a dead fixture could
  pass it. It also holds about 4 GiB resident for those seconds on every run
  of the suite. Say whether that cost is worth what the arm proves.
- **`direct::drive_within`** now kills a worker that stays silent. Check that
  no existing caller of `drive` can lose a report to it, and that a read
  timeout is told apart from a closed channel on Linux as well as macOS.
- **The policy-file arm has no firing configuration.** The TMPDIR case asserts
  that a module in a file is unanswered under the `unmoved` probe too, and
  rests its fixture's liveness on the other three kinds of module. Decide
  whether that is a control or a claim.
- **The probes changed with the switch.** Every probe derives its switches
  from the shipped set, so the `tsconfig` and `autoload` probes now have
  package.json autoloading on as well. Check that each still differs from the
  shipped worker by its one control, and that no case which passed with the
  switch off now passes for a different reason.
- **The documents.** Check that the spec's resolution paragraphs, both tables,
  the README, the ADR and the changelog say one thing, and that each sentence
  has a test or a stated limit behind it. `/package.json` is said to be read
  on every evaluation; nothing planted one.
- **Linux is unmeasured for every hostile class here.** The installed smoke
  test's `declared_package` case is the only reading there. Say whether that
  is enough for a change to what the worker reads.

## Done when

- Each doubt above has a finding or a stated reason there is none, with the
  evidence: a source citation at bun-v1.4.2, a command and its output, or a
  test that fails.
- Any finding worth acting on has an `integrate-review-impl` leaf cut where
  `pick` reaches it next. A review that finds nothing cuts nothing and
  retires.

## Notes

Probe builds and scratch builds are instruments, so building one to test a
doubt is inspection. Changing shipped source, tests or documents is not.

## Decisions (running log)

**The graph cannot evidence this review.** The targeted Tier 2 graph lookups returned no dispatch
symbols. Coverage generation `2026-09-29T11:18:53Z` reports
these files as `not_tracked`, and excludes `docs/`. This review reads the
committed diff and exact source directly; it makes no graph-completeness claim.

**Two findings have settled.** The oversized firing arm accepts an unrelated
post-hello exit as evidence of the manifest's effect. The public nearest-file
rule omits that malformed manifests are skipped: a scratch invocation selected
an outer map, while the nearest manifest did not parse. Findings and each
remaining doubt's disposition follow below. No implementation, tests or durable
documents have been changed; no test, build, lint or format command has run.

## Findings

Reviewed producer commit `37b1a0f4023d4dc8801914025952b11535b7ef4f` against its
parent and the current files. The working copy was empty at entry.

### F1 — P2: an unrelated probe exit satisfies the oversized firing arm

Location: `crates/harness-dispatch/tests/hostile.rs:1223`, with the distinction
already available at `crates/harness-dispatch/tests/support/direct.rs:189`.

After checking the hello's build identity, the oversized arm asserts only
`driven.report.is_none()`. A probe that reads the evaluate request and closes
its channel immediately, without opening the manifest, satisfies that assertion:
`hello = correct identity`, `report = None`, `stalled = false`. The helper tells
a closed channel apart from a timeout, but this assertion discards the distinction.
Neither the successful shipped-worker arm nor the earlier, different importing
policies proves that this probe evaluated the plain routed entry successfully
without this manifest. An unrelated crash on this path can therefore be counted
as the oversized fixture firing. This is an inspection of the assertion, not a
claim that the current Bun probe crashed for an unrelated reason.

The source-backed observation that Bun reads the file whole is credible, and
k62 recorded an actual stall. The regression control does not retain that
observation's causal evidence. The spec's table at line 1121, its introductory
claim at line 50, and the runtime-evidence table at line 638 give the control
more credit than this assertion supports. Holding roughly 4 GiB resident on
ordinary checks is especially difficult to justify for an arm that accepts any
post-hello death.

Integration should distinguish a measured timeout from unrelated termination,
and establish a healthy baseline with the same probe, plain entry and starting
directory without the oversized manifest. If OOM remains an accepted firing
outcome, retain evidence tying it to this read rather than any worker exit.
Alternatively use a cheaper causal control and describe the historical large-file
measurement as such. This review does not require copying Bun's resolver or
turning package.json autoloading off.

### F2 — P3: the public nearest-manifest rule omits malformed-file fallback

Locations: `docs/specs/harness-selection-and-execution.md:384`,
`crates/harness-dispatch/README.md:125`, and `CHANGELOG.md:174`.

These passages say the nearest `package.json` is the importing file's package.
That is false when the nearest file does not parse: Bun skips it and uses an
outer valid manifest. k62's own running log and runtime evidence already record
this exception, but the public contract does not. Consequently a malformed
local manifest does not stop an outer `imports` map from choosing the code.
The unconditional comparison to Node also overstates what was established.

Reproduced through the current checkout's front with this layout:

```text
owner/package.json          {"name":"outer","imports":{"#x":"./outer.ts"}}
owner/outer.ts              export const value="outer";
owner/nested/package.json   { broken
owner/nested/policy.ts      import {value} from "#x"; /* valid routes policy, version: value */
```

Invocation: an empty environment plus private HOME/TMPDIR and
`PATH=/usr/bin:/bin`, then
`target/debug/harness-dispatch inspect --config <absolute owner/nested/policy.ts> --kind impl --json --timeout-ms 3000`.
Output: exit 0, `policy.version = "outer"`. Replacing the nearest file with a
nameless valid map to `./inner.ts` gives exit 0 and `"inner"`; replacing it with
`{}` gives exit 3, `policy_import_failed` naming `#x`. These three results
separate an absent name, an absent imports field and a parse failure.

The pinned source agrees: `dir_info_uncached` records only a regular file that
parsed, and `load_node_modules` walks to the first recorded package. See
[the recording gate](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L6317-L6356)
and [the upward search](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L2523-L2550).

Integration should state the nearest successfully parsed regular manifest,
including the malformed-file fallback and valid-map boundary, consistently in
the public documents. Keep a command-seam regression for this distinction if
the rule remains part of the advertised contract. No new untrusted party was
shown to gain code-execution authority: the outer directory is already among
the entry's admitted ancestors.

## Disposition of every mandated doubt

1. **Aliasing a registered specifier. No additional finding.** The final scratch
   run reproduced exit 3 without a shadow and exit 0 with
   `policy.version = [["shadowed"],true]`: the alias loaded the shadow and the
   direct SDK import retained `definePolicy`. The entry's manifest must supply
   the alias. A shadow author alone cannot replace a direct registered import,
   and with the switch off the `#` import could not have worked. No previously
   successful registered import was shown to become a shadow import. The source
   path is `load_package_imports`' call to `load_node_modules`, bypassing the
   registration at `worker/src/main.ts:144`.
   [Pinned source](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L4944-L5044).
   A complete transparent repair would need correct package-map semantics;
   blanket refusal of `#` imports would also break ordinary packages, while
   trapping a resolved shadow file would not repair the missing-package case.
   k62's claim that every conceivable closure requires a copied resolver is
   stronger than necessary, but no cheap transparent closure was established.
   The documented behavior is within admitted package code, so this review
   does not identify a new hostile class requiring human acceptance.
2. **Before the move and alternate resolver APIs. No additional finding within
   the examined paths.** `boot_standalone` applies the compiled flags before
   `configure_defines`; disabled dotenv loading avoids directory traversal.
   `init_with_module_graph` configures the linker with automatic JSX discovery
   off, explicitly avoiding startup package/tsconfig reads. The worker's static
   imports are bundled modules or prefixed built-ins; the policy import occurs
   after its move and hello. Sources:
   [startup](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/cli/run_command.rs#L1116-L1224),
   [flags](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bun.js.rs#L10-L27),
   [VM initialization](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/VirtualMachine.rs#L4084-L4110),
   [disabled traversal](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/transpiler.rs#L834-L842).
   The scratch matrix put `#hostile` only in TMPDIR's manifest and invoked,
   from a `data:` module, `import.meta.resolve`, `Bun.resolveSync` with
   `process.cwd()` as its parent, `import.meta.require.resolve`, and
   `createRequire(import.meta.url).resolve`. None found it. The same
   `Bun.resolveSync` found it after an explicit policy `process.chdir(TMPDIR)`.
   The other synthetic-referrer variants still refused after a move, so those
   observations establish refusal, not an independent firing control. A native
   Worker started from a data URL also failed to resolve it. Native APIs with
   an explicitly supplied file parent admit that location; they do not prove
   ambient discovery. These are bounded observations, not an exhaustive audit
   of Bun APIs. The spec's narrow phrase "on that account" is supported.
3. **Which manifest is the module's own. F2.** The three nearest-file outcomes
   above reproduce k62's source reading. A package reached through
   `node_modules/linked -> real/module` also loaded `#real` from the manifest
   above its real location, reporting `"real-ancestor"`. These are ancestors
   of admitted code. The module `type` field remains a stated limit of k62's
   observations; this review makes no universal module-format claim.
4. **Conditions. No additional finding.** An imports map ordered
   `production`, `development`, `bun`, `default` selected `bun` with NODE_ENV
   granted as either production or development. The existing authority case
   does the corresponding exports check. `ESMConditions::init` constructs the
   maps from runtime defaults, import/require, addons and explicit condition
   arguments; `set_production` changes flags/JSX rather than those maps.
   [Condition construction](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/options.rs#L724-L771),
   [production setting](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/options.rs#L1404-L1414).
   The front excludes BUN_* and NODE_OPTIONS in `src/environment.rs:138`;
   the policy worker is started without condition arguments. No additional
   environment-selected condition was established on this path.
5. **Populated package cache. No additional finding.** The cache lookup and
   installation share a `use_package_manager()` gate, which returns false for
   a standalone graph before either is entered. That rules out a populated
   HOME cache as well as an empty one, by source inspection; this review did
   not populate/download a cache or claim a new cache experiment.
   [Gate](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L882-L899),
   [cache entry](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L2830-L2844).
6. **Oversized firing arm and its cost. F1.** No 4 GiB experiment was repeated
   here. Historical memory/time readings are k62's; the false-positive path
   comes directly from the current assertion and helper.
7. **`direct::drive_within`. No separate caller regression found.** `drive`
   retains the previous 30-second read timeout. The callers in `tests/hostile.rs`
   drive short import/report fixtures and require a policy or failure frame;
   the new five-second patience is used only for the oversized pair. The
   header reader recognizes both WouldBlock and TimedOut. Rust documents Unix
   socket timeouts as typically WouldBlock; accepting TimedOut as well is conservative:
   [Rust socket documentation](https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout).
   A closed channel is distinct. The helper does not promise a total wall-clock
   deadline: body-read failure still panics and process reaping is separate.
   F1 concerns the caller discarding the distinction, not Linux having a
   different timeout spelling.
8. **Policy-file arm with no firing configuration. No additional finding.**
   Its negative assertion is a resolver claim, not a positive control for that
   file-origin path. The spec at line 1143 and runtime-evidence table say so.
   The same live manifest fires for the three no-file module variants, and
   the cwd case admits a file in that directory explicitly and sees it fire.
   These demonstrate fixture liveness without pretending the unrelated file
   should have resolved from TMPDIR.
9. **Changed probes. No additional finding.** `switches_enabling` copies the
   shipped array and changes only the named negative switches; autoload still
   removes the dotenv/bunfig control together, tsconfig removes its own switch,
   and unmoved/unregistered retain the shipped switches. Their build identities
   remain distinct and the front refuses them. The recorded k62 mutation runs
   say the existing hostile fixtures still fire under their probes. That is
   historical verification, not a rerun here. No changed fixture was shown to
   pass for a different reason.
10. **Documents and /package.json. F1 and F2 are the inconsistencies found.**
    Alias and root exceptions otherwise agree in the spec, README, ADR,
    changelog and current runtime-evidence section. `/package.json` was not
    planted in k62, but k64's recorded Linux container did plant it in a
    package-autoload-enabled build and saw it answer a no-file module. Source
    resolution of the worker's absolute policy import records `/`, explaining
    the every-evaluation read independently of that import-answer experiment.
    "Read on every evaluation" is source-backed, not a newly measured root
    fixture in this review.
11. **Linux evidence. No additional finding against the agreed release seam.**
    k62's installed smoke imports main/exports packages through both fronts
    on all supported targets. k64 already measured the TMPDIR/root distinction
    on Linux arm64 with package autoloading on. Hostile regressions are still
    absent from the per-target smoke and those one-off container cases do not
    rerun automatically. The documents state that limit; neither smoke success
    nor identical JavaScript proves all Linux hostile classes. The accepted
    per-target requirement is delivery/TypeScript execution and compatibility
    floors, which the producer records as checked. This review does not
    independently certify those historical checks.

## Review evidence and handoff

The completed scratch command was `python3 /tmp/grove-review-k65/probe.py`.
It ran only fresh temporary policies through the existing front, with no
Grove signal or other caller environment inherited. Outputs above come from
its final run; earlier instrument attempts with an invalid catalog were
corrected and are not credited. SHA-256 before and after that final run matched
for the script, front and worker:

- script: `aeab1be804557ff78c5c53ba57f7f6388d3b49d4e2493737873f47b361a05f0c`
- front: `59a71a9c6d63bea9ae9742d1892b2a7f58c730b7740a2cdf30635e58f9aa03c7`
- worker: `c840885f5a45075aa95e369c44f8087848522ba51e9ffed09d21f831f8e627cf`

`package-json-autoloading-k66` is the paired integration, inserted immediately
ahead of `dispatch-documentation-k41`. It owns triage, all fixes and post-fix
verification. This root-level review closes no ancestor node. The producer's
`task check` and `task release:smoke` results are recorded evidence only; this
review ran neither and changed no production code, tests or durable documents.
