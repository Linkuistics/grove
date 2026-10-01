# dispatch-documentation-k41 — brief

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

## Decomposition

This leaf proved bigger than one session. Its reading alone is the spec, the
dispatch README, the runtime evidence, three ADRs, the Grove usage and
configuration references, configure-grove and the test suites of two crates.
The children split it by what each one demonstrates, and where the conditions
above say "this leaf", read the child named here.

1. `acceptance-walk-k67` walks the acceptance cases and the seam rows, and
   runs `task check` and the installed smoke. It runs first because it is the
   evidence the documents rest on: the spec may drop its "not implemented"
   notice only for behavior the walk found an instrument for. Its running log
   holds the walk. It found an instrument for every case, so nothing was cut
   ahead of the two below. Tests that hold a clause more narrowly than their
   names say went to `seam-controls-k70` at the grove root, which the
   documents do not wait on.
2. `current-state-documents-k68` rewrote the design-tense artifacts as
   current state: the spec, the visual design README and viewer manifest, the
   runtime evidence and the three ADRs. The glossary needed nothing. Its
   running log has each judgement, and what moved out of the spec's notice
   into the body. `usage-agreement-k69` builds on these facts. Three sentences
   are now claims about k69's documents, so k69 keeps the documents true to
   them or changes the sentence: the last paragraph of the spec's
   `#grove-integration` says what the Grove-side documents explain and that
   the command they quote is the one `--help` carries; `#delivery` says the
   dispatch README and the release procedure state the kernel range; and the
   policy-ownership ADR names the documents that state the launch-time
   boundary. *Policy and joint choice* names every shipped example by
   specifier, which the README's example table must match. The seam table's
   Grove rows are labelled "Grove launch boundary", without "Existing". The
   research survey's *Adopted boundary* section is current too. k68 changed
   two module comments in the package source, and swept no other source
   comment and no help text for design tense. The review k69 cuts is pointed
   at k68's running log as well as k67's.
3. `usage-agreement-k69` placed the package in `docs/ARCHITECTURE.md`, brought
   the usage documents into agreement with one another and with `--help`, and
   rewrote the CHANGELOG's Unreleased section by subject. Its running log
   lists each disagreement it closed and the one judgement it made without the
   human, on `CONTEXT-MAP.md`. `task check` passed on its tree. Its retirement
   closed this node, so it first cut the documentation-acceptance review,
   `dispatch-documentation-k71`, at the grove root. The root brief's
   *Documentation closed* paragraph has what later leaves build on.
