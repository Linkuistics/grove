# ravel-rhai-skill-delivery-k34

## Goal

Repair Ravel's directory-delivery contract against its current Rhai main so
the local and pinned planners have one implementable public contract.

## Context

The planning review common-skill-loading-k31, finding 1, compares historical
directory design `b93bb927` with Ravel main `24a54523`. Read those committed
artifacts, then current main and its owning instructions. The default Ravel
workspace still holds the old design line. Do not mistake it for main or edit
the `Ravel.gh-issue-2` tree. Work belongs in a separate Ravel jj workspace.
Consume coordinated-ravel-release-k36's reviewed publication constraints and
owner-ravel-skill-delivery-k37's complete source/resource requirements first.

owner-ravel-skill-delivery-k48 integrated that design's review. Its item 8 now
requires a sandbox home that links only each sign-in store. The existing
smoke recipe links the whole `.codex` tree and does not satisfy the item.

k36's route is `docs/specs/coordinated-release.md`: combined Ravel
2.0.0 and script/request schema 4, first through keg-only `ravel@2`. The human
confirmed sole execution here until completion. Use the producer and its
review/integration result for final identities. Ravel's release-channel contract
is in `Ravel.coordinated-release-design`, separately based on main `24a54523`.
Reconcile that contract into the owning feature line without editing the active
Rhai task tree or assuming its unfinished book work is complete.

## Done when

- Record an exact current-main base and any intervening upstream changes.
  Reconcile `docs/specs/skill-discovery.md` and its evidence, session and
  configuration specifications, ADRs and design views in Ravel. Preserve the
  root's local/pinned/offline/lifetime, automatic preparation and confinement
  requirements without restoring the retired TypeScript runtime.
- Settle the feature release identity under the coordinated publication plan.
  Current Rhai main has no directory feature. The release design decides
  whether both changes ship together or in distinct releases. Settle request
  fields and schema, Rhai host descriptor
  reads, `load_context` ordering, settings beside `configEnv`, and complete
  encoded Rhai worker frame bounds. Do not reuse schema 3's existing meaning
  silently or require a TypeScript SDK that main removed.
- State compatibility of disabled delivery, old/new configuration readers,
  front/worker identities and isolated settings. Keep canonical session ending
  commands and exact original caller bytes. Every public decision has one owner.
<!-- tc: depart synonym-rotation because "checks" names the repository check suites, not a second verb for confirm -->
- Pin the named Ravel bookmark `skill-directory-delivery` and exact base for
  implementation. Local and pinned builds remain off main and unpublished
  through acceptance. Each stage owns its entry rebase and checks. Rollout
  owns final rebase, integration, checks and publication of the complete feature.
  Record the owner's cross-repository handoff before implementation starts.
- Commit durable artifacts in Ravel and record their exact revisions here.
  Repair the local/pinned stage inputs with the settled version/schema and
  actual task names. Give grove-rhai-skill-delivery-k35 the consumer handoff.
- Cut a focused review-design leaf for this public contract before dependent
  Grove design or local planning. Name this producer in its Reviews line and
  name the finishing provider evidence under the retirement procedure.

## Notes

This is substantial design repair, deliberately externalised from integration.
No implementation, release, tap change, shared activation or other-grove signal.
The human's release decision in common-skill-loading-k32 governs coordination.

## Decisions (running log)

**Base.** Ravel `main` and `main@origin` are both
`24a545236185b0910ba6117fe8f9e8b2e2c8a748` after a fetch on 2026-11-02. No
upstream change arrived after k36 observed that commit. The feature line starts
on the release-channel contract `2a77ffbda3b2510f0997c740c0642b18afaeece7`.
Its two ancestors above main are `80e38222` and `c9474196`. The line therefore
carries the channel contract without a rewrite of those commits.

**Identity.** The feature ships inside the combined Ravel 2.0.0 that k36
selected. The script and request schema is 4. A script that exports 3 refuses
as `unsupported_version`, also when delivery is disabled. Schema 3 was never
published, and a settings edit must not change what an unchanged script
receives.

Worker protocol 1, selection-context schema 1, routing-notice schema
2 and failure-document schema 2 keep their numbers. The new notice member and
stage are part of the first published 2.0.0 shape of those documents.

**Request.** Schema 4 adds `originalPrompt` and `skills`, and `prompt` becomes
the prepared prompt. With delivery disabled, `prompt` equals `originalPrompt`
byte for byte and `skills` is `()`. `limits` is unchanged.

**Order.** The Ravel CLI resolves sources after settings, caller inputs and
entry discovery. Resolution ends before confinement preparation and before the
worker starts, so the whole evaluation deadline excludes it. `load_context`
and `select` each receive a copy of the same request value. Top-level module
code sees no request.

**Host reads.** No new host function exists. A script that reads the descriptor
or a snapshot file makes an ordinary measured read. `RAVEL_SKILL_DIRECTORIES`
is in the reserved `RAVEL_*` class, so no grant exposes it to `ravel::env`.

**Settings.** `skills` is a settings key beside `configEnv`, `harnesses` and
`budgets`. Ravel 2.0.0 refuses a duplicate member name anywhere in the file.
Ravel 1.1.0 refuses every key other than `timeoutMs`, `contextBytes` and
`policyEnv` as `settings_invalid`, stage `cli`, exit 2, before evaluation.
Ravel 2.0.0 refuses `policyEnv` the same way. `end-session` reads no settings
in either release.

**Frames.** The evaluate-frame bound is derived from the public bounds at the
worst JSON escape. It is 38,541,312 bytes. The CLI checks the encoded evaluate
frame before it sends it. Only an exceptional kind, working directory or task
file path can pass the bound, and that case refuses as `malformed_input`, exit
2. The script-result message bound stays 1 MiB. The prepared-prompt bound is
1 MiB and refuses as `skill_limit_exceeded`.

**Environment.** `RAVEL_SKILL_DIRECTORIES` is set in every managed-process
environment that Ravel builds, which includes each harness run. With
`--confine`, harness runs receive the same read grants as the managed process.

**Trials.** The configuration specification owns one sandbox-home technique.
It links only the sign-in store of each harness, and Codex receives its own
`CODEX_HOME` with a link to `auth.json` only. The smoke tasks adopt it, and the
local stage implements that change.

**Loading rule.** The preamble tells the agent to load a skill whose metadata
sets `disable-model-invocation: true` only when the caller names it.

**Cancellation.** A signal during resolution refuses as
`skill_resolution_cancelled`, stage `skills`, exit 128 plus the signal number.

**Artifacts.** Ravel commit `fc87cb41a422bf2623ce362ef689c38ddf98a506` holds
the repaired contract, on the local bookmark `skill-directory-delivery` in the
jj workspace `Ravel.skill-directory-delivery`. Its parent is `2a77ffbd`.
Nobody pushed it. It adds `docs/specs/skill-discovery.md`, its evidence, the
decision records `skill-preparation-precedes-evaluation` and
`skill-resources-outlive-sessions`, and the views in
`docs/design/skill-discovery/`. It changes the configuration and session
specifications, `CONTEXT.md`, `Taskfile.yml` and
`ravel-supervises-the-session`.

**Handoffs.** The Ravel evidence document holds the stage handoff for the Ravel
owner and the consumer handoff. The subtree brief, the local and pinned stage
briefs and planners, and k35 now name the commit, the identity and the Ravel
task names. A separate Grove commit first converted those five notes to
controlled English.

**Review.** `ravel-rhai-skill-delivery-k49` reviews this producer before k35
and before local planning. This session had no provider evidence in its
environment, so the review carries no Creator line. No in-session reviewer ran.

**Checks.** The technical-communication checker reports no hard finding in the
changed documents. The strict STE check of Ravel passes `CONTEXT.md` and fails
a planted control. All 95 local links resolve, and a planted control fails.
The view manifest parses.

All four views rendered in a headless Playwright
browser at desktop width in the light scheme. Nobody measured dark or mobile
appearance. No product test, check suite or harness trial ran, because this
leaf changed documents only.
