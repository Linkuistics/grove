# Independent Ravel inputs for Grove delivery tests

Grove's target tests consume two **lanes** of independent Ravel archives. The
1.1 lane is the released **Ravel 1.1.0**, with `policy.ts`. The 2.0 lane is the
released **Ravel 2.0.0**, with `config.rhai`. Each lane has a record in
`scripts/ravel_target_inputs.py`, and admission accepts only the archives of
that record. No lane tests a Ravel that the PATH finds.

The tests do not depend on
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

## The 2.0 lane

The 2.0 lane records the published Ravel 2.0.0. Ravel's release tooling built
its archives at the tag `v2.0.0` and published them through the keg-only
formula `ravel@2`. Each URL is under
`https://github.com/Linkuistics/Ravel/releases/download/v2.0.0/`. The public
[release](https://github.com/Linkuistics/Ravel/releases/tag/v2.0.0) and the
section "Ravel 2.0.0 publication receipt" of
[the coordinated release evidence](coordinated-release-evidence.md) record the
publication provenance.

| Subject | Record |
|---|---|
| Source commit | `ed6cedbf9a14cd088bea13fcfb8b86300cb6aaca`, the peeled commit of the tag `v2.0.0` |
| Build identity | `7f2ff6b3f5222c13b69b78b8cd2c33d1c7edaa6a8599cfb27ebfb59d209d8ec5` |
| Archive name | `ravel-v2.0.0-TARGET.tar.xz` |
| Default directory | `target/ravel-archives-2.0`. Admission downloads each missing archive into it, as for the 1.1 lane |

| Target | SHA-256 of the archive |
|---|---|
| aarch64-apple-darwin | `5957840ab33f4331b794539dbd455d801f71c6c6485c781dc290cfb48490231d` |
| aarch64-unknown-linux-gnu | `a4750ea439450e88621be411183e7bf30d2616f9a545b64a30445df3a9f02bee` |
| x86_64-unknown-linux-gnu | `906e589f72161ed5dce5db683e6af10963c1952bdf177c6b343d1918928c3dd1` |

The lane records every target, so `unrecorded` is empty. Both Linux
executables need at most the symbol version `GLIBC_2.17`. An earlier local
build, of `ravel-local-sources-k90`, did not link at glibc 2.17. Its
`ravel::skills::dir::Dir::rename_noreplace` called the glibc `renameat2`
wrapper, which glibc added in 2.28.

The 2.0 manifest is `LICENSE`, `README.md`, `bin/ravel` and
`share/ravel/examples/{llm,sample,static}.rhai`, with
`share/ravel/notices/THIRD-PARTY-NOTICES.md`. Only `bin/ravel` is executable.
A 2.0 prefix has no `libexec/ravel`. The front is also the Rhai worker.

The archives hold no installation record `share/ravel/installation.json`.
Only Ravel's Homebrew formula writes one. So an admitted prefix has no default
skill directory, and its launches deliver skills only from settings.

Ravel's `build.rs` embeds the build identity in `bin/ravel` as ASCII text.
Admission checks the digest of the archive first. It then reads the bytes of
`bin/ravel` and refuses an executable that does not hold the recorded
identity. Admission runs no program, as for the 1.1 lane.

A target without a record refuses before admission reads any archive. The
refusal states the reason in the record. A unit case proves this refusal with a
synthetic lane, because the real record now covers every target. `--status` prints one line for each
target: `recorded` with the archive name, or `not recorded` with the reason.
Each runner that uses the 2.0 lane names each target that it did not run.

The 2.0 lane runs these checks:

- `ravel-target-smoke.sh` runs `scripts/ravel-installed-smoke-rhai.sh`. These
  cases are a port of Ravel's own installed smoke at the source commit. They
  copy `sample.rhai` and `static.rhai` into `config.rhai`. They check Rhai
  imports, measured context, params and the four result shapes. They also check
  the `config_format_retired` refusal of a TypeScript entry.
  On Linux they run natively in the floor container and under QEMU.
- Grove's release smoke gives the caller cases a 2.0 prefix as their fourth
  argument, on every target. `grove-installed-smoke.sh` then checks the Rhai
  caller set with `config.rhai`, in a HOME of its own.
- On Linux, the general route stages the 2.0 archive in `ravel-2.0/` of its
  mount, apart from the 1.1 archive of the same name. In the floor container
  and under QEMU, `grove-floor-directory-case.sh` then runs the directory case
  on the 2.0 lane. It publishes skill snapshots from the release archive at
  the glibc 2.17 floor, and the file modes of each snapshot refuse a write.

## Replacement of the local 2.0 record — 2026-11-12

Before Ravel's cut, the 2.0 lane recorded a local build of the workspace
`Ravel.skill-directory-delivery`. That build had the archive name
`ravel-v1.1.0-TARGET.tar.xz`, and admission never downloaded it.
`grove-ravel-two-pins-k314` replaced that record with the published 2.0.0
archives, by these steps:

1. Download the three published archives into a new empty directory.
2. Measure the SHA-256 of each archive with `shasum -a 256`.
3. Compare each value with the `digest` field that GitHub reports for the asset.
4. Set `stem`, `pins`, `release` and `source_commit` in the `"2.0"` lane of
   `scripts/ravel_target_inputs.py`. Keep `unrecorded` empty.
5. Set `build_identity` to the `RAVEL_BUILD_ID` that Ravel's `build.rs` gives
   for the tag commit.
6. Replace the two tables of the 2.0 lane.
7. Run the target controls and smoke, the release smoke and the installed checks.

Each measured digest equalled the GitHub `digest` and the publication receipt.
The manifest equalled the recorded 2.0 manifest. A re-implementation of
`crates/ravel/build.rs` over the git objects of the tag commit gave the build
identity. All three executables and the installed `ravel@2` keg hold it. None
holds the identity of the local build, `58d479ed…`.

Both Linux executables need at most `GLIBC_2.17`, as their ELF version
records show. These checks then passed on the published archives. The
installed checks used the `ravel@2` keg `/opt/homebrew/opt/ravel@2`:

| Check | Result |
|---|---|
| `task ravel:targets:controls ARCHIVES=target/ravel-archives` | 35 unit cases, and the shell controls of both lanes |
| `task ravel:targets:smoke` | Each lane on each of the three targets |
| `task release:smoke` | The three targets. On Linux, the 2.0 lane passed the directory case at the floor |
| `task release:installed` | The TypeScript set on 1.1.0, and the Rhai set and the six keg sources on `ravel@2` |
| `task release:formula:test` | The install and test methods, tap states A to D, and the shared directory seam, with `OLD` the published Grove 22.8.0 keg |
| `task ravel:directory` | 11 cases, with the `ravel@2` front and the 1.1.0 front as `OLD` |
| `task seam:shared-directory` | 10 cases, with the tap's skill formulae as subjects |

Two instruments changed. The shell controls take each 2.0 archive name from
`--status`, and they swap bytes under each lane's own name. The seam renders
the `@CLASS@` and `@KEG_ONLY@` placeholders of Ravel's 2.0.0 template as the
unversioned channel. The seam's Ravel formula was that template at the tag
commit.

Grove's release task runs `scripts/release-smoke.sh --archives target/dist`.
That smoke admits the 2.0 lane from its default directory, and it downloads
the published archives when they are missing.

## Consumer interface

```sh
# Download missing inputs into target/ravel-archives and prepare a fresh layout.
task ravel:targets:prepare OUT=target/my-ravel-inputs

# Strict offline admission of an explicit directory; no network fallback.
task ravel:targets:prepare ARCHIVES=target/ravel-archives OUT=target/my-other-inputs

# Full manifest/target/digest and real-input refusal controls.
task ravel:targets:controls ARCHIVES=target/ravel-archives

# All native/container/QEMU routes of both lanes; TARGETS may select targets.
task ravel:targets:smoke ARCHIVES=target/ravel-archives

# The 2.0 lane: admission, its status, and an explicit archive directory.
task ravel:targets:prepare LANE=2.0 OUT=target/my-ravel-2-inputs
task ravel:targets:status
task ravel:targets:smoke ARCHIVES_2=target/ravel-archives-2.0
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

`ravel-target-smoke.sh` sources the existing release platform runner and
substitutes its own target and case scripts. It keeps the pinned images, the QEMU
emulators, the Zig-built probes, the native container and the private binfmt
namespace machinery. The shared environment helper supplies the target, system
and glibc-floor assertions.

QEMU checks every archived ELF header against its registration, so no
executable escapes the CPU model. A 1.1 prefix must hold both the front and the
worker. A 2.0 prefix must hold its front and no worker. The runner labels each
path of a 2.0 archive with `ravel-2.0/`, and the check reads that label. Before
Ravel's cut the two lanes had the same archive names, and the label still
keeps them apart.

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
