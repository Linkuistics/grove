# harness-selection-and-execution-k9

**Reviews:** harness-selection-and-execution-k8

## Goal

Independently assess the whole current harness-dispatch design against the
accepted requirements and current Grove contracts before planning turns it into
implementation work. This covers the redesigned original-creator mechanism and
the integration repairs no review has yet read.

## Context

Read the producer's committed artifact via its stable handle, against the whole
current design, not only k8's diff:

- `docs/specs/harness-selection-and-execution.md`.
- The ADRs `harness-selection-is-owned-by-policy`,
  `policy-evaluation-precedes-process-replacement` and the new
  `a-review-carries-its-creator-reference`.
- The glossary entries the design uses.
- `docs/design/harness-selection-and-execution/`.

That design includes the repairs `harness-selection-and-execution-k7` applied
for review `harness-selection-and-execution-k5`: F6–F11 and the human's Linux
floor choice for F7. No review has read those repairs. The requirements
authority is the root brief, with the two acceptance sentences the human amended
in k8, together with the k1, k4, k7 and k8 running logs. The design is not
evidence of implemented behaviour or of new human approval beyond the choices
those logs record.

## Done when

- Findings distinguish contract gaps, unnecessary obligations and accepted
  trade-offs. Source or runtime evidence supports each one where it bears on the
  conclusion.
- The whole acceptance surface is assessed: standalone use, runtime delivery,
  authority, context limits, the original creator, records and both process
  seams. The spec and ADR set are coherent and claim no implementation results.
- If findings warrant integration, the integration leaf is inserted immediately
  before planning `harness-selection-and-execution-k6`. Its body names this
  review rather than copying the findings.

## Notes

Producer doubts worth adversarial examination:

- The creator reference depends on a session copying its own run ID. Test
  whether a wrong but existing run can pass. The design dropped a
  task-identity equality check because a decomposed producer is finished by a
  child task with its own handle. Decide whether that trade-off holds.
- The finishing producer writes its line on the review it cuts and on any live
  review already naming its handle. Check that a session can follow this, and
  that crash restarts, correction runs and decomposed producers all reach a
  deterministic creator.
- The methodology amendments are stated in the new ADR rather than applied, and
  ship with the implementation. They touch TASK-FORMAT, retire.md,
  decompose.md, the "no code reads them" wording and their conformance rows.
  Check that they are precise enough to implement and that nothing else in the
  corpus contradicts a `**Creator:**` line.
- Removing the scope and registration must not lose anything the requirements
  need, such as later outcome association or records after teardown.

The producer spent no in-session reviewer; this scheduled review is the
adversarial read.

## Decisions (running log)

Reviewed the whole current design at `4158f6cb3e8c`, including the independent
repairs committed by k7 at `4ab09343214c`. The requirements authority is the root
brief and the k1/k4/k7/k8 running logs. The human's creator-reference mechanism,
methodology amendments, launch-time validation boundary and documented kernel
trade-off remain accepted. This review does not infer new human approval from
the producer's design-owned details.

Three actionable findings are recorded below: two missing creator-reference
lifecycle rules and one inaccurate evidence claim. The repairs can stay within
the chosen mechanism; they require neither a handle namespace nor creator
registration. Dropping task-ID equality is justified by decomposed producers,
but does not make an arbitrary existing run reference authentic. No in-session
reviewer was used. Integration belongs immediately before planning k6.

## Findings

### F1 — High: a direct-harness restart can retain the previous creator

Location: `docs/specs/harness-selection-and-execution.md:359–369` and
`docs/adr/a-review-carries-its-creator-reference.md:23–36`.

The finishing invocation defines the original creator. A dispatched finishing
session replaces the reference on an already-cut review, but the direct-harness
case only says it has no run ID to write and therefore its review has no run
reference. There is no operation that makes that conclusion true for an
existing review. The proposed methodology amendment instructs the session to
write its run; it supplies no invalidation branch when there is no current run.

Contract trace: invocation R1 of producer P uses provider A and writes
`Reviews P` / `Creator run R1`, then stops before retiring P. On restart, personal
configuration routes P directly to provider B, which finishes P. Doing nothing
to the already-written line leaves R1, an existing run without a failure detail.
The supplied review policy reads A, passes catalog membership and can admit a
reviewer from B. That reviewer shares the finishing creator's provider, contrary
to the accepted rule. A stale owner declaration has the same problem when a
different invocation becomes the creator. This does not ask a direct-harness
route to enforce dispatch policy; it prevents its subsequent dispatched review
from treating obsolete provenance as current.

Smallest useful correction: specify replacement/invalidation for every finishing
producer, including one without a dispatch run. An obsolete reference must not
remain effective; the no-run case must require a declaration for the actual
finishing creator. State how a declaration supplied for that invocation is
preserved or refreshed. Carry that branch into the methodology amendment and
acceptance cases. Cover a dispatched attempt followed by a direct-harness finish
with a pre-existing review line; missing current provenance must refuse.

### F2 — Medium: the amendment does not carry provenance through node closes

Location: `docs/adr/a-review-carries-its-creator-reference.md:33–36,88–92`,
`docs/specs/harness-selection-and-execution.md:359–365,397–402`, and
`plugins/grove/skills/grove/references/retire.md:75–110`.

The design explicitly permits a child run to be the creator of a decomposed
producer, which is why run task identity need not equal the reviewed handle.
But the proposed writing rule only names reviews of the finishing producer's
own handle, before retirement. It does not amend the separate ancestor-close
procedure or say which reviewed handles a child session must update there.
A node is never retired: the last child retires, then checks and closes ancestors,
reporting their handles alongside its own in the commit.

Contract trace: producer P has a waiting review V with `Reviews P`, and P is
decomposed. Final child C finishes under run RC and closes P. Updating reviews
that name C does not update V. If V has no Creator line, the unattended review
refuses despite a recorded creator; if an earlier session wrote one, V can use
that earlier provider. Cascading closes can leave several ancestor reviews in
the same situation. The acceptance rows at spec lines 600–604 cover ordinary
retirement/reordering, but do not require this case.

Smallest useful correction: define the handoff over the completed leaf plus
each ancestor whose Done when is established during that session's close
cascade. For each relevant producer handle, update its waiting reviews from the
finishing run, or apply F1's no-run rule, in the same focused commit. Do not
update ancestors that remain incomplete or turn supporting repair work into a
new original creator automatically. Add this step to the node-close amendment
and give it a fixture with a pre-cut review of a decomposed producer and a
multi-level close. The dispatcher adapter can still read only its own task file.

### F3 — Low: accessible run references are mistaken for verified association

Location: `docs/adr/a-review-carries-its-creator-reference.md:49–55` and
`docs/specs/harness-selection-and-execution.md:351–353,397–402`.

The ADR acknowledges that a wrong existing run lends its provider, then limits
that risk by asserting that a producer can reach only its own run ID. The
mechanism itself disproves that assertion: another review body exposes another
producer's run ID, and task bodies and VCS history are readable by sessions.
The SDK and `record show --run` also admit lookup of a known ID. This is not a
hostile-owner scenario; a session reconciling previous task work can encounter
several valid IDs and copy the wrong one.

Contract trace: a task names an unrelated existing run RA of provider A, while
its actual creator used B. RA has no failure detail, and A is in the current
catalog. A reviewer from B passes every specified check. Inspection displays
the mismatch but launch does not require a human to inspect it. Similarly, a
reference to an attempt whose execution remains unknown is not independently
verified execution merely because a line names it.

Smallest useful correction: remove the own-ID-only claim and describe the
association and execution assertion as trusted session/caller attestation,
distinct from the immutable recorded candidate/provider. Make the wrong-existing-
run limitation explicit wherever execution-recorded provenance is explained.
The task-ID equality check still should not return: it would reject the legitimate
child/ancestor case. This is an evidence-wording repair and an acknowledged
transcription trade-off, not a request for authentication, multi-author
accounting or new registration machinery.

## Acceptance assessment

These are design assessments, not implementation acceptance results.

| Surface | Assessment |
|---|---|
| Standalone use and package boundary | Supported. Optional task/context, open caller kind and generic reviewed-artifact data require no Grove binary. The embedded adapter has a concrete substitution need and remains an explicit import. |
| Static/computed routing, choices and refusals | Supported. Joint catalog IDs, exactly one selector form, explicit-choice mismatch rejection and selected-executable availability checks preserve the no-fallback requirement. Static choice acceptance is explicit; constrained choices use select. |
| Inspection and context limits | Supported. Prompt omission for inspection, the refusal's equivalent inspect invocation, measured sources, authority, configurable finite bounds and overflow refusal are specified. Trusted policy can have effects; inspection promises no launch, not purity. |
| Authority and runtime startup | Supported at design scope. Private worker cwd, pre-start environment construction, disabled autoloading and registered embedded specifiers address the recorded ambient cases. The runtime evidence identifies firing controls and non-firing fixtures without claiming a production boundary was exercised. |
| Original creator and unattended review | Unresolved: F1 and F2. Recorded provider lookup, declared adoption, exact provider membership, retry checks and fail-closed review recognition otherwise satisfy the stated policy boundary. F3 limits the evidence claim. |
| Records and later outcomes | Supported. Run IDs avoid handle lookup and survive teardown; record show/observe provides an actual import path, idempotency, corrections and unknown/unobserved values. A review can associate later findings with the named run. No automatic post-teardown artifact lookup is promised. |
| Foreground execution, cancellation and authority | Supported at design scope. Reap-before-exec, null helper stdin, scrubbed helper environment, inherited final environment, explicit signal-state preservation and post-commit cancellation rules retain Grove's job ownership. SIGPIPE startup capture and the final handoff remain implementation obligations at the command and PTY seams. |
| Runtime delivery and compatibility | Supported at design scope. Installed matched front/worker, archive/Homebrew layout and per-target no-system-runtime smoke cases are explicit. glibc/CPU execution is required; the documented-but-unexecuted kernel range is the human's accepted trade-off. Only macOS arm64 design probes are recorded. |
| Grove integration and documentation | Supported apart from F1/F2. The three optional native slots carry authoritative data without prompt scraping or a second pick. Launch-time delegated validation and direct-harness compatibility remain explicit. Methodology, conformance and composition pins are named implementation work rather than already-applied amendments. |
| ADR set and seam count | Supported. Policy ownership, worker/exec choice and the review-carried reference are independent decisions with real rejected alternatives. Three records form a coherent set; neither merging unrelated choices nor adding another ADR is justified. The two agreed process seams remain, with release delivery checks in addition. |

## Evidence and verification limits

Tier 2 graph verification used project
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`,
generation `2026-09-29T11:18:53Z`. Searches were bounded and fully returned;
the launch function's inbound/outbound depth-one trace was not truncated. Exact
snippets were read for session expansion, configured launch, keyed-launch's
observed runner and spawn implementation, and the composition-guidance pin.
Coverage was checked for every relied-on path. Those Rust files and the inspected
methodology references had matching metadata and no recorded gap; that is
best-effort evidence, not a completeness proof. Docs and scripts are excluded,
and glossary/tree/config metadata was changed or unavailable, so the relevant
current source was read directly. No negative repo-wide claim relies on graph
absence.

Source inspection confirms the current launch seam supplies four native slots,
the runner creates the foreground child's process group and publishes its fresh
channel, and the composition pin still asserts that no code reads relationship
lines. The proposed slots and rescoped wording are future implementation work.
The current release script's explicit package builds and the formula's two-command
install show why delivery must change; the design already requires those changes.

F1–F3 are traces through the written contracts, not executed harness experiments.
Recorded k7/k8 checks and native probes were inspected as producer evidence.
This inspection-only review ran no test, build, lint, format or runtime probe,
changed no design/production/test artifact, and spent no nested review allowance.

Integration `harness-selection-and-execution-k10` was inserted immediately before
planning k6. Its body points to this review rather than copying the findings.
The root retains live integration and planning leaves, so no node-close cascade
is due in this session.
