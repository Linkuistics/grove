# release-layout-k17

## Goal

Make each release archive and the Homebrew formula carry the complete
`harness-dispatch` installation, in a relative layout the front executable
resolves from its real path.

## Context

Today's archives are flat, holding `grove`, `grove-llm`, `LICENSE` and `README.md`,
and the formula runs `bin.install` on the two binaries. A worker that is found
relative to the real front executable needs a layout the archive and the
Homebrew Cellar share. Choose that layout. If it moves the existing binaries,
update everything that assumes the flat archive, including the release
verification and the documentation. Record the choice in the running log.

Bun cross-compiles by fetching target runtimes at build time. Pin and verify
what it fetches, so that a release never builds against an unpinned runtime.

The front executable links bundled SQLite from `handoff-records-k24`. That
brings C into the Linux cross-builds, which use zigbuild at glibc 2.17. A clean
host build is not evidence that they work.

## Done when

- The release build compiles the worker for all three targets with Bun 1.4.2
  and the no-autoload switches. It stages the front executable, the worker, the
  SDK and example declarations and readable sources, and the runtime and
  license notices. The notices include those Bun 1.4.2 documents for its
  embedded components, and SQLite's. Each Linux front executable is
  cross-built with bundled SQLite at the glibc 2.17 floor.
- The formula installs the layout, and the front resolves its worker through
  the `bin` symlink. `brew test` checks that the front, the worker and Grove
  report one version. The release task's verification checks
  `harness-dispatch --version`, and the asset count stays three.
- `crates/harness-dispatch` opts out of a second cargo-release cut, and its
  worker embeds the workspace version.
- The release script tests assert each archive's contents. They check the front,
  the worker, the declarations, the sources and the notices at their layout
  paths. An omitted file fails the assertion.
- A locally built archive, extracted into a temporary prefix, resolves its
  worker and inspects a static policy on the host. The full per-target smoke
  is the next leaf's.
- `docs/RELEASING.md` names the new build prerequisites.

## Notes from routed-inspection-k13

- The front resolves its worker at `../libexec/harness-dispatch/harness-dispatch-policy`
  from its canonical path (spec `#delivery`), with the SDK declarations and
  source in `sdk/` beside it. The flat archives need `bin/` and `libexec/`, or
  that one constant (`WORKER_FROM_BIN` in `src/worker.rs`) changes.
- `crates/harness-dispatch/scripts/dispatch.sh build OUT_DIR` compiles the
  worker and emits the SDK declarations; extend it with a Bun `--target` rather
  than duplicating the four switches. Bun 1.4.2 leaves its ~60 MB compile
  template in its cwd, which is why the script compiles from a scratch
  directory.
- `docs/RELEASING.md`, *One release, seven packages, one tag*, now names
  harness-dispatch as a ninth member the cut does not ship yet. When the
  archives carry it, update that section's count. Five walkthrough books'
  orientation chapters, six crate manifests, `CONTEXT-MAP.md` and
  `docs/specs/jj-workspace-book-structure.md` cite the heading by name, so a
  rename moves source-exact book fragments too.

## Decisions (running log)

**Layout: the archive's top directory is an installation prefix.** Each
`grove-v<ver>-<target>/` holds `bin/grove`, `bin/grove-llm`,
`bin/harness-dispatch`, `libexec/harness-dispatch/` (worker, `sdk/`,
`examples/`, `notices/`), and `LICENSE` and `README.md` at the top. The two
existing binaries move into `bin/` rather than leaving a flat/prefix hybrid, so
the extracted directory and the Homebrew keg have one shape and
`WORKER_FROM_BIN` stays as the spec states it. The formula installs
`bin/*` by name and `libexec.install "libexec/harness-dispatch"`, which
Homebrew 7.0.7's `Pathname#install_p` moves as one directory.

**Homebrew leaves both executables' bytes alone.** Read from the installed
Homebrew 7.0.7 source: on Linux a source (non-bottle) install's
`fix_dynamic_linkage` only relativizes absolute symlinks; `patchelf` runs only
when bottling/pouring, and even then skips files with a `.bun` section. On macOS
it rewrites only non-absolute install names; both executables link absolute
system paths. So no relocation can strip the worker's embedded module graph.

**Target runtimes are fetched and verified by `dispatch.sh`, never by Bun.**
Read from the `bun-v1.4.2` source (`src/options_types/compile_target.rs`,
`src/standalone_graph/StandaloneModuleGraph.rs`): a cross-compile fetches
`https://registry.npmjs.org/@oven/bun-<os>-<arch>/-/…-1.4.2.tgz` into
`$BUN_INSTALL_CACHE_DIR` (default `~/.bun/install/cache`) with no integrity
check, and reuses any cached file of the right name unverified.
`--compile-executable-path` bypasses the fetch entirely. `dispatch.sh build
--target` downloads the same three tarballs itself, checks each against a
SHA-256 pinned in the script, and passes the runtime inside. Observed on
2026-09-30: each tarball's SHA-512 equals npm's published `dist.integrity`; a
pinned compile with an empty isolated cache left the cache empty, and the
positive control (the same compile without the flag) was seen to download
`bun-linux-x64-v1.4.2` into it, byte-identical to the pinned tarball's `bun`.
Every release target, darwin included, compiles from a pinned runtime, so the
host's Bun is only the compiler driver.

**Notices live in the package and ship beside the worker.**
`crates/harness-dispatch/notices/` holds an index, Bun 1.4.2's `LICENSE.md`
verbatim from its tag (its documented list of statically linked components,
including LGPL-2 JavaScriptCore/WebKit and where their source is), and
SQLite's public-domain blessing. `dispatch.sh build` copies them to
`notices/` beside the worker, so source installs carry them too. The npm
runtime tarballs carry no licence file.

**Cross-built front with bundled SQLite compiles at the floor.** `cargo
zigbuild --release --target x86_64-unknown-linux-gnu.2.17 -p harness-dispatch`
built on the host in 22 s; the highest glibc symbol version referenced is
`GLIBC_2.17`. That is a build observation, not floor evidence: the spec admits
only execution in a 2.17 userland (`installed-smoke-k18`).

**Local archives for later leaves.** `release-build.sh --unreleased [TARGET…]`
builds archives of the working copy into `target/archives/` with no tag,
formula, doctor or clean-tree requirement; `task release:archives` wraps it.
`installed-smoke-k18` extracts these. The real build asserts every archive
against the manifest in `release-common.sh` before rendering the formula, so
an omitted or unexpected file stops it; `cpu-floor-k19` still owns wiring the
smoke into `task release`.

**The heading becomes *One release, eight packages, one tag*.** harness-dispatch
now ships, so the count moves. "seven" and "eight" have the same length, so
every manifest comment and the source-exact book fragments quoting them keep
their line breaks; the substitution is applied to all of them together.

**All three archives build, and the host archive runs.** `scripts/release-build.sh
--unreleased` built all three targets in 1 m 40 s on 2026-09-30, each passing
the manifest check as it was packed. The macOS arm64 archive, extracted into a
fresh prefix and run with `env -i PATH=/usr/bin:/bin` (no Bun or Node), reported
`grove`, `grove-llm` and `harness-dispatch` 21.12.0 and inspected a static
policy both directly and through a symlinked front, each resolving
`libexec/harness-dispatch/harness-dispatch-policy` (package 21.12.0, Bun 1.4.2).
Control: the same front *copied* outside the prefix refused `worker_missing`,
exit 5.

**The formula was exercised, not only rendered.** A probe rendered from the
template (class renamed, `keg_only` so nothing linked over the live Grove,
`url` a `file://` of the local archive) installed from a throwaway local tap
with exactly the archive's layout; both executables were byte-identical to the
archive's. `brew test` passed under Homebrew's sandbox; with the worker moved
away it failed on `worker_missing` (the control). The probe keg and tap were
removed afterwards. Rendering first caught that `args: []` is an invalid policy
(the `prompt` slot must appear once), which would have failed the real
release's `brew test` after publication.

**The leaf's one in-session review, spent and triaged.** Claim put to a fresh
context, conclusions stripped: no path through `release-build.sh` packs a
worker from an unpinned runtime or an archive that differs from the manifest,
and every failure stops before the formula. It returned 13 findings; each was
re-read against the source and classified:

- F1 AppleDouble — **valid, most severe.** Reproduced: the built x64 archive
  holds 22 hidden `._*` members and `com.apple.provenance` xattr pax headers;
  bsdtar merges them away on `-t` and on extract, so `assert_archive` passed.
  (Pre-existing in every earlier archive; now Linux Homebrew would move them
  into the keg with `libexec`.) Fix: pack with `COPYFILE_DISABLE=1
  --no-xattrs` through one shared `pack_archive`; `assert_archive` reads with
  `--options '!mac-ext'` on bsdtar, which exposes them. Seen both ways in
  scratch.
- F2 stale/partial archives — valid. The output directory is emptied before
  anything can fail, and each archive is packed as `.partial` and renamed only
  after it passes.
- F3 re-derived manifest rules — valid. The test now runs the real `dispatch.sh
  build` into scratch and compares what it emits, so there is no second rule
  to drift; `check.sh` runs the release tests after the worker's deps exist.
- F4 no version check before publication — valid for the tag/workspace
  mismatch (now refused); executing the Linux archives is `installed-smoke-k18`
  and `cpu-floor-k19`'s, a visible trade-off.
- F5 hard-coded target directory — valid (pre-existing); the build pins
  `CARGO_TARGET_DIR` to the directory it copies from.
- F6 extra directories pass — valid; the check compares directories too.
- F7 test wiring — subset of executables in the manifest, exact-line refusal
  matching and leftover `@…@` placeholders: valid, fixed. A grep that the
  build calls `assert_archive`: visible trade-off, not added (two lines, read
  in review; a text match on them would be brittle).
- F8 relative OUT_DIR loses the worker — valid (pre-existing); OUT_DIR is made
  absolute first.
- F9 verify-then-reread TOCTOU — valid, minor; downloads land in a private
  temporary and the build verifies and extracts one private copy.
- F10 bash 3.2 fails after building — valid, minor; refused up front.
- F11 no `--locked` — valid, minor; added.
- F12 weak assertions — valid; the formula compares whole version lines, and
  the release task checks the Homebrew prefix's commands rather than PATH's
  first match (`dispatch:install` defaults to `~/.local/bin`).
- F13 nits — valid; fixed.

Every fix is covered by a release-test case or an observation below, so none
forces a second review.

**Post-fix evidence, on the finished source.** Each new release-test control was
seen to fail under a deliberate mutation and pass once restored: a reader that
merges AppleDouble, a pack that keeps AppleDouble, a pack that keeps xattrs,
a directory-blind check, and an unmanifested notice. The pinned-runtime
paths were rerun: a corrupted cache was refetched, and a wrong pin refused with
no worker built, no stray download and the true cache kept. `task
release:archives` rebuilt all three archives, each with 22 members (15 files,
7 directories) and no AppleDouble member or xattr header, read with Python's
`tarfile`. The published v21.12.0 archive, read the same way, does carry them,
which the changelog now says. The rebuilt macOS archive passed the host check
again, through a symlinked front, and passed the Homebrew probe again, with the
exact version-line assertions; the keg held no `._*` file, the worker-moved
control failed, and the probe was removed. Both Linux fronts reference no glibc
symbol above `GLIBC_2.17`. `task check`: all 12 principal checks pass, with 6
books and none failing.

**Left for the node's later leaves.** `installed-smoke-k18` extracts
`target/archives/` (built by `task release:archives TARGETS=…`) for the
per-target smoke tests. `cpu-floor-k19` wires them into `task release`.
`release-build.sh` already checks each archive's contents before the formula
exists. Neither Linux archive has been executed yet.
