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
  inject.
- Authoring-convention, progressive-disclosure, citation, harness declaration,
  plugin layout, installation/structure, skill-specific deterministic, and
  applicable repository checks run against that exact digest and preserve
  their commands and outputs.
- The target directory is exhaustively enumerated under the reviewed
  `delivery-channel-authority-k94` transport contract. Path ordering and
  boundaries, admissible file types and encoding, symlink/non-regular handling,
  payload-size limits, framing bytes, and aggregate digest all match that
  decision. The enabled effective request contains exactly that verified
  treatment; the control request omits it and has no alternate copy, plugin,
  instruction, or connector. Every other declared request and workspace byte is
  identical.
- The execution identity and reachable model-interface surfaces are enumerated
  for both arms. The authority's three surface tool declarations are verified
  separately from delivery. Any undeclared treatment channel or inability to
  verify its authoritative receipt yields a terminal failed or unavailable
  record.
- The result is exactly pass, failed, or unavailable. Only pass may feed
  `campaign-freeze-k84`; anything else leaves the deterministic parent conjunct
  open and launches no evaluated context.
- No skill byte is edited and no evaluated treatment or control context runs in
  this leaf. A required skill revision becomes a new pre-freeze tree item and a
  new treatment digest, never an in-campaign amendment.

## Notes

Historical evaluation bytes and records are inputs for reporting only and are
not modified or rescored by this verification.
