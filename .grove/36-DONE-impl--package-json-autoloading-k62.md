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

## Decisions (running log)

**k61 closed the chain, so this leaf ran.** The brief and k64's log say what
the move leaves: `/` itself. Nothing here reopens that.

**The parked patch applied as k64 said.** Three files applied with `patch
-p1`. The `tests/authority.rs` hunk was applied by hand, since k61 had
replaced the test beside it.

**An alias to a registered specifier fails with no shadow.** The task asked
which of k52's two readings holds. Through the checkout's front, with the
switch on: an entry's own `package.json` mapping `#sdk` to
`harness-dispatch/sdk` refused as `policy_import_failed`, "Cannot find package
'#sdk'". Beside a `node_modules/harness-dispatch/sdk.js` it loaded that file,
while `harness-dispatch/sdk` in the same entry was still the embedded module.
So the review was right and k52's probe was wrong.

**Why, from the bun-v1.4.2 source.** `load_package_imports` in
`src/resolver/resolver.rs` resolves the map, and for a target that names a
package it tries a builtin alias and then calls `load_node_modules` itself.
The registration is a module-loader hook on the specifier an import writes.
The resolver never returns to it.

**The spec says so, and the worker does not close it.** Closing it means the
worker answering every `#` import itself: finding the `package.json` Bun
would use, and resolving its map with Bun's conditions and patterns. A copy
that disagrees with Bun anywhere claims the wrong module, on a path every
package's internal `#` imports take. That is a larger risk than the limit it
removes. The limit admits no new party. Whoever writes the `imports` map can
already point a `#` name at any file, and a shadow's author alone gains
nothing, because no import the owner wrote names their package. With no
shadow it fails loudly. So this is inside what an entry admits, as the task
put it, and I did not put it to the human. It does make the shadow control
narrower than its old sentence: an owner who aliases the SDK and installs a
package named `harness-dispatch` gets that package. The spec and README say
it, a command-seam case holds both sides, and the review leaf is asked to
attack this call.

**The nearest `package.json` is the nearest one, named or not.** The source
has a second notion, `enclosing_package_json`, which skips a nameless file.
`#` imports and own-name imports do not use it: `load_node_modules` walks up
to the first directory with a `package.json` that parsed. A nearest one with
no `imports` field ends the search for a `#` name. The spec's sentence is
written from that.

**The TMPDIR case reuses the cwd case's package.** `hostile_package` answers
three ways: an `imports` alias, its own name, and a `node_modules` dependency
its `main` declares. The new case plants it in TMPDIR and imports each name
from the policy's file and from the three kinds of module with no file
location. For a module in a file the package has no firing configuration in
any build, since a file's imports resolve from the file, and the case asserts
that under the `unmoved` probe too. The same fixture fires there for the other
three.

**The oversized arm is in the suite, with its firing configuration.** A
sparse `package.json` of 4 GiB and one page. Measured under the `unmoved`
probe, driven directly: a file up to exactly 4 GiB is read whole and the
policy then loads (2.7 s and 4.1 GiB resident at 4 GiB), and one page more
never reports. So the stall is a length past a `u32`, and anything smaller
costs its size in memory on every evaluation. A FIFO named `package.json` is
not opened. The firing arm costs about 4 GiB resident for five seconds on each
run of the suite. I kept it: the alternative was a control seen once by hand.
It asserts that the probe reports nothing, not that it stalled, so a host that
kills the probe for the memory still counts as fired. The shipped worker gets
the same five seconds, so a slow host fails that arm and cannot pass the
probe's.

**`direct::drive_within` kills a silent worker.** `drive` waited 30 s for a
frame and then waited for exit, which a stalled worker never reaches.

**The cwd case also drives the `unmoved` probe.** The brief said the parked
comment, "Nor is it the front's private directory that keeps it out", was no
longer the whole reason. The shipped worker now leaves any directory it starts
in. The probe that stays finds nothing either, which is the real reason: a
file's imports resolve from the file.

**Each control was seen to fail.** With the switch put back to off in
`dispatch.sh` and the worker and probes rebuilt, six cases failed: both new
authority cases, and in `hostile` the shadow, cwd `package.json`, TMPDIR
`package.json` and alias cases. The auto-install case passed, as it must
either way. With the move removed from the shipped source instead, the TMPDIR
`package.json` case and k61's package case failed at their front arms, and by
hand the oversized file gave `selection_timeout`, exit 124, at a five-second
bound. That last was by hand because the case stops at its first arm. Both
files were restored to digests `323c445ac685…` and `030406f5f9ac…`, and the
worker rebuilt as `721aab0848f6…`.

**The nearest-file rule was also seen.** For an entry one directory below a
named `package.json` whose map answers `#x`: a nameless file beside the entry
with its own map answered instead, one with no `imports` field refused the
import, and one that did not parse left the outer map answering. The runtime
evidence has it.

**`task check` passes, all twelve checks, and `task release:smoke` passes on
all three targets.** Each ran once, after the last edit to anything either
reads. The working copy's snapshot was `513b3ed58f67…` before the check, after
it and after the smoke test. The smoke test ran four cases through both fronts
on macOS arm64, Linux arm64 natively and at the Cortex-A53, and Linux x64 at
Nehalem, worker build `721aab0848f6…`; the glibc and CPU controls fired. What
changed afterwards is prose only: two paragraphs in the runtime evidence that
report these runs, this log, the brief, and the review leaf.

**The parked patch is gone.** Its four files are applied, so it no longer
applies to anything, and a reviewer should not meet it.

**It has earned a `review-impl` leaf, as the task said, cut ahead of
`dispatch-documentation-k41`.** Its body names the alias call first. No
in-session reviewer was spent. This session ran on a direct harness and has no
dispatch run, so the review carries its `**Reviews:**` line and no
`**Creator:**` line.

**Done-when, by test.** `main` and `exports` packages beside an entry:
`authority::a_package_loads_by_the_entry_point_its_package_json_declares` and
`authority::an_entrys_own_package_json_applies_its_imports_map_and_answers_its_own_name`.
The hostile cases:
`hostile::a_cwd_package_json_stays_inert_and_fires_for_an_entry_admitted_there`,
`hostile::every_documented_specifier_resolves_to_its_embedded_module_beside_a_package_shadow`
in three layouts,
`hostile::tsconfig_paths_beside_an_admitted_entry_stay_inert_and_fire_under_the_tsconfig_probe`,
`hostile::a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe`
and `inspect::a_missing_package_is_never_installed_automatically`. A
`package.json` above the worker's directory, small and oversized:
`hostile::a_package_json_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe`.
The alias limit:
`hostile::an_imports_alias_to_a_registered_specifier_is_a_package_lookup_and_never_the_embedded_module`.
