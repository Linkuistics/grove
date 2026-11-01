# common-skill-loading-k3 — brief

## Goal

Deliver common Ravel skill discovery for Claude Code and Codex while the skill
sources still live in this checkout. Leave a working launch path on which the
external research-pack proof can build.

## Done when

- Ravel resolves configured local sources and exact pinned Git revisions,
  produces the discovery document, and automatically prepares the prompt before
  owner policy constructs the command. The original task and ending commands
  remain intact in their separate block.
- Exact cached revisions work offline; missing required revisions fail clearly
  without substitution. Catalogs and resources remain readable for sessions and
  delegates, including confined launches.
- Grove uses this common delivery path with its obsolete automatic native
  provisioning reconciled. Any core fallback uses the same path. No source
  pack has moved yet and no kind registry or new workflow protocol exists.
- Public fixture tests cover launch/resolution, failures, offline reuse,
  concurrency/lifetime and confinement. Existing Grove tree/launch behavior
  remains green.
- Real Claude Code and Codex trials activate a relevant craft skill without the
  task naming it, record the read before the action, and use isolated homes or
  equivalent isolation with native copies absent. Preserve the owner's installs.
- The owning specifications, affected architecture/glossary/walkthrough and
  installation documentation agree with delivered behavior. Applicable Ravel
  checks and Grove's full check pass using an explicit compatible Ravel build.

## Decomposition

First settle Ravel's generic contract, then Grove's adoption and compatibility
contract. The next planner consumes both and grows the smallest working
implementation increments, including actual harness acceptance. Do not split a
resolver from its only usable launch reader into separately dead halves.
Local-only and pinned-source delivery may be separate working increments if
the settled design supports each as useful behavior; make that judgment then.

This subtree must remain live until implementation and acceptance satisfy its
Done when. Retiring its design and planning children alone does not close it.
Design producers add any earned review ahead of dependent work; integrations
follow their reviews before later consumers edit the same artifacts.

## Pointers

- Ravel: `docs/specs/session-execution.md`, request preparation and confinement.
- Grove: `docs/specs/codex-skill-provisioning.md`,
  `docs/specs/harness-selection-and-execution.md`.
- Existing seams verified in skill-discovery-k2: Ravel request preparation before
  selection, and Grove's CLI provisioning and public session command.

## Notes

Keep one coordinated tree here. Work in other owning repositories must follow
their instructions, preserve unrelated changes, and leave discoverable commits.
Use independent temporary installed prefixes for acceptance if releases are not
yet available. Final publication belongs to discovery-rollout-k6; establish any
required earlier dependency release explicitly rather than silently assuming it.
Every session retains the ending command from its own launch prompt.
