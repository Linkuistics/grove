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
