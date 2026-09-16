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

## Decisions (running log)

- The real-driver selected-policy test now exercises a local-only requirements
  route with no tree; it fails because `.grove/` remains after refusal. Keep the
  existing locked vacancy observation and admit its initial kind before creation.
- The public driver transition will take the already resolved SessionConfig;
  the lifecycle helper receives a fallible kind-admission callback, invoked only
  in its vacancy branch. This keeps configuration out of the generic store and
  avoids an unguarded existence probe or a reload inside the critical section.
- Driver acceptance now proves inactive-profile and local-only bootstrap refusal,
  active selected-profile initialization and launch, and existing-tree launch
  without requirements authority. The original failing case passes after the
  repair. The public two-scaffolding-doors test now supplies resolved policy.
- The production edits belong only to the grove-loop book's recursive corpus;
  all six manifests were inspected. Its literals, ranges, root lengths, indices
  and owned-source totals are synchronized, with final validation passing.
- The leaf's one fresh-context review examined admission, locking, error paths,
  acceptance tests and updated contract prose; it reported no violations.
- `bash scripts/check.sh` passed all eight principal checks, including the full
  workspace suite and final validation of all six books. SHA-256 comparison of
  all 1,809 tracked source, manifest, documentation, script and fixture inputs
  before and after the run found no changes. The existing configuration ADRs
  already state the repaired just-in-time authority rule and need no revision.
- `workspace-reload-k30` remains live, so this retirement does not close k10.
