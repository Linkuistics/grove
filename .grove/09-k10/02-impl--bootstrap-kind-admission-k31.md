# bootstrap-kind-admission-k31

## Goal

Refuse an unconfigured initial requirements kind before the driver creates a
fresh grove, preserving the just-in-time personal-authority contract.

## Context

workspace-selection-k29's expanded acceptance exposed a preexisting gap:
loop_driver::drive loads and validates the configuration before
transition_to_current, but that transition creates the requirements leaf without
requiring its kind. A valid configuration with no active personal requirements
route therefore creates .grove/ and only fails at the subsequent expansion.
A local-only requirements route or an inactive personal route must not authorize
this mutation. This is a repair to the existing driver boundary, not a resolver
change or repetition of the selection implementation.

Reproduction: lifecycle_cutover's selected_policy test with no existing tree,
primary containing command/profile declarations but no active routes, and a
local-only route. The process fails and launches nothing but .grove/ exists.
The test currently keeps local-only refusal on an existing tree; extend it to
this absent-tree case when repairing the production boundary.

## Done when

- A fresh root is not created when requirements lacks active personal authority,
  including local-only and inactive-profile cases. An active selected personal
  requirements route still permits initialization and launch.
- Existing trees do not require requirements unnecessarily. Preserve transition
  locking/admission and both configuration load points; avoid a new race from a
  separate unguarded exists probe or teaching the generic tree store about kinds.
- Active configuration errors still precede mutation and launch globally.
- Focused real-driver acceptance and bash scripts/check.sh pass. Update all
  affected source-exact books and contract prose in this same leaf.
