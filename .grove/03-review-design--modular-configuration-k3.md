# modular-configuration-k3

**Reviews:** modular-configuration-k2


## Goal

Adversarially review the modular configuration design against the approved root
brief. Find contracts that cannot satisfy those requirements, ambiguous grammar
or precedence, unnecessary public surface, and defects that would make separate
implementations disagree. Produce findings rather than fixes.



## Context

Read the producer's commit by its stable handle, the requirements decisions in
`plan-k1`, and the current durable artifacts:

- `docs/specs/modular-configuration.md` and the configuration parts of
  `docs/specs/module-decomposition.md`.
- `docs/adr/complete-session-configuration.md` and
  `docs/adr/untracked-configuration-delta.md`.
- The configuration glossary entries and `CONTEXT-MAP.md` ownership.
- `docs/examples/modular-configuration/` and the editable visual document at
  `docs/design/modular-configuration/`.

The user explicitly approved reusing `Workspace::is_tracked` for inspection:
configuration/working-tree bytes stay unchanged, but jj metadata may snapshot.
This choice is not an unresolved requirement.

## Done when

- Findings cite the producer's artifact and the requirement or concrete scenario
  it violates, or the review explicitly finds no actionable defects.
- The read covers repeated/diamond includes, partial profiles, parameter
  specificity versus layer order, removal when changing command schemas, legacy
  compatibility, and the distinction between eager structure and active semantics.
- Try to admit a local-only kind using an inactive personal profile, repair a
  missing personal target from local policy, hide an invalid active command with
  inspection filtering, and inject argument/slot structure through a parameter.
- Assess whether the Catalog/Templates split and the flat-only compatibility
  helper earn their surface, whether source attribution explains mixed-origin
  arguments, and whether examples express the promised scenarios.
- Any integration leaf is cut lazily under the review procedure, before the
  already queued planning leaf, carrying this review's handle rather than a
  finding list as its charter.

## Notes

The producer changed design/documentation only. The installed reader still uses
the flat grammar, and no new-form examples were installed into personal policy.
The reference and example readme mark that implementation boundary explicitly.
Browser discovery returned no available browser in the design session, so the
served document's visual layout could not be inspected there. Check the views
in a connected browser if available; the viewer README gives the serving command.
