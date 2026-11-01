# common-skill-loading-k3 — brief

## Goal

Deliver common Ravel skill discovery for Claude Code and Codex while the skill
sources still live in this checkout. Leave a working launch path on which the
external research-pack proof can build.

## Done when

- Ravel acquires configured local sources and exact pinned Git revisions,
  injects their skill-directory paths, and automatically prepares the prompt before
  owner policy constructs the command. The original task and ending commands
  remain intact in their separate block.
- Exact cached revisions work offline; missing required revisions fail clearly
  without substitution. Directories and resources remain readable for sessions and
  delegates, including confined launches.
- Grove removes obsolete automatic native provisioning. Users may supply skills
  natively in each harness or through Ravel directory injection; Grove only names
  the skill and imposes no delivery route. Any core fallback uses the common path. No source
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

The human corrected the boundary during grove-skill-delivery-k8: Ravel injects
directory locations and does not resolve a requested skill. The earlier
ravel-skill-discovery-k7/k14 catalog design must be revised before planning.
Retain local/pinned/offline/lifetime requirements; do not reintroduce a catalog
dependency into Grove. Test native-only setup as well as isolated directory
delivery. The adoption design's new review must consume that upstream correction.

The reviewed Ravel directory contract is committed as
`b93bb927095ff09b91e9b78dc1ad05bdacfec649` by ravel-skill-directories-k18,
integrating ravel-skill-directories-k17 after producer ravel-skill-directories-k15.
It injects search directories and direct mandatory-file references, retaining a
directory-only descriptor for recovery/delegation. The integration defines
name versus explicit-loader outcomes and mandatory-before-caller read order.
Grove's mandate must qualify "first" with "after any mandatory guidance";
its ordinary and plugin-qualified name requests must work with native copies
absent. Shared settings remain a recorded trade-off: mixed-major enablement
needs separate HOME settings, not just separate executable prefixes.
Whole-preamble loss needs an unprompted recovery trial; a retained path alone
does not satisfy it. Grove's adoption review consumes this corrected handoff.
Ravel 2/schema 3 remains the feature target; Grove's
native-only Ravel 1.1 compatibility and explicit dual-major caller acceptance
remain independent. Consume the reviewed correction before planning implementation.

Keep one coordinated tree here. Work in other owning repositories must follow
their instructions, preserve unrelated changes, and leave discoverable commits.
Use independent temporary installed prefixes for acceptance if releases are not
yet available. Final publication belongs to discovery-rollout-k6; establish any
required earlier dependency release explicitly rather than silently assuming it.
Every session retains the ending command from its own launch prompt.
