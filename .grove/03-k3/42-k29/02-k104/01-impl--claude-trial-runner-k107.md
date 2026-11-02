# claude-trial-runner-k107

## Goal

Build a reusable runner for real harness trials, with its Claude Code adapter.
Prove it with fake-harness tests and one real smoke launch.

## Context

- Ravel's `scripts/harness-smoke.sh` and `scripts/harness-smoke.test.sh`, in
  `Ravel.skill-directory-delivery`. They build the sandbox home for one call
  inside a configuration script. Adapt the construction, not the call shape.
- The "Sandbox home for real harness runs" section of Ravel's
  `docs/specs/configuration-evaluation.md`.
- The Recipes section of `docs/grove-dual-ravel-stage-record.md`.
- `Sources::plant` and the network-denying profile in
  `crates/grove/tests/directory_route.rs`.
- The launch policy examples in `docs/USAGE.md` and the Rhai caller assets.

## Done when

- An opt-in Taskfile task runs one named case. It builds a sandbox home, a
  disposable repository with a Grove tree and the settings of the case's
  route. It then runs a scratch Grove driver with the 2.0 front first on
  `PATH`.
- The sandbox home links only `Library/Keychains`. The runner lists every
  link before and after the launch. It compares the effective configuration
  directory with the `configDirectory` of `claude auth status`.
- The runner refuses the five Claude Code location variables before any
  launch of the harness. It launches the driver from an allow-listed
  environment.
- A control shows that a trial child sees none of the live session's control
  names. A planted name makes that control fail once.
- Each case record names the builds, the harness, the model, the version, the
  effort, the sources, each `resourceDigest` and the descriptor identity. It
  also holds the native startup text, the transcript digest, the ordered
  events and the ending outcome.
- The ending outcome states the Ravel result, the leaf state, any teardown
  record, any published output and the number of launches.
- Fake-harness tests prove the refusals, the allow-list, the link list and the
  event extraction. Each test fails once against a deliberately broken runner.
- One real smoke launch reaches Claude Code with native copies absent. It
  records the configuration directory, any synced claude.ai skills and the
  native startup text. It credits no acceptance case.
- `docs/common-loading-acceptance-record.md` exists, with its conventions and
  the smoke record.

## Notes

Build the Grove executable from a recorded commit of this line into a scratch
prefix. Install the 2.0 build by the stage record's recipe, and check its
digest.

A trial session's ending command names the 2.0 front of the trial. It reaches
the trial's own Ravel channel. It never reaches the live 1.1 keg or this
session.

Choose the permission mode in the configuration script. Reads outside the
working directory must not need a prompt. Do not choose a mode that disables
Ravel confinement. Record the choice and the model.

The Claude Code session file under the sandbox `.claude/projects` is the
likely source of the ordered events. Check that each read and each tool
action appears there with its absolute path.
