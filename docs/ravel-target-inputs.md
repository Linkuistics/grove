# Independent Ravel inputs for Grove delivery tests

Grove's target tests consume released **Ravel 1.0.0**, independently of Grove's
version, archive manifest and source tree. No adjacent Ravel checkout, Homebrew,
host Bun/Node, installed-front replacement or release operation is required.
Grove's release smoke now admits these independent inputs before running its
installed callers. No Ravel executable or worker is bundled in a Grove archive.

## Pinned public inputs

The source/peeled tag is `bbab823803bd2f2796f8bd9570f732dce900efa2`.
Each URL is under
`https://github.com/Linkuistics/Ravel/releases/download/v1.0.0/`.
GitHub permits asset replacement; the SHA-256 pins enforce the tested identity.
The public [release](https://github.com/Linkuistics/Ravel/releases/tag/v1.0.0)
and Ravel's `docs/publication-evidence.md` record publication provenance.

| Archive | SHA-256 |
|---|---|
| ravel-v1.0.0-aarch64-apple-darwin.tar.xz | `d1f253db86cde3c625950cda2d8fa929ca4d1e4529ca6c3f223e138f9be9a922` |
| ravel-v1.0.0-aarch64-unknown-linux-gnu.tar.xz | `288158c1259667dbd11d9f71aa5d643b4da4e08ffc4402524f4852afb314f732` |
| ravel-v1.0.0-x86_64-unknown-linux-gnu.tar.xz | `1264ea0f3dd113f5dbb0914992ee9a7734285c0ddcc161e9acaffd83c037f9a9` |

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
per target on stdout, and returns nonzero on any failure. The output is exposed
only after all requested archives pass. Within `OUT`:

```text
archives/ravel-v1.0.0-TARGET.tar.xz
TARGET/ravel-v1.0.0-TARGET/bin/ravel
TARGET/ravel-v1.0.0-TARGET/libexec/ravel/...
```

The release smoke places Grove in a different installation prefix and supplies the matching
Ravel `bin` on its test child's PATH, or passes the absolute public front where
the caller fixture permits it. Ravel locates its own worker from its real front
path. Retain its whole prefix; do not move only the front, compare its version to
Grove, or use a shared Cellar. Use Ravel's `start-session`/`end-session` and opaque
returned result; its inherited context is Ravel's own public session boundary.

Every cached or explicit archive is read into private bytes, SHA-256 checked,
and parsed completely before any member is extracted. Exact top/version/target,
all files/directories, uniqueness, regular-file types, executable modes and both
programs' ELF/Mach-O architectures are checked. Missing/extra/traversal/AppleDouble
members, links, special modes and wrong targets fail. Extraction writes only
admitted regular files/directories. A corrupt cache fails rather than being
refetched; remove or repair that explicitly before retrying. Missing explicit
archives never fetch. There is no CLI digest/manifest override.

For installed smoke, prepare a new snapshot every run. The runner stages its
validated archives read-only into Linux environments. These artifacts remain
ordinary writable files on the host; an earlier successful preparation does not
authorize invoking an archive later modified by another process.

## Reused execution seams

Linux standalone confinement additionally requires the test environment's real
bubblewrap backend. [Linux floor backend preparation](linux-floor-backend.md)
provides pinned target builds and execution diagnostics; namespace availability
and actual Grove standalone acceptance remain required separately.

`ravel-target-smoke.sh` sources the existing release platform runner, substituting
its target/case scripts while retaining the pinned images, QEMU emulators,
Zig-built probes, native container and private binfmt namespace machinery.
The shared environment helper supplies target/system and glibc-floor assertions.
QEMU checks every archived ELF header against its registration and requires a
complete front/worker pair, so neither executable escapes the CPU model.

The Ravel installed cases were transferred from its MIT-licensed released source
at the tag above. They cover direct and relative-symlink prefixes, the public
SDK/sample/choice boundary, exact/empty/multiline/flag-like results, child-output
separation, TypeScript relative/package imports, measured context and params,
resource cleanup and missing/stale session context. Only shell fake agents run.
The child PATH admits selected system utilities and excludes Grove, legacy
dispatch, Bun and Node. Original and new session controls do not enter these
`env -i` children.

macOS arm64 runs natively. Linux arm64 runs natively in CentOS 7.9 and under
QEMU 10.2.2 Cortex-A53; Linux x86-64 runs under QEMU 10.2.3 Nehalem. Linux uses
glibc 2.17 with an observed GLIBC_2.25 refusal. CPU controls require SIGILL/132
beyond the floor and success under the same emulator's `-cpu max`. These checks
need arm64 Docker and Zig and fail if unavailable; they do not measure a kernel
version floor. See [release prerequisites](RELEASING.md).

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
