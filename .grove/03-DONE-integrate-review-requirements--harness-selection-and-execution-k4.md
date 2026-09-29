# harness-selection-and-execution-k4

**Integrates:** harness-selection-and-execution-k2

## Goal

Triage the requirements review's findings against the agreed first-release
requirements, and apply the real ones to the requirements artifact before design
begins.

## Context

Read the findings from the commit of `harness-selection-and-execution-k2`, not
from this body. The reviewed artifact is the committed work of
`harness-selection-and-execution-k1`: the root brief, its running decisions and
`docs/adr/harness-selection-is-owned-by-policy.md`, with
`docs/research/grove-model-effort-routing.md` as starting evidence.

## Done when

- Each finding is classified — contract stated unclearly, real issue, visible
  trade-off, or noise — with the reason recorded in this leaf's running log.
- Accepted findings are applied to the root brief, the ADR or the research record,
  and choices only the human can make are put to the human with a recommendation.
- Settled human choices stay settled; the superseded stronger review-identity
  rules are not restored.
- `harness-selection-and-execution-k3` can start from requirements that need no
  further interview.

## Notes

Several findings ask for a human decision rather than an edit; stopping to ask is
expected for this kind.

## Decisions (running log)

Evidence: read the findings from review commit `52183d8bc9af`, against the k1
running decisions, root brief, boundary ADR, research and cited Grove contracts.
The graph project is
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`,
Tier 2, generation `2026-09-29T11:18:53Z`. Coverage reports exclude `docs/` and
`scripts/` and report changed/untracked metadata for the grove artifacts; the
relevant markdown and script ranges were read directly. No source-completeness
claim relies on the graph.

F1 — real issue. Direct-harness compatibility leaves artifacts without selector
records; the root brief gives no adoption remedy. The supplied policy's scope
can be clarified to reviews it selects, without promising enforcement over
opaque direct commands. Asked the human whether to admit an explicit declared
original-creator provider (recommended, labelled separately from execution
records), or retain execution-record-only refusal and document adoption order.
The remedy remains pending; no silent permission is introduced.

F2 — real issue. `CONTEXT.md` defines position as mutable and the handle as
surviving renumbering; retirement changes the path. Require caller-supplied stable
artifact/task identity, with Grove supplying its selected handle as data.
Association must survive retirement/reordering across sessions in the same live
grove and workspace. Cross-checkout discovery and post-teardown lookup are not
first-release obligations; stored records still retain their identities for
later outcomes. Design chooses namespacing to avoid collisions between groves
and the representation outside task bodies.

F3 — contract stated unclearly. The existing configuration contract requires a
wrapper to exec; the brief cannot promise post-exec observations from a process
that no longer exists. Require a persisted pre-launch record with run/association
identity, selected candidate and argv, policy/context versions and time; refusal
to persist it stops launch. Distinguish proposal, attempted handoff, observed
failure and externally confirmed execution. An attempt is never reported as
observed success. Exit, duration, usage and later outcomes may remain unknown;
provide a documented, tested way to add later outcome evidence. Provenance names
the configured launched choice, not independently observed backend identity or
a promise to prevent an in-harness model change. Preserve the existing Grove
job/exec contract; do not expand this repair into a new supervisor design.

F4 — real trade-off. Grove's current completeness check resolves the configured
command, including an opaque wrapper, and cannot prove its later computation.
Asked the human whether wrapper-only validation before tree mutation plus
delegated launch-time refusal is acceptable (recommended), or static mappings
must also be checked before authoring. This choice remains pending.

F5(a) — real trade-off already accepted by the trust decision. An explicit
relative `--config` or deliberate personal-policy import grants authority;
forbidding these or requiring every imported file to be named individually would
reopen the agreed programmable-policy boundary. Inspection must show the resolved
entry path and its authority; document that a cwd-relative selection deliberately
trusts that location in each checkout. Do not infer admission from mere file
presence. F5(b) — real issue: the host must not implicitly load repository
runtime configuration, preload hooks, environment files or shadowing modules
while loading personal policy. Require adversarial fixtures for those ambient
inputs. Explicit imports by trusted policy retain that policy's authority.

F6 — contract stated unclearly. Replace the absolute "cannot complete" claim
with no completion authority granted to selection helpers. Test absence of Grove
control values in helper environments and presence of the fresh channel in the
final harness; do not claim protection against hostile same-user trusted code.

F7 — contract stated unclearly. The minimal explicit choice is one configured
joint candidate ID. Policy may accept or refuse it and must see it; the generic
command checks that a returned result agrees, without implementing review
semantics. Partial model/effort overrides are not required. Independent callers
can supply the ID; Grove owners can put a literal choice in personal command
configuration, without new Grove flags, environment overrides or task metadata.
"Retry" means a new invocation; automatic retries are not required.

F8 — contract stated unclearly. Deliver the provider-separation rule in an
inactive, shipped example policy with explicit activation instructions, and test
that actual artifact at the command seam. Do not overwrite personal policy.
The supplied Grove policy/context adapter recognises review kinds and the
`Reviews` relationship outside the generic core; missing/ambiguous relationships
refuse. Non-Grove callers can supply equivalent generic artifact/creator
association data. This preserves independent use without making Grove's open
kinds a core enumeration.

F9 — contract stated unclearly. Provider is an owner-declared candidate attribute
for model origin, not a harness/executable inference or gateway identity. A
different gateway for the same origin does not establish separation; local
candidates likewise need an explicit origin label. Original creator is the one
producer execution associated with the artifact, not the latest configured
producer or all contributors. Add concise glossary definitions. Choosing that
association when a producer is reinvoked remains design's delegated obligation.

F10 — real issue. Cancellation must cover context loading and policy evaluation
as well as the final harness. Require interruption/timeout to launch no harness,
document non-zero refusal results, and preserve process-group cancellation of
helpers. Policy evaluation consumes no interactive stdin and cannot corrupt
inspection/protocol output; the host handles its diagnostic output. Design sets
finite configurable selection/context bounds. Records distinguish refusal from
harness evidence, but an abrupt kill need not produce a final record.

F11 — contract stated unclearly. Require refusal for missing required context
and failed context loading. Design must state measurable volume/time limits;
inspection reports delivered sources and size, with overflow refused or explicitly
represented to policy rather than silently omitted. No specific limit is chosen
by requirements integration.

F12 — contract stated unclearly. First-release Grove integration covers lifecycle
sessions; dedicated `grove run` integration under its separate confinement
contract is follow-up. Existing standalone templates remain valid. Independent
use without Grove remains required and is not the same as a `grove run` route.

F13 — real issue in acceptance, with the runtime prerequisite pending human
choice. Verify independent command use without Grove binaries, configuration or
task files. Require package-content checks and a static/TypeScript fake-harness
smoke test on each supported target (native or emulated), preserving the existing
Linux compatibility floor. Documentation is reviewed against acceptance, not
proved by process tests. Runtime hosting remains design-owned; asked whether
Grove's install must provide everything needed for TypeScript evaluation
(recommended), or a separately installed runtime is acceptable.

F14 — real issue. The research's unqualified "configuration has not been
changed" conflicts with k1's account of defaults applied by the preceding
session. Preserve the dated research observation as historical evidence and
correct its claim about subsequent adoption; do not claim to have re-inspected
today's personal configuration.

Human resolution of F1: allow explicit declared original-creator provenance.
The declaration is artifact-associated, inspectable and recorded as declared,
not execution-recorded; missing both forms still stops review. Adoption docs must
explain this remedy for artifacts created before the tool or by direct commands.

Human resolution of F4: validate delegated policy at launch. Grove retains its
pre-authoring completeness check over its configured command only; usage docs and
configure-grove must explain this boundary. No selector call is added to tree
mutation for static or computed policies.

Human resolution of F13: Grove's installation must provide TypeScript evaluation
without a separately installed runtime. Embedding versus bundling remains a
design decision, and per-target install checks must exercise that property.

## Verification

Re-read the repaired brief, boundary ADR, glossary additions, research proposal
and design handoff against all fourteen findings and the three human answers.
Each finding has a disposition above; the exact-file-only trust proposal was not
adopted, and the earlier stronger review-identity rules remain superseded. No
substantial redesign or additional producer chain is needed for these repairs.

`bash scripts/check.sh` exited 0: all eleven principal check groups passed,
including the complete Rust suite and final validation of all six walkthrough
books. The repository's tracked-file snapshot, including the check script,
manifests, source, fixtures and documentation, stayed at
`806a7b89a9a287fed079eed46b06724650defc1d` before and after the run. Output is in
`/tmp/grove-harness-selection-k4-check.log`. This verification note and the
retirement rename follow the run; they change no checked implementation or
requirements. No new feature is claimed as implemented by this documentation
integration.

The design leaf remains live, so there is no ancestor close to cascade. Its
context now points to these dispositions rather than treating the review's
recommendations as accepted requirements.
