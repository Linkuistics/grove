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
