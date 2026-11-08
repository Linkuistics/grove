# owner-switch-plan-k254

## Goal

Complete the simplified real switch plan from the staging evidence and the
prepared real HOME. Place it with the program and its prepared restore entry
in the owner's prepared folder.

## Context

- owner-switch-contraction-k270 defines the smaller plan format and the
  draft generator. `task owner:switch-plan:draft` writes the draft again
  from the real HOME, reading only. `task owner:switch-plan:check` checks it
  in a disposable HOME.
- owner-staging-acceptance-k252 recorded the accepted values.
  owner-pack-checkouts-k93 installed the formulae and registered
  `guardrail`. owner-marketplace-freeze-k253 changed settings that the plan
  backs up as native state.
- The real plan names `.local/state/owner-switch` as its `state`. The owner
  keeps the plan and the program at `<state>/prepared/plan.json` and
  `<state>/prepared/owner_switch.py`. Without them, `mac-check` reports that
  the switch is not prepared.
- owner-switch-probes-k257 implements the probe program. It can change that
  interface.
- "The native settings inside finish", "The switch inside finish" and
  "Resuming the switch" of `docs/specs/owner-skill-delivery.md` state what
  finish and the owner need before mutation.

## Done when

- `accepted` holds the ten sources with their accepted digests, the formula
  versions, each source's trials and the absolute staging paths. They match
  owner-staging-acceptance-k252.
- The plan names the full write set of each command and its original
  content, metadata or absence in the real HOME now. It names the expected
  native and switched inventories.
- The plan identifies every file that a command runs by content, including
  scripts, local helpers and inputs. It records the logical prefix mappings.
- The plan keeps delivery assertions apart from whole-file restore
  identities. It records the off-value restoration exception.
- The plan holds no `session.mcp`, process pattern, cell field or part
  recovery field.
- `<state>/prepared/` holds the plan, the program, `inventory.py` and
  `owner_switch_probes.py`. Their digests are in the evidence document.
- Prepared instructions, readable after `.grove/` teardown, name the exact
  `native-off` and `restore --backup` commands. They also name the
  attempt-note format and the comparison command.
- The plan report against the real HOME lists the write set, the current
  values and the commands, and changes nothing.
- `mac-check` reports native delivery with the prepared plan.

## Notes

A change to the plan, the program, a helper, a command input or a tool
version after this leaf needs another dry run. Keep them unchanged from here
to the switch unless owner-switch-rehearsal-k255 runs again.
