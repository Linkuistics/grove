# dispatch-documentation-k41

## Goal

Bring the durable documentation to the current state of the delivered first
release, and demonstrate that every acceptance case has an owner and a
passing instrument. Then cut the documentation-acceptance review the
requirements make mandatory.

## Context

Each increment documented its own surface as it landed. This leaf consolidates,
and replaces design-tense wording with current state. It is not where missing
behavior gets built. If the walk below finds an acceptance case with no passing
instrument, cut or insert the missing work rather than absorbing it.

## Done when

- The spec no longer carries its "not implemented" notice. It describes the
  delivered command as current state, with no design-era hedges left. The
  visual design README and the runtime evidence are current. So are the ADRs
  and the glossary entries written in design terms.
- `docs/ARCHITECTURE.md` places the new package and its boundary. The dispatch
  usage documentation, the Grove usage and configuration references, and
  configure-grove agree with one another and with `--help`.
- A walk of every acceptance case in the root brief and every row of the spec's
  `#test-seams` table names its test or task and shows it passing. Hostile and
  limit classes show their firing controls. Record the walk in this leaf's
  running log. It is process evidence and does not need a durable document.
- `task check` passes on the integrated tree, and the per-target installed
  smoke passes on all three targets.
- The CHANGELOG's Unreleased section describes the first release of
  `harness-dispatch`, the Grove slots and the methodology amendment.
- As this leaf's last act, cut the documentation-acceptance review:
  `grove-llm leaf-add . dispatch-documentation --kind review-impl`, with
  `**Reviews:** dispatch-documentation-k41`. Its body asks for a review of the
  documentation against the requirements' acceptance cases. The review covers
  activation, both inspection surfaces, the `**Creator:**` conventions and
  their remedies, later outcome entry, launch-time validation guidance and
  the stated floors.
