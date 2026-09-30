# installed-smoke-k18 — brief

## Goal

Add a reusable per-target smoke task. It extracts a release archive and
executes the installed static TypeScript case with a fake harness, with no
separately installed runtime. It runs natively on macOS arm64 and in a
glibc-2.17 userland container on each Linux target.

## Context

The spec's `#delivery` Linux-floor table owns the claims. This leaf covers the
C-library dimension and host-native execution. The CPU models under emulation
are `cpu-floor-k19`'s. A matching runtime archive is not evidence until this
test executes.

## Done when

- Taskfile tasks run the installed smoke for a named target and for all
  targets. Each extracts the built archive into a fresh prefix. It runs
  `harness-dispatch inspect` and `run` against a static TypeScript policy and a
  fake harness, with PATH holding no Bun or Node. It asserts the selected
  candidate, the expanded argv and the fake harness's received arguments and
  exit. `run` uses a temporary state directory, and `record show` reads back
  its committed run, so the bundled SQLite executes on every target.
- The macOS arm64 case runs natively and also resolves its worker through a
  symlinked front executable, as Homebrew installs it.
- Each Linux target runs the same case in a glibc-2.17 userland container
  matching its architecture.
- Positive control: in the same container, a probe binary built against a newer
  glibc is seen to fail. This shows the container really enforces the 2.17
  floor. The control's observed failure is recorded in the task's output or the
  runtime evidence.
- Later increments can add cases without restructuring the task. The computed
  `select` case arrives with `computed-selection-k21`.
- The runtime evidence document records what executed, where, and which Bun and
  container images were used.

## Decomposition

The x64 case could not execute where the leaf expected it to: Docker Desktop's
emulator crashes the glibc-2.17 x86-64 userland on this arm64 host. So the leaf
became this node.

1. `smoke-instrument-k48`: the tasks, the scripts and their extensible case
   list, macOS arm64 natively, Linux arm64 in the floor container, and the glibc
   control on both Linux architectures, with its runtime evidence.
2. `x64-floor-k49`: an x86-64 environment that runs the glibc-2.17 userland,
   and the x64 case executed in it.
