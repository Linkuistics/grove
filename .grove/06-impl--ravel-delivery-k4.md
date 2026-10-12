# ravel-delivery-k4

## Goal

Prepare Ravel's independent release tasks, three supported unreleased archives
and tested formula template, ready for ravel-publication-k11 to publish/install.

## Context

- Consume ravel-cli-k3's committed Ravel source and validated build/install tasks.
- Adapt the actual `scripts/release-{common,build,doctor,prepare,publish,smoke*}`,
  release tests, release configuration and formula template. Retain the pinned
  runtime fetch/digest checks, glibc/CPU instruments and archive assertions.
- Supported targets: aarch64-apple-darwin, aarch64-unknown-linux-gnu and
  x86_64-unknown-linux-gnu. Keep Bun 1.4.2, glibc 2.17 and current CPU floors.

## Done when

- Ravel owns version cuts, release notes, check/build/install and archive/smoke
  tasks in its root Taskfile, documented prerequisites and partial-release
  recovery. Use k3's 0.0.0 development version and prepare a first minor cut to
  0.1.0 after rechecking remote version tags; its lifecycle is independent of
  Grove's. No tag or public cut here.
- Archives contain `ravel`, its matching private worker, generic SDK/examples
  and notices. They contain no Grove binary, adapter or policy. Exact manifest
  checks and their omission/extra/probe controls pass.
- The moved release tests prove prerequisite failures stop before the cut,
  dirty taps refuse before publication, and smoke runs before pushes/releases.
  Every supported target executes the installed-layout seam without Grove or a
  host TypeScript runtime; report missing prerequisites as unfinished work.
- The `ravel.rb` template installs the independent layout and passes its
  fake-agent checks from a symlinked temporary prefix. The tap's `ravel-lite.rb`
  is a different command and remains intact. Formula publication is k11.
- Record source commit, unreleased artifact paths/digests, per-target smoke
  evidence and validated publication/recovery commands in the handoff. The
  pinned Bun runtime evidence moved by k3 remains cited by build/release scripts.
  Missing prerequisites or unrun target checks are unfinished work, never skips.

## Notes

This is preparation in `/Users/antony/Development/Ravel`. Commit the working
recipes and instruments through jj; publication/installation is k11's separate
verified boundary. Do not publish/install Grove or replace the running tool.
