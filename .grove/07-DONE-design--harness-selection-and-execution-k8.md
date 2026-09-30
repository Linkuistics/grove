# harness-selection-and-execution-k8

## Goal

Redesign the harness-dispatch artifact-identity, original-creator and
supplied-review-policy area. An unattended Grove review must find its original
creator's provider without a per-review human step, and the design must hold
no Grove state that contradicts a standing decision.

## Context

This leaf exists because review `harness-selection-and-execution-k5` found the
original-creator mechanism unreachable in unattended loops (its F1). It also
found the per-grove dispatch scope in conflict with
`docs/adr/one-live-driver-per-working-tree.md` (its F2). Integration
`harness-selection-and-execution-k7` classified every k5 finding, and its running
log records the dispositions this design must satisfy. For F3 (fail-closed
review recognition), F4 (provider membership check) and F5 (no tree enumeration
the SDK cannot perform), satisfy each or record why the new design makes it
moot. The same applies to F11's association wording and the glossary's
dispatch-scope entry.

The human questioned why creators must be registered and why a handle namespace
exists. Their suggestions, none yet settled, are listed in the root brief's
design handoff. The earlier proposal k7 put to them — `grove-llm leaf-retire`
writing a `**Produced-by:**` line into the producing leaf — was not adopted, but
its analysis in k7's log still applies.

Rules any option may collide with:

- `plugins/grove/skills/grove/TASK-FORMAT.md` — the body carries no launch
  metadata, including no record of how a past session ran.
- `plugins/grove/skills/grove/references/retire.md` — retirement touches one
  filename.
- `plugins/grove/skills/grove/references/decompose.md` — Grove records nothing
  about how the producer ran.
- `plugins/grove/conformance/rules.tsv` — the corresponding conformance rules.
- The one-live-driver ADR — its rejected "generation id under `.grove/`" option
  and its statement that `.grove/` is the only durable workflow state.

The spec's affected sections are "Artifact identity and original creator",
"Supplied review policy", the `task_scope` slot and `dispatch-scope` commands in
"Grove integration", and creator revisions in "Records". The `creator.mmd` view
belongs to the same area.

## Done when

- The spec states one mechanism by which the supplied review policy obtains the
  original creator's provider for a Grove review. When the producer ran through
  dispatch, it needs no per-review human step. A declared remedy remains for
  direct-harness and pre-adoption artifacts, and missing both refuses.
  Non-Grove callers can still supply a generic reviewed-artifact/creator
  association, and the generic core still has no review semantics.
- It deterministically answers which producing invocation is the original
  creator when a leaf ran more than once. A retry of the review cannot silently
  change that answer.
- Every durable piece of state the design keeps has one home, reconciled with
  the ADR set in place. Machinery the new mechanism makes unnecessary is deleted
  from the spec, glossary, ADRs and views, not left beside it: the scope, its
  slot and commands, the inode tripwire and creator registration, where applicable.
- The human has explicitly confirmed any amendment to the brief's acceptance
  sentences or to the methodology rules above before it is adopted. If a choice
  still turns on the human's preference, stop and ask with a recommendation.
- The spec header's redesign note is removed, and the test-seam rows cover the
  new mechanism.
- As its last act, this leaf cuts `review-design` with `leaf-insert` at the
  planning leaf `harness-selection-and-execution-k6`, so the review runs before
  planning; the human required that review. Its body names this leaf's handle
  and scopes the read to the whole current design, including k7's repairs.

## Notes

The human asked not to overdevelop this rare case. Prefer the option that
removes machinery, and compare each suggestion concretely:

- Who writes the provenance, and from what? For example, dispatch could export
  the selected candidate to the final harness's environment.
- Which invocation does it name?
- How does a review find it: the task file supplied to dispatch, `grove-llm`, or
  a caller-supplied value?
- How is recorded provenance distinguished from declared?
- What happens with a review cut eagerly by a session that did not produce the
  artifact?

Grove passes an opaque command, so a Grove-runner option must say where Grove
would learn the provider. New runtime claims need their own probes and positive
controls, as k7's integration probe did.

## Decisions (running log)

Evidence: read the brief, this task, k1/k3/k4/k5/k7, the spec, both design ADRs,
the one-live-driver ADR, the glossary, `docs/USAGE.md`, `docs/ARCHITECTURE.md`,
`docs/CONFIGURATION.md`, the design views, the repository's methodology corpus
(identical to the installed skills by `diff -rq`) and
`crates/grove-llm/tests/composition_guidance.rs`. No harness-dispatch code
exists yet. Mentions of the machinery were enumerated with `rg --hidden`. A
positive control found this handle under `.grove/`, and the same search without
`--hidden` came back clean.

A handle-keyed lookup cannot identify its grove without stored state. A
workspace hosts successive groves. Finish deletes `.grove/` and keeps the
workspace, and a new tree reuses keys such as `plan-k1` (one-live-driver ADR;
USAGE "Each workspace is its own grove"). The task-file path therefore names a
workspace, not a grove generation. Consider a stale same-handle record from an
earlier grove, beside a current artifact made by a direct harness. A lookup by
(workspace, handle) would find the stale provider and admit a reviewer that may
share the real creator's provider. The brief forbids that outcome. Only three
things separate groves: an added durable identity, which the one-live-driver
ADR rejected ("Reopen only if handles must be comparable across separately
created groves"); VCS-history or inode derivation, which is fragile or heavy
(k5 F2); or a globally unique reference carried by the tree itself.

Grove cannot tell dispatch the producer's provider. Grove launches an opaque
command and cannot know the provider. It could learn it only by persisting
launch receipts, which the glossary's *Review target diversity* entry, USAGE and
`diversity-is-the-configs` all rule out. The human's Grove-runner suggestion is
therefore not viable without reversing those.

The `**Reviews:**` line is already contradicted by the accepted requirements.
The brief has the supplied Grove adapter recognise review relationships. Yet
TASK-FORMAT ("parsed by nothing"), the glossary ("**no code reads them**",
pinned by `composition_guidance.rs`), ARCHITECTURE and USAGE ("Grove neither
writes nor reads those lines") say no reader exists. Every option inherits this.
Those statements must be scoped to Grove's own code whichever mechanism is
chosen.

Four options were put to the human, with views served for the discussion:

- A: the review names the producer's dispatch run.
- B: runs are looked up by handle under a durable grove ID.
- C: the Grove runner supplies the provider.
- D: the current explicit registration.

C and D were shown as not viable, for the reasons above and k5 F1.
The recommendation was A, with evidence.

Human resolution of the mechanism: adopt A. Dispatch already exports
`HARNESS_DISPATCH_RUN_ID` to the final harness. The session that finishes a
producer writes `**Creator:** run <id>` directly under `**Reviews:**` on the
review leaf it cuts. The review's dispatch already receives that task file. The
supplied policy looks up the run's immutable record, so the provider is
recorded, not transcribed. For an artifact with no run, the owner writes
`**Creator:** declared <provider>`. Run identities are globally unique, so
nothing is looked up by handle, and no namespace, scope slot, `dispatch-scope`
command, inode tripwire or creator registration remains.

Human resolution of the methodology rules: adopt the amendments, including
reviews cut before their producer ran. TASK-FORMAT lets a review body carry one
`**Creator:**` line, its only record of a past session. retire.md rescopes its
"needs no record of how the session that produced it ran" sentence, while
retirement itself still touches one filename. decompose.md tells the finishing
producer to write its run on the review it cuts and on any live review already
naming its handle. The "no code reads them" wording (glossary, ARCHITECTURE,
USAGE, TASK-FORMAT "parsed by nothing") becomes "no Grove code". These land with
the dispatch implementation, with their conformance rows and the
`composition_guidance.rs` pins, so the corpus never instructs a session about a
tool that is not installed.

Human resolution of the acceptance sentences: amend both. "Design namespaces it
against other groves" becomes: lookups use globally unique run identities, so
handles need no namespace. "Association records live outside task bodies"
becomes: run and observation records live outside task bodies, and a review
task carries only its creator reference.

Design-owned consequences, settled on that basis. The original creator is the
dispatch run of the session that finished the producer; a crash-restarted
producer's finishing session rewrites the line before the review runs, and a
review retry reads the same line and immutable record. The example does not
require the named run's task ID to equal the `**Reviews:**` handle: a decomposed
producer is finished by a child with its own handle, so the check would refuse
correctly named runs. Inspection and the review's run record show the named
run's task ID beside the handle for audit instead. F3 (fail-closed recognition),
F4 (exact catalog membership) and F5 (read only the supplied task file) hold
unchanged. F11's association wording becomes: a run associates with its task
identity, a reviewed artifact, both or neither. The dispatch-scope glossary entry
is deleted.

The decision earns a new ADR, `a-review-carries-its-creator-reference`, which
meets all three tests: it amends a standing methodology rule, a run ID in a task
body is surprising, and six alternatives were rejected. The policy-ownership and
process-replacement ADRs were reworked in place to cite it. `CONTEXT-MAP.md` did
not register this area's `policy-evaluation-precedes-process-replacement` ADR or
its spec; both are now in the grove context's list. Fifteen other records are
also unregistered (enumerated against the map, with a registered slug as
control). They predate and fall outside this workstream, so they are reported
rather than grown into this tree.

Review `harness-selection-and-execution-k9` was cut with `leaf-insert` at planning
`harness-selection-and-execution-k6`, as this leaf's charter requires. Its body
names this handle and scopes the read to the whole current design, including
k7's repairs. It carries no `**Creator:**` line: the convention is not yet in the
installed methodology, and this session ran under no dispatcher. k6's body now
names the methodology amendments that ship with the implementation, so planning
schedules them with it.

## Verification

The area was swept for the removed machinery: the scope, `task_scope`, the
`dispatch-scope` and creator commands, `originalCreator`, `producedArtifact`
and the UUID. The sweep enumerated every hit with `rg --hidden` across the spec,
ADRs, views, glossary, map and tree, and classified each. One stale label was
fixed: option D was still called "current spec". The same pattern matched the
pre-session spec (`@-`) many times, a control seen to fail. It now matches only
the spec's statement that no creator registration exists.

A link and anchor checker over the eight changed documents found no missing
file or anchor. The same checker reported both faults planted in a scratch
control file, which was removed before any jj snapshot.

All five views rendered with pinned Mermaid 12.0.0 in Playwright's Chromium. At
1440 and 1200 px no view overflowed at 1200 px. They also rendered in dark mode
at 390 px. At that width the five-lane sequence views scale below legibility, as
the existing handoff view already did; the viewer's full-size and source links
remain. The k3 viewer process on port 8769 is wedged: it accepts connections and
never replies. It was left running, and this session served on 8772.

`task check` exited 0. All eleven principal checks passed, including the Rust
suite (1449 passed, 0 failed) and final validation of all six walkthrough books.
The jj working-copy commit was `f35f6a5e9168` before and after the run. The log
was kept in this session's scratch space. This note and the retirement rename
follow the run and change no checked contract. Nothing is claimed as
implemented. No in-session reviewer was spent; review
`harness-selection-and-execution-k9` is scheduled before planning.

The root still has live leaves, so there is no ancestor close to cascade.
