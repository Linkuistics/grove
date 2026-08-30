# treatment-verification-k83

## Goal

Verify and pin the final deployable treatment bytes and the exact control versus
enabled environment delta before the acceptance manifest freezes.

## Context

- Treatment: `plugins/linkuistics/skills/writing-code-walkthroughs`.
- Apparatus contract: `campaign-apparatus-k82`.
- Parent deterministic requirements: `walkthrough-skill-delivery-k19`.

## Done when

- The target skill receives a recursive path-and-digest manifest and aggregate
  digest. The verified bytes are the only treatment bytes the campaign may
  preload.
- Authoring-convention, progressive-disclosure, citation, harness declaration,
  plugin layout, installation/structure, skill-specific deterministic, and
  applicable repository checks run against that exact digest and preserve
  their commands and outputs.
- Sealed control and enabled templates are manifest-verified. Removing only the
  target skill subtree from the enabled template makes their declared content
  identical; the control has no alternate copy, plugin, instruction, or
  connector that delivers the treatment.
- The execution identity and reachable model-interface surfaces are enumerated
  for both templates. Any undeclared treatment channel or inability to verify
  the intended preload yields a terminal failed or unavailable record.
- The result is exactly pass, failed, or unavailable. Only pass may feed
  `campaign-freeze-k84`; anything else leaves the deterministic parent conjunct
  open and launches no evaluated context.
- No skill byte is edited and no evaluated treatment or control context runs in
  this leaf. A required skill revision becomes a new pre-freeze tree item and a
  new treatment digest, never an in-campaign amendment.

## Notes

Historical evaluation bytes and records are inputs for reporting only and are
not modified or rescored by this verification.
