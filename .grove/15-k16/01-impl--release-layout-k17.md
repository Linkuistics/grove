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
