# grove-rhai-skill-delivery-k51

**Reviews:** grove-rhai-skill-delivery-k35

## Goal

Read Grove's repaired adoption contract adversarially before local planning
consumes it. Challenge the mapping from the reviewed Ravel contract to Grove,
and the release constraints of that mapping. Report findings, and change
nothing.

## Context

The producer is grove-rhai-skill-delivery-k35. Its commit holds the artefact.
Read its decision log first. The upstream contract is Ravel commit
`822c0ed2f67d9a446b74476fdad7d05922552b87` in the workspace
`Ravel.skill-directory-delivery`. Read it with `jj file show`, and change
nothing there. Its consumer handoff is in
`docs/skill-discovery-design-evidence.md`.

The Grove artefact is these documents:

- `docs/specs/codex-skill-provisioning.md`, especially Supported routes,
  Launch and compatibility, Packaging and rollout and Test seams.
- The Rhai contract mapping in `docs/grove-skill-delivery-design-evidence.md`.
- `docs/specs/harness-selection-and-execution.md`,
  `docs/adr/dispatch-supervises-the-harness.md` and the identity table of
  `docs/specs/coordinated-release.md`.
- The views `routes`, `skill-transition`, `packages` and `handoff` in
  `docs/design/harness-selection-and-execution/`. Serve them with
  `task design:harness-selection PORT=<free port>`.
- The stage briefs and planners under `.grove/03-k3/` and
  `.grove/06-k6/_discovery-rollout.md` that the producer changed.

A separate commit before the producer's commit converted eight of these
documents to controlled English. It changes wording only. Read the producer's
commit for the design change.

## Done when

- Each claim below is checked against the cited source, or reported as a
  finding with evidence.
- The mapping covers every fact of the Ravel consumer handoff. Check each row
  of the mapping table against the Ravel specifications, not only against the
  handoff summary.
- The findings are committed in this task's record, each classified by
  severity, with the artefact location and the source that contradicts it.

## Notes

Challenge these specific claims:

1. **Prompt fields.** The producer judged that the handoff's example, task data
   from `request.originalPrompt`, does not match Grove. Check in
   `assets/ravel-policy/` that no Grove asset reads task data from a prompt.
   Find any Grove reader of prompt text, including the glossary's dynamic
   dispatch and any deciding-agent example.
2. **Admission and routes.** Grove admits stable 2.x from 2.0.0 and selects no
   route. Test whether a 2.0.0 that a stale 1.1-only owner HOME meets gives a
   clear remedy through Ravel's refusal alone. Test whether admitting later
   2.x minors is safe while schema 4 is the only script schema.
3. **Version output.** A feature-line build reports `ravel 1.1.0`, so Grove
   admits it as 1.x. Check the manifest version. Then judge whether stubs
   before the cut and installed checks after it close the acceptance gap.
   Name any acceptance case that only a real `ravel 2.0.0` output can prove
   before Grove's release.
4. **Caller assets.** The design moves the whole of Grove issue 24 into the
   dual-Ravel stage. Check that Ravel's documented Rhai host functions can
   reproduce each refusal of `grove.ts` and `provenance.ts`. Check that they
   can reproduce the receipt bytes too. Judge whether one case table can drive
   both checks without a typechecker on the Rhai side.
5. **Settings and HOME.** Check the rule that a separate HOME is necessary only
   for a 2.0-only key. Check the consequence for Grove's standalone run logs.
   Check that the owner-delivery design and this contract do not disagree
   about which HOME the finish session uses.
6. **Release constraints.** Check the supported-route table, the prefix
   selection and the formula dependency against tap states A to D. The
   coordinated release and Ravel's release-channel specification define
   those states. A Grove
   release must not advertise directory delivery before its own acceptance.
   The human's release decision, the Ravel and tap handoffs and the original
   1.1.0 ending command must stay unchanged.
7. **Separate proofs.** Check that every acceptance row names its route. No
   row may credit the native route on 2.0 as directory evidence, or the
   reverse.
8. **Stage briefs.** Check each live common-loading brief and planner, and the
   rollout brief. Each must name the reviewed commit and identities. None may
   assign directory delivery to schema 3 or a TypeScript SDK.

Make no product code, Ravel, tap, owner configuration or release change. If
the findings warrant action, cut the integrate-review-design step after this
leaf, before ravel-local-delivery-k23.
