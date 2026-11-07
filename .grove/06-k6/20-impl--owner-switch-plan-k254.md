# owner-switch-plan-k254

## Goal

Complete the real switch plan from the staging evidence and the prepared
real HOME. Place it with the program in the owner's prepared folder.

## Context

- owner-switch-plan-draft-k247 drafted the plan with provisional
  `accepted` values. owner-staging-acceptance-k252 recorded the accepted
  values.
- owner-pack-checkouts-k93 installed the formulae and registered
  `guardrail`. owner-marketplace-freeze-k253 changed settings that the plan
  records as earlier values.
- The real plan names `.local/state/owner-switch` as its `state`. The
  owner keeps the plan and the program at `<state>/prepared/plan.json` and
  `<state>/prepared/owner_switch.py`. `mac-check` reads the switch state
  through them, and without them it reports native delivery.
- The parent brief's "Owner-delivery handoff" states the plan's required
  fields, the MCP launcher chains and the staging paths.
- `docs/owner-cutover/switch-plan-draft.json` is the draft.
  `task owner:switch-plan:draft` writes it again from the real HOME, and
  `task owner:switch-plan:check` admits it in a disposable HOME. The
  section "Owner switch plan draft" of
  `docs/owner-skill-delivery-evidence.md` states each choice and each
  provisional value. The draft's provisional trials fail on purpose.
- owner-switch-probes-k257 implements the probe program that `probes` and
  `native_probe` name. It can change that interface.

## Done when

- `accepted` holds the ten sources with their accepted digests, the formula
  versions, each source's trials and the absolute staging paths. They match
  owner-staging-acceptance-k252.
- Each part's earlier value matches the real HOME now. The exceptions are
  the cells that step 3's preludes write, which hold the values that the
  preludes leave. Each entry's change identity matches
  `owner_switch.py identity`.
- `session.mcp` matches the owner's current MCP process tree, launcher
  chains included.
- `<state>/prepared/` holds the plan and the program. It also holds
  `inventory.py` and `owner_switch_probes.py`, because `baseline`,
  `probes` and `native_probe` name them there. Their digests are in the
  evidence document.
- The cells of `technical-communication-hooks@writing-hooks` and
  `guardrail@personal` hold `false` as their earlier values. `guardrail`
  holds that value after owner-pack-checkouts-k93, and the hooks plugin
  after step 3's prelude.
- `run --dry-run` against the real HOME lists each part, each cell's current
  value and the inverse, and changes nothing.
- `mac-check` reports the native state with the prepared plan.

## Notes

A change to the plan or the program after this leaf needs a new rehearsal.
Keep the plan unchanged from here to the switch unless a rehearsal follows.
