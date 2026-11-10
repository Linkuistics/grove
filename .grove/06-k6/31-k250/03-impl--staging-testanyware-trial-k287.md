# staging-testanyware-trial-k287

## Goal

Prove on each harness in the staging HOME that the TestAnyware guidance
reaches its command reference and its installed command through Ravel
delivery.

## Context

- The parent brief holds the trial list and the staging layout.
- "Resources and requirements of each source" of
  `docs/repository-moves-record.md` names the tools of the `testanyware`
  source.
- The staging HOME does not isolate SSH. OpenSSH reads the account's real
  `~/.ssh`, as the guidance case of owner-staging-discovery-k249 found.

## Done when

- On each harness, an agent reads `using-testanyware` and its command
  reference from the `testanyware` source.
- On each harness, the agent runs one real VM operation through the
  installed `testanyware` command, and the VM is gone afterwards.
- The evidence document records each trial, its versions and its result.
