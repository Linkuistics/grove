# finish-k4

## Goal

Propose the complete finish cycle and wait for explicit human confirmation.

## Done when

- Promote durable material from the grove briefs.
- Run `grove-llm finish-commit finish-k4`.
- Run `grove-llm complete --done` as the last action.

## Decisions (running log)

- The human confirmed the complete finish sequence: helper teardown, integration
  onto current `main`, checks and jj push, followed by `task release:minor` from
  the default workspace, publication and installation verification. Return to
  this workspace for the final done signal; preserve the workspace.
- Durable release and configuration docs already capture the brief's decisions.
  All ten principal checks and release prerequisites passed before confirmation.
