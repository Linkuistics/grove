# ravel-delivery-k4

## Goal

Deliver Ravel independently through its release tasks, three supported archives
and `linkuistics/taps/ravel`, ready for Grove to consume as an installed command.

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
  recovery. Use its existing repository/version history when choosing the first
  cut; its version lifecycle is independent of Grove's.
- Archives contain `ravel`, its matching private worker, generic SDK/examples
  and notices. They contain no Grove binary, adapter or policy. Exact manifest
  checks and their omission/extra/probe controls pass.
- The moved release tests prove prerequisite failures stop before the cut,
  dirty taps refuse before publication, and smoke runs before pushes/releases.
  Every supported target executes the installed-layout seam without Grove or a
  host TypeScript runtime; report missing prerequisites as unfinished work.
- Publish the verified Ravel release and formula, then install and `brew test`
  Ravel. The Homebrew-linked front finds its own matching worker from its real
  path. Generic fake-agent selection/run/record checks succeed from the installed
  command, using the preserved personal config where applicable.
- Record the published version and installation evidence in Ravel's normal
  release documentation and this leaf's handoff; Grove's existing installation
  still works. If a cut/publication partly succeeds, resume it rather than
  rerunning a task that would bump again.

## Notes

This is delivery work in `/Users/antony/Development/Ravel` and the existing tap.
Commit intended changes through each repository's VCS interface. Do not publish
or install a new Grove here. Preserve the original live session ending command
until the session has signalled from its original workspace.
