# grove-rhai-skill-delivery-k35

## Goal

Reconcile the directory-adoption contract of Grove with the reviewed Rhai Ravel
feature contract before any dependent directory implementation or caller work.

## Context

Consume ravel-rhai-skill-delivery-k34 and its review and integration commits.
The existing `docs/specs/codex-skill-provisioning.md` and
`docs/grove-skill-delivery-design-evidence.md` of Grove assign directory
delivery to Ravel 2.0, schema 3 and TypeScript SDKs. Those assignments predate
Rhai main and are not settled feature identities. Native removal and the
route-independent mandate remain separate work.

## Consumer handoff

ravel-rhai-skill-delivery-k34 supplies the upstream facts. They are in the
"Consumer handoff" section of `docs/skill-discovery-design-evidence.md` at
Ravel commit `822c0ed2f67d`, on the bookmark `skill-directory-delivery`.
ravel-rhai-skill-delivery-k50 recorded that reviewed commit. It changed the
sandbox home and the frame-bound tests, not this handoff. In summary:

- Directory delivery needs Ravel 2.0.0, with `config.rhai` and
  `SCHEMA_VERSION` 4. The `check_version` of Grove admits only stable 1.x
  today.
- The native route stays Ravel 1.1.0, `policy.ts` with schema 2 and settings
  without 2.0 keys.
- Ravel 1.1.0 refuses settings that hold `skills`, `configEnv`, `harnesses` or
  `budgets`, at stage `cli` with exit 2. While a 1.1 reader remains, the
  directory route uses a separate `HOME`.
- Examples for the directory route put `request.prompt` into the harness
  arguments. They read task data, such as the handle and the provider receipt,
  from `request.originalPrompt`.
- The ending command stays in the caller block, byte for byte. Grove needs
  neither `RAVEL_SKILL_DIRECTORIES` nor `skills resolve`.
- Until promotion, the directory route selects the keg-only `ravel@2` by its
  prefix.

## Done when

- Repair the delivery specification, evidence, caller specification and any
  cited ADR or view for the reviewed upstream contract. Separate the proof of
  Rhai native compatibility from the proof of directory delivery and its
  actual minimum version. Both can ship together if the coordinated release
  design says so.
- Replace stale SDK and schema assumptions with explicit runtime, schema,
  asset and settings requirements for each supported route. Keep original
  launch endings, user-selected native delivery, open kinds and source
  ownership.
- Update all live common-loading stage briefs and planners with exact reviewed
  contract commits and feature identities. Reconcile summaries and consumer
  maps as well as the sections that name the obsolete assignments.
- Retain the release decision of the human and the named Ravel and tap
  handoffs. Feature publication and shared activation stay with rollout. Any
  earlier native compatibility release needs its own explicit owner and
  authorisation.
- Run applicable design and documentation checks, and cut a focused
  review-design leaf before dependent local planning. Its body names this
  producer and challenges the upstream-to-consumer mapping and release
  constraints.

## Notes

This leaf makes no product code, Ravel or tap edit, owner configuration or
release. The review of the producer is lazy. Cut it once the repaired artifact
exists.

## Decisions (running log)

**Inputs.** The upstream input is Ravel commit
`822c0ed2f67d9a446b74476fdad7d05922552b87` in the clean workspace
`Ravel.skill-directory-delivery`. This session read it with `jj file show` and
changed nothing there. The Grove base is `314d464e936`. Grove issue 24 is open.
It asks for a Rhai port of every Grove caller asset, because Ravel 2.0.0
removes `ravel/sdk`.

**Three routes and two proofs.** Grove supports three routes. The native route
on 1.1 uses `policy.ts` with schema 2. The native route on 2.0 uses
`config.rhai` with schema 4 and delivery disabled. The directory route on 2.0
adds the `skills` settings key.

The actual minimum of directory delivery is
2.0.0, because the feature first ships there. Rhai native compatibility and
directory delivery are two separate proofs over the same 2.0.0 executable.
The coordinated release ships both in one 2.0.0. An earlier native-only 2.0.0
release would need its own explicit owner and authorisation.

**Admission selects no route.** Grove admits stable 2.x from 2.0.0 beside
stable 1.x from 1.1.0, and refuses every other major. The predicate decides
admission only. The PATH, the configuration file and the settings select the
route, and Grove reads none of them. The remedy names both majors, their
configuration files and the `ravel@2` prefix until promotion.

**Caller assets.** The 2.0 route needs Rhai equivalents of every Grove caller
asset, with the same refusals and the same receipt format. That is the scope of
issue 24, and the dual-Ravel stage delivers it. The TypeScript assets keep
their installed paths for 1.x, because owner policies import them by path. One
case table drives the checks of both asset sets.

**Prompt fields.** The upstream handoff says that the examples read task data,
such as the handle and the receipt, from `request.originalPrompt`. That example
does not match Grove.

Grove's assets read the handle from `request.taskId`, the
Reviews and Creator lines from `request.taskFile` and caller facts from
`request.context`. Schema 4 leaves those fields unchanged. The receipt reaches
the harness through `GROVE_PROVIDER_EVIDENCE`. No Grove asset analyses prompt
text. An owner script that analyses the task reads `request.originalPrompt`,
which is the upstream rule.

**Version output.** The feature line's manifest still says 1.1.0, so a
temporary build reports `ravel 1.1.0`, and Grove admits it as 1.x. Acceptance
therefore identifies each Ravel build by commit and build identity. Stub
executables prove admission of `ravel 2.0.0` before the cut. Rollout repeats
the installed checks against the published archives.

**HOME.** Grove sets no HOME. A separate HOME for the 2.0 route therefore also
moves Grove's standalone run logs and the HOME-based configuration of the
harnesses and the VCS. The owner-delivery design owns what that HOME links.

**No scrub change.** `RAVEL_SKILL_DIRECTORIES` grants no authority, and Ravel
replaces or removes it in every environment that it builds. Grove's scrub list
and the cargo guard need no new entry.

**Records.** No new ADR earns its place. The dual-major trade-off already lives
in `dispatch-supervises-the-harness`. This leaf edits that record in place for
the Rhai assets and the settings HOME.

**Artefacts.** The delivery specification gains the section Supported routes.
It states the route table, the actual minimum and separate proofs, settings and
HOME, prefix selection and caller assets. Its compatibility, failure,
packaging and test-seam sections follow it. The evidence gains the Rhai
contract mapping. The caller specification, the supervision record, the
standalone specification and the identity table of the coordinated release
follow the same identities.

**Views.** The new view `routes` shows the three routes and the refusals of a
mismatched configuration. The views `skill-transition`, `packages` and
`handoff` changed, and the discussion panel names this topic. The old server
on port 52710 returned empty replies. This session served the views on port
52735 and left that old process alone.

**Conversion.** A separate commit first converted eight documents to
controlled English. Two subagents converted the delivery specification and
its evidence. Their comparisons kept every link target, heading and table row.
This session read each wording choice that they flagged and kept it.

**Checks.** The technical-communication checker reports no hard finding in any
changed prose document. The 95 local links of the changed documents resolve,
and so do 315 inbound links. A planted control failed on both of its defects.

The manifest has unique identifiers, and each source exists. `git diff --check`
passes. The sweep for stale assignments finds only history, refusals and
settled identities.

All six views of the discussion rendered in headless Chromium at 1,400 pixels,
in light and dark schemes. A fresh deep link reached the route view. Nobody
measured mobile appearance. No product test or `scripts/check.sh` ran, because
this leaf changes documents only.

**Review.** grove-rhai-skill-delivery-k51 reviews this producer before
ravel-local-delivery-k23. `leaf-insert` placed it there. Its body lists eight
specific doubts. This session had no provider evidence in its environment, so
the review carries no Creator line. No in-session reviewer ran.
