# package-json-autoloading-k62

## Goal

Turn package.json autoloading on in the shipped worker, so that owner policy
can import an ordinary npm package: one whose entry point its `package.json`
declares with `main` or `exports`.

## Context

`package-entry-resolution-k52` built this, saw it pass on every release
target, and then took it back out. Its one adversarial review found that the
switch makes the worker read every `package.json` at or above its own
directory, which sits under the caller's TMPDIR.
`worker-directory-chain-k61` closes that chain. This leaf runs after it and
depends on it. If k61 ended without closing the chain, stop and say so.

Read *Package entry resolution* in the runtime evidence first. It holds the
source citations, what each build did, and what the review found. k52's
running log holds the reasoning, including one probe build that was silently
wrong.

The work k52 parked is `.grove/package-json-autoloading-k62.enable.patch`,
made against k52's final tree. `patch -p1 --dry-run` it first: k61 will have
moved `tests/hostile.rs`. It carries:

- `tests/authority.rs`: two cases that replace
  `a_package_resolves_by_its_file_layout_and_never_through_its_package_json`.
  One loads packages declared by layout, `main`, `exports`, a subpath, a
  condition and an own `imports` map, and shows a granted `NODE_ENV` choosing
  nothing. The other shows the entry's own `package.json` supplying its
  `imports` map and answering its own name ahead of a nearer `node_modules`.
- `tests/hostile.rs`: the shadow fixture in three layouts, and a cwd
  `package.json` class whose firing configuration is an entry in that
  directory named with `--config`.
- `tests/inspect.rs`: the auto-install case with a `package.json` naming the
  dependency.
- `scripts/installed-smoke.sh`: a `declared_package` case.

It does not carry the switch or any document. The switch is one word in
`SHIPPED_SWITCHES` in `scripts/dispatch.sh`, and the comment above it must be
rewritten with it. Each probe derives its switches from that array, so the
probes follow.

Three things change inside what an entry admits, and the spec must state each:

- The nearest `package.json` at or above an importing module supplies that
  module's `imports` map.
- A bare import of that package's own `name` resolves through its `exports`
  ahead of any `node_modules` package of the name, a nearer one included.
- An `imports` alias whose target is a registered specifier, such as `"#sdk":
  "harness-dispatch/sdk"`, does not reach the embedded module. Beside a
  `node_modules/harness-dispatch` shadow it loaded the shadow. With no shadow,
  k52's probe saw it load and the review saw it fail, so establish which. The
  spec's sentence that a registered specifier always resolves to its embedded
  module is then false for an aliased one, and either the worker closes that
  or the spec says so.

k52 wrote one sentence that the review proved false: that every `package.json`
the worker reads sits at or above a module being imported or in a package an
import resolved to. Do not restore it. Say what k61's control makes true.

## Done when

- The shipped build enables the switch, with the reason and its source
  citations at the decision site.
- Command-seam cases show `main` and `exports` packages beside an entry load,
  and the hostile cases stay green: cwd `package.json`, the shadow in each
  layout, tsconfig `paths`, dotenv and bunfig, and no automatic installation.
- A case proves a `package.json` above the worker's directory inert through
  the front, for a policy file and for a module with no file location, beside
  a firing configuration seen to fire. An oversized one there does not stall a
  selection.
- The spec's resolution paragraphs, its firing-configuration table, the
  README's *Which policy runs*, the ADR's list of controls and the changelog
  say what holds. The runtime evidence records what was seen.
- `task check` passes and `task release:smoke` reruns with the
  `declared_package` case.

## Notes

A trade-off that admits a hostile class is the human's to accept. Stop and
ask, with a recommendation, rather than choosing it alone.

k52 spent its one in-session review on this, and it found the break. A change
to what a shipped worker reads has earned a `review-impl` leaf here. Cut it
before retiring.
