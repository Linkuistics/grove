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

## Decisions (running log)

**Inputs and scope.** This review inspects Grove producer commit
`50ce143f4316d180deaf3f7f0b14506664ace0ac`. It first read the producer's
decision log. The preceding wording conversion is commit `9ff416f2`.
The upstream contract is Ravel commit
`822c0ed2f67d9a446b74476fdad7d05922552b87`, read through `jj file show` in
`Ravel.skill-directory-delivery`. The historical SDK read below uses Ravel
1.1.0 release commit `2095cbd8`. This session changed no upstream file.

**Review method.** This is an inspection-only review. No test, build, lint or
format command ran. The interface review used `codebase-design`. The review
checked the eight doubts below and each row of the upstream mapping.

**Caller behaviour.** The structured-field mapping is supported. The inspected
assets forward the prompt and read task facts separately. The complete asset
inventory contains no deciding-agent implementation. The glossary describes
an owner-supplied deciding agent. Such an owner's task analysis must use
`originalPrompt` on schema 4.

**Runtime differences.** The promise of identical refusals needs correction.
The reviewed upstream contract retains explicit differences between runtimes.
The Grove sample consumes the affected choice-file helper. Finding F1 records
the mismatch. Receipt encoding remains a separate requirement.

## Findings

### F1 — P2 / medium: reconcile refusal parity with the upstream runtime differences

**Artefact.** `docs/specs/codex-skill-provisioning.md:163` promises that each
Rhai asset keeps its TypeScript counterpart's refusals. Lines 181–185 prescribe
one case table for both sets. The native-2.0 acceptance row at line 397 repeats
that equivalence. The dual-Ravel brief repeats it at
`.grove/03-k3/27-k27/_grove-dual-ravel.md:21`.
These coordinates name producer commit `50ce143f`, before this review inserted
its integration. The brief's stable handle is grove-dual-ravel-k27.

**Contradicting source.** At Ravel `822c0ed2`,
`docs/specs/configuration-evaluation.md:1209`, *Frozen comparison rules*,
explicitly retains three runtime differences. An oversized choice file gives
`source_too_large` in Rhai instead of `choice_unreadable`. JavaScript and Rhai
classify U+FEFF and U+0085 differently when splitting or trimming text.
An invalid UTF-8 jj store path refuses in Rhai, while TypeScript decodes
replacement characters.

These inputs reach Grove's sample. At
`assets/ravel-policy/sample/policy.ts:266`, it calls the SDK's `readChoice`.
At Ravel 1.1.0 commit `2095cbd8`,
`crates/ravel/worker/sdk/index.ts:292` defines that helper. It checks the file's
size before reading and returns `choice_unreadable` above 4,096 bytes.
Grove's sample also reads the jj store path at
`assets/ravel-policy/sample/policy.ts:232` through Node's UTF-8 decoder.

The Rhai host contract makes a measured-read bound fatal, even inside a catch.
That rule appears at Ravel `822c0ed2`,
`docs/specs/configuration-evaluation.md:507` and `:912`.
Its maximum read bound is the context budget, at most 8,388,608 bytes.
Thus a choice file beyond that maximum cannot become the old helper's
`choice_unreadable` through the documented measured-read path.

**Consequence.** The next planner receives an unqualified equivalence promise
that the reviewed runtime contract does not support. It could require extra
capabilities or silently omit cases from the shared table. Either response
would conceal a behaviour change from the adoption contract.

**Smallest useful correction.** Reconcile the parity requirement with the
upstream differences before implementation. State which differences Grove
accepts and which its helpers must preserve deliberately. Give the shared
case table explicit expected outcomes for each runtime. Keep byte-exact
canonical receipts and provider-safety refusals as distinct obligations.
Carry the settled limits into the evidence, acceptance row and dual-Ravel brief.

## Checks of the requested claims

These are source conclusions. They are not runtime acceptance results.

| Claim | Evidence and conclusion |
| --- | --- |
| 1. Prompt fields | The eight TypeScript files under `assets/ravel-policy/` were read directly. Their prompt uses forward the launch text. `grove.ts:20` reads `taskFile` and `context`. `provenance.ts:47` reads `kind` and `taskId`. `CONTEXT.md:994` describes owner-supplied dynamic dispatch. Ravel's request table at `configuration-evaluation.md:188` preserves the structured fields. The mapping is supported. |
| 2. Admission and remedies | Grove's current `dispatch/ravel.rs:34` admits stable 1.x from 1.1.0. The proposed admission selects no delivery route. Ravel's entry table at `configuration-evaluation.md:109` gives a lone `policy.ts` the `config_format_retired` remedy. Its settings section at `:879` refuses `policyEnv`. These refusals reach the user without a Grove parser. Later 2.x admission relies on the stable public CLI contract. Schema admission remains Ravel's responsibility. No schema negotiation is promised. |
| 3. Version output | Ravel's pinned `Cargo.toml` still sets 1.1.0. Grove therefore admits that build through its existing predicate. Stubs establish the proposed 2.0.0 predicate only. Real 2.0.0 output must still prove canonical prefix selection, installed package identities and end-to-end admission. The delivery specification at `:376`, rollout brief and coordinated-release step 5 require those checks against published archives before Grove's release. The design does not credit stubs for those results. |
| 4. Caller assets | Ravel's host table at `configuration-evaluation.md:482` supplies measured reads, strict JSON helpers, UUIDs and path helpers. Its module rules at `:124` permit ordinary imports and selection wrappers. Grove's marker and receipt validation can use those capabilities. Its JSON rules at `:534` support fixed receipt ordering by joining encoded members. One public-launch case table is a useful seam without a Rhai typechecker. F1 qualifies the promise of identical refusal behaviour across all assets. |
| 5. Settings and HOME | Ravel's `skill-discovery.md`, *Configuration readers*, permits shared settings containing only `timeoutMs` and `contextBytes`. It rejects the other major's keys. Separate prefixes and `--config` do not isolate settings. Grove's `standalone.rs:52` places logs below HOME. The owner design at `owner-skill-delivery.md:253` uses staging homes for trials. Its final-session section at `:304` keeps finish in the real HOME on 1.1. The switch changes settings after that driver's last settings read. No conflict is supported. |
| 6. Release constraints | Grove's coordinated-release states A–D agree with Ravel's pinned `release-channels.md`. Unversioned Ravel stays 1.1.0 through compatible Grove publication. The directory route selects keg-only `ravel@2` explicitly. Promotion reuses the same archives after Grove's installed checks. Directory advertising waits for its own acceptance. The human's release ownership, tap handoffs and original 1.1.0 ending remain intact. |
| 7. Separate proofs | The delivery specification at `:108` requires separate native-2.0 and directory-2.0 proofs. Its acceptance table separates native launch, directory bootstrap and confined reads. Shared rows describe cross-cutting obligations and require each case to name its route. The common-acceptance brief repeats that requirement. Native acceptance supplies no directory evidence. F1 concerns refusal equivalence within the native-2.0 row. |
| 8. Stage briefs | The live local, pinned, dual-Ravel and common-acceptance briefs and planners were read, with the rollout brief. Their contract references lead to Ravel `822c0ed2` and the Grove producer and review handles. Their feature identity is combined 2.0.0, schema 4 and worker protocol 1. References to schema 3 and the TypeScript SDK describe history or explicit refusals. The local subset stays unpublished. F1 must also reach the dual-Ravel handoff. |

## Upstream mapping audit

Every row in the Rhai mapping was checked against the owning Ravel specification
at `822c0ed2`, beyond the handoff summary.

| Mapping row | Owning source and result |
| --- | --- |
| Release, script and schema | `skill-discovery.md`, *Schemas and identities*, requires combined 2.0.0 and schema 4. Schema 3 refuses even with delivery disabled. Grove's mapping agrees. |
| Retained native route | `skill-discovery.md`, *Configuration readers*, retains the 1.1.0 TypeScript route. Grove keeps its assets and pinned inputs. |
| Settings and HOME | The same reader table names all four 2.0-only keys and the 1.1 refusal. Its shared-settings rule requires HOME isolation for simultaneous directory delivery. Grove's mapping agrees. |
| Prompt fields | `configuration-evaluation.md`, *Request, context and selection*, defines both prompt fields and the unchanged structured fields. `skill-discovery.md`, *Caller migration*, requires task analysis to read `originalPrompt`. Grove's forwarding and structured readers satisfy that distinction. |
| First load and ending bytes | `skill-discovery.md`, *The prepared prompt*, requires mandatory reads first and preserves caller bytes contiguously. Its migration rule preserves ending commands. Grove's mandate and acceptance account for both. |
| Descriptor environment and resolver | `skill-discovery.md`, *RAVEL_SKILL_DIRECTORIES* and *ravel skills resolve*, make both optional for Grove. The environment variable grants no authority and is replaced or removed in managed environments. No Grove scrub change follows. |
| Notice and failure documents | `skill-discovery.md`, *Schemas and identities*, retains both schema versions at 2. The session specification keeps diagnostics on stderr. Grove consumes neither document as a protocol. |
| Versioned prefix | `release-channels.md`, *One release, two formula channels*, makes `ravel@2` keg-only. Grove's explicit PATH selection and unversioned formula dependency match the coordinated tap states. |

## Evidence limits

The graph inspection used Tier 2 on project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`.
Coverage generation `2026-11-01T22:46:44Z` matched its recorded metadata.
The targeted asset search returned all 27 function matches without pagination.
Exact snippets and the `groveContext` trace supported the caller checks.

Coverage reported no recorded gap for the checked caller code and documents.
That is a best-effort signal. The four reviewed Mermaid sources had partial
parse ranges. The review read every line of those files directly.
The asset inventory and direct reads also checked the graph's bounded scope.

The review served the design through
`mise exec -- task design:harness-selection PORT=52751`.
It inspected the four view sources and fetched the served manifest.
The browser inventory was empty. This review therefore makes no new claim
about rendered appearance. The producer's rendering evidence remains recorded.

No real harness, confinement, acquisition or release trial ran in this review.
Those proofs remain with their named implementation and acceptance owners.

**Handoff.** This review inserted grove-rhai-skill-delivery-k52 with kind
`integrate-review-design` before ravel-local-delivery-k23. The integration
reads these findings and owns their disposition. The common-loading node stays
live with implementation and acceptance ahead.
