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
