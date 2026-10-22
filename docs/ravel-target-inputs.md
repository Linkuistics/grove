# Independent Ravel inputs for Grove delivery tests

Grove's target tests consume released **Ravel 1.1.0**. They do not depend on
Grove's version, archive manifest or source tree. The tests need no adjacent
Ravel checkout, no Homebrew, no host Bun or Node, no replacement of the
installed front and no release operation. Grove's release smoke now admits these
independent inputs before it runs its installed callers. No Grove archive
bundles a Ravel executable or worker.

## Pinned public inputs

The source tag, peeled, is `2095cbd879c968a26f322905422ddaff9348f6ca`.
Each URL is under
`https://github.com/Linkuistics/Ravel/releases/download/v1.1.0/`.
GitHub permits asset replacement. The SHA-256 pins enforce the tested identity.
The public [release](https://github.com/Linkuistics/Ravel/releases/tag/v1.1.0)
and Ravel's `docs/publication-evidence.md` record the publication provenance.

| Archive | SHA-256 |
|---|---|
| ravel-v1.1.0-aarch64-apple-darwin.tar.xz | `92ec94083495f536136a17310ce1624004d145569f6fbf92da978aee65e7c357` |
| ravel-v1.1.0-aarch64-unknown-linux-gnu.tar.xz | `4334bd0adfa97391925b44029e96ce70744ecf3d141a2b5bb21174fd7cfa9c60` |
| ravel-v1.1.0-x86_64-unknown-linux-gnu.tar.xz | `2a0f91c793069fa2dd8d73fcbe0fe383d383d029f60adc4a5b7f3726919b60fb` |

## Consumer interface

```sh
# Download missing inputs into target/ravel-archives and prepare a fresh layout.
task ravel:targets:prepare OUT=target/my-ravel-inputs

# Strict offline admission of an explicit directory; no network fallback.
task ravel:targets:prepare ARCHIVES=target/ravel-archives OUT=target/my-other-inputs

# Full manifest/target/digest and real-input refusal controls.
task ravel:targets:controls ARCHIVES=target/ravel-archives

# All native/container/QEMU routes; TARGETS may select supported targets.
task ravel:targets:smoke ARCHIVES=target/ravel-archives
```

`OUT` must not exist. The default target set is all three supported targets.
`TARGETS='aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu'` selects Linux.

Preparation prints one tab-separated `TARGET<TAB>absolute-bin/ravel-path` line
per target on stdout. It returns a nonzero status on any failure. It exposes the
output only after all requested archives pass. Within `OUT`:

```text
archives/ravel-v1.1.0-TARGET.tar.xz
TARGET/ravel-v1.1.0-TARGET/bin/ravel
TARGET/ravel-v1.1.0-TARGET/libexec/ravel/...
```

The release smoke places Grove in a different installation prefix. It puts the
matching Ravel `bin` on the PATH of its test child. Where the caller fixture
permits, it passes the absolute public front instead.

Ravel locates its own worker from its real front path. Keep the whole Ravel prefix. Do not move only
the front, compare its version to Grove, or use a shared Cellar. Use the
`start-session` and `end-session` verbs of Ravel and the opaque result that they
return. The inherited context is the public session boundary of Ravel.

The preparation task reads every cached or explicit archive into private bytes.
It checks the SHA-256 digest and parses the whole archive before it extracts any
member. It checks these properties:

- the exact top directory, version and target.
- all files and directories, and their uniqueness.
- regular-file types and executable modes.
- the ELF or Mach-O architecture of both programs.

A missing, extra, traversal or AppleDouble member fails. A link, a special mode
or a wrong target also fails.

Extraction writes only admitted regular files and
directories. A corrupt cache fails and is not fetched again. Remove or repair
that cache explicitly before you retry. A missing explicit archive never causes
a fetch. No command-line option overrides the digest or the manifest.

For installed smoke, prepare a new snapshot on every run. The runner stages its
checked archives read-only into Linux environments. On the host, these
artifacts stay ordinary writable files. An earlier successful preparation does
not authorise the use of an archive that another process modified later.

## Reused execution seams

Linux standalone confinement also requires the real bubblewrap backend of the
test environment. [Linux floor backend preparation](linux-floor-backend.md)
provides pinned target builds and execution diagnostics. Namespace availability
and the actual Grove standalone acceptance stay required separately.

`ravel-target-smoke.sh` sources the existing release platform runner and
substitutes its own target and case scripts. It keeps the pinned images, the QEMU
emulators, the Zig-built probes, the native container and the private binfmt
namespace machinery. The shared environment helper supplies the target, system
and glibc-floor assertions. QEMU checks every archived ELF header against its
registration. It also requires a complete front and worker pair, so neither
executable escapes the CPU model.

Grove took the Ravel installed cases from the MIT-licensed released source at the
v1.0.0 tag. They cover these subjects:

- direct and relative-symlink prefixes.
- the public SDK, sample and choice boundary.
- exact, empty, multiline and flag-like results.
- the separation of child output.
- TypeScript relative and package imports.
- measured context and params.
- resource cleanup.
- missing and stale session context.

Only shell fake agents run. The PATH of the child admits selected system
utilities. It excludes Grove, legacy dispatch, Bun and Node. The original
session controls and the new ones do not enter these `env -i` children.

macOS arm64 runs natively. Linux arm64 runs natively in CentOS 7.9 and under
QEMU 10.2.2 Cortex-A53. Linux x86-64 runs under QEMU 10.2.3 Nehalem.

Linux uses
glibc 2.17 with an observed GLIBC_2.25 refusal. The CPU controls require SIGILL
(status 132) beyond the floor. They also require success under the same
emulator with `-cpu max`. These checks need arm64 Docker and Zig, and they fail
if either is unavailable. They do not measure a kernel version floor. See
[release prerequisites](RELEASING.md).

## Measured digests of Ravel 1.1.0 — 2026-10-22

Grove moved the pins from Ravel 1.0.0 to Ravel 1.1.0, because Grove now requires
Ravel 1.1.0 or later in the stable 1.x line. The three archives came from the
published release with `curl`, each from its URL above. `shasum -a 256` measured
each digest. The digests equal the `digest` field that GitHub reports for each
asset. The release has the same three targets as Ravel 1.0.0.

The archive controls (`task ravel:targets:controls`) then passed against these
three archives. The 27 stdlib cases passed, and the real-layout controls passed.
The file manifest in `scripts/ravel_target_inputs.py` did not change.

<!-- tc: off because dated evidence of a bounded run stays unchanged (docs/adr/evidence-outlives-the-instrument.md) -->
## Verified delivery input handoff — 2026-10-13

The actual default prepare task downloaded all three public archives and matched
the pins above. All four platform routes passed, with direct/symlink execution,
opaque results, public context and resource cleanup in each environment. Both
QEMU runs observed the CPU SIGILL/132 refusal and successful `-cpu max` twin;
every Linux run observed the GLIBC_2.25 refusal at glibc 2.17. No model launched.

The 27 stdlib archive cases passed. Real-input controls passed for missing inputs,
tampered bytes, a renamed wrong-target archive, partial target sets, existing
output and corrupted default cache. The runner refused bad input before execution;
its actual registration assertion admitted the correct pair and refused a wrong
architecture and mixed Ravel/legacy pair. Ruff and shellcheck passed. A bounded
fresh read found no additional contract violations.

The final `task check` passed all 15 principal checks, including 1,723 selected
workspace nextest cases, nine doctests and six final/all books. The existing 31
ignored workspace cases remain outside that default suite; the target routes and
archive controls described above run explicitly without skips. Development-only
compiler checks used the already present Bun 1.4.2 on PATH. Installed Ravel child
PATHs contain neither Bun nor Node. No host runtime installation was performed.

Complete local records are under Grove's `target/k48/`:

- `public-download.log`, `public-inputs/`, and `../ravel-archives/`: actual task
  fetch/admission output, retained separate layouts and exact published inputs.
- `check-final.log`, `controls-final.log`, `final-platforms.log`: complete passing
  repository, real/fixture refusal and all-platform records.
- `measure.py`, `before.json`, `after-first.json`, `comparison-first.json`,
  `before-final.json`, `after-final.json`, `comparison-final.json`: individual
  SHA-256 subjects and comparisons. All 18,172 named subjects match within both
  runs: tracked Grove/adjacent sources, dependency trees, measured tools,
  retained archives/layouts, owner configuration and complete installed original
  Grove 22.1.0 and Ravel 1.0.0 trees. The only between-run file changes were the
  registration control fixture and its task note; platform/production inputs did
  not move. Generated run outputs are excluded from this static subject set.
- `ravel-state-final.txt`, `tap-state-final.txt`, `grove-state-final.txt`: clean
  adjacent checkouts at Ravel main `3916a4a1`, tap main `bd5aa37b`, and default
  Grove main `89f947d5`, respectively.
- `red.log`, `real-red.log`, `native.log`, `linux.log`, `trap-red.log`,
  `check.log`, `final-controls.log`: full initial/disproof records. These include
  the missing implementation, corrected cleanup/test-fixture defects and the
  first full check's refusal of host Bun 1.4.3. The final check uses the existing
  `/Users/antony/Development/Ravel/target/tools/bun` 1.4.2, without changing
  installation or runtime inputs.

The original driver, its complete keg and ending/admission channels are retained.
This handoff supplies dependency inputs only. The later delivery contraction owns
switching Grove's smoke consumers/archives/formula, and final delivery acceptance
owns rebuilding that new combination on all targets.
