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
- owner-switch-probes-k257 implements the probe program. It keeps the plan
  draft's probe commands unchanged. The probe program imports
  `owner_switch.py` and `harness_trial.py` from its own folder, so
  `<state>/prepared/` holds both beside it. The section "Session probes" of
  `docs/owner-skill-delivery-evidence.md` lists each file that a probe
  session writes, for the expected inventories.
- "The native settings inside finish", "The switch inside finish" and
  "Resuming the switch" of `docs/specs/owner-skill-delivery.md` state what
  finish and the owner need before mutation.
- The owner's installed `~/.config/ravel/config.rhai` has the SHA-256
  `0ccd5cdaa954…`, not that of the committed `owner-config.rhai`. The owner
  changed `lead` and `review` on 2026-11-08. The staging HOME copies the
  installed file. The section "Staging setup and isolation trial" of
  `docs/owner-skill-delivery-evidence.md` names the paths for
  `accepted.staging`.
- "Trials for gate 4" of "Staging acceptance" in
  `docs/owner-skill-delivery-evidence.md` names each Grove source's trials
  for `accepted.trials`. Each trial is a `staging_trial.py rerun` command,
  which replaces the provisional trials of the draft. Choose a runner path
  whose content stays the same through the switch. The main checkout gains
  `rerun` only when finish moves `main`.

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
- `<state>/prepared/` holds the plan, the program, `inventory.py`,
  `owner_switch_probes.py` and `harness_trial.py`. Their digests are in the
  evidence document.
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

owner-switch-status-k269 found two needs of the real plan:

- Step 8 requires exit 0 from the plan's `mac_check` command. The full
  `mac-check` exits 10 for CCC's manual review alone. Name
  `mac-check --only "Owner skill switch"`, and record the other areas
  under the machine contract, as step 8 states.
- `status` evaluates native health with `switch.native_assertions`.
  Before promotion, the real Ravel settings may lack `skills`. Give that
  assertion `"values"` with the off value and `{"$absent": true}`, so
  that `mac-check` reports native delivery before finish.

owner-staging-hooks-k251 found that the switch refuses a plan whose
`.config/ravel/settings.json` original is absent, as the stored draft's is.
Finish runs `native-off` before the switch, and the switch compares each
planned original. The real plan therefore needs the identity of the file
that `native-off` writes. `native-off` is deterministic, so a disposable
HOME can compute it. The section "Quiet interval in staging" of
`docs/owner-skill-delivery-evidence.md` records the refusal.
